# Host crate codepaths

See the [repository guide](../AGENTS.md) and [diagrams](../DIAGRAMS.md).

## Binding and runtime layers

```text
cublas-sys/build.rs: SPEC
  -> nv_bindgen::generate
  -> OUT_DIR/cublas_types.rs, cublas_api.rs, cublas_shims.rs, inventory
  -> cublas-sys/src/cublas_sys.rs: exports_sys!
  -> nv_loader::open_first -> CublasApi::from_library
  -> cublas wrapper checks status and owns handle
```

`Shape::Exports` generates normal symbol-loading shims. `SplitExports` supplies separate API tables for library parts (NPP). `QueryInterface` resolves numeric IDs (NVAPI). `FunctionTable` supplies types for runtime tables (OptiX/NVENC). `StaticLink` emits native link requirements (NGX).

Keep the recipe in each sys `build.rs`, shared generation in `nv-bindgen`, and shared discovery in `nv-loader`. Do not duplicate loaders in wrapper crates.

## Resource and interop ownership

```text
CUDA Context (Arc)
  -> OptiX DeviceContext
     -> Module -> ProgramGroup -> Pipeline
     -> acceleration structures / denoiser / transforms

wgpu adapter + device -> backend support + UUID/LUID checks
  -> InteropContext
     -> SharedBuffer / SharedTexture -> CUDA imported memory
     -> InteropSemaphore -> GPU-side ordering
```

OptiX objects retain dependencies through owned handles and `Arc` references. Preserve these relationships on error paths and destruction. Buffer validity includes allocation size, element layout, alignment, context identity, and completion of asynchronous use.

## Review checklist

- A finding from review or audit is a claim until re-read against the source and the native header, and, where reading cannot decide it, measured on hardware. In the 2026-11-23 audit a substantial number of re-verified findings were refuted or much narrower than reported; they are listed one by one in `plan2.md`, "Execution record".
- Buffer lifetime: `cuda-core` frees a dropped buffer without waiting for GPU work. Synchronous buffers (`zeroed`, `from_host`) did not corrupt in measurement; a stream-ordered buffer (`uninitialized_async`) freed on its own stream while another stream still uses it did, 20/20. Keep internal scratch on one checked stream; lease caller buffers with `cuda_core_ext::lease::enqueue_leased` where they can cross streams.
- Scratch budgets bound the whole allocation, and the old buffer is released before the new one is allocated.
- Search sibling wrappers when a status, dimension, lifetime, overflow, or callback bug appears.
- Validate sizes and layout before unsafe native calls; document why each pointer remains valid.
- Read the actual target-specific handle ownership rules before closing exported handles.
- For interop, match the physical CUDA and graphics device and enable required backend extensions.
- Treat `let _ =` in cleanup separately from ignored operational failures; inspect how destruction errors are recorded.
- Use synthetic generator fixtures and loader probes for hardware-independent regression coverage; run ignored integration tests for driver-dependent changes.
