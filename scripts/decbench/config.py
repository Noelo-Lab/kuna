"""Environment-driven configuration for the decbench improvement loop.

Defaults fit this machine; every knob is overridable so the loop survives a
venv/results move with no code change.

    KUNA_DECBENCH_PYTHON   interpreter that can `import decbench` (the decbench venv)
    KUNA_DECBENCH_REPO     the decbench checkout (for the `decbench` CLI + scripts)
    KUNA_DECBENCH_RESULTS  the results tree the campaign mines (function_results.json)
    KUNA_BIN               the kuna CLI binary (defaults to the release build)
"""
import os
import re
from pathlib import Path

from .. import paths

DEFAULT_DECBENCH_PYTHON = str(Path.home() / ".virtualenvs" / "decbench" / "bin" / "python")
DEFAULT_DECBENCH_REPO = str(Path.home() / "github" / "decbench")
DEFAULT_RESULTS = str(Path.home() / "github" / "decbench" / "results" / "full_run")


def decbench_python() -> str:
    return os.environ.get("KUNA_DECBENCH_PYTHON", DEFAULT_DECBENCH_PYTHON)


def decbench_repo() -> Path:
    return Path(os.environ.get("KUNA_DECBENCH_REPO", DEFAULT_DECBENCH_REPO))


def decbench_cli() -> str:
    """The `decbench` console script that sits next to the venv python."""
    return str(Path(decbench_python()).parent / "decbench")


def results_root() -> Path:
    return Path(os.environ.get("KUNA_DECBENCH_RESULTS", DEFAULT_RESULTS))


def kuna_bin() -> str:
    env = os.environ.get("KUNA_BIN")
    if env:
        return env
    return str(paths.repo_root() / "decompiler" / "target" / "release" / "kuna")


def repo_root() -> Path:
    return paths.repo_root()


def campaign_dir() -> Path:
    return repo_root() / "docs" / "decbench"


def cases_path() -> Path:
    return campaign_dir() / "cases.json"


def missing_path() -> Path:
    return campaign_dir() / "cases-missing.json"


def queue_path() -> Path:
    return campaign_dir() / "triage-queue.json"


def triage_dir() -> Path:
    return campaign_dir() / "triage"


_SLUG_RE = re.compile(r"[^a-zA-Z0-9_.-]+")


def case_id(opt: str, project: str, binary_stem: str, function: str) -> str:
    """Stable per-case id: <opt>-<project>-<stem>-<function> (slugified)."""
    raw = f"{opt}-{project}-{binary_stem}-{function}"
    return _SLUG_RE.sub("_", raw)


def group_id(project: str, function: str) -> str:
    """Dedup key: the same source function across binaries/opt levels."""
    return f"{project}::{function}"


def stripped_path(binary_path: str) -> str:
    """The stripped sibling of a results-tree compiled binary path."""
    return binary_path.replace("/compiled/", "/stripped/")
