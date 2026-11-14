# Contributing to nv-rs

Thank you for contributing to `nv-rs`. Every contribution must meet the project's licensing,
toolchain, and verification standards.

## Developer Certificate of Origin (DCO) & CLA

All commits **must** carry a Developer Certificate of Origin (DCO) sign-off:

```bash
git commit -s -m "your commit message"
```

This appends a `Signed-off-by: Name <email>` line attesting that you have the right to submit the
work under the project's Apache-2.0 license. External contributors may also be asked to complete
the NVIDIA Contributor License Agreement (CLA) during pull request review.

Do not commit NVIDIA proprietary headers, redistributable binaries, or generated files derived
outside the permitted build/prebuilt workflow.

## Toolchain & Code Quality

- **Host Toolchain**: `1.99.1` (pinned in `rust-toolchain.toml`).
- **Edition**: Rust 2024.
- **Warnings are errors**: All workspace targets must pass `clippy` with `-D warnings` and
  `rustfmt` checks.
- **Unsafe invariants**: Every `unsafe` block requires a `// SAFETY:` comment explaining the
  upheld invariants. Prefer safe wrappers and avoid `#[allow(...)]` suppressions.
- **No silent fallbacks**: Explicit backend or feature requests must return typed errors rather
  than silently downgrading.

## Verification Before Opening a PR

Run the standard checks on Windows and, when touching Linux-targeted crates (`nccl`, `cufile`,
`nv-loader`, `nv-bindgen`, `nv-wgpu`, `ngx`), verify inside WSL2 (or native Linux) as well:

```bash
# Formatting + Clippy (-D warnings)
python bootstrap.py c
# or directly:
cargo +1.99.1 fmt --all --check
cargo +1.99.1 clippy --workspace --all-targets -- -D warnings

# Unit tests (T0)
cargo +1.99.1 test --workspace --lib

# Dependency & license audit
cargo deny check
```

For WSL2 verification:

```bash
cargo +1.99.1 fmt --all --check
cargo +1.99.1 clippy --workspace --all-targets -- -D warnings
cargo +1.99.1 test --workspace --lib
```

GPU integration tests (`#[ignore]`) should be run serialized on a machine with an NVIDIA GPU
via `python bootstrap.py t --gpu` or `cargo xtask gpu-test`.
