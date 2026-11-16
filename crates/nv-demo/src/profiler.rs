//! GPU/CPU frame timing ring buffers, NVML telemetry, and graceful CUPTI/NGX probing.
//!
//! **What.**
//! - [`RingBuffer`] and [`FrameTimings`] — fixed-capacity `[f32; 120]` ring buffers for
//!   `rt_ms`, `denoise_ms`, `bloom_ms`, `tonemap_ms`, and `fps` with zero per-frame heap
//!   allocations and zero panics.
//! - [`GpuTelemetry`] — extended GPU telemetry snapshot (NVML thermals, utilization, VRAM,
//!   power, graphics/memory clocks, plus CUPTI kernel time and CUPTI/NGX availability strings).
//! - [`Profiler`] — probes `cupti` (`ActivityProfiler` + `Subscriber`) and `ngx`
//!   (`NgxVulkan::required_extensions`) gracefully at startup without panicking on non-admin
//!   Windows or missing runtimes, and polls NVML + CUPTI telemetry.
//!
//! **Why.** Keeps per-frame profiling strictly allocation-free while surfacing rich hardware
//! telemetry and optional CUPTI/NGX diagnostics in the `nv-demo` UI.
//! **How.** All FFI is delegated to the safe `nvml`, `cupti`, and `ngx` crates (zero `unsafe` in
//! this module); CUPTI activity records drain into a pre-allocated `Vec` reused across polls.
//! **Used by** `nv-demo` (`main.rs`, `ui.rs`).

use std::sync::Arc;

use cuda_core::Context as CudaContext;
use cupti::{ActivityKind, ActivityProfiler, ActivityRecord, CallbackDomain, Subscriber};
use ngx::NgxVulkan;
use nvml::{ClockDomain, Device as NvmlDevice};

/// Fixed history capacity (frames) for [`FrameTimings`] ring buffers.
pub const HISTORY_LEN: usize = 120;

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

/// Per-frame CPU/GPU pass timings stored in fixed-capacity `[f32; 120]` ring buffers.
#[derive(Clone, Debug, Default)]
pub struct FrameTimings {
    /// Ray-tracing / OptiX kernel pass duration in milliseconds.
    pub rt_ms: RingBuffer<HISTORY_LEN>,
    /// Denoiser pass duration in milliseconds.
    pub denoise_ms: RingBuffer<HISTORY_LEN>,
    /// Bloom pass duration in milliseconds.
    pub bloom_ms: RingBuffer<HISTORY_LEN>,
    /// Tonemap / composite pass duration in milliseconds.
    pub tonemap_ms: RingBuffer<HISTORY_LEN>,
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
            fps: RingBuffer::new(),
        }
    }

    /// Records one frame's pass timings and FPS without allocating.
    #[inline]
    pub fn record(
        &mut self,
        rt_ms: f32,
        denoise_ms: f32,
        bloom_ms: f32,
        tonemap_ms: f32,
        fps: f32,
    ) {
        self.rt_ms.push(rt_ms);
        self.denoise_ms.push(denoise_ms);
        self.bloom_ms.push(bloom_ms);
        self.tonemap_ms.push(tonemap_ms);
        self.fps.push(fps);
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

    /// Records one frame's pass timings and FPS with zero heap allocations.
    #[inline]
    pub fn record_frame(
        &mut self,
        rt_ms: f32,
        denoise_ms: f32,
        bloom_ms: f32,
        tonemap_ms: f32,
        fps: f32,
    ) {
        self.timings
            .record(rt_ms, denoise_ms, bloom_ms, tonemap_ms, fps);
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
    fn frame_timings_records_all_passes() {
        let mut ft = FrameTimings::new();
        ft.record(1.5, 0.5, 0.25, 0.25, 60.0);
        assert_eq!(ft.rt_ms.latest(), 1.5);
        assert_eq!(ft.denoise_ms.latest(), 0.5);
        assert_eq!(ft.bloom_ms.latest(), 0.25);
        assert_eq!(ft.tonemap_ms.latest(), 0.25);
        assert_eq!(ft.fps.latest(), 60.0);
    }
}
