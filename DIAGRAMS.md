# Architecture and runtime flows

Source map verified on 2026-11-15. See [repository guidance](AGENTS.md) and [crate codepaths](crates/AGENTS.md).

## Build architecture

```mermaid
flowchart TD
  Spec["*-sys/build.rs: LibSpec"] --> Generate["nv-bindgen::generate"]
  SDK["vcv-rs SDK discovery"] --> Generate
  Headers["Installed SDK headers"] --> Bindgen["bindings / shims / inventory"]
  Generate --> Bindgen
  Prebuilt["Target-specific prebuilt artifacts"] --> Generate
  Bindgen --> Out["OUT_DIR generated Rust + inventory JSON"]
  Generate --> Out
  Out --> Sys["*-sys crate"]
  Sys --> Wrapper["Rust wrapper"]
  Cuda["cuda-core / cuda-bindings"] --> Wrapper
  Wrapper --> App["nv-demo / applications"]
  Wrapper --> Py["Separate python workspace"]
  Device["Separate device workspace"] --> PTX["PTX / device code"]
  PTX --> App
```

Prebuilt fallback depends on matching artifacts. Static-link shapes still require native link inputs; normal export shapes load libraries at runtime.

## First call and cached failures

```mermaid
sequenceDiagram
  participant Caller
  participant Shim as Generated shim
  participant Cache as Sys OnceLock
  participant Loader as nv-loader
  participant Native as Native library
  Caller->>Shim: Call entry point
  Shim->>Cache: Get API result
  alt First call
    Cache->>Loader: open_first(DllSpec)
    Loader->>Loader: Enumerate candidates and attempt load
    Loader-->>Cache: Loaded API or LoadError
  end
  Cache-->>Shim: Cached result
  alt API and symbol available
    Shim->>Native: Native call
    Native-->>Caller: Native result
  else Library or symbol unavailable
    Shim-->>Caller: Configured return policy / checked failure
  end
```

The shared macros retain a failed initialization for the process lifetime. Configure search paths before the first call.

## Graphics and CUDA interop

```mermaid
flowchart LR
  Adapter["wgpu adapter/device"] --> Verify["Backend support + UUID/LUID match"]
  CUDA["CUDA Context"] --> Verify
  Verify --> Context["InteropContext"]
  Context --> Shared["SharedBuffer / SharedTexture"]
  Shared --> Export["Export Vulkan or DX12 handle"]
  Export --> Import["cuda-core-ext import"]
  Import --> Work["CUDA device access"]
  Context --> Semaphore["InteropSemaphore"]
  Semaphore --> Work
  Semaphore --> Graphics["Graphics queue access"]
  Shared --> Graphics
```

External memory shares storage. Timeline synchronization controls when either API may access it; sharing alone does not order work.
