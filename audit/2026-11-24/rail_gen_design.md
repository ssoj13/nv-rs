# Camera rail generator: root causes and redesign (`RAIL:GEN`)

Date 2026-11-24. Read-only investigation against nv-rs `28a3598` (dirty tree) and csg-rs `fa5e170`
(the `Cargo.lock` pin; `rail_gen.rs`, `rail.rs`, `view_score.rs` and `csg-scene-gundo` are unchanged in
local csg-rs `HEAD` `6e84ff6`). All csg-rs paths below are relative to
`~/.cargo/git/checkouts/csg-rs-3a6ed3e588aad103/fa5e170/`. Nothing was built or run. Every claim about
the gundo trajectory comes from reading the code and the scene constants. Milestone M0 checks the
claims numerically before any code changes.

## 1. Every rail code path today

```text
scene build (csg-rs, render worker)                          nv-demo
---------------------------------------------------------    -----------------------------------------
csg-scene-gundo build_gundo_scene                            csg_scene_host.rs:2422 SceneLoadedMetadata.rails
  hints (2 road spines, 3 landmarks, 1 corridor,               -> main.rs:3503 CameraRail::set_scene_rails
         1 ocean AABB, 9 walkways)                                 camera_rail.rs:441-476 (auto-selects rails[0]
  viewpoints (4 curated)                                           = road_cruise, duration from spec)
  generate_default_rails  (csg_scene_gundo.rs:1644)          UI "Random Rail" -> RenderControl::GenerateRandomRail
    RoadCruise / LandmarkSlalom / UrbanCanyon / GrandTour      renderer.rs:4159 -> csg_scene_host.rs:1230
    rail_gen.rs:2603-2619 -> generate_rail 2564                  -> rail_gen::generate_random_rail 2698
      seed_* -> relax_with_altitude_retry 2724                   (runs synchronously on the render thread)
        -> relax_rail_against_world 530                      UI "Smart Rail" -> RenderControl::RunSmartRail
           (26-ray stencil + chord ray, Laplacian,              renderer.rs:4307 -> csg_scene_host.rs:1244
            subdivision, roof escape)                            -> rail_gen::build_smart_rail_candidates 2656
        -> recompute_arc_lookahead_targets 829                   -> 8 samples/rail scored by score_views_on_rig
        -> steer_occluded_targets 972                               (renderer.rs:3234, OptiX __raygen__score_views
        -> verify_rail_with_collider 370                            raytracer.cu:2785 / WGPU / CPU)
                                                                 -> best average composite wins (4333-4350)
playback: csg-scene rail.rs CompiledRail::eval_at_fraction 278 UI "Smart View" -> SmartViewSelector::
  (arc-length LUT, 7-tap gaze filter, look blended toward        generate_candidates (view_score.rs:549) + same scorer
   flight direction in eval_stabilized_look 696, banking)     Custom keyframes: camera_rail.rs:648-731 (own
                                                                 Catmull-Rom + SQUAD + 256-sample LUT)
bench: bench.rs `--bench-rail <id>` replays one scene cassette in lockstep (A/B performance path).
```

The `Custom` deck (`camera_rail.rs`) and the `CompiledRail` evaluator are sound and are kept as they
are. Every defect is in how the waypoints are chosen, ordered, timed and aimed.

## 2. Root causes

### Gundo geometry (from `scenes/csg-scene-gundo/src/csg_scene_gundo.rs`)

Default params (`:76-82`): 28 segments x 32 m, so `z` spans [-448, 448]. `lanes_per_dir = 2`, which
gives `half_total_road = 13.1` and `seawall_x = -19.6` (`:153-162`).

| Content | x range (m) | Source |
| --- | --- | --- |
| Ocean tiles, `semantic::WATER`, top `y = -3.2` | [-359.6, -39.6] | `:224-236`, `:670-676` |
| Beach, `GROUND`, top `y = -3.1` | [-55.1, -21.1] | `:237-249`, `:663-669` |
| Promenade / road / sidewalks | [-23, +13] | `:1167-1242`, `:1278-1288` |
| Refinery pads, `GROUND` | [13.1, 205.1] | `:250-262`, `:677-683` |
| Landmarks (Hortonspheres, towers, tank farm) | x = 39, 75, 135 | `:1243-1277` |
| `WaterZone` "Pacific Ocean" (AABB) | [-399.6, -39.6] | `:1289-1296` |

`world_bounds` therefore spans about x in [-360, 205]. **Its center (x of about -77) lies over open
ocean.** All navigation polylines (2 road spines, 1 corridor, 3 sidewalks) run north-south inside a
35 m band, x in [-23, +12]. The only east-west links are 6 crosswalks, each about 22 m long
(`:1199-1209`).

### R1. Generated stations land in the ocean

1. **The scenic overlook ring is centered on `world_bounds.center()` without any land test.**
   `rail_gen.rs:2358-2387` places 4 `Overlook` spots on an ellipse around the bounds center. On
   gundo that ellipse is `rx = 0.58 * 282 = 164` m around x = -77, so about 2 of the 4 spots sit at
   x < -39.6, over the ocean. Their altitude is `floor_y + 16..24`.
2. **The diversity selector prefers exactly those spots.** In `rail_gen.rs:2048-2058`, `utility`
   is `(score + kind_bonus) * (0.5 + 0.5 * spread) + jitter`. Here `kind_bonus = +26` for an unseen
   kind (`Overlook` is a unique kind), and `spread` rises to 3x with the distance to already
   selected spots. On a north-south strip, the only candidates that spread in x are the ocean
   overlooks. An ocean overlook reaches `(54 + 26) * 2 = 160`, while a street spot reaches about
   `100 * 1.25 = 125`. Farthest-point diversity on a coastal scene picks the sea.
3. **Water is a floor, not a forbidden region.** `SceneHints::floor_y`
   (`crates/csg-scene/src/scene_module.rs:687-700`) only raises the floor to `surface_y`. On gundo
   that floor is `max(0, -3.2) = 0`. Relaxation (`rail_gen.rs:549-555`, `:605-612`) and verification
   (`:370-460`) only test solid clearance. A camera 20 m above open water is "clear", so nothing
   rejects it.
4. **Other unmasked domains.**
   - Landmark approach points are clamped to `world_bounds`, not to land (`rail_gen.rs:2143-2152`).
   - `build_fallback_orbit` orbits the bounds center (`:3355-3382`), which is in the ocean.
   - Non-street legs bow "outward from scene center" (`:2522-2536`), and the center is in the
     ocean.
   - `SmartViewSelector::generate_candidates` orbits landmarks at random 360° azimuths and radii up
     to `2.6 * hero_radius + 12` (`view_score.rs:564-578`). For the Hortonspheres that reaches
     x = -61, over the ocean.
   - `generate_random_rail` mode 0 passes `&[]` for the curated viewpoints (`rail_gen.rs:2708`), so
     the 4 hand-framed gundo viewpoints are dropped.
5. **The scorer does not penalize water.** `score_probe_grid` (`view_score.rs:336-349`) rewards
   semantic entropy and depth stratification. An ocean plane under a low camera gives graded
   depths and adds a WATER class, so sea-facing views score well. Smart Rail ranks rails by the
   average of this composite over 8 samples (`renderer.rs:4314-4350`), so it cannot reject a sea
   rail either.

### R2. The loop flies along the shore, turns sharply and flies back

1. **The scene graph is a ladder.** `StreetNavGraph::from_hints` builds parallel north-south rails
   35 m apart, joined only by about 22 m crosswalk rungs. On such a graph every closed street cycle
   goes up one rail, crosses a rung, and comes back down a rail 11 to 23 m away.
   - `build_street_network_loop` rejects cycles whose node span is below `min_minor_span = 36 m`
     (`rail_gen.rs:1682`, `:1858-1867`). On gundo the band is about 31 to 35 m wide, so the cycles
     are either rejected or, if one passes, they are exactly this out-and-back ladder.
   - If they are rejected, `seed_road_cruise` falls back to `select_diverse_tour_spots` plus
     `build_tour_polyline` (`:2753-2757`).
   - `route_leg_along_streets` (`:1562-1638`) then snaps every leg longer than 14 m onto the same
     band (`snap_r` reaches 80 m). Its edge-reuse penalty (`1 + 2.5 * uses`, `:1584-1587`) only
     moves the return leg onto the neighbouring parallel sidewalk, 11 m away.
   - The 2-opt TSP (`:2410-2479`) runs on stations spread along a 900 m line. The optimal closed
     tour of near-collinear points is out-and-back.
2. **The duration is fixed and ignores path length.**
   - `RailArchetype::default_duration_s` is 22 to 34 s (`rail_gen.rs:75-84`).
   - The smart rail uses `22 + 1.8 * stops`, clamped to 24..40 s (`:2638`).
   - `set_scene_rails` copies that duration (`camera_rail.rs:470`).
   - A loop that covers the 848 m walkway band twice is about 1.7 to 2 km long. At 24 s that is
     70 to 80 m/s.
3. **The U-turn radius is tiny for that speed.** `soften_hairpin_turns` replaces reversals with an
   arc of radius 8 to 14 m (`rail_gen.rs:3307`), and `bevel_sharp_corners` cuts 6.5 m corners
   (`:2560`). At 75 m/s over 14 m the yaw rate is `v/R`, about 5 rad/s (about 300°/s). Nothing
   limits turn rate, pitch rate or curvature against speed. The 7-tap gaze filter
   (`rail.rs:303-327`) covers only ±3 x `5/L` of arc fraction, a fraction of a second at that speed.
4. **The view always faces forward, so the return leg mirrors the outbound leg.**
   - `recompute_arc_lookahead_targets` overwrites every target with the integrated flight tangent.
     It blends in landmarks only within a ±35° cone and with a weight of at most 0.25
     (`rail_gen.rs:829-960`, `:903`).
   - `eval_stabilized_look` then fades any target that is not ahead back toward the velocity vector
     (`rail.rs:696-718`). A 90° side look keeps only about half its weight.
   - The outbound and return legs therefore show the same corridor in mirror image, which makes the
     loop dull after the first lap.
5. **No test catches it.** `parallel_coastal_gundo_layout_crosses_between_highway_promenade_and_refinery`
   (`rail_gen.rs:3871-4024`) asserts `dx >= 20 m, dz >= 75 m` and ">= 4 locations 35 m apart". An
   out-and-back pass 20 m wide satisfies both. There is no test for water, retracing, turn rate or
   speed.

### R3. Secondary defects (fix in passing)

- `GenerateRandomRail` and `RunSmartRail` run the full relax on the render thread
  (`renderer.rs:4159-4172`, `:4307-4390`). With 24 iterations x 256 samples x 27 rays, every press
  stalls the frame loop.
- `CpuViewProber::trace_primary` loops over every instance without a spatial index
  (`view_score.rs:426-455`). This is acceptable for a 64-view Smart View, but too slow for scoring
  hundreds of candidates.
- `WaterZone` is an AABB. In the OSM path the "Pacific Ocean" AABB is padded by 1600 m
  (`crates/csg-citygen/src/nodes/terrain_grade.rs:219-224`), so for diagonal coasts it covers land
  and cannot serve as a land/water mask.

## 3. Research: how others build fly-throughs and score views

Each entry gives the source and what applies to us. Search date: 2026-11-24. Several summaries come
from abstracts or citing papers rather than full texts; these are marked "(abstract-level)".

### 3.1 Virtual camera planning and cinematography

- **Christie et al., "Virtual Camera Planning: A Survey"**
  (<https://research.monash.edu/en/publications/virtual-camera-planning-a-survey/>) and the
  SIGGRAPH Asia 2009 course "Camera Control in Computer Graphics"
  (<https://history.siggraph.org/wp-content/uploads/2025/07/2009-SA_Courses_Christie_Camera-Control-in-Computer-Graphics.pdf>).
  They frame the problem as screen-space properties (visibility, framing, size, angle) solved as
  constraint and optimization problems over camera pose. Applies: our view score is a set of
  screen-space properties measured from probe AOVs, not a placement heuristic.
- **Dehghanian et al., "Camera Trajectory Generation: A Comprehensive Survey" (2025)**
  (<https://arxiv.org/abs/2506.00974v1>). Covers rule-based, optimization-based and learned
  trajectory generators and their metrics. Applies: the split of the problem into viewpoint
  quality, path feasibility and sequence-level metrics.
- **Lino and Christie, Toric space (SIGGRAPH 2015)**
  (<https://history.siggraph.org/?p=112722>, <https://sites.google.com/site/christophelino/research/toric-space>).
  Reduces two- or three-subject framing to 3 DoF with exact on-screen subject positions. Applies:
  for landmark stations, place the camera so that the landmark and a second subject (street,
  coastline) land on rule-of-thirds positions analytically, instead of by random search.
- **Oskam et al., "Visibility Transition Planning for Dynamic Camera Control" (SCA 2009)**
  (<https://diglib.eg.org/items/7995924b-fa69-4263-9783-e2cfb1bb24b7>). Fills free space with
  spheres, connects overlapping spheres into a roadmap, precomputes sphere-to-sphere visibility by
  stochastic ray casting, plans coarse paths on the roadmap and refines them. Applies directly: our
  free-space roadmap is a sphere graph seeded on the distance-field ridges (4.11), and visibility
  between nodes comes from exact CSG rays.
- **Salomon et al. (2003)**, cited in the Oskam paper and in
  <https://users.dimi.uniud.it/~roberto.ranon/slides/eg2016/camera-motions.pdf>. Collision-free
  camera paths come from a global probabilistic roadmap plus graph search.
- **Jovane, Louarn, Christie, "Topology-aware Camera Control for Real-time Applications" (MIG 2020)**
  (<https://hal.archives-ouvertes.fr/hal-02969056v1>, abstract-level). Builds camera roadmaps from
  the environment's topology rather than random samples, then picks cameras and trajectories by
  artistic rules. Applies: a skeleton (medial-axis) roadmap of free space instead of a PRM.
- **Geraerts, medial-axis-retracted roadmaps**
  (<https://webspace.science.uu.nl/~gerae101/pdf/hqrJournal.pdf>). Retracting roadmap paths to the
  medial axis yields high-clearance paths in real time. Applies: corridor following through street
  canyons and under bridges by moving along the free-space skeleton.
- **Galvane et al., "Directing Cinematographic Drones"** (<https://arxiv.org/pdf/1712.04216>) and
  **Nägeli et al., SIGGRAPH 2017** (<https://people.csail.mit.edu/jalonsom/docs/17-naegeli-siggraph.pdf>).
  These enforce cinematographic properties along physically feasible quadrotor paths. Applies: hard
  limits on velocity, yaw rate and acceleration, the same limits we put in the metric gates.
- **Joubert et al. (SIGGRAPH Asia 2015), Roberts and Hanrahan (2016), Gebhardt et al. "Airways"
  (2016)**, summarized in <https://ar5iv.arxiv.org/html/1808.09563> and
  <https://history.siggraph.org/?p=102454> (abstract-level). They take keyframes, possibly
  infeasible, and optimize time-parametrized trajectories into dynamically feasible, smooth ones.
  Applies: stage E's feasibility pass (curvature against speed, jerk-limited altitude), and the
  optional speed profile (R-A4).
- **Leake et al., "Computational Video Editing for Dialogue-Driven Scenes" (SIGGRAPH 2017)**
  (<https://graphics.stanford.edu/papers/roughcut>). Editing idioms ("start wide", "avoid jump
  cuts") are composable costs over a shot sequence, optimized globally. Wu et al. model idioms as
  finite state machines (abstract-level, via <https://core.ac.uk/download/195749523.pdf>). Applies:
  our tour is a sequence of shot types whose order is optimized with idiom-like rules (4.8, 4.9).

### 3.2 Viewpoint quality and interest

- **Vázquez et al., viewpoint entropy (VMV 2001)**
  (<https://www.cs.ubc.ca/nest/imager/tr/2001/vazquez.2001a/vazquez.2001a.pdf>). The best view
  maximizes the entropy of relative projected areas. A greedy selection then picks a small set of
  covering views (<https://dblp.uni-trier.de/rec/journals/cgf/VazquezFSH03.html>). Applies: our
  "diversity" term is entropy over projected screen coverage of semantic classes and distinct
  objects, and station selection is greedy coverage.
- **Bonaventura, Feixas, Sbert et al., survey of viewpoint selection measures (Entropy, 2018)**
  (<https://doaj.org/article/b69dfdae5e604cf0a9399e652ecafb89>). Compares many measures in one
  framework and notes there is no single agreed definition of a good view. Applies: combine
  several measures, and fit their weights to operator preferences rather than hard-coding one.
- **Sokolov and Plemenos, "High level methods for scene exploration" (JVRB 2006)**
  (<https://www.jvrb.org/jvrb/article/download/3.2006.12/pdf>). Viewpoint quality weights object
  importance, not only geometric visibility, and drives an offline exploration path. Applies:
  per-object importance weights in the interest term.
- **Andújar, Vázquez, Fairén, "Way-Finder" (CGF 2004)**
  (<https://diglib.eg.org/handle/10.2312/CGF.v23i3pp499-508>). Guided tours through walkthrough
  models: entropy-based relevance over a cell-and-portal graph picks the cells to visit and where
  the waypoints go. **Elmqvist et al. (VRST 2007)** (<https://www.cse.chalmers.se/~tsigas/papers/TourGen-VRST07.pdf>)
  do the same on a voxelized world. Applies: a tour over a spatial partition (our column map and
  skeleton), with stations chosen by view relevance.
- **"Automatic Fly-through Camera Animations" (2014)**
  (<https://scitepress.org/Papers/2014/46703/pdf/index.html>). Camera path extraction for
  arbitrary 3D models, using viewpoint quality.
- **Learned aesthetics: NIMA and its reimplementation** (<https://arxiv.org/html/2502.01873v1>),
  **Wang et al., learning to compose by pairwise view preference** (<https://arxiv.org/pdf/1702.00503v1.pdf>),
  **Datta-style hand-crafted composition features** (rule of thirds and others, cited in the same
  paper). Applies: composition rules as measurable features on our AOVs, and pairwise operator
  preferences as the tuning signal. A learned model on RGB is offline-only for us (section 6).
- **Aesthetic 3DGS field (2026)** (<https://arxiv.org/html/2602.20363>) and **SplaTraj (2024)**
  (<https://arxiv.org/html/2410.06014v1>). Distill a 2D aesthetic model into a 3D field, or optimize
  a trajectory through a differentiable renderer for requested content. Applies conceptually: score
  first, coarse to fine. We have no differentiable renderer and need no learned field, because
  exact analytic probes are cheap.
- **ACDC, "Agentic Aerial Cinematography" (2025)** (<https://arxiv.org/html/2509.16176v1>).
  Vision-language retrieval picks initial waypoints, then preference-based Bayesian optimization
  refines poses with aesthetic feedback. **ShotBench** (<https://arxiv.org/html/2506.21356v2>)
  shows that even the best of 24 VLMs is below 60 % accuracy on cinematic grammar. Applies: a VLM
  is acceptable only as an offline curation hint, never as the benchmark path (section 6).

### 3.3 Drone and photogrammetry planners

- **Smith et al., "Aerial Path Planning for Urban Scene Reconstruction" (TOG 2018)**
  (<https://download.hrz.tu-darmstadt.de/media/FB20/GCC/paper/Smith-2018-APP.pdf>). Plans views and
  path jointly, using a coarse proxy model and a coverage objective. A semantic orienteering
  formulation is in <https://mediatum.ub.tum.de/doc/1520062/document.pdf>. Applies: the tour is an
  **orienteering problem**: maximize collected interest under a length budget, rather than visit
  all candidates (TSP). Our Held-Karp with a station budget is a small exact orienteering solver.
- **Next-best-view planning** (<https://link.springer.com/article/10.1007/s40747-022-00831-5>).
  Recursive greedy choice of the best view under a quality criterion. Applies: greedy selection with
  marginal gain.

### 3.4 Free-space representations

- **Voxblox** (<https://ar5iv.arxiv.org/html/1611.03631>), **FIESTA**
  (<https://ar5iv.labs.arxiv.org/html/1903.02144>), **nvblox**
  (<https://alexmillane.github.io/files/nvblox/nvblox_paper.pdf>). Euclidean signed distance fields
  give clearance everywhere in free space and push trajectories away from obstacles. FIESTA builds
  the ESDF from occupancy with full Euclidean distance. Applies: build occupancy exactly from CSG
  rays, then a full-Euclidean EDT, which gives clearance, openness and the medial axis.
- **Keeter, "Massively Parallel Rendering of Complex Closed-Form Implicit Surfaces" (TOG 2020)**
  (<https://www.mattkeeter.com/research/mpr/keeter_mpr20.pdf>). Interval arithmetic classifies whole
  tiles or octree cells as empty, full or ambiguous, and evaluates only the ambiguous ones. Applies:
  the same empty, full or ambiguous classification for our free-space cells (4.11). For us the
  "interval test" is instance-AABB culling plus exact ray intervals, because csg-core evaluates CSG
  along rays (`trace_compound`, `crates/csg-core/src/eval.rs:175`) and has no point-SDF API.

### 3.5 Production tools

- **Unreal Sequencer and Camera Rig Rail** (<https://dev.epicgames.com/documentation/en-us/unreal-engine/creating-camera-cuts-using-sequencer-in-unreal-engine?lang=en-US>,
  <https://forums.unrealengine.com/t/community-tutorial-unreal-flythrough-and-dolly-track-camera-rig-rail/2246664>).
  These are manual. **The ArcGIS Unreal flythrough sample**
  (<https://developers.arcgis.com/unreal-engine/sample-code/Flythrough/>) only ray-tests the path
  and lifts it over obstacles, the same idea as our current relaxation. Houdini and City Sample
  workflows (<https://dev.epicgames.com/documentation/unreal-engine/city-sample-quick-start-for-generating-a-city-and-freeway-in-unreal-engine-5>)
  generate cities, not cameras. No production tool solves "automatic interesting fly-through", so
  we build it from the research above.

### 3.6 Film grammar and automated cinematography

- **He, Cohen, Salesin, "The Virtual Cinematographer" (SIGGRAPH 1996)**
  (<https://www.microsoft.com/en-us/research/?p=150986>). Film idioms are encoded as small,
  hierarchically organized finite state machines over shared "camera modules" that place each
  shot. It includes the "don't cross the line" rule. Applies: our grammar is an automaton over shot
  descriptors (accept or reject, plus costs), and our move primitives are the camera modules.
- **Christianson et al., "Declarative Camera Control for Automatic Cinematography" (AAAI 1996)**
  (<https://mlanthology.org/aaai/1996/christianson1996aaai-declarative>). DCCL encodes 16 textbook
  idioms declaratively. Applies: shot types and grammar rules are declarative data (tables), not
  code branches, so they can be tested and tuned.
- **Galvane, Ronfard, Lino, Christie, "Continuity Editing for 3D Animation" (AAAI 2015)**
  (<https://ojs.aaai.org/index.php/AAAI/article/view/9288>). Editing is the optimal path through an
  editing graph. Dynamic programming under a semi-Markov assumption minimizes three costs: shot
  quality, continuity-rule violations and deviation from a rhythm (cutting pace) model. **This is
  our sequencer**: shots have variable durations (semi-Markov), and the costs are interest,
  harmony and pacing.
- **Galvane, Christie, Lino, Ronfard, "Camera-on-rails" (MIG 2015)**
  (<https://dblp1.uni-trier.de/rec/conf/mig/GalvaneCLR15.html>, abstract-level). Constrains the
  camera to a rail and optimizes position and orientation over time under velocity and
  acceleration limits. Applies: the transition solver between two camera states.
- **Ronfard et al., "The Prose Storyboard Language"** (<https://arxiv.org/pdf/1508.07593v4>,
  <https://diglib.eg.org:443/handle/10.2312/wiced20221048>). A shot is one or more compositions
  (shot size and framing at a moment) plus screen events and camera movements. Shot size describes
  a composition, not a whole shot. Applies: our `ShotDescriptor` separates **composition**
  (scale, subject, screen position) at the start and end from **movement** (the move primitive), so
  a push-in from long shot to medium shot is a single shot.
- **Lino, Christie, Ranon, Bares, "The Director's Lens" (ACM MM 2011)**
  (<https://www.irisa.fr/mimetic/GENS/mchristi/Publications/2011/LCRB11a.html>). Suggests camera
  placements by classical framing rules (shot size, angle, composition) and groups suggestions as
  "Extreme Long Shot, High Angle" and so on. Applies: the candidate catalog is indexed by
  (scale, angle, lens), and the grammar asks for a missing cell of that index.
- **Lino and Christie, Toric space** (cited in 3.1). Gives exact two-subject framing for orbit and
  dolly moves with a chosen lens.
- **Cutting et al., shot-length statistics** (<https://flowingdata.com/2014/09/22/evolution-of-movies/>,
  <https://news.cornell.edu/index%2ephp/node/271563>, abstract-level). Average shot length fell from
  about 10 to 12 s (early sound era) to about 2.5 to 4 s, and shot-duration fluctuations approach a
  1/f pattern. Applies: the pacing model draws durations around an operator-chosen mean shot length
  with correlated (not uniform, not metronomic) variation. A fly-through is slower than narrative
  cutting, so the default mean is 6 s, range 3 to 14 s.
- **Long-lens practice** (<https://store.dji.com/event/content-how-to-use-a-zoom-camera>,
  <https://photography.tutsplus.com/tutorials/how-to-make-cinematic-shots-with-a-telephoto-lens--cms-57>).
  Telephoto compression shrinks apparent depth and apparent motion. Long-lens moves need foreground
  layers to read motion and are harder to fly, so they need slower, steadier moves. Digital crop is
  not compression; true compression needs distance. Applies: lens-dependent speed and angular-rate
  limits, a parallax-layer requirement for telephoto moves, and subject distance derived from the
  focal length.

### 3.7 What we take

1. Exact occupancy from analytic rays, a distance field, and a skeleton roadmap: clearance,
   openness and corridors (Oskam, Geraerts, FIESTA, Keeter-style culling).
2. Frame interest measured on probe AOVs, with data-derived object importance (Vázquez, Sokolov
   and Plemenos). The weights are fitted to operator pairwise preferences (Wang, ACDC).
3. Shots as composition plus movement (Prose Storyboard Language), camera modules for the moves
   (Virtual Cinematographer), declarative idioms (DCCL).
4. The sequence and the duration allocation solved together by semi-Markov dynamic programming
   over shot quality, continuity and rhythm (Galvane 2015). Transitions are rail-constrained and
   rate-limited (Camera-on-rails, Joubert, Roberts).
5. A lens is part of the shot, and the distance follows from focal length and shot scale. The
   speed limits depend on the lens.
6. Learned aesthetics and VLMs only offline, with frozen output (ShotBench accuracy, determinism).

## 4. Design

### 4.1 Operator decisions this design implements

| # | Decision |
| --- | --- |
| D1 | Water: no control points or path over open water. **Approved exception:** covered water (under a bridge or overpass) within 40 m of land. |
| D2 | Duration is a user parameter, 15 to 120 s. Longer is allowed only if the planned shots are spectacular (4.9). The generator decides how many shots fit and how long each lasts by optimizing harmony; the shot count is never fixed. |
| D3 | A vocabulary of camera moves and lenses, applied in cinematic order by film grammar: establishing, then medium, then detail; rhythm; contrast of scale and focal length; 180° rule and screen direction. |
| D4 | Systemic architecture: frame **interest** is an isolated crate, **harmony** is another isolated crate, and the pipeline is node-based with typed inputs and outputs. |
| D5 | Camera state is a first-class value (pose, target, lens optics). **One transition engine** interpolates everything, optics included, with easing. A shot is a parametric move over time on top of it. |
| D6 | Street runs along the street graph with named height bands (asphalt skim, second floor, rooftop) and band transitions. |

### 4.2 System architecture: typed node pipeline

```text
 csg-rs (scene data)                         shot-rs (scene-agnostic)                            camera-rs
 ------------------                          -------------------------                           ---------
 WorldRayQuery, intervals,  --SceneOracle--> [SpaceMap] --> [LocationClasses] --> [CandidateGen x N]
 semantics, hero/bridge                         |  ESDF, columns, skeleton,         StreetRun, Orbit, Reveal,
 flags, StreetGraph                             |  roof envelope, street bands      Establishing, TopDown,
                                                |                                   UnderBridge, Crane, Dolly...
 nv-demo (glue)                                 v                                        |
 ------------------          ProbeFrame  [Probe] <------- CameraState samples ----------+
 CpuProber (deterministic) ------------->  |  (depth/semantic/instance AOVs)
 OptixProber (interactive)                 v
 ObjectTable (idf, class,   ---------->  [shot-interest]  [Openness]  [shot-harmony::frame]
 hero, dynamic)                            \______________|______________/
                                                          v
                                                    [ShotCatalog]  shots with quality(d), duration models
                                                          |
                                     +--- cost oracle ---[TransitionSolver] (feasible A->B, min time)
                                     v                    |
                                [GrammarSequencer]  semi-Markov DP: order + durations + transitions
                                     |                    (shot-harmony::sequence, pacing model)
                                     v
                                [TrackAssembly] ------------------------------------------> camera-cine::Track
                                     |                                                         |
                                [Validate/Metrics]                                       TrackPlayer::eval(t)
                                                                                               -> CameraState
                                                                                      nv-demo: CameraState ->
                                                                                      CameraPose + lens -> render
```

**Node contract** (`shot-plan::node`):

```rust
pub trait Node {
    const NAME: &'static str;
    type In<'a>;
    type Out: Serialize + DeserializeOwned;           // every artifact is inspectable and cacheable
    fn run(&self, ctx: &PlanCtx, input: Self::In<'_>) -> Result<Self::Out, NodeError>;
}
```

- `PlanCtx` carries the seed streams (one per node), the config and the version.
- The DAG is static and typed in Rust: the compiler checks the wiring, and execution order is
  fixed, which keeps it deterministic.
- Every artifact is hashed. Artifacts are cached per scene, so the space map is reused when only
  the duration or seed changes. They can be dumped as JSON for inspection and for golden tests per
  node.
- A visual node editor (the existing private `nodes-rs`/`nodes-core`) can wrap the same nodes
  later as an adapter. It is not a dependency of the planner.
- *Rejected:* a dynamic, string-keyed node graph at runtime. It loses compile-time typing, and
  nothing in the requirements needs runtime re-wiring.

### 4.3 Camera state, lens, transitions (camera-rs `camera-cine`)

```rust
pub struct Lens { pub focal_mm: f32, pub film: FilmBack /* 36x24 default */,
                  pub focus_m: Option<f32>, pub f_stop: Option<f32> }
pub struct CameraState { pub eye: Vec3, pub orientation: Quat,
                         pub target: Option<Vec3>,   // look-at anchor, when the shot has a subject
                         pub lens: Lens }
impl CameraState { pub fn pose(&self, znear: f32, zfar: f32) -> CameraPose; pub fn fov_y(&self) -> f32; }
```

- `fov_y` comes from `camera_core::lens::angle_of_view` (`lens.rs:128`), so there is one lens math.
  400 mm on a 24 mm film height gives a vertical FOV of about 3.4°, inside `FOV_MIN = 1°`
  (`lens.rs:97`).
- Today's formats **cannot** express telephoto: `RailWaypoint` evaluation clamps the vertical FOV
  to [20°, 110°] (`csg-scene rail.rs:298`), and `ViewCandidate` clamps the same way
  (`view_score.rs:82`). The new pipeline therefore plays `camera-cine::Track`, not `RailSpec`.

**Transition engine (single implementation):**

```rust
pub struct Transition { pub from: CameraState, pub to: CameraState, pub duration: f32,
                        pub path: PathSpec,        // Straight | Through(Vec<Vec3>) (skeleton route) | Hermite{v0,v1}
                        pub aim: AimSpec,          // Slerp | TargetPath(Vec<Vec3>) | Squad
                        pub time_ease: Ease,       // Linear | SmoothStep | SmootherStep | Bezier(x1,y1,x2,y2) | MatchVelocity{v0,v1}
                        pub optics_ease: Ease }
pub trait Move { fn duration(&self) -> f32; fn state_at(&self, t: f32) -> CameraState;
                 fn velocity_at(&self, t: f32) -> (Vec3, Vec3 /*angular*/, f32 /*d ln f / dt*/); }
pub enum Segment { Shot(Box<dyn Move>), Transition(Transition), Cut }
pub struct Track { pub segments: Vec<Segment>, pub looping: bool }   // TrackPlayer::eval(t) -> CameraState
```

- **Position:** centripetal Catmull-Rom or Hermite through the route, reparameterized by arc
  length (the LUT code moves here from `nv-demo` `camera_rail.rs:285-348`), then by `time_ease`.
- **Orientation:** when both states have targets, a target path is interpolated and the camera
  looks at it, with roll kept level. Otherwise SQUAD (the code moves here from
  `camera_rail.rs:160-210`).
- **Optics:** focal length is interpolated in `ln(focal)`, so equal time gives an equal zoom
  ratio, which is perceptually uniform. Focus is interpolated in diopters (`1 / distance`) and
  f-stop in stops (`log2`).
- **Continuity:** a transition between two moves uses `MatchVelocity` with the end velocity of
  shot A and the start velocity of shot B (position, angular and focal rates). That makes the joins
  C1, so the take never stops and starts at shot boundaries. Easing to rest is used only at
  deliberate holds.
- **Cuts:** `Segment::Cut` exists for an "edited" style, but the default fly-through is one
  continuous take.
- **Moves:** every move is a `Move` built from the same curve and ease primitives, so there is no
  second interpolator.
- **Custom keyframes:** the `Custom` deck becomes a `Track` of transitions between keyframed
  `CameraState`s. `CameraKeyframe` gains a focal length. Curated csg-rs `RailSpec`s are converted
  once at load into a `Track` (data conversion, not a runtime shim).

### 4.4 Move vocabulary

All moves are parametric in a **subject frame**: subject center `c`, up `Y`, and a reference
azimuth from the line of interest. The planner fills the parameters.

| Move | Parameters | Typical lenses | Notes |
| --- | --- | --- | --- |
| `Establishing` | subject or area, scale ELS or LS, high elevation, slow drift (lateral or forward ≤ 3 % of distance per s) | 18-24 mm wide, or 200-400 mm long-lens skyline | Opens a sequence or location. The long-lens variant compresses the skyline. |
| `PanH` / `TiltV` | pivot at eye, angle span, rate | any; slow at long lenses | A pure rotation. The angular rate is limited by lens (4.6). |
| `TopDown` | nadir view over subject, altitude, optional slow rotation or rise | 24-35 mm | A "neutral" shot: allowed between shots on opposite sides of the line. |
| `Orbit` | subject, radius from lens and scale, arc 30-120°, elevation, direction | 35-80 mm | Screen direction = orbit direction. Toric-space framing keeps the subject on a third. |
| `PushIn` / `PullOut` | along the subject axis, scale change (LS to MS) | 35-80 mm | Composition changes inside one shot. |
| `FlyForward` | along a free corridor, look ahead with a subject bias | 18-35 mm | Generic travel shot. |
| `DollyTrack` | lateral to the subject, parallel to a facade or shore, side of the line | 50-135 mm | Lateral parallax. Needs foreground layers at longer lenses. |
| `Crane` / `Pedestal` | vertical rise or fall, pitch compensation | 24-50 mm | Includes the band-change crane (4.5). |
| `Reveal` | occluder found by an exact ray; the move rises or slides until the subject becomes visible at about 60 % of the duration | 24-50 mm | The visibility transition is timed. |
| `DollyZoom` | distance and focal change with `distance * tan(fov / 2)` constant (`camera_core::lens::dolly_zoom_factor`, `lens.rs:190`) | 24 to 80 mm | A deliberate effect; at most once per tour. |
| `OpticalZoom` | focal ramp at a fixed eye | 35 to 400 mm | Exempt from the transition optics-rate limit, with its own cap. |
| `StreetRun` | street path, height band, speed, look mode (ahead, side facade, ahead with subject bias) | 18-35 mm (asphalt), 24-50 mm (others) | See 4.5. |
| `BandChange` | within a `StreetRun`: from band to band at a chosen arc position | same as the run | A crane-up reveal or a drop into a canyon. |
| `UnderBridgePass` | across the span's minor axis, gap mid-height | 18-24 mm | D1 exception applies. |
| `Flyover` | over a subject at mid or high band, pitch-down tracking | 24-50 mm | |

**Lens classes:** Wide (14-24 mm), Normal (35-50 mm), Short-tele (80-135 mm), Long-tele
(200-400 mm).

**Subject distance from the lens:** for shot scale `k`, the subject's projected height fraction is
`k = H_subj / (2 * d * tan(fov_y / 2))`. ELS 0.1, LS 0.3, MLS 0.5, MS 0.7, detail 0.9 of the
frame height. Solve for `d`, then check line of sight with exact rays and that the eye is valid
(land or the approved underpass, clearance). If the result is infeasible, try the next lens class.
Long-tele shots typically stand 300 to 1500 m away, which needs a long clear sightline and gives
strong compression.

### 4.5 Street runs and height bands

**Street paths** come from the csg-rs street graph (centrelines, widths, kinds; R-A11), with the
space-map ground skeleton as fallback for scenes without street hints. Cornering: at each
intersection the path chooses straight, turn, or (in the rooftop band) a shortcut over the block.
A turn is a clothoid-blended arc with radius at least `R_min(v, lens)` (4.6).

**Space-map inputs per street sample `s`:**

- `ground(s)`: the road surface top from the column map.
- `facade_dist(s, side)`: the 2D distance at band height to the `Structure` column, both sides.
- `roof_env(s)`: the roof envelope. It is the maximum `Structure` top within `half_width + 5 m` of
  the centreline. A max-preserving smoothing (running max over 15 m, then a Gaussian with sigma
  10 m, then `max` with the raw envelope plus 0.5 m) keeps the camera above every roof while
  following the roofline. The slope is limited to `|d env / ds| <= 0.35` by a forward and backward
  max-propagation, so the camera never bobs over every chimney.
- Traffic lane centrelines from `SceneHints::traffic_lanes`.

| Band | Eye height | Clearance rules | Speed and look |
| --- | --- | --- | --- |
| `AsphaltSkim` | `ground + 0.5..1.5 m` | Static clearance radius 0.3 m (camera body). Lateral distance at least 1.5 m from any traffic-lane centreline, so the camera runs along the median, shoulder, parking strip or a pedestrian street. Any CURB/BARRIER/POLE within 1 m ahead fails the segment. | Speed feel: `v <= Omega_band * d_near`, with `Omega = 6 rad/s` (high optical flow is the point), capped at 14 m/s. Wide lens. |
| `SecondFloor` | `ground + 5..8 m`, inside the canyon | `facade_dist >= max(2 m, 0.15 * width)` on both sides. Exact 26-ray stencil at radius 1.5 m. | `Omega = 2.5 rad/s` (readable facades), 6-12 m/s. Look ahead, or a 30-45° side glance at a hero facade. |
| `Rooftop` | `roof_env + 2..6 m` | At least 2 m above the actual roof (exact downward ray) and stencil 2 m. Altitude curvature `abs(d²y/ds²) <= 0.01 /m`. | `Omega = 1.5 rad/s` relative to the roof distance, 8-18 m/s. The operator called this a very powerful shot, so its quality prior is raised (4.7). |

**Band transitions (grammar-level):**

- `AsphaltSkim -> Rooftop` crane-up reveal. The crest is timed so that a landmark goes from
  occluded to visible at the top: the occluder is found with exact rays at the low and high eye
  positions.
- `Rooftop -> SecondFloor` drop into the canyon, at a canyon entrance.
- `SecondFloor -> AsphaltSkim` settle, before a speed-feel segment.
- Vertical rate is at most 4 m/s. Pitch compensation keeps the horizon readable. One band change
  per 150 m of run at most.

**Speed scales with height:** the readability limit is the angular flow of the nearest significant
surface, `v <= Omega_band * d_near * (f_ref / f)`. Here `d_near` is the minimum of the height above
ground and the facade distance, and `f_ref = 24 mm`. Low bands are therefore slower in absolute
terms but read as faster, which is the intended "speed feel". The rooftop band flies faster, yet
reads as calm and majestic.

### 4.6 Rate limits (one table, used by moves, transitions and tests)

| Quantity | Limit |
| --- | --- |
| Yaw rate | `<= 30°/s * (f_ref / f)^0.7` (long lens is slower) |
| Pitch rate | `<= 12°/s * (f_ref / f)^0.7` |
| Roll | 0 (level), or banking `<= 6°` on curved runs only |
| Optics rate | `abs(d ln f / dt) <= ln 2 /s` in transitions, `<= 2 ln 2 /s` inside `OpticalZoom`/`DollyZoom` |
| Path curvature | `kappa <= omega_yaw / v`, which gives `R_min` |
| Climb | `abs(dy/ds) <= 0.35` in street runs, `<= 0.25` in other transitions; vertical rate `<= 4 m/s` |
| Linear acceleration | `<= 3 m/s²`, so joins are smooth |
| Screen flow | `v * f / (d_near * film_h) <= Omega_band` (the band rule, expressed in screen space) |

### 4.7 Interest: crate `shot-interest` (isolated)

A pure crate, independent of any scene. It depends only on glam and serde.

```rust
pub struct ProbeFrame<'a> { pub w: u32, pub h: u32, pub depth: &'a [f32], pub semantic: &'a [u16],
                            pub object: &'a [u32], pub normal_y: Option<&'a [f32]> }
pub struct ObjectInfo { pub class: ClassId, pub importance: f32 /* idf * hero */, pub dynamic: bool, pub size_m: f32 }
pub trait ObjectTable { fn info(&self, object: u32) -> Option<ObjectInfo>; }
pub struct ClassWeights { /* per ClassId, versioned, serde */ }
pub fn frame_interest(f: &ProbeFrame, objs: &impl ObjectTable, w: &ClassWeights) -> FrameInterest;
pub struct FrameInterest { pub density: f32, pub count: u32, pub diversity: f32, pub depth_layers: f32,
                           pub openness: f32, pub empty: Emptiness /* sky, flat ground, water */,
                           pub salient_centroid: Vec2, pub top_objects: [(u32, f32); 8] }
```

- **Interest density `I`:**

  ```text
  I = 0.7 * norm(sum over objects of imp(o) * min(cov(o), 0.25) / 0.25) + 0.3 * min(count, 12) / 12
  ```

  `cov(o)` is projected coverage with a 1.0 center / 0.5 border mask. `count` is the number of
  objects with `imp >= 0.5` and coverage at least 0.5 %. The 0.25 cap means "a concentration of
  interesting objects", not one object filling the frame.
- **Openness `O`:**

  ```text
  O = 0.5 * min(q50(t_center) / 150, 1) + 0.3 * frac(t > 60 m) + 0.2 * (1 - frac(t < 8 m))
  ```

  A miss counts as `t = 600 m`. Openness is always multiplied by the content gate, so it never
  acts alone.
- **Diversity `D`:**

  ```text
  D = 0.5 * H(classes by coverage) / ln 6 + 0.5 * H(depth layers [<15, 15-60, 60-220, >220 m]) / ln 4
  ```

  This is the same entropy as `score_probe_grid`, `view_score.rs:290-313`.
- **Gate `G`:**

  ```text
  G = (1 - clamp(2.5 * near_occ, 0, 1)) * (1 - smoothstep(0.55, 0.85, empty))
      * (1 - smoothstep(0.30, 0.55, water))
  ```

  `empty` is the sky, flat-ground and water fraction.
- **Default weights in shot quality:** `w_I = 0.40, w_O = 0.20, w_D = 0.20, w_C = 0.20`, where `C`
  is frame harmony. Interest has the largest weight because the operator's primary criterion is a
  concentration of interesting objects. The gates are multiplicative so that openness cannot rescue
  an empty sea view. All weights are versioned and refitted from operator pairwise preferences
  (Bradley-Terry) in Creative mode (section 6).
- Importance: `idf` over assembly frequency separates distinctive objects from repetitive filler,
  times the hero flag, times the class weight. Water is backdrop, with weight 0.05.
- nv-demo builds the `ObjectTable` from `WorldScene` (assembly frequencies, semantics, hero flags,
  dynamic ids). The crate never sees csg types.

### 4.8 Harmony: crate `shot-harmony` (isolated)

Three levels. Each is a pure function returning a score with a breakdown, so the reasons are
inspectable.

1. **Frame harmony** (`frame_harmony(&ProbeFrame, &FrameInterest) -> FrameHarmony`): rule of thirds
   for the salient mass and the top subject; visual balance (salient mass left and right); horizon
   level and placement; headroom and lead room in the motion direction; leading lines (depth
   gradient coherence); subject separation (depth discontinuity).
2. **Transition harmony** (`transition_harmony(&ShotDescriptor, &ShotDescriptor, &TransitionInfo) ->
   TransitionHarmony`):
   - **Motion continuity:** the change of velocity direction and the angular-velocity jump at the
     join (after `MatchVelocity` these should be small, so the term catches infeasible pairs).
   - **Screen direction / 180° rule:** a subject's line of interest is the subject axis
     (street direction, facade normal, coast line, landmark main axis). The camera side is
     `sign((eye - c) x axis)`. Consecutive shots on the same subject keep the side, or pass through
     a neutral shot (`TopDown`, head-on `PushIn`), or cross with a continuous visible move (an
     orbit crossing the line on screen is legal). Lateral screen motion direction (left or right)
     is preserved across consecutive lateral moves.
   - **Scale and lens contrast:** consecutive shots differ in scale or lens class. The progression
     establishing, then LS, then MS, then detail inside one location is rewarded. A new location
     starts at least as wide as LS.
   - **Subject continuity:** shared top objects across the join (a "match" between A's end frame
     and B's start frame) are rewarded on cuts and allowed on continuous transitions.
3. **Sequence harmony** (`sequence_harmony(&[ShotDescriptor], &[f32 /*durations*/]) ->
   SequenceHarmony`):
   - **Rhythm:** durations around the mean shot length with 1/f-like correlated variation; a
     metronome (equal durations) is penalized.
   - **Tension and release:** a fast or close shot (asphalt skim, canyon, under bridge) is followed
     within 2 shots by a release (establishing, rooftop, slow orbit).
   - **Lens variety:** every lens class is used once per 4 to 6 shots when the duration allows.
   - **Altitude variety:** band alternation.
   - **Arc:** an opening (establishing), development, a climax (the highest-quality shot placed in
     the last 35 to 75 % of the duration) and a closing that matches the opening state for
     seamless looping.

`ShotDescriptor { kind: MoveKind, scale_start, scale_end, lens_class, band: Option<Band>, subject:
Option<SubjectId>, side: Option<Side>, screen_dir: Option<ScreenDir>, location: LocationId,
start: CameraState, end: CameraState, dur_range: (f32, f32) }`. This is a Prose-Storyboard-style
composition plus movement.

### 4.9 Shot catalog, grammar and duration allocation (crate `shot-plan`)

**Shot catalog.**

- The candidate generators per location class (canyon, underpass, rooftop cluster, plaza,
  landmark, waterfront, hotspot, open vista, street network) emit shots: a move with parameters,
  a lens and a subject.
- Each shot is probed at 3 to 5 times along its move (coarse 8 x 6, then fine 32 x 18 for the
  finalists). Shot quality:

  ```text
  Q = mean over samples of G * (w_I * I + w_O * O + w_D * D + w_C * frame_harmony)
      * kind_prior + spectacle bonus
  ```

  The spectacle bonus is for a reveal crest, a long-lens skyline or a rooftop roofline.
- Each shot carries a **duration model** with `d_min` (readability: at least 2.5 s plus the time
  for the subject to be on screen for 1.5 s; long lens adds 1 s), `d_pref` and `d_max` (when the
  content runs out or the move hits its geometric end). The quality depends on duration:
  `Q(d) = Q * (1 - ((d - d_pref) / (d_max - d_min))²)`, a concave curve.
- The catalog is pruned to the top 40 shots by `Q` with diversity (marginal gain), so every
  location class and lens class stays represented.

**Grammar** (declarative tables, DCCL-style, in `grammar.rs`):

- **Hard rules (automaton):**
  - Start with `Establishing` (or a `Reveal` that ends at ELS or LS).
  - No two consecutive shots of the same `MoveKind`.
  - No 180° violation without a neutral shot.
  - No location revisited (unless it is the final loop closure).
  - At most one `DollyZoom`.
  - Every `StreetRun` band change goes through a `BandChange` (crane).
- **Soft rules (costs)** come from the transition and sequence harmony, plus a pacing prior (mean
  shot length `L_shot`, user setting, default 6 s).

**Sequencer: semi-Markov DP with duration allocation** (Galvane 2015 structure):

- Time is discretized at `dt = 0.25 s`, so `D = 120 s` gives 480 steps.
- State: `(last shot index, end time step, grammar automaton state)`. A visited-location bitset is
  kept per beam entry.
- A step chooses the next shot `j`, its duration `d_j` on the grid within `[d_min, d_max]`, and the
  transition `A -> j`. The transition time `t_tr` and feasibility come from the transition solver
  (4.10) as a cached cost oracle. Its value:

  ```text
  value = Q_j(d_j) + transition_harmony(A, j) - pacing_cost(d_j | L_shot, history)
          - lambda_retrace * overlap(route)
  ```

- Termination: the total `sum(d_j + t_tr) = D` within `±max(0.25 s, 1 %)`. For looping, the closing
  transition back to the first shot's start state is included in the budget.
- Search: beam search with width 4096, deterministic ordering (value, then shot index, then
  duration step). The candidate set is at most 40 shots and the depth is about 20 shots. That is
  exact enough and costs about 1 to 3 s.
- **The number of shots emerges from the optimization.** Short durations favour a few long, strong
  shots: at 15 s, typically an establishing shot, one hero move, and a release. Long durations add
  locations and lens and height variety, until extra shots lower the mean value. Each shot gets
  exactly the time that maximizes total harmony, not `D / n`.
- **Long requests:** `D > 120 s` is accepted only if the optimal plan's mean shot `Q` is at least
  `q_spectacular` (versioned constant, calibrated in M8). Otherwise the planner returns
  `PlanError::DurationExceedsContent { max_s }`. The same check runs between 15 and 120 s, and a
  plan whose mean drops below `q_floor` is shortened only with the operator's consent (a
  diagnostic in the UI); the requested duration is never silently changed.

### 4.10 Transition solver

Input: shot A's end state and velocity, and shot B's start state and velocity.

1. Route the eye on the free-space skeleton (shot-space), with the water rule D1 and lateral and
   vertical separation from routes already used by the plan.
2. Fit a Hermite / centripetal path matched to both velocities.
3. Plan the target path from A's target to B's target, through intermediate "glance" targets of
   the highest importance visible along the route.
4. Optics are interpolated in `ln f`.
5. The minimal feasible duration follows from all limits in 4.6. A longer allotted duration slows
   the transition with a velocity-matched time ease.
6. Verify with exact clearance (stencil at a band-appropriate radius) along 1 m samples.

The result is a `camera-cine::Transition` plus the route, and the cost is cached by `(A, B)`.

### 4.11 Space map and location classes

Crate `shot-space`, through the `SceneOracle` trait:

- **Column map (2.5D, exact).** XZ grid with 2 m cells. One vertical multi-hit ray per cell
  (R-A8) gives the solid intervals with semantic and instance. Gundo: 127k columns, about 0.4 M ray
  segments, estimated 0.5 to 1.5 s CPU.
- **Occupancy and ESDF.** Voxels at 4 m (Gundo about 0.95 M), with instance-AABB culling (Keeter
  style: empty, full or ambiguous), then an exact Felzenszwalb-Huttenlocher EDT (about 0.1 s).
- **Skeleton roadmap.** ESDF ridges thinned to a graph of Oskam-style spheres, edges confirmed by
  exact chord rays, cost `length / clamp(clear, 2, 20)`.
- **Derived XZ fields.** `dist_to_water`, `dist_to_land`, land components, and building footprints
  (connected `Structure` columns with height statistics).
- Classes: canyon, underpass, rooftop cluster, plaza, landmark, waterfront, hotspot, open vista,
  plus **street network** with per-sample `ground`, `facade_dist`, `roof_env`, `lanes`.
- Water rule D1: a column is `Covered` when an overhead solid interval lies at least 4 m above
  water or ground. Covered water within 40 m of land is allowed for eye and path. Open water is
  never allowed.

### 4.12 Metrics (test oracles)

All sampled from the final `Track` at 60 Hz and 1 m arc:

| Metric | Gate |
| --- | --- |
| `water_eye` | 0 samples over open water. Covered-water samples only within 40 m of land. |
| `clearance_by_band` | Asphalt at least 0.3 m and at least 1.5 m from lane centres; SecondFloor facade distance at least `max(2, 0.15w)` and stencil 1.5 m; Rooftop at least 2 m above the actual roof; others stencil 1.2 m |
| `roofline_follow` | During rooftop runs, height above `roof_env` in [2, 6] m for at least 90 % of samples; `abs(dy/ds) <= 0.35`; `abs(d²y/ds²) <= 0.01` |
| `rates` | Every limit in 4.6 (yaw, pitch with lens scaling, optics, curvature, climb, acceleration, screen flow per band) |
| `duration` | `abs(track.duration - D) <= max(0.25 s, 1 %)`; for `D > 120` the spectacle gate passed |
| `grammar_valid` | The automaton accepts the shot sequence; there are no 180° violations; the first shot is `Establishing` |
| `contrast` | Consecutive shots differ in at least 2 of {scale, lens class, band, move kind} |
| `retrace_ratio` | `<= 0.03` (definition unchanged) |
| `content_aim`, `water_view`, `mean_S_along_path` | `>= 0.9`, `<= 0.35`, `>= 0.8 x` mean shot `Q` |
| `continuity` | C1 at every segment join: position, angular and focal-rate jumps below epsilon |

## 5. Crate layout and responsibilities

### 5.1 Layout

| Where | Crate | Contents | Why here |
| --- | --- | --- | --- |
| **camera-rs** (cglibs) | `camera-cine` (new member `crates/cine`; re-exported by the `camera` umbrella under feature `cine`) | `Lens`, `FilmBack`, `CameraState`; `Ease`; `PathSpec`/`AimSpec`; **the** `Transition` engine; the `Move` trait and **all move primitives** (4.4) as pure parametric functions; `Track`, `TrackPlayer`; arc-length LUT, Catmull-Rom, SQUAD (moved from nv-demo `camera_rail.rs`); rate-limit evaluation helpers | Camera state, optics and moves are generic camera concepts that any cglibs viewer could use (egui-widgets-rs viewers, playa, render-rs). camera-rs is the declared home of camera concepts. `camera-cine` stays pure (glam, serde; caller-supplied time), like `camera-core`, and reuses `camera_core::lens` and `CameraPose`. It is a separate crate, not part of `camera-core`, because core is interactive navigation and the cinematic timeline is a different concern. **Needs the camera-rs owner's agreement** (its `PLAN §7a`). |
| **New cglibs repo `shot-rs`** | `shot-interest` | Frame interest (4.7) | Isolated, testable on synthetic buffers, reusable (thumbnail ranking, SDG view selection) |
| | `shot-harmony` | Frame, transition and sequence harmony (4.8) | Isolated. Depends only on `camera-cine` types (for `CameraState` in descriptors) and on `shot-interest` output types. |
| | `shot-space` | Space map, ESDF, skeleton, classes, street bands, through the `SceneOracle` trait | Scene-agnostic: any renderer with exact ray queries can implement the oracle |
| | `shot-plan` | Node pipeline, candidate generators, shot catalog, grammar tables, sequencer, transition solver, metrics, plan config and errors, curation-file format | The planner itself. Depends on the three crates above and `camera-cine`. |
| **nv-demo** | `rail_plan` module (glue only) | `SceneOracle` implementation over csg-rs `WorldRayQuery` plus `SceneHints`; `ViewProber` implementations (CPU deterministic and OptiX interactive) producing `ProbeFrame`; `ObjectTable` from `WorldScene`; UI (duration 15-120 s, mean shot length, seed, style, creative labeling); `CameraRail` deck holding `Track`s; conversion of curated `RailSpec` to `Track` at load; bench on `Track`; `CameraState` to renderer (FOV from lens) | Scene-specific and GPU-specific code stays in the app |
| **csg-rs** | data and queries (5.2) | Exact rays, intervals, semantics, flags, street graph | Owns the geometry and scene data |

`curves-rs` (cglibs, Maya-style Bezier scalar tracks, used by `animation-rs`) is the SSOT for
authorable scalar curves. `camera-cine` uses it for the `Bezier` ease and for user-authored optics
channels in the Custom deck. That is one curve evaluator, not a second one. Whether `camera-cine`
takes that dependency is decided with the camera-rs owner. The fallback is an internal cubic-Bezier
ease with the same semantics (about 40 lines). Both are listed; the owner picks.

*Rejected:*

- Everything in nv-demo: the interest and harmony scorers and the camera timeline are generic,
  and the operator asked for isolated crates.
- Everything in csg-rs: it would make the geometry library own cinematography and app policy.
- `shot-*` inside camera-rs: image scoring and scene planning are not camera math, and they would
  bloat a navigation crate that is consumed by many apps.

### 5.2 Requests to csg-rs

1. **R-A1: `WorldRayQuery`** (nearest hit with semantic and instance id; stencil clearance).
2. **R-A8: `intervals(ray)`** (multi-hit solid intervals).
3. **R-A2: semantic contract** (WATER and GROUND), with a per-scene test.
4. **R-A9: flags.** `inst_flags::HERO` (or `LandmarkHint::instances`), and `semantic::BRIDGE` or
   `inst_flags::SPAN`.
5. **R-A11 (new): public street graph.** Move `StreetNavGraph` out of `rail_gen` into its own
   module `street_graph`, with centreline polylines, `width_m`, kind, lanes per edge and
   intersection nodes, built from `SceneHints`. Rail generation is being deleted (R-A5), but the
   graph is needed for street runs.
6. **R-A5: stop auto-generating rails** in scene builds and delete the archetype generators.
7. **R-A6: landmarks for OSM scenes.**

**Dropped since the last revision:**

- R-A3 (`RailSpec::look_mode`) and R-A4 (`RailSpec` timing): playback no longer uses `RailSpec`.
  Curated rails are converted to `Track`.
- R-A7 and R-A10 stay "not needed".

## 6. Modes and determinism

| | Deterministic (default; benchmark) | Creative (offline, opt-in) |
| --- | --- | --- |
| Pipeline | All nodes on CPU, `CpuProber`, committed `ClassWeights::V{n}` and `HarmonyWeights::V{n}` | Same nodes. Optional learned aesthetic score on path-traced thumbnails. Optional VLM ranking of the top 30 shots, with the operator's consent because images leave the machine. Pairwise labeling to fit new weight versions. |
| Output | `Track` id `tour_v{planner}_{weights}_{D}s_{seed:08x}`: byte-identical across runs, machines and thread counts | `curation/<scene>.json`: the chosen shot ids, durations and seed. The deterministic mode replays it exactly. |
| GPU | Never decides output | OptiX/WGPU for interactive previews only (scores differ from CPU by up to 0.08, `csg_scene.rs:1402`) |

- **Inputs hashed into the id:** scene id, build-config hash, frozen frame-0 dynamic poses, `D`,
  `L_shot`, style, seed, and the planner, weight and curation versions.
- **RNG:** one `splitmix64` stream per node. All loops are index-ordered. Parallel work writes to
  pre-sized slots and reduces in index order.
- **Errors:** typed `PlanError`: `NoLand`, `TooFewShots`, `DurationExceedsContent { max_s }`,
  `ClearanceFailed`, `MetricGate { name, value, limit }`, `CurationMismatch`. There is no fallback
  rail.
- **Bench:** `--bench-rail tour_v1_..._30s_...` with `--frames`. Re-baseline `PERF:RT`.

## 7. Tests

| Crate | Test | Asserts |
| --- | --- | --- |
| camera-cine | `transition_hits_endpoints_exactly` | `state_at(0) == from`, `state_at(T) == to` (position, orientation, focal) |
| | `transition_optics_log_uniform` | Focal follows `exp(lerp(ln f0, ln f1, ease(u)))`; FOV matches `angle_of_view` |
| | `match_velocity_joins_are_c1` | Position, angular and focal-rate continuity at Move to Transition to Move joins |
| | `moves_parametric_invariants` | Orbit radius constant; `DollyZoom` keeps `d * tan(fov / 2)`; `PushIn` scale monotonic; `TopDown` pitch -90° |
| | `track_player_deterministic` | Same inputs give bit-identical states at 10k random times |
| shot-interest | `idf_ranks_unique_above_repeated`, `empty_and_water_gates`, `density_counts_objects` | Synthetic buffers with known answers |
| shot-harmony | `thirds_and_balance`, `detects_180_violation`, `neutral_shot_allows_side_change`, `scale_progression_rewarded`, `metronome_penalized`, `tension_release_rule` | Hand-built descriptors with known verdicts |
| shot-space | `columns_match_exact_rays`, `esdf_matches_bruteforce`, `classes_fixture_canyon_bridge_plaza`, `roof_envelope_smooth_and_above_roofs`, `covered_water_detected` | Fixtures plus Gundo |
| shot-plan | `duration_honoured` | `D` in {15, 30, 60, 120} within `max(0.25 s, 1 %)`; the shot count differs between 15 s and 120 s |
| | `long_duration_requires_spectacle` | 180 s on a sparse fixture returns `DurationExceedsContent` |
| | `grammar_valid`, `contrast`, `no_consecutive_same_move` | On all plans |
| | `rates_within_limits` | All of 4.6, lens-scaled |
| | `street_run_bands` | Per-band clearance; asphalt lane offset; rooftop `roofline_follow` gates |
| | `band_change_is_crane_reveal` | Vertical rate `<= 4 m/s`; landmark occluded before the crest and visible after it |
| | `no_open_water_eye` | Only approved covered water within 40 m |
| | `deterministic_bytes` | 3 runs and different thread counts give identical serialized `Track`; a different seed gives a different result; curation replay is identical |
| nv-demo | `plan_<scene>_end_to_end` (Gundo, GundoOSM, Marina, SF House of AI; CPU, headless) | All metrics in 4.12 |
| | `curated_railspec_converts_to_track` | Conversion preserves samples within 1 cm and 0.1° |
| | `bench_tour_track_runs` | `--bench-rail` parses the tour id; one-loop frame count matches `D` |
| csg-rs | `world_ray_query_matches_verify_clearance`, `intervals_match_restarted_raycast`, `<scene>_water_zones_are_semantic_water`, `street_graph_public_matches_old` | |

## 8. Milestones

Effort is about 30 to 33 days across repos, plus about 3 csg-rs-days and 2 to 3 days of tuning
with the operator.

| Milestone | Repo | Change | Verify |
| --- | --- | --- | --- |
| **M0** Reproduce (0.5 d) | nv-rs | Headless metrics of today's rails on Gundo | Recorded in `audit/2026-11-24/rail_gen_m0.md` |
| **M1** csg-rs data (3 d) | csg-rs | R-A1, R-A8, R-A2, R-A9, R-A11 | csg-rs tests; nv-rs pin bump plus clippy |
| **M2** `camera-cine` (4 d) | camera-rs | State, lens, ease, transition engine, moves, track, player; Catmull-Rom/SQUAD/LUT moved in | camera-cine tests; `python bootstrap.py g` (camera-rs gate) |
| **M3** `shot-space` (3 d) | shot-rs | Columns, ESDF, skeleton, classes, street bands, roof envelope | shot-space tests |
| **M4** `shot-interest` + `shot-harmony` (4.5 d) | shot-rs | 4.7, 4.8 | Unit tests on synthetic inputs |
| **M5** `shot-plan` catalog + generators (3 d) | shot-rs | Generators per class including `StreetRun` and `BandChange`; probing through the `ViewProber` trait; duration models | Generator invariants; catalog diversity |
| **M6** Transition solver + sequencer (4 d) | shot-rs | 4.9, 4.10 | `duration_honoured`, `grammar_valid`, `rates_within_limits`, `deterministic_bytes` on fixtures |
| **M7** nv-demo integration (3.5 d) | nv-rs | `SceneOracle`/`ViewProber`/`ObjectTable` glue; `Track` deck; `RailSpec` conversion; UI; bench; deletion of the old rail call sites; interim filter of auto rails | nv-demo tests; workspace clippy and test; bench CSV |
| **M8** Gates and tuning (2.5 d) | all | All scenes end to end; calibrate `q_spectacular`, band speeds, `L_shot` | Section 7 green; operator review session |
| **M9** Creative mode (2 d, VLM +1 d optional) | shot-rs / nv-rs | Pairwise labeling, Bradley-Terry fit to new weight versions, curation replay | Held-out pair accuracy `>= 75 %`; replay determinism |
| **M10** Cleanup (after M8) | csg-rs, nv-rs | R-A5: remove scene auto rails and the archetype generators; remove the nv-demo filter and the old `CameraRail` spline code | No `rail_gen::generate_*` and no duplicate spline code remain |

**Out of scope:** a runtime-rewirable node editor (the adapter comes later), depth-of-field
rendering in nv-demo (the `focus_m`/`f_stop` state is carried, not rendered), differentiable or
learned 3D fields, and dynamic-traffic avoidance at playback time (planning uses frozen frame-0
poses, see risk 9.3).

## 9. Risks and open questions

1. **camera-rs scope.** Adding `camera-cine` extends "the home of camera navigation" to
   cinematics, which needs the camera-rs owner's agreement. Fallback: the same crate as a new
   cglibs repo `cine-rs`, with no design change.
2. **New repo `shot-rs`.** It needs a home on GitHub with SSH git refs, a `deny.toml` entry in
   nv-rs, and its own CI gate.
3. **Asphalt skim against moving traffic.** The plan is made on frozen frame-0 poses. During live
   playback, vehicles move into the skim path. Mitigation: skim only at least 1.5 m from lane
   centrelines (median, shoulder, pedestrian streets). An optional playback rule would pause
   traffic spawning on the skim lane while the shot runs; that is deterministic because it is part
   of the track, but it needs a traffic API, so it is deferred.
4. **Cost.** Space map plus catalog probing plus DP is estimated at 3 to 6 s CPU on Gundo at scene
   load (render worker), cached per scene. Re-planning for a new duration or seed reuses the space
   map and catalog, about 1 to 3 s. Measured in M3, M5 and M6.
5. **Long-lens shots need long clear sightlines**, 300 to 1500 m. Gundo's coast supports them; dense
   OSM cores may yield few. The catalog then simply has fewer long-lens shots, and lens variety
   becomes a soft cost only.
6. **Semantics drive the masks** (R-A2 guard).
7. **Cassette length is now a user choice.** Bench ids embed `D`. Old `PERF:RT` numbers are not
   comparable.
8. **Research summaries marked abstract-level** are used only for the ideas stated in them.
