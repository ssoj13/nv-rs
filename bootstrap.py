#!/usr/bin/env python3
"""
bootstrap.py - Unified build/test/check/deps/xtask script for nv-rs.

Cross-platform, Python 3, standard library only.
"""

from __future__ import annotations

import argparse
import os
import platform
import shutil
import subprocess
import sys
import tarfile
import time
import urllib.error
import urllib.request
import zipfile
from dataclasses import dataclass
from pathlib import Path

if hasattr(sys.stdout, "reconfigure"):
    try:
        sys.stdout.reconfigure(line_buffering=True, encoding="utf-8", errors="replace")
    except Exception:
        pass
if hasattr(sys.stderr, "reconfigure"):
    try:
        sys.stderr.reconfigure(line_buffering=True, encoding="utf-8", errors="replace")
    except Exception:
        pass

ROOT_DIR = Path(__file__).parent.resolve()
DEFAULT_SDK_DIR = ROOT_DIR / ".sdk"
DEFAULT_TOOLCHAIN = "1.98.0"
IS_WINDOWS = platform.system() == "Windows"
IS_MACOS = platform.system() == "Darwin"
IS_LINUX = platform.system() == "Linux"

HELP_TEXT = """\
nv-rs bootstrap.py - Unified workspace build, test, check, xtask, and SDK helper

COMMANDS (short or long):
    b,  build [opts] [-- <cargo_args>]
        Build workspace (--release by default; -d/--debug for dev; -p/--package repeatable).
    t,  test  [opts] [-- <test_args>]
        Run workspace tests (--release by default; -d/--debug; -n/--nocapture;
        -p/--package repeatable; --gpu for #[ignore] GPU tests).
    c,  check [opts]
        Run `cargo fmt --all --check` and `cargo clippy --workspace --all-targets -- -D warnings`.
    s,  status
        Show Rust toolchain, git status, and detected NVIDIA SDK summary.
    cl, clean [opts]
        Run `cargo clean` (-p/--package optional; --sdk removes .sdk cache).
    x,  xtask <args...>
        Forward arguments to `cargo xtask <args...>` (e.g. `coverage check`, `coverage optix`).
    d,  deps <subcmd>
        Manage native SDK dependencies:
          d s,  deps status          Inspect detected SDKs and NV_SDK_DIRS.
          d dl, deps dl [opts]       Download/extract public SDKs into .sdk (or --dir).
    h,  help
        Print this help screen and exit 0.

GLOBAL OPTIONS:
    -d, --debug              Use debug profile instead of release (for build/test/check).
    -p, --package <crate>    Scope build/test/check/clean to specific crate(s) (repeatable).
    --toolchain <tc>         Override cargo +<toolchain> (default: rust-toolchain.toml).
    -h, --help               Show this help screen.

DEPS DL OPTIONS:
    --dir <path>             Target SDK root (default: <repo>/.sdk).
    --only <sdk>             Filter to specific SDK(s), comma-separated or repeatable.
    --dry-run                Show what would be downloaded/extracted or skipped.
    --force                  Re-download/re-extract even if already present.
    --allow-manual           Skip auth-gated SDKs with notice instead of failing.

EXAMPLES:
    python bootstrap.py                          Print this help screen
    python bootstrap.py b                        Build workspace (release)
    python bootstrap.py b -d -p nvenc -p nvdec   Build specific crates in debug mode
    python bootstrap.py t -p optix -- --lib      Run optix unit tests
    python bootstrap.py t --gpu -p nvrtc         Run GPU #[ignore] tests for nvrtc
    python bootstrap.py c                        Run fmt --check + clippy -D warnings
    python bootstrap.py x coverage check         Run cargo xtask coverage check
    python bootstrap.py d s                      Inspect detected SDKs & NV_SDK_DIRS
    python bootstrap.py d dl --dry-run           Preview public SDK downloads
    python bootstrap.py d dl --only optix,nvcomp --allow-manual
"""


class C:
    RST = "\033[0m"
    RED = "\033[91m"
    GRN = "\033[92m"
    YLW = "\033[93m"
    CYN = "\033[96m"
    WHT = "\033[97m"
    DIM = "\033[90m"

    @classmethod
    def init(cls) -> None:
        if IS_WINDOWS:
            os.system("")


def fmt_time(ms: float) -> str:
    if ms < 1000:
        return f"{ms:.0f}ms"
    if ms < 60_000:
        return f"{ms / 1000:.1f}s"
    return f"{int(ms // 60_000)}m{(ms % 60_000) / 1000:.0f}s"


def header(text: str) -> None:
    line = "=" * 64
    print(f"\n{C.CYN}{line}\n{text}\n{line}{C.RST}")


def step(text: str) -> None:
    print(f"  {C.WHT}{text}{C.RST}")


def ok(text: str) -> None:
    print(f"  {C.GRN}[OK] {text}{C.RST}")


def warn(text: str) -> None:
    print(f"  {C.YLW}[WARN] {text}{C.RST}")


def err(text: str) -> None:
    print(f"  {C.RED}[ERR] {text}{C.RST}")


def which(cmd: str) -> Path | None:
    found = shutil.which(cmd)
    if found:
        return Path(found)
    bin_name = f"{cmd}.exe" if IS_WINDOWS else cmd
    cargo_bin = Path(os.environ.get("CARGO_HOME") or (Path.home() / ".cargo")) / "bin" / bin_name
    if cargo_bin.is_file():
        os.environ["PATH"] = f"{cargo_bin.parent}{os.pathsep}{os.environ.get('PATH', '')}"
        return cargo_bin
    return None


def check_cargo() -> bool:
    if not which("cargo"):
        err("Rust/Cargo not found -- install from https://rustup.rs/")
        return False
    return True


def cargo_cmd(toolchain: str | None, *args: str) -> list[str]:
    cmd = ["cargo"]
    if toolchain:
        cmd.append(f"+{toolchain}")
    cmd.extend(args)
    return cmd


def run(
    args: list[str],
    cwd: Path | None = None,
    capture: bool = False,
    env: dict[str, str] | None = None,
) -> tuple[int, str, float]:
    t0 = time.perf_counter()
    merged = {**os.environ, **env} if env else None
    result = subprocess.run(
        args,
        cwd=cwd or ROOT_DIR,
        capture_output=capture,
        text=True,
        encoding="utf-8",
        errors="replace",
        env=merged,
    )
    elapsed_ms = (time.perf_counter() - t0) * 1000
    output = ((result.stdout or "") + (result.stderr or "")) if capture else ""
    return result.returncode, output, elapsed_ms


def split_passthrough(argv: list[str]) -> tuple[list[str], list[str]]:
    if "--" in argv:
        idx = argv.index("--")
        return argv[:idx], argv[idx + 1 :]
    return argv, []


def pkg_flags(packages: list[str]) -> list[str]:
    flags: list[str] = []
    for p in packages:
        flags.extend(["-p", p])
    return flags


# ==============================================================================
# SDK Specs & Detection (aligned with vcv-rs & @sdk-urls)
# ==============================================================================


@dataclass(frozen=True)
class SdkSpec:
    key: str
    display: str
    env_vars: tuple[str, ...]
    subdir: str
    marker_paths: tuple[str, ...]
    url: str | None
    archive_type: str  # "zip" | "tar.xz" | "tar.gz" | "git" | "manual"
    manual_url: str
    manual_note: str
    system_globs: tuple[str, ...] = ()
    platforms: tuple[str, ...] = ("Windows", "Linux")
    strip_top: bool = True


def _platform_url(win: str | None, linux: str | None) -> tuple[str | None, str]:
    if IS_WINDOWS:
        return win, ("zip" if win and win.endswith(".zip") else "manual")
    if IS_LINUX:
        if linux and linux.endswith(".tar.xz"):
            return linux, "tar.xz"
        if linux and linux.endswith(".zip"):
            return linux, "zip"
        return linux, ("git" if linux and linux.endswith(".git") else "manual")
    return None, "manual"


_NVCOMP_WIN_URL = (
    "https://developer.download.nvidia.com/compute/nvcomp/redist/nvcomp/"
    "windows-x86_64/nvcomp-windows-x86_64-6.0.0.12_cuda13-archive.zip"
)
_NVCOMP_LIN_URL = (
    "https://developer.download.nvidia.com/compute/nvcomp/redist/nvcomp/"
    "linux-x86_64/nvcomp-linux-x86_64-6.0.0.12_cuda13-archive.tar.xz"
)
_CUDNN_WIN_URL = (
    "https://developer.download.nvidia.com/compute/cudnn/redist/cudnn/"
    "windows-x86_64/cudnn-windows-x86_64-9.28.1.4_cuda13-archive.zip"
)
_CUDNN_LIN_URL = (
    "https://developer.download.nvidia.com/compute/cudnn/redist/cudnn/"
    "linux-x86_64/cudnn-linux-x86_64-9.28.1.4_cuda13-archive.tar.xz"
)

_NVCOMP_URL, _NVCOMP_ARC = _platform_url(_NVCOMP_WIN_URL, _NVCOMP_LIN_URL)
_CUDNN_URL, _CUDNN_ARC = _platform_url(_CUDNN_WIN_URL, _CUDNN_LIN_URL)
_VULKAN_URL, _VULKAN_ARC = _platform_url(
    None,
    "https://sdk.lunarg.com/sdk/download/latest/linux/vulkan-sdk.tar.xz",
)

SDK_SPECS: tuple[SdkSpec, ...] = (
    SdkSpec(
        key="cuda",
        display="CUDA Toolkit",
        env_vars=("CUDA_PATH", "CUDA_HOME", "CUDA_ROOT", "CUDA_TOOLKIT_ROOT_DIR"),
        subdir="cuda",
        marker_paths=("include/cuda.h", "include/crt/host_config.h"),
        url=None,
        archive_type="manual",
        manual_url="https://developer.download.nvidia.com/compute/cuda/13.3.1/network_installers/cuda_13.3.1_windows_network.exe",
        manual_note="System installer (or redistrib_13.5.0.json)",
        system_globs=(
            r"C:\Program Files\NVIDIA GPU Computing Toolkit\CUDA\v*",
            "/usr/local/cuda*",
        ),
    ),
    SdkSpec(
        key="optix",
        display="OptiX SDK",
        env_vars=("OPTIX_ROOT", "OptiX_INSTALL_DIR", "OPTIX_PATH"),
        subdir="optix",
        marker_paths=("include/optix.h", "include/optix_function_table.h"),
        url="https://github.com/NVIDIA/optix-dev/archive/refs/heads/master.zip",
        archive_type="zip",
        manual_url="https://developer.nvidia.com/rtx/ray-tracing/optix",
        manual_note="Public headers via NVIDIA/optix-dev",
        system_globs=(
            r"C:\ProgramData\NVIDIA Corporation\OptiX SDK *",
            "/opt/*optix*",
        ),
    ),
    SdkSpec(
        key="vulkan",
        display="Vulkan SDK",
        env_vars=("VULKAN_SDK",),
        subdir="vulkan",
        marker_paths=("Include/vulkan/vulkan.h", "include/vulkan/vulkan.h"),
        url=_VULKAN_URL,
        archive_type=_VULKAN_ARC,
        manual_url="https://sdk.lunarg.com/sdk/download/latest/windows/vulkan-sdk.exe",
        manual_note="Windows requires running LunarG vulkan-sdk.exe installer",
        system_globs=(
            r"C:\VulkanSDK\*",
            "/usr",
        ),
    ),
    SdkSpec(
        key="opticalflow",
        display="Optical Flow",
        env_vars=("OPTICAL_FLOW_SDK_ROOT",),
        subdir="opticalflow",
        marker_paths=("NvOFInterface/nvOpticalFlowCommon.h", "include/nvOpticalFlowCommon.h"),
        url=None,
        archive_type="manual",
        manual_url="https://developer.nvidia.com/optical-flow-sdk",
        manual_note="Auth-gated NVIDIA Developer portal; extract under NV_SDK_DIRS",
    ),
    SdkSpec(
        key="videocodec",
        display="Video Codec",
        env_vars=("VIDEO_CODEC_SDK_ROOT", "NVIDIA_VIDEO_CODEC_SDK"),
        subdir="videocodec",
        marker_paths=("Interface/nvEncodeAPI.h", "include/nvEncodeAPI.h"),
        url=None,
        archive_type="manual",
        manual_url="https://developer.nvidia.com/nvidia-video-codec-sdk/download",
        manual_note="Auth-gated NVIDIA Developer portal; extract under NV_SDK_DIRS",
    ),
    SdkSpec(
        key="nvcomp",
        display="nvCOMP",
        env_vars=("NVCOMP_ROOT", "NVCOMP_PATH"),
        subdir="nvcomp",
        marker_paths=("include/nvcomp.h", "include/nvcomp/version.h"),
        url=_NVCOMP_URL,
        archive_type=_NVCOMP_ARC,
        manual_url="https://developer.download.nvidia.com/compute/nvcomp/redist/nvcomp/redistrib_6.0.0.json",
        manual_note="NVIDIA nvCOMP redist archive",
    ),
    SdkSpec(
        key="nvapi",
        display="NVAPI",
        env_vars=("NVAPI_ROOT", "NVAPI_PATH"),
        subdir="nvapi",
        marker_paths=("nvapi_interface.h", "nvapi_lite_common.h"),
        url="https://github.com/NVIDIA/nvapi/archive/refs/heads/main.zip",
        archive_type="zip",
        manual_url="https://github.com/NVIDIA/nvapi",
        manual_note="Public NVIDIA/nvapi repository",
        platforms=("Windows",),
    ),
    SdkSpec(
        key="cudnn",
        display="cuDNN",
        env_vars=("CUDNN_ROOT", "CUDNN_PATH", "CUDNN_HOME"),
        subdir="cudnn",
        marker_paths=("include/cudnn_version.h", "include/*/cudnn_version.h"),
        url=_CUDNN_URL,
        archive_type=_CUDNN_ARC,
        manual_url="https://developer.download.nvidia.com/compute/cudnn/redist/cudnn/redistrib_9.28.1.json",
        manual_note="NVIDIA cuDNN redist archive",
    ),
    SdkSpec(
        key="ngx",
        display="NGX / DLSS",
        env_vars=("NGX_SDK_ROOT", "DLSS_ROOT"),
        subdir="ngx",
        marker_paths=("include/nvsdk_ngx.h", "include/nvsdk_ngx_defs.h", "include/nvsdk_ngx_vk.h"),
        url="https://github.com/NVIDIA/DLSS/archive/refs/heads/main.zip",
        archive_type="zip",
        manual_url="https://github.com/NVIDIA/DLSS",
        manual_note="Public NVIDIA/DLSS repository",
    ),
    SdkSpec(
        key="nccl",
        display="NCCL",
        env_vars=("NCCL_ROOT", "NCCL_PATH"),
        subdir="nccl",
        marker_paths=("include/nccl.h", "src/include/nccl.h"),
        url="https://github.com/NVIDIA/nccl/archive/refs/heads/master.zip" if IS_LINUX else None,
        archive_type="zip" if IS_LINUX else "manual",
        manual_url="https://github.com/NVIDIA/nccl",
        manual_note="Linux-only; source archive or libnccl-dev package",
        platforms=("Linux",),
    ),
)


def parse_nv_sdk_dirs() -> list[Path]:
    raw = os.environ.get("NV_SDK_DIRS", "").strip()
    if not raw:
        return []
    parts = [p.strip() for p in raw.split(os.pathsep) if p.strip()]
    return [Path(p) for p in parts]


def _has_marker(base: Path, markers: tuple[str, ...]) -> bool:
    if not base.exists():
        return False
    if not markers:
        return True
    for m in markers:
        if "*" in m:
            if any(base.glob(m)):
                return True
        elif (base / m).exists():
            return True
    return False


def _glob_dirs(pattern: str) -> list[Path]:
    p = Path(pattern)
    if "*" not in pattern:
        return [p] if p.exists() else []
    parent = p.parent
    if not parent.exists():
        return []
    return sorted(parent.glob(p.name), reverse=True)


def _scan_dir_depth(base: Path, markers: tuple[str, ...], max_depth: int) -> Path | None:
    if _has_marker(base, markers):
        return base
    if max_depth <= 0 or not base.is_dir():
        return None
    try:
        children = sorted(p for p in base.iterdir() if p.is_dir())
    except OSError:
        return None
    for child in children:
        found = _scan_dir_depth(child, markers, max_depth - 1)
        if found is not None:
            return found
    return None


def detect_sdk(spec: SdkSpec, sdk_root: Path = DEFAULT_SDK_DIR) -> tuple[Path | None, str]:
    for var in spec.env_vars:
        val = os.environ.get(var, "").strip()
        if val:
            p = Path(val)
            if _has_marker(p, spec.marker_paths):
                return p, f"env:{var}"

    local_candidate = sdk_root / spec.subdir
    if _has_marker(local_candidate, spec.marker_paths):
        return local_candidate, "local:.sdk"

    for d in parse_nv_sdk_dirs():
        sub = d / spec.subdir
        if _has_marker(sub, spec.marker_paths):
            return sub, "env:NV_SDK_DIRS"
        found = _scan_dir_depth(d, spec.marker_paths, max_depth=3)
        if found is not None:
            return found, "env:NV_SDK_DIRS"

    for pat in spec.system_globs:
        for cand in _glob_dirs(pat):
            if _has_marker(cand, spec.marker_paths):
                return cand, "system"

    return None, "missing"


def cmd_deps_status(sdk_root: Path = DEFAULT_SDK_DIR) -> int:
    header("NV-RS SDK DEPENDENCIES STATUS")
    nv_dirs = parse_nv_sdk_dirs()
    step(f"Default SDK dir : {sdk_root} ({'exists' if sdk_root.exists() else 'not created'})")
    if nv_dirs:
        step(f"NV_SDK_DIRS     : {os.pathsep.join(str(p) for p in nv_dirs)}")
    else:
        step("NV_SDK_DIRS     : <unset>")

    print()
    found_count = 0
    host_os = platform.system()
    for spec in SDK_SPECS:
        path, source = detect_sdk(spec, sdk_root)
        if path is not None:
            found_count += 1
            ok(f"{spec.key:<13} ({spec.display:<14}) [{source}] -> {path}")
        elif host_os not in spec.platforms:
            step(f"{C.DIM}[N/A]  {spec.key:<13} ({spec.display:<14}) [other-OS: {'/'.join(spec.platforms)}]{C.RST}")
        else:
            kind = "manual" if spec.url is None else f"auto:{spec.archive_type}"
            warn(f"{spec.key:<13} ({spec.display:<14}) [missing, {kind}]")
    print()
    step(f"Detected {found_count}/{len(SDK_SPECS)} SDKs.")
    return 0


def _download_file(url: str, dst: Path) -> None:
    dst.parent.mkdir(parents=True, exist_ok=True)
    req = urllib.request.Request(url, headers={"User-Agent": "nv-rs-bootstrap/1.0"})
    with urllib.request.urlopen(req, timeout=60) as resp:
        total = int(resp.headers.get("Content-Length") or 0)
        downloaded = 0
        last_pct = -1
        with dst.open("wb") as f:
            while True:
                chunk = resp.read(65536)
                if not chunk:
                    break
                f.write(chunk)
                downloaded += len(chunk)
                if total > 0:
                    pct = int(downloaded * 100 / total)
                    if pct != last_pct and pct % 10 == 0:
                        last_pct = pct
                        print(f"    downloading... {pct}% ({downloaded // (1024 * 1024)} MiB)")


def _safe_relpath(raw_name: str, strip_top: bool) -> Path | None:
    norm = raw_name.replace("\\", "/").lstrip("/")
    if not norm or ":" in norm.split("/", 1)[0]:
        return None
    p = Path(norm)
    if p.drive or p.is_absolute():
        return None
    parts = [part for part in p.parts if part not in ("", ".")]
    if strip_top:
        parts = parts[1:]
    if not parts or any(part == ".." for part in parts):
        return None
    return Path(*parts)


def _extract_archive(archive: Path, dst: Path, archive_type: str, strip_top: bool = True) -> None:
    dst.mkdir(parents=True, exist_ok=True)
    dst_resolved = dst.resolve()
    if archive_type == "zip":
        with zipfile.ZipFile(archive, "r") as zf:
            for info in zf.infolist():
                if info.is_dir():
                    continue
                rel = _safe_relpath(info.filename, strip_top)
                if rel is None:
                    continue
                out_path = (dst_resolved / rel).resolve()
                if not out_path.is_relative_to(dst_resolved):
                    raise ValueError(f"Unsafe archive entry path: {info.filename}")
                out_path.parent.mkdir(parents=True, exist_ok=True)
                with zf.open(info, "r") as src, out_path.open("wb") as out:
                    shutil.copyfileobj(src, out)
    elif archive_type in ("tar.xz", "tar.gz"):
        mode = "r:xz" if archive_type == "tar.xz" else "r:gz"
        with tarfile.open(archive, mode) as tf:
            for member in tf.getmembers():
                if not member.isfile():
                    continue
                rel = _safe_relpath(member.name, strip_top)
                if rel is None:
                    continue
                out_path = (dst_resolved / rel).resolve()
                if not out_path.is_relative_to(dst_resolved):
                    raise ValueError(f"Unsafe archive entry path: {member.name}")
                out_path.parent.mkdir(parents=True, exist_ok=True)
                src = tf.extractfile(member)
                if src is not None:
                    with src, out_path.open("wb") as out:
                        shutil.copyfileobj(src, out)
    else:
        raise ValueError(f"Unsupported archive type: {archive_type}")


def cmd_deps_dl(args: argparse.Namespace) -> int:
    sdk_root = Path(args.dir).resolve()
    header(f"NV-RS SDK DOWNLOAD ({'DRY-RUN' if args.dry_run else str(sdk_root)})")

    only_set: set[str] = set()
    for item in args.only or []:
        for part in item.split(","):
            p = part.strip().lower()
            if p:
                only_set.add(p)

    selected = [s for s in SDK_SPECS if not only_set or s.key.lower() in only_set]
    if only_set and not selected:
        err(f"No matching SDKs for --only={','.join(sorted(only_set))}")
        return 1

    host_os = platform.system()
    manual_failures = 0
    for spec in selected:
        if host_os not in spec.platforms:
            step(f"[SKIP] {spec.key}: not applicable on {host_os}")
            continue

        existing, source = detect_sdk(spec, sdk_root)
        if existing is not None and not args.force:
            ok(f"{spec.key}: already present ({source} -> {existing}); skipping (use --force to override)")
            continue

        if spec.url is None:
            msg = f"{spec.key} ({spec.display}) requires manual download: {spec.manual_url}"
            if spec.manual_note:
                msg += f" ({spec.manual_note})"
            if args.allow_manual or args.dry_run:
                warn(f"[MANUAL] {msg}")
            else:
                err(msg)
                manual_failures += 1
            continue

        target_dir = sdk_root / spec.subdir
        if args.dry_run:
            step(f"[DRY-RUN] {spec.key}: {spec.url} ({spec.archive_type}) -> {target_dir}")
            continue

        step(f"Fetching {spec.key} from {spec.url} ...")
        if spec.archive_type == "git":
            if target_dir.exists() and args.force:
                shutil.rmtree(target_dir)
            code, _, ms = run(["git", "clone", "--depth", "1", spec.url, str(target_dir)])
            if code != 0:
                err(f"git clone failed for {spec.key} ({fmt_time(ms)})")
                return code
            ok(f"{spec.key} cloned into {target_dir} ({fmt_time(ms)})")
        else:
            ext = f".{spec.archive_type}"
            archive_path = sdk_root / "_downloads" / f"{spec.key}{ext}"
            try:
                t0 = time.perf_counter()
                _download_file(spec.url, archive_path)
                if target_dir.exists() and args.force:
                    shutil.rmtree(target_dir)
                _extract_archive(archive_path, target_dir, spec.archive_type, strip_top=spec.strip_top)
                ms = (time.perf_counter() - t0) * 1000
                ok(f"{spec.key} extracted to {target_dir} ({fmt_time(ms)})")
            except (urllib.error.URLError, OSError, ValueError, zipfile.BadZipFile, tarfile.TarError) as exc:
                err(f"{spec.key} download/extract failed: {exc}")
                return 1

    if manual_failures > 0:
        err(f"{manual_failures} SDK(s) require manual download (pass --allow-manual to skip them).")
        return 1
    return 0


# ==============================================================================
# Build / Test / Check / Status / Clean / Xtask
# ==============================================================================


def cmd_build(args: argparse.Namespace, passthrough: list[str]) -> int:
    if not check_cargo():
        return 1
    header("BUILD (nv-rs)")
    profile = "debug" if args.debug else "release"
    step(f"Profile  : {profile}")
    if args.package:
        step(f"Packages : {', '.join(args.package)}")

    cmd = cargo_cmd(args.toolchain, "build")
    if not args.debug:
        cmd.append("--release")
    if args.package:
        cmd.extend(pkg_flags(args.package))
    else:
        cmd.append("--workspace")
    cmd.extend(passthrough)

    code, _, ms = run(cmd)
    (ok if code == 0 else err)(f"build {'OK' if code == 0 else 'FAILED'} ({fmt_time(ms)})")
    return code


def cmd_test(args: argparse.Namespace, passthrough: list[str]) -> int:
    if not check_cargo():
        return 1
    header("TEST (nv-rs)")
    profile = "debug" if args.debug else "release"
    step(f"Profile  : {profile}")
    if args.package:
        step(f"Packages : {', '.join(args.package)}")

    cmd = cargo_cmd(args.toolchain, "test")
    if not args.debug:
        cmd.append("--release")
    if args.package:
        cmd.extend(pkg_flags(args.package))
    else:
        cmd.append("--workspace")

    test_tail = list(passthrough)
    if args.nocapture and "--nocapture" not in test_tail:
        test_tail.append("--nocapture")
    if args.gpu:
        if "--ignored" not in test_tail:
            test_tail.append("--ignored")
        if not any(a.startswith("--test-threads") for a in test_tail):
            test_tail.append("--test-threads=1")
    if test_tail:
        cmd.append("--")
        cmd.extend(test_tail)

    code, _, ms = run(cmd)
    (ok if code == 0 else err)(f"test {'OK' if code == 0 else 'FAILED'} ({fmt_time(ms)})")
    return code


def cmd_check(args: argparse.Namespace) -> int:
    if not check_cargo():
        return 1
    header("CHECK (fmt + clippy -D warnings)")

    fmt_cmd = cargo_cmd(args.toolchain, "fmt", "--all", "--check")
    code, _, ms = run(fmt_cmd)
    (ok if code == 0 else err)(f"cargo fmt --check ({fmt_time(ms)})")
    if code != 0:
        return code

    clippy_cmd = cargo_cmd(args.toolchain, "clippy")
    if not args.debug:
        clippy_cmd.append("--release")
    if args.package:
        clippy_cmd.extend(pkg_flags(args.package))
    else:
        clippy_cmd.append("--workspace")
    clippy_cmd.extend(["--all-targets", "--", "-D", "warnings"])

    code, _, ms = run(clippy_cmd)
    (ok if code == 0 else err)(f"cargo clippy ({fmt_time(ms)})")
    return code


def cmd_status(args: argparse.Namespace) -> int:
    header("NV-RS WORKSPACE STATUS")
    if check_cargo():
        tc_cmd = (
            ["rustc", "--version"]
            if not args.toolchain
            else ["rustup", "run", args.toolchain, "rustc", "--version"]
        )
        _, rustc_out, _ = run(tc_cmd, capture=True)
        step(f"Toolchain : {rustc_out.strip() or 'unknown'}")
    tc_file = ROOT_DIR / "rust-toolchain.toml"
    if tc_file.is_file():
        step(f"Pinned    : {tc_file.name} present (default {DEFAULT_TOOLCHAIN})")
    if which("git"):
        rc, rev, _ = run(["git", "rev-parse", "--short", "HEAD"], capture=True)
        rc2, st, _ = run(["git", "status", "--porcelain"], capture=True)
        dirty = len([ln for ln in st.splitlines() if ln.strip()]) if rc2 == 0 else 0
        if rc == 0:
            step(f"Git HEAD  : {rev.strip()} ({dirty} dirty path(s))")
    return cmd_deps_status(DEFAULT_SDK_DIR)


def cmd_clean(args: argparse.Namespace) -> int:
    if not check_cargo():
        return 1
    header("CLEAN (nv-rs)")
    cmd = cargo_cmd(args.toolchain, "clean")
    if args.package:
        cmd.extend(pkg_flags(args.package))
    code, _, ms = run(cmd)
    (ok if code == 0 else err)(f"cargo clean ({fmt_time(ms)})")
    if code != 0:
        return code
    if args.sdk and DEFAULT_SDK_DIR.exists():
        shutil.rmtree(DEFAULT_SDK_DIR)
        ok(f"Removed {DEFAULT_SDK_DIR}")
    return 0


def cmd_xtask(rest: list[str], toolchain: str | None) -> int:
    if not check_cargo():
        return 1
    header(f"XTASK ({' '.join(rest) if rest else 'help'})")
    cmd = cargo_cmd(toolchain, "xtask", *rest)
    code, _, ms = run(cmd)
    (ok if code == 0 else err)(f"xtask ({fmt_time(ms)})")
    return code


# ==============================================================================
# CLI Dispatch
# ==============================================================================


def _add_common_flags(p: argparse.ArgumentParser) -> None:
    p.add_argument("-d", "--debug", action="store_true", help="Use debug profile")
    p.add_argument("-p", "--package", action="append", default=[], help="Target crate (repeatable)")
    p.add_argument("--toolchain", default=None, help="Cargo toolchain override (e.g. 1.98.0)")


def main() -> int:
    C.init()
    raw_argv = sys.argv[1:]
    if not raw_argv or raw_argv[0] in ("h", "help", "-h", "--help"):
        print(HELP_TEXT)
        return 0

    argv, passthrough = split_passthrough(raw_argv)
    if not argv:
        print(HELP_TEXT)
        return 0

    cmd = argv[0]
    rest = argv[1:]

    if cmd in ("x", "xtask"):
        return cmd_xtask(rest + (["--", *passthrough] if passthrough else []), None)

    if cmd in ("b", "build"):
        p = argparse.ArgumentParser(prog="bootstrap.py build")
        _add_common_flags(p)
        return cmd_build(p.parse_args(rest), passthrough)

    if cmd in ("t", "test"):
        p = argparse.ArgumentParser(prog="bootstrap.py test")
        _add_common_flags(p)
        p.add_argument("-n", "--nocapture", action="store_true", help="Pass --nocapture")
        p.add_argument("--gpu", action="store_true", help="Run #[ignore] GPU tests")
        return cmd_test(p.parse_args(rest), passthrough)

    if cmd in ("c", "check"):
        p = argparse.ArgumentParser(prog="bootstrap.py check")
        _add_common_flags(p)
        return cmd_check(p.parse_args(rest))

    if cmd in ("s", "status"):
        p = argparse.ArgumentParser(prog="bootstrap.py status")
        p.add_argument("--toolchain", default=None, help="Cargo toolchain override")
        return cmd_status(p.parse_args(rest))

    if cmd in ("cl", "clean"):
        p = argparse.ArgumentParser(prog="bootstrap.py clean")
        p.add_argument("-p", "--package", action="append", default=[], help="Target crate (repeatable)")
        p.add_argument("--toolchain", default=None, help="Cargo toolchain override")
        p.add_argument("--sdk", action="store_true", help="Also remove .sdk directory")
        return cmd_clean(p.parse_args(rest))

    if cmd in ("d", "deps"):
        if not rest or rest[0] in ("h", "help", "-h", "--help"):
            print(HELP_TEXT)
            return 0
        sub = rest[0]
        sub_rest = rest[1:]
        if sub in ("s", "status"):
            p = argparse.ArgumentParser(prog="bootstrap.py deps status")
            p.add_argument("--dir", default=str(DEFAULT_SDK_DIR), help="SDK root directory")
            ns = p.parse_args(sub_rest)
            return cmd_deps_status(Path(ns.dir).resolve())
        if sub in ("dl", "l", "download"):
            p = argparse.ArgumentParser(prog="bootstrap.py deps dl")
            p.add_argument("--dir", default=str(DEFAULT_SDK_DIR), help="Target SDK directory")
            p.add_argument("--only", action="append", default=[], help="Filter SDK(s)")
            p.add_argument("--dry-run", action="store_true", help="Preview without downloading")
            p.add_argument("--force", action="store_true", help="Re-download existing SDKs")
            p.add_argument("--allow-manual", action="store_true", help="Skip auth-gated SDKs")
            return cmd_deps_dl(p.parse_args(sub_rest))
        err(f"Unknown deps subcommand: {sub}")
        return 2

    err(f"Unknown command: {cmd}")
    print(HELP_TEXT)
    return 2


if __name__ == "__main__":
    raise SystemExit(main())
