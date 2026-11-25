//! Deterministic benchmarks: camera-rail (`--bench-rail`), frozen `flight-navigator-rs` track
//! (`--bench-track`) and playlist (`--bench-playlist`), and fixed camera set (`--bench-views`),
//! plus the `--is-counters` debug flag.
//!
//! **What.** Parses the command line ([`parse_args`] -> [`CliArgs`]) and holds the run state
//! ([`BenchRun`]). A rail, track, or playlist run loads a scene (tracks carry their own
//! `scene.id` metadata; `--scene` overrides it or picks the rail scene), freezes traffic and crowd
//! poses, steps the free camera (Pinhole, never possessed) in lockstep with the renderer at the
//! fixed `1 / 60 s * time_scale` clock (one rendered frame per step, the Smooth 60 FPS recording
//! path without a video file), records `warmup + frames` frames through
//! [`crate::perf_log::PerfLog`] and exits with the CSV written atomically.
//! A views run ([`ViewRun`]) visits a fixed list of cameras ([`ViewSpec`], a built-in set or a
//! file): per view it switches scene if needed, renders `warmup` frames, measures `frames`
//! frames (RT ms from `GpuPassProfiler`), then renders `count_frames` frames with the
//! intersection counters on (`crate::is_counters`) and writes one CSV row per view: RT ms median
//! / min / p95, intersection-program invocations per hit group and rays per frame.
//! **Why.** Free-running timings depend on wall-clock pacing, traffic positions and whatever the
//! camera happens to be doing; an A/B needs every run to render exactly the same cameras over
//! exactly the same geometry. Frozen `.fnav.json` tracks and `.fnav-playlist.json` playlists give
//! reproducible multi-scene flight-navigator benchmarks while `--bench-rail` remains for
//! historical `Gundo` baselines. The views run is the A/B harness of each csg-rs P37 wave (sweep
//! tiles replacing box chains): RT time and the intersection work it explains, side by side.
//! **How.** `main` parses the arguments before the event loop; `DemoApp::new` applies the scene
//! and freeze overrides and never writes the user config in bench mode; the rail/track/playlist
//! run starts when the scene metadata arrives (`RenderEvent::SceneMetaUpdated`), the views run
//! when the first view's scene is loaded; `RenderEvent::PerfLogWritten` (rail/track/playlist) or
//! the last view row (views) ends the process (exit code 0 on success). Counted frames are never
//! timed: the counters cost atomics, so RT ms comes from the measured frames before them.
//! **Used by** `main.rs`. Documented in `crates/nv-demo/AGENTS.md` (Benchmark).

use std::ffi::OsString;
use std::path::PathBuf;

use crate::is_counters::{IsCounts, IsGroup};

/// Usage text printed on a command-line error.
pub const USAGE: &str = "usage: nv-demo [--is-counters] [--bench-rail <rail-id> | --bench-track <file.fnav.json> | --bench-playlist <file.fnav-playlist.json> | --bench-views <set|file>] --out <csv> \
[--scene <scene-id>] [--frames <n>] [--warmup <n>] [--count-frames <n>] [--image <pfm>] [--png-dir <dir>]\n  \
--is-counters     count intersection-program invocations per hit group (debug; Flight stats shows them)\n  \
--bench-rail      scene rail cassette id (e.g. road_cruise); unknown ids list the available ones\n  \
--bench-track     frozen flight-navigator recording (*.fnav.json); carries its own scene.id\n  \
--bench-playlist  frozen flight-navigator playlist (*.fnav-playlist.json) of track recordings\n  \
--bench-views     fixed camera set: a built-in set (p37) or a file of `name scene viewpoint <id>` /\n                    \
`name scene eye x y z target x y z fov deg` lines\n  \
--out             CSV written when the run ends (atomic rename)\n  \
--scene           scene descriptor id for rail/track/playlist (rail default: saved scene; track/playlist default: track's scene.id); views name their scene\n  \
--frames          measured frames (rail/track/playlist default: full duration at 60 fps * time_scale; views default 60 per view)\n  \
--warmup          frames rendered and discarded before measuring (rail/track/playlist default 60, views default 30 per view)\n  \
--count-frames    views: frames per view rendered with the IS counters on after the measured ones (default 4)\n  \
--image           rail/track/playlist: linear colour of the last frame as PFM (pixel A/B)\n  \
--png-dir         rail/track/playlist: every frame's displayed image as PNG (temporal A/B)";

/// Default number of discarded frames at the start of the rail.
pub const DEFAULT_WARMUP: u32 = 60;
/// Default warm-up frames per view of a views run.
pub const DEFAULT_VIEW_WARMUP: u32 = 30;
/// Default measured frames per view of a views run.
pub const DEFAULT_VIEW_FRAMES: u32 = 60;
/// Default counted frames per view of a views run.
pub const DEFAULT_COUNT_FRAMES: u32 = 4;
/// Extra frames a view waits for the counts of its last counted frames (they arrive one frame
/// later, after the readback); a view that still misses counts reports how many it got.
const COUNT_DRAIN_FRAMES: u32 = 4;

/// The built-in camera set for the csg-rs P37 A/B: a gundo_osm bookmark, Marina del Rey at
/// street level, a grazing view along a street and a top-down view (cameras chosen from the
/// scenes' own viewpoints; coordinates in metres, fov in degrees).
pub const P37_VIEWS: &str = include_str!("../bench_views/p37.views");

/// Built-in view sets by name.
pub const BUILTIN_VIEW_SETS: &[(&str, &str)] = &[("p37", P37_VIEWS)];

/// How a view's camera is defined.
#[derive(Clone, Debug, PartialEq)]
pub enum ViewCam {
    /// A curated scene viewpoint by id (`SceneViewpoint::id`).
    Viewpoint(String),
    /// An explicit eye, look-at target and vertical fov in degrees.
    Explicit { eye: [f32; 3], target: [f32; 3], vfov_deg: f32 },
}

/// One camera of a views run.
#[derive(Clone, Debug, PartialEq)]
pub struct ViewSpec {
    /// CSV row name.
    pub name: String,
    /// Scene descriptor id the view belongs to.
    pub scene: String,
    /// The camera.
    pub cam: ViewCam,
}

/// Parses a view list: one view per line, `#` comments, blank lines ignored.
/// `name scene viewpoint <id>` or `name scene eye x y z target x y z fov deg`.
pub fn parse_views(text: &str) -> Result<Vec<ViewSpec>, String> {
    let mut out = Vec::new();
    for (ln, raw) in text.lines().enumerate() {
        let line = raw.split('#').next().unwrap_or("").trim();
        if line.is_empty() {
            continue;
        }
        let t: Vec<&str> = line.split_whitespace().collect();
        let err = |m: &str| format!("view line {}: {m}: {raw:?}", ln + 1);
        let num = |s: &str| s.parse::<f32>().map_err(|e| err(&format!("{s:?} is not a number ({e})")));
        let cam = match t.as_slice() {
            [_, _, "viewpoint", id] => ViewCam::Viewpoint((*id).to_string()),
            [_, _, "eye", ex, ey, ez, "target", tx, ty, tz, "fov", f] => ViewCam::Explicit {
                eye: [num(ex)?, num(ey)?, num(ez)?],
                target: [num(tx)?, num(ty)?, num(tz)?],
                vfov_deg: num(f)?,
            },
            _ => return Err(err("expected `name scene viewpoint <id>` or `name scene eye x y z target x y z fov deg`")),
        };
        if out.iter().any(|v: &ViewSpec| v.name == t[0]) {
            return Err(err("duplicate view name"));
        }
        out.push(ViewSpec { name: t[0].to_string(), scene: t[1].to_string(), cam });
    }
    if out.is_empty() {
        return Err("the view list is empty".to_string());
    }
    Ok(out)
}

/// Resolves `--bench-views`: a built-in set name, else a file path.
pub fn load_views(arg: &str) -> Result<Vec<ViewSpec>, String> {
    if let Some((_, text)) = BUILTIN_VIEW_SETS.iter().find(|(n, _)| *n == arg) {
        return parse_views(text);
    }
    let text = std::fs::read_to_string(arg).map_err(|e| {
        let names: Vec<&str> = BUILTIN_VIEW_SETS.iter().map(|(n, _)| *n).collect();
        format!("--bench-views {arg:?}: not a built-in set ({}) and not a readable file ({e})", names.join(", "))
    })?;
    parse_views(&text)
}

/// What a benchmark run renders.
#[derive(Clone, Debug, PartialEq)]
pub enum BenchTarget {
    /// One scene rail cassette (`RailSpec::id`), kept for historical `Gundo` baselines.
    Rail(String),
    /// A frozen `flight-navigator-rs` track recording (`*.fnav.json`) carrying its own `scene.id`.
    Track(PathBuf),
    /// A frozen `flight-navigator-rs` playlist (`*.fnav-playlist.json`) of track recordings.
    Playlist(PathBuf),
    /// A fixed camera set (`--bench-views <set|file>`).
    Views(Vec<ViewSpec>),
}

/// Benchmark command line.
#[derive(Clone, Debug, PartialEq)]
pub struct BenchArgs {
    /// Rail, frozen flight track, playlist, or camera set.
    pub target: BenchTarget,
    /// CSV output path.
    pub out: PathBuf,
    /// Scene descriptor id for rail/track/playlist runs; `None` keeps the saved scene (rail) or
    /// uses the track's embedded `scene.id` (track/playlist).
    pub scene: Option<String>,
    /// Measured frames (rail/track/playlist: `None` = full duration; views: per view).
    pub frames: Option<u32>,
    /// Discarded frames before measuring (rail/track/playlist: once; views: per view).
    pub warmup: u32,
    /// Views: counted frames per view.
    pub count_frames: u32,
    /// PFM written with the last measured frame's linear colour (rail/track/playlist).
    pub image: Option<PathBuf>,
    /// Directory receiving every rendered frame's displayed image (tonemap + overlay) as
    /// `frame_NNNNN.png` (rail/track/playlist): temporal artefacts such as line shimmer need a
    /// sequence.
    pub png_dir: Option<PathBuf>,
}

/// The whole command line.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CliArgs {
    /// A benchmark run; `None` = the interactive app.
    pub bench: Option<BenchArgs>,
    /// `--is-counters`: start with the intersection counters on.
    pub is_counters: bool,
}

/// Parses the process arguments after the program name. No arguments = the interactive app;
/// any unknown argument is an error.
pub fn parse_args(args: impl IntoIterator<Item = OsString>) -> Result<CliArgs, String> {
    let mut rail = None;
    let mut track = None;
    let mut playlist = None;
    let mut views = None;
    let mut out = None;
    let mut scene = None;
    let mut frames = None;
    let mut warmup = None;
    let mut count_frames = None;
    let mut image = None;
    let mut png_dir = None;
    let mut is_counters = false;
    let mut any_bench = false;
    let mut it = args.into_iter();
    while let Some(flag) = it.next() {
        let flag = flag
            .into_string()
            .map_err(|f| format!("argument {f:?} is not UTF-8"))?;
        let mut value = |name: &str| it.next().ok_or_else(|| format!("{name} needs a value"));
        if flag != "--is-counters" {
            any_bench = true;
        }
        match flag.as_str() {
            "--is-counters" => is_counters = true,
            "--bench-rail" => rail = Some(utf8(value("--bench-rail")?, "--bench-rail")?),
            "--bench-track" => track = Some(PathBuf::from(value("--bench-track")?)),
            "--bench-playlist" => playlist = Some(PathBuf::from(value("--bench-playlist")?)),
            "--bench-views" => views = Some(utf8(value("--bench-views")?, "--bench-views")?),
            "--out" => out = Some(PathBuf::from(value("--out")?)),
            "--scene" => scene = Some(utf8(value("--scene")?, "--scene")?),
            "--frames" => {
                let n = count(value("--frames")?, "--frames")?;
                if n == 0 {
                    return Err("--frames must be at least 1".to_string());
                }
                frames = Some(n);
            }
            "--warmup" => warmup = Some(count(value("--warmup")?, "--warmup")?),
            "--count-frames" => {
                let n = count(value("--count-frames")?, "--count-frames")?;
                if n == 0 {
                    return Err("--count-frames must be at least 1".to_string());
                }
                count_frames = Some(n);
            }
            "--image" => image = Some(PathBuf::from(value("--image")?)),
            "--png-dir" => png_dir = Some(PathBuf::from(value("--png-dir")?)),
            other => return Err(format!("unknown argument {other:?}")),
        }
    }
    if !any_bench {
        return Ok(CliArgs { bench: None, is_counters });
    }
    let chosen_flags: Vec<&str> = [
        rail.is_some().then_some("--bench-rail"),
        track.is_some().then_some("--bench-track"),
        playlist.is_some().then_some("--bench-playlist"),
        views.is_some().then_some("--bench-views"),
    ]
    .into_iter()
    .flatten()
    .collect();
    if chosen_flags.len() > 1 {
        return Err(format!(
            "{} are mutually exclusive",
            chosen_flags.join(", ")
        ));
    }
    let target = match (rail, track, playlist, views) {
        (Some(r), None, None, None) => {
            if r.is_empty() {
                return Err("--bench-rail needs a non-empty rail id".to_string());
            }
            if count_frames.is_some() {
                return Err("--count-frames applies to --bench-views only".to_string());
            }
            BenchTarget::Rail(r)
        }
        (None, Some(p), None, None) => {
            if p.as_os_str().is_empty() {
                return Err("--bench-track needs a non-empty file path".to_string());
            }
            if count_frames.is_some() {
                return Err("--count-frames applies to --bench-views only".to_string());
            }
            BenchTarget::Track(p)
        }
        (None, None, Some(p), None) => {
            if p.as_os_str().is_empty() {
                return Err("--bench-playlist needs a non-empty file path".to_string());
            }
            if count_frames.is_some() {
                return Err("--count-frames applies to --bench-views only".to_string());
            }
            BenchTarget::Playlist(p)
        }
        (None, None, None, Some(v)) => {
            if scene.is_some() || image.is_some() || png_dir.is_some() {
                return Err("--scene / --image / --png-dir do not apply to --bench-views (views name their scene)".to_string());
            }
            BenchTarget::Views(load_views(&v)?)
        }
        _ => {
            return Err(
                "one of --bench-rail <rail-id>, --bench-track <file.fnav.json>, --bench-playlist <file.fnav-playlist.json>, or --bench-views <set|file> is required"
                    .to_string(),
            );
        }
    };
    let out = out.ok_or("--out <csv> is required")?;
    let views = matches!(target, BenchTarget::Views(_));
    Ok(CliArgs {
        bench: Some(BenchArgs {
            frames: frames.or(views.then_some(DEFAULT_VIEW_FRAMES)),
            warmup: warmup.unwrap_or(if views { DEFAULT_VIEW_WARMUP } else { DEFAULT_WARMUP }),
            count_frames: count_frames.unwrap_or(DEFAULT_COUNT_FRAMES),
            target,
            out,
            scene,
            image,
            png_dir,
        }),
        is_counters,
    })
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
    /// Waiting for the scene's rails (`SceneMetaUpdated`) or, for views, the first scene.
    WaitingForRails,
    /// Rail selected / views started, lockstep frames flowing.
    Running,
    /// Finished: `Ok(csv)` or the reason it failed; the app exits.
    Done(Result<PathBuf, String>),
}

/// A benchmark run: its arguments, phase and (views) the per-view state.
#[derive(Clone, Debug)]
pub struct BenchRun {
    pub args: BenchArgs,
    pub phase: BenchPhase,
    /// Views run state (`None` for a rail run).
    pub views: Option<ViewRun>,
}

impl BenchRun {
    pub fn new(args: BenchArgs) -> Self {
        let views = match &args.target {
            BenchTarget::Views(v) => Some(ViewRun::new(
                v.clone(),
                args.warmup,
                args.frames.unwrap_or(DEFAULT_VIEW_FRAMES),
                args.count_frames,
            )),
            BenchTarget::Rail(_) | BenchTarget::Track(_) | BenchTarget::Playlist(_) => None,
        };
        Self { args, phase: BenchPhase::WaitingForRails, views }
    }

    /// Lockstep frames are being rendered.
    pub fn is_running(&self) -> bool {
        self.phase == BenchPhase::Running
    }
}

/// Phase of the current view.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ViewPhase {
    /// The view's scene is loading (frames are ignored).
    WaitScene,
    /// Discarded frames (accumulation, caches, profiler lag settle).
    Warmup,
    /// Timed frames (counters off).
    Measure,
    /// Frames rendered with the IS counters on.
    Count,
    /// Counters off again, waiting for the last counts to arrive.
    Drain,
}

/// What the host must do after [`ViewRun::on_frame`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ViewEvent {
    /// Keep rendering.
    None,
    /// Switch the intersection counters on (`true`) or off.
    SetCounters(bool),
    /// The view is finished (row recorded): start view `next`, or write the CSV when it is past
    /// the last view.
    ViewDone { next: usize },
}

/// One CSV row of a views run.
#[derive(Clone, Debug, PartialEq)]
pub struct ViewRow {
    pub name: String,
    pub scene: String,
    pub state: crate::perf_log::PerfState,
    pub frames: u32,
    pub rt_median: f32,
    pub rt_min: f32,
    pub rt_p95: f32,
    /// Counted frames whose counts arrived.
    pub counted: u32,
    /// Counts summed over the counted frames.
    pub counts: IsCounts,
}

/// State machine of a views run, driven by one call per rendered frame.
#[derive(Clone, Debug)]
pub struct ViewRun {
    pub views: Vec<ViewSpec>,
    warmup: u32,
    frames: u32,
    count_frames: u32,
    /// Index of the current view.
    pub idx: usize,
    pub phase: ViewPhase,
    n: u32,
    rt: Vec<f32>,
    counts: IsCounts,
    counted: u32,
    state: crate::perf_log::PerfState,
    pub rows: Vec<ViewRow>,
}

impl ViewRun {
    pub fn new(views: Vec<ViewSpec>, warmup: u32, frames: u32, count_frames: u32) -> Self {
        Self {
            rows: Vec::with_capacity(views.len()),
            views,
            warmup,
            frames: frames.max(1),
            count_frames: count_frames.max(1),
            idx: 0,
            phase: ViewPhase::WaitScene,
            n: 0,
            rt: Vec::with_capacity(frames.max(1) as usize),
            counts: IsCounts::default(),
            counted: 0,
            state: crate::perf_log::PerfState::default(),
        }
    }

    /// The camera of view `idx` is in place: start its warm-up.
    pub fn begin_view(&mut self) {
        self.phase = if self.warmup == 0 { ViewPhase::Measure } else { ViewPhase::Warmup };
        self.n = 0;
        self.rt.clear();
        self.counts = IsCounts::default();
        self.counted = 0;
    }

    /// One rendered frame: its RT ms, launch state and the counts that arrived with it.
    pub fn on_frame(&mut self, rt_ms: f32, state: crate::perf_log::PerfState, counts: Option<IsCounts>) -> ViewEvent {
        if let Some(c) = counts
            && matches!(self.phase, ViewPhase::Count | ViewPhase::Drain)
        {
            self.counts.accumulate(&c);
            self.counted += 1;
        }
        self.n += 1;
        match self.phase {
            ViewPhase::WaitScene => ViewEvent::None,
            ViewPhase::Warmup => {
                if self.n >= self.warmup {
                    self.phase = ViewPhase::Measure;
                    self.n = 0;
                }
                ViewEvent::None
            }
            ViewPhase::Measure => {
                self.rt.push(rt_ms);
                self.state = state;
                if self.n >= self.frames {
                    self.phase = ViewPhase::Count;
                    self.n = 0;
                    return ViewEvent::SetCounters(true);
                }
                ViewEvent::None
            }
            ViewPhase::Count => {
                if self.n >= self.count_frames {
                    self.phase = ViewPhase::Drain;
                    self.n = 0;
                    return ViewEvent::SetCounters(false);
                }
                ViewEvent::None
            }
            ViewPhase::Drain => {
                if self.counted >= self.count_frames || self.n >= COUNT_DRAIN_FRAMES {
                    self.finish_view();
                    return ViewEvent::ViewDone { next: self.idx };
                }
                ViewEvent::None
            }
        }
    }

    fn finish_view(&mut self) {
        let mut rt = self.rt.clone();
        rt.sort_by(f32::total_cmp);
        let pick = |q: f32| rt.get(((rt.len() as f32 - 1.0) * q).round() as usize).copied().unwrap_or(0.0);
        let v = &self.views[self.idx];
        self.rows.push(ViewRow {
            name: v.name.clone(),
            scene: v.scene.clone(),
            state: self.state,
            frames: rt.len() as u32,
            rt_median: pick(0.5),
            rt_min: rt.first().copied().unwrap_or(0.0),
            rt_p95: pick(0.95),
            counted: self.counted,
            counts: self.counts,
        });
        self.idx += 1;
        self.phase = ViewPhase::WaitScene;
    }
}

/// Writes the views CSV (temporary file, `sync_all`, rename). IS columns are per counted frame.
pub fn write_views_csv(path: &std::path::Path, rows: &[ViewRow]) -> Result<(), String> {
    use std::io::Write;
    let io = |e: std::io::Error| format!("views csv {}: {e}", path.display());
    let mut tmp_name = path.file_name().unwrap_or_default().to_os_string();
    tmp_name.push(format!(".tmp.{}", std::process::id()));
    let tmp = path.with_file_name(tmp_name);
    let written = (|| -> std::io::Result<()> {
        let mut out = std::io::BufWriter::new(std::fs::File::create(&tmp)?);
        write!(out, "view,scene,width,height,spp,bounces,eye_x,eye_y,eye_z,frames,rt_ms_median,rt_ms_min,rt_ms_p95,counted_frames")?;
        for g in IsGroup::ALL {
            write!(out, ",{}", g.label())?;
        }
        writeln!(out, ",is_total")?;
        for r in rows {
            let st = &r.state;
            write!(
                out,
                "{},{},{},{},{},{},{:.2},{:.2},{:.2},{},{:.4},{:.4},{:.4},{}",
                r.name, r.scene, st.width, st.height, st.spp, st.bounces, st.eye[0], st.eye[1], st.eye[2],
                r.frames, r.rt_median, r.rt_min, r.rt_p95, r.counted
            )?;
            let per = |v: u64| if r.counted == 0 { 0 } else { v / u64::from(r.counted) };
            for g in IsGroup::ALL {
                write!(out, ",{}", per(r.counts.get(g)))?;
            }
            writeln!(out, ",{}", per(r.counts.total_is()))?;
        }
        out.into_inner().map_err(std::io::IntoInnerError::into_error)?.sync_all()
    })()
    .map_err(io)
    .and_then(|()| std::fs::rename(&tmp, path).map_err(io));
    if written.is_err()
        && let Err(e) = std::fs::remove_file(&tmp)
        && e.kind() != std::io::ErrorKind::NotFound
    {
        eprintln!("nv-demo: views csv temp file {} not removed: {e}", tmp.display());
    }
    written
}

#[cfg(test)]
mod tests {
    use super::*;

    fn args(v: &[&str]) -> Result<CliArgs, String> {
        parse_args(v.iter().map(OsString::from))
    }

    #[test]
    fn no_arguments_is_the_interactive_app() {
        assert_eq!(args(&[]), Ok(CliArgs::default()));
        assert_eq!(args(&["--is-counters"]), Ok(CliArgs { bench: None, is_counters: true }));
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
        .bench
        .expect("bench");
        assert_eq!(
            a,
            BenchArgs {
                target: BenchTarget::Rail("road_cruise".into()),
                out: PathBuf::from("x.csv"),
                scene: Some("gundo".into()),
                frames: Some(120),
                warmup: 10,
                count_frames: DEFAULT_COUNT_FRAMES,
                image: None,
                png_dir: None,
            }
        );
        let d = args(&["--out", "y.csv", "--bench-rail", "r"]).expect("valid").bench.expect("bench");
        assert_eq!((d.frames, d.warmup, d.scene, d.image), (None, DEFAULT_WARMUP, None, None));
        let i = args(&["--bench-rail", "r", "--out", "o.csv", "--image", "x.pfm"]).expect("valid").bench.expect("bench");
        assert_eq!(i.image, Some(PathBuf::from("x.pfm")));
    }

    #[test]
    fn parses_the_builtin_views_with_view_defaults() {
        let a = args(&["--bench-views", "p37", "--out", "v.csv", "--is-counters"]).expect("valid");
        assert!(a.is_counters);
        let b = a.bench.expect("bench");
        let BenchTarget::Views(v) = &b.target else { panic!("views target") };
        assert!(v.len() >= 4, "p37 has at least 4 views");
        for scene in ["gundo_osm", "marina"] {
            assert!(v.iter().any(|x| x.scene.starts_with(scene)), "p37 covers {scene}");
        }
        assert_eq!((b.frames, b.warmup, b.count_frames), (Some(DEFAULT_VIEW_FRAMES), DEFAULT_VIEW_WARMUP, DEFAULT_COUNT_FRAMES));
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
        assert!(args(&["--bench-rail", "r", "--bench-views", "p37", "--out", "x"]).is_err());
        assert!(args(&["--bench-views", "p37", "--scene", "gundo", "--out", "x"]).is_err());
        assert!(args(&["--bench-views", "no_such_set_or_file", "--out", "x"]).is_err());
        assert!(args(&["--bench-rail", "r", "--out", "x", "--count-frames", "2"]).is_err());
    }

    #[test]
    fn parses_bench_track_and_playlist_with_optional_scene_and_outputs() {
        let t = args(&["--bench-track", "tracks/tour.fnav.json", "--out", "t.csv"])
            .expect("valid track")
            .bench
            .expect("bench");
        assert_eq!(
            t,
            BenchArgs {
                target: BenchTarget::Track(PathBuf::from("tracks/tour.fnav.json")),
                out: PathBuf::from("t.csv"),
                scene: None,
                frames: None,
                warmup: DEFAULT_WARMUP,
                count_frames: DEFAULT_COUNT_FRAMES,
                image: None,
                png_dir: None,
            }
        );
        assert!(BenchRun::new(t).views.is_none());

        let t_full = args(&[
            "--bench-track",
            "tracks/tour.fnav.json",
            "--out",
            "t.csv",
            "--scene",
            "gundo",
            "--frames",
            "240",
            "--warmup",
            "15",
            "--image",
            "last.pfm",
            "--png-dir",
            "frames_out",
        ])
        .expect("valid full track")
        .bench
        .expect("bench");
        assert_eq!(
            t_full,
            BenchArgs {
                target: BenchTarget::Track(PathBuf::from("tracks/tour.fnav.json")),
                out: PathBuf::from("t.csv"),
                scene: Some("gundo".into()),
                frames: Some(240),
                warmup: 15,
                count_frames: DEFAULT_COUNT_FRAMES,
                image: Some(PathBuf::from("last.pfm")),
                png_dir: Some(PathBuf::from("frames_out")),
            }
        );

        let p = args(&[
            "--bench-playlist",
            "lists/suite.fnav-playlist.json",
            "--out",
            "p.csv",
            "--image",
            "p.pfm",
            "--png-dir",
            "p_pngs",
        ])
        .expect("valid playlist")
        .bench
        .expect("bench");
        assert_eq!(
            p,
            BenchArgs {
                target: BenchTarget::Playlist(PathBuf::from("lists/suite.fnav-playlist.json")),
                out: PathBuf::from("p.csv"),
                scene: None,
                frames: None,
                warmup: DEFAULT_WARMUP,
                count_frames: DEFAULT_COUNT_FRAMES,
                image: Some(PathBuf::from("p.pfm")),
                png_dir: Some(PathBuf::from("p_pngs")),
            }
        );
        assert!(BenchRun::new(p).views.is_none());
    }

    #[test]
    fn rejects_mutually_exclusive_targets_and_invalid_track_playlist_flags() {
        let pairs: &[&[&str]] = &[
            &["--bench-rail", "r", "--bench-track", "a.fnav.json", "--out", "x.csv"],
            &["--bench-rail", "r", "--bench-playlist", "a.fnav-playlist.json", "--out", "x.csv"],
            &["--bench-track", "a.fnav.json", "--bench-playlist", "b.fnav-playlist.json", "--out", "x.csv"],
            &["--bench-track", "a.fnav.json", "--bench-views", "p37", "--out", "x.csv"],
            &["--bench-playlist", "a.fnav-playlist.json", "--bench-views", "p37", "--out", "x.csv"],
            &["--bench-rail", "r", "--bench-track", "a.fnav.json", "--bench-views", "p37", "--out", "x.csv"],
        ];
        for argv in pairs {
            let err = args(argv).expect_err("mutually exclusive targets must fail");
            assert!(
                err.contains("mutually exclusive"),
                "expected mutual-exclusion error for {argv:?}, got: {err}"
            );
        }
        assert!(args(&["--bench-track", "", "--out", "x.csv"]).is_err());
        assert!(args(&["--bench-playlist", "", "--out", "x.csv"]).is_err());
        assert!(args(&["--bench-track", "a.fnav.json"]).is_err(), "--out missing");
        assert!(args(&["--bench-playlist", "a.fnav-playlist.json"]).is_err(), "--out missing");
        assert!(
            args(&["--bench-track", "a.fnav.json", "--out", "x.csv", "--count-frames", "2"])
                .is_err()
        );
        assert!(
            args(&[
                "--bench-playlist",
                "a.fnav-playlist.json",
                "--out",
                "x.csv",
                "--count-frames",
                "2",
            ])
            .is_err()
        );
    }

    #[test]
    fn view_lines_parse_and_reject_garbage() {
        let v = parse_views(
            "# comment\n a gundo viewpoint vp_1 \n b marina eye 1 2 3 target 4 5 6 fov 50 # tail\n",
        )
        .expect("valid");
        assert_eq!(v[0], ViewSpec { name: "a".into(), scene: "gundo".into(), cam: ViewCam::Viewpoint("vp_1".into()) });
        assert_eq!(v[1].cam, ViewCam::Explicit { eye: [1.0, 2.0, 3.0], target: [4.0, 5.0, 6.0], vfov_deg: 50.0 });
        assert!(parse_views("").is_err());
        assert!(parse_views("a gundo eye 1 2 target 4 5 6 fov 50").is_err());
        assert!(parse_views("a gundo eye 1 2 x target 4 5 6 fov 50").is_err());
        assert!(parse_views("a g viewpoint x\na g viewpoint y").is_err(), "duplicate names");
    }

    #[test]
    fn view_run_measures_then_counts_then_moves_on() {
        let views = parse_views("a g viewpoint x\nb g viewpoint y").expect("views");
        let mut run = ViewRun::new(views, 2, 3, 2);
        let st = crate::perf_log::PerfState { width: 8, ..Default::default() };
        let c = {
            let mut w = [0u64; crate::is_counters::IS_WORDS];
            w[IsGroup::CsgPrim as usize * crate::is_counters::IS_STRIPES] = 10;
            IsCounts::from_stripes(&w)
        };
        assert_eq!(run.on_frame(9.0, st, None), ViewEvent::None, "waiting for the scene");
        run.begin_view();
        assert_eq!(run.on_frame(99.0, st, None), ViewEvent::None);
        assert_eq!(run.on_frame(99.0, st, None), ViewEvent::None, "warm-up done");
        assert_eq!(run.on_frame(3.0, st, None), ViewEvent::None);
        assert_eq!(run.on_frame(1.0, st, None), ViewEvent::None);
        assert_eq!(run.on_frame(2.0, st, None), ViewEvent::SetCounters(true));
        assert_eq!(run.on_frame(50.0, st, None), ViewEvent::None, "first counted frame, counts not back yet");
        assert_eq!(run.on_frame(50.0, st, Some(c)), ViewEvent::SetCounters(false));
        assert_eq!(run.on_frame(50.0, st, Some(c)), ViewEvent::ViewDone { next: 1 });
        let row = &run.rows[0];
        assert_eq!((row.frames, row.rt_median, row.rt_min, row.rt_p95), (3, 2.0, 1.0, 3.0));
        assert_eq!((row.counted, row.counts.get(IsGroup::CsgPrim)), (2, 20));
        assert_eq!(row.state.width, 8);
        assert_eq!(run.phase, ViewPhase::WaitScene);

        let dir = std::env::temp_dir().join(format!("nv_demo_views_{}", std::process::id()));
        std::fs::create_dir_all(&dir).expect("dir");
        let path = dir.join("v.csv");
        write_views_csv(&path, &run.rows).expect("csv");
        let text = std::fs::read_to_string(&path).expect("read");
        let lines: Vec<&str> = text.lines().collect();
        assert_eq!(lines.len(), 2);
        assert!(lines[0].starts_with("view,scene,width,height,spp,bounces"));
        assert!(lines[1].starts_with("a,g,8,"));
        assert!(lines[1].contains(",10,"), "counts are per counted frame: {}", lines[1]);
        std::fs::remove_dir_all(&dir).expect("cleanup");
    }

    #[test]
    fn one_loop_covers_the_rail_duration_at_the_fixed_step() {
        assert_eq!(loop_frames(24.0, 60, 1.0), 1440);
        assert_eq!(loop_frames(24.0, 60, 2.0), 720);
        assert_eq!(loop_frames(0.0, 60, 1.0), 1);
        assert_eq!(loop_frames(10.0, 60, f32::NAN), 1);
    }
}
