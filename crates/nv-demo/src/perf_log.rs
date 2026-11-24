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

    fn write(&self) -> Result<(), String> {
        write_csv(&self.path, &self.samples)
    }
}

/// Writes `samples` as the perf CSV (the schema of `--bench-rail` and `NV_DEMO_PERF_CSV`) to a
/// temporary file next to `path`, syncs it and renames it over `path`, so a reader (or a
/// benchmark script polling for the file) never sees a partial CSV. Also used by the rail
/// Flight stats export (`crate::flight_stats`).
pub fn write_csv<'a>(
    path: &std::path::Path,
    samples: impl IntoIterator<Item = &'a PerfSample>,
) -> Result<(), String> {
    let io = |e: std::io::Error| format!("perf log {}: {e}", path.display());
    let mut tmp_name = path.file_name().unwrap_or_default().to_os_string();
    tmp_name.push(format!(".tmp.{}", std::process::id()));
    let tmp = path.with_file_name(tmp_name);
    let written = write_rows(&tmp, samples).and_then(|()| std::fs::rename(&tmp, path).map_err(io));
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

/// Writes `rgba` (`width * height` linear colours, row 0 at the top) as a little-endian RGB PFM
/// (`PF`, scale `-1.0`, rows bottom-up), atomically like [`write_csv`]. The rail benchmark's
/// pixel A/B reads it back exactly (no tonemap, no quantisation).
pub fn write_pfm(
    path: &std::path::Path,
    width: u32,
    height: u32,
    rgba: &[[f32; 4]],
) -> Result<(), String> {
    let (w, h) = (width as usize, height as usize);
    if rgba.len() != w * h {
        return Err(format!(
            "image {}: {} pixels for {width}x{height}",
            path.display(),
            rgba.len()
        ));
    }
    let io = |e: std::io::Error| format!("image {}: {e}", path.display());
    let mut tmp_name = path.file_name().unwrap_or_default().to_os_string();
    tmp_name.push(format!(".tmp.{}", std::process::id()));
    let tmp = path.with_file_name(tmp_name);
    let written = (|| -> std::io::Result<()> {
        let mut out = std::io::BufWriter::new(std::fs::File::create(&tmp)?);
        write!(out, "PF\n{width} {height}\n-1.0\n")?;
        for row in rgba.chunks_exact(w.max(1)).rev() {
            for px in row {
                for c in &px[..3] {
                    out.write_all(&c.to_le_bytes())?;
                }
            }
        }
        out.into_inner()
            .map_err(std::io::IntoInnerError::into_error)?
            .sync_all()
    })()
    .map_err(io)
    .and_then(|()| std::fs::rename(&tmp, path).map_err(io));
    if written.is_err()
        && let Err(e) = std::fs::remove_file(&tmp)
        && e.kind() != std::io::ErrorKind::NotFound
    {
        eprintln!(
            "nv-demo: image temp file {} not removed: {e}",
            tmp.display()
        );
    }
    written
}

fn write_rows<'a>(
    tmp: &std::path::Path,
    samples: impl IntoIterator<Item = &'a PerfSample>,
) -> Result<(), String> {
    let io = |e: std::io::Error| format!("perf log {}: {e}", tmp.display());
    let file = std::fs::File::create(tmp).map_err(io)?;
    let mut out = std::io::BufWriter::new(file);
    writeln!(
        out,
        "host_prep_ms,launch_to_fence_ms,frame_work_ms,rt_ms,denoise_ms,bloom_ms,tonemap_ms,cuda_compute_ms,crowd_step_ms,possessed,projection,eye_x,eye_y,eye_z,width,height,spp,bounces"
    )
    .map_err(io)?;
    for s in samples {
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

    #[test]
    fn pfm_is_rgb_little_endian_bottom_row_first() {
        let dir = std::env::temp_dir().join(format!("nv_demo_pfm_{}", std::process::id()));
        std::fs::create_dir_all(&dir).expect("temp dir");
        let path = dir.join("img.pfm");
        let top = [1.0, 2.0, 3.0, 9.0];
        let bottom = [4.0, 5.0, 6.0, 9.0];
        write_pfm(&path, 1, 2, &[top, bottom]).expect("written");
        let bytes = std::fs::read(&path).expect("pfm");
        let header = b"PF\n1 2\n-1.0\n";
        assert_eq!(&bytes[..header.len()], header);
        let floats: Vec<f32> = bytes[header.len()..]
            .as_chunks::<4>()
            .0
            .iter()
            .map(|b| f32::from_le_bytes(*b))
            .collect();
        assert_eq!(floats, vec![4.0, 5.0, 6.0, 1.0, 2.0, 3.0]);
        assert!(write_pfm(&path, 2, 2, &[top]).is_err(), "size mismatch");
        std::fs::remove_dir_all(&dir).expect("cleanup");
    }
}
