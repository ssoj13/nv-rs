//! OptiX SDK include-directory discovery shared by `build.rs` and the runtime NVRTC path.
//!
//! **What.** [`include_dir`] returns the directory that holds `optix.h`: `OPTIX_ROOT/include` when
//! `OPTIX_ROOT` is set, otherwise the `OptiX SDK*` install under `%ProgramData%\NVIDIA Corporation`
//! with the highest `OPTIX_VERSION`.
//! **Why.** `raytracer.cu` needs the OptiX 8+/9 device API (`optixTraverse`, `optixReorder`), and
//! directory order (oldest SDK first on NTFS) must not decide which SDK compiles it. `build.rs`
//! (nvcc, embedded PTX) and the runtime fallback (NVRTC, shader hot reload) used to carry two
//! copies that drifted; one source guarantees both compile against the same headers.
//! **How.** Self-contained (`std` only, no output) so `build.rs` includes this file with
//! `#[path = "src/optix_sdk.rs"]`. An explicit `OPTIX_ROOT` that has no `optix.h` is an error and
//! never falls back to an installed SDK: the operator asked for that SDK.
//! **Used by** `build.rs` (`find_optix_include`) and `renderer.rs` (`raytracer_ptx_inputs`).

use std::path::{Path, PathBuf};

/// `OPTIX_VERSION` (e.g. `90000`) declared by `<include>/optix.h`, if readable.
fn header_version(include: &Path) -> Option<u32> {
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

/// Returns the OptiX include directory (the one containing `optix.h`).
///
/// # Errors
/// `OPTIX_ROOT` is set but has no `include/optix.h`, or no installed SDK was found.
pub fn include_dir() -> Result<PathBuf, String> {
    if let Some(root) = std::env::var_os("OPTIX_ROOT") {
        let inc = PathBuf::from(root).join("include");
        if inc.join("optix.h").is_file() {
            return Ok(inc);
        }
        return Err(format!(
            "OPTIX_ROOT is set but {} has no optix.h",
            inc.display()
        ));
    }
    let program_data = std::env::var_os("ProgramData")
        .ok_or_else(|| "OPTIX_ROOT is not set and %ProgramData% is unavailable".to_string())?;
    let base = PathBuf::from(program_data).join("NVIDIA Corporation");
    let entries = std::fs::read_dir(&base).map_err(|e| {
        format!(
            "OPTIX_ROOT is not set and {} cannot be read: {e}",
            base.display()
        )
    })?;
    highest_version(
        entries
            .flatten()
            .filter(|entry| entry.file_name().to_string_lossy().starts_with("OptiX SDK"))
            .map(|entry| entry.path().join("include")),
    )
    .ok_or_else(|| {
        format!(
            "OPTIX_ROOT is not set and no 'OptiX SDK*' install with optix.h under {}",
            base.display()
        )
    })
}

/// The include directory whose `optix.h` declares the highest `OPTIX_VERSION`; directories
/// without a readable version are skipped.
fn highest_version(includes: impl Iterator<Item = PathBuf>) -> Option<PathBuf> {
    includes
        .filter_map(|inc| header_version(&inc).map(|ver| (ver, inc)))
        .max_by_key(|(ver, _)| *ver)
        .map(|(_, inc)| inc)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Creates `<tmp>/<tag>-<pid>/include/optix.h` with `body` and returns the include dir.
    fn fake_sdk(tag: &str, body: &str) -> PathBuf {
        let inc = std::env::temp_dir()
            .join(format!("nv_demo_optix_sdk_{tag}_{}", std::process::id()))
            .join("include");
        std::fs::create_dir_all(&inc).expect("create fake SDK include dir");
        std::fs::write(inc.join("optix.h"), body).expect("write fake optix.h");
        inc
    }

    #[test]
    fn header_version_reads_the_exact_macro_only() {
        let inc = fake_sdk(
            "ver",
            "#define OPTIX_VERSION_MAJOR 9
#define OPTIX_VERSION 90100
",
        );
        assert_eq!(header_version(&inc), Some(90100));
        let none = fake_sdk("nover", "#define OPTIX_VERSION_MAJOR 9
");
        assert_eq!(header_version(&none), None);
    }

    #[test]
    fn highest_version_wins_regardless_of_order() {
        let old = fake_sdk("old", "#define OPTIX_VERSION 70700
");
        let new = fake_sdk("new", "#define OPTIX_VERSION 90000
");
        let missing = old.with_file_name("no_such_include");
        for order in [
            vec![old.clone(), new.clone(), missing.clone()],
            vec![missing, new.clone(), old],
        ] {
            assert_eq!(highest_version(order.into_iter()), Some(new.clone()));
        }
        assert_eq!(highest_version(std::iter::empty()), None);
    }
}
