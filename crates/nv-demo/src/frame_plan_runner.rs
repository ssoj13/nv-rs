//! Generic CUDA/NVRTC executor for [`csg_scene::frame_plan::FramePlan`].
//!
//! What: compiles and validates any declarative `csg_scene::frame_plan::FramePlan` (checking its
//! version, structural invariants, FNV-1a source hash, and every kernel's `extern "C"` parameter
//! list against the plan's argument list), manages grow-only VRAM buffers for all owned buffer
//! classes (`PerFrameUpload`, `Table`, `DeviceState`, `DeviceScratch`), and launches raised event
//! lists followed by `plan.frame` on one CUDA stream with zero mid-frame synchronization and zero
//! per-launch heap allocations.
//!
//! Why: hand-copied launch lists in render hosts drift from `csg-rs`'s simulation plans (missing
//! kernels, reordered arguments, or stale buffer layouts). Executing the exact `FramePlan` data
//! structure shared with `csg-cuda` guarantees structural parity across `PlanKind::Crowd`,
//! `PlanKind::Camera`, and future GPU plans while keeping steady-state frame execution allocation-
//! free.
//!
//! How: `GpuFramePlan::new` verifies the plan against `extern_c_signatures(&plan.source)`, loads
//! the NVRTC-compiled `CUmodule` through the content-addressed disk PTX cache, and caches every
//! `CUfunction`. Owned buffers grow in power-of-two byte steps and track table upload generations
//! (`table_needs_upload` / `mark_table_uploaded`) so cold scene tables upload only on change.
//! `execute` packs kernel arguments into a fixed stack buffer and dispatches `cuLaunchKernel`
//! directly on the caller's stream. Small every-frame status readbacks (`ReadbackWhen::Always`)
//! use page-locked [`PinnedHostBuf`] transfers (`copy_to_pinned_async`).
//!
//! Uses: `csg_scene::frame_plan`, `cuda_bindings`, `cuda_core`, `nvrtc`, `bytemuck`,
//! `crate::gpu_export::load_ptx_cached`.
//! Used by: `crate::cuda_crowd`, `crate::cuda_camera`.

use std::collections::BTreeMap;
use std::ffi::{CString, c_void};
use std::sync::Arc;

use bytemuck::Pod;
use csg_scene::frame_plan::{
    Arg, BufClass, BufId, EventId, ExternalId, FramePlan, FrameScalars, FrameSizes, KernelLaunch,
    PlanKind, ScalarValue, extern_c_signatures,
};
use cuda_bindings as cu;
use cuda_core::{Context as CudaContext, DeviceBuffer, Stream};

/// Maximum kernel parameter count supported on the stack in [`GpuFramePlan::execute`] (zero heap
/// allocations per launch; largest current plan kernel takes 23 parameters).
const MAX_STACK_KERNEL_ARGS: usize = 64;

/// Compiles CUDA C++ `source` via NVRTC (`compute_75`, C++17) under `math_mode` and returns a
/// NUL-terminated PTX byte buffer ready for `cuModuleLoadData`.
///
/// Use [`nvrtc::MathMode::StrictF32`] for verification or polynomial root-finding kernels that
/// must preserve IEEE-754 subnormals and exact division/square-root rounding without
/// `--use_fast_math`.
pub fn compile_nvrtc_ptx_with_math(
    source: &str,
    name: &str,
    math_mode: nvrtc::MathMode,
) -> Result<Vec<u8>, String> {
    let opts = nvrtc::CompileOptions::new()
        .arch("compute_75")
        .std("c++17")
        .math_mode(math_mode);
    let mut prog = nvrtc::Program::new(source, Some(name), &[])
        .map_err(|e| format!("NVRTC program creation failed for {name}: {e}"))?;
    prog.compile(&opts)
        .map_err(|e| format!("NVRTC compile failed for {name}: {e}"))?;
    let mut ptx = prog
        .ptx()
        .map_err(|e| format!("NVRTC PTX retrieval failed for {name}: {e}"))?;
    if ptx.last() != Some(&0) {
        ptx.push(0);
    }
    Ok(ptx)
}

/// Compiles CUDA C++ `source` via NVRTC (`compute_75`, C++17, [`nvrtc::MathMode::Fast`]) and
/// returns a NUL-terminated PTX byte buffer ready for `cuModuleLoadData`.
///
/// Shared helper for standalone host bridge kernels (such as `cuda_crowd`'s scatter bridge) that
/// compile a small CUDA unit without going through a `FramePlan`.
pub fn compile_nvrtc_ptx(source: &str, name: &str) -> Result<Vec<u8>, String> {
    compile_nvrtc_ptx_with_math(source, name, nvrtc::MathMode::Fast)
}

/// Resolves an `extern "C"` kernel symbol `name` inside a loaded `CUmodule`.
pub fn load_cuda_fn(module: cu::CUmodule, name: &str) -> Result<cu::CUfunction, String> {
    let cname = CString::new(name).map_err(|e| format!("CString {name} failed: {e}"))?;
    let mut func: cu::CUfunction = std::ptr::null_mut();
    // SAFETY: `module` is a valid loaded `CUmodule` on the currently bound CUDA context and
    // `cname` is a valid NUL-terminated string.
    let rc = unsafe { cu::cuModuleGetFunction(&mut func, module, cname.as_ptr()) };
    if rc != cu::cudaError_enum_CUDA_SUCCESS || func.is_null() {
        return Err(format!("cuModuleGetFunction({name}) failed ({rc:?})"));
    }
    Ok(func)
}

/// Page-locked CUDA host buffer holding `len` elements of `T: Pod` for zero-allocation async D2H
/// readbacks (`ReadbackWhen::Always` status words, displacement flags, and 64 B camera records).
pub struct PinnedHostBuf<T: Pod> {
    ptr: *mut T,
    len: usize,
    cuda: Arc<CudaContext>,
}

// SAFETY: Page-locked CUDA host memory belongs to `self.cuda`; exclusive mutation is enforced by
// `&mut self`, and `T: Pod + Send` permits moving across threads safely.
unsafe impl<T: Pod + Send> Send for PinnedHostBuf<T> {}

impl<T: Pod> PinnedHostBuf<T> {
    /// Allocates and zero-initializes a page-locked host buffer of `len` (`>= 1`) elements of `T`.
    pub fn new(cuda: &Arc<CudaContext>, len: usize) -> Result<Self, String> {
        if len == 0 {
            return Err("PinnedHostBuf length must be >= 1".to_string());
        }
        let bytes = len
            .checked_mul(std::mem::size_of::<T>())
            .ok_or_else(|| "PinnedHostBuf byte size overflow".to_string())?;
        if bytes == 0 {
            return Err("PinnedHostBuf does not support zero-sized types".to_string());
        }
        cuda.bind_to_thread()
            .map_err(|e| format!("CUDA bind failed for PinnedHostBuf: {e}"))?;
        let mut raw: *mut c_void = std::ptr::null_mut();
        // SAFETY: `cuda` is bound on this thread, `raw` is a valid out-pointer, and `bytes > 0`.
        let rc = unsafe { cu::cuMemAllocHost_v2(&mut raw, bytes) };
        if rc != cu::cudaError_enum_CUDA_SUCCESS || raw.is_null() {
            return Err(format!("cuMemAllocHost_v2({bytes} B) failed ({rc:?})"));
        }
        // SAFETY: `raw` points to `bytes` writable host bytes owned exclusively by `Self`, and
        // `T: Pod` guarantees all-zero bytes form valid `T` values.
        unsafe {
            std::ptr::write_bytes(raw.cast::<u8>(), 0, bytes);
        }
        Ok(Self {
            ptr: raw.cast::<T>(),
            len,
            cuda: cuda.clone(),
        })
    }

    /// Number of elements of `T` in this page-locked allocation.
    #[inline]
    pub fn capacity(&self) -> usize {
        self.len
    }

    /// Writable pointer to element `0` for `cuMemcpyDtoHAsync_v2`.
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.ptr
    }

    /// Writes `value` into `index` using a volatile store so the compiler never elides host
    /// initialization prior to an async DMA.
    pub fn write(&mut self, index: usize, value: T) -> Result<(), String> {
        if index >= self.len {
            return Err(format!(
                "PinnedHostBuf::write index {index} out of bounds (len {})",
                self.len
            ));
        }
        // SAFETY: `index < self.len`, `self.ptr` is properly aligned and valid for `self.len`
        // elements of `T`, and `&mut self` guarantees exclusive access.
        unsafe {
            std::ptr::write_volatile(self.ptr.add(index), value);
        }
        Ok(())
    }

    /// Reads element `index` using a volatile load; caller must ensure any preceding async DMA
    /// targeting this buffer has completed on its stream.
    pub fn read(&self, index: usize) -> Result<T, String> {
        if index >= self.len {
            return Err(format!(
                "PinnedHostBuf::read index {index} out of bounds (len {})",
                self.len
            ));
        }
        // SAFETY: `index < self.len` and `self.ptr` is valid and initialized for `self.len`
        // elements of `T`.
        let val = unsafe { std::ptr::read_volatile(self.ptr.add(index)) };
        Ok(val)
    }
}

impl<T: Pod> Drop for PinnedHostBuf<T> {
    fn drop(&mut self) {
        if !self.ptr.is_null() && self.cuda.bind_to_thread().is_ok() {
            // SAFETY: `self.ptr` was allocated via `cuMemAllocHost_v2` on `self.cuda` and is freed
            // exactly once on drop.
            let rc = unsafe { cu::cuMemFreeHost(self.ptr.cast()) };
            if rc != cu::cudaError_enum_CUDA_SUCCESS {
                eprintln!("nv-demo: PinnedHostBuf cuMemFreeHost failed ({rc:?})");
            }
            self.ptr = std::ptr::null_mut();
        }
    }
}

/// One grow-only device buffer slot managed by [`GpuFramePlan`].
struct DeviceSlot {
    buf: Option<DeviceBuffer<u8>>,
    cap_bytes: usize,
    len_elems: u32,
    elem_size: u32,
    uploaded_gen: Option<u64>,
}

impl DeviceSlot {
    fn new(elem_size: u32) -> Self {
        Self {
            buf: None,
            cap_bytes: 0,
            len_elems: 0,
            elem_size,
            uploaded_gen: None,
        }
    }

    #[inline]
    fn device_ptr(&self) -> Option<u64> {
        self.buf.as_ref().map(DeviceBuffer::cu_deviceptr)
    }

    /// Ensures the slot owns a non-null device allocation of at least `max(1, elems) * elem_size`
    /// bytes, growing in power-of-two byte steps to eliminate steady-state VRAM reallocations.
    fn ensure_elems(&mut self, stream: &Stream, id: BufId, elems: u32) -> Result<(), String> {
        let min_elems = elems.max(1) as usize;
        let need_bytes = min_elems
            .checked_mul(self.elem_size as usize)
            .ok_or_else(|| format!("{id:?}: byte size overflow ({elems} x {})", self.elem_size))?;
        if need_bytes > self.cap_bytes || self.buf.is_none() {
            let alloc_bytes = need_bytes.max(256).next_power_of_two();
            let new_buf = DeviceBuffer::<u8>::zeroed(stream, alloc_bytes)
                .map_err(|e| format!("{id:?}: CUDA buffer alloc ({alloc_bytes} B) failed: {e}"))?;
            self.buf = Some(new_buf);
            self.cap_bytes = alloc_bytes;
            // A fresh zeroed allocation invalidated any previously uploaded table contents.
            self.uploaded_gen = None;
        }
        self.len_elems = elems;
        Ok(())
    }
}

/// Stack-allocated value cell holding one resolved CUDA kernel argument without heap allocation
/// or endianness assumptions.
#[derive(Clone, Copy, Default)]
struct ArgCell {
    ptr: u64,
    u32_val: u32,
    f32_val: f32,
}

/// Reusable CUDA runner executing any [`csg_scene::frame_plan::FramePlan`] on a shared stream.
pub struct GpuFramePlan {
    cuda: Arc<CudaContext>,
    plan: FramePlan,
    math_mode: nvrtc::MathMode,
    module: cu::CUmodule,
    kernels: BTreeMap<&'static str, cu::CUfunction>,
    slots: BTreeMap<BufId, DeviceSlot>,
}

// SAFETY: `CUmodule` and `CUfunction` belong to `self.cuda`, `DeviceBuffer<u8>` is `Send`, and
// `Drop` binds `self.cuda` before unloading `self.module`.
unsafe impl Send for GpuFramePlan {}

impl GpuFramePlan {
    /// Validates `plan` against `expected_version`, its own FNV-1a source hash, and the parsed
    /// `extern "C"` kernel signatures in `plan.source`, compiles/loads its PTX module with
    /// [`nvrtc::MathMode::Fast`], caches all referenced `CUfunction`s, and initializes grow-only
    /// device slots for every owned buffer.
    pub fn new(
        cuda: &Arc<CudaContext>,
        plan: FramePlan,
        expected_version: u32,
        unit_name: &str,
    ) -> Result<Self, String> {
        Self::new_with_math(cuda, plan, expected_version, unit_name, nvrtc::MathMode::Fast)
    }

    /// Like [`Self::new`] with explicit [`nvrtc::MathMode`] (`Fast` vs `StrictF32`) so
    /// verification and root-finding plans compile with IEEE-754 precision into their own PTX
    /// cache slot.
    pub fn new_with_math(
        cuda: &Arc<CudaContext>,
        plan: FramePlan,
        expected_version: u32,
        unit_name: &str,
        math_mode: nvrtc::MathMode,
    ) -> Result<Self, String> {
        if plan.version != expected_version {
            return Err(format!(
                "FramePlan {:?} ({unit_name}) version mismatch: expected {expected_version}, got {}",
                plan.kind, plan.version
            ));
        }
        plan.validate().map_err(|e| {
            format!(
                "FramePlan {:?} ({unit_name}) validation failed: {e}",
                plan.kind
            )
        })?;
        plan.check_source(&plan.source).map_err(|e| {
            format!(
                "FramePlan {:?} ({unit_name}) source hash check failed: {e}",
                plan.kind
            )
        })?;

        // Verify every launch symbol exists in `plan.source` with exact arity and pointer/scalar
        // parameter agreement before compiling or launching anything on the GPU.
        let sigs: BTreeMap<String, Vec<bool>> = extern_c_signatures(&plan.source)
            .into_iter()
            .map(|(name, params)| (name, params.into_iter().map(|p| p.pointer).collect()))
            .collect();
        for launch in plan.all_launches() {
            let Some(param_is_ptr) = sigs.get(launch.kernel) else {
                return Err(format!(
                    "FramePlan {:?} ({unit_name}): kernel '{}' not found in plan source",
                    plan.kind, launch.kernel
                ));
            };
            if param_is_ptr.len() != launch.args.len() {
                return Err(format!(
                    "FramePlan {:?} ({unit_name}): kernel '{}' has {} CUDA params, plan passes {}",
                    plan.kind,
                    launch.kernel,
                    param_is_ptr.len(),
                    launch.args.len()
                ));
            }
            if launch.args.len() > MAX_STACK_KERNEL_ARGS {
                return Err(format!(
                    "FramePlan {:?} ({unit_name}): kernel '{}' has {} args (max {MAX_STACK_KERNEL_ARGS})",
                    plan.kind,
                    launch.kernel,
                    launch.args.len()
                ));
            }
            for (idx, (arg, &expects_ptr)) in
                launch.args.iter().zip(param_is_ptr.iter()).enumerate()
            {
                let is_ptr = matches!(arg, Arg::Buffer(_));
                if is_ptr != expects_ptr {
                    return Err(format!(
                        "FramePlan {:?} ({unit_name}): kernel '{}' arg {idx} pointer mismatch (plan ptr={is_ptr}, CUDA ptr={expects_ptr})",
                        plan.kind, launch.kernel
                    ));
                }
            }
        }

        cuda.bind_to_thread()
            .map_err(|e| format!("CUDA bind failed for FramePlan {:?} ({unit_name}): {e}", plan.kind))?;

        let module = crate::gpu_export::load_ptx_cached_with_math(
            &plan.source,
            unit_name,
            "compute_75",
            &[],
            &[],
            math_mode,
            |ptx| {
                let mut m: cu::CUmodule = std::ptr::null_mut();
                // SAFETY: `ptx` is a NUL-terminated PTX buffer and `cuda` is current on this thread.
                let rc = unsafe { cu::cuModuleLoadData(&mut m, ptx.as_ptr().cast()) };
                if rc != cu::cudaError_enum_CUDA_SUCCESS || m.is_null() {
                    return Err(format!("cuModuleLoadData({unit_name}) failed ({rc:?})"));
                }
                Ok(m)
            },
        )?;

        let mut kernels = BTreeMap::new();
        for launch in plan.all_launches() {
            if kernels.contains_key(launch.kernel) {
                continue;
            }
            match load_cuda_fn(module, launch.kernel) {
                Ok(func) => {
                    kernels.insert(launch.kernel, func);
                }
                Err(err) => {
                    // SAFETY: unloading valid `module` on error path prevents leaking the module.
                    let rc = unsafe { cu::cuModuleUnload(module) };
                    if rc != cu::cudaError_enum_CUDA_SUCCESS {
                        eprintln!(
                            "nv-demo: cuModuleUnload({unit_name}) on init error failed ({rc:?})"
                        );
                    }
                    return Err(err);
                }
            }
        }

        let mut slots = BTreeMap::new();
        for spec in plan.buffers {
            if matches!(spec.class, BufClass::External(_) | BufClass::Bound { .. }) {
                continue;
            }
            slots.insert(spec.id, DeviceSlot::new(spec.elem_size));
        }

        Ok(Self {
            cuda: cuda.clone(),
            plan,
            math_mode,
            module,
            kernels,
            slots,
        })
    }

    /// Ensures every owned (`PerFrameUpload`, `Table`, `DeviceState`, `DeviceScratch`) buffer has
    /// capacity for at least `max(1, elems) * elem_size` bytes using grow-only power-of-two
    /// allocations (zero reallocations in steady state).
    pub fn sync_sizes(&mut self, stream: &Stream, sizes: &dyn FrameSizes) -> Result<(), String> {
        for spec in self.plan.buffers {
            if matches!(spec.class, BufClass::External(_) | BufClass::Bound { .. }) {
                continue;
            }
            let elems = spec
                .count
                .resolve(sizes)
                .map_err(|e| format!("{:?} count resolution failed: {e}", spec.id))?;
            let slot = self
                .slots
                .get_mut(&spec.id)
                .ok_or_else(|| format!("{:?} slot missing from GpuFramePlan", spec.id))?;
            slot.ensure_elems(stream, spec.id, elems)?;
        }
        Ok(())
    }

    /// Returns `true` when table buffer `id` has never been uploaded, was reallocated, or was last
    /// uploaded for a different `generation`.
    #[inline]
    pub fn table_needs_upload(&self, id: BufId, generation: u64) -> bool {
        self.slots
            .get(&id)
            .is_none_or(|s| s.buf.is_none() || s.uploaded_gen != Some(generation))
    }

    /// Records that table buffer `id` now holds the contents for `generation`.
    #[inline]
    pub fn mark_table_uploaded(&mut self, id: BufId, generation: u64) {
        if let Some(slot) = self.slots.get_mut(&id) {
            slot.uploaded_gen = Some(generation);
        }
    }

    /// Uploads a typed slice `data` asynchronously into owned buffer `id` on `stream`, checking
    /// `size_of::<T>() == spec.elem_size` and keeping a non-null device pointer even when `data`
    /// is empty.
    pub fn upload<T: Pod>(
        &mut self,
        stream: &Stream,
        id: BufId,
        data: &[T],
    ) -> Result<(), String> {
        let elem_size = self
            .slots
            .get(&id)
            .ok_or_else(|| format!("{id:?} is not an owned buffer of {:?}", self.plan.kind))?
            .elem_size as usize;
        if std::mem::size_of::<T>() != elem_size {
            return Err(format!(
                "{id:?}: upload element size {} B != plan elem_size {elem_size} B",
                std::mem::size_of::<T>()
            ));
        }
        let elems = u32::try_from(data.len())
            .map_err(|_| format!("{id:?}: upload element count {} overflows u32", data.len()))?;
        let bytes: &[u8] = bytemuck::cast_slice(data);
        self.upload_bytes(stream, id, bytes, elems)
    }

    /// Uploads raw `bytes` representing `elems` elements asynchronously into owned buffer `id` on
    /// `stream`, growing the backing allocation if needed and guaranteeing a non-null pointer.
    pub fn upload_bytes(
        &mut self,
        stream: &Stream,
        id: BufId,
        bytes: &[u8],
        elems: u32,
    ) -> Result<(), String> {
        let slot = self
            .slots
            .get_mut(&id)
            .ok_or_else(|| format!("{id:?} is not an owned buffer of {:?}", self.plan.kind))?;
        let expected_bytes = (elems as usize)
            .checked_mul(slot.elem_size as usize)
            .ok_or_else(|| format!("{id:?}: upload byte size overflow"))?;
        if bytes.len() != expected_bytes {
            return Err(format!(
                "{id:?}: upload bytes {} != {elems} x {} B",
                bytes.len(),
                slot.elem_size
            ));
        }
        slot.ensure_elems(stream, id, elems)?;
        if !bytes.is_empty() {
            let ptr = slot
                .device_ptr()
                .ok_or_else(|| format!("{id:?}: device allocation missing after ensure_elems"))?;
            // SAFETY: `slot` holds at least `bytes.len()` valid device bytes on `stream` and
            // `cuMemcpyHtoDAsync_v2` reads `bytes.len()` bytes from the initialized slice.
            let rc = unsafe {
                cu::cuMemcpyHtoDAsync_v2(ptr, bytes.as_ptr().cast(), bytes.len(), stream.cu_stream())
            };
            if rc != cu::cudaError_enum_CUDA_SUCCESS {
                return Err(format!("{id:?}: cuMemcpyHtoDAsync_v2 failed ({rc:?})"));
            }
        }
        Ok(())
    }

    /// Asynchronously zeroes `max(1, elems)` elements of owned buffer `id` directly on the GPU
    /// (`cuMemsetD8Async`) with zero host allocation.
    pub fn zero_buffer(
        &mut self,
        stream: &Stream,
        id: BufId,
        elems: u32,
    ) -> Result<(), String> {
        let slot = self
            .slots
            .get_mut(&id)
            .ok_or_else(|| format!("{id:?} is not an owned buffer of {:?}", self.plan.kind))?;
        slot.ensure_elems(stream, id, elems)?;
        let ptr = slot
            .device_ptr()
            .ok_or_else(|| format!("{id:?}: device allocation missing after ensure_elems"))?;
        let byte_len = (elems.max(1) as usize)
            .checked_mul(slot.elem_size as usize)
            .ok_or_else(|| format!("{id:?}: zero_buffer byte size overflow"))?;
        // SAFETY: `slot` has capacity >= `byte_len` bytes on `stream`.
        let rc = unsafe { cu::cuMemsetD8Async(ptr, 0u8, byte_len, stream.cu_stream()) };
        if rc != cu::cudaError_enum_CUDA_SUCCESS {
            return Err(format!("{id:?}: cuMemsetD8Async failed ({rc:?})"));
        }
        Ok(())
    }

    /// Returns the raw CUDA device pointer of owned buffer `id`, or `None` if unallocated/unowned.
    #[inline]
    pub fn device_ptr(&self, id: BufId) -> Option<u64> {
        self.slots.get(&id).and_then(DeviceSlot::device_ptr)
    }

    /// Returns the active element count currently tracked for owned buffer `id`.
    #[inline]
    pub fn elem_count(&self, id: BufId) -> u32 {
        self.slots.get(&id).map_or(0, |s| s.len_elems)
    }

    /// Returns the underlying declarative [`FramePlan`].
    #[inline]
    pub fn plan(&self) -> &FramePlan {
        &self.plan
    }

    /// Returns the [`nvrtc::MathMode`] (`Fast` or `StrictF32`) used to compile this plan's PTX.
    #[inline]
    pub fn math_mode(&self) -> nvrtc::MathMode {
        self.math_mode
    }

    /// Returns the total allocated VRAM footprint in bytes across all owned plan buffers.
    #[inline]
    pub fn gpu_bytes(&self) -> u64 {
        self.slots.values().map(|s| s.cap_bytes as u64).sum()
    }

    /// Executes each raised `EventId`'s launch list in order, then `plan.frame` in exact order on
    /// `stream`, with zero mid-frame synchronization and zero heap allocation per launch.
    pub fn execute(
        &self,
        stream: &Stream,
        sizes: &dyn FrameSizes,
        scalars: &dyn FrameScalars,
        events: &[EventId],
        externals: &[(ExternalId, u64)],
        bounds: &[((PlanKind, BufId), u64)],
    ) -> Result<(), String> {
        for &event in events {
            let ev = self
                .plan
                .events
                .iter()
                .find(|e| e.event == event)
                .ok_or_else(|| {
                    format!(
                        "FramePlan {:?}: raised event {event:?} is not in plan.events",
                        self.plan.kind
                    )
                })?;
            for launch in ev.launches {
                self.launch_one(stream, launch, sizes, scalars, externals, bounds)?;
            }
        }
        for launch in self.plan.frame {
            self.launch_one(stream, launch, sizes, scalars, externals, bounds)?;
        }
        Ok(())
    }

    fn resolve_buffer_ptr(
        &self,
        id: BufId,
        externals: &[(ExternalId, u64)],
        bounds: &[((PlanKind, BufId), u64)],
    ) -> Result<u64, String> {
        if let Some(slot) = self.slots.get(&id) {
            return slot.device_ptr().ok_or_else(|| {
                format!(
                    "FramePlan {:?}: owned buffer {id:?} not allocated (call sync_sizes/upload first)",
                    self.plan.kind
                )
            });
        }
        let spec = self.plan.buffer(id).map_err(|e| {
            format!(
                "FramePlan {:?}: buffer {id:?} lookup failed: {e}",
                self.plan.kind
            )
        })?;
        match spec.class {
            BufClass::External(ext) => Ok(externals
                .iter()
                .find(|(k, _)| *k == ext)
                .map_or(0u64, |(_, ptr)| *ptr)),
            BufClass::Bound { plan, buf } => Ok(bounds
                .iter()
                .find(|((pk, b), _)| *pk == plan && *b == buf)
                .map_or(0u64, |(_, ptr)| *ptr)),
            _ => Err(format!(
                "FramePlan {:?}: owned buffer {id:?} missing from device slots",
                self.plan.kind
            )),
        }
    }

    fn launch_one(
        &self,
        stream: &Stream,
        launch: &KernelLaunch,
        sizes: &dyn FrameSizes,
        scalars: &dyn FrameScalars,
        externals: &[(ExternalId, u64)],
        bounds: &[((PlanKind, BufId), u64)],
    ) -> Result<(), String> {
        let (blocks, block) = launch.grid.shape(sizes).map_err(|e| {
            format!(
                "FramePlan {:?}: grid shape for '{}' failed: {e}",
                self.plan.kind, launch.kernel
            )
        })?;
        if blocks == 0 {
            return Ok(());
        }
        let func = *self.kernels.get(launch.kernel).ok_or_else(|| {
            format!(
                "FramePlan {:?}: kernel '{}' not loaded",
                self.plan.kind, launch.kernel
            )
        })?;
        let argc = launch.args.len();
        if argc > MAX_STACK_KERNEL_ARGS {
            return Err(format!(
                "FramePlan {:?}: kernel '{}' exceeds {MAX_STACK_KERNEL_ARGS} args",
                self.plan.kind, launch.kernel
            ));
        }

        let mut cells = [ArgCell::default(); MAX_STACK_KERNEL_ARGS];
        let mut arg_ptrs: [*mut c_void; MAX_STACK_KERNEL_ARGS] =
            [std::ptr::null_mut(); MAX_STACK_KERNEL_ARGS];
        for (i, arg) in launch.args.iter().enumerate() {
            match *arg {
                Arg::Buffer(id) => {
                    cells[i].ptr = self.resolve_buffer_ptr(id, externals, bounds)?;
                    arg_ptrs[i] = (&raw mut cells[i].ptr).cast();
                }
                Arg::U32(v) => {
                    cells[i].u32_val = v;
                    arg_ptrs[i] = (&raw mut cells[i].u32_val).cast();
                }
                Arg::F32(v) => {
                    cells[i].f32_val = v;
                    arg_ptrs[i] = (&raw mut cells[i].f32_val).cast();
                }
                Arg::Scalar(s) => match scalars.value(s).map_err(|e| {
                    format!(
                        "FramePlan {:?}: scalar {s:?} for '{}' failed: {e}",
                        self.plan.kind, launch.kernel
                    )
                })? {
                    ScalarValue::U32(v) => {
                        cells[i].u32_val = v;
                        arg_ptrs[i] = (&raw mut cells[i].u32_val).cast();
                    }
                    ScalarValue::F32(v) => {
                        cells[i].f32_val = v;
                        arg_ptrs[i] = (&raw mut cells[i].f32_val).cast();
                    }
                },
            }
        }

        // SAFETY: `func` was verified in `GpuFramePlan::new` to match `launch.args` arity and
        // pointer/scalar kinds; `arg_ptrs[..argc]` points into live stack `cells[..argc]` for the
        // duration of `cuLaunchKernel`, and all owned device pointers are allocated on `stream`.
        let rc = unsafe {
            cu::cuLaunchKernel(
                func,
                blocks,
                1,
                1,
                block,
                1,
                1,
                0,
                stream.cu_stream(),
                arg_ptrs.as_mut_ptr(),
                std::ptr::null_mut(),
            )
        };
        if rc != cu::cudaError_enum_CUDA_SUCCESS {
            return Err(format!(
                "cuLaunchKernel({}) failed ({rc:?})",
                launch.kernel
            ));
        }
        Ok(())
    }

    /// Enqueues an asynchronous D2H copy of owned buffer `id` into `dst` (resizing `dst` to the
    /// buffer's active element count without shrinking its capacity). Caller MUST synchronize
    /// `stream` before reading `dst`.
    pub fn download_into_async<T: Pod>(
        &self,
        stream: &Stream,
        id: BufId,
        dst: &mut Vec<T>,
    ) -> Result<(), String> {
        let slot = self
            .slots
            .get(&id)
            .ok_or_else(|| format!("{id:?} is not an owned buffer of {:?}", self.plan.kind))?;
        if std::mem::size_of::<T>() != slot.elem_size as usize {
            return Err(format!(
                "{id:?}: download element size {} B != plan elem_size {} B",
                std::mem::size_of::<T>(),
                slot.elem_size
            ));
        }
        let count = slot.len_elems as usize;
        dst.resize(count, T::zeroed());
        if count == 0 {
            return Ok(());
        }
        let ptr = slot
            .device_ptr()
            .ok_or_else(|| format!("{id:?}: device buffer not allocated"))?;
        let raw: &mut [u8] = bytemuck::cast_slice_mut(dst.as_mut_slice());
        if raw.len() > slot.cap_bytes {
            return Err(format!(
                "{id:?}: download size {} B exceeds capacity {} B",
                raw.len(),
                slot.cap_bytes
            ));
        }
        // SAFETY: `slot` owns at least `raw.len()` bytes on `stream`, `dst` owns `raw.len()`
        // writable bytes, and caller synchronizes `stream` before reading `dst`.
        let rc = unsafe {
            cu::cuMemcpyDtoHAsync_v2(raw.as_mut_ptr().cast(), ptr, raw.len(), stream.cu_stream())
        };
        if rc != cu::cudaError_enum_CUDA_SUCCESS {
            return Err(format!("{id:?}: cuMemcpyDtoHAsync_v2 failed ({rc:?})"));
        }
        Ok(())
    }

    /// Downloads owned buffer `id` into a new `Vec<T>` and synchronizes `stream` (for
    /// `ReadbackWhen::OnRequest` snapshots and tests).
    pub fn download_sync<T: Pod>(&self, stream: &Stream, id: BufId) -> Result<Vec<T>, String> {
        let mut out = Vec::new();
        self.download_into_async(stream, id, &mut out)?;
        stream
            .synchronize()
            .map_err(|e| format!("{id:?}: stream synchronize after D2H failed: {e}"))?;
        Ok(out)
    }

    /// Enqueues an asynchronous D2H copy of `count` elements from owned buffer `id` into
    /// caller-supplied page-locked host pointer `dst_ptr` (for small `ReadbackWhen::Always` words
    /// with zero allocation).
    ///
    /// # Safety
    /// `dst_ptr` must point to at least `count` writable, properly aligned elements of `T` in host
    /// memory that remains valid and unaliased until `stream` completes this DMA transfer.
    pub unsafe fn copy_to_pinned_async<T: Pod>(
        &self,
        stream: &Stream,
        id: BufId,
        dst_ptr: *mut T,
        count: usize,
    ) -> Result<(), String> {
        if count == 0 {
            return Ok(());
        }
        if dst_ptr.is_null() {
            return Err(format!("{id:?}: copy_to_pinned_async received null dst_ptr"));
        }
        let slot = self
            .slots
            .get(&id)
            .ok_or_else(|| format!("{id:?} is not an owned buffer of {:?}", self.plan.kind))?;
        if std::mem::size_of::<T>() != slot.elem_size as usize {
            return Err(format!(
                "{id:?}: pinned D2H element size {} B != plan elem_size {} B",
                std::mem::size_of::<T>(),
                slot.elem_size
            ));
        }
        let bytes = count
            .checked_mul(std::mem::size_of::<T>())
            .ok_or_else(|| format!("{id:?}: pinned D2H byte size overflow"))?;
        if bytes > slot.cap_bytes {
            return Err(format!(
                "{id:?}: pinned D2H size {bytes} B exceeds capacity {} B",
                slot.cap_bytes
            ));
        }
        let ptr = slot
            .device_ptr()
            .ok_or_else(|| format!("{id:?}: device buffer not allocated"))?;
        // SAFETY: `slot` owns at least `bytes` valid device bytes on `stream`, and caller
        // guarantees `dst_ptr` points to `count` writable elements valid until `stream` finishes.
        let rc = unsafe {
            cu::cuMemcpyDtoHAsync_v2(dst_ptr.cast(), ptr, bytes, stream.cu_stream())
        };
        if rc != cu::cudaError_enum_CUDA_SUCCESS {
            return Err(format!(
                "{id:?}: pinned cuMemcpyDtoHAsync_v2 failed ({rc:?})"
            ));
        }
        Ok(())
    }

    /// Safe wrapper over [`Self::copy_to_pinned_async`] targeting `dst[dst_offset..dst_offset + count]`.
    pub fn copy_to_pinned_buf_async<T: Pod>(
        &self,
        stream: &Stream,
        id: BufId,
        dst: &mut PinnedHostBuf<T>,
        dst_offset: usize,
        count: usize,
    ) -> Result<(), String> {
        let end = dst_offset
            .checked_add(count)
            .ok_or_else(|| format!("{id:?}: pinned destination range overflow"))?;
        if end > dst.capacity() {
            return Err(format!(
                "{id:?}: pinned destination range {dst_offset}..{end} exceeds PinnedHostBuf capacity {}",
                dst.capacity()
            ));
        }
        // SAFETY: `dst_offset + count <= dst.len()` guarantees `dst.as_mut_ptr().add(dst_offset)`
        // points to `count` writable page-locked elements owned by `dst`.
        unsafe { self.copy_to_pinned_async(stream, id, dst.as_mut_ptr().add(dst_offset), count) }
    }
}

impl Drop for GpuFramePlan {
    fn drop(&mut self) {
        if !self.module.is_null() && self.cuda.bind_to_thread().is_ok() {
            // SAFETY: `self.module` was loaded via `cuModuleLoadData` on `self.cuda` and is
            // unloaded exactly once when `GpuFramePlan` drops.
            let rc = unsafe { cu::cuModuleUnload(self.module) };
            if rc != cu::cudaError_enum_CUDA_SUCCESS {
                eprintln!(
                    "nv-demo: GpuFramePlan({:?}) cuModuleUnload failed ({rc:?})",
                    self.plan.kind
                );
            }
            self.module = std::ptr::null_mut();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "requires CUDA/NVRTC runtime"]
    fn strict_f32_ptx_emits_ieee754_div_and_sqrt_without_ftz() {
        let src = r#"
extern "C" __global__ void strict_probe(const float* a, const float* b, float* out) {
    int i = blockIdx.x * blockDim.x + threadIdx.x;
    out[i] = sqrtf(a[i] / b[i]);
}
"#;
        let strict_ptx =
            compile_nvrtc_ptx_with_math(src, "strict_probe.cu", nvrtc::MathMode::StrictF32)
                .expect("NVRTC strict_f32 compile");
        let strict_text = String::from_utf8_lossy(&strict_ptx);
        assert!(
            strict_text.contains("div.rn.f32"),
            "StrictF32 PTX must use IEEE-754 div.rn.f32:\n{strict_text}"
        );
        assert!(
            strict_text.contains("sqrt.rn.f32"),
            "StrictF32 PTX must use IEEE-754 sqrt.rn.f32:\n{strict_text}"
        );
        assert!(
            !strict_text.contains(".ftz."),
            "StrictF32 PTX must not flush subnormals (.ftz):\n{strict_text}"
        );

        let fast_ptx = compile_nvrtc_ptx_with_math(src, "fast_probe.cu", nvrtc::MathMode::Fast)
            .expect("NVRTC fast_math compile");
        let fast_text = String::from_utf8_lossy(&fast_ptx);
        assert!(
            fast_text.contains(".ftz."),
            "Fast PTX should emit flush-to-zero instructions:\n{fast_text}"
        );
    }
}
