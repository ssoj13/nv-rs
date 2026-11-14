# Security Policy

## Reporting a Vulnerability

Please **do not** report security vulnerabilities through public GitHub issues, discussions, or
pull requests.

Instead, report security issues privately to the NVIDIA Product Security Incident Response Team
(PSIRT):

- **Email**: [psirt@nvidia.com](mailto:psirt@nvidia.com)
- **Web**: [NVIDIA Product Security](https://www.nvidia.com/en-us/security/)

If the repository has GitHub Private Vulnerability Reporting enabled, you may also use the
**Security -> Advisories -> Report a vulnerability** workflow on GitHub.

## What to Include

To help us triage and reproduce the issue quickly, please include:

- Affected crate(s) and commit/version.
- Target triple, operating system (Windows / Linux / WSL2), GPU model, and NVIDIA driver / CUDA /
  SDK versions.
- A minimal reproduction case or proof of concept.
- Description of the impact (e.g., unsoundness in safe wrappers, out-of-bounds memory access,
  FFI lifetime violation, path traversal in tooling).

## Scope Notes

- `nv-rs` wraps native NVIDIA libraries via FFI (`*-sys` crates and `nv-loader`). Unsoundness in
  safe Rust APIs (violations of Rust's aliasing, lifetime, or thread-safety guarantees without
  `unsafe`) is treated as a security bug.
- Vulnerabilities in underlying proprietary NVIDIA drivers or SDKs are routed through NVIDIA PSIRT.
