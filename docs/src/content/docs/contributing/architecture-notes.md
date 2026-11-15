---
title: Architecture Notes & Ship's Log
description: Map of internal engineering logs (PLAN.md, COVERAGE.md, AGENTS.md, DIAGRAMS.md) and archived documentation in docs/old/.
sidebar:
  order: 2
---

`nv-rs` maintains a set of root-level engineering logs ("the ship's log") to track architecture decisions, platform verification checkpoints, active work queues, and FFI symbol coverage.

## Ship's Log & Internal Docs Map

| Document | Role |
|---|---|
| `PLAN.md` | Consolidated master plan, completed milestone ledger (`M1`–`M4`), and prioritized backlog (`P2`/`P3` future work). |
| `AGENTS.md` / `DIAGRAMS.md` | Repository engineering guide, workspace boundaries, and architecture dataflow diagrams. |
| `COVERAGE.md` | Generated symbol inventory and coverage matrix produced by `cargo xtask coverage generate` from `coverage/`, recording **bound**, **wrapped**, and **tested-on-gpu** status per library. Do not edit manually. |

## Workspace Boundaries (`Cargo.toml`)

- Active workspace members are `crates/*` and `xtask`.
- `spikes` (throwaway proofs with their own toolchains and lockfiles), `device` (`cuda-oxide` device-code workspace requiring nightly), and `python` (`nv-py` maturin/PyO3 extension workspace) are excluded from the root host workspace.

## Archived Documentation (`docs/old/`)

Historical planning and multi-domain audit records are archived under `docs/old/`:

- `docs/old/mastermind/MASTERMIND_INDEX.md` — archived domain audit index.
- `docs/old/mastermind/SEAM_MATRIX.md` — archived cross-module seam matrix.
- `docs/old/mastermind/domains/D01_m2_sys.md` through `D09_sdk_wrappers.md` — archived per-domain audit notes.

`docs/old/` is kept for historical reference only and is excluded from the active Starlight documentation site under `docs/src/content/docs/`.
