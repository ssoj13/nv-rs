# nv-rs — work queue

## Transfer checkpoint — 2026-11-13 (Codex)

**Status: paused by owner for an environment change.** All task agents have stopped/completed; do not resume implementation automatically. The owner explicitly requested updating progress and pushing the unfinished source checkpoint to main.

- vcv-rs checkpoint is published on main at `a8cab6688c1ef01f1067ea3d150ad7283e1367cb`.
- Both host and device Cargo.lock files now resolve that exact vcv-rs revision. Metadata-only commands succeeded: `cargo +1.98.0 update -p vcv-rs` and `cargo +1.99.1 update --manifest-path device/Cargo.toml -p vcv-rs`. They added object 0.40.0, removed unused optional CLI dependencies, and recorded the three written host sys crates; no compile/test result follows.
- nv-rs D01/D02 source remains **unreviewed and untested**. M2 is open. Historical M1/M3/M4/M5 and prior 18-library results below do not validate this checkpoint.
- vcv-rs D11 reached a 225-ID source-only static closure. Added ID226 (real COFF/ELF archive test) has not been formatted after insertion, independently reviewed or executed.
- vcv-rs D12 has 125 IDs; the full review is incomplete. **Known defect remains:** fmt_ps accepts oversized Windows/.NET environment keys/values/joined lists and may fail after earlier assignments. D13's 110-ID independent review remains pending.
- No Windows/Linux build, check, test, shell round trip, seeded mutant or GPU gate ran for this wave. Platform gates are 0/2.
- Durable transfer records, exact continuation, source hashes and unexecuted validation scripts are in sibling [vcv-rs HANDOFF.md](../vcv-rs/HANDOFF.md) and its `docs/mastermind/evidence/2026-11-13-transfer/`. SDK binaries and proprietary headers were excluded.
- Recovery documentation-agent OAuth failure produced no result; root wrote this checkpoint. No new agent or chat-wait loop was started after stop.

After the owner resumes: repair/review the vcv-rs length boundary, review ID226 and complete D12/D13/seam reviews; commit/push any new edits before every build/check/test. Root monitors CPU/RAM/GPU/other builds and serializes both platform gates under the existing build-lock override. Then verify nv-rs consumers and finish M2 NCCL/nvCOMP/cuFile reviews and supported-platform gates. Preserve separate NCCL multi-GPU and native-Linux cuFile/storage requirements.


## Historical checkpoint (superseded above)


Updated: 2026-11-13. Current priority: vcv-rs Windows/Linux support; remaining M2 stays open.
Sources: [plan](PLAN.md), [handoff](HANDOFF.md), [domain index](docs/mastermind/MASTERMIND_INDEX.md),
[seam checks](docs/mastermind/SEAM_MATRIX.md).

## Current step

Status: in_progress — complete the canonical vcv-rs Windows/Linux discovery and environment contracts, then resume M2 repairs and verification.
Every involved repository must be committed and pushed to main before any build, check or test.
Only the root orchestrator runs serialized gates. Workers edit disjoint files and never build concurrently.

## Recovery snapshot (2026-11-13, 22:35–22:37 UTC)

Recovery confirmed nv-rs `main` at `8b7e13c` and vcv-rs `main` at `6eb28c0`, both with
uncommitted changes. Before recovery documentation edits, nv-rs had 23 modified tracked files
and nine untracked source files across the three new sys crates. The previous handoff
(19:40:44 UTC) predates source edits through 21:58:52 UTC; its next steps have been refreshed.
One nv-rs worktree and no `cargo`/`rustc` processes were observed. No build, check or test ran
during recovery, and inspected logs established no execution gate for this wave.
The source graph needs an unstaged reindex before use. Queried memory and OMH scopes supplied
no additional recovery records.

Immediate continuation: repair/re-review the documented vcv-rs D11 findings: OptiX fixture ownership, NGX runtime
names/package paths, nested Windows NGX import libraries and Windows CUDA .LIB case handling;
obtain D11/D12/D13 reviews, then follow the publication and serialized verification rules below.

## Current implementation checkpoint

This 2026-11-13 M2 source checkpoint started from baseline `8b7e13c`. Publication, compiler/test
gates and fresh independent Inquisitor verdicts remain pending at this checkpoint.
The immediate priority is [vcv-rs Windows/Linux support](../vcv-rs/TODO.md).

- D01: nine files for nvcomp-sys, nccl-sys and cufile-sys are written; the original 22 declarations plus six NCCL cleanup/helper declarations are inventoried in the [domain ledger](docs/mastermind/domains/D01_m2_sys.md).
- D02: shared return policies, boxed call errors, mixed-return fixtures and Linux SDK load hints are written; 14 existing recipes are migrated. Review and all execution gates remain pending.
- Written repairs: NCCL partial-init cleanup and abort-before-wait ordering have six sequence/four initialization-state fixture cases; void native calls now use checked Rust `Result<()>`. Fresh review and every execution gate remain pending.
- Root integration: nv-bindgen takes one target-aware vcv-rs SDK snapshot for header/runtime/link paths, rejects unrepresentable Cargo/libclang paths (including explicit bindings include/system inputs), and tracks discovery-owner aliases as rebuild inputs. Negative fixture source is written; the new vcv-rs revision has not been pushed or consumed by a checked build.
- vcv-rs D11 SDK, D12 environment/formatter/native-shell harness and D13 CLI source are frozen after bounded readback. CUDA-runtime/vcpkg-triplet and CMD list-snapshot repairs are written. Fresh reviews, platform gates and source pushes remain open.
- Execution policy: Owner instruction on 2026-11-13 overrides the absent build-lock requirement for this task: root monitors CPU/RAM/GPU load and other build processes, then serializes Cargo/check/test runs. No replacement lock or further permission is required.
- Root's independent NCCL reference probe reported runtime version `23102` and resolved the inspected ELF exports on Ubuntu 26.04 / glibc 2.43. This is SDK/runtime discovery evidence, not a test of the new nv-rs crates.
- NVIDIA manylinux wheel SHA-256: `0bcaf0308854cb55fcc35af72e2c83143f3b71e65a4e865e2c586b1cdcdb5ae0`. Ubuntu 24.04 debs were unused and have been deleted.

## Recovered baseline

- [x] Read the full plan and handoff; preserve their decisions and outstanding work.
- [ ] Verify the M2 additions against current code and SDK headers. Previous milestone results below are historical evidence, not checks rerun in this wave.

The handoff records S1a, S1b, S2 and S3 passing, M1/M3/M4/M5 complete, and 18 sys libraries tested.
M5 records 55 host GPU tests, 15 device cases and the OptiX coverage gate. Do not repeat these as new results.

## M2 — remaining sys crates

- [ ] Record the actual dependency revisions and active cuda-rust_b checkout before changing dependencies.
- [ ] Discover nvCOMP headers/runtime through vcv-rs; record SDK version, paths and installation needs.
- [ ] Inspect WSL2/native Linux availability, CUDA, NCCL and cuFile headers/runtime; distinguish header-only, compile and GPU validation.
- [ ] Map shared nv-bindgen/nv-loader contracts and call-graph impact before editing symbols.
- [ ] Complete vcv-rs Windows/Linux support, including explicit CUDA/OptiX root precedence, and consume its pushed main revision.
- [ ] Repair failed NCCL initialization cleanup and review abort-before-stream-sync ordering.
- [ ] Replace silent void-return fallback with checked Result<()> error propagation; cover loaded, missing-library and missing-symbol paths.
- [ ] Assign disjoint domains and micro-units; write numbered symbol manifests before worker dispatch.
- [ ] Implement nvcomp-sys from installed SDK headers through the shared generator and loader.
- [ ] Implement nccl-sys against the supported Linux API, with explicit platform behavior.
- [ ] Implement cufile-sys against the supported Linux API, with explicit platform behavior.
- [ ] Reuse CUDA types from cuda-bindings; verify cross-library handle identity rather than duplicating declarations.
- [ ] Add fixture and negative tests for every newly encountered generator/loader behavior and missing-library/symbol path.
- [ ] Collect SDK symbol inventories and coverage evidence for each new library; regenerate and check coverage.
- [ ] Validate each supported platform at T1; record unavailable SDK/platform gates with exact reasons.
- [ ] Run NCCL GPU validation where available; do not claim multi-GPU coverage from a single device.
- [ ] Run cuFile GPU/storage validation on native Linux with GPUDirect Storage; WSL2 does not establish this gate.
- [ ] Obtain price and owner approval before any RunPod rental (D12).
- [ ] Run fresh independent seven-question Inquisitor reviews per domain; repair findings and repeat review.
- [ ] Verify producer/consumer contracts in the seam matrix, including FFI layouts and library lifetimes.
- [ ] Prove at least one relevant invariant test fails under a seeded mutant; restore the original implementation and verify the normal gate.
- [ ] Finish edits and commit/push every involved repository to main before any build/check/test, as the user now requires; use DCO sign-off and no agent co-author footer.
- [ ] Run narrow package checks and targeted tests once the complete change is ready.
- [ ] Root checks CPU/RAM/GPU load and other build processes, then runs the final check, workspace clippy and full tests serially; capture exit codes and logs.
- [ ] Run required GPU gates one process at a time with the specified VRAM budget; report skips and unavailable hardware explicitly.
- [ ] Re-index changed code, verify diff scope, and update handoff, decisions and traps from evidence.
- [ ] Mark M2 complete only after every supported library has the required inventory and T1 evidence; preserve external validation blockers.

## M6 — CUDA/wgpu adapter

- [ ] Implement nv-wgpu on cuda-ext using gpu-info-rs as the shared device owner.
- [ ] Validate Vulkan Windows, DX12 and Vulkan Linux memory import and synchronization independently.
- [ ] Verify device UUID/LUID matching, allocation sizes, lifetimes and supported texture formats.
- [ ] Pass T2 per backend; reject unsupported explicit requests without silent fallback.

## M7 — toolkit safe wrappers

- [ ] Implement the planned toolkit wrappers, including the cuRAND safe layer over existing cuda-bindings.
- [ ] Apply shared ownership, stream, byte-budget, error and scratch-buffer contracts.
- [ ] Pass T3 numerical/reference tests per library.

## M8 — video and optical flow

- [ ] Implement nvenc, nvdec and nvofa safe layers.
- [ ] Verify encode/decode round trips against the planned PSNR tolerance and required hardware.
- [ ] Bind NVENC GUIDs and the NVOFA D3D11/D3D12/Vulkan variants from their authoritative SDK declarations.

## M9 — separately shipped SDK wrappers

- [ ] Implement cudnn, nvcomp, nccl and cufile safe wrappers; pass T1 and available T2 gates.
- [ ] Resolve D13 owner confirmation before implementing the full NGX ProjectID wrapper.
- [ ] Implement NGX, including its D3D11 helpers, after that decision; preserve the D15 static-link contract.
- [ ] Obtain separate approval for any required native-Linux/multi-GPU rental under D12.

## M10 — render-rs OptiX backend

- [ ] Implement pt-optix using optix, optix-device and M6 interop beside pt-cuda.
- [ ] Verify the same scenes against pt-cuda within an explicitly recorded tolerance.

## M11 — pitch package

- [ ] Complete user/developer documentation, generated coverage and the Rust OptiX demo.
- [ ] Run the demo from a clean checkout and retain reproducible build/run instructions.

## Open decisions and cross-cutting work

- [ ] Reconcile the handoff's pending #[device(export_name)] item with its M5 completion claim and the actual fork API.
- [ ] Reconcile D6 with committed cuda-bindings/generated files in the fork.
- [ ] Unify toolkit handle types shared by cuSOLVER, cuBLAS and cuSPARSE.
- [ ] Resolve stale planning/status/install text against current code and installed SDKs.
- [x] Correct the handoff's upstream-offer suggestion to preserve D5; no upstream PR is authorized by the current plan.
- [ ] Maintain local AGENTS.md invariants and traps after major verified waves.
- [ ] Keep hot paths allocation-free, preserve FFI layouts and document every unsafe boundary; reject stubs, swallowed errors and suppression shims.
