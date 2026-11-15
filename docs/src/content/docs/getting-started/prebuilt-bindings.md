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

`cargo xtask prebuilt` (alias `stage-prebuilt`) scans `target/*/build/*/out/` and `target/*/*/build/*/out/` for the newest `*_types.rs`, `*_api.rs`, `*_shims.rs`, and `*_inventory.json` files of each `crates/*-sys` crate and copies them into `crates/<crate>/prebuilt/<target_triple>/`.

Optional flags:
- `--target <triple>` — override the target triple (defaults to the `host:` reported by `rustc -vV`)
- `--out <dir>` — write staged `<crate>/prebuilt/<target_triple>/` directories under `<dir>` instead of `crates/`

```bash
cargo xtask prebuilt --target x86_64-unknown-linux-gnu --out dist
```

For SDK installation instructions on machines that do generate bindings from headers, see [SDK Setup](/getting-started/sdk-setup/).
