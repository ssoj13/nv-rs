//! Zero-overhead runtime diagnostics, library probing, and event ring-buffer for `nv-rs`.
//!
//! **What.** Tracks dynamic library loads, failures, GPU memory allocations/frees, and
//! arbitrary subsystem trace events in a bounded in-memory ring-buffer (`256` entries) with
//! atomic counters (`DiagStats`) and optional stderr logging.
//!
//! **Why.** Debugging missing DLL/SO paths, ABI mismatches, GPU memory leaks, and FFI call
//! latencies across 20+ NVIDIA libraries requires one shared diagnostic bus accessible from
//! both Rust crates and the Python API (`nv_rs.diag`).
//!
//! **How.** The active [`Level`] is stored in an [`AtomicU8`] initialized once from
//! `NV_RS_DEBUG` (or `NV_RS_LOG`: `off`, `error`, `warn`, `info`, `trace`, `1`). When the
//! level is [`Level::Off`], [`is_enabled`] is a single relaxed atomic load with zero
//! allocations. Counters remain always-on via atomics so `stats()` is always accurate.

use std::collections::VecDeque;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU8, AtomicU64, Ordering};
use std::sync::{Mutex, OnceLock};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::{DllSpec, Probe, SystemProbe, candidates_with};

const RING_CAPACITY: usize = 256;

/// Runtime diagnostic verbosity level.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum Level {
    /// No event recording or stderr logging (atomic counters remain active).
    Off = 0,
    /// Record and log errors only.
    Error = 1,
    /// Record and log warnings and errors.
    Warn = 2,
    /// Record and log library loads, key lifecycle ops, warnings, and errors.
    Info = 3,
    /// Record and log all FFI, buffer, and kernel/stream operations.
    Trace = 4,
}

impl Level {
    /// Parse a case-insensitive level string (`off`, `error`, `warn`, `info`, `trace`, `1`).
    pub fn parse(raw: &str) -> Option<Self> {
        match raw.trim().to_ascii_lowercase().as_str() {
            "0" | "off" | "none" | "false" => Some(Self::Off),
            "1" | "err" | "error" => Some(Self::Error),
            "2" | "warn" | "warning" => Some(Self::Warn),
            "3" | "info" => Some(Self::Info),
            "4" | "trace" | "debug" | "true" => Some(Self::Trace),
            _ => None,
        }
    }

    /// Short uppercase label for logs and Python formatting.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Off => "OFF",
            Self::Error => "ERROR",
            Self::Warn => "WARN",
            Self::Info => "INFO",
            Self::Trace => "TRACE",
        }
    }

    fn from_u8(v: u8) -> Self {
        match v {
            1 => Self::Error,
            2 => Self::Warn,
            3 => Self::Info,
            4.. => Self::Trace,
            _ => Self::Off,
        }
    }
}

/// One recorded diagnostic event in the ring-buffer.
#[derive(Clone, Debug)]
pub struct DiagEvent {
    /// Monotonic sequence number.
    pub seq: u64,
    /// Unix epoch timestamp in milliseconds.
    pub timestamp_ms: u64,
    /// Event severity level.
    pub level: Level,
    /// Subsystem or crate category (`"loader"`, `"core"`, `"nvrtc"`, `"nvcomp"`, ...).
    pub category: String,
    /// Operation name (`"open_first"`, `"alloc"`, `"compile"`, ...).
    pub op: String,
    /// Human-readable detail or path.
    pub detail: String,
    /// Duration in nanoseconds (`0` when not timed).
    pub elapsed_ns: u64,
    /// Whether the operation succeeded.
    pub ok: bool,
}

/// Cumulative runtime counters across the process.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct DiagStats {
    /// Successful dynamic library loads through `nv-loader`.
    pub lib_loads: u64,
    /// Failed dynamic library load attempts through `nv-loader`.
    pub lib_load_failures: u64,
    /// Total GPU buffer allocations tracked.
    pub gpu_alloc_count: u64,
    /// Total GPU buffer frees tracked.
    pub gpu_free_count: u64,
    /// Cumulative bytes allocated on GPU.
    pub gpu_alloc_bytes: u64,
    /// Currently live bytes on GPU.
    pub gpu_live_bytes: u64,
    /// Peak live bytes observed on GPU.
    pub gpu_peak_bytes: u64,
    /// Total recorded errors.
    pub errors: u64,
    /// Most recent error message, if any.
    pub last_error: Option<String>,
}

/// Non-invasive filesystem probe result for one [`DllSpec`].
#[derive(Clone, Debug)]
pub struct ProbeResult {
    /// Short library name (`spec.lib`).
    pub lib: &'static str,
    /// All candidate paths/sonames in priority order.
    pub candidates: Vec<PathBuf>,
    /// First candidate that exists on disk (Windows) or first candidate soname (Linux).
    pub first_present: Option<PathBuf>,
}

struct State {
    level: AtomicU8,
    seq: AtomicU64,
    lib_loads: AtomicU64,
    lib_load_failures: AtomicU64,
    gpu_alloc_count: AtomicU64,
    gpu_free_count: AtomicU64,
    gpu_alloc_bytes: AtomicU64,
    gpu_live_bytes: AtomicU64,
    gpu_peak_bytes: AtomicU64,
    errors: AtomicU64,
    last_error: Mutex<Option<String>>,
    ring: Mutex<VecDeque<DiagEvent>>,
}

fn state() -> &'static State {
    static STATE: OnceLock<State> = OnceLock::new();
    STATE.get_or_init(|| {
        let env_val = std::env::var("NV_RS_DEBUG")
            .or_else(|_| std::env::var("NV_RS_LOG"))
            .ok();
        let initial = env_val
            .as_deref()
            .and_then(Level::parse)
            .unwrap_or(Level::Off);
        State {
            level: AtomicU8::new(initial as u8),
            seq: AtomicU64::new(1),
            lib_loads: AtomicU64::new(0),
            lib_load_failures: AtomicU64::new(0),
            gpu_alloc_count: AtomicU64::new(0),
            gpu_free_count: AtomicU64::new(0),
            gpu_alloc_bytes: AtomicU64::new(0),
            gpu_live_bytes: AtomicU64::new(0),
            gpu_peak_bytes: AtomicU64::new(0),
            errors: AtomicU64::new(0),
            last_error: Mutex::new(None),
            ring: Mutex::new(VecDeque::with_capacity(RING_CAPACITY)),
        }
    })
}

/// Returns the current global diagnostic [`Level`].
#[inline]
pub fn level() -> Level {
    Level::from_u8(state().level.load(Ordering::Relaxed))
}

/// Sets the global diagnostic [`Level`] at runtime.
pub fn set_level(new_level: Level) {
    state().level.store(new_level as u8, Ordering::Relaxed);
}

/// Fast-path check for whether `msg_level` is enabled by the current [`level()`].
#[inline]
pub fn is_enabled(msg_level: Level) -> bool {
    msg_level != Level::Off && (msg_level as u8) <= state().level.load(Ordering::Relaxed)
}

fn now_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}

/// Records a diagnostic event if `msg_level` is active; also updates error counters on `!ok`.
pub fn record_event(
    msg_level: Level,
    category: &str,
    op: &str,
    detail: impl Into<String>,
    elapsed_ns: u64,
    ok: bool,
) {
    let detail = detail.into();
    let st = state();
    if !ok {
        st.errors.fetch_add(1, Ordering::Relaxed);
        if let Ok(mut guard) = st.last_error.lock() {
            *guard = Some(format!("{category}::{op}: {detail}"));
        }
    }
    if !is_enabled(msg_level) {
        return;
    }
    let seq = st.seq.fetch_add(1, Ordering::Relaxed);
    let ts = now_ms();
    if elapsed_ns > 0 {
        eprintln!(
            "[nv-rs {} {category}::{op}] {detail} ({:.3}ms)",
            msg_level.as_str(),
            elapsed_ns as f64 / 1_000_000.0
        );
    } else {
        eprintln!("[nv-rs {} {category}::{op}] {detail}", msg_level.as_str());
    }
    if let Ok(mut ring) = st.ring.lock() {
        if ring.len() >= RING_CAPACITY {
            ring.pop_front();
        }
        ring.push_back(DiagEvent {
            seq,
            timestamp_ms: ts,
            level: msg_level,
            category: category.to_owned(),
            op: op.to_owned(),
            detail,
            elapsed_ns,
            ok,
        });
    }
}

/// Records a dynamic library load attempt from [`crate::open_first_with`].
pub(crate) fn record_lib_load(
    lib: &str,
    loaded_path: Option<&Path>,
    error: Option<&str>,
    elapsed_ns: u64,
) {
    let st = state();
    match (loaded_path, error) {
        (Some(path), None) => {
            st.lib_loads.fetch_add(1, Ordering::Relaxed);
            record_event(
                Level::Info,
                "loader",
                "open",
                format!("{lib} -> {}", path.display()),
                elapsed_ns,
                true,
            );
        }
        (_, Some(err)) => {
            st.lib_load_failures.fetch_add(1, Ordering::Relaxed);
            record_event(
                Level::Error,
                "loader",
                "open",
                format!("{lib}: {err}"),
                elapsed_ns,
                false,
            );
        }
        _ => {}
    }
}

/// Records a GPU buffer allocation in bytes.
pub fn record_gpu_alloc(bytes: usize) {
    let st = state();
    let b = bytes as u64;
    st.gpu_alloc_count.fetch_add(1, Ordering::Relaxed);
    st.gpu_alloc_bytes.fetch_add(b, Ordering::Relaxed);
    let live = st.gpu_live_bytes.fetch_add(b, Ordering::Relaxed) + b;
    let mut peak = st.gpu_peak_bytes.load(Ordering::Relaxed);
    while live > peak {
        match st.gpu_peak_bytes.compare_exchange_weak(
            peak,
            live,
            Ordering::Relaxed,
            Ordering::Relaxed,
        ) {
            Ok(_) => break,
            Err(actual) => peak = actual,
        }
    }
    record_event(
        Level::Trace,
        "mem",
        "alloc",
        format!("{bytes} B (live={live} B)"),
        0,
        true,
    );
}

/// Records a GPU buffer deallocation in bytes.
pub fn record_gpu_free(bytes: usize) {
    let st = state();
    let b = bytes as u64;
    st.gpu_free_count.fetch_add(1, Ordering::Relaxed);
    // Saturating subtract guards against mismatched external-free accounting.
    let mut cur = st.gpu_live_bytes.load(Ordering::Relaxed);
    let live = loop {
        let next = cur.saturating_sub(b);
        match st.gpu_live_bytes.compare_exchange_weak(
            cur,
            next,
            Ordering::Relaxed,
            Ordering::Relaxed,
        ) {
            Ok(_) => break next,
            Err(actual) => cur = actual,
        }
    };
    record_event(
        Level::Trace,
        "mem",
        "free",
        format!("{bytes} B (live={live} B)"),
        0,
        true,
    );
}

/// Records a subsystem error without requiring an `elapsed_ns` measurement.
pub fn record_error(category: &str, op: &str, msg: &str) {
    record_event(Level::Error, category, op, msg, 0, false);
}

/// Returns a snapshot of cumulative runtime counters.
pub fn stats() -> DiagStats {
    let st = state();
    let last_error = st.last_error.lock().ok().and_then(|g| g.clone());
    DiagStats {
        lib_loads: st.lib_loads.load(Ordering::Relaxed),
        lib_load_failures: st.lib_load_failures.load(Ordering::Relaxed),
        gpu_alloc_count: st.gpu_alloc_count.load(Ordering::Relaxed),
        gpu_free_count: st.gpu_free_count.load(Ordering::Relaxed),
        gpu_alloc_bytes: st.gpu_alloc_bytes.load(Ordering::Relaxed),
        gpu_live_bytes: st.gpu_live_bytes.load(Ordering::Relaxed),
        gpu_peak_bytes: st.gpu_peak_bytes.load(Ordering::Relaxed),
        errors: st.errors.load(Ordering::Relaxed),
        last_error,
    }
}

/// Returns a snapshot of the bounded event ring-buffer (oldest first).
pub fn events() -> Vec<DiagEvent> {
    state()
        .ring
        .lock()
        .map(|r| r.iter().cloned().collect())
        .unwrap_or_default()
}

/// Clears the event ring-buffer (counters remain intact).
pub fn clear_events() {
    if let Ok(mut r) = state().ring.lock() {
        r.clear();
    }
}

/// Probes candidate locations for `spec` without loading the library into the process.
pub fn probe_spec(spec: &DllSpec) -> ProbeResult {
    let probe = SystemProbe;
    let candidates = candidates_with(&probe, spec);
    let first_present = if cfg!(windows) {
        candidates.iter().find(|p| probe.is_file(p)).cloned()
    } else {
        candidates.first().cloned()
    };
    ProbeResult {
        lib: spec.lib,
        candidates,
        first_present,
    }
}

/// Probes all standard NVIDIA libraries known to `nv-rs` without loading them.
pub fn probe_all() -> Vec<ProbeResult> {
    use crate::Group;
    const SPECS: &[DllSpec] = &[
        DllSpec {
            lib: "nvcuda",
            group: Group::Driver,
            windows: &["nvcuda.dll"],
            linux: &["libcuda.so.1", "libcuda.so"],
        },
        DllSpec {
            lib: "nvml",
            group: Group::Driver,
            windows: &["nvml.dll"],
            linux: &["libnvidia-ml.so.1"],
        },
        DllSpec {
            lib: "nvoptix",
            group: Group::DriverStore,
            windows: &["nvoptix.dll"],
            linux: &["libnvoptix.so.1"],
        },
        DllSpec {
            lib: "nvrtc",
            group: Group::Toolkit,
            windows: &["nvrtc64_*"],
            linux: &["libnvrtc.so.13", "libnvrtc.so.12", "libnvrtc.so"],
        },
        DllSpec {
            lib: "cublas",
            group: Group::Toolkit,
            windows: &["cublas64_*"],
            linux: &["libcublas.so.13", "libcublas.so.12", "libcublas.so"],
        },
        DllSpec {
            lib: "cufft",
            group: Group::Toolkit,
            windows: &["cufft64_*"],
            linux: &["libcufft.so.12", "libcufft.so.11", "libcufft.so"],
        },
        DllSpec {
            lib: "curand",
            group: Group::Toolkit,
            windows: &["curand64_*"],
            linux: &["libcurand.so.10", "libcurand.so"],
        },
        DllSpec {
            lib: "cusolver",
            group: Group::Toolkit,
            windows: &["cusolver64_*"],
            linux: &["libcusolver.so.12", "libcusolver.so.11", "libcusolver.so"],
        },
        DllSpec {
            lib: "cusparse",
            group: Group::Toolkit,
            windows: &["cusparse64_*"],
            linux: &["libcusparse.so.12", "libcusparse.so"],
        },
        DllSpec {
            lib: "nvjpeg",
            group: Group::Toolkit,
            windows: &["nvjpeg64_*"],
            linux: &["libnvjpeg.so.13", "libnvjpeg.so.12", "libnvjpeg.so"],
        },
        DllSpec {
            lib: "cudnn",
            group: Group::Sdk {
                root_var: "CUDNN_ROOT",
                hint: None,
            },
            windows: &["cudnn64_9.dll", "cudnn64_*"],
            linux: &["libcudnn.so.9", "libcudnn.so"],
        },
        DllSpec {
            lib: "nvcomp",
            group: Group::Sdk {
                root_var: "NVCOMP_ROOT",
                hint: None,
            },
            windows: &["nvcomp64_*", "nvcomp.dll"],
            linux: &["libnvcomp.so.6", "libnvcomp.so"],
        },
        DllSpec {
            lib: "nvcuvid",
            group: Group::Driver,
            windows: &["nvcuvid.dll"],
            linux: &["libnvcuvid.so.1"],
        },
        DllSpec {
            lib: "nvenc",
            group: Group::Driver,
            windows: &["nvEncodeAPI64.dll"],
            linux: &["libnvidia-encode.so.1"],
        },
        DllSpec {
            lib: "nvofapi",
            group: Group::Driver,
            windows: &["nvofapi64.dll"],
            linux: &["libnvidia-opticalflow.so.1"],
        },
        #[cfg(target_os = "linux")]
        DllSpec {
            lib: "nccl",
            group: Group::Sdk {
                root_var: "NCCL_ROOT",
                hint: None,
            },
            windows: &[],
            linux: &["libnccl.so.2", "libnccl.so"],
        },
        #[cfg(target_os = "linux")]
        DllSpec {
            lib: "cufile",
            group: Group::Toolkit,
            windows: &[],
            linux: &["libcufile.so.0", "libcufile.so"],
        },
        #[cfg(target_os = "windows")]
        DllSpec {
            lib: "nvapi",
            group: Group::Driver,
            windows: &["nvapi64.dll"],
            linux: &[],
        },
    ];
    SPECS.iter().map(probe_spec).collect()
}
