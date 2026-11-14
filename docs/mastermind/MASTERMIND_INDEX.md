# M2 domain index

## Transfer checkpoint — 2026-11-13 (Codex)

**Status: paused by owner for an environment change.** All task agents have stopped/completed; do not resume implementation automatically. The owner explicitly requested updating progress and pushing the unfinished source checkpoint to main.

- vcv-rs checkpoint is published on main at `a8cab6688c1ef01f1067ea3d150ad7283e1367cb`.
- Both host and device Cargo.lock files now resolve that exact vcv-rs revision. Metadata-only commands succeeded: `cargo +1.98.0 update -p vcv-rs` and `cargo +1.99.1 update --manifest-path device/Cargo.toml -p vcv-rs`. They added object 0.40.0, removed unused optional CLI dependencies, and recorded the three written host sys crates; no compile/test result follows.
- nv-rs D01/D02 source remains **unreviewed and untested**. M2 is open. Historical M1/M3/M4/M5 and prior 18-library results below do not validate this checkpoint.
- vcv-rs D11 reached a 225-ID source-only static closure. Added ID226 (real COFF/ELF archive test) has not been formatted after insertion, independently reviewed or executed.
- vcv-rs D12 has 125 IDs; the full review is incomplete. **Known defect remains:** fmt_ps accepts oversized Windows/.NET environment keys/values/joined lists and may fail after earlier assignments. D13's 110-ID independent review remains pending.
- No Windows/Linux build, check, test, shell round trip, seeded mutant or GPU gate ran for this wave. Platform gates are 0/2.
- Durable transfer records, exact continuation, source hashes and unexecuted validation scripts are in sibling [vcv-rs HANDOFF.md](../../../vcv-rs/HANDOFF.md) and its `docs/mastermind/evidence/2026-11-13-transfer/`. SDK binaries and proprietary headers were excluded.
- Recovery documentation-agent OAuth failure produced no result; root wrote this checkpoint. No new agent or chat-wait loop was started after stop.

After the owner resumes: repair/review the vcv-rs length boundary, review ID226 and complete D12/D13/seam reviews; commit/push any new edits before every build/check/test. Root monitors CPU/RAM/GPU/other builds and serializes both platform gates under the existing build-lock override. Then verify nv-rs consumers and finish M2 NCCL/nvCOMP/cuFile reviews and supported-platform gates. Preserve separate NCCL multi-GPU and native-Linux cuFile/storage requirements.


## Historical checkpoint (superseded above)


Updated: 2026-11-13. Current priority: [vcv-rs Windows/Linux support](../../../vcv-rs/TODO.md). M2 implementation is written in part and remains unverified.
This ledger tracks the remaining sys-crate work from [PLAN.md](../../PLAN.md).
The complete queue is in [TODO.md](../../TODO.md); cross-domain checks are in the
[seam matrix](SEAM_MATRIX.md).

## Evidence and status

The [handoff](../../HANDOFF.md) records M1, M3, M4 and M5 complete and 18 sys libraries tested.
Those are recovered results. D01/D02 source is written but uncommitted and untested; no compiler, GPU, mutant or fresh independent-review gate for the new M2 work is verified by this document.
The 2026-11-13 source checkpoint started from nv-rs baseline `8b7e13c` and vcv-rs baseline `6eb28c0`; publication and execution gates were pending at this checkpoint.

Recovery at 22:35–22:37 UTC confirmed those local `main` baselines with uncommitted changes.
The nv-rs snapshot before recovery documentation edits contained 23 modified tracked files and
nine untracked source files in the three new sys crates. Its previous handoff (19:40:44 UTC)
predated later source and ledger edits; the [updated handoff](../../HANDOFF.md) now records the
continuation order. No build/check/test ran during recovery or new wave gate was established
from inspected logs. GitNexus's 20:12:48 UTC index predates source edits and requires an unstaged
reindex before source queries are trusted.

Use `pending`, `in_progress`, `review_required` and `verified` explicitly. Mark a domain verified
only after its symbol ledger, independent Inquisitor ratchet, seam checks and root build gates
carry evidence. An unavailable platform remains a named limitation.

## Domains

| Domain | Scope | Owner | Write boundary | Status |
|---|---|---|---|---|
| [D01](domains/D01_m2_sys.md) | nvCOMP, NCCL and cuFile sys crates; 28 handwritten declarations including NCCL repair helpers | D01 lane; bounded D02 cleanup repairs, root coordinates | Nine files listed in ledger | source written; review/execution pending |
| [D02](domains/D02_binding_infra.md) | Shared generator/loader, checked mixed/void returns, fixtures and 14 migrated recipes | D02 lane; root owns vcv-rs adapter integration | 21 files listed in ledger; bounded D01 test repairs recorded separately | source written; review/execution pending |

Domains must stay at or below 2,500 inspected implementation lines. Split larger domains.
Each worker micro-unit must stay at or below 600 lines or 5–10 functions. Before dispatch,
record each exact symbol and source range; require one verdict row per symbol.
Do not grant shared-file write ownership to more than one active domain.

## Root responsibilities

- Establish SDK versions and actual CUDA/fork revisions before assigning implementation.
- Preserve a single generator, loader, SDK-discovery owner and set of CUDA FFI types.
- Supply fixtures and reference oracles; distinguish generated bindings from handwritten wrappers.
- Serialize every check, clippy, test and GPU run after monitoring CPU/RAM/GPU load and other build processes, as the owner instructed on 2026-11-13. The absent build-lock requirement is overridden for this task; no replacement lock or further permission is required.
- Finish source edits and commit/push every involved repository to main before any build, check or test; use DCO sign-off.
- Run a relevant seeded mutant and verify the expected test fails before restoring the invariant.
- Collect fresh independent seven-question reviews and repair/review until no evidenced defects remain.
- Re-index edits and verify all [seams](SEAM_MATRIX.md) before finalizing the wave.
- Keep D5 in force: fork patches only; the corrected handoff does not authorize an upstream PR.
- Keep D13 pending until the owner confirms the full NGX ProjectID wrapper.
- Obtain the price and owner approval before a RunPod rental under D12.

## Required domain ledger

Each `domains/Dxx_name.md` must contain:

1. Exact allowed write files and inspected file ranges.
2. Micro-unit partition and numbered symbol manifest.
3. Symbol-by-symbol parity, errors, allocation, layout, concurrency and wiring evidence.
4. Every Inquisitor pass, defect, repair and final verdict against all seven questions.
5. Root gate requests, fixture/oracle requirements and unresolved platform constraints.
6. Cross-domain producer/consumer contracts for the seam matrix.

The current ledgers record source inventory and written repairs, not zero-defect approval.
vcv-rs D11 SDK, D12 environment/formatter/native-shell harness and D13 CLI source are frozen for
review. CUDA runtime-proof, selected vcpkg-triplet emission and native CMD list-snapshot repairs
are written; their independent review and execution remain open. M2 verification depends on the coherent vcv-rs change, its pushed
main revision, and separate Windows/Ubuntu gates. The vcv-rs work queue reports four open D11
findings (OptiX fixture ownership, NGX runtime names/paths, nested Windows NGX import libraries,
and Windows CUDA .LIB case handling); recovery did not independently reproduce them. The final
185-row audit/verdict is pending.

Root's nv-bindgen adapter uses one SDK snapshot for headers/runtime/link metadata, validates
`CARGO_CFG_TARGET_ARCH` and Cargo path representation, and tracks discovery aliases as rebuild
inputs. NCCL cleanup and checked void-return repairs are written with fixture source; none has
fresh independent review or execution evidence.

## Verification queue

- [ ] SDK and platform inventory recorded, with root-reported NCCL probe separated from nv-rs tests.
- [ ] vcv-rs Windows/Linux owner fix completed and its pushed revision consumed.
- [ ] Failed NCCL init cleanup and abort-before-sync ordering repaired/reviewed.
- [ ] Void-return loader/symbol failures propagate through checked Result<()>.
- [ ] Exact domain ownership and symbol manifests recorded.
- [ ] nvCOMP implementation and negative/fixture tests reviewed.
- [ ] NCCL implementation and negative/fixture tests reviewed.
- [ ] cuFile implementation and negative/fixture tests reviewed.
- [ ] Shared changes reviewed against existing sys crates.
- [ ] Coverage inventories generated and checked.
- [ ] Independent domain Inquisitor ratchets passed.
- [ ] Seam matrix passed.
- [ ] Seeded mutant proved a relevant test detects a broken invariant.
- [ ] Root serialized check/clippy/full test gate passed with logs and exit codes.
- [ ] Supported GPU/platform gates passed; unavailable environments recorded accurately.
- [ ] Diff scope, graph freshness, handoff and local traps updated.

## Platform evidence limits

- Windows evidence cannot prove NCCL or cuFile Linux execution.
- WSL2 NCCL evidence cannot prove multi-GPU behavior without the required devices.
- WSL2 is not a native Linux GPUDirect Storage gate for cuFile.
- Header discovery, compilation, runtime loading and GPU/storage execution are separate results.
- Keep missing SDKs or unavailable hardware explicit; never represent a skip as a pass.
