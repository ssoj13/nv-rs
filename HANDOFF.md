# HANDOFF - resume pointer (rewritten 2026-11-23)

Read [`PLAN.md`](PLAN.md) first: it holds status, open items, decisions (ADR), done history, refuted findings and the gate commands. Full old documents (plan1, plan2, TODO, old milestones, the previous long handoff in git history) are archived under `audit/`; the list is at the end of `PLAN.md`.

## In-flight work

- Final wave: done and applied to `PLAN.md` (report `audit/2026-11-23/wave_final.md`; gate 500 passed / 0 failed / 207 ignored, pytest 118 passed / 2 skipped, device 15/15). Nothing of it is in flight.
- Next: review the csg-rs `R00` (GPU crowd steering into a csg-rs-owned kernel) and `R00b` (GPU traffic) branches when `Mullis-16-3934` pings, merge, then run one more full gate (`GATE:FINAL`).
- Still open (`PLAN.md` section 2): `PERF:RT` (RT shader speed-up, needs Nsight Compute; 98 % of the frame is the OptiX RT pass), `LDR:L-31a`, `PREBUILT:REGEN` (NVAPI and other prebuilt sets need newer SDKs than this machine has), `LNX:ALL` (Linux runtime never executed; NCCL needs 2+ GPUs, cuFile needs GDS; `python/` not cross-compiled), `CI:EXEC` (CI never executed; `device` job needs secret `CUDA_RUST_B_TOKEN`), `DENY:LIC` (`flex-geom`, `nodes-core`, `ui-theme` have no `license` field; operator decision), `DPY:DLPACK-WAIT`, `DPY:P17`, `APP:F24` geo-origin (needs a csg-rs `SceneDescriptor` field), `VIS:DEMO` (manual visual check), `REV:W1` / `REV:W2` (lead review of the W1 / W2 diffs), `CSG:R00` / `CSG:R00b`.

## Chat coordination

The csg-rs session is the chat peer `Mullis-16-3934` (filesystem MCP `chat_*`). Coordinate with it directly; the operator does not relay. It pings when a branch is green on the `cuda_crowd` GPU run; the lead reviews and merges, then reruns the root gate.

## Operator rules

- Chat in Russian; code, comments, docs and commits in English.
- No `Co-authored-by` footers.
- Never run `git reset`, `git checkout --` or `git stash` without explicit permission; never kill `bun` processes.
- Decide design questions yourself, record decision plus rejected alternative in `PLAN.md` section 3, push to `main` after each green gate, report afterwards.
- The lead model does not run cargo, clippy, tests or log triage: delegate to a subagent. New tests are checked by reading, not by revert-and-rerun mutation.
- Workers on disjoint files never run cargo; one gate agent per workspace; at most two builds on the machine.
