# Chaos AI Talk Plan — December 10, San Francisco
30 minutes, four blocks of seven.

## Block 1. Squarebob → render-rs → nv-rs: how I got to the metal

Squarebob — disk-usage viewer, created in May. Its renderer, render-rs, is what forced me into the GPU. From there: nv-rs — 47 host crates, 2 device crates, OptiX, DLSS, nvenc, NPP, NVML, NVAPI. Five merged PRs into official cuda-rust. Not stars — a trace in their code.

Key line: "I didn't choose NVIDIA. NVIDIA grew out of my pain."

## Block 2. Warpbro — tech demo. Then csg-rs — the magician's rabbit

Warpbro: 28 fractal families (mandelbulb, kleinian, menger sponge, gyroid). Pure Rust kernels via cuda-oxide, zero shaders. Two denoisers in VRAM. Zero-copy via nv-wgpu. 69 materials from render-rs standard-surface-bsdf. Animation, timeline, REST API, CUDA-vs-WGPU parity tests.

Fractal is the perfect stress test: no geometry at all, just math. If path tracing a fractal flies with DLSS and OptiX, a CSG city is a logical next step, not a leap.

Then csg-rs. Render-time analytical CSG, not a single triangle. Exact normals. Bit-exact C-ABI buffers between CPU golden reference and CUDA. 136 tests, 33 Kani harnesses, zero spill in hot kernels. Real places: El Segundo, Marina del Rey, San Francisco from open maps. Road seams flush, smoothed. Wave 9 actors: five-fingered hand, crowd, pedestrian-crossing sign, five-segment-tail dog. All pure math, self-balancing, walking sidewalks generated from the same roads. Export to robotics simulator formats. Scene 109 in the inspector — live showcase of all four.

City is the stage. People make it alive.

Pause here. Let the audience sit with the megabyte number.

## Block 3. Gracefold and tars-rs — the thread continues

Gracefold: 37 crates, inference and training in pure Rust. Qwen3-4B — 89 tokens/sec vs 12 in PyTorch. On B300 — 116K tokens/sec during training. AeroInspect: 161 fps, training on one Blackwell in 7 hours for $19.50. Hy-Nomas — world model generating action-conditioned video, validated on 30-second clips.

tars-rs: one no-std kernel across CUDA, WGPU, and CPU. 1.5M sim steps/sec on LEAP Hand. 38M on Cartpole. 43 Kani proofs, 209 checks. USD, URDF, MJCF import. ARS and PPO policy training in pure Rust. Live hardware: SO-101 with tactile sensors, Unitree G1.

## Block 4. The stitch and the ask

All four are one principle: pure Rust down to silicon, zero Python in the runtime, formal verification, GPU-resident with no copies. Twenty minutes on the thread, ten on one live example — tars with SO-101. A balancing robot beats any benchmark slide.

Close with the question: what does NVIDIA itself want from Rust on device?

Teaser: the demo can be ported to iPad.

## Delivery notes

- Squarebob spinning on a background screen as a KDPV prop. People walk up and ask.
- Backup slides: dependency diagram from Squarebob to tars.
- If the room is cold, pull Squarebob in the first two minutes.
- Do not enumerate crates. The story is the principle, not the inventory.
- Demo runs 10–12 minutes, not 20.
- Leave five minutes for questions — the best conversations start there.
