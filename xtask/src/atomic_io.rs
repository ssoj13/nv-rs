//! Atomic file writes for every artifact xtask produces.
//!
//! **What.** [`write`] and [`copy`] replace a file in one step: the bytes go to a temp file in the
//! destination's own directory, which is flushed and `sync_all`ed, then renamed over the
//! destination. [`copy_to_temp`] / [`commit`] / [`discard_all`] expose the two halves so a caller
//! can stage a whole set of files first and rename only when every copy succeeded.
//!
//! **Why.** Generated artifacts are committed (`prebuilt/*.rs`, `COVERAGE.md`, inventory evidence,
//! `.pyi` stubs, docs) or shipped (staged DLLs). An in-place `fs::write` / `fs::copy` interrupted by
//! Ctrl-C, a crash or a full disk leaves a truncated file that later runs treat as valid. A rename
//! within one directory is atomic on NTFS and POSIX file systems, so readers see the old file or the
//! new one, never a prefix.
//!
//! **How.** The temp name is `.<file name>.<pid>.<seq>.tmp` next to the destination (same volume,
//! so the rename never degrades into a copy). Every error names the operation and the path. A temp
//! file left by a failed step is removed; a failure to remove it is reported, never dropped.
//!
//! **Used by** `xtask.rs` (`COVERAGE.md`, inventory evidence), `prebuilt.rs`, `py_stubs.rs`,
//! `stage_dlls.rs`.

use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

/// Writes `bytes` to `path` atomically (temp file, flush, `sync_all`, rename).
pub fn write(path: &Path, bytes: &[u8]) -> Result<(), String> {
    let tmp = temp_path(path)?;
    match write_synced(&tmp, bytes) {
        Ok(()) => commit(&tmp, path),
        Err(e) => Err(discard_all(&[tmp], e)),
    }
}

fn write_synced(tmp: &Path, bytes: &[u8]) -> Result<(), String> {
    let mut file =
        std::fs::File::create_new(tmp).map_err(|e| format!("creating {}: {e}", tmp.display()))?;
    file.write_all(bytes)
        .map_err(|e| format!("writing {}: {e}", tmp.display()))?;
    file.flush()
        .map_err(|e| format!("flushing {}: {e}", tmp.display()))?;
    file.sync_all()
        .map_err(|e| format!("syncing {}: {e}", tmp.display()))
}

/// Copies `src` over `dst` atomically, keeping `src`'s modification time.
pub fn copy(src: &Path, dst: &Path) -> Result<(), String> {
    let tmp = copy_to_temp(src, dst)?;
    commit(&tmp, dst)
}

/// Copies `src` into a synced temp file next to `dst` (with `src`'s modification time) and returns
/// its path; [`commit`] moves it into place. `dst` itself is not touched.
pub fn copy_to_temp(src: &Path, dst: &Path) -> Result<PathBuf, String> {
    let tmp = temp_path(dst)?;
    match copy_synced(src, &tmp) {
        Ok(()) => Ok(tmp),
        Err(e) => Err(discard_all(&[tmp], e)),
    }
}

fn copy_synced(src: &Path, tmp: &Path) -> Result<(), String> {
    std::fs::copy(src, tmp)
        .map_err(|e| format!("copying {} -> {}: {e}", src.display(), tmp.display()))?;
    let mtime = std::fs::metadata(src)
        .and_then(|m| m.modified())
        .map_err(|e| format!("reading modification time of {}: {e}", src.display()))?;
    let file = std::fs::File::options()
        .write(true)
        .open(tmp)
        .map_err(|e| format!("opening {}: {e}", tmp.display()))?;
    file.set_modified(mtime)
        .map_err(|e| format!("setting modification time on {}: {e}", tmp.display()))?;
    file.sync_all()
        .map_err(|e| format!("syncing {}: {e}", tmp.display()))
}

/// Renames the temp file `tmp` over `dst`; on failure the temp file is removed.
pub fn commit(tmp: &Path, dst: &Path) -> Result<(), String> {
    std::fs::rename(tmp, dst).map_err(|e| {
        discard_all(
            &[tmp.to_path_buf()],
            format!("renaming {} -> {}: {e}", tmp.display(), dst.display()),
        )
    })
}

/// Removes every temp file in `temps` and returns `err`, extended with any removal failure.
/// A temp file that does not exist (the failing step never created it) is not an error.
pub fn discard_all(temps: &[PathBuf], err: String) -> String {
    let mut msg = err;
    for tmp in temps {
        match std::fs::remove_file(tmp) {
            Ok(()) => {}
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
            Err(e) => msg = format!("{msg}; removing temp file {}: {e}", tmp.display()),
        }
    }
    msg
}

/// `.<file name>.<pid>.<seq>.tmp` in `dst`'s directory.
fn temp_path(dst: &Path) -> Result<PathBuf, String> {
    static SEQ: AtomicU64 = AtomicU64::new(0);
    let name = dst
        .file_name()
        .ok_or_else(|| format!("{} has no file name", dst.display()))?;
    let seq = SEQ.fetch_add(1, Ordering::Relaxed);
    let mut tmp_name = std::ffi::OsString::from(".");
    tmp_name.push(name);
    tmp_name.push(format!(".{}.{seq}.tmp", std::process::id()));
    Ok(dst.with_file_name(tmp_name))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn unique_dir(label: &str) -> PathBuf {
        static NEXT: AtomicU64 = AtomicU64::new(0);
        let id = NEXT.fetch_add(1, Ordering::Relaxed);
        let dir = std::env::temp_dir().join(format!(
            "nv_rs_xtask_atomic_{label}_{}_{id}",
            std::process::id()
        ));
        std::fs::remove_dir_all(&dir).ok();
        std::fs::create_dir_all(&dir).expect("create test dir");
        dir
    }

    fn entries(dir: &Path) -> Vec<String> {
        let mut v: Vec<String> = std::fs::read_dir(dir)
            .expect("read test dir")
            .map(|e| e.expect("entry").file_name().to_string_lossy().into_owned())
            .collect();
        v.sort();
        v
    }

    #[test]
    fn write_replaces_the_file_and_leaves_no_temp_behind() {
        let dir = unique_dir("write");
        let path = dir.join("COVERAGE.md");
        std::fs::write(&path, "old contents that are longer").expect("seed");
        write(&path, b"new").expect("atomic write");
        assert_eq!(std::fs::read_to_string(&path).expect("read"), "new");
        assert_eq!(entries(&dir), ["COVERAGE.md"]);
        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn failures_name_the_path_and_keep_the_destination() {
        let dir = unique_dir("fail");
        let dst = dir.join("cublas64_13.dll");
        std::fs::write(&dst, "old").expect("seed");
        let missing = dir.join("absent.dll");
        let err = copy(&missing, &dst).expect_err("missing source must fail");
        assert!(err.contains("absent.dll"), "{err}");
        assert_eq!(std::fs::read_to_string(&dst).expect("read"), "old");
        assert_eq!(entries(&dir), ["cublas64_13.dll"]);

        let err = write(&dir.join("no_such_dir").join("x.pyi"), b"x").expect_err("no dir");
        assert!(err.contains("no_such_dir"), "{err}");
        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn copy_keeps_the_source_modification_time() {
        let dir = unique_dir("mtime");
        let src = dir.join("src.dll");
        std::fs::write(&src, "bytes").expect("seed");
        let past = std::time::SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(1_700_000_000);
        std::fs::File::options()
            .write(true)
            .open(&src)
            .and_then(|f| f.set_modified(past))
            .expect("set source mtime");
        let dst = dir.join("dst.dll");
        copy(&src, &dst).expect("copy");
        let got = std::fs::metadata(&dst)
            .and_then(|m| m.modified())
            .expect("dst mtime");
        assert_eq!(got, past);
        assert_eq!(std::fs::read_to_string(&dst).expect("read"), "bytes");
        std::fs::remove_dir_all(&dir).ok();
    }
}
