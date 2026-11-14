# M2 seam matrix

Updated: 2026-11-13. All checks below are pending. Canonical vcv-rs Windows/Linux discovery is the current priority; D01/D02 implementation is not approved.
See the [domain index](MASTERMIND_INDEX.md), [work queue](../../TODO.md) and [plan](../../PLAN.md).

Root replaces the pending producer/consumer locations with exact `file:symbol:line` evidence
after domain manifests exist. No row is a verified contract yet.

| Seam | Producer | Consumer | Contract to verify | Status |
|---|---|---|---|---|
| S01 | vcv-rs nvsdk.rs:detect(kind,target)/SdkInfo | nv-bindgen.rs:sdk_paths/generate and new sys specifications | Single snapshot for headers/runtime/link paths; explicit target and representable paths; discovery aliases trigger regeneration | adapter written; vcv-rs integration/review/push/gates pending |
| S02 | Linux CUDA/NCCL/cuFile discovery | NCCL and cuFile build specifications | Supported target selects real installed headers and sonames; unsupported targets report explicit behavior | pending |
| S03 | cuda-bindings | All three new sys crates | CUDA context, stream, pointer and related types retain type identity across library calls | pending |
| S04 | SDK headers and nv-bindgen | Generated FFI declarations and shim entry points | ABI, calling convention, enum representation, signatures, struct size/alignment and field offsets match the reference headers | pending |
| S05 | nv-loader | Generated API holders and shims | Loaded libraries outlive every function pointer; initialization is shared and missing libraries/symbols produce the documented errors without panic | pending |
| S06 | nv-bindgen.rs:wrapper_plan/shims; nv-loader.rs:CallError | Generated entry points and actual fixture callers | Preserve native configured status/pointer ABI; scalar/void calls without native sentinel use checked Rust Result<T>/Result<()> | repair written; review/execution pending |
| S07 | Library inventory collection | Coverage generation and check | Header-derived symbol names, SDK/target identity and evidence records agree; claims separate bound, wrapped and GPU-tested | pending |
| S08 | Reference fixtures and negative cases | Generator/loader regression tests | Test declarations exercise actual generated paths; at least one seeded invariant mutant makes its gate fail | pending |
| S09 | Root build/test commands and platform inventory | Domain completion and handoff | Every involved repo pushed to main before any build/check/test; one build at a time, exact exit codes/logs and GPU limits | pending |

## Known repair gates

- [ ] Review and execute the written NCCL partial-init/abort-before-wait repairs, including six cleanup sequences and four initialization states.
- [ ] Review and execute checked void calls for loaded, missing-library, missing-symbol and query-interface-null paths.
- [ ] Complete vcv-rs CUDA runtime-proof, selected vcpkg-triplet and native CMD list-snapshot repairs before accepting S01.
- [ ] Fresh independent Inquisitors review both domains and affected existing recipes.
- [ ] Final graph refresh follows source freeze; the reported incremental reindex alone is not the final gate.

## Cross-domain checks

- [ ] Trace every new public entry point from its crate/build specification through generation and runtime loading.
- [ ] Verify CUDA and inter-library handle identity at each producer/consumer boundary.
- [ ] Verify all required CPU/GPU FFI layouts from their authoritative headers.
- [ ] Verify pointer ownership, library lifetime, callback lifetime and thread-safety assumptions.
- [ ] Verify no steady-state allocation is introduced at runtime call boundaries.
- [ ] Verify fallible calls retain their errors and explicit requests cannot silently downgrade.
- [ ] Record where coordinate, matrix, unit, CPU SIMD and wgpu row-pitch contracts do not apply to raw sys bindings; check them where shared payloads require them.
- [ ] Record domain review evidence and root gate evidence before marking any row verified.

## Evidence format

For each row, record exact producer and consumer locations, domain IDs, payload type and size
where applicable, ownership/lifetime invariant, the live caller, and the independent test or
reference comparison. Use `not applicable` with a reason when a contract has no payload/layout
dimension. Record unavailable environments as unavailable, not verified.
