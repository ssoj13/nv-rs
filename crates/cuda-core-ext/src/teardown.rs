//! Process-wide sink for failures of native teardown calls that run in `Drop`.
//!
//! **What.** [`record`] keeps a [`TeardownError`] (library, operation, real native status);
//! [`take`] drains them, for one library or for all, explicitly.
//! **Why.** `Drop` cannot return an error. The wrappers used to map a library status (for example
//! `CUBLAS_STATUS_INTERNAL_ERROR` from `cublasDestroy`) to `CUDA_ERROR_UNKNOWN` and park it on
//! the owning context with `Context::record_err`, where it surfaced as the error of the next,
//! unrelated call on that context (`bind_to_thread` reads and clears that slot) and the real code
//! was lost. Here the failure keeps its identity and never becomes the result of another call.
//! **How.** A bounded queue behind a mutex: at most [`CAPACITY`] entries; when full, the oldest
//! entry is dropped and counted per library, and [`take`] reports that count in
//! [`Drained::lost`]. [`record`] never panics (a poisoned lock is recovered: every update is a
//! single push/pop). CUDA *driver* failures in a teardown (`cuMemFree`, a failed
//! `bind_to_thread`) stay on `Context::record_err`, the convention `cuda-core`'s own `Drop` impls
//! use for the context they belong to; [`bind`] restores such an error on the context.
//! **Used by** the `Drop` impls of `cublas`, `cudnn`, `cufft`, `cufile`, `cupti`, `curand`,
//! `cusolver`, `cusparse`, `nccl`, `ngx`, `nvcomp`, `nvenc`, `nvjpeg`, `nvofa`, `nv-wgpu` and `optix`;
//! `ngx::take_teardown_errors` is [`take`] for [`Library::Ngx`].

use std::collections::VecDeque;
use std::fmt;
use std::sync::{Mutex, PoisonError};

use cuda_core::Context;

/// Most entries kept before the oldest is dropped (and counted in [`Drained::lost`]).
pub const CAPACITY: usize = 256;

/// The native library whose teardown call failed.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Library {
    /// cuBLAS.
    Cublas,
    /// cuBLASLt.
    CublasLt,
    /// cuDNN.
    Cudnn,
    /// cuFFT.
    Cufft,
    /// cuFile (GPUDirect Storage).
    Cufile,
    /// CUPTI.
    Cupti,
    /// cuRAND.
    Curand,
    /// cuSOLVER (dense, sparse and multi-GPU).
    Cusolver,
    /// cuSPARSE.
    Cusparse,
    /// NCCL.
    Nccl,
    /// NGX (DLSS).
    Ngx,
    /// nvCOMP.
    Nvcomp,
    /// NVENC.
    Nvenc,
    /// nvJPEG.
    Nvjpeg,
    /// NVIDIA Optical Flow.
    Nvofa,
    /// OptiX.
    Optix,
    /// `nv-wgpu` (bounded waits for `wgpu` submissions in `Drop`; the status is an
    /// `nv_wgpu` wait code, not a native driver status).
    NvWgpu,
}

impl Library {
    const ALL: [Library; 17] = [
        Library::Cublas,
        Library::CublasLt,
        Library::Cudnn,
        Library::Cufft,
        Library::Cufile,
        Library::Cupti,
        Library::Curand,
        Library::Cusolver,
        Library::Cusparse,
        Library::Nccl,
        Library::Ngx,
        Library::Nvcomp,
        Library::Nvenc,
        Library::Nvjpeg,
        Library::Nvofa,
        Library::Optix,
        Library::NvWgpu,
    ];

    fn index(self) -> usize {
        self as usize
    }

    /// Short lowercase name (`"cublas"`).
    pub fn name(self) -> &'static str {
        match self {
            Library::Cublas => "cublas",
            Library::CublasLt => "cublasLt",
            Library::Cudnn => "cudnn",
            Library::Cufft => "cufft",
            Library::Cufile => "cufile",
            Library::Cupti => "cupti",
            Library::Curand => "curand",
            Library::Cusolver => "cusolver",
            Library::Cusparse => "cusparse",
            Library::Nccl => "nccl",
            Library::Ngx => "ngx",
            Library::Nvcomp => "nvcomp",
            Library::Nvenc => "nvenc",
            Library::Nvjpeg => "nvjpeg",
            Library::Nvofa => "nvofa",
            Library::Optix => "optix",
            Library::NvWgpu => "nv-wgpu",
        }
    }
}

/// One failed native teardown call.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TeardownError {
    /// The library that reported the failure.
    pub library: Library,
    /// The native entry point that failed (`"cublasDestroy_v2"`).
    pub op: &'static str,
    /// The status the entry point returned, unchanged (widened to `i64` so every library's status
    /// type fits, signed or not).
    pub status: i64,
}

impl fmt::Display for TeardownError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}: {} failed during teardown with status {}",
            self.library.name(),
            self.op,
            self.status
        )
    }
}

impl std::error::Error for TeardownError {}

/// What [`take`] drained.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Drained {
    /// The kept failures, oldest first.
    pub errors: Vec<TeardownError>,
    /// Failures of the drained libraries that were dropped because the sink was full.
    pub lost: u64,
}

impl Drained {
    /// Whether nothing failed (nothing kept, nothing lost).
    pub fn is_empty(&self) -> bool {
        self.errors.is_empty() && self.lost == 0
    }
}

struct Sink {
    errors: VecDeque<TeardownError>,
    lost: [u64; Library::ALL.len()],
}

static SINK: Mutex<Sink> = Mutex::new(Sink {
    errors: VecDeque::new(),
    lost: [0; Library::ALL.len()],
});

fn sink() -> std::sync::MutexGuard<'static, Sink> {
    // Every update is a single push/pop/counter step, so a panic in another holder cannot leave
    // the queue inconsistent; recover the guard instead of panicking inside `Drop`.
    SINK.lock().unwrap_or_else(PoisonError::into_inner)
}

/// Keeps a failed teardown call. Never panics; callable from `Drop`.
pub fn record(library: Library, op: &'static str, status: i64) {
    let mut sink = sink();
    if sink.errors.len() >= CAPACITY
        && let Some(oldest) = sink.errors.pop_front()
    {
        sink.lost[oldest.library.index()] = sink.lost[oldest.library.index()].saturating_add(1);
    }
    sink.errors.push_back(TeardownError {
        library,
        op,
        status,
    });
}

/// Keeps `status` when it differs from `success` (the library's success code).
pub fn check(library: Library, op: &'static str, status: i64, success: i64) {
    if status != success {
        record(library, op, status);
    }
}

/// Drains the failures of `library`, or of every library when `None`, oldest first.
pub fn take(library: Option<Library>) -> Drained {
    let mut sink = sink();
    let mut drained = Drained::default();
    match library {
        None => {
            drained.errors = sink.errors.drain(..).collect();
            drained.lost = sink.lost.iter().fold(0u64, |a, b| a.saturating_add(*b));
            sink.lost = [0; Library::ALL.len()];
        }
        Some(lib) => {
            let (taken, kept): (Vec<_>, Vec<_>) =
                sink.errors.drain(..).partition(|e| e.library == lib);
            sink.errors = kept.into();
            drained.errors = taken;
            drained.lost = std::mem::take(&mut sink.lost[lib.index()]);
        }
    }
    drained
}

/// Makes `ctx` current for a teardown call. On failure the driver error is put back on `ctx`
/// (`Context::record_err`: `bind_to_thread` reads and clears a pending error first, so it would
/// otherwise be lost) and `false` is returned: the caller must then skip the native call.
pub fn bind(ctx: &Context) -> bool {
    match ctx.bind_to_thread() {
        Ok(()) => true,
        Err(e) => {
            ctx.record_err(Err::<(), _>(e));
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // The sink is process-wide; one test owns it so parallel tests cannot interleave.
    #[test]
    fn records_drains_by_library_and_counts_overflow() {
        let _ = take(None);
        check(Library::Cublas, "cublasDestroy_v2", 0, 0);
        assert!(take(None).is_empty(), "success is not recorded");

        record(Library::Cublas, "cublasDestroy_v2", 14);
        record(Library::Ngx, "NVSDK_NGX_VULKAN_Shutdown1", -1_160_773_631);
        let ngx = take(Some(Library::Ngx));
        assert_eq!(ngx.errors.len(), 1);
        assert_eq!(ngx.errors[0].status, -1_160_773_631);
        assert_eq!(ngx.lost, 0);
        let rest = take(None);
        assert_eq!(
            rest.errors,
            vec![TeardownError {
                library: Library::Cublas,
                op: "cublasDestroy_v2",
                status: 14
            }]
        );
        assert!(rest.errors[0].to_string().contains("cublasDestroy_v2"));

        for i in 0..CAPACITY + 3 {
            record(Library::Cufft, "cufftDestroy", i as i64);
        }
        record(Library::Nvenc, "nvEncDestroyEncoder", 8);
        let cufft = take(Some(Library::Cufft));
        assert_eq!(cufft.lost, 4);
        assert_eq!(cufft.errors.len(), CAPACITY - 1);
        assert_eq!(cufft.errors[0].status, 4, "the oldest entries were dropped");
        let nvenc = take(Some(Library::Nvenc));
        assert_eq!(nvenc.errors.len(), 1);
        assert!(take(None).is_empty());
    }
}
