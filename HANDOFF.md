# HANDOFF - resume pointer (rewritten 2026-11-23)

Read [`PLAN.md`](PLAN.md) first: it holds status, open items, decisions (ADR), done history, refuted findings and the gate commands. Full old documents (plan1, plan2, TODO, old milestones, the previous long handoff in git history) are archived under `audit/`; the list is at the end of `PLAN.md`.

## In-flight work

- Final wave agent (launched by the lead): will write `audit/2026-11-23/wave_final.md`. If the file exists, read it and tick the matching items in `PLAN.md` section 2 after review; if it does not exist and no agent is alive, check `git status` for its uncommitted edits and do not revert them.
- csg-rs session: `R00` (GPU crowd steering into a csg-rs-owned kernel) and `R00b` (GPU traffic) on branches from `origin/main`.
- Another agent edits `CHANGELOG.md` and `crates/*/AGENTS.md` concurrently; links in them to the moved plan files are stale until it finishes (`PLAN.md` item `W7.1`).

## Chat coordination

The csg-rs session is the chat peer `Mullis-16-3934` (filesystem MCP `chat_*`). Coordinate with it directly; the operator does not relay. It pings when a branch is green on the `cuda_crowd` GPU run; the lead reviews and merges, then reruns the root gate.

## Operator rules

- Chat in Russian; code, comments, docs and commits in English.
- No `Co-authored-by` footers.
- Never run `git reset`, `git checkout --` or `git stash` without explicit permission; never kill `bun` processes.
- Decide design questions yourself, record decision plus rejected alternative in `PLAN.md` section 3, push to `main` after each green gate, report afterwards.
- The lead model does not run cargo, clippy, tests or log triage: delegate to a subagent. New tests are checked by reading, not by revert-and-rerun mutation.
- Workers on disjoint files never run cargo; one gate agent per workspace; at most two builds on the machine.
