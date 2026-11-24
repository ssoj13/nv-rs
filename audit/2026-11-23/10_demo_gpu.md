# nv-demo GPU slice audit (shaders, CUDA crowd, bloom, export, profiler, build)

Date: 2026-11-23. Repo HEAD `38ec140`. csg-rs pinned by Cargo.lock at `f7c8474` (checkout `~/.cargo/git/checkouts/csg-rs-3a6ed3e588aad103/f7c8474`).
Mode: read-only. No cargo/nvcc/build/test was run.

Severity rubric used: P0 = UB/OOB/crash/wrong exported data; P1 = wrong displayed AOV, leak, swallowed error with real consequence, silent fallback, significant perf; P2 = dedup/SSOT, dead code, doc drift, minor visual.

## 1) Item-by-item ledger

| ID | Sev | Verdict | Location | Defect | Failing scenario | Fix direction |
|---|---|---|---|---|---|---|
| D01 | P0 | CONFIRMED | `shaders/raytracer.cu:2114` (+ `renderer.rs:3030-3033` export, `sdg_export.rs:143`) | Depth AOV is planar `max(0, dot(hit-cam, cam_fwd))` for every projection. With `Equirect360` / `SlitCam`, the rear hemisphere is clamped to `0`. With `Fisheye180`, depth goes to ~0 at the rim. `first_range` is computed but never exported. | An SDG burst with `CameraProjection::Equirect360` writes `depth.Z = 0` (reads as "at the lens") for every pixel behind the camera, so ground truth is wrong for half the panorama. | For non-pinhole projections, export Euclidean range (or add a separate `range` channel) and record the depth semantics in the manifest. |
| D02 | P1 | CONFIRMED | `raytracer.cu:1914` (`has_prev_cam = ... && proj == 0u`), `2079-2092`, `2142-2156` | Motion vectors are only produced for Pinhole. Fisheye/Equirect/SlitCam always write `(0,0)`, which looks like valid zero flow. | An SDG EXR `motion.X/Y` from a 360 sensor is all zeros while the camera or actors move. No flag or manifest field says it is invalid. | Add a reprojection per projection model, or write NaN / a validity flag and record it in the manifest. |
| D03 | P1 | CONFIRMED (code) / PLAUSIBLE (visual) | `raytracer.cu:2115-2141` | Object motion is only applied to CSG instances that have `MOTION_BLUR`. For classic geometry (fractal TRS in TLAS, orbiting glass spheres driven by `sat_angle`, OMM halo), `prev_world_pos = hit_pos`, so only camera motion is encoded. | With `auto_rotate` / sat orbit on, DLSS-SR/RR and the temporal OptiX denoiser get wrong motion vectors for the moving fractal and spheres: ghosting and smearing. | Pass previous classic instance transforms (prev `fractal_transform`, prev sat params, or a small prev-xform table indexed by classic instance id) and reuse `csg_inv_xform_pt_ldg`. |
| D04 | P1 | CONFIRMED | `raytracer.cu:1868`, `2118`, `2599` + external `csg_eval.cuh:2380-2393` | The Cryptomatte hash is `murmur3(inst_id, part_id, sem_id)`. On `RoadTerrainTile` hits, `part_id` is the packed per-hit road UV (`s mod 64`, lateral). Every road/sidewalk/curb pixel therefore gets a different ID. | AOV mode 10 shows per-pixel colour noise on all roads, so no object matte can be extracted for road classes. | Exclude the UV-packed `part_id` from the hash (zero it for road-tile prims or carry the UV in a separate attribute). Keep `part_id` as a stable sub-part ID. |
| D05 | P1 | CONFIRMED | `cuda_crowd.rs:1181-1184` (+ caller `renderer.rs:4348-4364`) | `step_gpu_direct` scatters into CSG/OptiX buffers whenever `d_bone_slot_map.len() >= n_bones`. It never checks `self.topology_dirty`. The renderer only logs a failed `upload_scatter_topology` and then calls `step_gpu_direct` anyway. | After a `FullRepack`, if the topology upload fails (H2D or alloc error), the stale slot map scatters crowd bones over unrelated CSG instances and OptiX slots. Result: the wrong GAS handle and transform at those slots, a corrupted IAS and image, and wrong `csg_instances` metadata. | Inside `step_gpu_direct`, skip the scatter (or return Err) while `topology_dirty`. Alternatively have the renderer abort the scatter on upload failure. |
| D06 | P1 | PLAUSIBLE (perf) / CONFIRMED (no-op) | `profiler.rs:641-643` | A CUPTI `Subscriber` is enabled on the whole `DriverApi` domain with an empty closure `|_domain, _cbid| {}`. Every driver call (launches, memcpys, event records, OptiX internals) pays the callback cost. `ConcurrentKernel` activity is also always on. The subscriber is never read. | In steady-state rendering the CPU-side driver overhead goes up for no output. The extra cost scales with the per-frame call count (crowd: 4 launches plus copies; denoiser/OptiX: many). | Remove the subscriber, or enable activity and subscriber only while the Performance panel needs CUPTI data. |
| D07 | P1 | PLAUSIBLE | `build.rs:23-40`, `renderer.rs:998-1018` (duplicated) | OptiX SDK discovery takes the first `OptiX SDK*` folder in `read_dir` order. On NTFS that order is sorted by name, so the oldest SDK wins. `raytracer.cu` needs OptiX 8+/9 device APIs (`optixTraverse`, `optixReorder`, `optixHitObject*`). | A host with `OptiX SDK 7.7.0` and `OptiX SDK 9.x` installed: `nvcc` fails, `build.rs` prints a warning and falls back to runtime NVRTC, which picks the same old SDK, and `initial_ptx().expect("raytracer.cu PTX")` panics (`renderer.rs:2005`). An 8.x pick gives PTX built against a header set that differs from the 9.x host ABI. | Pick the highest version, put the lookup in one shared helper, and prefer `OPTIX_ROOT` / fail loudly. |
| D08 | P2 | CONFIRMED | `gpu_export.rs:560-583`, `renderer.rs:1070-1092` | The PTX cache key is incomplete. For raytracer the OptiX tag is the include path string, not the header content or version. NVRTC version and compile options (`fast_math`, `-std`) are not hashed. A corrupt or unloadable cached PTX is never evicted (load fails, an error is returned, and the file stays). | (a) An in-place SDK update in the same folder, or a toolkit/NVRTC upgrade with an unchanged driver, reuses stale PTX. (b) A damaged cache file makes the strict `CrowdBackend::Cuda` fail on every launch until someone deletes it by hand. | Hash `optix.h` / `optix_device.h` bytes (or `OPTIX_VERSION`), the NVRTC version and the option list. Delete the cache entry and recompile when `cuModuleLoadData` / OptiX module creation fails. |
| D09 | P2 | CONFIRMED | `raytracer.cu:2138`, `2730` | Classic semantic ID = `inst_id+1` lands in the Cityscapes table: fractal→2 SIDEWALK, glass→3 CURB, neon→4 BUILDING, OMM halo→5 WINDOW (floor→16 GROUND). The mapping is duplicated in `__raygen__score_views`. | Semantic AOV / SDG `semantic.id` labels the fractal as sidewalk when the classic scene is shown. Smart View scoring is biased the same way. | Reserve dedicated semantic IDs (or UNKNOWN) for classic objects in one shared table. |
| D10 | P2 | CONFIRMED | `raytracer.cu:2119-2122`, `2139`, `2721-2725`, `2732`, `2774-2780` | Instance IDs are inconsistent. Render path: CSG uses `user_id`, or `p6+1` when `user_id == 0`. Classic uses `p6+1`. That collides (classic floor id 0 → 1 == CSG instance 0 with user_id 0 → 1). `score_views` and `sdg_keypoints` return raw `meta.y` (can be `0`), and raw `p6` (including hue bits) for classic. | Instance AOV / SDG `instance.id` can merge a classic object with a CSG instance. The occlusion `blocker_uid` does not match the IDs in the EXR. | Use one ID function shared by all three raygens. Namespace classic IDs, for example a high bit. |
| D11 | P2 | CONFIRMED | `raytracer.cu:2558-2570`, `2611-2622`, `2650-2651`; `renderer.rs:4934-4937` | In AOV modes the 4x RGSS average goes only into `color_buf`. `albedo_buf`, `normal_buf` and the raw outputs (motion/depth/semantic/instance) get sample `s0` at offset (-0.375,-0.125). The tonemap shows `albedo_f32` / `normal_f32` for modes 1 and 2. | The Albedo/Normal AOV display is not anti-aliased and is shifted by 3/8 px; the RGSS work for those modes is discarded. SDG raw label buffers are also sampled off-centre. | For AOV mode 1/2, either display `color_buf` or write the RGSS average into the albedo/normal buffers. For the raw label outputs, use a pixel-centre sample (0,0). |
| D12 | P2 | CONFIRMED | `raytracer.cu:2591-2593`, `2609-2610`; `tonemap.cu:4` | WorldPos AOV: `final_col = sum(pos)*0.25` is not divided by the hit weight, and `w = hits/4`. The tonemap treats `w < 0.5` as a miss. | At silhouettes, pixels with 1/4 coverage turn black and pixels with 2/4 or 3/4 coverage show `pos*coverage` (pulled toward the world origin). | Divide `aov_accum` by `aov_w_accum` for mode 7 and keep coverage in alpha. |
| D13 | P2 | CONFIRMED | `raytracer.cu:1287-1289`, `1305`, `1403-1404`, `1436`; external `csg_eval.cuh:2383` | `road_s` wraps every 64 m (`rem_euclid(s, 64)`). Tire-wear `sin(road_s*0.35)` (period 17.95 m) and the 1.5 m slab grid (64/1.5 is not an integer) jump at every wrap. `has_road_uv` is set for any sem 1..3 prim with a non-zero `part_id`, including non-road assets. | A visible seam across asphalt and sidewalk every 64 m along long roads. Sidewalk assets with user part IDs get garbage joints. | Use wrap-commensurate frequencies (64/n), or pass `s` with more range. Flag road-UV hits explicitly instead of inferring from sem/part_id. |
| D14 | P2 | CONFIRMED | `raytracer.cu:1293`, `1334-1335` | The asphalt micro-normal "gradient" subtracts `grain` (sampled at frequency 22) from a sample at frequency 24, so it is not a finite difference. | The asphalt bump becomes uncorrelated normal noise instead of aggregate relief. | Use the same frequency for the centre and offset taps (as the CONCRETE branch does at 1374-1375). |
| D15 | P2 | PLAUSIBLE | `raytracer.cu:2285-2302`, `2526` | Sun tint is mode-based (8 → night, 1 → sunset) only while `sky_blend` is strictly between 0 and 1. Otherwise it comes from `csg_flags` NIGHT/SUNSET (from `csg_time_of_day`, `renderer.rs:4571-4575`). The classic path uses a third fixed tint. | Sky mode 8 picked without TOD=night: during a crossfade the sun is night-tinted, and at blend ≥ 0.9999 it pops to the day tint. | Derive the tint from sky modes in all cases (one helper shared by CSG and classic paths). |
| D16 | P2 | CONFIRMED | `raytracer.cu:2437`, `2483-2487`, `2537-2540`, `2544-2547` | `horizon_fade` is the constant `0.0f`, so three `sky_color` branches are unreachable. | Dead code left over from an unfinished feature. | Delete the branches, or implement the fade. |
| D17 | P2 | CONFIRMED | `raytracer.cu:22`, `41` (`renderer.rs:90`, `109`) | `Params::fractal_xform` and `Params::sat_pos` are never read by device code (fractal TRS now lives in the TLAS). `csg_flags` bit0 (ENABLED) and bit2 (WET_ASPHALT) are also unread on device. | Dead fields in the 640 B ABI. Host writes are wasted and mislead readers. | Remove them, or mark them reserved in both structs. |
| D18 | P2 | CONFIRMED | `raytracer.cu:1756-1765` | The comment says CSG buckets never collide, but classic hints 60..63 overlap CSG material buckets 60..63 (`mat_id & 0x3F`). | SER groups classic hits with CSG materials 60..63, so coherence is slightly worse (perf only). The comment is wrong. | Reserve 60..63 (map CSG buckets into 1..59) or fix the comment. |
| D19 | P2 | PLAUSIBLE | `raytracer.cu:1881` | Seed = `pcg(x + y*8192 + frame*16777619)`. Because `16777619 = 2048*8192 + 403`, frame f+1 at pixel (x,y) replays the exact random sequence of frame f at (x+403, y+2048). Widths above 8192 alias rows. | 4K and larger: rows 0..111 reuse the previous frame's sequences from rows 2048+, which correlates noise across frames for temporal denoising/DLSS. Widths above 8192: duplicated noise between rows. | Hash `(x, y, frame)` with independent rounds, e.g. `pcg(pcg(pcg(frame) ^ y) ^ x)`. |
| D20 | P2 | PLAUSIBLE | `raytracer.cu:1068-1073` (and modes 1/8) with NEE `2275-2318`, `2516-2543` | The sun is counted twice. Next-event estimation adds direct sun, and indirect bounce rays that reach the sky also pick up the sun disc (x1.1) and the broad Mie lobe (`pow(sun_dot,8)*0.16`). | Indirect lighting is biased bright (most visible on diffuse ground at high spp). | Exclude the sun disc and Mie lobe from sky lookups on bounce > 0 (NEE already covers them), or use MIS. |
| D21 | P2 | PLAUSIBLE | `raytracer.cu:2881-2893`, `2969-2976` | The fractal hit epsilon is scaled by distance to `params.cam_pos` and by the main camera FOV/height, even for `score_views` / `sdg_keypoints` rays that start at probe eyes or keypoints. | Probe and occlusion rays against the fractal use a mis-scaled epsilon, so hit distances and visibility are inaccurate (over-fat or noisy). | Scale the epsilon by distance from the ray origin (`optixGetWorldRayOrigin`), or pass the cone angle per launch. |
| D22 | P2 | PLAUSIBLE | `raytracer.cu:3130-3159` | A multi-AABB `PURE_UNION` GAS reports each primitive's boundary independently. For rays starting inside one member, this exposes internal surfaces of the union. There is also no prim-count bound on `prim_idx` (`Params` has no prim count). | Camera or refracted rays inside an overlapping union hit internal faces. A host `primitive_offset` bug would read `csg_prims` out of bounds. | Restrict multi-AABB to non-overlapping members or opaque-only materials. Add `csg_prim_count` to `Params` and bound-check it. |
| D23 | P2 | CONFIRMED | `tonemap.cu:208`; `gpu_export.rs:449-483` | The comment claims parity with `hdr_f32_to_p010_pq`, which no longer exists. The real PQ encoder `scrgb_f16_to_p010_pq` has no 750→1000-nit rolloff; it relies on the scRGB tonemap having applied it. | Doc drift. If the source scRGB were produced without the tonemap, PQ output would clip at 10,000 nits instead of rolling off. | Fix the comment and state the dependency in one place. |
| D24 | P2 | CONFIRMED | `gpu_export.rs:946` | The JPEG is written with `std::fs::write` (not atomic), unlike the SDG files, which use `write_bytes_atomic`. | A crash or full disk during capture leaves a truncated `.jpg` next to an atomically written `.sdg.json` sidecar. | Reuse `sdg_export::write_bytes_atomic`. |
| D25 | P2 | CONFIRMED | `fft_bloom.rs:378-392` | Bloom compiles its kernels with NVRTC directly, bypassing `compile_ptx_cached`. There is no disk cache and the NVRTC options are duplicated. | NVRTC recompiles on every start; options can drift from the other kernels. | Route it through `compile_ptx_cached`. |
| D26 | P2 | CONFIRMED | `cuda_crowd.rs:1605`, `1700`, `1785`, `1850` | GPU tests are not `#[ignore]` (`CudaContext::new(0).expect(...)`). `cuda_gpu_direct_dem_and_walkway_steering_matches_cpu_without_midframe_sync` builds `cpu_scene`/`cpu_grounds` but never compares them to GPU results (only checks finiteness), and returns silently when there are no actors. | `cargo test --workspace` panics on a host without CUDA. The parity test is vacuous: GPU DEM or walkway divergence passes. | Add `#[ignore]` to the GPU tests and actually compare grounds, intents and states against the CPU reference. |
| D27 | P2 | CONFIRMED | `cuda_crowd.rs:885-910` | `sync_cpu_snapshot` skips the download when device buffers are smaller than host vectors, yet clears `cpu_bones_stale` and returns `Ok`. | When sizes change before the next step (for example a spawn without an upload), picking or SDG export runs on a stale CPU pose with no error. | Return Err (or keep it stale) when sizes mismatch. |
| D28 | P2 | PLAUSIBLE | `cuda_crowd.rs:1106`; external `actor_sim.cuh:402` | `dt` is clamped to `[1e-4, 1/30]`, so `dt = 0` still advances the crowd by 0.1 ms per step and produces non-zero prev/cur deltas. | If the crowd is stepped while paused (time scale 0), actors creep and motion vectors are non-zero. | Allow `dt == 0` (skip integration) and clamp only the upper bound. |
| D29 | P2 | CONFIRMED | `profiler.rs:76-95`, `132-138`, `241` | `FrameEventSlot::new` returns early with `?` after creating some events, and there is no `Drop` on the slot, so those events leak. `GpuPassProfiler::new` leaks `s0`/`s1` if `s2` fails. `cuEventRecord` errors are discarded with `let _`, so elapsed times may come from stale events. | A partial init failure leaks up to 17 events. A record failure silently reports bogus or zero timings. | Add `Drop` on the slot (bind the context) and surface record errors (for example mark the slot invalid). |
| D30 | P2 | CONFIRMED | `build.rs:10-21`, `188-190`, `149`, `163-165` | `find_nvcc` never returns `None` (dead `else return`) and falls back to a bare `nvcc.exe`. The Linux NGX staging copies only when the destination is missing, so a stale `.so` is never refreshed. `fs::copy` failures are discarded with `let _`. | An SDK update leaves an old DLSS `.so` next to the binary on Linux. A copy failure goes unnoticed. | Compare size/mtime as on Windows and emit `cargo:warning` on failure. Return `None` when nvcc is not found. |
| D31 | P2 | CONFIRMED | `audit/2026-11-23/plan1_nvdemo.md:33`; `AGENTS.md` (repo and crate), `DIAGRAMS.md:82`, `docs/.../nv-demo.md:31,36` | The docs claim "Typed Payloads", but the pipeline uses plain 8 payload values (`renderer.rs:1193`, no `OptixPayloadType` anywhere in nv-demo history). The docs say CSG uses "SBT 4 / SBT 5"; the actual hit records are 8/9 (RPN) and 10/11 (prim) (`renderer.rs:1213-1214`, `csg_scene_host.rs:50-55`). | Readers and agents reason from the wrong SBT indices. A claimed optimisation does not exist. | Correct the docs (or implement payload types). |
| D32 | P2 | CONFIRMED | `sdg_export.rs:142-149`, `289-307` | The docs call `motion` "optical flow [dx, dy]", but it is backward flow (`prev - current`, pixels, y-down, jitter-free). Depth uses an undocumented `1e6` miss sentinel. "SemanticId 0..=19" is not true for classic objects (D09). `write_bytes_atomic` leaves the `.tmp` behind on write/sync/rename failure. | Dataset consumers apply the wrong flow sign. Leftover temp files accumulate. | Document the conventions in the manifest and remove the temp file on the error path. |

### External: csg-rs (`f7c8474`)

| ID | Sev | Verdict | Location | Defect | Failing scenario | Fix direction |
|---|---|---|---|---|---|---|
| E01 | P2 | PLAUSIBLE | `crates/csg-device/shaders/csg_eval.cuh:5298-5317`, `5354-5384` | The large-stack RPN paths (`csg_trace_rpn_slow_large_val`, `_large_shadow`, including the FINITE_REPEAT branch) compute `overflow` and ignore it. | A compound with `max_stack > CSG_MAX_STACK (12)` silently produces truncated spans: wrong geometry with no error. | Assert `max_stack <= 12` at compile time on the host, or report the overflow (for example a debug colour or miss counter). |
| E02 | P2 | CONFIRMED (latent) | `crates/csg-actors/shaders/actor_sim.cuh:844`, `846` | When there is no SBT table, the in-kernel OptiX instance fallback uses `sbtOffset = 5u` (stale; the prim hit group is now 10) and `flags = 0u` instead of DISABLE_ANYHIT. | Latent: nv-demo passes null `optix_instances` to `step_and_emit_actors_cuda` and scatters separately. Any other consumer using the kernel's own OptiX write gets the wrong hit group. | Take the fallback offset from a shared constant (10) and set DISABLE_ANYHIT. |

### `#[expect(...)]` attributes reviewed (all justified; the lint fires: `self` plus arguments > 7)
- `profiler.rs:391` `record_with_engines` (self + 8): valid.
- `profiler.rs:515` `record_frame_engines` (self + 8): valid. Optional SSOT improvement: pass `GpuPassSample` instead of a flat list of 8 `f32`.
- `cuda_crowd.rs:916` `step_gpu_direct` (self + 8): valid.
- `cuda_crowd.rs:1321` `step_crowd_raw_for_test` (self + 9, `cfg(test)`): valid in test builds.
- `gpu_export.rs:732` `save_jpeg_from_pitched` (9): valid.
- `fft_bloom.rs:545` `apply` (self + 7 = 8): valid.

## 2) Files inspected

| File | Lines | Read |
|---|---|---|
| `crates/nv-demo/shaders/raytracer.cu` | 3207 | fully |
| `crates/nv-demo/shaders/tonemap.cu` | 259 | fully |
| `crates/nv-demo/src/cuda_crowd.rs` | 1992 | fully |
| `crates/nv-demo/src/fft_bloom.rs` | 842 | fully |
| `crates/nv-demo/src/gpu_export.rs` | 1348 | fully |
| `crates/nv-demo/src/sdg_export.rs` | 444 | fully |
| `crates/nv-demo/src/profiler.rs` | 723 | fully |
| `crates/nv-demo/src/nvtx.rs` | 83 | fully |
| `crates/nv-demo/nvtx_shim.c` | 48 | fully |
| `crates/nv-demo/build.rs` | 212 | fully |
| `AGENTS.md`, `crates/AGENTS.md`, `crates/nv-demo/AGENTS.md` | n/a | fully |
| `crates/nv-demo/src/renderer.rs` | ~5400 | contract ranges only: 40-175, 998-1093, 1180-1389, 1850-2008, 2990-3145, 3340-3364, 3905-3934, 4300-4420, 4540-4630, 4800-4950, plus greps |
| `crates/nv-demo/src/csg_scene_host.rs` | ~3100 | contract ranges only: 40-120, 840-980, 2100-2172, plus greps |
| external `csg-actors/shaders/actor_sim.cuh` | 856 | 1-150 via grep, 370-460, 720-856 |
| external `csg-device/shaders/csg_eval.cuh` | 5629 | 1-250, 2378-2395, 5290-5629, plus greps |
| external `cuda-core/src/allocation.rs` (cuda-rust `348895e`) | n/a | `RawDeviceBuffer` drop (lines ~90-140) |
| `audit/2026-11-23/plan1_nvdemo.md`, `DIAGRAMS.md`, `docs/.../nv-demo.md` | n/a | grep only (doc drift) |

## 3) Files changed

None. Only this report was written, in the session scratchpad outside the repo.

## 4) Static and invariant checks performed

- **`Params` vs `LaunchParams`:** compared field by field (`raytracer.cu:4-76` vs `renderer.rs:72-143`). The order and types are identical. Both sides assert size 640 and the tail offsets 592/600/608/616/624/632. Head offsets are covered implicitly by the identical order plus the total size.
- **Payloads and attributes:** the pipeline uses 8 payloads and `attribute_values = 5` (`renderer.rs:1193-1195`).
  - Max attributes reported: CSG 5 (`3084-3092`, `3149-3157`), fractal and sphere 4.
  - The SER hint reads `optixHitObjectGetAttribute_3` only for hitKind 1 (CSG, 5 attributes). This is valid.
  - The shadow trace uses 1 payload. Miss and CH write only p0. Primary CH programs write p0-p7. Miss writes p0 = 0 and raygen branches on p0 before reading the rest. There are no reads of unwritten slots.
- **SBT:** 12 hit records `[radiance, shadow]` with stride 2 and miss index = ray type (0/1).
  - Instance offsets: floor/neon 0, fractal 2, sphere 4, OMM 6, CSG RPN 8, CSG prim 10 (`renderer.rs:1863-1958`, `csg_scene_host.rs:50-55`, scatter fallback 10 at `cuda_crowd.rs:532`).
  - The shadow hit groups carry the matching IS programs, so custom primitives intersect on shadow rays.
- **Instance ID contract:** `optixGetInstanceId()` is the index into `csg_instances` (`csg_scene_host.rs:2142-2170`; scatter `oi.instanceId = out_idx`). Every device read of `csg_instances[p6]` is guarded by `p6 < csg_instance_count`.
- **Multi-AABB mapping:** `primitive_offset = first_prim` / `prim_idx` (`csg_scene_host.rs:911-941`) matches `optixGetPrimitiveIndex()` being used as the global `prim_idx` (`raytracer.cu:3132`).
- **Struct layouts:**
  - `CsgGpuInstance` / `CsgGpuMaterial` have size 64 (`csg_eval.cuh:198-199`). The shader adds offset asserts (`1133-1142`).
  - `load_csg_instance_meta` (`uint4` index 3) and the `float4` x4 material loads are consistent.
  - The `prev_xforms` stride is 48 B, so the 16 B-aligned `float4` `__ldg` reads are valid.
  - `ActorBoneOutPod` (64 B, flags at 60) matches `CsgGpuInstance`. `ActorOptixInstancePod` is 80 B with offsets asserted (`actor_sim.cuh:126-147`).
  - The instance flag bits 0..3 do not collide with `part_id << 8` in the bone flags.
- **CUDA grids:** every kernel bounds-checks against its element count.
  - Crowd uses `div_ceil(128/256)`.
  - Bloom grids cover `fft_w x fft_h` (bright pass, PSF), `half_w x fft_h` (spectral) and `w x h` (composite).
  - Export grids cover `w x h`. The 4:2:0 chroma taps at x+1/y+1 are in bounds because even dimensions are enforced (`gpu_export.rs:985`).
- **Bloom maths:**
  - R2C/C2R `half_w = n/2+1` and `idist`/`odist` are consistent.
  - The normalisation `inv_n / DC` gives a unit-gain PSF with the unnormalised C2R. DC is at least 1 (core term at r = 0).
  - PSF support `r < r_cut = min(pad)` guarantees no wrap into the image region.
  - Old plans and buffers are dropped before re-creation on resize.
- **Russian roulette:** starts at bounce ≥ 2, with `p = clamp(max(throughput), 0.05, 0.95)` and throughput divided by `p`. This is unbiased.
- **Bloom input:** bloom writes the denoised copy, not the accumulator (`renderer.rs:4871-4905`), so there is no feedback.
- **NaN guards:** `f3_norm` has a NaN- and zero-safe fallback. The DLSS pack and scRGB sanitise NaN. ACES maps NaN to 1.
- **Async staging (crowd):**
  - The pinned displacement flag is read only after the end-of-frame fence.
  - Pageable H2D sources are staged before the call returns.
  - `GrowCudaBuf` reallocation drops buffers through cuda-core's deferred, context-bound free.
- **Atomicity:** `write_bytes_atomic` does temp file, flush, `sync_all`, then rename. The PTX cache write is temp-with-pid then rename.

## 5) Risks and open seam questions for other slices

1. **DLSS motion and depth conventions (renderer):**
   - The shader writes backward pixel motion vectors (`prev - cur`, y-down, jitter removed) and linear view depth, with `1e6` on miss.
   - The DLSS setup needs to be checked for MV scale/sign and the depth flags (linear vs hardware/inverted).
   - With DLSS-RR, sky pixels get normal `(0,0,0)` and roughness 0 (`raytracer.cu:2066-2071`).
2. **`csg_prev_xforms` length (csg_scene_host):** the shader indexes `prev_xforms[p6*12]` guarded only by `csg_instance_count`. The full upload path (`csg_scene_host.rs:1491`, `2040-2046`) uploads `motion_prev_xforms_slice()` without asserting its length equals the instance count. A shorter slice leads to out-of-bounds `__ldg` reads.
3. **Silent fallbacks (renderer):**
   - Bloom is skipped silently when the denoiser is off and `denoised_f32` is `None` (`renderer.rs:4877-4895`).
   - SDG semantic/instance buffers are allocated with `.ok()` (`4593-4596`), which swallows OOM.
   - `initial_ptx().expect` (`2005-2006`) and `Accel::instances_into(...).expect("IAS CUDA crowd refit")` (`4405`) can panic at runtime.
4. **IAS ↔ CSG slot mapping:**
   - The scatter assumes CSG index `i` maps to OptiX slot `classic + i`.
   - `append_optix_instances` skips `cid >= gas_count`.
   - The renderer guard (`4327`) then sets `optix_ptr = 0`, so CSG buffers are updated while the IAS keeps stale transforms. The CSG intersection then evaluates object-space rays from stale instances against updated metadata.
5. **SDG EXR beauty source:** `write_sdg_exr` reads `rig.color_f32`. If `aov_mode != 0` during a burst, the "beauty" RGBA is the AOV visualisation. The beauty RGB is accumulated, while the label buffers are single-sample from the last frame (jittered or s0 RGSS offset, D11).
6. **Crowd stepping cadence:** if `step_gpu_direct` runs more than once per fenced frame (shutter subframes or burst steps without a fence), the pinned displacement flag is read while its DMA may still be in flight. The renderer's subframe loop needs a fence check.
7. **MP4 finalisation:** if `VideoRecorder::push_frame` errors, dropping the recorder without `finish()` leaves an MP4 with no `moov` box. Check renderer error handling.
8. **Freeze and possess:** intents are synced on freeze, but `sync_cpu_snapshot` skips silently on size mismatch (D27). PokeCrowd and Freeze rely on it.
9. **UI ranges:** sky mode 3 uses `powf(cloud, neb_cont)`. If the UI allows `neb_cont <= 0` (or a negative `neb_dens`), the result is Inf/NaN sky radiance that sticks in the accumulator. Confirm the UI clamps.
10. **External csg-rs:** confirm the host compiler caps `max_stack` at 12 (E01). Confirm whether RoadTerrainTile `part_id` UV packing is meant to reach Cryptomatte (D04).
