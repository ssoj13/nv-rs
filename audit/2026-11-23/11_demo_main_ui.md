# nv-demo bug hunt: main.rs / ui.rs / app_events.rs / persist.rs / viewport_toolbar.rs

Audit date: 2026-11-23, HEAD `38ec140`. Read-only: no builds were run and no files in the repo were changed.
egui/egui-wgpu/egui-winit 0.36.2, wgpu 30.0.2 and winit 0.30.13 were resolved from `Cargo.lock`. Library behaviour cited below was checked against those exact registry sources.

## 1) Item-by-item ledger

Verdicts: CONFIRMED means the failing path was traced in code. PLAUSIBLE means the trigger needs runtime or hardware to confirm.

### P0

**F01 | P0 | CONFIRMED | main.rs:3561-3593, renderer.rs:1055-1063, renderer.rs:4926-4933**
- Defect: a Viewport "Save PNG" reads `shared_rgba[front_buf_idx]` back to the host. While DLSS-RR owns the beauty output (`dlss_rr_on && rr_ptrs.is_some() && aov_mode == 0 && !split_on && pending_jpeg.is_none()`), the worker skips the CUDA tonemap, so it never writes `shared_rgba` (or `shared_hdr`). The JPEG path forces a tonemap through `pending_jpeg`; the PNG path does not.
- Failing scenario: AI = DLSS RR, AOV = Beauty, Capture = Viewport, click Save PNG. The file holds stale contents: zeros from allocation, or the last frame from before RR was enabled or before the last resize. The status bar still reports "Saved".
- Fix direction: route Viewport PNG through the same source the viewport shows. Copy `rr.display_ldr` (or `dlss.output_out`) into a pitched buffer, as the MP4 path does at main.rs:4687-4737. Alternatively add a `pending_png` flag to the worker, mirroring `pending_jpeg`.

### P1

**F02 | P1 | CONFIRMED | main.rs:4110-4123 (early `return`), main.rs:2998, 4128-4148, 4489-4506, 4672-4677, 4681-4975**
- Defect: `render()` runs the whole egui pass (`full_output`), drains UI commands, and clears `shutter_in_flight` on FrameReady. It then returns early on `Outdated | Lost | Timeout | Occluded | Validation`. That return discards:
  1. `textures_delta.set` and `textures_delta.free`: lost glyph-atlas patches and leaked user textures.
  2. `platform_output`: clipboard copy from the WGS84 pill, cursor icon, open-URL.
  3. The one-shot `ViewportCommand::CursorGrab(Locked)`. This is the trap AGENTS.md documents; `camera_egui` emits it only once.
  4. Full-window PNG/JPEG requests (the `capture_window_*` locals).
  5. The MP4 push for frames received this tick.
- Failing scenarios:
  - (a) Minimize or occlude the window while recording "Smooth 60". FrameReady clears `shutter_in_flight` and the next lockstep batch is dispatched, but no frame is pushed. The deterministic MP4 silently skips simulated time.
  - (b) A frame that hits `Outdated` during a resize drops an egui atlas delta. Text renders garbled until egui rebuilds the atlas.
  - (c) A drag starting on a `Timeout` frame never locks the cursor.
- Fix direction: acquire the surface texture before running egui, or keep the CPU-side work on the early-return path. Always apply `textures_delta`, `platform_output` and the cursor commands, and always run the recorder push from the already-copied GPU buffers. Only the surface render and present should be skipped.

**F03 | P1 | CONFIRMED | main.rs:3565-3569, 3606-3610, 3623-3627, 3637-3641, 3691-3703, 3798, 4759-4764, 4787-4792**
- Defect: every output path is named with a whole-second UNIX timestamp (`nv-demo-{ts}...`) and has no collision check.
- Failing scenarios:
  - Stop and restart a recording within the same second. `FinishVideo` and `StartVideo` are processed in order on the export thread, and the new session reopens the same `nv-demo-{ts}-sdr.mp4`, destroying the clip just finalized.
  - Two PNG clicks within one second: the atomic rename silently replaces the first PNG.
  - Two SDG bursts within one second write into the same `sdg-burst-{ts}` directory, and `manifest.json` is overwritten.
- Fix direction: use millisecond timestamps plus a monotonically increasing counter, or create files with `create_new` and retry with a `-N` suffix. Factor the naming into one helper.

**F04 | P1 | CONFIRMED | main.rs:4829-4855 (Full Window), main.rs:4937-4963 (Viewport), main.rs:1682-1686**
- Defect: in "Live RT" timing, `push_video_frame(.., allow_drop_when_full = true)` returns `Ok(false)` when 4 frames are already in flight. Before the push, `active.last_frame_at = Some(rec_now)` is set unconditionally, and `Ok(_)` treats a dropped frame as success. The dropped frame's interval is therefore removed from the timeline.
- Failing scenario: live recording of a heavy scene where NVENC lags. Every dropped frame shortens the clip, so playback runs faster than real time. The UI promises "Live RT keeps real-time speed".
- Fix direction: on `Ok(false)`, leave `last_frame_at` unchanged so the next frame's duration covers the gap, or carry the dropped duration forward. Update `last_frame_at` only after an accepted push.

**F05 | P1 | CONFIRMED | main.rs:1821-1857, 3107-3112, 3951-3955; ui.rs:1965-1980, 3371-3381, 2353-2373, 2376-2435**
- Defect: while `tod_transition` is `Some`, `TodTransition::step` writes `sun_azimuth`, `sun_elevation`, `sun_intensity`, `sky_intensity`, `exposure`, `sky_mode`, `sky_mode_from` and `sky_blend` every frame, from stored from/to keyframes. It runs after the UI pass. Nothing cancels it when other code changes the same fields:
  - the Background combo, the Sun/Sky/Exposure sliders, and the viewport-toolbar EV;
  - scene buttons, which call `apply_csg_time_of_day_preset` directly (ui.rs:1974);
  - Auto-Drive's scene activation (ui.rs:3380).
- Failing scenarios:
  - Start a 15 s Day to Night transition, then drag Exposure or pick a sky. The control snaps back every frame for up to 15 s, and the transition ends at its own preset.
  - Click a different scene mid-transition. The scene's `default_tod` and lighting are overwritten by the old transition's target, while the UI shows the new scene's TOD button state.
- Fix direction: cancel (`tod_transition = None`) whenever a lighting field is edited, or move TOD preset application into main and route scene-change TOD through `UiCommand::SetTimeOfDay`. Alternatively blend only fields the user has not touched.

**F06 | P1 | CONFIRMED | ui.rs:968-974 vs ui.rs:2426-2435 and ui.rs:4483-4489; egui-0.36.2 drag_value.rs:94/114-145, slider.rs:598-605**
- Defect: the viewport-toolbar "EV" DragValue allows `0.05..=8.0`. The Sky & Lighting "Exposure" control is an `anim_slider_f32` with range `0.25..=2.0`: a `Slider` (default `SliderClamping::Always`) plus a `DragValue::range` whose default `clamp_existing_to_range = true` clamps every frame even without interaction. `sec_sky` defaults open (ui.rs:4147) and is laid out even when scrolled out of view.
- Failing scenario: set EV to 4.0 or 0.1 in the viewport toolbar. In the same or next frame the Render & Floor tab clamps it back into [0.25, 2.0]. Out-of-range values are never published, so the control silently does nothing there.
- Fix direction: one SSOT constant for the exposure range used by both widgets. Search for other dual widgets with mismatched ranges; currently only exposure differs (`csg_lod_bias` matches).

**F07 | P1 | CONFIRMED | persist.rs:181-186, main.rs:2637-2644, main.rs:4978-4981**
- Defect: `load_from` reads with `read_to_string(..).ok()` semantics and parses with `serde_json::from_str(..).unwrap_or_default()`. Any of the following silently resets the whole `PersistedState` (window, camera pose, Custom rail keyframes, theme, all settings):
  - an I/O error other than not-found;
  - truncated or corrupt JSON;
  - an unknown or renamed enum variant (`CaptureSource`, `CameraProjection`, `ProbeBackendKind`, `CrowdBackend`, `SdgOccluderBackend`, `UiTheme`; none have `#[serde(other)]`);
  - a type change.

  About 1.5 s later `save_persisted` writes the defaults over the user's file, because `cur != last_saved_state` once window geometry or the pose differ.
- Failing scenario: an upgrade that renames an enum variant in csg-scene or here permanently wipes all user keyframes and settings, with no log line.
- Fix direction: log the parse error. On failure, rename the bad file to `nv-demo.json.bad-<ts>` before writing. Deserialize section-by-section (`serde_json::Value`, then per-field `from_value` with fallback). Add `#[serde(other)]` fallbacks to persisted enums.

**F08 | P1 | CONFIRMED | ui.rs:1090-1113, main.rs:3965-3977, main.rs:2723**
- Defect: `last_cam_raw_input` is refreshed only inside the camera branch `else if !split_dragging && !toolbar_blocking && !pick_clicked && !pick_active`. When that branch is skipped, the previous `RawInput`, including `keys_down` and `buttons_down`, is kept. Lockstep subframes then replay it through `lockstep_subframe_raw_input` and `cam_driver.update`.
- Failing scenario: during "Smooth 60" recording, fly with W held, move the pointer over the viewport toolbar (or toggle Pick, or drag the A/B split) and release W. The camera keeps flying at full speed in the recorded video until the pointer leaves the toolbar.
- Fix direction: set `*last_cam_raw_input = None` (or a keys-cleared input) whenever the camera branch is skipped. Alternatively build the lockstep `RawInput` from egui input every frame, independent of gating.

**F09 | P1 | CONFIRMED | main.rs:3145-3178, 3790-3794, 4697-4707; ui.rs:828-832; raytracer.cu:1888**
- Defect: with DLSS, `render_w` is rounded to a multiple of 64 and `render_h` to even, independently: `((opt.render_width + 31) & !63)` and `opt.render_height & !1`. The output is `align_render_size(desired, 1.0)`. The shader uses `aspect = width / height` of the render target, so the render and output aspects differ (example: 1920x1080 Balanced gives 1088x626, 1.738 vs 1.778, a 2.3% error). The viewport hides this by drawing the 1920x1080 DLSS texture into a rect with the render aspect. The MP4 Viewport recorder copies `dlss.output_out` or `rr.display_*` at out_w x out_h unchanged.
- Failing scenario: record MP4 with DLSS SR or RR active. Content is horizontally stretched by up to about 3%. The on-screen image is also resampled non-uniformly.
- Fix direction: derive `render_h` from `render_w * out_h / out_w` (rounded to even and clamped), or keep the aspect by choosing out dimensions compatible with the 64-pixel alignment.

**F10 | P1 | CONFIRMED | main.rs:3598-3612 (Viewport JPEG), main.rs:3561-3593 (Viewport PNG), main.rs:4687-4710 (MP4 source); renderer.rs:3284-3287, 5128-5136**
- Defect: Viewport stills do not capture what the viewport shows:
  - JPEG goes to the worker's `pending_jpeg`, which forces the CUDA tonemap of `final_f32`. With RR on, the OptiX denoiser is forced off (main.rs:3118-3120), so the JPEG is the noisy, non-denoised render-resolution frame.
  - With DLSS SR, PNG and JPEG save the pre-upscale low-resolution input.
  - MP4 Viewport correctly captures the DLSS output.
- Failing scenario: DLSS RR on, Save JPEG. The file is grainy and smaller than the viewport.
- Fix direction: one capture-source resolver shared by PNG, JPEG and MP4 (it overlaps with F01).

**F11 | P1 | CONFIRMED | main.rs:1358-1363, main.rs:1561-1575**
- Defect: the `ExportWorker` device pool keeps at most 4 buffers and looks them up by exact `cap`. `recycle_buf` drops a buffer when the pool is full, and `acquire_device_copy` never evicts entries of the wrong size.
- Failing scenario: record at one resolution, or take 4 Full-Window JPEGs, then record at another size. The pool is full of stale-size buffers, so every new frame does `cuMemAlloc_v2`, a D2D copy, and `cuMemFree_v2` on drop. `cuMemFree` is device-synchronizing, so this stalls the render stream once per encoded frame for the rest of the session.
- Fix direction: in `recycle_buf`, replace or evict mismatched-size entries (LRU by size). Alternatively clear the pool on `StartVideo` and size it per session.

**F12 | P1 | CONFIRMED (missing check) / PLAUSIBLE (trigger) | main.rs:2105, 2751-2760, 2775, 3942-4026; renderer.rs:3183-3184, 4985-4989, 5020-5024**
- Defect: the UI never checks `render_thread.is_finished()`. The worker has panicking paths: `expect("cuda bind_to_thread on render thread")`, and `assert_eq!` on `cuLaunchKernel` for the tonemap and scRGB tonemap. If the worker dies:
  - the UI keeps running at 60 Hz with a frozen viewport and no status message;
  - lockstep `shutter_in_flight` stays `true` forever;
  - an active MP4 session stays open with no frames until the user presses Stop;
  - the panic surfaces only at exit, as "render thread join error".
- Failing scenario: a launch failure after a driver TDR or an NVRTC hot-reload of a broken kernel signature. The app appears hung.
- Fix direction: poll `is_finished()` each frame. On death, stop recording, show the panic payload (join it), and disable render commands. Alternatively convert the worker asserts into a `RenderEvent::Fatal`.

**F13 | P1 | PLAUSIBLE (code path CONFIRMED) | main.rs:2382-2388, 3507-3515, 4549-4555; egui-wgpu-0.36.2 renderer.rs:409-413, egui.wgsl:151-162**
- Defect: for the non-sRGB `Rgba16Float` surface, egui-wgpu selects `fs_main_gamma_framebuffer`, which writes texture and vertex colours unchanged. The scRGB OptiX viewport (`shared_hdr`, already linear) is therefore correct. However:
  - the DLSS-SR output (`Rgba8Unorm`, gamma-encoded), the LDR `display_texture` used while `shared_hdr` is not yet allocated, and all egui chrome are written as gamma values into a linear scRGB swapchain;
  - the UI allows View = HDR together with AI = DLSS SR.
- Failing scenario: HDR view with DLSS SR. The viewport is washed out and dim, because SDR white maps to 80 nits and gamma mid-tones are treated as linear. Full-Window PQ10 MP4 records the same wrong values.
- Fix direction: blit or linearize LDR sources into the HDR display texture (sRGB EOTF times SDR-white scale) before egui samples them. Alternatively block DLSS SR in HDR view, as is already done for PQ10 at main.rs:3777-3781. Render egui chrome with an sRGB-to-linear step on the HDR renderer.

**F14 | P1 | PLAUSIBLE | main.rs:3240-3253, 3309-3323, 2825**
- Defect: on every resize, the old `shared_rgba`, `depth`, `motion`, `hdr` and four RR buffer pairs are pushed into `retired_buffers`. They are freed only when a FrameReady carrying the newest `gen_id` arrives. While the worker is blocked (a multi-second CSG/OSM scene build, `RenderControl` not yet polled), each 120 ms throttle tick retains another full set: roughly 0.9 GB per generation at 4K with RR and HDR.
- Failing scenario: start a GundoOSM build and drag a dock splitter or resize the window for a few seconds. VRAM balloons and allocations can fail (`Resize failed: ...`), which then turns RR or HDR off at main.rs:3349-3356.
- Fix direction: have the worker acknowledge `gen_id` (an event, or an atomic in `AppBus`). Generations the worker never acknowledged were never touched by it and can be dropped immediately. Alternatively suppress new resizes while a previous `Resize` is unacknowledged.

### P2

**F15 | P2 | CONFIRMED | main.rs:1798-1803, main.rs:3087-3104**
- Defect: restarting a TOD transition mid-way captures `from_sky_mode = current.sky_mode` (the old target) and ignores the current `sky_blend`.
- Failing scenario: click Night, then Sunset halfway through. The sky pops from a 50% blend to the pure old target.
- Fix direction: when a transition is active, start the new one from the visually dominant mode, or extend the shader blend to three keys.

**F16 | P2 | CONFIRMED | main.rs:3109 and 4065-4072 vs 3949-3990**
- Defect: `time_scale` is applied inconsistently. In Live mode, the TOD transition, `rail_blend_remaining` and the `CameraDamper` tau step with unscaled `dt`, while rail and animation use `dt * time_scale`. In lockstep, all of them use the scaled `sub_dt`.
- Failing scenario: preview at Time Scale 0.25 and record at Time Scale 0.25. The camera smoothing and TOD speed in the MP4 differ from the preview.
- Fix direction: pick one convention (simulation time or wall time) per subsystem and use it in both branches.

**F17 | P2 | CONFIRMED | ui.rs:1939-1963, renderer.rs:618-621, csg_scene_host.rs:2274**
- Defect: the selected scene is persisted as the raw registry index `csg_mode` (1-based), not as `SceneDescriptor::id`. Commit 7b5497a moved labels to the SSOT names but kept the index.
- Failing scenario: a registry reorder or insertion in csg-scenes opens a different scene from the saved config. If the registry shrinks, the host clamps `reg_idx` to the last scene, while the UI (`descs.get(...)` returns `None`) shows no scene selected and hides the CSG panel.
- Fix direction: persist the scene id string (plus variant label or id) and resolve it to an index at startup.

**F18 | P2 | CONFIRMED | main.rs:2620-2635, 1759-1774, 2422-2432**
- Defect: transient runtime overrides are persisted as user preferences:
  - the LiDAR mount auto-switch to `Equirect360` (`possess_saved_projection` is not persisted);
  - mid-transition sun, sky and exposure values;
  - `hdr_display` and `dlss_rr_on` forced to `false` on a session where they are unsupported, for example when Windows HDR is temporarily off.
- Failing scenario: quit while riding a LiDAR mount. The next launch starts in Equirect360 with no way back to the previous lens.
- Fix direction: build `PersistedState` from the user-intent values (saved projection, transition target, the user's original toggles).

**F19 | P2 | CONFIRMED | viewport_toolbar.rs:61-77, persist.rs:158-167, main.rs:2463, 2560**
- Defect: `ToolbarState` derives Serialize/Deserialize and is documented as "Persistent state", and the persist.rs header says "preserves layout". Neither the toolbar edge or expanded state nor the `DockState` layout is part of `PersistedState`.
- Fix direction: add both, with serde defaults, or drop the claims.

**F20 | P2 | CONFIRMED | persist.rs:244-271, persist.rs:289-304**
- Defect: the persist thread spawn result is discarded with `.ok()`. With `handle == None`, `enqueue` falls back to a synchronous save, but `flush_and_join(Some(final_state))` only stores the state in the slot and returns, so the final state is never written.
- Fix direction: in `flush_and_join`, when there is no handle, call `save()` synchronously, and log the spawn error.

**F21 | P2 | CONFIRMED | main.rs:7, main.rs:9**
- Defect: the module-level allows on `camera_core` (about 16.5k LOC) and `camera_egui` (1.27k LOC) hide real problems:
  1. `unexpected_cfgs` hides that `feature = "serde"` is not declared in nv-demo's Cargo.toml. All camera_core serde derives are compiled out, along with its serde tests (mode.rs:405, prefs.rs:589/619, settings.rs:315/338). That is why persist.rs hand-writes stringly-typed `nav_*_name/parse_*` maps (persist.rs:313-374), and ui.rs duplicates the labels (ui.rs:145-164).
  2. `dead_code` and `unused_imports` hide unused vendored API.
  3. `clippy::all` disables every lint on about 17.7k LOC.
  4. `deprecated` on `camera_egui` hides deprecated egui API usage. egui 0.37.0 is already in the local registry, so this is the next upgrade break.
- Fix direction: declare a `serde` feature (or depend on camera_core as a crate), replace the string maps with derived serde, and narrow each allow to `#[expect(.., reason)]` at item level.

**F22 | P2 | CONFIRMED | ui.rs:4461, ui.rs:4152-4167**
- Defect:
  - `#[allow(clippy::too_many_arguments)]` on `anim_slider_f32` violates the repo rule (use `#[expect]` or a parameter struct). Its trailing `decimals: usize, log: bool` positional pair is error-prone; `prop_slider_u32` repeats the bare `log: bool`.
  - `collapsing_section`'s `_default_open` parameter is ignored. Every caller passes `true`, but sections such as `sec_transform`, `sec_detail`, `sec_dof`, `sec_floor`, `sec_scene`, `sec_bloom` and `sec_split` actually default to closed via `section_default_open`. This is dead, misleading API.
- Fix direction: remove the parameter (or honour it), and use an options struct for the slider helpers.

**F23 | P2 | CONFIRMED | per-frame allocation sites in always-visible UI**
- Allocation sites, each run every frame:
  - toolbar `format!` at ui.rs:631, 646, 667, 704/708, 732/734, 807;
  - viewport toolbar `format!` at ui.rs:882, 897;
  - `status_buf.clone()` at ui.rs:1328, which defeats the scratch buffer;
  - WGS84 `pill_text` `format!` at ui.rs:1357/1359;
  - `label.to_owned()` in every `prop_row` (ui.rs:4378), dozens per frame;
  - `format!("{caret}  {title}")` per section (ui.rs:4253);
  - `format!` in `cycle_toggle_btn` (ui.rs:4292);
  - Path Tracer button labels at ui.rs:2831, 2970, 2987, 3004, 3020-3024, 3082/3084, 3107, 3127, 3145, 3287;
  - CSG labels at ui.rs:2196/2198, 2214, 2244;
  - Cinema labels at ui.rs:3326/3328, 3409, 3437, 3463, 3488, 3503, 3631;
  - `Vec` allocation per sparkline (ui.rs:4637);
  - `let prev_lod = *s;` copies the whole `DemoSettings` every frame (ui.rs:2313).
- This contradicts the "zero steady-state churn" claim for `UiScratch`.
- Fix direction: `LazyLock` static labels for icon+text constants, `fmt_into` for dynamic labels, `Arc<str>`/`WidgetText` caching, and `layout_no_wrap(status_buf.as_str().into())`.

**F24 | P2 | CONFIRMED | duplicated constants and widgets (SSOT)**
- Duplicates found:
  - `AOV_LABELS` vs `AOV_HUD_TAGS`, two parallel 11-entry arrays (ui.rs:24-50).
  - Three AOV selectors: toolbar (ui.rs:676-685), viewport toolbar (ui.rs:877-892), Path Tracer cycle (ui.rs:2851-2867).
  - The lens list is hard-coded at ui.rs:904-909 while the main toolbar uses `CameraProjection::ALL` (ui.rs:692).
  - Sky-mode labels (ui.rs:2353-2363) vs preset ids 0/1/8 (ui.rs:3905/3913/3921).
  - The "Inspector 109" rule appears 3 times (ui.rs:949-952, 1039-1042, 2192).
  - `scene_wgs84_origin` hard-codes scene ids and coordinates that belong to `SceneDescriptor` (ui.rs:71-78).
  - The DLSS quality table (ui.rs:2924-2930) vs `dlss_quality_from_ui` (main.rs:65-75).
  - The `"renders"` literal appears 7 times.
  - The fixed fps `60` appears at main.rs:2604, 3737, 3822, 3944.
  - The rail blend constant `1.6` appears at main.rs:2862, 3925, 3980, 4066.
  - The shutter clamp `1..=4` appears at main.rs:2609, 3917 and in ui.rs `SHUTTER_MODES` (3256).
  - The `time_scale` clamp `0.05..=2.0` appears at main.rs:3915, renderer.rs:3855 and ui.rs:3570.
- Fix direction: move these into typed constants or methods on `DemoSettings` and `SceneDescriptor`.

**F25 | P2 | CONFIRMED | ui.rs:1955, 1965-1980, 2260-2265 vs ui.rs:3371-3384**
- Defect: scene switching has divergent paths:
  - The scene buttons emit `SelectScene`, clamp the variant, apply the TOD preset, and main then ejects possession.
  - "Auto-Drive: OFF (Start)" with Classic sets `s.csg_mode = 1` directly. It emits no `SelectScene` (so no `EjectPossess`) and does not clamp `csg_asset_idx` to the scene's variants.
  - The "Off" button emits nothing either, so there is no `EjectPossess` while possessed.
- Fix direction: route every scene change through one `UiCommand::SelectScene` handler in main that applies the TOD, variant clamp and eject.

**F26 | P2 | CONFIRMED | main.rs:2199, 2206-2211, 2232-2253, 2291, 2452-2455**
- Defect: startup errors are swallowed:
  - `required_extensions().ok()`;
  - `create_vulkan_instance_with_extensions(..).unwrap_or_else(|_| ..)`;
  - `open_vulkan_with_extensions`/`open_vulkan` errors discarded, ending in `expect("Vulkan+CUDA interop adapter")` with no cause;
  - `is_dlss_rr_available().ok()`;
  - NVML init and device errors.
- Failing scenario: NGX instance-extension failure shows the generic "NGX Vulkan extensions unavailable on this host/adapter" message instead of the real reason.
- Fix direction: keep the first error per stage and surface it in `ngx_init_error`, or panic with it in the expect message.

**F27 | P2 | CONFIRMED | main.rs:4249-4255, 4398-4404, 4743-4747, 4880-4884**
- Defect: `device.poll(PollType::wait_indefinitely())` blocks the UI thread every DLSS/RR evaluation and every recorded or window-captured frame, with no timeout. A GPU hang or TDR hangs the UI forever, and the full queue drain costs frame time on every RT frame.
- Fix direction: use a timeout poll, or a submission-index wait for only the copy submission; report failure and disable the feature.

**F28 | P2 | CONFIRMED | main.rs:1255-1258 vs persist.rs:204-205**
- Defect: the PNG "atomic" writer flushes the `BufWriter` but never calls `sync_all()` before rename, while persist.rs does. The atomic-write contract differs between the two, so a power loss can leave a zero-length PNG under the final name.
- Fix direction: share one `atomic_write` helper.

**F29 | P2 | CONFIRMED | ui.rs:1034-1047 vs ui.rs:1115-1120**
- Defect: the `P` (poke) hotkey is gated only by `kb_free && csg_mode > 0`, not by `viewport_focused` as `F` and `A` are. It fires while the pointer is over side panels.
- Fix direction: gate it on `viewport_focused`, like the other viewport hotkeys.

**F30 | P2 | PLAUSIBLE | main.rs:2833-2858, main.rs:3855-3861**
- Defect: `pending_scene_auto_frame` is armed on every `SelectScene`. If the user switches A to B to A before the worker consumes the snapshot (StateSlot coalesces), the worker sees no key change and emits no metadata, so the flag stays armed.
- Failing scenario: a later Road Mode switch rebuilds the same scene key and emits `SceneMetaUpdated(A)`. The camera unexpectedly teleports to viewpoint 0.
- Fix direction: clear the flag when the snapshot is consumed with no key change, or tag requests with a sequence id echoed in the metadata.

**F31 | P2 | CONFIRMED | crates/nv-demo/AGENTS.md:41, 43; repo AGENTS.md "Audit traps"**
- Defect: doc drift. The traps for the `rr_last_exposure` NaN sentinel ("main.rs:1770, 3468-3470") and the `CursorGrabController` one-shot ("main.rs:1189-1348") cite line numbers and states that no longer exist. The code now uses `Option<f32>` (main.rs:1114-1123) and `pending_grab` (main.rs:2022-2025).
- Fix direction: mark both traps as fixed, with current references.

**F32 | P2 | CONFIRMED | main.rs:3561-3564, main.rs:3598-3601**
- Defect: Save PNG or JPEG with Viewport capture and `!has_frame`, for example during the first frames or right after a resize, silently does nothing. No status message is shown.
- Fix direction: report "No frame yet", or defer the request until the next FrameReady.

## 2) Files inspected

| File | Lines | Read fully |
| --- | --- | --- |
| crates/nv-demo/src/main.rs | 5481 | Yes, in 12 chunks |
| crates/nv-demo/src/ui.rs | 4930 | Yes, in 10 chunks including tests |
| crates/nv-demo/src/app_events.rs | 557 | Yes |
| crates/nv-demo/src/persist.rs | 505 | Yes |
| crates/nv-demo/src/viewport_toolbar.rs | 418 | Yes |
| AGENTS.md, crates/AGENTS.md, crates/nv-demo/AGENTS.md | n/a | Yes |
| crates/nv-demo/Cargo.toml | n/a | Yes (features check) |
| crates/nv-demo/src/renderer.rs | about 5300 | No. Contract checks only: 62-68, 165-232, 520-760, 1053-1063, 3150-3880 (worker loop, controls, shutter queue, snapshot handling), 4920-5050 (tonemap skip), 5128-5140 (JPEG) |
| crates/nv-demo/src/csg_scene_host.rs | n/a | No. Grep for `scene_idx`, plus 2255-2294 |
| crates/nv-demo/src/camera_rail.rs | n/a | No. 376-435 and 530-690 (serde skip, LUT fallback) |
| crates/nv-demo/shaders/raytracer.cu | n/a | No. 1075-1087 (sky blend), grep for aspect (line 1888) |
| camera_core/*, camera_egui.rs | about 17.7k | No. LOC count, cfg/feature grep and git log only |
| appbus-rs/src/bus/queue.rs | n/a | No. 325-370 (`emit` evicts the oldest half on overflow) |
| egui-0.36.2 slider.rs and drag_value.rs | n/a | No. Clamping semantics only |
| egui-wgpu-0.36.2 renderer.rs and egui.wgsl | n/a | No. Fragment entry selection and shaders only |
| nv-wgpu src/buffer.rs | n/a | No. Grep for Drop impls and SharedBuffer fields |
| cuda-core context.rs | n/a | No. Grep only: `new_stream` uses `CU_STREAM_NON_BLOCKING`, so the UI-thread sync copies do not serialize with the render stream |
| git history | n/a | Diffs of d942935 (main.rs) and 7b5497a (ui.rs); stats of 44e634b, 6cff3a9 and 38ec140 |

## 3) Files changed

None. No repository files were created, edited or deleted. No cargo, build or test commands were run. Git usage was limited to `git show` and `git log`.

## 4) Static and invariant verification performed

- **Command coverage:** every `UiCommand` variant is constructed in ui.rs and handled in the main.rs match at 3035-3105. Every `RenderControl` variant is constructed in main.rs and handled in renderer.rs. Every `RenderEvent` variant is emitted by the worker or the export thread and consumed at main.rs:2769-2928. All `EnginePhase` variants are emitted (renderer.rs:3201, 3203, 4144, 4269, 4441).
- **Lockstep desync:** `record_fixed_fps`, `record_shutter_subframes`, `record_video_mode` and Capture are all UI-disabled while recording (ui.rs:3183, 3225, 3246, 3261). The worker's `rec_fixed_fps` is latched only at `SetRecording`, so the guard is UI-only; see section 5.
- **Lockstep FrameReady stalls:** the worker publishes only with a free buffer, and the UI holds exactly one. At most one FrameReady can arrive per UI tick, so lockstep frames cannot be coalesced or dropped by the double-buffer protocol. The only drop paths are F02 (early return) and F12 (worker death).
- **DLSS-RR tonemap gating:** `should_skip_cuda_tonemap` (renderer.rs:1055-1063) was cross-checked against the PNG readback source; this produced F01.
- **egui clamping:** confirmed in the egui 0.36.2 source that `DragValue::range` defaults to `clamp_existing_to_range = true` and `Slider` defaults to `SliderClamping::Always`; this produced F06.
- **egui-wgpu shader selection:** confirmed `fs_main_gamma_framebuffer` for non-sRGB `Rgba16Float` with no linearization of textures; this produced F13.
- **appbus overflow:** `EventBus::emit` evicts the oldest half on overflow, with capacity 256. FrameReady volume is bounded at 2 by buffer ownership, so there is no current overflow path. Eviction would be silent; see section 5.
- **Serde:** `DemoSettings`, `PersistedState`, `WindowState`, `CameraState`, `CameraRail` and `DemoAnimator` are all container-level `#[serde(default)]`. Persisted enums have no `#[serde(other)]`, and the persist `load_from` fallback discards everything (F07).
- **Drop order:** `DemoApp::drop` finishes the video, shuts down and joins the export worker, flushes persistence, then sends Shutdown and joins the render worker before any field drops. `dlss_*`, then `ngx_vulkan`, then `interop` drop in declaration order. `InteropContext` has no Drop impl and `SharedBuffer` owns its own Arc and handles, so the later drop of `shared_*` after `interop` is not a defect.
- **CursorGrabController:** traced press, threshold, lock and release, and the focus-loss and persistent-fly paths. The deferred `pending_grab` correctly replaces the earlier one-shot trap. Its only remaining loss path is F02.
- **Pacing:** `compute_next_ui_tick` together with `about_to_wait` has no catch-up burst. An OS-triggered early redraw resets to `now + interval`. Minimize does not shrink the render targets, because egui keeps the previous `screen_rect` when the size is 0.
- **NaN/zero guards checked:** `render_h.max(1)`, `dt.clamp(1e-4, 0.1)`, `tan_half_fov.max(0.05)`, `max_accum` (the UI range starts at 1; the renderer guards 0), `TodTransition` duration clamp `0.1..=15`, `SavedPose::to_pose` finiteness, `WindowState::sanitized`.

## 5) Risks and open seam questions for other slices

- **renderer.rs (worker):**
  - Convert the `assert_eq!`/`expect` paths at renderer.rs:3183, 4985 and 5020 into reported errors (F12).
  - `rec_fixed_fps` is latched only at `SetRecording`. If anything ever mutates `settings.record_fixed_fps` while recording (a preset, `demo_anim`, a future command), the UI and worker desync: either the `shutter_queue` grows without bound (UI sends batches while the worker is not in shutter mode) or the worker waits forever on an empty queue. Today the guard is a disabled widget only.
  - Should a viewport PNG be added as `pending_png`, mirroring `pending_jpeg` (F01/F10)?
- **renderer.rs `DemoSettings` serde:** the field-level `#[serde(default = "default_*")]` functions duplicate `impl Default` values; verify both agree (SSOT). Add `#[serde(other)]` or tolerant deserialization for `CameraProjection` and `CaptureSource`, and for the csg-scene `ProbeBackendKind` plus sdg_export `CrowdBackend`/`SdgOccluderBackend` that are persisted through `DemoSettings` (F07).
- **csg_scene_host.rs:**
  - Does unloading a scene (csg_mode set to 0 by the "Off" button, F25) eject the possessed body and emit `PossessChanged(None)`? Main does not send `EjectPossess` on that path.
  - Out-of-range `scene_idx` is silently clamped to the last scene (csg_scene_host.rs:2274) while the UI shows none (F17).
- **gpu_export.rs:** do `VideoRecorder::push_frame` and `save_jpeg_from_pitched` finish reading `buf.ptr` before returning? The export thread recycles the `OwnedDeviceBytes` right after the call (main.rs:1447, 1513). If NVENC or nvJPEG read asynchronously, a recycled buffer could be overwritten by the next D2D copy mid-encode. Also confirm `VideoRecorder::start_with_format` truncates existing files (F03 data-loss path).
- **appbus-rs:** `EventBus::emit` silently evicts half the queue on overflow. If a FrameReady were ever evicted, its buffer would never be returned (`ReturnBuffer` is only sent for received frames), and the worker would stall permanently with `free_bufs` empty. Consider `try_emit` with an error for FrameReady, or a dedicated slot.
- **camera_core / camera_egui owners:** declare the `serde` feature and remove the module-wide `#[allow(..., clippy::all)]` (F21). Is the `deprecated` allow masking egui 0.36 APIs that are removed in 0.37?
- **renderer.rs `ReloadShader`:** uses the CWD-relative path `crates/nv-demo/shaders/raytracer.cu`, and all exports use CWD-relative `renders/`. Running the binary from another directory breaks hot-reload and scatters outputs. Should one base path come from `current_exe` or the workspace root?
- **DLSS sizing (F09):** whoever owns `align_render_size`/DLSS policy should decide whether the output dimensions or the render height adapt to keep the aspect.
- **nv-wgpu / NGX:** `DlssPass::evaluate` returns early (`?`) from inside the Vulkan command closure after some image barriers were recorded. Confirm that `submit_vulkan_commands` discards the command buffer on `Err`, otherwise image layouts may be left inconsistent (TransferDst vs ShaderRead) for the next evaluation.
