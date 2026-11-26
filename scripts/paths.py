"""Locate the kuna repo root and its built artifacts.

These helpers back the still-Python feature pipeline (``scripts/pipeline/``).
Run the pipeline from the repo root (``python -m scripts.pipeline.<mod>``) so the
``scripts`` package is importable; the repo root is the parent of this package
directory. Environment overrides are honored so the tooling can point at an
out-of-tree build.
"""
import os
from pathlib import Path


def repo_root() -> Path:
    """Return the kuna repository root (override with KUNA_ROOT)."""
    env = os.environ.get("KUNA_ROOT")
    if env:
        return Path(env).resolve()
    return Path(__file__).resolve().parents[1]


def specs_dir() -> Path:
    """Root passed as -sleighpath / SLEIGHHOME (contains Ghidra/Processors/...)."""
    env = os.environ.get("KUNA_SPECS")
    if env:
        return Path(env).resolve()
    return repo_root() / "specs"


def datatests_dir() -> Path:
    return repo_root() / "tests" / "datatests"


def stage_datatests_dir() -> Path:
    """kuna-owned stage-model issue testcases (see tests/stages/README.md)."""
    return repo_root() / "tests" / "stages"


def binary(name: str, env_var: str = None) -> Path:
    """Resolve a built engine binary, honoring a per-binary env override.

    An explicit ``$<env_var>`` override (e.g. ``KUNA_DECOMP_DBG``) always wins;
    otherwise the binary is the Rust build at
    ``decompiler/target/<KUNA_RUST_PROFILE or release>/<name>``. The SLEIGH
    compiler binary is named ``slacomp`` (the decompiler binaries keep the
    upstream names ``decomp_dbg`` / ``decomp_test_dbg``).
    """
    if env_var:
        override = os.environ.get(env_var)
        if override:
            return Path(override).resolve()
    profile = os.environ.get("KUNA_RUST_PROFILE") or "release"
    exe = "slacomp" if name == "sleigh_opt" else name
    return repo_root() / "decompiler" / "target" / profile / exe


def decomp_dbg() -> Path:
    return binary("decomp_dbg", "KUNA_DECOMP_DBG")


def decomp_test_dbg() -> Path:
    return binary("decomp_test_dbg", "KUNA_DECOMP_TEST")


def sleigh() -> Path:
    return binary("sleigh_opt", "KUNA_SLEIGH")


def phases_toml(root=None) -> Path:
    """The phase registry's shared tables (groups, subphases, surfaces)."""
    return Path(root or repo_root()) / "decompiler" / "crates" / "kuna-decomp" / "phases.toml"


def phases_files(path=None) -> list:
    """``phases.toml`` then every ``phases.d/*.toml`` option row in catalog (file-name) order."""
    path = Path(path) if path else phases_toml()
    rows = sorted((path.parent / "phases.d").glob("*.toml"), key=lambda p: p.name)
    return [path] + rows


def phases_text(path=None) -> str:
    """The whole registry as one TOML document, as if every option row were in phases.toml."""
    return "\n".join(p.read_text(encoding="utf-8") for p in phases_files(path))
