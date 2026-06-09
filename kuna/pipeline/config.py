"""Environment-driven configuration for the kuna pipeline.

Every knob has a sensible default for this machine but is overridable by an env var, so
the pipeline runs today against the installed angr 9.2.213 (decbench venv) and can be
re-pointed at a Python-3.12 exact-checkout venv (angr 9.2.222) later with no code change.

    KUNA_PIPELINE_ANGR_PYTHON   interpreter that can `import angr` (default: decbench venv)
    KUNA_PIPELINE_ANGR_REPO     the angr source checkout (for the test file)
    KUNA_PIPELINE_BIN_ROOT      angr's test-binaries root (bin_location)
    KUNA_PIPELINE_TEST_FILE     the reference testcase file to mine
    KUNA_PIPELINE_STATE_DIR     runtime worker state (heartbeats/inventory), gitignored
"""
import os
from pathlib import Path

from .. import paths

# --- angr runtime + sources -------------------------------------------------

# The installed (NON-editable) angr that imports cleanly on this box's Python 3.10.
# The editable checkout (9.2.222) needs Python 3.12 (PEP 695 syntax) -- unavailable here.
DEFAULT_ANGR_PYTHON = str(Path.home() / ".virtualenvs" / "decbench" / "bin" / "python")
DEFAULT_ANGR_REPO = "/home/mahaloz/github/angr-dev/angr"
DEFAULT_BIN_ROOT = "/home/mahaloz/github/angr-dev/binaries"


def angr_python() -> str:
    return os.environ.get("KUNA_PIPELINE_ANGR_PYTHON", DEFAULT_ANGR_PYTHON)


def angr_repo() -> Path:
    return Path(os.environ.get("KUNA_PIPELINE_ANGR_REPO", DEFAULT_ANGR_REPO))


def bin_root() -> Path:
    """angr's tests.common.bin_location == <repo>/../binaries."""
    return Path(os.environ.get("KUNA_PIPELINE_BIN_ROOT", DEFAULT_BIN_ROOT))


def test_location() -> Path:
    """test_decompiler.py's test_location == bin_location/tests."""
    return bin_root() / "tests"


def test_file() -> Path:
    env = os.environ.get("KUNA_PIPELINE_TEST_FILE")
    if env:
        return Path(env)
    return angr_repo() / "tests" / "analyses" / "decompiler" / "test_decompiler.py"


# --- kuna repo locations ----------------------------------------------------

def repo_root() -> Path:
    return paths.repo_root()


def pipeline_docs_dir() -> Path:
    return repo_root() / "docs" / "pipeline"


def features_dir() -> Path:
    return repo_root() / "docs" / "features"


def state_dir() -> Path:
    """Runtime worker state (gitignored): inventory + per-worker heartbeats."""
    env = os.environ.get("KUNA_PIPELINE_STATE_DIR")
    if env:
        return Path(env)
    return repo_root() / ".kuna-pipeline"


# --- knobs ------------------------------------------------------------------

# Per-call wall-clock budgets (seconds). angr is 0.2-3.6s on small ELFs; cap generously.
ANGR_TIMEOUT = int(os.environ.get("KUNA_PIPELINE_ANGR_TIMEOUT", "180"))
KUNA_TIMEOUT = int(os.environ.get("KUNA_PIPELINE_KUNA_TIMEOUT", "120"))

# The reference structurer to pin for deterministic single-valued output (angr default).
DEFAULT_STRUCTURER = os.environ.get("KUNA_PIPELINE_STRUCTURER", "")  # "" -> angr default (sailr)
