---
title: Architecture Notes & Ship's Log
description: Map of internal engineering logs (PLAN.md, HANDOFF.md, TODO.md, COVERAGE.md) and archived documentation in docs/old/.
sidebar:
  order: 2
---

`nv-rs` maintains a set of root-level engineering logs ("the ship's log") to track architecture decisions, platform verification checkpoints, active work queues, and FFI symbol coverage.

## Ship's Log & Internal Docs Map

| Document | Role |
|---|---|
| `PLAN.md` | Architecture decisions, milestone checkpoints, and workspace updates (including toolchain unification to `1.99.1`, `bootstrap.py` provisioning, `ngx` D13 wrapper design, `cuda-core-ext` rename, and historical transfer checkpoints). |
| `HANDOFF.md` | Current project state, verified Windows/WSL2/GPU test checkpoints (such as RTX 3080 Ti integration runs and Wave 2 release readiness), and continuation notes across sessions. |
| `TODO.md` | Active work queue tracking completed items and remaining tasks across workspace waves. |
| `COVERAGE.md` | Generated symbol inventory and coverage matrix produced by `cargo xtask coverage generate` from `coverage/`, recording **bound**, **wrapped**, and **tested-on-gpu** status per library. Do not edit manually. |

## Workspace Boundaries (`Cargo.toml`)

- Active workspace members are `crates/*` and `xtask`.
- `spikes` (throwaway proofs with their own toolchains and lockfiles) and `device` (`cuda-oxide` device-code workspace with its own toolchain) are excluded from the root host workspace.

## Archived Documentation (`docs/old/`)

Historical planning and multi-domain audit records are archived under `docs/old/`:

- `docs/old/mastermind/MASTERMIND_INDEX.md` — archived domain audit index.
- `docs/old/mastermind/SEAM_MATRIX.md` — archived cross-module seam matrix.
- `docs/old/mastermind/domains/D01_m2_sys.md` through `D09_sdk_wrappers.md` — archived per-domain audit notes.

`docs/old/` is kept for historical reference only and is excluded from the active Starlight documentation site under `docs/src/content/docs/`.
