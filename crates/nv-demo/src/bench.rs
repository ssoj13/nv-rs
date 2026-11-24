//! Deterministic camera-rail benchmark: `nv-demo --bench-rail <rail-id> --out <csv>`.
//!
//! **What.** Parses the benchmark command line ([`BenchRailArgs`]) and holds the run state
//! ([`BenchRun`]). A run loads a scene, freezes traffic and crowd poses, puts the free camera
//! (Pinhole, never possessed) on one scene rail cassette from rail time 0, steps it in lockstep
//! with the renderer at the fixed `1 / 60 s * time_scale` rail clock (one rendered frame per
//! step, the Smooth 60 FPS recording path without a video file), records `warmup + frames`
//! frames through [`crate::perf_log::PerfLog`] and exits with the CSV written atomically.
//! **Why.** Free-running timings depend on wall-clock pacing, traffic positions and whatever the
//! camera happens to be doing; an A/B needs every run to render exactly the same camera sequence
//! over exactly the same geometry.
//! **How.** `main` parses the arguments before the event loop; `DemoApp::new` applies the scene
//! and freeze overrides and never writes the user config in bench mode; when the scene's rails
//! arrive (`RenderEvent::SceneMetaUpdated`) the run selects the cassette and starts the perf log
//! and lockstep mode; `RenderEvent::PerfLogWritten` ends the process (exit code 0 on success).
//! **Used by** `main.rs`. Documented in `crates/nv-demo/AGENTS.md` (Benchmark).

use std::ffi::OsString;
use std::path::PathBuf;

/// Usage text printed on a command-line error.
pub const USAGE: &str = "usage: nv-demo [--bench-rail <rail-id> --out <csv> [--scene <scene-id>] \
[--frames <n>] [--warmup <n>]]\n  --bench-rail  scene rail cassette id (e.g. road_cruise); unknown \
ids list the available ones\n  --out         CSV written when the run ends (atomic rename)\n  \
--scene       scene descriptor id (default: the saved scene)\n  --frames      measured frames \
(default: one rail loop at 60 fps * time_scale)\n  --warmup      frames rendered and discarded \
before measuring (default 60)";

/// Default number of discarded frames at the start of the rail.
pub const DEFAULT_WARMUP: u32 = 60;

/// `--bench-rail` command line.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BenchRailArgs {
    /// Scene rail cassette id (`RailSpec::id`).
    pub rail: String,
    /// CSV output path.
    pub out: PathBuf,
    /// Scene descriptor id; `None` keeps the saved scene.
    pub scene: Option<String>,
    /// Measured frames; `None` = one loop of the rail.
    pub frames: Option<u32>,
    /// Discarded frames before measuring.
    pub warmup: u32,
}

/// Parses the process arguments after the program name. `Ok(None)` = no arguments (normal
/// interactive start); any argument other than the benchmark ones is an error.
pub fn parse_args(
    args: impl IntoIterator<Item = OsString>,
) -> Result<Option<BenchRailArgs>, String> {
    let mut rail = None;
    let mut out = None;
    let mut scene = None;
    let mut frames = None;
    let mut warmup = DEFAULT_WARMUP;
    let mut it = args.into_iter();
    let mut any = false;
    while let Some(flag) = it.next() {
        any = true;
        let flag = flag
            .into_string()
            .map_err(|f| format!("argument {f:?} is not UTF-8"))?;
        let mut value = |name: &str| it.next().ok_or_else(|| format!("{name} needs a value"));
        match flag.as_str() {
            "--bench-rail" => rail = Some(utf8(value("--bench-rail")?, "--bench-rail")?),
            "--out" => out = Some(PathBuf::from(value("--out")?)),
            "--scene" => scene = Some(utf8(value("--scene")?, "--scene")?),
            "--frames" => {
                let n = count(value("--frames")?, "--frames")?;
                if n == 0 {
                    return Err("--frames must be at least 1".to_string());
                }
                frames = Some(n);
            }
            "--warmup" => warmup = count(value("--warmup")?, "--warmup")?,
            other => return Err(format!("unknown argument {other:?}")),
        }
    }
    if !any {
        return Ok(None);
    }
    let rail = rail.ok_or("--bench-rail <rail-id> is required")?;
    if rail.is_empty() {
        return Err("--bench-rail needs a non-empty rail id".to_string());
    }
    let out = out.ok_or("--out <csv> is required")?;
    Ok(Some(BenchRailArgs {
        rail,
        out,
        scene,
        frames,
        warmup,
    }))
}

fn utf8(v: OsString, name: &str) -> Result<String, String> {
    v.into_string()
        .map_err(|v| format!("{name} value {v:?} is not UTF-8"))
}

fn count(v: OsString, name: &str) -> Result<u32, String> {
    let s = utf8(v, name)?;
    s.trim()
        .parse()
        .map_err(|e| format!("{name} value {s:?} is not a frame count: {e}"))
}

/// Measured frames for one rail loop of `duration_s` at `fps` rail steps per second of rail time
/// scaled by `time_scale` (each step advances the rail by `time_scale / fps` seconds).
pub fn loop_frames(duration_s: f32, fps: u32, time_scale: f32) -> u32 {
    let step = time_scale / fps.max(1) as f32;
    if !(step.is_finite() && step > 0.0 && duration_s.is_finite() && duration_s > 0.0) {
        return 1;
    }
    ((duration_s / step).ceil() as u32).max(1)
}

/// Progress of a benchmark run inside `DemoApp`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BenchPhase {
    /// Waiting for the scene's rails (`SceneMetaUpdated`).
    WaitingForRails,
    /// Rail selected, perf log started, lockstep frames flowing.
    Running,
    /// Finished: `Ok(csv)` or the reason it failed; the app exits.
    Done(Result<PathBuf, String>),
}

/// A benchmark run: its arguments and phase.
#[derive(Clone, Debug)]
pub struct BenchRun {
    pub args: BenchRailArgs,
    pub phase: BenchPhase,
}

impl BenchRun {
    pub fn new(args: BenchRailArgs) -> Self {
        Self {
            args,
            phase: BenchPhase::WaitingForRails,
        }
    }

    /// Lockstep rail frames are being rendered.
    pub fn is_running(&self) -> bool {
        self.phase == BenchPhase::Running
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn args(v: &[&str]) -> Result<Option<BenchRailArgs>, String> {
        parse_args(v.iter().map(OsString::from))
    }

    #[test]
    fn no_arguments_is_the_interactive_app() {
        assert_eq!(args(&[]), Ok(None));
    }

    #[test]
    fn parses_a_full_bench_command_line() {
        let a = args(&[
            "--bench-rail",
            "road_cruise",
            "--out",
            "x.csv",
            "--scene",
            "gundo",
            "--frames",
            "120",
            "--warmup",
            "10",
        ])
        .expect("valid")
        .expect("bench");
        assert_eq!(
            a,
            BenchRailArgs {
                rail: "road_cruise".into(),
                out: PathBuf::from("x.csv"),
                scene: Some("gundo".into()),
                frames: Some(120),
                warmup: 10,
            }
        );
        let d = args(&["--out", "y.csv", "--bench-rail", "r"])
            .expect("valid")
            .expect("bench");
        assert_eq!((d.frames, d.warmup, d.scene), (None, DEFAULT_WARMUP, None));
    }

    #[test]
    fn rejects_incomplete_or_unknown_arguments() {
        assert!(args(&["--bench-rail", "r"]).is_err(), "--out missing");
        assert!(args(&["--out", "x.csv"]).is_err(), "--bench-rail missing");
        assert!(args(&["--bench-rail"]).is_err(), "value missing");
        assert!(args(&["--bench-rail", "r", "--out", "x", "--frames", "0"]).is_err());
        assert!(args(&["--bench-rail", "r", "--out", "x", "--frames", "ten"]).is_err());
        assert!(args(&["--bench-rail", "r", "--out", "x", "--fast"]).is_err());
        assert!(args(&["--bench-rail", "", "--out", "x"]).is_err());
    }

    #[test]
    fn one_loop_covers_the_rail_duration_at_the_fixed_step() {
        assert_eq!(loop_frames(24.0, 60, 1.0), 1440);
        assert_eq!(loop_frames(24.0, 60, 2.0), 720);
        assert_eq!(loop_frames(0.0, 60, 1.0), 1);
        assert_eq!(loop_frames(10.0, 60, f32::NAN), 1);
    }
}
