# Repository guide

Verified against source on 2026-11-15. See [crate guidance](crates/AGENTS.md), [architecture diagrams](DIAGRAMS.md), and [build instructions](README.md#building--verification).

## Workspace boundaries

The root Cargo workspace contains `crates/*` and `xtask`. `device/`, `python/`, and `spikes/` are excluded and have separate build requirements. A successful root workspace check does not validate them.

## Dataflow

```text
SDK headers + LibSpec in each sys build.rs
  -> nv-bindgen::generate (vcv-rs discovery, bindgen, shims, inventory)
  -> OUT_DIR generated types/API/shims
  -> *-sys crate -> Rust wrapper -> application / Python extension
                         |
                         +-> cuda-core contexts, streams, device buffers

runtime DllSpec -> nv-loader::candidates -> open_first -> open_library
  -> cached API / cached LoadError -> native entry point / typed failure

wgpu Vulkan or DX12 allocation
  -> exported OS handle -> cuda-core-ext import
  -> nv-wgpu shared resource + timeline semaphore -> CUDA / graphics work
```

## Source owners

| Area | Responsibility |
| --- | --- |
| `crates/nv-bindgen` | Header discovery, generated bindings, return policies, prebuilt fallback, device PTX wrapper generation |
| `crates/nv-loader` | Runtime library search, shared loader macros, diagnostics |
| `crates/*-sys` | Per-library binding recipe and runtime entry-point adaptation |
| Wrapper crates | Validation, native status conversion, resource ownership |
| `crates/cuda-core-ext` | External memory/semaphores, arrays, textures, sparse and green-context extensions |
| `crates/nv-wgpu` | GPU identity matching, external memory and semaphore interop |
| `crates/nv-demo` | Interactive renderer, camera controllers, exports and profiling |
| `xtask` | Coverage inventories, prebuilt artifacts and runtime deployment tooling |
| `device/` | Separately compiled CUDA/OptiX device code |
| `python/` | Separate Python extension workspace |
| `docs/` | Astro/Starlight documentation site |

## Traps and verification

- Inspect each crate's Cargo `[lib].path`; entry points commonly use names such as `nv_loader.rs`, not `lib.rs`.
- Configure runtime search directories before the first sys call. The loader macros cache both success and failure in `OnceLock<Result<...>>`; later search-directory additions do not retry a failed initialization.
- Generated output belongs in `OUT_DIR`; change `LibSpec`, generator logic, or loader macros rather than patching generated files.
- NGX uses static linking; prebuilt Rust bindings do not supply its native libraries.
- GPU integration tests marked `#[ignore]` need explicit execution on suitable hardware. Compile success alone does not verify driver behavior, stream ordering, callbacks, or interop.
- Check OS-gated branches separately: Windows-only NVAPI and Linux storage/collective paths cannot all execute on one host.
- Preserve CUDA context ownership and stream ordering when changing wrappers. Keep parent objects alive until dependent native objects and queued operations are finished.

Run `cargo clippy --workspace --all-targets` and `cargo test --workspace` for host changes. Inspect full diagnostics before suppressing warnings. Scope extra checks to the affected excluded workspace when relevant.
