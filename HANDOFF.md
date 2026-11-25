# HANDOFF - resume pointer (rewritten 2026-11-24, Claude Code lead session "Halsted-16-2877")

Read [`PLAN.md`](PLAN.md) first (status, open items, ADRs, gate commands). The camera planner lives in the separate repo `cglibs/flight-navigator-rs` (`CAM_PLAN.md`, private GitHub `ssoj13/flight-navigator-rs`).

## State at handoff

- `nv-rs` `main` = `5b2b69b` (pushed, clean except `crates/cudnn-sys/prebuilt/x86_64-pc-windows-msvc/cudnn_api.rs`, a line-ending-only change: leave it unstaged). No agents running.
- `flight-navigator-rs` `main` = `5b0f92d` (pushed). `CAM_PLAN.md` is complete: all operator questions Q1-Q5 answered, section 12 = track library / playlists / player / Flight Lab + Player UI. No code yet.
- `camera-guide-rs` (csg-rs owns it) `main` = `6cffddf`: one speed/bank/timing planner, f64 time.
- csg-rs pin in nv-rs: `0fb1087`. csg-rs `main` is at `697a203` (privacy scrub); bump pending.

## Done on 2026-11-24 (all gated: clippy -D warnings clean, nv-demo tests incl. GPU green)

| Commit | What |
| --- | --- |
| `d4f1098`, `32a6ab1`, `365fad0`, `8aefc88` | `--bench-rail` deterministic benchmark; csg-rs P23 adopted; Flight stats; `audit/2026-11-24/perf_rt.md`. PERF:RT parked (operator: accept the regression). |
| `64cf59a`, `5a5ed3a`, `b24c217` | csg-rs `0fb1087`; viewport + possession toolbars (phosphor icons, shared style); generic GPU overlay renderer; native-resolution wireframe contours. |
| `beadd94`, `8e73de9`, `8c34cf1`, `ec1b039`, `9522a10`, `80fd055` | GPU axes overlay; GPU picking (32 B pick record, host ray-march path deleted, squarebob/warpbro click model, `sl-config` dependency); IS counters + `--bench-views p37`; eye category filter (u64 hidden_semantics; `LaunchParams` grew to 672 B). |
| `27f8f78`, `7c374e5`, `5b2b69b` | Eye fast path (3-way classify, visibilityMask hide, render rays mask 0x7F, 0x80 reserved, `Params` 680 B); overlay composited after DLSS SR/RR at output resolution; axes LOD per part (96 px). |
| `db3577d`, `724d00c` | PLAN rows GPU:FRAMEPLANS / OVERLAY:NEXT / P37:SWEEPS; residential street address removed from docs. |

## Exact resume point

- [ ] **Waiting on csg-rs hashes**, then bump nv-rs and adopt (one wave):
  - `697a203`: privacy scrub (viewpoint label becomes "Marina Club - Balcony View").
  - P41 `semantic_masks` + `instance_semantic_mask` / `csg_instance_semantic_mask`: replace the body of nv-demo `semantic_filter::instance_semantic_mask` (PLAN `FILTER:FAST`); expect the eye filter to become near free. Verify with `--bench-views p37`, H = {Vehicles, Industrial}.
  - Palette 84 -> 85 (SOIL): only docs mention 84.
  - Scene registry: `Gundo2` clone + historical order + display names + phosphor icon hints. Show the ordered list to the operator before csg-rs commits, then make the CSG World scene buttons one row of phosphor icon buttons.
- [ ] **GPU:FRAMEPLANS**: when csg-rs ships Crowd/Traffic/CameraFramePlan, write the generic runner and delete `prepare_crowd_steering_and_grounds`, the CPU traffic step and the private pack_visible copy (fixes feet in sidewalks, one-frame leg jumps, crowd semantic overwrite).
- [ ] **OVERLAY:NEXT**: skeletons (needs `ArchetypeSkeleton`), per-object axes/selection (needs P42 object_id), switch to csg-scene `OverlayCurvePod` (P36 exact lines, K6 visibility raygen with `vis_tmax`), OSM section of the eye filter (P42).
- [ ] **P37:SWEEPS**: A/B every csg-rs sweep wave with `--bench-views p37`; payload slot 9 = ray cone half2 when the first MESH record lands.
- [ ] **flight-navigator-rs implementation** (operator asked why wait for voxels; answer: don't). Start now, everything at once per `CAM_PLAN.md`: fnav-core (track format f64, transition engine, moves, camera-guide-rs wrapper), fnav-interest, fnav-harmony, fnav-space against the `Volume` trait with synthetic fixtures, fnav-plan, library/playlist files, and the nv-demo Flight Lab + Player panels replacing "Cinema & Camera Rail". Plug in the csg-scene voxel volume when csg-rs ships it. **This was the next step when the session ended: the operator had not yet said go.**
- [ ] RAIL:ROLL / dog trot / leg-jump fixes: adopt when csg-rs pushes them.

Out of scope: rewriting git history (operator: never); stripping `addr:*` tags of other buildings in the OSM extracts (operator: leave as is, don't raise again); further RT perf hunting (parked).

## Chat coordination

The csg-rs session is the chat peer `Mullis-16-3934` (filesystem MCP `chat_*`, mode auto). Coordinate directly, don't make the operator relay, and restart `chat-wait` in the background after each message. The tars-rs session (`tars-rs-30`) shares the machine: CARGO_BUILD_JOBS=6, and no simultaneous full suites; ping it for a quiet GPU.

## Operator rules (additions today)

- Rustdocs everywhere explaining WHY (`//!` + `///`, CUDA block comments); put this in every agent brief and peer relay.
- Never put residential addresses in labels, code, docs or commits.
- Systemic, built-in, GPU-first features; nothing may slow the frame; no fakes or stubs; a disabled UI shows its exact blocking reason.
- Existing rules still hold: Russian chat, English code; no Co-authored-by; no git reset/checkout--/stash; never kill bun; push to main after green gates; the lead delegates cargo runs to subagents.
