"""Locate the kuna repo root and its built artifacts.

The package is installed editable (`pip install -e .`) from the repo, so the repo
root is the parent of this package directory. Environment overrides are honored so
the tooling can point at an out-of-tree build (e.g. the /tmp baseline tree).
"""
import os
from pathlib import Path


def repo_root() -> Path:
    """Return the kuna repository root (override with KUNA_ROOT)."""
    env = os.environ.get("KUNA_ROOT")
    if env:
        return Path(env).resolve()
    return Path(__file__).resolve().parents[1]


def cpp_dir() -> Path:
    return repo_root() / "decompiler" / "cpp"


def specs_dir() -> Path:
    """Root passed as -sleighpath / SLEIGHHOME (contains Ghidra/Processors/...)."""
    env = os.environ.get("KUNA_SPECS")
    if env:
        return Path(env).resolve()
    return repo_root() / "specs"


def datatests_dir() -> Path:
    return repo_root() / "decompiler" / "datatests"


def stage_datatests_dir() -> Path:
    """kuna-owned stage-model issue testcases (see tests/stages/README.md)."""
    return repo_root() / "tests" / "stages"


def binary(name: str, env_var: str = None) -> Path:
    """Resolve a built binary, honoring an env override and the engine switch.

    Resolution order: an explicit per-binary override (``env_var``, e.g.
    KUNA_DECOMP_DBG) always wins; else ``KUNA_ENGINE=rust`` selects the Rust
    port's build at ``rust/target/<KUNA_RUST_PROFILE or release>/<name>``
    (the Rust binaries use the same names); else the C++ build under
    ``decompiler/cpp/``.
    """
    if env_var:
        env = os.environ.get(env_var)
        if env:
            return Path(env).resolve()
    if os.environ.get("KUNA_ENGINE") == "rust":
        profile = os.environ.get("KUNA_RUST_PROFILE") or "release"
        return repo_root() / "rust" / "target" / profile / name
    return cpp_dir() / name


def decomp_dbg() -> Path:
    return binary("decomp_dbg", "KUNA_DECOMP_DBG")


def decomp_test_dbg() -> Path:
    return binary("decomp_test_dbg", "KUNA_DECOMP_TEST")


def sleigh() -> Path:
    return binary("sleigh_opt", "KUNA_SLEIGH")
