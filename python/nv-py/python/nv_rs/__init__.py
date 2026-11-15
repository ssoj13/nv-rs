"""nv_rs — Python API for the nv-rs NVIDIA Rust stack."""

from __future__ import annotations

from . import _nv_rs
from ._nv_rs import (
    __version__,
    core,
    cublas,
    cuda_ext,
    cudnn,
    cufft,
    cufile,
    cupti,
    curand,
    cusolver,
    cusparse,
    diag,
    errors,
    nccl,
    ngx,
    npp,
    nvapi,
    nvcomp,
    nvdec,
    nvenc,
    nvjpeg,
    nvml,
    nvofa,
    nvrtc,
    optix,
    wgpu,
)


def trace(level: str = "trace", print_on_exit: bool = True) -> diag.TraceGuard:
    """Context manager enabling `nv_rs.diag` tracing and reporting GPU live-byte deltas."""
    return diag.TraceGuard(level=level, print_on_exit=print_on_exit)


def format_doctor() -> str:
    """Return a human-readable multi-line diagnostic summary of the host environment."""
    info = diag.doctor()
    lines = [
        f"nv_rs {info['version']} ({info['os']}/{info['arch']}, diag={info['diag_level']})",
        f"Python interop: {info['python_interop']}",
        "Libraries:",
    ]
    for name, entry in sorted(info["libraries"].items()):
        status = f"OK -> {entry['path']}" if entry["present"] else "MISSING"
        lines.append(f"  - {name:10s}: {status}")
    st = info["stats"]
    lines.append(
        f"Stats: loads={st['lib_loads']} load_errs={st['lib_load_failures']} "
        f"gpu_live={st['gpu_live_bytes']} B gpu_peak={st['gpu_peak_bytes']} B errors={st['errors']}"
    )
    return "\n".join(lines)


diag.trace = trace  # type: ignore[attr-defined]
diag.format_doctor = format_doctor  # type: ignore[attr-defined]

__all__ = [
    "__version__",
    "core",
    "cublas",
    "cuda_ext",
    "cudnn",
    "cufft",
    "cufile",
    "cupti",
    "curand",
    "cusolver",
    "cusparse",
    "diag",
    "errors",
    "nccl",
    "ngx",
    "npp",
    "nvapi",
    "nvcomp",
    "nvdec",
    "nvenc",
    "nvjpeg",
    "nvml",
    "nvofa",
    "nvrtc",
    "optix",
    "wgpu",
    "trace",
    "format_doctor",
]
