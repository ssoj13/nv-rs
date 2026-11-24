//! Opt-in per-frame performance log of the render thread, for before/after measurements.
//!
//! **What.** [`PerfLog`] records one [`PerfSample`] per rendered frame into a buffer allocated
//! once at startup and writes it as CSV a single time (temporary file, `sync_all`, rename), when
//! the requested number of frames is collected (after a warm-up). Nothing is recorded or allocated per frame when it is disabled.
//! **Why.** The UI shows smoothed timings only; claims about speed-ups need raw per-frame numbers
//! of the same scene and camera, taken with nothing else on the GPU.
//! **How.** Enabled by `NV_DEMO_PERF_CSV=<path>` (read once). `NV_DEMO_PERF_FRAMES` (default
//! 1800) frames are kept after `NV_DEMO_PERF_WARMUP` (default 300) skipped frames. Columns:
//! host prep (frame start to the OptiX launch), launch-to-fence wall time, whole frame work wall
//! time, the [`crate::profiler::GpuPassProfiler`] engine times, the crowd step time, and the
//! launch state ([`PerfState`]: possession, sensor projection, eye, resolution, spp, bounces) so a
//! run that drifted into another camera state is detectable.
//! The rail benchmark (`crate::bench`, `--bench-rail`) starts the same log through
//! `RenderControl::StartPerfLog`.
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
    /// What the launch traced, so runs in different camera states are never compared: a
    /// possessed vehicle mount can switch the sensor to a 360 projection, and fast preview lowers
    /// spp / bounces.
    pub state: PerfState,
}

/// Camera and launch state of one logged frame (see [`PerfSample::state`]).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PerfState {
    /// Riding a possessed vehicle / actor / landmark mount.
    pub possessed: bool,
    /// `CameraProjection as u32` (0 Pinhole, 1 Fisheye180, 2 Equirect360, 3 SlitCam).
    pub projection: u32,
    /// Launch camera position (world).
    pub eye: [f32; 3],
    pub width: u32,
    pub height: u32,
    /// Samples per pixel and bounces of the launch (after fast-preview reduction).
    pub spp: u32,
    pub bounces: u32,
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
        let frames = env_u32("NV_DEMO_PERF_FRAMES", 1800)?;
        let warmup = env_u32("NV_DEMO_PERF_WARMUP", 300)?;
        Ok(Some(Self::new(PathBuf::from(path), frames, warmup)))
    }

    /// A log that skips `warmup` frames, keeps `frames` (at least 1) and then writes `path`.
    pub fn new(path: PathBuf, frames: u32, warmup: u32) -> Self {
        Self {
            path,
            warmup,
            seen: 0,
            samples: Vec::with_capacity(frames.max(1) as usize),
            done: false,
        }
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

    /// Writes a temporary file next to `path`, syncs it and renames it over `path`, so a reader
    /// (or a benchmark script polling for the file) never sees a partial CSV.
    fn write(&self) -> Result<(), String> {
        let io = |e: std::io::Error| format!("perf log {}: {e}", self.path.display());
        let mut tmp_name = self.path.file_name().unwrap_or_default().to_os_string();
        tmp_name.push(format!(".tmp.{}", std::process::id()));
        let tmp = self.path.with_file_name(tmp_name);
        let written = self
            .write_rows(&tmp)
            .and_then(|()| std::fs::rename(&tmp, &self.path).map_err(io));
        if written.is_err()
            && let Err(e) = std::fs::remove_file(&tmp)
            && e.kind() != std::io::ErrorKind::NotFound
        {
            eprintln!(
                "nv-demo: perf log temp file {} not removed: {e}",
                tmp.display()
            );
        }
        written
    }

    fn write_rows(&self, tmp: &std::path::Path) -> Result<(), String> {
        let io = |e: std::io::Error| format!("perf log {}: {e}", tmp.display());
        let file = std::fs::File::create(tmp).map_err(io)?;
        let mut out = std::io::BufWriter::new(file);
        writeln!(
            out,
            "host_prep_ms,launch_to_fence_ms,frame_work_ms,rt_ms,denoise_ms,bloom_ms,tonemap_ms,cuda_compute_ms,crowd_step_ms,possessed,projection,eye_x,eye_y,eye_z,width,height,spp,bounces"
        )
        .map_err(io)?;
        for s in &self.samples {
            let st = &s.state;
            writeln!(
                out,
                "{:.4},{:.4},{:.4},{:.4},{:.4},{:.4},{:.4},{:.4},{:.4},{},{},{:.3},{:.3},{:.3},{},{},{},{}",
                s.host_prep_ms,
                s.launch_to_fence_ms,
                s.frame_work_ms,
                s.rt_ms,
                s.denoise_ms,
                s.bloom_ms,
                s.tonemap_ms,
                s.cuda_compute_ms,
                s.crowd_step_ms,
                u8::from(st.possessed),
                st.projection,
                st.eye[0],
                st.eye[1],
                st.eye[2],
                st.width,
                st.height,
                st.spp,
                st.bounces
            )
            .map_err(io)?;
        }
        let file = out.into_inner().map_err(|e| io(e.into_error()))?;
        file.sync_all().map_err(io)
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
        assert!(
            rows[0].ends_with(",possessed,projection,eye_x,eye_y,eye_z,width,height,spp,bounces")
        );
        assert!(rows[1].ends_with(",0,0,0.000,0.000,0.000,0,0,0,0"));
        let leftovers = std::fs::read_dir(&dir).expect("dir").count();
        assert_eq!(leftovers, 1, "only perf.csv remains, no temporary file");
        std::fs::remove_dir_all(&dir).expect("cleanup");
    }
}
