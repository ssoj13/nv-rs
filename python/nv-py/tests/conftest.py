"""Session setup for the nv-rs test suite.

PyTorch ships its own cuDNN 9 build. On Windows two different cuDNN builds cannot share one
process: the loader binds cuDNN's bare-name imports to whichever copy was loaded first. nv-rs
adopts an already-loaded cuDNN (``crates/cudnn-sys``), so the supported order is "torch first";
if nv-rs loads cuDNN first, a later ``import torch`` fails with WinError 127. That order cannot
be fixed from Rust, so the suite imports torch (when installed) before any test module loads
nv-rs, exactly as an application mixing both libraries must do.
"""

try:
    import torch  # noqa: F401  (imported for its side effect: loads torch's cuDNN first)
except ImportError:
    pass
