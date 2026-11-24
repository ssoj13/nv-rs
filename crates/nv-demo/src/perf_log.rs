//! Opt-in per-frame performance log of the render thread, for before/after measurements.
//!
//! **What.** [`PerfLog`] records one [`PerfSample`] per rendered frame into a buffer allocated
//! once at startup and writes it as CSV a single time, when the requested number of frames is
//! collected (after a warm-up). Nothing is recorded or allocated per frame when it is disabled.
//! **Why.** The UI shows smoothed timings only; claims about speed-ups need raw per-frame numbers
//! of the same scene and camera, taken with nothing else on the GPU.
//! **How.** Enabled by `NV_DEMO_PERF_CSV=<path>` (read once). `NV_DEMO_PERF_FRAMES` (default
//! 1800) frames are kept after `NV_DEMO_PERF_WARMUP` (default 300) skipped frames. Columns:
//! host prep (frame start to the OptiX launch), launch-to-fence wall time, whole frame work wall
//! time, the [`crate::profiler::GpuPassProfiler`] engine times and the crowd step time.
//! **Used by** `renderer.rs` (render loop).

use std::io::Write;
use std::path::PathBuf;

/// One rendered frame. Wall times come from `Instant`; GPU times from the pass profiler (which
/// reports a frame that completed up to two frames earlier).
#[derive(Clone, Copy, Debug, Default)]
pub struct PerfSample {
    pub host_prep_ms: f32,
    pub launch_to_fence_ms: f32,
    pub frame_work_ms: f32,
    pub rt_ms: f32,
    pub denoise_ms: f32,
    pub bloom_ms: f32,
    pub tonemap_ms: f32,
    pub cuda_compute_ms: f32,
    pub crowd_step_ms: f32,
}

/// See the module docs.
pub struct PerfLog {
    path: PathBuf,
    warmup: u32,
    seen: u32,
    samples: Vec<PerfSample>,
    done: bool,
}

impl PerfLog {
    /// Reads the environment once; `None` when `NV_DEMO_PERF_CSV` is unset. An unparsable frame
    /// count is an error, not a silent default.
    pub fn from_env() -> Result<Option<Self>, String> {
        let Some(path) = std::env::var_os("NV_DEMO_PERF_CSV") else {
            return Ok(None);
        };
        let frames = env_u32("NV_DEMO_PERF_FRAMES", 1800)?.max(1);
        let warmup = env_u32("NV_DEMO_PERF_WARMUP", 300)?;
        Ok(Some(Self {
            path: PathBuf::from(path),
            warmup,
            seen: 0,
            samples: Vec::with_capacity(frames as usize),
            done: false,
        }))
    }

    /// Records one frame. Returns `Some(result)` exactly once, when the CSV was written (or
    /// failed to write); never allocates.
    pub fn record(&mut self, sample: PerfSample) -> Option<Result<PathBuf, String>> {
        if self.done {
            return None;
        }
        self.seen = self.seen.saturating_add(1);
        if self.seen <= self.warmup {
            return None;
        }
        self.samples.push(sample);
        if self.samples.len() < self.samples.capacity() {
            return None;
        }
        self.done = true;
        Some(self.write().map(|()| self.path.clone()))
    }

    fn write(&self) -> Result<(), String> {
        let file = std::fs::File::create(&self.path)
            .map_err(|e| format!("perf log {}: {e}", self.path.display()))?;
        let mut out = std::io::BufWriter::new(file);
        let io = |e: std::io::Error| format!("perf log {}: {e}", self.path.display());
        writeln!(
            out,
            "host_prep_ms,launch_to_fence_ms,frame_work_ms,rt_ms,denoise_ms,bloom_ms,tonemap_ms,cuda_compute_ms,crowd_step_ms"
        )
        .map_err(io)?;
        for s in &self.samples {
            writeln!(
                out,
                "{:.4},{:.4},{:.4},{:.4},{:.4},{:.4},{:.4},{:.4},{:.4}",
                s.host_prep_ms,
                s.launch_to_fence_ms,
                s.frame_work_ms,
                s.rt_ms,
                s.denoise_ms,
                s.bloom_ms,
                s.tonemap_ms,
                s.cuda_compute_ms,
                s.crowd_step_ms
            )
            .map_err(io)?;
        }
        out.flush().map_err(io)
    }
}

fn env_u32(name: &str, default: u32) -> Result<u32, String> {
    match std::env::var(name) {
        Ok(v) => v
            .trim()
            .parse()
            .map_err(|e| format!("{name}={v:?} is not a frame count: {e}")),
        Err(std::env::VarError::NotPresent) => Ok(default),
        Err(e) => Err(format!("{name}: {e}")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn writes_once_after_warmup_with_one_row_per_kept_frame() {
        let dir = std::env::temp_dir().join(format!("nv_demo_perf_{}", std::process::id()));
        std::fs::create_dir_all(&dir).expect("temp dir");
        let path = dir.join("perf.csv");
        let mut log = PerfLog {
            path: path.clone(),
            warmup: 2,
            seen: 0,
            samples: Vec::with_capacity(3),
            done: false,
        };
        let mut results = 0;
        for i in 0..10 {
            let s = PerfSample {
                frame_work_ms: i as f32,
                ..PerfSample::default()
            };
            if let Some(r) = log.record(s) {
                results += 1;
                assert_eq!(r.expect("written"), path);
                assert_eq!(i, 4, "2 warm-up frames + 3 kept frames");
            }
        }
        assert_eq!(results, 1);
        let text = std::fs::read_to_string(&path).expect("csv");
        let rows: Vec<&str> = text.lines().collect();
        assert_eq!(rows.len(), 4, "header + 3 rows");
        assert!(rows[1].starts_with("0.0000,0.0000,2.0000,"));
        assert!(rows[3].starts_with("0.0000,0.0000,4.0000,"));
        std::fs::remove_dir_all(&dir).expect("cleanup");
    }
}
