---
title: Prebuilt Bindings
description: Using pre-generated bindings when SDK headers or libclang are unavailable, and staging them with cargo xtask prebuilt.
sidebar:
  order: 3
---

When SDK headers or `libclang` are not installed on a build host, `nv-bindgen` can fall back to pre-generated bindings instead of running `bindgen` at build time.

## How Prebuilt Fallback Works

`nv-bindgen` checks for prebuilt bindings in two situations:

1. **Forced via `NV_RS_USE_PREBUILT=1`** — skips header discovery and `libclang` entirely and loads prebuilt artifacts directly.
2. **Automatic fallback** — when SDK headers or `libclang` are unavailable and matching prebuilt artifacts exist for the Cargo `TARGET` triple.

## Artifact Naming

For a library with `LibSpec::name` = `{name}`, `nv-bindgen` expects the following files depending on the library's binding shape:

| Artifact | Purpose |
|---|---|
| `{name}_types.rs` | Generated C types, structs, enums, and constants (always required) |
| `{name}_shims.rs` | Runtime loader or static-link shims (for `Exports`, `QueryInterface`, and `StaticLink` shapes; split shapes use `{name}_{part}_shims.rs`) |
| `{name}_api.rs` | Exported function table bindings (for `Exports` shapes; split shapes use `{name}_{part}_api.rs`) |
| `{name}_inventory.json` | Symbol inventory and header SHA-256 metadata (always required) |

Every build also writes `{name}_manifest.json` into `OUT_DIR` after all other files: `{"lib": "...", "origin": "generated" | "prebuilt", "files": [...]}`. `nv-bindgen` fails the build if a promised file is missing, a build that used the committed prebuilt set prints a `cargo:warning` and records `origin: prebuilt`, and each sys crate exposes the manifest path as `BINDINGS_MANIFEST`. The manifest itself is never staged.

## Directory Resolution

`nv-bindgen` resolves the prebuilt directory in this order:

1. **`NV_RS_PREBUILT_DIR`** — if set, `nv-bindgen` checks `<NV_RS_PREBUILT_DIR>/<target_triple>/` first, then `<NV_RS_PREBUILT_DIR>/` directly.
2. **Crate-local `prebuilt/` directory** — otherwise it looks in `crates/<crate>/prebuilt/<target_triple>/` (specifically `<CARGO_MANIFEST_DIR>/prebuilt/<TARGET>/`).

```powershell
# Force prebuilt bindings from a shared directory
$env:NV_RS_USE_PREBUILT = "1"
$env:NV_RS_PREBUILT_DIR = "C:\path\to\prebuilt"
```

```bash
export NV_RS_USE_PREBUILT=1
export NV_RS_PREBUILT_DIR="/path/to/prebuilt"
```

## Staging Prebuilt Artifacts (`cargo xtask prebuilt`)

On a machine that has the NVIDIA SDKs and `libclang` installed, build the `*-sys` crates first and then stage their generated `OUT_DIR` artifacts with `xtask`:

```bash
cargo xtask prebuilt
```

`cargo xtask prebuilt` (alias `stage-prebuilt`) resolves the Cargo target directory from `CARGO_TARGET_DIR` (resolving relative paths against the workspace root, defaulting to `target/`) and, for each `crates/*-sys` crate, picks the most recent out dir whose `{name}_manifest.json` says `origin: generated` and lists only files that exist; exactly those files are copied all-or-nothing into `crates/<crate>/prebuilt/<target_triple>/`. Out dirs without a manifest or built from the prebuilt set are rejected with the reason, and the command fails if a built crate has no complete set.

Target isolation rules:
- **Target-qualified builds** (`<target_dir>/<target>/<profile>/build/*/out/`): always scanned for the requested `--target`.
- **Unqualified builds** (`<target_dir>/<profile>/build/*/out/`, e.g. `debug/` or `release/`): scanned **only** when `--target` matches the `host:` triple reported by `rustc -vV`, preventing host build artifacts from leaking into a foreign-target prebuilt directory.

Optional flags:
- `--target <triple>` — override the target triple (defaults to the `host:` reported by `rustc -vV`)
- `--out <dir>` — write staged `<crate>/prebuilt/<target_triple>/` directories under `<dir>` instead of `crates/`

```bash
cargo xtask prebuilt --target x86_64-unknown-linux-gnu --out dist
```

## Safe SDK Archive Extraction (`bootstrap.py`)

When `bootstrap.py` unpacks `.tar.xz` / `.tar.gz` SDK archives using Python's `tarfile` (on Windows or when external `tar` is unavailable), it uses a two-pass extraction with path-traversal guards:

1. **Pass 1 (files & directories)**: validates every member path against absolute paths, drive prefixes, `..` components, and escapes outside the destination root before creating directories and extracting regular files.
2. **Pass 2 (hardlinks & symlinks)**: validates link targets against absolute, drive-prefixed, or escaping paths, then creates hardlinks/symlinks (falling back to file/directory copies with bounded multi-pass resolution for chained symlinks when OS symlink/hardlink privileges are unavailable).

For SDK installation instructions on machines that do generate bindings from headers, see [SDK Setup](/nv-rs/getting-started/sdk-setup/).
