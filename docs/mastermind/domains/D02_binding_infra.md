# D02 — shared binding infrastructure

Updated: 2026-11-13. Status: source written and untested; checked-void repair is present, independent review and execution remain pending.
See the [domain index](../MASTERMIND_INDEX.md), [D01 ledger](D01_m2_sys.md), [seam matrix](../SEAM_MATRIX.md)
and [work queue](../../../TODO.md).

## Exact source boundary

D02's original boundary contains these 21 files. Root now owns nv-bindgen's vcv-rs adapter integration. D02 also received bounded permission for NCCL/cuFile checked-call fixtures and NCCL cleanup repairs, recorded in the [D01 ledger](D01_m2_sys.md). Documentation is maintained by the documentation lane.
Workers do not build, run tests or mutate git.

| # | Allowed source file |
|---|---|
| 1 | crates/nv-bindgen/src/nv_bindgen.rs |
| 2 | crates/nv-loader/src/nv_loader.rs |
| 3 | crates/nv-bindgen-fixture/build.rs |
| 4 | crates/nv-bindgen-fixture/src/nv_bindgen_fixture.rs |
| 5 | crates/nv-bindgen/tests/fixture/fake.h |
| 6 | crates/nv-bindgen/tests/fixture/qi.h |
| 7 | crates/nv-bindgen/tests/fixture/qi_interface.h |
| 8 | crates/cublas-sys/build.rs |
| 9 | crates/cublaslt-sys/build.rs |
| 10 | crates/cudnn-sys/build.rs |
| 11 | crates/cufft-sys/build.rs |
| 12 | crates/cupti-sys/build.rs |
| 13 | crates/cusolver-sys/build.rs |
| 14 | crates/cusolvermg-sys/build.rs |
| 15 | crates/cusparse-sys/build.rs |
| 16 | crates/npp-sys/build.rs |
| 17 | crates/nvapi-sys/build.rs |
| 18 | crates/nvcuvid-sys/build.rs |
| 19 | crates/nvjpeg-sys/build.rs |
| 20 | crates/nvml-sys/build.rs |
| 21 | crates/nvrtc-sys/build.rs |

## Current source checkpoint

Root reports shared ReturnPolicy handling, boxed CallError diagnostics, mixed-return fixtures,
Linux SDK load hints and migration of the 14 recipes listed above. All changes remain uncommitted
and untested. No fresh independent Inquisitor verdict exists.

Read-only declaration inspection confirms these anchors; it is not a complete symbol manifest:

| Anchor | Source at checkpoint | Gate |
|---|---|---|
| ReturnPolicy | crates/nv-bindgen/src/nv_bindgen.rs:681 | Static presence only; review pending |
| parse_return_policies | crates/nv-bindgen/src/nv_bindgen.rs:697 | Empty/duplicate/invalid policy rejection written; untested |
| fallback | crates/nv-bindgen/src/nv_bindgen.rs:736 | Void has no native sentinel; checked plan selected |
| wrapper_plan | crates/nv-bindgen/src/nv_bindgen.rs:761 | Shared exports/query checked scalar/void return plan; untested |
| shims | crates/nv-bindgen/src/nv_bindgen.rs:803 | Native policy/pointer versus checked Rust call shapes; review pending |
| CallError | crates/nv-loader/src/nv_loader.rs:100 | Borrowed load cause, boxed cold symbol diagnostic; layout/error semantics untested |
| mixed_return_policies_preserve_missing_symbol_errors | crates/nv-bindgen-fixture/src/nv_bindgen_fixture.rs:155 | Actual generated fixture test written; not run |
| sdk_paths | crates/nv-bindgen/src/nv_bindgen.rs:1039 | Root adapter: one target-aware SDK snapshot; dependency revision/gates pending |
| sdk_rebuild_inputs_follow_discovery_aliases | crates/nv-bindgen/src/nv_bindgen.rs:1158 | Root adapter regression written; not run |

The domain owner must partition inspected ranges to the mastermind limits and supply the full numbered
symbol manifest before a reviewer can issue a symbol-by-symbol verdict. Do not infer completeness
from the anchors above.

## Written repair and integration checkpoint

Void and scalar native calls without a native failure sentinel now keep their original names
with a Rust ABI and `Result<T, CallError>`; void uses `Result<()>`. Configured status returns and
pointers retain native C ABI behavior. Exports and query-interface generation share the checked
wrapper plan. Fixture source covers load failure, missing symbols, null query results and successful
native calls; no fixture has executed in this wave.

NCCL partial-init and abort-before-wait cleanup repairs are written with deterministic sequence
fixtures. cuFile/NCCL checked void declarations are wired into call-shape tests. Independent review
must validate these changes against their native contracts.

Root's SDK adapter reads a single vcv-rs discovery snapshot, separates runtime and link paths,
validates the Cargo target and path representation (including explicit bindings include/system
paths before libclang and core_types system paths), and tracks owner-defined root aliases as
rebuild inputs. Negative fixture source is written but unexecuted. This source depends on the unpushed vcv-rs API. No compiler, ABI/layout or zero-defect
approval is claimed.

## Review and verification queue

- [ ] Supply exact micro-unit ranges and the full changed-symbol manifest.
- [ ] Review the written checked void-return generation and every affected caller/fixture.
- [ ] Verify mixed structured-status, negative-I/O, scalar, pointer and unit return semantics.
- [ ] Verify invalid policy rejection and missing-library/missing-symbol diagnostics without panic.
- [ ] Verify boxed CallError layout and library/function-pointer lifetimes.
- [ ] Verify Linux load hints come from canonical vcv-rs discovery rather than a duplicate policy.
- [ ] Review all 14 migrated recipes against their authoritative SDK return types.
- [ ] Run a fresh independent Inquisitor through Q1 stubs, Q2 wiring, Q3 errors/panics, Q4 boundaries, Q5 churn, Q6 layout and Q7 assumptions.
- [ ] Repair every evidenced defect and rerun fresh review.
- [ ] Commit/push all involved repositories to main before root starts any build/check/test.
- [ ] Run shared fixture/negative tests and a relevant seeded mutant.
- [ ] Run serialized root check/clippy/full tests and affected Linux/Windows gates.
- [ ] Re-index after final source freeze and prove S01–S09 with D01 and vcv-rs.

## Cross-domain contracts

D01 depends on ReturnPolicy preserving each SDK's native status semantics while checked APIs expose
load/symbol failure. Loaded function pointers must retain the SDK ABI. Checked Rust entry points may
change our alpha API to expose errors; update every caller rather than retain a silent compatibility path.
Library handles must outlive all native pointers; CUDA handle types must remain shared.
SDK discovery and runtime hints must agree between vcv-rs CLI and nv-bindgen.

No compiler/test result, ABI parity, complete symbol audit or zero-defect verdict is claimed here.
