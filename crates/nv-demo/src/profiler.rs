//! GPU/CPU frame timing ring buffers, non-stalling 3-slot CUDA pass profiler, NVML telemetry,
//! and graceful CUPTI/NGX probing.
//!
//! **What.**
//! - [`RingBuffer`] and [`FrameTimings`] — fixed-capacity `[f32; 120]` ring buffers for per-pass
//!   timings (`rt_ms`, `denoise_ms`, `bloom_ms`, `tonemap_ms`, `fps`) and per-hardware-engine
//!   timings (`rt_cores_ms`, `tensor_cores_ms`, `cuda_compute_ms`) with zero per-frame heap
//!   allocations and zero panics.
//! - [`GpuPassProfiler`] — non-stalling 3-slot ring of hardware CUDA events (`cuEventQuery`,
//!   never `cuEventSynchronize`) measuring exact GPU durations for **RT Cores (OptiX)**,
//!   **Tensor Cores (Denoiser / DLSS)**, and **CUDA Compute (IAS / cuFFT / Tonemap)**.
//! - [`GpuTelemetry`] — extended GPU telemetry snapshot (NVML thermals, utilization, VRAM,
//!   power, graphics/memory clocks, plus CUPTI kernel time and CUPTI/NGX availability strings).
//! - [`Profiler`] — probes `cupti` (`ActivityProfiler` + `Subscriber`) and `ngx`
//!   (`NgxVulkan::required_extensions`) gracefully at startup without panicking on non-admin
//!   Windows or missing runtimes, and polls NVML + CUPTI telemetry.
//!
//! **Why.** Eliminates blocking mid-frame stream syncs while surfacing exact hardware GPU pass
//! timings across RTX RT Cores, Tensor Cores, and CUDA Compute SMs in the `nv-demo` UI.
//! **Used by** `nv-demo` (`main.rs`, `renderer.rs`, `ui.rs`).

use std::sync::Arc;

use cuda_bindings as cu;
use cuda_core::{Context as CudaContext, Stream};
use cupti::{ActivityKind, ActivityProfiler, ActivityRecord, CallbackDomain, Subscriber};
use ngx::NgxVulkan;
use nvml::{ClockDomain, Device as NvmlDevice};

/// Fixed history capacity (frames) for [`FrameTimings`] ring buffers.
pub const HISTORY_LEN: usize = 120;

/// Number of buffered frame event slots in [`GpuPassProfiler`].
pub const GPU_PROFILER_SLOTS: usize = 3;

/// Hardware GPU durations (milliseconds) resolved from one completed [`GpuPassProfiler`] slot.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GpuPassSample {
    /// Acceleration structure update / traffic upload duration in milliseconds.
    pub accel_ms: f32,
    /// OptiX ray-tracing launch duration (**RT Cores**) in milliseconds.
    pub rt_cores_ms: f32,
    /// OptiX AI denoiser duration (**Tensor Cores**, before any Vulkan DLSS addition) in milliseconds.
    pub tensor_cores_ms: f32,
    /// cuFFT 2D diffraction bloom duration in milliseconds.
    pub bloom_ms: f32,
    /// Tonemap / HDR scRGB / DLSS-RR AOV pack duration in milliseconds.
    pub tonemap_ms: f32,
}

impl GpuPassSample {
    /// Combined **CUDA Compute** SM time (`IAS/Traffic + cuFFT Bloom + Tonemap/Pack`) in milliseconds.
    #[inline]
    pub fn cuda_compute_ms(self) -> f32 {
        sanitize_ms(self.accel_ms) + sanitize_ms(self.bloom_ms) + sanitize_ms(self.tonemap_ms)
    }
}

#[inline]
fn sanitize_ms(v: f32) -> f32 {
    if v.is_finite() && v > 0.0 { v } else { 0.0 }
}

/// Six hardware CUDA timing events capturing all stage boundaries for one in-flight frame.
struct FrameEventSlot {
    start: cu::CUevent,
    accel_end: cu::CUevent,
    rt_end: cu::CUevent,
    denoise_end: cu::CUevent,
    bloom_end: cu::CUevent,
    frame_end: cu::CUevent,
    pending: bool,
}

impl FrameEventSlot {
    fn new() -> Result<Self, String> {
        let create = |label: &str| -> Result<cu::CUevent, String> {
            let mut ev: cu::CUevent = std::ptr::null_mut();
            // SAFETY: active CUDA context is bound; `CU_EVENT_DEFAULT` enables hardware timing.
            let res = unsafe { cu::cuEventCreate(&mut ev, cu::CUevent_flags_enum_CU_EVENT_DEFAULT) };
            if res != cu::cudaError_enum_CUDA_SUCCESS || ev.is_null() {
                return Err(format!("cuEventCreate({label}) failed ({res:?})"));
            }
            Ok(ev)
        };
        Ok(Self {
            start: create("start")?,
            accel_end: create("accel_end")?,
            rt_end: create("rt_end")?,
            denoise_end: create("denoise_end")?,
            bloom_end: create("bloom_end")?,
            frame_end: create("frame_end")?,
            pending: false,
        })
    }

    fn destroy(&mut self) {
        for ev in [
            &mut self.start,
            &mut self.accel_end,
            &mut self.rt_end,
            &mut self.denoise_end,
            &mut self.bloom_end,
            &mut self.frame_end,
        ] {
            if !ev.is_null() {
                // SAFETY: each handle was created by `cuEventCreate` on the owning CUDA context.
                unsafe {
                    let _ = cu::cuEventDestroy_v2(*ev);
                }
                *ev = std::ptr::null_mut();
            }
        }
        self.pending = false;
    }
}

/// Non-stalling 3-slot ring of hardware CUDA events (`cuEventQuery`, never `cuEventSynchronize`)
/// recording per-engine GPU execution time for RT Cores, Tensor Cores, and CUDA Compute.
pub struct GpuPassProfiler {
    ctx: Arc<CudaContext>,
    slots: [FrameEventSlot; GPU_PROFILER_SLOTS],
    write_idx: usize,
    latest: GpuPassSample,
}

// SAFETY: CUDA event handles belong to `ctx` and are accessed exclusively from the render worker.
unsafe impl Send for GpuPassProfiler {}

impl GpuPassProfiler {
    /// Pre-allocates all 3 frame event slots (`18` hardware `CUevent`s) with zero steady-state churn.
    pub fn new(cuda: &Arc<CudaContext>) -> Result<Self, String> {
        cuda.bind_to_thread()
            .map_err(|e| format!("GpuPassProfiler bind_to_thread failed: {e}"))?;
        let s0 = FrameEventSlot::new()?;
        let s1 = FrameEventSlot::new()?;
        let s2 = FrameEventSlot::new()?;
        Ok(Self {
            ctx: cuda.clone(),
            slots: [s0, s1, s2],
            write_idx: 0,
            latest: GpuPassSample::default(),
        })
    }

    /// Records the frame-start event on `stream` in the active ring slot and returns its slot index.
    #[inline]
    pub fn begin_frame(&mut self, stream: &Stream) -> usize {
        let slot = self.write_idx;
        self.slots[slot].pending = false;
        record_event(self.slots[slot].start, stream);
        slot
    }

    /// Marks completion of IAS build/update and launch-parameter upload on `stream`.
    #[inline]
    pub fn mark_accel_end(&self, slot: usize, stream: &Stream) {
        if let Some(s) = self.slots.get(slot) {
            record_event(s.accel_end, stream);
        }
    }

    /// Marks completion of the OptiX ray-tracing launch (**RT Cores**) on `stream`.
    #[inline]
    pub fn mark_rt_end(&self, slot: usize, stream: &Stream) {
        if let Some(s) = self.slots.get(slot) {
            record_event(s.rt_end, stream);
        }
    }

    /// Marks completion of the OptiX AI denoiser (**Tensor Cores**) on `stream`.
    #[inline]
    pub fn mark_denoise_end(&self, slot: usize, stream: &Stream) {
        if let Some(s) = self.slots.get(slot) {
            record_event(s.denoise_end, stream);
        }
    }

    /// Marks completion of the cuFFT 2D diffraction bloom pass (**CUDA Compute**) on `stream`.
    #[inline]
    pub fn mark_bloom_end(&self, slot: usize, stream: &Stream) {
        if let Some(s) = self.slots.get(slot) {
            record_event(s.bloom_end, stream);
        }
    }

    /// Marks end-of-frame tonemap/AOV packing on `stream`, seals `slot` as pending, and advances
    /// the ring index for the next frame.
    #[inline]
    pub fn mark_frame_end(&mut self, slot: usize, stream: &Stream) {
        if let Some(s) = self.slots.get_mut(slot) {
            record_event(s.frame_end, stream);
            s.pending = true;
        }
        self.write_idx = (slot + 1) % GPU_PROFILER_SLOTS;
    }

    /// Polls all completed pending slots using non-blocking `cuEventQuery` (never blocks or calls
    /// `cuEventSynchronize`) and returns the latest resolved [`GpuPassSample`].
    pub fn poll_completed(&mut self) -> GpuPassSample {
        // Walk chronologically from oldest slot (`write_idx`) to newest so `self.latest` always
        // reflects the most recent completed frame.
        for offset in 0..GPU_PROFILER_SLOTS {
            let idx = (self.write_idx + offset) % GPU_PROFILER_SLOTS;
            let slot = &mut self.slots[idx];
            if !slot.pending {
                continue;
            }
            // SAFETY: `slot.frame_end` is a valid `CUevent` created on `self.ctx`.
            let status = unsafe { cu::cuEventQuery(slot.frame_end) };
            if status == cu::cudaError_enum_CUDA_SUCCESS {
                self.latest = GpuPassSample {
                    accel_ms: elapsed_ms(slot.start, slot.accel_end),
                    rt_cores_ms: elapsed_ms(slot.accel_end, slot.rt_end),
                    tensor_cores_ms: elapsed_ms(slot.rt_end, slot.denoise_end),
                    bloom_ms: elapsed_ms(slot.denoise_end, slot.bloom_end),
                    tonemap_ms: elapsed_ms(slot.bloom_end, slot.frame_end),
                };
                slot.pending = false;
            }
        }
        self.latest
    }
}

impl Drop for GpuPassProfiler {
    fn drop(&mut self) {
        if self.ctx.bind_to_thread().is_ok() {
            for slot in &mut self.slots {
                slot.destroy();
            }
        }
    }
}

#[inline]
fn record_event(ev: cu::CUevent, stream: &Stream) {
    if !ev.is_null() {
        // SAFETY: `ev` is a valid timing event on the active context and `stream` is live.
        unsafe {
            let _ = cu::cuEventRecord(ev, stream.cu_stream());
        }
    }
}

#[inline]
fn elapsed_ms(start: cu::CUevent, end: cu::CUevent) -> f32 {
    if start.is_null() || end.is_null() {
        return 0.0;
    }
    let mut ms = 0.0f32;
    // SAFETY: caller verified `frame_end` completed via `cuEventQuery == CUDA_SUCCESS`, so both
    // `start` and `end` (recorded earlier on the same stream) are complete.
    let res = unsafe { cu::cuEventElapsedTime_v2(&mut ms, start, end) };
    if res == cu::cudaError_enum_CUDA_SUCCESS {
        sanitize_ms(ms)
    } else {
        0.0
    }
}

/// Fixed-capacity, stack-allocated ring buffer for scalar `f32` frame metrics.
#[derive(Clone, Debug)]
pub struct RingBuffer<const N: usize> {
    data: [f32; N],
    head: usize,
    len: usize,
}

impl<const N: usize> Default for RingBuffer<N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const N: usize> RingBuffer<N> {
    /// Creates an empty ring buffer zero-initialized on the stack.
    pub const fn new() -> Self {
        Self {
            data: [0.0; N],
            head: 0,
            len: 0,
        }
    }

    /// Pushes a sample into the ring buffer, overwriting the oldest entry when full.
    ///
    /// Performs zero heap allocations and never panics (no-op if `N == 0`).
    #[inline]
    pub fn push(&mut self, value: f32) {
        if N == 0 {
            return;
        }
        if let Some(slot) = self.data.get_mut(self.head) {
            *slot = value;
        }
        self.head += 1;
        if self.head >= N {
            self.head = 0;
        }
        if self.len < N {
            self.len += 1;
        }
    }

    /// Number of valid samples currently stored (`0..=N`).
    #[inline]
    pub const fn len(&self) -> usize {
        self.len
    }

    /// Returns the most recently pushed sample, or `0.0` if empty.
    #[inline]
    pub fn latest(&self) -> f32 {
        if self.len == 0 || N == 0 {
            return 0.0;
        }
        let idx = if self.head == 0 { N - 1 } else { self.head - 1 };
        self.data.get(idx).copied().unwrap_or(0.0)
    }

    /// Returns `(min, max)` across stored samples, or `(0.0, 0.0)` if empty.
    pub fn min_max(&self) -> (f32, f32) {
        if self.len == 0 || N == 0 {
            return (0.0, 0.0);
        }
        let mut min_v = f32::INFINITY;
        let mut max_v = f32::NEG_INFINITY;
        for v in self.iter() {
            if v < min_v {
                min_v = v;
            }
            if v > max_v {
                max_v = v;
            }
        }
        if min_v.is_infinite() || max_v.is_infinite() {
            (0.0, 0.0)
        } else {
            (min_v, max_v)
        }
    }

    /// Iterates over stored samples in chronological order (oldest to newest).
    pub fn iter(&self) -> impl Iterator<Item = f32> + '_ {
        let start = if self.len < N { 0 } else { self.head };
        (0..self.len).filter_map(move |i| {
            let idx = (start + i) % N.max(1);
            self.data.get(idx).copied()
        })
    }
}

/// Per-frame CPU/GPU pass & hardware engine timings stored in fixed-capacity `[f32; 120]` ring buffers.
#[derive(Clone, Debug, Default)]
pub struct FrameTimings {
    /// Ray-tracing / OptiX kernel pass duration in milliseconds (**RT Cores**).
    pub rt_ms: RingBuffer<HISTORY_LEN>,
    /// Denoiser pass duration in milliseconds.
    pub denoise_ms: RingBuffer<HISTORY_LEN>,
    /// Bloom pass duration in milliseconds.
    pub bloom_ms: RingBuffer<HISTORY_LEN>,
    /// Tonemap / composite pass duration in milliseconds.
    pub tonemap_ms: RingBuffer<HISTORY_LEN>,
    /// Hardware **RT Cores (OptiX)** execution duration in milliseconds.
    pub rt_cores_ms: RingBuffer<HISTORY_LEN>,
    /// Hardware **Tensor Cores (Denoiser / DLSS)** execution duration in milliseconds.
    pub tensor_cores_ms: RingBuffer<HISTORY_LEN>,
    /// Hardware **CUDA Compute (IAS / cuFFT / Tonemap)** execution duration in milliseconds.
    pub cuda_compute_ms: RingBuffer<HISTORY_LEN>,
    /// Instantaneous frames per second.
    pub fps: RingBuffer<HISTORY_LEN>,
}

impl FrameTimings {
    /// Creates an empty [`FrameTimings`] history with zero heap allocations.
    pub const fn new() -> Self {
        Self {
            rt_ms: RingBuffer::new(),
            denoise_ms: RingBuffer::new(),
            bloom_ms: RingBuffer::new(),
            tonemap_ms: RingBuffer::new(),
            rt_cores_ms: RingBuffer::new(),
            tensor_cores_ms: RingBuffer::new(),
            cuda_compute_ms: RingBuffer::new(),
            fps: RingBuffer::new(),
        }
    }

    /// Records one frame's pass timings, hardware engine breakdown, and FPS without allocating.
    #[expect(
        clippy::too_many_arguments,
        reason = "mirrors flat per-pass and per-engine timing ring buffers with zero allocation"
    )]
    #[inline]
    pub fn record_with_engines(
        &mut self,
        rt_ms: f32,
        denoise_ms: f32,
        bloom_ms: f32,
        tonemap_ms: f32,
        rt_cores_ms: f32,
        tensor_cores_ms: f32,
        cuda_compute_ms: f32,
        fps: f32,
    ) {
        self.rt_ms.push(sanitize_ms(rt_ms));
        self.denoise_ms.push(sanitize_ms(denoise_ms));
        self.bloom_ms.push(sanitize_ms(bloom_ms));
        self.tonemap_ms.push(sanitize_ms(tonemap_ms));
        self.rt_cores_ms.push(sanitize_ms(rt_cores_ms));
        self.tensor_cores_ms.push(sanitize_ms(tensor_cores_ms));
        self.cuda_compute_ms.push(sanitize_ms(cuda_compute_ms));
        self.fps.push(sanitize_ms(fps));
    }
}

/// Hardware GPU telemetry and CUPTI/NGX runtime status snapshot for UI display.
#[derive(Clone, Debug)]
pub struct GpuTelemetry {
    /// GPU product name from NVML.
    pub gpu_name: String,
    /// GPU die temperature in degrees Celsius.
    pub temp_c: u32,
    /// GPU core utilization percentage (`0..=100`).
    pub util_gpu: u32,
    /// Used framebuffer memory in MiB.
    pub vram_used_mib: u64,
    /// Total framebuffer memory in MiB.
    pub vram_total_mib: u64,
    /// Instantaneous board power draw in Watts.
    pub power_w: f32,
    /// Current graphics clock frequency in MHz.
    pub gpu_clock_mhz: u32,
    /// Current memory clock frequency in MHz.
    pub mem_clock_mhz: u32,
    /// Human-readable CUPTI probe/activity status.
    pub cupti_status: String,
    /// Human-readable NGX/DLSS runtime availability status.
    pub ngx_status: String,
    /// Most recent drained CUPTI kernel execution duration in microseconds.
    pub cupti_kernel_us: f32,
}

impl Default for GpuTelemetry {
    fn default() -> Self {
        Self {
            gpu_name: "NVML unavailable".to_string(),
            temp_c: 0,
            util_gpu: 0,
            vram_used_mib: 0,
            vram_total_mib: 0,
            power_w: 0.0,
            gpu_clock_mhz: 0,
            mem_clock_mhz: 0,
            cupti_status: "Not probed".to_string(),
            ngx_status: "Not probed".to_string(),
            cupti_kernel_us: 0.0,
        }
    }
}

/// Probes whether the NVIDIA NGX / DLSS runtime is available on this host.
///
/// Never panics; returns `(available, status_description)`.
pub fn probe_ngx_status() -> (bool, String) {
    match NgxVulkan::required_extensions() {
        Ok(exts) => (
            true,
            format!(
                "Available ({} inst / {} dev exts)",
                exts.instance_extensions.len(),
                exts.device_extensions.len()
            ),
        ),
        Err(err) => (false, format!("Unavailable: {err}")),
    }
}

/// Manages frame timing history, optional CUPTI activity/subscriber handles, and NVML polling.
pub struct Profiler {
    /// Fixed-capacity frame timing ring buffers.
    pub timings: FrameTimings,
    cupti_profiler: Option<ActivityProfiler>,
    _cupti_subscriber: Option<Subscriber>,
    cupti_records: Vec<ActivityRecord>,
    cupti_status: String,
    ngx_status: String,
    last_kernel_us: f32,
}

impl Profiler {
    /// Initializes the profiler, probing CUPTI (`ActivityProfiler` + `Subscriber`) and NGX
    /// gracefully without panicking if privileges or runtimes are missing.
    pub fn new(cuda: Option<&Arc<CudaContext>>) -> Self {
        let (_ngx_available, ngx_status) = probe_ngx_status();

        let (cupti_profiler, cupti_subscriber, cupti_status) = match cuda {
            Some(ctx) => probe_cupti(ctx),
            None => (None, None, "Unavailable: no CUDA context".to_string()),
        };

        Self {
            timings: FrameTimings::new(),
            cupti_profiler,
            _cupti_subscriber: cupti_subscriber,
            cupti_records: Vec::with_capacity(256),
            cupti_status,
            ngx_status,
            last_kernel_us: 0.0,
        }
    }

    /// Records one frame's pass timings, per-engine hardware GPU breakdown, and FPS with zero heap allocations.
    #[expect(
        clippy::too_many_arguments,
        reason = "forwards per-pass and per-engine timing scalars without heap allocation"
    )]
    #[inline]
    pub fn record_frame_engines(
        &mut self,
        rt_ms: f32,
        denoise_ms: f32,
        bloom_ms: f32,
        tonemap_ms: f32,
        rt_cores_ms: f32,
        tensor_cores_ms: f32,
        cuda_compute_ms: f32,
        fps: f32,
    ) {
        self.timings.record_with_engines(
            rt_ms,
            denoise_ms,
            bloom_ms,
            tonemap_ms,
            rt_cores_ms,
            tensor_cores_ms,
            cuda_compute_ms,
            fps,
        );
    }

    /// Flushes and drains pending CUPTI activity records into the pre-allocated scratch buffer,
    /// returning the summed kernel execution duration in microseconds.
    pub fn poll_cupti(&mut self) -> f32 {
        let Some(ref prof) = self.cupti_profiler else {
            return self.last_kernel_us;
        };
        if prof.flush(false).is_err() {
            return self.last_kernel_us;
        }
        self.cupti_records.clear();
        if prof.drain_records_into(&mut self.cupti_records).is_err() {
            return self.last_kernel_us;
        }
        let mut total_ns = 0u64;
        let mut saw_kernel = false;
        for rec in &self.cupti_records {
            if let ActivityRecord::Kernel {
                start_ns, end_ns, ..
            } = rec
            {
                total_ns = total_ns.saturating_add(end_ns.saturating_sub(*start_ns));
                saw_kernel = true;
            }
        }
        if saw_kernel {
            self.last_kernel_us = total_ns as f32 / 1_000.0;
        }
        self.last_kernel_us
    }

    /// Reads NVML device metrics and CUPTI/NGX status into a fresh [`GpuTelemetry`] snapshot.
    pub fn read_telemetry(&mut self, dev: Option<&NvmlDevice>) -> GpuTelemetry {
        let cupti_kernel_us = self.poll_cupti();
        let mut out = GpuTelemetry {
            cupti_status: self.cupti_status.clone(),
            ngx_status: self.ngx_status.clone(),
            cupti_kernel_us,
            ..GpuTelemetry::default()
        };
        if let Some(d) = dev {
            populate_nvml_telemetry(d, &mut out);
        }
        out
    }

    /// Updates an existing [`GpuTelemetry`] in place (avoids cloning status strings when unchanged).
    pub fn update_telemetry(&mut self, dev: Option<&NvmlDevice>, out: &mut GpuTelemetry) {
        out.cupti_kernel_us = self.poll_cupti();
        if out.cupti_status != self.cupti_status {
            out.cupti_status.clone_from(&self.cupti_status);
        }
        if out.ngx_status != self.ngx_status {
            out.ngx_status.clone_from(&self.ngx_status);
        }
        if let Some(d) = dev {
            populate_nvml_telemetry(d, out);
        } else {
            out.gpu_name.clear();
            out.gpu_name.push_str("NVML unavailable");
            out.temp_c = 0;
            out.util_gpu = 0;
            out.vram_used_mib = 0;
            out.vram_total_mib = 0;
            out.power_w = 0.0;
            out.gpu_clock_mhz = 0;
            out.mem_clock_mhz = 0;
        }
    }
}

fn populate_nvml_telemetry(d: &NvmlDevice, out: &mut GpuTelemetry) {
    out.gpu_name = d.name().unwrap_or_else(|_| "NVIDIA GPU".to_string());
    out.temp_c = d.temperature_gpu().unwrap_or(0);
    out.util_gpu = d.utilization_rates().map(|u| u.gpu).unwrap_or(0);
    let (vram_used_mib, vram_total_mib) = d
        .memory_info()
        .map(|m| (m.used / (1024 * 1024), m.total / (1024 * 1024)))
        .unwrap_or((0, 0));
    out.vram_used_mib = vram_used_mib;
    out.vram_total_mib = vram_total_mib;
    out.power_w = d.power_usage_mw().unwrap_or(0) as f32 / 1000.0;
    out.gpu_clock_mhz = d.clock_mhz(ClockDomain::Graphics).unwrap_or(0);
    out.mem_clock_mhz = d.clock_mhz(ClockDomain::Memory).unwrap_or(0);
}

fn probe_cupti(cuda: &Arc<CudaContext>) -> (Option<ActivityProfiler>, Option<Subscriber>, String) {
    let ver = match cupti::version() {
        Ok(v) => v,
        Err(err) => return (None, None, format!("Unavailable: {err}")),
    };

    let activity_res = ActivityProfiler::new(cuda).and_then(|mut prof| {
        if prof.enable(ActivityKind::ConcurrentKernel).is_err() {
            prof.enable(ActivityKind::Kernel)?;
        }
        Ok(prof)
    });

    let sub_res = Subscriber::new(cuda, Box::new(|_domain, _cbid| {})).and_then(|sub| {
        sub.enable_domain(CallbackDomain::DriverApi, true)?;
        Ok(sub)
    });

    match (activity_res, sub_res) {
        (Ok(prof), Ok(sub)) => (
            Some(prof),
            Some(sub),
            format!("Active v{ver} (Activity + Subscriber)"),
        ),
        (Ok(prof), Err(sub_err)) => (
            Some(prof),
            None,
            format!("Partial v{ver} (Activity only; Subscriber: {sub_err})"),
        ),
        (Err(act_err), Ok(sub)) => (
            None,
            Some(sub),
            format!("Partial v{ver} (Subscriber only; Activity: {act_err})"),
        ),
        (Err(act_err), Err(sub_err)) => (
            None,
            None,
            format!("Init failed v{ver} (Activity: {act_err}; Subscriber: {sub_err})"),
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ring_buffer_wraps_without_panics() {
        let mut rb = RingBuffer::<4>::new();
        assert_eq!(rb.len(), 0);
        assert_eq!(rb.latest(), 0.0);
        assert_eq!(rb.min_max(), (0.0, 0.0));

        for i in 1..=6 {
            rb.push(i as f32);
        }
        assert_eq!(rb.len(), 4);
        assert_eq!(rb.latest(), 6.0);
        assert_eq!(rb.min_max(), (3.0, 6.0));
        let collected: Vec<f32> = rb.iter().collect();
        assert_eq!(collected, vec![3.0, 4.0, 5.0, 6.0]);
    }

    #[test]
    fn frame_timings_records_all_passes_and_engines() {
        let mut ft = FrameTimings::new();
        ft.record_with_engines(2.4, 0.8, 0.35, 0.15, 2.4, 1.1, 0.65, 120.0);
        assert_eq!(ft.rt_ms.latest(), 2.4);
        assert_eq!(ft.denoise_ms.latest(), 0.8);
        assert_eq!(ft.bloom_ms.latest(), 0.35);
        assert_eq!(ft.tonemap_ms.latest(), 0.15);
        assert_eq!(ft.rt_cores_ms.latest(), 2.4);
        assert_eq!(ft.tensor_cores_ms.latest(), 1.1);
        assert_eq!(ft.cuda_compute_ms.latest(), 0.65);
        assert_eq!(ft.fps.latest(), 120.0);

        let sample = GpuPassSample {
            accel_ms: 0.15,
            rt_cores_ms: 3.2,
            tensor_cores_ms: 0.75,
            bloom_ms: 0.40,
            tonemap_ms: 0.20,
        };
        assert!((sample.cuda_compute_ms() - 0.75).abs() < 1e-5);

        // Non-finite or negative CUDA event deltas clamp safely to 0.0.
        let bad = GpuPassSample {
            accel_ms: f32::NAN,
            rt_cores_ms: -1.0,
            tensor_cores_ms: f32::INFINITY,
            bloom_ms: 0.25,
            tonemap_ms: -0.5,
        };
        assert!((bad.cuda_compute_ms() - 0.25).abs() < 1e-5);
    }
}
