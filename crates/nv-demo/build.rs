//! Build-time PTX precompilation for `nv-demo` shaders (`raytracer.cu` and `tonemap.cu`).
//!
//! Compiles both `.cu` sources to `$OUT_DIR/*.ptx` via `nvcc` when CUDA + OptiX headers are
//! available so `nv-demo` binaries start without runtime NVRTC or `<optix.h>` on target machines.

use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

/// `nvcc` from `CUDA_PATH` / `CUDA_HOME` / `CUDA_ROOT`, else from `PATH`; `None` when absent.
fn find_nvcc() -> Option<PathBuf> {
    let exe = if cfg!(windows) { "nvcc.exe" } else { "nvcc" };
    for var in ["CUDA_PATH", "CUDA_HOME", "CUDA_ROOT"] {
        if let Ok(root) = env::var(var) {
            let p = PathBuf::from(root).join("bin").join(exe);
            if p.is_file() {
                return Some(p);
            }
        }
    }
    let path = env::var_os("PATH")?;
    env::split_paths(&path)
        .map(|dir| dir.join(exe))
        .find(|p| p.is_file())
}

/// `OPTIX_VERSION` (e.g. `90000`) declared by `<include>/optix.h`, if readable.
fn optix_header_version(include: &Path) -> Option<u32> {
    let text = std::fs::read_to_string(include.join("optix.h")).ok()?;
    text.lines().find_map(|line| {
        let rest = line.trim().strip_prefix("#define")?.trim_start();
        let value = rest.strip_prefix("OPTIX_VERSION")?;
        // Reject longer macro names that merely start with `OPTIX_VERSION`.
        if !value.starts_with(char::is_whitespace) {
            return None;
        }
        value.trim().parse().ok()
    })
}

/// OptiX include directory: `OPTIX_ROOT` when it holds `optix.h` (explicit choice wins), else the
/// `ProgramData/NVIDIA Corporation/OptiX SDK*` install with the highest `OPTIX_VERSION`.
/// `raytracer.cu` needs the OptiX 8+/9 device API (`optixTraverse`, `optixReorder`), so directory
/// order (oldest SDK first on NTFS) must not decide.
fn find_optix_include() -> Option<PathBuf> {
    if let Ok(root) = env::var("OPTIX_ROOT") {
        let inc = PathBuf::from(root).join("include");
        if inc.join("optix.h").is_file() {
            return Some(inc);
        }
        println!(
            "cargo:warning=OPTIX_ROOT is set but {} has no optix.h; searching installed SDKs",
            inc.display()
        );
    }
    let base = PathBuf::from(env::var_os("ProgramData")?).join("NVIDIA Corporation");
    std::fs::read_dir(&base)
        .ok()?
        .flatten()
        .filter(|entry| entry.file_name().to_string_lossy().starts_with("OptiX SDK"))
        .map(|entry| entry.path().join("include"))
        .filter_map(|inc| optix_header_version(&inc).map(|ver| (ver, inc)))
        .max_by_key(|(ver, _)| *ver)
        .map(|(_, inc)| inc)
}

fn compile_ptx(
    nvcc: &Path,
    src: &Path,
    out: &Path,
    extra_incs: &[&Path],
) -> Result<(), String> {
    let mut cmd = Command::new(nvcc);
    cmd.arg("--ptx")
        .arg("-arch=compute_75")
        .arg("-std=c++17")
        .arg("--use_fast_math");
    for inc in extra_incs {
        cmd.arg("-I").arg(inc);
    }
    if cfg!(windows)
        && let Ok(compiler) = cc::Build::new().try_get_compiler()
    {
        cmd.arg("-allow-unsupported-compiler")
            .arg("-Xcompiler")
            .arg("/D_ALLOW_COMPILER_AND_STL_VERSION_MISMATCH");
        if let Some(dir) = compiler.path().parent() {
            cmd.arg("-ccbin").arg(dir);
        }
        for (k, v) in compiler.get_envs() {
            cmd.env(k, v);
        }
    }
    cmd.arg("-o").arg(out).arg(src);

    let output = cmd
        .output()
        .map_err(|e| format!("failed to run {}: {e}", nvcc.display()))?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).replace(['\r', '\n'], " ");
        let stdout = String::from_utf8_lossy(&output.stdout).replace(['\r', '\n'], " ");
        return Err(format!(
            "nvcc failed for {} ({}): {stdout} {stderr}",
            src.display(),
            output.status
        ));
    }
    Ok(())
}

fn find_cuda_nvtx_include() -> Option<PathBuf> {
    let mut candidates = Vec::new();
    for var in ["CUDA_PATH", "CUDA_HOME", "CUDA_ROOT"] {
        if let Ok(root) = env::var(var) {
            candidates.push(PathBuf::from(root).join("include"));
        }
    }
    #[cfg(not(windows))]
    candidates.push(PathBuf::from("/usr/local/cuda/include"));
    candidates
        .into_iter()
        .find(|p| p.join("nvtx3").join("nvToolsExt.h").is_file())
}

fn build_nvtx_shim() {
    println!("cargo::rustc-check-cfg=cfg(has_nvtx3)");
    println!("cargo::rerun-if-changed=nvtx_shim.c");
    println!("cargo::rerun-if-env-changed=CUDA_HOME");
    println!("cargo::rerun-if-env-changed=CUDA_ROOT");

    let Some(inc) = find_cuda_nvtx_include() else {
        return;
    };
    let mut build = cc::Build::new();
    build.file("nvtx_shim.c").include(&inc).warnings(false);
    match build.try_compile("nv_demo_nvtx") {
        Ok(()) => {
            if cfg!(target_os = "linux") {
                println!("cargo::rustc-link-lib=dl");
            }
            println!("cargo::rustc-cfg=has_nvtx3");
        }
        Err(err) => {
            println!("cargo::warning=NVTX3 shim compile failed ({err}); using no-op markers");
        }
    }
}

fn stage_dlss_runtime(out_dir: &Path) {
    println!("cargo:rerun-if-env-changed=NGX_SDK_ROOT");
    let ngx_root = env::var_os("NGX_SDK_ROOT")
        .map(PathBuf::from)
        .or_else(|| {
            let p = Path::new("../../.sdk/ngx");
            p.is_dir().then(|| p.to_path_buf())
        });
    let Some(ngx_root) = ngx_root else {
        return;
    };
    let Some(target_dir) = out_dir.ancestors().nth(3) else {
        return;
    };
    if cfg!(windows) {
        let rel_dir = ngx_root.join("lib/Windows_x86_64/rel");
        for dll in ["nvngx_dlss.dll", "nvngx_dlssd.dll"] {
            let src = rel_dir.join(dll);
            if src.is_file() {
                stage_if_changed(&src, &target_dir.join(dll));
            }
        }
    } else if cfg!(target_os = "linux") {
        let rel_dir = ngx_root.join("lib/Linux_x86_64/rel");
        if let Ok(entries) = std::fs::read_dir(&rel_dir) {
            for entry in entries.flatten() {
                let name = entry.file_name();
                let name_str = name.to_string_lossy();
                if name_str.starts_with("libnvidia-ngx-dlss.so")
                    || name_str.starts_with("libnvidia-ngx-dlssd.so")
                {
                    stage_if_changed(&entry.path(), &target_dir.join(&name));
                }
            }
        }
    }
}

/// Copies `src` over `dst` when `dst` is missing or differs in size or is older (an SDK update),
/// reporting a failed copy as a cargo warning.
fn stage_if_changed(src: &Path, dst: &Path) {
    let up_to_date = match (std::fs::metadata(src), std::fs::metadata(dst)) {
        (Ok(s), Ok(d)) => {
            s.len() == d.len()
                && matches!((s.modified(), d.modified()), (Ok(sm), Ok(dm)) if dm >= sm)
        }
        _ => false,
    };
    if up_to_date {
        return;
    }
    if let Err(err) = std::fs::copy(src, dst) {
        println!(
            "cargo:warning=failed to stage {} -> {}: {err}",
            src.display(),
            dst.display()
        );
    }
}

fn main() {
    println!("cargo:rustc-check-cfg=cfg(has_embedded_ptx)");
    println!("cargo:rerun-if-changed=shaders/raytracer.cu");
    println!("cargo:rerun-if-changed=shaders/tonemap.cu");
    println!("cargo:rerun-if-env-changed=OPTIX_ROOT");
    println!("cargo:rerun-if-env-changed=CUDA_PATH");

    let csg_inc = PathBuf::from(csg_device::csg_eval_cuh_dir());
    println!(
        "cargo:rerun-if-changed={}",
        csg_inc.join("csg_eval.cuh").display()
    );

    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR"));
    stage_dlss_runtime(&out_dir);
    build_nvtx_shim();
    let Some(nvcc) = find_nvcc() else {
        println!("cargo:warning=nvcc not found; nv-demo shaders will be compiled at runtime by NVRTC");
        return;
    };
    let Some(optix_inc) = find_optix_include() else {
        println!(
            "cargo:warning=OptiX SDK headers not found (set OPTIX_ROOT); nv-demo shaders will be compiled at runtime by NVRTC"
        );
        return;
    };

    let rt_out = out_dir.join("raytracer.ptx");
    let tm_out = out_dir.join("tonemap.ptx");
    if let Err(err) = compile_ptx(
        &nvcc,
        Path::new("shaders/raytracer.cu"),
        &rt_out,
        &[optix_inc.as_path(), csg_inc.as_path()],
    ) {
        println!("cargo:warning={err}");
        return;
    }
    if let Err(err) = compile_ptx(&nvcc, Path::new("shaders/tonemap.cu"), &tm_out, &[]) {
        println!("cargo:warning={err}");
        return;
    }

    println!("cargo:rustc-cfg=has_embedded_ptx");
}
