# nv-rs audit: system crates (ngx, nvapi, nvml, cupti, nvrtc, nccl) + xtask / bootstrap / CI

Date: 2026-11-23. Read-only audit. HEAD `38ec140`. No cargo/build/test run.

Severity: P0 = UB / memory unsafety / crash / wrong results; P1 = leak, swallowed error, silent fallback, significant perf; P2 = SSOT/dedup, dead code, doc drift.
Verdict: CONFIRMED = failing path traced in code (and, where relevant, in the vendored SDK header under `.sdk/`); PLAUSIBLE = needs runtime/hardware or vendor-behaviour confirmation.

## 1) Item-by-item ledger

### NGX (`crates/ngx`)

| ID | Sev | Verdict | Location | Defect | Failing scenario | Fix direction |
|---|---|---|---|---|---|---|
| NGX-1 | P0 | CONFIRMED (contract) / PLAUSIBLE (impact) | `crates/ngx/src/vulkan.rs:273-280`, `:299-303` | `VulkanSession` is `unsafe impl Send + Sync`, and `NgxVulkan` is `Clone` (+ auto Send/Sync). But `.sdk/ngx/include/nvsdk_ngx_vk.h:16-17` and `nvsdk_ngx.h:18-19` say: "Methods in this library are NOT thread safe. It is up to the client to ensure thread safety". The SAFETY comments only justify the final `Shutdown1`, not concurrent NGX calls. | Clone `NgxVulkan` into two threads. Thread A calls `capability_parameters()` / `is_dlss_rr_available()` while thread B calls `allocate_parameters()` or `create_dlss_*`. That races NGX global state from safe code. | Make `NgxVulkan` `!Sync`, or serialize every NGX entry point through a `Mutex` held in `VulkanSession`. Rewrite the SAFETY comments to cite the header contract. |
| NGX-2 | P1 | PLAUSIBLE | `crates/ngx/src/vulkan.rs:776-785` (FeatureHandle Drop), `:283-293` (VulkanSession Drop) | `ReleaseFeature` and `Shutdown1` run from a safe `Drop` with no GPU-idle precondition. The header says handles are freed immediately (`nvsdk_ngx_vk.h:535-553`). Nothing documents that command buffers recorded with `evaluate*` must have completed. | Drop a `FeatureHandle` while a submitted command buffer from `evaluate_dlss_rr_with_params` is still in flight. NGX frees the feature's internal resources, so the GPU reads freed memory or the device is lost. | Document the precondition on `FeatureHandle`/`NgxVulkan`, or have callers pass a fence/timeline value that Drop waits on. Expose an explicit `unsafe fn release(self)`. |
| NGX-3 | P1 | PLAUSIBLE | `crates/ngx/src/vulkan.rs:1149-1197` | Nothing guards against a second `init` on the same `VkDevice`. Each `NgxVulkan` owns its own `Arc<VulkanSession>` whose Drop calls `Shutdown1(device)`. | `NgxVulkan::init(dev)` twice (for example two subsystems). Dropping the first session shuts NGX down for that device while the second session's features and parameter bags are still alive. Use-after-shutdown. | Keep a process-global registry (device -> `Weak<VulkanSession>`) and return the existing session, or reject a duplicate init. |
| NGX-4 | P1 | CONFIRMED | `crates/ngx/src/error.rs:34-37`, `:120-127`; `crates/ngx/src/ngx.rs:42-44` | `check` treats only `== 1` as success. The SDK predicate is mask-based: `.sdk/ngx/include/nvsdk_ngx_defs.h:183-184`, `NVSDK_NGX_SUCCEED(v) ((v & 0xFFF00000) != NVSDK_NGX_Result_Fail)`. The official helper uses `NVSDK_NGX_FAILED(Res)` on the optimal-settings callback (`nvsdk_ngx_helpers.h:97`). | A runtime DLL returns an informational non-`0xBAD0xxxx` success code from `Init`, `CreateFeature` or the optimal-settings callback. The wrapper reports `Error::Ngx` and drops a working feature. On `CreateFeature` the handle may already be allocated and is then leaked. | `is_success = (raw as u32 & 0xFFF0_0000) != 0xBAD0_0000`. Add a unit test pinned to the header macro. |
| NGX-5 | P1 | CONFIRMED | `crates/ngx/src/vulkan.rs:609-621` | `query_optimal_settings_with_key` uses `.unwrap_or(render_width)`, `.unwrap_or(render_height)` and `.unwrap_or(0.0)` on dynamic min/max and sharpness. Any Get failure (missing key or a real error) becomes plausible-looking data. This matches the C helper, which also ignores these, but nothing documents it. | A runtime without dynamic-resolution keys reports `min == max == optimal`. Callers believe dynamic resolution is unsupported, or sharpness is silently 0. The caller cannot tell real data from a fallback. | Return `Option<u32>` for min/max/sharpness, or document the fallback on `DlssOptimalSettings`. |
| NGX-6 | P2 | CONFIRMED | `crates/ngx/src/vulkan.rs:556-565` | `is_dlss_rr_available` maps every `Error::Ngx` from `get_i32` to `Ok(false)`. It never reads `SuperSamplingDenoising.NeedsUpdatedDriver` or `.FeatureInitResult` (`nvsdk_ngx_defs_dlssd.h:152-155`). There is no SR availability query at all. | On an old driver, RR reports "unavailable" with no reason. The user cannot tell "update driver" from "unsupported GPU". | Return a typed availability struct (available, needs driver update with min version, init result). |
| NGX-7 | P2 | CONFIRMED | `crates/ngx/src/vulkan.rs:864-870`, `:1042-1055` | `evaluate_*_with_params` stores pointers to stack locals (`color_res`, `depth_res`, matrices) into a caller-owned, reusable `Parameters` bag. After the call returns, the bag holds dangling pointers. | The caller reuses the bag and calls `FeatureHandle::evaluate(cmd, &params)` directly, which is an `unsafe` but documented entry. NGX dereferences freed stack memory (UAF). | After evaluate, overwrite the resource keys with null. Or document that the bag is invalid for raw `evaluate` afterwards. |
| NGX-8 | P2 | CONFIRMED | `crates/ngx/src/vulkan.rs:190`, `:853-856`, `:1008-1017` | The doc says `output` "requires `read_write = true`", but `validate` only checks null handles. Render dims are not checked against the input extent, and output dims are not checked against creation. | `output.read_write = false` -> NGX returns `FAIL_RWFlagMissing` at runtime instead of an early `InvalidArgument`. | Validate `output.read_write` and `render_width <= color.width` in `validate`. |
| NGX-9 | P2 | CONFIRMED | `crates/ngx/src/vulkan.rs:683`, `:830`, `:922` | Three `#[allow(clippy::too_many_arguments)]`. The repo rule says `#[expect(.., reason)]`. RR already uses `DlssRrCreateParams`/`DlssRrEvalParams`, so the SR API is asymmetric. | — | Add `DlssSrCreateParams`/`DlssSrEvalParams` mirroring RR. That removes all three allows. |
| NGX-10 | P2 | CONFIRMED | `crates/ngx/src/vulkan.rs:941-945` vs `:447-455` | `evaluate_dlss_sr` duplicates `allocate_parameters` (raw `AllocateParameters` + `from_raw`). It also allocates and destroys a parameter bag every frame, which is per-frame churn. | — | Delegate to a shared allocator. Consider removing the allocating variant. |
| NGX-11 | P2 | CONFIRMED | `crates/ngx/src/vulkan.rs:308-331` | Uses `NVSDK_NGX_VULKAN_RequiredExtensions`, which `nvsdk_ngx_vk.h:85-96` marks DEPRECATED (use `GetFeatureInstanceExtensionRequirements` / `GetFeatureRequirements`). | Future SDK removes or changes it, and RR-specific device extensions are not reported. | Bind the per-feature requirement queries. |
| NGX-12 | P2 | CONFIRMED | `crates/ngx/src/vulkan.rs:152, 289, 781` | Teardown failures of `DestroyParameters`/`Shutdown1`/`ReleaseFeature` are dropped silently with `let _ =`. cupti/nccl record teardown errors on the `Context`. | Shutdown failure is invisible. | Log or record the error (no `Context` here, so use a `tracing::warn!` or an error slot). |
| NGX-13 | P2 | CONFIRMED | `crates/ngx/tests/gpu.rs:7-8` vs `:24-26` | The module doc says the test "gracefully skips if `FAIL_NotImplemented` or `PlatformError`". The code panics on every `Error::Ngx`. | Doc drift. | Align the doc or the match arms. |
| NGX-14 | P2 | CONFIRMED | `crates/ngx/src/vulkan.rs:873` | Sets `Sharpness` on every SR evaluate, but `DoSharpening` is marked `SR_DEPRECATED_SHARPENING ... Sharpness is not supported` (`nvsdk_ngx_defs.h:296`). | Misleading API parameter. | Drop or deprecate the `sharpness` argument. |

Of the 13 `let _ =` sites in `params.rs`, 12 are argument sinks in the `cfg(not(has_ngx_sdk))` stubs and one is the `DestroyParameters` in Drop (NGX-12). The 13 sites in `vulkan.rs` are 11 stub sinks plus `Shutdown1` and `ReleaseFeature` in Drop (NGX-12). None of them hides a capability-query or feature-creation failure. Those are hidden instead by the `unwrap_or` (NGX-5) and the `Err(Ngx)=>Ok(false)` (NGX-6).

### NVAPI (`crates/nvapi`)

| ID | Sev | Verdict | Location | Defect | Failing scenario | Fix direction |
|---|---|---|---|---|---|---|
| NVAPI-1 | P2 | CONFIRMED | `crates/nvapi/src/error.rs:124-134` (`Display` -> `driver_message` :93-106) | `Display for Status` calls FFI (`sys::load()` + `NvAPI_GetErrorMessage`) every time an error is formatted, on any thread, even after the last `NvApi` dropped and `NvAPI_Unload` ran. | Formatting a stored error after unload calls into an uninitialized NVAPI. It returns not-initialized (handled), but the result is nondeterministic, and logging has side effects. | Capture the driver message once, when the error is created inside `check` (session alive). |
| NVAPI-2 | P2 | CONFIRMED | `crates/nvapi/src/error.rs:162` | `#[cfg_attr(not(windows), allow(dead_code))]`. The repo rule wants `#[expect(.., reason)]`, or gate `check` with `#[cfg(windows)]`. | — | `#[cfg(windows)]` on `check`. |
| NVAPI-3 | P2 | CONFIRMED | `crates/nvapi/src/win.rs:187, 241, 279, 299, 345, 381, 486, 512, 546, 564, 662` | The struct version numbers (1/2/3) are hand-copied magic numbers. All 11 match `.sdk/nvapi/nvapi.h` today (verified: `NV_GPU_UUID_VER1`, `ARCH_INFO_VER_2`, `THERMAL_SETTINGS_VER_2`, `CLOCK_FREQUENCIES_VER_3`, `HDR_CAPABILITIES_VER3`, `GSYNC_CAPABILITIES_VER3`, `VRR_INFO_VER1`, `GSYNC_STATUS` 1, `DYNAMIC_PSTATES_INFO_EX` 1, `MEMORY_INFO_EX` 1, `NVDRS_PROFILE_VER1`). No test ties them to the header, so an SDK bump that retypedefs a struct to `_V(n+1)` returns `INCOMPATIBLE_STRUCT_VERSION` at runtime. | SDK update -> runtime-only failure. | Generate `*_VER` consts in `nvapi-sys` (nv-bindgen macro evaluation), or add a per-struct size assertion against the prebuilt layout. |

Clean: enumeration loops (END_ENUMERATION and DEVICE_NOT_FOUND are handled separately from real errors), array sizes = `NVAPI_MAX_*` matching the header prototypes, Initialize/Unload pairing per `Arc` (NVAPI refcounts), `DrsSession` destroy on the `LoadSettings` error path, and non-Windows returning a typed `UnsupportedPlatform`.

### NVML (`crates/nvml`)

| ID | Sev | Verdict | Location | Defect | Failing scenario | Fix direction |
|---|---|---|---|---|---|---|
| NVML-1 | P2 | CONFIRMED | `crates/nvml/src/types.rs:437-438` | Rubin is matched as the raw literal `13`, a magic number that duplicates a future `sys::NVML_DEVICE_ARCH_RUBIN`. | When the constant arrives with a different value, the name is wrong. | Version-gate on the sys constant via nv-bindgen cfg. |
| NVML-2 | P2 | CONFIRMED | `crates/nvml/src/error.rs:121-130` | Same pattern as NVAPI-1: `Display` calls `nvmlErrorString` via `sys::load()` on every format. This is benign (that call needs no init). | — | Optional: cache it. |

Clean: init/shutdown refcount per `Arc<NvmlSession>`, `Device` pins the session, string buffers use the NVML `*_BUFFER_SIZE` constants with full-buffer-safe decode, the bounded INSUFFICIENT_SIZE retry (`device.rs:500-526`, handles count shrink/grow and a zero first call), and the `FUNCTION_NOT_FOUND` fallbacks (the sys missing-symbol policy is verified in `nvml-sys/build.rs:21`).

### CUPTI (`crates/cupti`)

| ID | Sev | Verdict | Location | Defect | Failing scenario | Fix direction |
|---|---|---|---|---|---|---|
| CUPTI-1 | P1 | CONFIRMED | `crates/cupti/src/activity.rs:170`, `:316-341`, `:384-389`, `:407-418` | Activity state (records, dropped count, enabled kinds in CUPTI) is process-global. `ActivityProfiler` presents itself as "bound to `cuda`" and tracks `enabled` per instance. `cuptiActivityEnable` is global, so the per-context call `cuptiActivityEnableContext` is never used. | Two profilers enable `Memcpy`. Dropping one disables `Memcpy` for the other, which silently stops receiving records. `drain_records` on profiler A steals profiler B's records, including records from other contexts. | Make the profiler a process singleton with refcounted kinds, or use `cuptiActivityEnableContext`/`DisableContext` and filter records by `contextId`. |
| CUPTI-2 | P1 | CONFIRMED | `crates/cupti/src/activity.rs:419-429` + `:243-246` | The Drop's forced flush appends final records into the global `records` Vec. Nothing clears them. | The next `ActivityProfiler::new` followed by `drain_records` returns stale records (and the dropped count) from the previous session, which pollutes measurements. Records also grow unbounded if never drained. | Clear `records`/`dropped` when the last profiler drops (refcount), or let Drop return the final records. |
| CUPTI-3 | P1 | PLAUSIBLE | `crates/cupti/src/activity.rs:268-299` | Records are cast to `CUpti_ActivityKernelLatest` (= `Kernel12`, `CUPTI_API_VERSION 130301` in prebuilt) and `CUpti_ActivityMemcpy6`, with no check that the loaded CUPTI library produces those versions. `cupti-sys` accepts `libcupti.so.12`, and `cupti64_*` picks whatever toolkit is installed. `version()` exists but is never compared to `CUPTI_API_VERSION`. | CUDA 12 runtime CUPTI with CUDA 13.3 headers. Kernel records are a smaller `Kernel9/10`, and `&*record_ptr.cast::<Kernel12>()` creates a reference that extends past the record, possibly past `valid_size`. That is UB, with garbage in any field beyond the common prefix. | In `ActivityProfiler::new`, require `cuptiGetVersion() >= CUPTI_API_VERSION` (or map version to struct). Read fields with `addr_of!`/`read_unaligned` instead of forming references. |
| CUPTI-4 | P1 | PLAUSIBLE | `crates/cupti/src/subscriber.rs:152-168` | Drop frees `SubscriberInner` right after `cuptiUnsubscribe`. CUPTI does not document that Unsubscribe waits for callbacks already running on other threads. | A driver-API callback is executing on thread B while thread A drops the `Subscriber`. B dereferences freed `userdata` (UAF). | Keep `inner` in an `Arc` with a quiescence counter, or leak a small tombstone. At minimum, document the requirement. |
| CUPTI-5 | P2 | CONFIRMED | `crates/cupti/src/activity.rs:413-428`, `subscriber.rs:158-161` | Teardown failures are recorded as `CUDA_ERROR_UNKNOWN`, which loses the real `CUptiResult`. | Diagnostics show "unknown". | Add an error variant that carries the CUPTI code. |
| CUPTI-6 | P2 | CONFIRMED | `crates/cupti/src/subscriber.rs:54-55` | The doc says the callback gets `(domain, cbid, raw_cbdata)`, but the type is `Fn(CallbackDomain, u32)` and `_cbdata` is dropped (`:66`). The callback cannot tell API enter from exit, and gets no correlation ID or function params. | Doc drift. The feature is half-finished. | Pass a typed `CallbackData<'_>` view (site, correlation id, function name). |
| CUPTI-7 | P2 | CONFIRMED | `crates/cupti/src/cupti.rs:21` | A crate-wide `#![cfg_attr(target_os = "linux", allow(clippy::unnecessary_cast))]` violates the repo rule (no blanket `allow`). | — | Localize the casts with `from_ne_bytes`, as ngx does. |

### NVRTC (`crates/nvrtc`)

| ID | Sev | Verdict | Location | Defect | Failing scenario | Fix direction |
|---|---|---|---|---|---|---|
| NVRTC-1 | P2 | CONFIRMED | `crates/nvrtc/src/program.rs:109-110` | After a successful `nvrtcCompileProgram`, a failure of `build_log()` turns the whole compile into `Err(Status{nvrtcGetProgramLogSize})`. The compiled program is fine, but the caller treats it as failed. | Unlikely log-query failure -> the successful compile is discarded. | On the success path, return `Ok(log.unwrap_or_default())`, or return the log separately. |
| NVRTC-2 | P2 | CONFIRMED | `crates/nvrtc/tests/gpu.rs:19-26` | `version_and_supported_archs` is `#[ignore = "GPU"]` but needs no GPU, only the NVRTC DLL, so it is never run by default. `cuModuleUnload`'s result is ignored (`:119`), and the module leaks if an assert fails before it. | Coverage gap. | Un-ignore the host-only test, or tag it "needs toolkit". |

Clean: CString lifetimes span each FFI call, log/PTX/CUBIN/LTO-IR/OptiX-IR size queries allocate exactly `size` bytes (PTX keeps its NUL for `cuModuleLoadData`, and the log is trimmed at the first NUL), program destroy happens in Drop on every path, name-expression support works, and `c_int::try_from` guards counts.

### NCCL (`crates/nccl`)

| ID | Sev | Verdict | Location | Defect | Failing scenario | Fix direction |
|---|---|---|---|---|---|---|
| NCCL-1 | P1 | CONFIRMED | `crates/nccl/src/comm.rs:89-115` | `GroupGuard { active: bool }` is auto `Send + Sync`. NCCL group nesting is thread-local (the comment at `:122` says so). | Create the guard on thread A, move it to thread B, drop it. `ncclGroupEnd` runs on B (`ncclInvalidUsage`, swallowed by `let _`). Thread A stays in group mode forever, so every later collective on A is deferred indefinitely and hangs. | Add `PhantomData<*const ()>` to make it `!Send + !Sync`. |
| NCCL-2 | P1 | PLAUSIBLE | `crates/nccl/src/comm.rs:941-972` (and the doc at `:159-162`) | Drop calls `ncclCommDestroy` when `async_err == 0`. In NCCL >= 2.18, Destroy finalizes implicitly and can block until peers participate. The doc claims "drop never stalls". | Multi-process clique where a peer has exited without error yet. Drop blocks forever in `ncclCommDestroy`. | Use the non-blocking config plus `ncclCommFinalize` polling with a timeout in Drop, falling back to `ncclCommAbort`. Or document the blocking. |
| NCCL-3 | P1 | PLAUSIBLE | `crates/nccl/src/comm.rs:437-444`, `:954-961` | After `ncclCommDestroy` fails, the code calls `ncclCommAbort(raw)` on the same handle. NCCL does not guarantee the handle is still valid after a failed Destroy. | Partially destroyed communicator -> abort double-frees or touches freed state. | Abort only when Destroy was not attempted, or check the NCCL docs for the version in use. |
| NCCL-4 | P2 | CONFIRMED | `crates/nccl/src/comm.rs:112` | `let _ = group_end()` in `GroupGuard::drop` swallows the error. The `group()` helper also drops the end error when `f` failed (`:150-154`). This is documented, but on an error path a partial group still launches the already-enqueued collectives. | `f` enqueues rank-0's all_reduce, then fails validation on the next op. `ncclGroupEnd` launches an unmatched collective and peers deadlock. | Document it. Consider `ncclGroupSimulateEnd`/abort semantics for the error path. |
| NCCL-5 | P2 | CONFIRMED | `crates/nccl/src/nccl.rs:135-141`, `comm.rs:966-969` | Teardown NCCL codes collapse into `CUDA_ERROR_UNKNOWN` (same as CUPTI-5). | Diagnostics lost. | Carry the `NcclStatus`. |
| NCCL-6 | P2 | PLAUSIBLE | `crates/nccl/src/comm.rs:204-213` | `ncclCommInitAll`'s `devlist` takes CUDA device ordinals. The code passes `ctx.cu_device()`, a `CUdevice` driver handle (cuda-core `runtime.rs:86`). These are numerically equal on current drivers, but the API does not guarantee it. | Future driver with opaque `CUdevice` -> wrong GPU mapping. | Store and pass the ordinal used in `Context::new`. |
| NCCL-7 | P2 | CONFIRMED | `crates/nccl/src/comm.rs:480-492` | `all_reduce(&S, &mut DeviceBuffer<R>)` cannot express in-place reduction (one buffer can't be borrowed `&` and `&mut` at once). NCCL supports in-place and it is the common case. | Users must allocate a second buffer. | Add `all_reduce_in_place(&mut DeviceBuffer)`. |
| NCCL-8 | P2 | CONFIRMED | `crates/nccl/Cargo.toml:3,7` | `version = "0.1.0"` and `publish = false` are hard-coded instead of `.workspace = true` like every other crate. | SSOT drift on a version bump. | Use `version.workspace = true`, `publish.workspace = true`. |

The 9 `let _ =` sites in `comm.rs` are: `:112` (the real swallow, NCCL-4), `:197` (the try_from result is checked with `?`, so it is benign), and `:299, :453, :474, :534, :617, :704, :827`, which are argument sinks in the non-Linux stubs. On Windows every runtime entry point returns a typed `Error::Unsupported` (not a silent no-op), and `UniqueId`/`DataType`/validation stay usable. That behaviour is correct.

### xtask / bootstrap / CI / workspace config

| ID | Sev | Verdict | Location | Defect | Failing scenario | Fix direction |
|---|---|---|---|---|---|---|
| X-1 | P1 | CONFIRMED | `xtask/src/stage_dlls.rs:333-336`, `:386-400`; `.github/workflows/release.yml:158`, `:176` | `stage-dlls` records missing libraries but always returns `Ok`, so the exit code is 0 even when the user explicitly named them in `--libs`. The release workflow relies on this command to bundle DLLs. | No `cufft` or `dlss` found -> the release zip/tgz ships without them, CI is green, and users get a runtime `LoadError`. | Return `Err` when any explicitly requested (`--libs`) library is missing. Add a `--allow-missing` opt-out. |
| X-2 | P1 | CONFIRMED | `xtask/src/stage_dlls.rs:144-152`; `crates/nv-loader/src/nv_loader.rs:423-471` | The `dlss` spec stages only `nvngx_dlss.dll`. `nvngx_dlssd.dll` (Ray Reconstruction), which AGENTS.md says nv-demo needs, is not in the catalog. On Linux the spec is `libnvidia-ngx-dlss.so*`. `linux_candidates_with` does no glob expansion (the literal `*` stays in the path, so `is_file()` is never true) and does not search `lib/Linux_x86_64/rel`. | Windows release: no RR DLL in the bundle (nv-demo's build.rs copy lands only in `target/`). Linux release: no DLSS library at all. Exit 0 either way (X-1). | Add `nvngx_dlssd.dll` / `libnvidia-ngx-dlssd.so*`. Implement glob expansion and the NGX rel dirs in the Linux candidate path. |
| X-3 | P1 | CONFIRMED | `xtask/src/stage_dlls.rs:38-183` vs `crates/*-sys/src/*.rs`, `crates/nv-loader/src/diag.rs:410-500` | The DllSpec catalog is hand-copied ("mirroring crates/*-sys"), with drift. Every `Group::Sdk` entry here has `hint: None`, while `cudnn-sys`, `nccl-sys` and `nvcomp-sys` use `hint: nv_loader::hint(env!("NV_RS_SDK_BIN"))`. A third catalog exists in `nv-loader/src/diag.rs` (with `curand`, `libcufft.so.11`). `curand` is absent from stage-dlls. | cuDNN/nvCOMP found by the runtime loader via the `.sdk` bin hint are reported "missing" by stage-dlls (exit 0) and are not bundled. | Re-export each sys crate's `DLL` const (several already export `pub const DLL`) and build the list from them. Delete the copies. |
| X-4 | P1 | PLAUSIBLE | `xtask/src/stage_dlls.rs:52-60`, `:171-176` | It stages only the primary DLL of split libraries. cuDNN 9 loads `cudnn_graph64_9`, `cudnn_ops64_9`, `cudnn_engines_*` siblings (see the `cudnn-sys` doc at `src/cudnn_sys.rs:8-10`). NVRTC loads `nvrtc-builtins64_*.dll`, which `release.yml:159-164` works around with a manual glob. | A bundled cuDNN/NVRTC fails at first use on a machine without the toolkit. | Model companion DLLs in `DllSpec` (SSOT), so stage-dlls and the loader preload agree. |
| X-5 | P1 | CONFIRMED | `xtask/src/prebuilt.rs:136-153`, `:174-199` | Prebuilt staging (1) picks the out dir with the newest mtime even if it holds only some artifacts (a single `_types.rs` qualifies, see the test at `:374-413`), (2) copies file by file with no completeness check, and (3) never deletes stale files in `prebuilt/<target>/`. | An interrupted or partial build leaves a newer `nvml_types.rs` only. Staging overwrites `types.rs` and keeps old `api.rs`/`shims.rs` from a different SDK. The committed prebuilt set is inconsistent, so the fallback build fails or binds mismatched layouts. | Require the full {types, api, shims, inventory} set per out dir. Stage into a temp dir and swap atomically (rename), removing files that are no longer produced. |
| X-6 | P1 | CONFIRMED | `xtask/src/prebuilt.rs:150`, `xtask/src/xtask.rs:178, 229`, `xtask/src/py_stubs.rs:129-134`, `stage_dlls.rs:364` | Committed artifacts (prebuilt bindings, COVERAGE.md, inventory evidence, `.pyi`, docs) and staged DLLs are written in place with `fs::copy`/`fs::write`, not temp + rename. | Ctrl-C or a crash mid-copy leaves a truncated generated `.rs` or `.dll`. stage-dlls' size+mtime skip would recopy the DLL, but nothing heals the prebuilt `.rs`. | Write `name.tmp` in the same directory, `sync_all`, then rename (the repo's atomic I/O rule 8). |
| X-7 | P2 | CONFIRMED | `xtask/src/xtask.rs:360-361`, `:375` | `rows()` re-reads the entire `crates/` tree (including the huge generated `prebuilt/*.rs`) once per library. "Wrapped" is decided by `contains_word` over the wrapper sources, which also matches doc comments. Every wrapper mentions its C symbol in docs, so an entry point that is documented but never called counts as wrapped. | Coverage matrix overstates wrapping. The run is O(libs x tree). | Read once, strip comments (as `optix_cov::strip_comments` does), and match call syntax `sys::name(`. |
| X-8 | P2 | CONFIRMED | `xtask/src/xtask.rs:190-191` | `collect` scans only `target/<profile>/build`, not `target/<triple>/<profile>/build` or `CARGO_TARGET_DIR`. `prebuilt.rs:87-95` handles both, so the two scanners have diverged. | Inventories from `--target` or custom target-dir builds are never collected. | Share one build-dir scanner (`prebuilt::resolve_target_dir` plus the triple scan). |
| X-9 | P2 | CONFIRMED | `xtask/src/py_stubs.rs:2724` | The `unimplemented!()` is inside a raw-string test fixture (`r#"..."#` starting at `:2713`) that is parsed by `syn` and never compiled or executed. **Not a defect.** | — | None. Optionally replace it with `todo_fixture()` to stop grep-based audits flagging it. |
| X-10 | P2 | CONFIRMED | `xtask/src/py_stubs.rs:2083-2091`, `:1972-1980` | Methods render in Rust source order. If a `#[setter]` comes before its `#[getter]`, the stub emits `@prop.setter` before `@property def prop`. That is an invalid `.pyi` (NameError for type checkers). | A source reorder breaks stubs while `py-stubs check` still passes (it is self-consistent). | Sort getters before setters per property, or error when a setter has no preceding getter. |
| X-11 | P2 | CONFIRMED | `xtask/src/py_stubs.rs:2954-2971` | `workspace_nv_py_builds_deterministic_model` hard-codes `25` submodules (brittle), and its "deterministic" assertion renders the same in-memory model twice, which is vacuous: it does not prove output is independent of map ordering. | — | Compare against `check()` output, or drop the vacuous assertions. |
| X-12 | P2 | CONFIRMED | `xtask/src/optix_cov.rs:48-55` | `strip_comments` cuts at the first `//` even inside string literals, and ignores `/* */` block comments, so block-comment prose counts as use. | A field mentioned only in a block comment passes the gate. | Use a tokenizer (`syn`/`proc_macro2` lexing) instead of a line split. |
| B-1 | P1 | CONFIRMED | `bootstrap.py:731-734`, `:766-769`, `:555-580` | `deps dl` extracts directly into the final `.sdk/<sdk>` directory. Detection only checks marker files. | An interrupted extract (Ctrl-C, disk full) leaves the markers (`include/optix.h`) but missing files. The next run reports "already present; skipping" for good, and builds fail with confusing errors. | Extract to `.sdk/<sdk>.partial` and rename on success (also download to `*.part`). |
| B-2 | P1 | CONFIRMED | `bootstrap.py:305, 370, 394, 406` (+ `release.yml:132, 138, 144`) | Public SDKs come from unpinned branch archives (`optix-dev/master`, `nvapi/main`, `DLSS/main`, `nccl/master`, LunarG `latest`) with no checksum. | Two machines or CI runs get different headers. The prebuilt bindings and coverage sha256 evidence change without a code change. A supply-chain swap goes undetected. | Pin tags/commits plus sha256 in one table that `release.yml` also reads. |
| B-3 | P2 | CONFIRMED | `bootstrap.py:1150-1170` vs `AGENTS.md` ("Run `cargo clippy --workspace --all-targets` and `cargo test --workspace`") vs `ci.yml:45-48` | Three different gates. bootstrap `check` runs clippy `--release` (a different cfg/profile from AGENTS and CI) and no `cargo test` or `coverage check`. CI clippy/test covers only `nv-loader`, `nv-bindgen`, `nv-bindgen-fixture`, `xtask`. None of the six crates in this slice is compiled in CI, so the non-SDK `cfg(not(has_ngx_sdk))` stubs and the nvapi non-Windows path are never compiled. | A regression in ngx/nvml/cupti/nvrtc/nccl, or in a Linux-only cfg, merges green. | One gate definition (xtask `ci` subcommand) used by CI, bootstrap and AGENTS. At minimum, `cargo clippy --workspace --all-targets` on both OSes with `NV_RS_USE_PREBUILT=1`. |
| B-4 | P2 | CONFIRMED | `deny.toml:25-35`; `.github/workflows/*` | `deny.toml` exists but nothing runs `cargo deny`, so it is dead config. `allow-git` lists `cuda-rust_b.git` (unused per `Cargo.lock`) and omits the SSH sources actually used (`appbus-rs`, `csg-rs`, `exr-rs`; `Cargo.toml:20-28`). Exceptions such as `wildcards = "allow"` and `multiple-versions = "warn"` carry no justification. | — | Add a `cargo deny check` CI step, fix `allow-git`, and comment each relaxation. |
| B-5 | P2 | CONFIRMED | `rust-toolchain.toml:4`, `bootstrap.py:37`, `ci.yml:29`, `release.yml:35` | The toolchain is set in three places. CI and release install `@stable`, but `rust-toolchain.toml` pins `1.99.1`, so rustup silently installs a second toolchain and the action's components don't apply to it. `DEFAULT_TOOLCHAIN` in bootstrap is display-only and duplicated. | Wasted CI time, and a confusing toolchain if the pin changes. | Use `dtolnay/rust-toolchain@master` with `toolchain: 1.99.1` read from the file, and drop the bootstrap constant. |
| B-6 | P2 | CONFIRMED | `bootstrap.py:292-293`; `release.yml:66-99` | The CUDA version is hard-coded twice (13.3.1 URL plus `13.3` component names). The bootstrap manual note says `redistrib_13.5.0.json`, which conflicts with its own 13.3.1 URL. The linux sonames (`.so.13`/`.so.12`) are also duplicated in stage-dlls. | Version bump drift. | Keep a single version table. |
| B-7 | P2 | PLAUSIBLE | `bootstrap.py:1136-1142`, `:1212-1219` | `py-stubs`, `xtask` and `fmt` run without `_cargo_env()`, so `.sdk` roots and `NV_SDK_DIRS` are not exported. `python bootstrap.py x coverage optix` locates OptiX through vcv-rs env/defaults only, not `.sdk/optix`, unlike `build`. | The coverage gate reports "OptiX SDK not found" on a machine where `b` works. | Pass `env=_cargo_env()` to `cmd_xtask`. |
| B-8 | P2 | CONFIRMED | `bootstrap.py:1206-1208` | `clean --sdk` runs `rmtree(.sdk)` with no confirmation. That also deletes manually provisioned auth-gated SDKs (Optical Flow, Video Codec) that cannot be re-downloaded automatically. | Accidental loss of manual SDKs. | Delete only `_downloads` and auto-downloaded subdirs, or require `--yes`. |
| B-9 | P2 | CONFIRMED | `Cargo.toml:6` | `exclude = ["spikes", ...]`, but there is no `spikes/` directory in the repo. The entry is stale. | — | Remove it. |

## 2) Files inspected

Read fully unless noted.

| File | Lines | Read |
|---|---:|---|
| AGENTS.md (root) | — | fully (in context) |
| crates/AGENTS.md | 43 | fully |
| crates/ngx/src/ngx.rs | 382 | fully |
| crates/ngx/src/params.rs | 525 | fully |
| crates/ngx/src/types.rs | 500 | fully |
| crates/ngx/src/vulkan.rs | 1217 | fully |
| crates/ngx/src/error.rs | 127 | fully |
| crates/ngx/build.rs | 9 | fully |
| crates/ngx/Cargo.toml | 20 | fully |
| crates/ngx/tests/gpu.rs | 28 | fully |
| crates/nvapi/src/nvapi.rs | 711 | fully |
| crates/nvapi/src/win.rs | 676 | fully |
| crates/nvapi/src/error.rs | 171 | fully |
| crates/nvapi/src/types.rs | 281 | fully |
| crates/nvapi/Cargo.toml | 19 | fully |
| crates/nvml/src/nvml.rs | 269 | fully |
| crates/nvml/src/session.rs | 170 | fully |
| crates/nvml/src/device.rs | 622 | fully |
| crates/nvml/src/error.rs | 164 | fully |
| crates/nvml/src/types.rs | 442 | fully |
| crates/nvml/Cargo.toml | 19 | fully |
| crates/cupti/src/cupti.rs | 142 | fully |
| crates/cupti/src/activity.rs | 456 | fully |
| crates/cupti/src/subscriber.rs | 186 | fully |
| crates/cupti/tests/gpu.rs | 63 | fully |
| crates/cupti/Cargo.toml | 19 | fully |
| crates/nvrtc/src/nvrtc.rs | 179 | fully |
| crates/nvrtc/src/program.rs | 272 | fully |
| crates/nvrtc/src/options.rs | 152 | fully |
| crates/nvrtc/tests/gpu.rs | 162 | fully |
| crates/nvrtc/Cargo.toml | 22 | fully |
| crates/nccl/src/nccl.rs | 448 | fully |
| crates/nccl/src/comm.rs | 993 | fully |
| crates/nccl/Cargo.toml | 22 | fully |
| xtask/src/xtask.rs | 598 | fully |
| xtask/src/prebuilt.rs | 457 | fully |
| xtask/src/stage_dlls.rs | 537 | fully |
| xtask/src/optix_cov.rs | 724 | fully |
| xtask/src/py_stubs.rs | 2991 | fully (4 pages) |
| xtask/Cargo.toml | 25 | fully |
| bootstrap.py | 1353 | fully (2 pages) |
| Cargo.toml (root) | 54 | fully |
| deny.toml | 35 | fully |
| .cargo/config.toml | 5 | fully |
| rust-toolchain.toml | 6 | fully |
| .github/workflows/ci.yml | 54 | fully |
| .github/workflows/pages.yml | 50 | fully |
| .github/workflows/release.yml | 217 | fully |

Reference-only (excerpts, to verify a signature, constant or behaviour):
- `.sdk/ngx/include/{nvsdk_ngx.h, nvsdk_ngx_defs.h, nvsdk_ngx_defs_dlssd.h, nvsdk_ngx_vk.h, nvsdk_ngx_helpers.h, nvsdk_ngx_helpers_dlssd_vk.h}`
- `.sdk/nvapi/nvapi.h`
- `crates/cupti-sys/prebuilt/*/cupti_types.rs`, `crates/nvapi-sys/prebuilt/.../nvapi_types.rs` (grep only)
- `crates/nvml-sys/src/nvml_sys.rs`, `crates/nvml-sys/build.rs:18-21`
- `crates/*-sys/src/*.rs` (DllSpec blocks)
- `crates/ngx-sys/build.rs` (grep), `crates/ngx-sys/src/ngx_sys.rs:54`
- `crates/nv-loader/src/nv_loader.rs:300-472` (candidate search)
- cuda-core checkout `348895e` (`cudarc_shim.rs:32` primary ctx, `runtime.rs:86` `cu_device`)

## 3) Files changed

None. Only this report was written, to the scratchpad. No cargo, build, test or state-changing git command was run.

## 4) Static/invariant verification performed

- NGX result success predicate: compared `error.rs` against `nvsdk_ngx_defs.h:183-184` (the mask macro) -> NGX-4.
- NGX error code table `0xBAD00000..0xBAD00012`: matches `nvsdk_ngx_defs.h` one to one.
- NGX keys/enums: the RR keys (`DLSS.Input.DiffuseAlbedo`, `GBuffer.*`, `DLSS.Use.HW.Depth`, `DLSS.Roughness.Mode`, `DLSS.Denoise.Mode`), the RR presets D/E/F = 4/5/6, the `DlssFeatureFlags` bits and `RayReconstruction.Hint.Render.Preset.*` all verified against `nvsdk_ngx_defs*.h`.
- NGX optimal settings: wrapper flow verified against `NGX_DLSS_GET_OPTIMAL_SETTINGS` (helpers.h:65-105): same key set, same `RTXValue=0`, `OutWidth`/`OutHeight` readback.
- NGX RR create/eval: wrapper key set verified against `NGX_VULKAN_CREATE_DLSSD_EXT1` / `NGX_VULKAN_EVALUATE_DLSSD_EXT`, including the MV-scale and exposure `0 -> 1` defaults.
- NGX thread-safety and deprecation notes read from `nvsdk_ngx_vk.h:16-17, 85-96` -> NGX-1, NGX-11.
- NVAPI: all 11 versioned-struct versions verified against the `nvapi.h` typedef/`_VER` macros. Handle-array sizes verified against the prototypes (`NvAPI_EnumTCCPhysicalGPUs[NVAPI_MAX_PHYSICAL_GPUS]`, `GSync[NVAPI_MAX_GSYNC_DEVICES=4]`, ...).
- NVML: `NVML_STRUCT_VERSION` encoding (`<<24`) and the FUNCTION_NOT_FOUND missing-symbol policy (`nvml-sys/build.rs:21`) checked. The retry loop was hand-traced for count 0, grow and shrink.
- CUPTI: confirmed the prebuilt binds `CUPTI_API_VERSION=130301` and `KernelLatest=Kernel12`, and that `cupti-sys` accepts `libcupti.so.12` -> CUPTI-3.
- NCCL: confirmed cuda-core contexts are primary contexts (`cuDevicePrimaryCtxRetain`, compatible with NCCL's runtime-API use) and that `cu_device()` returns `CUdevice` -> NCCL-6. `GroupGuard` auto-trait derivation reasoned from its field set -> NCCL-1.
- stage-dlls vs loader: traced `linux_candidates_with` and confirmed it has no glob expansion and no `lib/Linux_x86_64/rel` -> X-2. Diffed every `DllSpec` against the sys crates -> X-3.
- prebuilt staging: traced `scan_build_dir`/`collect_binding_files` and confirmed that a one-file out dir is accepted and that stale files persist -> X-5.
- `let _ =` census: 13 in params.rs, 13 in vulkan.rs, 9 in comm.rs, each classified individually above.
- py_stubs `:2724`: confirmed it is inside the raw string literal opened at `:2713` -> X-9 (not a defect).
- CI vs AGENTS.md vs bootstrap gate commands diffed -> B-3. `deny.toml` `allow-git` diffed against `Cargo.toml` git deps and the `Cargo.lock` sources -> B-4.

## 5) Risks / open seam questions for other slices

1. **nv-loader** (`nv_loader.rs:397-411`): the Windows wildcard `nvrtc64_*` also matches `nvrtc64_130_0.alt.dll`. It ties on the version key with `nvrtc64_130_0.dll`, and the name tiebreak sorts `.alt.dll` first, so the loader (and stage-dlls) may pick the `.alt` variant. Is that intended? The same question applies to any `*.alt.dll`.
2. **nv-loader**: `linux_candidates_with` does not expand `*` in `spec.linux` (`libnvidia-ngx-dlss.so*`), and its Sdk search lacks the `lib/Linux_x86_64/rel` dir that the Windows branch has (`:538-539`). This affects the runtime loader, not only stage-dlls.
3. **nv-demo**: does it drop `ngx::FeatureHandle` / `NgxVulkan` only after `vkDeviceWaitIdle` or a fence wait (NGX-2)? Does it ever share `NgxVulkan` across the UI and render worker threads (NGX-1)? It uses an `appbus` worker thread.
4. **nv-demo**: is CUPTI used by `GpuPassProfiler` or any other component? If so, `ActivityProfiler`'s global state (CUPTI-1/2) and callback re-registration conflict with it.
5. **ngx-sys build.rs**: confirm that the static `nvsdk_ngx_d.lib` link plus the `SDK_BIN` search path finds `nvngx_dlssd.dll` at runtime when nv-demo is installed via `cargo install` (outside `target/`). This is the same gap as X-2.
6. **python/nv-py** (excluded workspace): the `ngx`/`cupti`/`nccl` Python bindings inherit NGX-1 (GIL release + `Send` `NgxVulkan`), CUPTI-1 (global records) and NCCL-1. Check whether any PyO3 class releases the GIL around NGX calls.
7. **cuda-core**: `Context::record_err` is the only teardown sink for cupti/nccl, and every code is mapped to `CUDA_ERROR_UNKNOWN` (CUPTI-5, NCCL-5). Should cuda-core accept a foreign error payload?
8. **CI coverage**: no job compiles the `cfg(not(has_ngx_sdk))` stubs, the non-Windows nvapi path or the non-Linux nccl path (B-3). The other slices should check whether their OS-gated branches have the same gap.
9. **nv-bindgen**: hand-coded NVAPI struct versions (NVAPI-3) and CUPTI record versions (CUPTI-3) could be generated or asserted by nv-bindgen. Is there an existing mechanism for evaluating `MAKE_NVAPI_VERSION`-style macros?
