# Adopting csg-rs `main` `125b870` in `nv-demo` (change list from the csg-rs session, 2026-11-23)

Source: chat message #629 from the csg-rs session (Mullis-16-3934). Their nv-demo agent was stopped before any edit; it changed no nv-rs file. `Cargo.lock` still pins csg-rs `f7c8474a`. Apply AFTER the root gate is green. Line numbers are at nv-rs `4dd4e2d`; re-anchor before editing.

1. `src/cuda_crowd.rs:22`, `:765-766`: compile `csg_actors::pod::actor_sim_cuh_source()` instead of pasting `ACTOR_SIM_CUH` (otherwise NVRTC fails with `#error "actor_sim.cuh needs the generated constant block"`).
2. Kernel launch (`cuda_crowd.rs:1165-1201` and the second site near `:1463`; argument array `[*mut c_void; 18]` at `:1182`): `step_and_emit_actors_cuda` now has 19 parameters. Append `uint32_t lod2_pawn_compound` after `cull_dist_m` (the actor bank's LOD2 pawn compound id, or `0xFFFFFFFF` to keep the root bone's own compound). `archetypes` must be non-null (the kernel returns early otherwise). Signature: `csg-rs/crates/csg-actors/shaders/actor_sim.cuh:641-660`.
3. `ActorStatePod` is 96 B (new `foot_lock: [[f32; 2]; 4]`); `ArchetypeMetaPod` is 272 B (new `limb_mounts`; an unused chain index is `u32::MAX`). nv-demo's own kernels at `cuda_crowd.rs:343/361` and `:458/467` take the state by value: nothing may write a 64 B state or clobber `foot_lock`.
4. `shaders/raytracer.cu:3170` (`__intersection__csg`) and `:3235` (`__intersection__csg_prim`): pack `csg_instance_semantic(flags, hit.sem_id)` (from `csg_eval.cuh`; `flags` is `meta.w`) instead of `hit.sem_id`. Keep `GpuInstance::flags` bits 16..24 wherever flags are masked or rebuilt on the host.
5. `src/csg_scene_host.rs:2570-2571`: replace the literal 256 compound check with `csg_scene::gas_ledger::MAX_GPU_COMPOUNDS`. Log `world.library.take_issues()` at scene upload (compounds the compiler rejected). Call `world.set_crowd_culling(false)` where the GPU steps the crowd. `InstanceId.0` is no longer an index into `dynamic_instances()` (use `dynamic_index`): check `csg_scene_host.rs:766`, `:2939`, `:2952`, `:3000`. `src/sdg_export.rs:365` (`800_000 + i`): crowd bones are now tagged by `DynamicKind`, not by a user_id range.
6. `CrowdSim::spawn_actor` returns `CrowdSim::INVALID_ACTOR` on bad input: the tests at `cuda_crowd.rs:1640`, `:2004` should check it or use `try_spawn_actor`.

Also removed from `csg_eval.cuh`: `csg_xform_pt/vec/normal_from_inv`, `csg_intersect_box_direct`, `csg_intersect_prim_direct`. Evaluator overflow now returns a miss (closes `GPU:E01`).

Verify: `cargo test -p nv-demo cuda_crowd -- --ignored --test-threads=1` (all four fail today with the `#error`), then start `nv-demo`.

Known on the csg-rs side: the body of `actor_sim.cuh` has never been compiled past that `#error`; NVRTC errors inside it go back to the csg-rs session (send them the output). `cargo test -p csg-actors --features wgpu test_wgpu_cpu_parity_60_frames` is red there (WGSL footfall differs from CPU); a CUDA-vs-CPU `foot_lock` parity failure would be the same defect, on their side.
