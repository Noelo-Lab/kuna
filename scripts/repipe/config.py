"""Environment-driven configuration for the RE-friction loop.

Every knob has a default that works on this machine and is overridable by an env var, in
the idiom of scripts/pipeline/config.py. Nothing here imports a third-party package: kuna's
Python tooling is stdlib-only by convention (no pyproject.toml, no requirements.txt, and
~/.virtualenvs/kuna carries zero third-party packages).
"""
import os
import shutil
from pathlib import Path

from .. import paths

# --- repo + state -----------------------------------------------------------

STATE_DIRNAME = os.environ.get("REPIPE_STATE_DIRNAME", ".kuna-repipe")


def repo_root() -> Path:
    return paths.repo_root()


def state_dir() -> Path:
    """Live state (gitignored): inventory, rounds, arenas, worktrees, logs."""
    env = os.environ.get("KUNA_PIPELINE_STATE_DIR")
    if env:
        return Path(env)
    return repo_root() / STATE_DIRNAME


def needs_dir() -> Path:
    """The durable, COMMITTED backlog. No gate reads it (check_spec.py walks docs/spec/ only)."""
    return repo_root() / "docs" / "re-needs"


def rejected_dir() -> Path:
    """The honest denominator: already-supported, not-reproducible, unprobeable, user-error."""
    return needs_dir() / "rejected"


def cli_tests_dir() -> Path:
    """Promoted acceptance probes. A shipped need leaves a permanent regression test here."""
    return repo_root() / "tests" / "cli"


def rounds_dir() -> Path:
    return state_dir() / "rounds"


def arena_dir() -> Path:
    return state_dir() / "arena"


def runs_dir() -> Path:
    return state_dir() / "runs"


def logs_dir() -> Path:
    return state_dir() / "logs"


# --- the dataset ------------------------------------------------------------

DEFAULT_DATASET = str(Path.home() / "github" / "kuna-re-dataset")


def dataset_root() -> Path:
    return Path(os.environ.get("REPIPE_DATASET", DEFAULT_DATASET))


def manifest_path() -> Path:
    return dataset_root() / "manifest.json"


# --- tools ------------------------------------------------------------------

def kuna_bin() -> Path:
    """The release kuna the testers and probes drive."""
    env = os.environ.get("REPIPE_KUNA")
    if env:
        return Path(env)
    return repo_root() / "decompiler" / "target" / "release" / "kuna"


def specs_dir() -> Path:
    return paths.specs_dir()


# The reference stack. IDA Pro 9.2 is installed here; its python bindings live in the
# decbench venv, NOT in the default python3 and NOT in ~/.virtualenvs/kuna.
DEFAULT_REF_PYTHON = str(Path.home() / ".virtualenvs" / "decbench" / "bin" / "python")
DEFAULT_DECOMPILER = str(Path.home() / ".virtualenvs" / "decbench" / "bin" / "decompiler")


def reference_python() -> str:
    return os.environ.get("REPIPE_REF_PYTHON", DEFAULT_REF_PYTHON)


def decompiler_cli() -> str:
    return os.environ.get("REPIPE_DECOMPILER", DEFAULT_DECOMPILER)


def ida_available() -> bool:
    return os.path.exists(decompiler_cli()) and bool(os.environ.get("IDA_INSTALL_DIR", ""))


def sandbox_mode() -> str:
    """'bwrap' hides the dataset from the tester outright; 'none' falls back to the tripwire."""
    mode = os.environ.get("REPIPE_SANDBOX", "auto")
    if mode == "auto":
        return "bwrap" if shutil.which("bwrap") else "none"
    return mode


# --- agents -----------------------------------------------------------------

MAX_AGENTS = int(os.environ.get("REPIPE_MAX_AGENTS", "7"))
TESTER_SHARE = float(os.environ.get("REPIPE_TESTER_SHARE", "0.5"))


def agent_split(max_agents=None, testers=None, builders=None):
    """1 captain + testers + builders, summing to at most max_agents.

    Default 7 -> 1/3/3. Explicit --testers/--builders override the share but are still
    capped at max_agents-1 in total.

    The pools are separate caps, and testers and builders are active in different captain
    states, so below max_agents=5 (where the two tracks cannot overlap) the live process
    count is captain + max(testers, builders), not captain + testers + builders.
    """
    n = int(max_agents if max_agents is not None else MAX_AGENTS)
    n = max(2, n)
    workers = max(1, n - 1)
    if testers is None and builders is None:
        t = max(1, int(workers * TESTER_SHARE + 0.5))
        b = max(1, workers - t)
    elif testers is None:
        b = max(1, min(int(builders), workers - 1))
        t = max(1, workers - b)
    elif builders is None:
        t = max(1, min(int(testers), workers - 1))
        b = max(1, workers - t)
    else:
        t, b = max(1, int(testers)), max(1, int(builders))
    while t + b > workers and b > 1:
        b -= 1
    while t + b > workers and t > 1:
        t -= 1
    return {"captain": 1, "testers": t, "builders": b, "max_agents": n}


# --- budgets and timeouts ---------------------------------------------------

ROUND_CHALLENGES = int(os.environ.get("REPIPE_ROUND_CHALLENGES", "9"))
TESTER_TIMEOUT = int(os.environ.get("REPIPE_TESTER_TIMEOUT", "3600"))
BUILDER_TIMEOUT = int(os.environ.get("REPIPE_BUILDER_TIMEOUT", "7200"))
CAPTAIN_TIMEOUT = int(os.environ.get("REPIPE_CAPTAIN_TIMEOUT", "1200"))
BUILDER_USD = float(os.environ.get("REPIPE_BUILDER_USD", "25"))
ROUND_USD = float(os.environ.get("REPIPE_ROUND_USD", "150"))
RUN_USD = float(os.environ.get("REPIPE_RUN_USD", "1500"))
ROUNDS = int(os.environ.get("REPIPE_ROUNDS", "3"))          # 0 = until the backlog is dry
MIN_FREE_GB = int(os.environ.get("REPIPE_MIN_FREE_GB", "250"))
HALT_FREE_GB = int(os.environ.get("REPIPE_HALT_FREE_GB", "60"))
POLL = float(os.environ.get("REPIPE_POLL", "15"))

# Probe replay. A probe is run REPLAY_REPS times and must agree with itself; a flaky probe
# is not evidence. Timing probes need more reps because single-target timing noise on this
# machine routinely exceeds 5% (docs/features/returncopysplit/record.json measured a -20%
# and -12% "noise floor" on byte-identical output).
REPLAY_REPS = int(os.environ.get("REPIPE_REPLAY_REPS", "3"))
TIMING_REPS = int(os.environ.get("REPIPE_TIMING_REPS", "7"))
PROBE_TIMEOUT = int(os.environ.get("REPIPE_PROBE_TIMEOUT", "300"))

# Refutation. `absence` needs ("there is no xrefs subcommand") have no interesting root
# cause, so paying two agent-runs to confirm that is waste. Shipped ON.
REFUTE_MODE = os.environ.get("REPIPE_REFUTE_MODE", "absence-skip")

ENABLE_IDA = os.environ.get("REPIPE_ENABLE_IDA", "1") not in ("0", "false", "no")

# --- models -----------------------------------------------------------------

TESTER_MODEL = os.environ.get("REPIPE_TESTER_MODEL", "")     # "" -> codex config default
BUILDER_MODEL = os.environ.get("REPIPE_BUILDER_MODEL", "opus")
CAPTAIN_MODEL = os.environ.get("REPIPE_CAPTAIN_MODEL", "opus")

GH_REPO = os.environ.get("REPIPE_GH_REPO", "Noelo-Lab/kuna")
