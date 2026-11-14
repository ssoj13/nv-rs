//! RAII cuFile driver session, registered file/buffer/stream guards, and sync/async I/O.
//!
//! All CUDA stream-ordered async parameters (`size`, `file_offset`, `buf_offset`, and
//! `bytes_transferred`) live inside a stationary heap block inside [`AsyncIoSlot`], guarded by a
//! CUDA [`Event`] fence and strong `Arc` pins on [`FileHandle`], the target device buffer
//! ([`RegisteredBuffer`] or [`cuda_core::DeviceBuffer`]), and [`Stream`] until stream completion.
//! Neither the handle nor the buffer can be deregistered or dropped while async DMA is in flight,
//! and steady-state I/O performs zero heap allocations.

use std::ffi::c_void;
use std::marker::PhantomData;
use std::sync::{Arc, Mutex};

use cuda_core::{Context, DeviceBufferView, Event, PinnedHostBuffer, Stream};

#[cfg(target_os = "linux")]
use crate::check_status;
#[cfg(target_os = "linux")]
use crate::record_teardown_cufile;
use crate::{decode_io_ret, record_teardown_driver_err, DeviceSpan, DeviceSpanMut, Error, IoMode, Result};

/// Properties returned by `cuFileDriverGetProperties` (`CUfileDrvProps_t`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DriverProperties {
    /// NVFS kernel driver properties.
    pub nvfs: NvfsProperties,
    /// Feature flags bitmask (`CUfileFeatureFlags_t` bits).
    pub feature_flags: u32,
    /// Maximum device bounce cache size in KB.
    pub max_device_cache_size_kb: u32,
    /// Per-buffer cache size in KB.
    pub per_buffer_cache_size_kb: u32,
    /// Maximum device pinned memory size in KB.
    pub max_device_pinned_mem_size_kb: u32,
    /// Maximum batch I/O queue size.
    pub max_batch_io_size: u32,
    /// Maximum batch I/O timeout in milliseconds.
    pub max_batch_io_timeout_msecs: u32,
}

/// Nested `nvfs` fields from `CUfileDrvProps_t`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct NvfsProperties {
    /// NVFS driver major version.
    pub major_version: u32,
    /// NVFS driver minor version.
    pub minor_version: u32,
    /// Polling threshold size in KB.
    pub poll_thresh_size_kb: usize,
    /// Maximum direct I/O transfer size in KB.
    pub max_direct_io_size_kb: usize,
    /// Supported filesystem bitmask (`CUfileDriverStatusFlags_t` bits).
    pub driver_status_flags: u32,
    /// Driver control bitmask (`CUfileDriverControlFlags_t` bits).
    pub driver_control_flags: u32,
}

/// Flags passed to `cuFileBufRegister` (`CU_FILE_RDMA_*`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct BufRegisterFlags(u32);

impl BufRegisterFlags {
    /// No special buffer registration flags (`0`).
    pub const NONE: Self = Self(0);
    /// `CU_FILE_RDMA_REGISTER` (`1`).
    pub const RDMA_REGISTER: Self = Self(1);
    /// `CU_FILE_RDMA_RELAXED_ORDERING` (`2`).
    pub const RDMA_RELAXED_ORDERING: Self = Self(1 << 1);

    /// Combines two registration flag sets.
    pub const fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    /// Raw C bitmask passed to `cuFileBufRegister`.
    pub const fn bits(self) -> u32 {
        self.0
    }
}

/// Flags passed to `cuFileStreamRegister` (`CU_FILE_STREAM_*`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct StreamFlags(u32);

impl StreamFlags {
    /// Default stream behavior (`0`); parameters are read at stream execution time.
    pub const NONE: Self = Self(0);
    /// `CU_FILE_STREAM_FIXED_BUF_OFFSET` (`1`): buffer offset is fixed at submission time.
    pub const FIXED_BUF_OFFSET: Self = Self(1);
    /// `CU_FILE_STREAM_FIXED_FILE_OFFSET` (`2`): file offset is fixed at submission time.
    pub const FIXED_FILE_OFFSET: Self = Self(2);
    /// `CU_FILE_STREAM_FIXED_FILE_SIZE` (`4`): transfer byte count is fixed at submission time.
    pub const FIXED_FILE_SIZE: Self = Self(4);
    /// `CU_FILE_STREAM_PAGE_ALIGNED_INPUTS` (`8`): size, file offset, and buffer offset are 4 KiB aligned.
    pub const PAGE_ALIGNED_INPUTS: Self = Self(8);

    /// Combines two stream registration flag sets.
    pub const fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    /// Raw C bitmask passed to `cuFileStreamRegister`.
    pub const fn bits(self) -> u32 {
        self.0
    }
}

/// RAII guard for an open cuFile / nvidia-fs driver session (`cuFileDriverOpen` / `cuFileDriverClose_v2`).
pub struct DriverSession {
    ctx: Arc<Context>,
    active: bool,
}

impl DriverSession {
    /// Loads `libcufile` and opens the cuFile driver session (`cuFileDriverOpen`).
    ///
    /// Never silently falls back if GPUDirect Storage is unavailable.
    pub fn open(ctx: &Arc<Context>) -> Result<Self> {
        ctx.bind_to_thread()?;
        #[cfg(target_os = "linux")]
        {
            cufile_sys::load()?;
            // SAFETY: `cufile_sys` is loaded and takes no pointer inputs.
            let status = unsafe { cufile_sys::cuFileDriverOpen() };
            check_status("cuFileDriverOpen", status)?;
            Ok(Self { ctx: ctx.clone(), active: true })
        }
        #[cfg(not(target_os = "linux"))]
        {
            Err(Error::Unsupported("cuFileDriverOpen requires Linux"))
        }
    }

    /// Queries the active driver session properties (`cuFileDriverGetProperties`).
    pub fn properties(&self) -> Result<DriverProperties> {
        #[cfg(target_os = "linux")]
        {
            self.ctx.bind_to_thread()?;
            let mut raw = std::mem::MaybeUninit::<cufile_sys::CUfileDrvProps_t>::zeroed();
            // SAFETY: `raw` is properly aligned writable storage for `CUfileDrvProps_t`.
            let status = unsafe { cufile_sys::cuFileDriverGetProperties(raw.as_mut_ptr()) };
            check_status("cuFileDriverGetProperties", status)?;
            // SAFETY: `cuFileDriverGetProperties` succeeded and populated `raw`.
            let props = unsafe { raw.assume_init() };
            Ok(DriverProperties {
                nvfs: NvfsProperties {
                    major_version: props.nvfs.major_version,
                    minor_version: props.nvfs.minor_version,
                    poll_thresh_size_kb: props.nvfs.poll_thresh_size,
                    max_direct_io_size_kb: props.nvfs.max_direct_io_size,
                    driver_status_flags: props.nvfs.dstatusflags,
                    driver_control_flags: props.nvfs.dcontrolflags,
                },
                feature_flags: props.fflags,
                max_device_cache_size_kb: props.max_device_cache_size,
                per_buffer_cache_size_kb: props.per_buffer_cache_size,
                max_device_pinned_mem_size_kb: props.max_device_pinned_mem_size,
                max_batch_io_size: props.max_batch_io_size,
                max_batch_io_timeout_msecs: props.max_batch_io_timeout_msecs,
            })
        }
        #[cfg(not(target_os = "linux"))]
        {
            Err(Error::Unsupported("cuFileDriverGetProperties requires Linux"))
        }
    }

    /// Returns the process-wide cuFile driver reference count (`cuFileUseCount`).
    pub fn use_count() -> Result<i64> {
        #[cfg(target_os = "linux")]
        {
            cufile_sys::load()?;
            // SAFETY: `cuFileUseCount` reads process-global state without dereferencing arguments.
            let count = unsafe { cufile_sys::cuFileUseCount()? };
            Ok(count)
        }
        #[cfg(not(target_os = "linux"))]
        {
            Err(Error::Unsupported("cuFileUseCount requires Linux"))
        }
    }

    /// Explicitly closes the cuFile driver session (`cuFileDriverClose_v2`), surfacing any error.
    pub fn close(mut self) -> Result<()> {
        self.close_inner()
    }

    fn close_inner(&mut self) -> Result<()> {
        if !self.active {
            return Ok(());
        }
        self.ctx.bind_to_thread()?;
        self.active = false;
        #[cfg(target_os = "linux")]
        {
            // SAFETY: this session previously opened `cuFileDriverOpen` successfully.
            let status = unsafe { cufile_sys::cuFileDriverClose_v2() };
            check_status("cuFileDriverClose_v2", status)?;
        }
        Ok(())
    }
}

impl Drop for DriverSession {
    fn drop(&mut self) {
        if self.active {
            self.active = false;
            match self.ctx.bind_to_thread() {
                Err(err) => self.ctx.record_err(Err::<(), _>(err)),
                Ok(()) => {
                    #[cfg(target_os = "linux")]
                    {
                        // SAFETY: closes the session opened by `DriverSession::open` on bound context.
                        let status = unsafe { cufile_sys::cuFileDriverClose_v2() };
                        record_teardown_cufile(&self.ctx, status);
                    }
                }
            }
        }
    }
}

/// RAII guard for a device or host memory region registered with `cuFileBufRegister`.
///
/// Wrapped in [`Arc`] so [`FileHandle::read_async`] and [`FileHandle::write_async`] keep the
/// registration and backing buffer alive until stream completion. Deregisters via
/// `cuFileBufDeregister` before the wrapped buffer `B` is dropped.
pub struct RegisteredBuffer<B: DeviceSpan> {
    inner: Option<B>,
    ptr: cuda_bindings::CUdeviceptr,
    len: usize,
    ctx: Arc<Context>,
    registered: bool,
}

impl<B: DeviceSpan> RegisteredBuffer<B> {
    /// Registers `buf`'s entire byte range with cuFile (`cuFileBufRegister`).
    pub fn register(buf: B, flags: BufRegisterFlags) -> Result<Arc<Self>> {
        let ptr = buf.cu_deviceptr();
        let len = buf.num_bytes();
        let ctx = buf.context().clone();
        if ptr == 0 || len == 0 {
            return Err(Error::Invalid("cannot register null or zero-length device buffer with cuFile"));
        }
        ctx.bind_to_thread()?;

        #[cfg(target_os = "linux")]
        {
            cufile_sys::load()?;
            let c_flags = i32::try_from(flags.bits())
                .map_err(|_| Error::Invalid("cuFileBufRegister flags overflow i32"))?;
            // SAFETY: `ptr..ptr+len` is owned by `buf` until deregistration in `deregister` or `Drop`.
            let status = unsafe {
                cufile_sys::cuFileBufRegister((ptr as usize) as *const c_void, len, c_flags)
            };
            check_status("cuFileBufRegister", status)?;
            Ok(Arc::new(Self { inner: Some(buf), ptr, len, ctx, registered: true }))
        }
        #[cfg(not(target_os = "linux"))]
        {
            let _ = (buf, flags);
            Err(Error::Unsupported("cuFileBufRegister requires Linux"))
        }
    }

    /// Borrows the underlying buffer.
    pub fn inner(&self) -> Result<&B> {
        self.inner.as_ref().ok_or(Error::Invalid("registered buffer already consumed"))
    }

    /// Mutably borrows the underlying buffer when exclusively held.
    pub fn inner_mut(&mut self) -> Result<&mut B> {
        self.inner.as_mut().ok_or(Error::Invalid("registered buffer already consumed"))
    }

    /// Explicitly deregisters the memory region (`cuFileBufDeregister`) and returns `B`.
    ///
    /// Fails with [`Error::Invalid`] if an async I/O operation still pins this buffer.
    pub fn deregister(this: Arc<Self>) -> Result<B> {
        let mut this = Arc::try_unwrap(this)
            .map_err(|_| Error::Invalid("registered buffer still referenced by in-flight async I/O"))?;
        this.deregister_raw()?;
        this.inner.take().ok_or(Error::Invalid("registered buffer already consumed"))
    }

    fn deregister_raw(&mut self) -> Result<()> {
        if !self.registered {
            return Ok(());
        }
        self.ctx.bind_to_thread()?;
        self.registered = false;
        #[cfg(target_os = "linux")]
        {
            // SAFETY: `self.ptr` was previously registered with `cuFileBufRegister` and `self.inner`
            // is still alive until after this call completes.
            let status = unsafe { cufile_sys::cuFileBufDeregister((self.ptr as usize) as *const c_void) };
            check_status("cuFileBufDeregister", status)?;
        }
        Ok(())
    }
}

impl<B: DeviceSpan> DeviceSpan for RegisteredBuffer<B> {
    fn context(&self) -> &Arc<Context> {
        &self.ctx
    }

    fn cu_deviceptr(&self) -> cuda_bindings::CUdeviceptr {
        self.ptr
    }

    fn num_bytes(&self) -> usize {
        self.len
    }
}

impl<B: DeviceSpanMut> DeviceSpanMut for RegisteredBuffer<B> {}

impl<B: DeviceSpan> Drop for RegisteredBuffer<B> {
    fn drop(&mut self) {
        if self.registered {
            self.registered = false;
            match self.ctx.bind_to_thread() {
                Err(err) => self.ctx.record_err(Err::<(), _>(err)),
                Ok(()) => {
                    #[cfg(target_os = "linux")]
                    {
                        // SAFETY: `self.inner` drops after `Drop::drop` returns, so `self.ptr` is still valid.
                        let status = unsafe {
                            cufile_sys::cuFileBufDeregister((self.ptr as usize) as *const c_void)
                        };
                        record_teardown_cufile(&self.ctx, status);
                    }
                }
            }
        }
    }
}

/// RAII guard for a CUDA stream registered with cuFile hints (`cuFileStreamRegister`).
pub struct RegisteredStream {
    stream: Arc<Stream>,
    registered: bool,
}

impl RegisteredStream {
    /// Registers `stream` with cuFile (`cuFileStreamRegister`).
    pub fn register(stream: &Arc<Stream>, flags: StreamFlags) -> Result<Self> {
        stream.context().bind_to_thread()?;
        #[cfg(target_os = "linux")]
        {
            cufile_sys::load()?;
            // SAFETY: `stream.cu_stream()` is a valid CUDA stream owned by `Arc<Stream>`.
            let status = unsafe { cufile_sys::cuFileStreamRegister(stream.cu_stream(), flags.bits()) };
            check_status("cuFileStreamRegister", status)?;
            Ok(Self { stream: stream.clone(), registered: true })
        }
        #[cfg(not(target_os = "linux"))]
        {
            let _ = flags;
            Err(Error::Unsupported("cuFileStreamRegister requires Linux"))
        }
    }

    /// Underlying CUDA stream.
    pub fn stream(&self) -> &Arc<Stream> {
        &self.stream
    }

    /// Explicitly deregisters the stream (`cuFileStreamDeregister`).
    pub fn deregister(mut self) -> Result<()> {
        if !self.registered {
            return Ok(());
        }
        self.stream.context().bind_to_thread()?;
        self.registered = false;
        #[cfg(target_os = "linux")]
        {
            // SAFETY: `stream` was previously registered with `cuFileStreamRegister`.
            let status = unsafe { cufile_sys::cuFileStreamDeregister(self.stream.cu_stream()) };
            check_status("cuFileStreamDeregister", status)?;
        }
        Ok(())
    }
}

impl Drop for RegisteredStream {
    fn drop(&mut self) {
        if self.registered {
            self.registered = false;
            match self.stream.context().bind_to_thread() {
                Err(err) => self.stream.context().record_err(Err::<(), _>(err)),
                Ok(()) => {
                    #[cfg(target_os = "linux")]
                    {
                        // SAFETY: `self.stream` remains alive for the duration of this teardown call.
                        let status =
                            unsafe { cufile_sys::cuFileStreamDeregister(self.stream.cu_stream()) };
                        record_teardown_cufile(self.stream.context(), status);
                    }
                }
            }
        }
    }
}

#[repr(C)]
struct AsyncParams {
    size: usize,
    file_offset: i64,
    buf_offset: i64,
    bytes_transferred: isize,
}

struct InFlightPins {
    fh: Arc<FileHandle>,
    _buf: Arc<dyn DeviceSpan + Send + Sync>,
    _stream: Arc<Stream>,
}

struct PendingPosixWrite {
    file_offset: u64,
    len: usize,
}

/// Reusable async transfer slot holding stationary C parameter storage, an event fence, strong
/// `Arc` pins for the active [`FileHandle`], target device buffer, and [`Stream`], plus an
/// on-demand pinned staging buffer for explicit [`IoMode::PosixFallback`] transfers.
///
/// Allocates parameter storage once in [`AsyncIoSlot::new`] and grows staging capacity monotonically,
/// yielding zero heap allocations in steady-state stream-ordered I/O.
pub struct AsyncIoSlot {
    ctx: Arc<Context>,
    params: Box<AsyncParams>,
    fence: Event,
    fence_armed: bool,
    pending_call: Option<&'static str>,
    posix_bytes: Option<usize>,
    pending_posix_write: Option<PendingPosixWrite>,
    in_flight: Option<InFlightPins>,
    staging: Option<PinnedHostBuffer<u8>>,
}

impl AsyncIoSlot {
    /// Creates an idle async I/O slot bound to `ctx`.
    pub fn new(ctx: &Arc<Context>) -> Result<Self> {
        let fence = ctx.new_event_with_flags(None)?;
        Ok(Self {
            ctx: ctx.clone(),
            params: Box::new(AsyncParams { size: 0, file_offset: 0, buf_offset: 0, bytes_transferred: 0 }),
            fence,
            fence_armed: false,
            pending_call: None,
            posix_bytes: None,
            pending_posix_write: None,
            in_flight: None,
            staging: None,
        })
    }

    fn wait_fence_if_armed(&mut self) -> Result<()> {
        if self.fence_armed {
            self.fence.synchronize()?;
            self.fence_armed = false;
        }
        Ok(())
    }

    fn finish_prior_before_reuse(&mut self) -> Result<()> {
        self.wait_fence_if_armed()?;
        let _ = self.harvest_result()?;
        Ok(())
    }

    fn ensure_staging(&mut self, len: usize) -> Result<&mut PinnedHostBuffer<u8>> {
        let need_alloc = match &self.staging {
            Some(buf) => buf.as_slice().len() < len,
            None => true,
        };
        if need_alloc {
            let cap = len.max(4096);
            self.staging = Some(PinnedHostBuffer::zeroed(&self.ctx, cap)?);
        }
        self.staging
            .as_mut()
            .ok_or(Error::Invalid("pinned staging buffer missing after allocation"))
    }

    /// Non-blocking check for async operation completion; returns `Ok(None)` while still in flight.
    pub fn try_complete(&mut self) -> Result<Option<usize>> {
        if self.fence_armed && !self.fence.query()? {
            return Ok(None);
        }
        self.fence_armed = false;
        self.harvest_result()
    }

    /// Blocks until the pending async transfer finishes on its CUDA stream and returns the
    /// transferred byte count (or surfaces any deferred GDS / OS error).
    pub fn wait(&mut self) -> Result<usize> {
        self.wait_fence_if_armed()?;
        self.harvest_result()?.ok_or(Error::Invalid("no async I/O operation pending in slot"))
    }

    fn harvest_result(&mut self) -> Result<Option<usize>> {
        if let Some(pw) = self.pending_posix_write.take() {
            let pins = self.in_flight.take();
            let fh = pins.ok_or(Error::Invalid("missing FileHandle pin for pending async write"))?;
            let staging = self
                .staging
                .as_ref()
                .ok_or(Error::Invalid("missing staging buffer for pending async write"))?;
            let n = posix_write_at(&fh.fh.file, pw.file_offset, &staging.as_slice()[..pw.len])?;
            self.pending_call = None;
            return Ok(Some(n));
        }
        self.in_flight = None;
        if let Some(bytes) = self.posix_bytes.take() {
            self.pending_call = None;
            return Ok(Some(bytes));
        }
        let Some(call) = self.pending_call.take() else {
            return Ok(None);
        };
        decode_io_ret(call, self.params.bytes_transferred).map(Some)
    }
}

impl Drop for AsyncIoSlot {
    fn drop(&mut self) {
        if self.fence_armed {
            // Wait for GPU stream completion before flushing pending writes or releasing pins/params.
            if let Err(err) = self.fence.synchronize() {
                self.ctx.record_err(Err::<(), _>(err));
            }
            self.fence_armed = false;
        }
        if let Some(pw) = self.pending_posix_write.take()
            && let (Some(pins), Some(staging)) = (&self.in_flight, &self.staging)
            && let Err(_io_err) =
                posix_write_at(&pins.fh.file, pw.file_offset, &staging.as_slice()[..pw.len])
        {
            record_teardown_driver_err(
                &self.ctx,
                cuda_bindings::cudaError_enum_CUDA_ERROR_UNKNOWN,
            );
        }
        self.in_flight = None;
    }
}

/// Registered file handle for synchronous and stream-ordered GPU file I/O.
///
/// Managed via [`Arc<FileHandle>`] so [`FileHandle::read_async`] and [`FileHandle::write_async`]
/// prevent handle closure or `cuFileHandleDeregister` while stream I/O is in flight, and enforces
/// [`IoMode`] strictly (never silently switching from GDS to POSIX).
pub struct FileHandle {
    ctx: Arc<Context>,
    mode: IoMode,
    #[cfg(target_os = "linux")]
    raw: Option<cufile_sys::CUfileHandle_t>,
    file: std::fs::File,
    posix_staging: Mutex<Option<PinnedHostBuffer<u8>>>,
    _no_sync_raw: PhantomData<*mut c_void>,
}

// SAFETY: `FileHandle` synchronizes `posix_staging` via `Mutex`, uses immutable file/GDS handles
// for concurrent I/O, binds `Context` before FFI, and only clears `raw` under exclusive ownership.
unsafe impl Send for FileHandle {}
// SAFETY: all interior state (`Mutex`, immutable `File` fd, thread-safe cuFile handle) is `Sync`.
unsafe impl Sync for FileHandle {}

impl FileHandle {
    /// Registers an already-open `file` according to `mode`, returning a pin-safe [`Arc<Self>`].
    ///
    /// For [`IoMode::GdsOnly`], callers on Linux should generally open `file` with `O_DIRECT` when
    /// targeting kernel-bypass NVFS; if registration fails, this returns an explicit [`Error`] and
    /// never downgrades to POSIX.
    pub fn register(ctx: &Arc<Context>, file: std::fs::File, mode: IoMode) -> Result<Arc<Self>> {
        ctx.bind_to_thread()?;
        match mode {
            IoMode::GdsOnly => {
                #[cfg(target_os = "linux")]
                {
                    use std::os::fd::AsRawFd;
                    cufile_sys::load()?;
                    // SAFETY: `CUfileDescr_t` is a plain C union/struct zero-initialized before populating fields.
                    let mut descr: cufile_sys::CUfileDescr_t = unsafe { std::mem::zeroed() };
                    descr.type_ = cufile_sys::CUfileFileHandleType_CU_FILE_HANDLE_TYPE_OPAQUE_FD;
                    descr.handle.fd = file.as_raw_fd();
                    descr.fs_ops = std::ptr::null();
                    let mut raw: cufile_sys::CUfileHandle_t = std::ptr::null_mut();
                    // SAFETY: `raw` receives the handle and `descr` describes `file`'s valid live fd.
                    let status = unsafe { cufile_sys::cuFileHandleRegister(&mut raw, &mut descr) };
                    check_status("cuFileHandleRegister", status)?;
                    Ok(Arc::new(Self {
                        ctx: ctx.clone(),
                        mode,
                        raw: Some(raw),
                        file,
                        posix_staging: Mutex::new(None),
                        _no_sync_raw: PhantomData,
                    }))
                }
                #[cfg(not(target_os = "linux"))]
                {
                    let _ = file;
                    Err(Error::Unsupported(
                        "cuFile IoMode::GdsOnly requires Linux; select IoMode::PosixFallback explicitly if desired",
                    ))
                }
            }
            IoMode::PosixFallback => Ok(Arc::new(Self {
                ctx: ctx.clone(),
                mode,
                #[cfg(target_os = "linux")]
                raw: None,
                file,
                posix_staging: Mutex::new(None),
                _no_sync_raw: PhantomData,
            })),
        }
    }

    /// Configured explicit execution mode.
    pub const fn mode(&self) -> IoMode {
        self.mode
    }

    /// Underlying OS file handle.
    pub const fn file(&self) -> &std::fs::File {
        &self.file
    }

    /// Explicitly deregisters the cuFile handle (`cuFileHandleDeregister`) and returns the
    /// underlying [`std::fs::File`].
    ///
    /// Fails with [`Error::Invalid`] if any [`AsyncIoSlot`] still holds an in-flight pin on this
    /// handle.
    pub fn deregister(this: Arc<Self>) -> Result<std::fs::File> {
        let this = Arc::try_unwrap(this)
            .map_err(|_| Error::Invalid("file handle still referenced by in-flight async I/O"))?;
        this.ctx.bind_to_thread()?;
        #[cfg(target_os = "linux")]
        let mut this = this;
        #[cfg(target_os = "linux")]
        if let Some(raw) = this.raw.take() {
            // SAFETY: `raw` was registered by `cuFileHandleRegister` and `this.file` is still open.
            unsafe { cufile_sys::cuFileHandleDeregister(raw)? };
        }
        let mut me = std::mem::ManuallyDrop::new(this);
        // SAFETY: `me.raw` was already taken and `me.file` is read once before dropping the rest.
        let file = unsafe { std::ptr::read(&me.file) };
        // SAFETY: drop remaining initialized fields except `me.file`.
        unsafe {
            std::ptr::drop_in_place(&mut me.ctx);
            std::ptr::drop_in_place(&mut me.posix_staging);
        }
        Ok(file)
    }

    fn validate_transfer<D: DeviceSpan + ?Sized>(
        &self,
        file_offset: u64,
        buf: &D,
        buf_offset: usize,
        len: usize,
    ) -> Result<(i64, i64)> {
        if !Arc::ptr_eq(buf.context(), &self.ctx) {
            return Err(Error::Invalid("device buffer belongs to a different CUDA context"));
        }
        let end = buf_offset
            .checked_add(len)
            .ok_or(Error::Invalid("buffer offset + transfer length overflow"))?;
        if end > buf.num_bytes() {
            return Err(Error::Invalid("transfer range exceeds device buffer size"));
        }
        let file_off =
            i64::try_from(file_offset).map_err(|_| Error::Invalid("file_offset exceeds off_t range"))?;
        let buf_off =
            i64::try_from(buf_offset).map_err(|_| Error::Invalid("buf_offset exceeds off_t range"))?;
        Ok((file_off, buf_off))
    }

    fn with_sync_staging<R>(
        &self,
        len: usize,
        f: impl FnOnce(&mut PinnedHostBuffer<u8>) -> Result<R>,
    ) -> Result<R> {
        let mut guard = self
            .posix_staging
            .lock()
            .map_err(|_| Error::Invalid("cuFile POSIX staging mutex poisoned"))?;
        let need_alloc = match guard.as_ref() {
            Some(buf) => buf.as_slice().len() < len,
            None => true,
        };
        if need_alloc {
            let cap = len.max(4096);
            *guard = Some(PinnedHostBuffer::zeroed(&self.ctx, cap)?);
        }
        let buf = guard
            .as_mut()
            .ok_or(Error::Invalid("pinned staging buffer missing after allocation"))?;
        f(buf)
    }

    /// Synchronously reads up to `len` bytes from `file_offset` into `dst` at `buf_offset`.
    pub fn read<D: DeviceSpanMut + ?Sized>(
        &self,
        file_offset: u64,
        dst: &D,
        buf_offset: usize,
        len: usize,
    ) -> Result<usize> {
        let (file_off, buf_off) = self.validate_transfer(file_offset, dst, buf_offset, len)?;
        if len == 0 {
            return Ok(0);
        }
        self.ctx.bind_to_thread()?;

        match self.mode {
            IoMode::GdsOnly => {
                #[cfg(target_os = "linux")]
                {
                    let raw = self.raw.ok_or(Error::Invalid("cuFile handle already deregistered"))?;
                    // SAFETY: `raw` is registered and `buf_off..buf_off+len` is within `dst`.
                    let ret = unsafe {
                        cufile_sys::cuFileRead(
                            raw,
                            (dst.cu_deviceptr() as usize) as *mut c_void,
                            len,
                            file_off,
                            buf_off,
                        )
                    };
                    decode_io_ret("cuFileRead", ret?)
                }
                #[cfg(not(target_os = "linux"))]
                {
                    let _ = (file_off, buf_off);
                    Err(Error::Unsupported("cuFileRead requires Linux"))
                }
            }
            IoMode::PosixFallback => {
                let _ = (file_off, buf_off);
                let dst_ptr = dst
                    .cu_deviceptr()
                    .checked_add(buf_offset as u64)
                    .ok_or(Error::Invalid("device pointer offset overflow"))?;
                self.with_sync_staging(len, |staging| {
                    let n = posix_read_at(&self.file, file_offset, &mut staging.as_mut_slice()[..len])?;
                    if n > 0 {
                        // SAFETY: `dst_ptr..dst_ptr+n` is inside `dst` and `staging` holds `n` initialized bytes.
                        unsafe { cuda_core::memory::memcpy_htod_sync(dst_ptr, staging.as_ptr(), n)? };
                    }
                    Ok(n)
                })
            }
        }
    }

    /// Synchronously writes up to `len` bytes from `src` at `buf_offset` to `file_offset`.
    pub fn write<D: DeviceSpan + ?Sized>(
        &self,
        file_offset: u64,
        src: &D,
        buf_offset: usize,
        len: usize,
    ) -> Result<usize> {
        let (file_off, buf_off) = self.validate_transfer(file_offset, src, buf_offset, len)?;
        if len == 0 {
            return Ok(0);
        }
        self.ctx.bind_to_thread()?;

        match self.mode {
            IoMode::GdsOnly => {
                #[cfg(target_os = "linux")]
                {
                    let raw = self.raw.ok_or(Error::Invalid("cuFile handle already deregistered"))?;
                    // SAFETY: `raw` is registered and `buf_off..buf_off+len` is within `src`.
                    let ret = unsafe {
                        cufile_sys::cuFileWrite(
                            raw,
                            (src.cu_deviceptr() as usize) as *const c_void,
                            len,
                            file_off,
                            buf_off,
                        )
                    };
                    decode_io_ret("cuFileWrite", ret?)
                }
                #[cfg(not(target_os = "linux"))]
                {
                    let _ = (file_off, buf_off);
                    Err(Error::Unsupported("cuFileWrite requires Linux"))
                }
            }
            IoMode::PosixFallback => {
                let _ = (file_off, buf_off);
                let src_ptr = src
                    .cu_deviceptr()
                    .checked_add(buf_offset as u64)
                    .ok_or(Error::Invalid("device pointer offset overflow"))?;
                self.with_sync_staging(len, |staging| {
                    // SAFETY: `src_ptr..src_ptr+len` is within `src` and `staging` has capacity `>= len`.
                    unsafe {
                        cuda_core::IntoResult::result(cuda_bindings::cuMemcpyDtoH_v2(
                            staging.as_mut_ptr().cast::<c_void>(),
                            src_ptr,
                            len,
                        ))?;
                    }
                    Ok(posix_write_at(&self.file, file_offset, &staging.as_slice()[..len])?)
                })
            }
        }
    }

    /// Enqueues a non-blocking, stream-ordered asynchronous read into `dst`, retaining strong
    /// `Arc` pins on `self`, `dst`, and `stream` inside `slot` until [`AsyncIoSlot::wait`] or
    /// [`AsyncIoSlot::try_complete`] finishes.
    pub fn read_async<D: DeviceSpanMut + Send + Sync + 'static>(
        self: &Arc<Self>,
        stream: &Arc<Stream>,
        file_offset: u64,
        dst: &Arc<D>,
        buf_offset: usize,
        len: usize,
        slot: &mut AsyncIoSlot,
    ) -> Result<()> {
        if !Arc::ptr_eq(stream.context(), &self.ctx) {
            return Err(Error::Invalid("stream belongs to a different CUDA context"));
        }
        if !Arc::ptr_eq(&slot.ctx, &self.ctx) {
            return Err(Error::Invalid("AsyncIoSlot belongs to a different CUDA context"));
        }
        let (file_off, buf_off) = self.validate_transfer(file_offset, dst.as_ref(), buf_offset, len)?;
        slot.finish_prior_before_reuse()?;

        if len == 0 {
            slot.posix_bytes = Some(0);
            return Ok(());
        }
        self.ctx.bind_to_thread()?;

        match self.mode {
            IoMode::GdsOnly => {
                #[cfg(target_os = "linux")]
                {
                    let raw = self.raw.ok_or(Error::Invalid("cuFile handle already deregistered"))?;
                    slot.params.size = len;
                    slot.params.file_offset = file_off;
                    slot.params.buf_offset = buf_off;
                    slot.params.bytes_transferred = 0;
                    let p = &mut *slot.params;
                    // SAFETY: `p` lives inside `slot.params` (`Box<AsyncParams>`), and `slot.in_flight`
                    // + `slot.fence` keep `self`, `dst`, `stream`, and `p` valid until completion.
                    let status = unsafe {
                        cufile_sys::cuFileReadAsync(
                            raw,
                            (dst.cu_deviceptr() as usize) as *mut c_void,
                            &mut p.size,
                            &mut p.file_offset,
                            &mut p.buf_offset,
                            &mut p.bytes_transferred,
                            stream.cu_stream(),
                        )
                    };
                    check_status("cuFileReadAsync", status)?;
                    slot.fence.record(stream)?;
                    slot.fence_armed = true;
                    slot.pending_call = Some("cuFileReadAsync");
                    slot.in_flight = Some(InFlightPins {
                        fh: self.clone(),
                        _buf: dst.clone(),
                        _stream: stream.clone(),
                    });
                    Ok(())
                }
                #[cfg(not(target_os = "linux"))]
                {
                    let _ = (file_off, buf_off);
                    Err(Error::Unsupported("cuFileReadAsync requires Linux"))
                }
            }
            IoMode::PosixFallback => {
                let dst_ptr = dst
                    .cu_deviceptr()
                    .checked_add(buf_offset as u64)
                    .ok_or(Error::Invalid("device pointer offset overflow"))?;
                let staging = slot.ensure_staging(len)?;
                let n = posix_read_at(&self.file, file_offset, &mut staging.as_mut_slice()[..len])?;
                if n > 0 {
                    // SAFETY: `dst_ptr..dst_ptr+n` is valid and `slot.fence` + `slot.in_flight`
                    // keep `dst`, `stream`, and `staging` alive until async HtoD DMA completes.
                    unsafe {
                        cuda_core::memory::memcpy_htod_async(
                            dst_ptr,
                            staging.as_ptr(),
                            n,
                            stream.cu_stream(),
                        )?;
                    }
                    slot.fence.record(stream)?;
                    slot.fence_armed = true;
                    slot.in_flight = Some(InFlightPins {
                        fh: self.clone(),
                        _buf: dst.clone(),
                        _stream: stream.clone(),
                    });
                }
                slot.posix_bytes = Some(n);
                Ok(())
            }
        }
    }

    /// Enqueues a non-blocking, stream-ordered asynchronous write from `src`, retaining strong
    /// `Arc` pins on `self`, `src`, and `stream` inside `slot` until [`AsyncIoSlot::wait`] or
    /// [`AsyncIoSlot::try_complete`] finishes.
    ///
    /// Under explicit [`IoMode::PosixFallback`], GPU-to-pinned DtoH DMA runs asynchronously on
    /// `stream`, and the final `posix_write_at` executes non-blockingly when [`AsyncIoSlot::wait`]
    /// or [`AsyncIoSlot::try_complete`] observes event completion.
    pub fn write_async<D: DeviceSpan + Send + Sync + 'static>(
        self: &Arc<Self>,
        stream: &Arc<Stream>,
        file_offset: u64,
        src: &Arc<D>,
        buf_offset: usize,
        len: usize,
        slot: &mut AsyncIoSlot,
    ) -> Result<()> {
        if !Arc::ptr_eq(stream.context(), &self.ctx) {
            return Err(Error::Invalid("stream belongs to a different CUDA context"));
        }
        if !Arc::ptr_eq(&slot.ctx, &self.ctx) {
            return Err(Error::Invalid("AsyncIoSlot belongs to a different CUDA context"));
        }
        let (file_off, buf_off) = self.validate_transfer(file_offset, src.as_ref(), buf_offset, len)?;
        slot.finish_prior_before_reuse()?;

        if len == 0 {
            slot.posix_bytes = Some(0);
            return Ok(());
        }
        self.ctx.bind_to_thread()?;

        match self.mode {
            IoMode::GdsOnly => {
                #[cfg(target_os = "linux")]
                {
                    let raw = self.raw.ok_or(Error::Invalid("cuFile handle already deregistered"))?;
                    slot.params.size = len;
                    slot.params.file_offset = file_off;
                    slot.params.buf_offset = buf_off;
                    slot.params.bytes_transferred = 0;
                    let p = &mut *slot.params;
                    // SAFETY: `p` lives inside `slot.params` (`Box<AsyncParams>`), and `slot.in_flight`
                    // + `slot.fence` keep `self`, `src`, `stream`, and `p` valid until completion.
                    let status = unsafe {
                        cufile_sys::cuFileWriteAsync(
                            raw,
                            (src.cu_deviceptr() as usize) as *mut c_void,
                            &mut p.size,
                            &mut p.file_offset,
                            &mut p.buf_offset,
                            &mut p.bytes_transferred,
                            stream.cu_stream(),
                        )
                    };
                    check_status("cuFileWriteAsync", status)?;
                    slot.fence.record(stream)?;
                    slot.fence_armed = true;
                    slot.pending_call = Some("cuFileWriteAsync");
                    slot.in_flight = Some(InFlightPins {
                        fh: self.clone(),
                        _buf: src.clone(),
                        _stream: stream.clone(),
                    });
                    Ok(())
                }
                #[cfg(not(target_os = "linux"))]
                {
                    let _ = (file_off, buf_off);
                    Err(Error::Unsupported("cuFileWriteAsync requires Linux"))
                }
            }
            IoMode::PosixFallback => {
                let src_ptr = src
                    .cu_deviceptr()
                    .checked_add(buf_offset as u64)
                    .ok_or(Error::Invalid("device pointer offset overflow"))?;
                let staging = slot.ensure_staging(len)?;
                // SAFETY: `src_ptr..src_ptr+len` is valid device memory; `slot.fence` and
                // `slot.in_flight` keep `src`, `stream`, and `staging` live until DtoH DMA finishes.
                unsafe {
                    cuda_core::memory::memcpy_dtoh_async(
                        staging.as_mut_ptr(),
                        src_ptr,
                        len,
                        stream.cu_stream(),
                    )?;
                }
                slot.fence.record(stream)?;
                slot.fence_armed = true;
                slot.pending_posix_write = Some(PendingPosixWrite { file_offset, len });
                slot.in_flight = Some(InFlightPins {
                    fh: self.clone(),
                    _buf: src.clone(),
                    _stream: stream.clone(),
                });
                Ok(())
            }
        }
    }
}

impl Drop for FileHandle {
    fn drop(&mut self) {
        #[cfg(target_os = "linux")]
        if let Some(raw) = self.raw.take() {
            if let Err(err) = self.ctx.bind_to_thread() {
                self.ctx.record_err(Err::<(), _>(err));
                return;
            }
            // SAFETY: `raw` was opened via `cuFileHandleRegister` and `self.file` is still live.
            if let Err(_err) = unsafe { cufile_sys::cuFileHandleDeregister(raw) } {
                record_teardown_driver_err(&self.ctx, cuda_bindings::cudaError_enum_CUDA_ERROR_UNKNOWN);
            }
        }
    }
}

/// Extension wrapper over a raw [`DeviceBufferView`] allowing `RegisteredBuffer` construction on a
/// subslice without transferring ownership.
pub fn as_device_view<'a, T>(view: &'a DeviceBufferView<'a, T>) -> &'a DeviceBufferView<'a, T> {
    view
}

fn posix_read_at(file: &std::fs::File, offset: u64, dst: &mut [u8]) -> std::io::Result<usize> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::FileExt;
        file.read_at(dst, offset)
    }
    #[cfg(windows)]
    {
        use std::os::windows::fs::FileExt;
        file.seek_read(dst, offset)
    }
}

fn posix_write_at(file: &std::fs::File, offset: u64, src: &[u8]) -> std::io::Result<usize> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::FileExt;
        file.write_at(src, offset)
    }
    #[cfg(windows)]
    {
        use std::os::windows::fs::FileExt;
        file.seek_write(src, offset)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t0_flags_and_posix_offset_io_invariants() {
        let bflags = BufRegisterFlags::RDMA_REGISTER.union(BufRegisterFlags::RDMA_RELAXED_ORDERING);
        assert_eq!(bflags.bits(), 3);
        assert_eq!(BufRegisterFlags::NONE.bits(), 0);

        let sflags = StreamFlags::FIXED_BUF_OFFSET
            .union(StreamFlags::FIXED_FILE_OFFSET)
            .union(StreamFlags::FIXED_FILE_SIZE)
            .union(StreamFlags::PAGE_ALIGNED_INPUTS);
        assert_eq!(sflags.bits(), 1 | 2 | 4 | 8);

        let path = std::env::temp_dir().join("nv_rs_cufile_t0_posix_rw.bin");
        let file = std::fs::OpenOptions::new()
            .create(true)
            .truncate(true)
            .read(true)
            .write(true)
            .open(&path)
            .unwrap();

        let data = b"gpudirect-fallback-check";
        let wrote = posix_write_at(&file, 16, data).unwrap();
        assert_eq!(wrote, data.len());

        let mut read_back = vec![0u8; data.len()];
        let read = posix_read_at(&file, 16, &mut read_back).unwrap();
        assert_eq!(read, data.len());
        assert_eq!(&read_back, data);

        drop(file);
        let _ = std::fs::remove_file(path);
    }
}
