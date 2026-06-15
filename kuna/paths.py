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


def binary(name: str, env_var: str = None, engine: str = None) -> Path:
    """Resolve a built binary, honoring an env override and the engine switch.

    Resolution order: an explicit per-binary override always wins. When an
    ``engine`` is explicitly requested (``"cpp"`` / ``"rust"``), an engine-suffixed
    override (``<env_var>_CPP`` / ``<env_var>_RUST``, e.g. ``KUNA_DECOMP_DBG_RUST``)
    is consulted first -- this lets one process point each engine at its own
    out-of-tree binary, which is what the pipeline's cross-engine comparison needs
    (a single bare ``KUNA_DECOMP_DBG`` can only name one binary). The bare
    ``<env_var>`` applies only in the ambient case (no explicit ``engine``), so it
    keeps the single-engine ``KUNA_ENGINE=rust`` + ``KUNA_DECOMP_DBG`` workflow
    intact without hijacking the *other* engine during a comparison.

    Absent an override, the engine selects the build -- an explicit ``engine``
    overrides the ambient ``KUNA_ENGINE`` env var, so a single process can resolve
    both engines' binaries without mutating the environment. ``rust`` selects the
    Rust port's build at ``rust/target/<KUNA_RUST_PROFILE or release>/<name>`` (the
    Rust binaries use the same names); ``cpp`` (the default) selects the C++ build
    under ``decompiler/cpp/``.
    """
    selected = engine if engine is not None else os.environ.get("KUNA_ENGINE")
    if env_var:
        if engine is not None:
            # Engine-specific override (e.g. KUNA_DECOMP_DBG_RUST) -- so a single
            # process can give each engine its own binary during a comparison.
            suffixed = os.environ.get("%s_%s" % (env_var, engine.upper()))
            if suffixed:
                return Path(suffixed).resolve()
        else:
            # Ambient (no explicit engine): the bare override names the one engine
            # in play. Don't apply it when an explicit engine was requested, or it
            # would clobber the *other* engine's binary mid-comparison.
            bare = os.environ.get(env_var)
            if bare:
                return Path(bare).resolve()
    if selected == "rust":
        profile = os.environ.get("KUNA_RUST_PROFILE") or "release"
        return repo_root() / "rust" / "target" / profile / name
    return cpp_dir() / name


def decomp_dbg() -> Path:
    return binary("decomp_dbg", "KUNA_DECOMP_DBG")


def decomp_test_dbg() -> Path:
    return binary("decomp_test_dbg", "KUNA_DECOMP_TEST")


def sleigh() -> Path:
    return binary("sleigh_opt", "KUNA_SLEIGH")
