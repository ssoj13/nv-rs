# Domain D07 (M7d) — Driver & System Management (`nvml`, `nvapi`)

## Scope
- `crates/nvml/**`: safe NVML wrapper over `nvml-sys`
- `crates/nvapi/**`: safe Windows NVAPI wrapper over `nvapi-sys`

## Implemented Files
### `crates/nvml`
- [Cargo.toml](file:///C:/projects/projects.rust.cg/cglibs/nv-rs/crates/nvml/Cargo.toml) (`[lib] path = "src/nvml.rs"`, `[lints] workspace = true`)
- [src/nvml.rs](file:///C:/projects/projects.rust.cg/cglibs/nv-rs/crates/nvml/src/nvml.rs) (crate root, re-exports, 7 T0 unit tests, 1 `#[ignore]` T2 RTX 3080 Ti telemetry test)
- [src/error.rs](file:///C:/projects/projects.rust.cg/cglibs/nv-rs/crates/nvml/src/error.rs) (`Status(nvmlReturn_t)` `Copy` newtype, `nvmlErrorString` lookup, `thiserror::Error`)
- [src/session.rs](file:///C:/projects/projects.rust.cg/cglibs/nv-rs/crates/nvml/src/session.rs) (`Nvml` with `Arc<NvmlSession>` so `nvmlShutdown` runs once on final drop; index/UUID/PCI bus ID device lookup)
- [src/device.rs](file:///C:/projects/projects.rust.cg/cglibs/nv-rs/crates/nvml/src/device.rs) (`Device` pinning `Arc<NvmlSession>`, identity, `nvmlDeviceGetMemoryInfo_v2` with v1 fallback, BAR1, utilization, encoder/decoder, temp/thresholds, fan, power/energy, clocks, ECC, bounded two-phase process queries, PCIe, NVLink)
- [src/types.rs](file:///C:/projects/projects.rust.cg/cglibs/nv-rs/crates/nvml/src/types.rs) (`PciBusId`, `PciInfo`, `MemoryInfo`, `Bar1MemoryInfo`, `Utilization`, `CodecUtilization`, `ProcessInfo`, `Architecture`, `ClockDomain`, `TemperatureThreshold`, `PcieCounter`, `NvLinkCapability`, `NvLinkErrorCounter`, stack C-string codecs)

### `crates/nvapi`
- [Cargo.toml](file:///C:/projects/projects.rust.cg/cglibs/nv-rs/crates/nvapi/Cargo.toml) (`[lib] path = "src/nvapi.rs"`, `[lints] workspace = true`)
- [src/nvapi.rs](file:///C:/projects/projects.rust.cg/cglibs/nv-rs/crates/nvapi/src/nvapi.rs) (cross-platform API, `UnsupportedPlatform` on non-Windows, 5 T0 unit tests, 1 `#[ignore]` T2 RTX 3080 Ti test)
- [src/error.rs](file:///C:/projects/projects.rust.cg/cglibs/nv-rs/crates/nvapi/src/error.rs) (`Status(i32)` `Copy` newtype, Windows `NvAPI_GetErrorMessage` formatting, `thiserror::Error`)
- [src/types.rs](file:///C:/projects/projects.rust.cg/cglibs/nv-rs/crates/nvapi/src/types.rs) (`make_version`, `decode_short_string`, `decode_utf16_z`, `encode_utf16_z`, `DriverVersion`, `PciIdentifiers`, `ArchInfo`, `GpuMemoryInfo`, `ThermalSensor`, `DynamicPstates`, `ClockFrequencies`, `PerfDecreaseReasons`, `HdrCapabilities`, `VrrInfo`, `GSyncCapabilities`, `GSyncStatus`, `DrsProfileInfo`)
- [src/win.rs](file:///C:/projects/projects.rust.cg/cglibs/nv-rs/crates/nvapi/src/win.rs) (`#[cfg(windows)]` `Arc<NvApiSession>` with `NvAPI_Unload` on drop, `DrsSession` with `NvAPI_DRS_DestroySession` on drop, physical/TCC/logical GPU, display HDR/VRR, G-Sync, and DRS profile queries)

## 7-Question Inquisitor Self-Audit
1. **Are there any unfinished stubs, `TODO`s, or placeholder branches?**
   - No. All declared methods in `nvml` and `nvapi` are fully implemented; non-Windows `nvapi` returns `Error::UnsupportedPlatform` by design.
2. **Are there any dead/uncalled functions, unused imports, or `#[allow(...)]` suppressions?**
   - Removed unused `unsupported_platform` helper in `nvapi/src/win.rs`. Only `#[cfg_attr(not(windows), allow(dead_code))]` on `nvapi::error::check` remains for non-Windows builds.
3. **Can any production or FFI path panic (`unwrap`, `expect`, unchecked indexing)?**
   - No `unwrap()` or `expect()` in production code. Array lengths returned by drivers (`count` in NVML processes, NVAPI GPU/thermal/display/DRS enumeration) are clamped with `.min(slice.len())`.
4. **Are all `unsafe` blocks justified with `// SAFETY:` and minimal?**
   - Yes, every `unsafe` block and `unsafe impl Send/Sync` carries a `// SAFETY:` invariant comment.
5. **Are hot-path allocations avoided?**
   - Fixed-size string/struct queries use stack arrays (`[c_char; N]`, `[u16; 2048]`, `MaybeUninit`). NVML process queries take `&mut ProcessScratch` + `&mut Vec<ProcessInfo>` in `*_into` to reuse both raw and decoded buffers with zero allocations in steady state.
6. **Are errors and status codes preserved without silent fallbacks?**
   - Both crates use `Copy` integer newtype `Status` codes; newer driver codes remain representable. Only `nvmlDeviceGetMemoryInfo_v2` -> v1 and `nvmlDeviceGetCurrentClocksEventReasons` -> `ThrottleReasons` fall back on `NVML_ERROR_FUNCTION_NOT_FOUND` (older driver ABI compatibility).
7. **Verification status:**
   - Static self-audit + independent Inquisitor audit (`#21`) repairs applied: (1) removed non-existent `sys::nvmlMemory_v2` from T0 test, (2) fixed T2 `mem.free + mem.used == mem.total` assertion, (3) added `ProcessScratch` to `*_into` for zero-alloc polling, (4) checked `hdr_capabilities`/`vrr_info` errors in NVAPI T2 test. Pending orchestrator serialized `cargo check`, `cargo clippy -- -D warnings`, T0 unit tests, and T2 RTX 3080 Ti GPU tests.
