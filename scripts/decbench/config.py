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


def _suffix(base: str) -> str:
    """Pool file suffix: the angr pool keeps the original unsuffixed names."""
    return "" if base == "angr" else f"-{base}"


def cases_path(base: str = "angr") -> Path:
    return campaign_dir() / f"cases{_suffix(base)}.json"


def missing_path(base: str = "angr") -> Path:
    return campaign_dir() / f"cases-missing{_suffix(base)}.json"


def queue_path(base: str = "angr") -> Path:
    return campaign_dir() / f"triage-queue{_suffix(base)}.json"


def backlog_path(base: str = "angr") -> Path:
    return campaign_dir() / f"backlog{_suffix(base)}.md"


def novel_path() -> Path:
    return campaign_dir() / "novel.json"


def pool_paths() -> list[Path]:
    """Every file a case id may live in (triage resolves a case against these)."""
    if not campaign_dir().is_dir():
        return []
    return sorted(campaign_dir().glob("triage-queue*.json")) + \
        sorted(campaign_dir().glob("cases*.json")) + \
        sorted(campaign_dir().glob("novel*.json"))


def src_sizes_path() -> Path:
    """Cache of per-function SOURCE CFG sizes (see scripts/decbench/srcsizes.py)."""
    return Path(os.environ.get(
        "KUNA_DECBENCH_SRC_SIZES",
        str(Path.home() / ".cache" / "decbench" / "kuna-src-sizes.json"),
    ))


def load_src_sizes() -> dict:
    """{"by_function": {...}, "by_case": {...}} — see scripts/decbench/srcsizes.py."""
    path = src_sizes_path()
    if not path.is_file():
        return {}
    import json
    doc = json.loads(path.read_text())
    return doc if "by_function" in doc else {"by_function": doc, "by_case": {}}


def source_cfg(sizes: dict, opt: str, project: str, binary: str, function: str) -> dict:
    """Best available source-CFG facts for one case (see srcsizes.py for scope)."""
    exact = sizes.get("by_case", {}).get(f"{opt}::{project}::{binary}::{function}")
    if exact:
        return {"source_nodes": exact[0], "source_edges": exact[1],
                "source_exact": True, "approximated": bool(exact[2]),
                "source_ambiguous": False}
    bound = sizes.get("by_function", {}).get(f"{project}::{function}")
    if not bound:
        return {"source_nodes": None, "source_edges": None, "source_exact": False,
                "approximated": False, "source_ambiguous": False}
    return {"source_nodes": bound[0], "source_edges": bound[1], "source_exact": False,
            "approximated": False, "source_ambiguous": bound[2] > 1}


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


_IMAGE_BASE: dict[str, int | None] = {}


def image_base(binary_path: str | None) -> int | None:
    """PE ImageBase of a corpus binary; None for anything that is not a PE.

    Detected from the file's own headers (``MZ`` + ``PE\\0\\0`` at ``e_lfanew``),
    never from the name — the corpus binaries are stripped and not all are named
    ``.exe``. The base differs per binary (``mydoom.exe`` 0x400000,
    ``dexter.dll`` 0x69940000), so it is always read, never assumed.
    """
    if not binary_path:
        return None
    if binary_path not in _IMAGE_BASE:
        _IMAGE_BASE[binary_path] = _read_image_base(binary_path)
    return _IMAGE_BASE[binary_path]


def _read_image_base(binary_path: str) -> int | None:
    try:
        with open(binary_path, "rb") as fh:
            if fh.read(2) != b"MZ":
                return None
            fh.seek(0x3C)
            fh.seek(int.from_bytes(fh.read(4), "little"))
            if fh.read(4) != b"PE\0\0":
                return None
            fh.seek(20, 1)  # past the COFF header, at the optional header
            magic = int.from_bytes(fh.read(2), "little")
            if magic == 0x10B:  # PE32: ImageBase at optional+28
                fh.seek(26, 1)
                return int.from_bytes(fh.read(4), "little")
            if magic == 0x20B:  # PE32+: ImageBase at optional+24
                fh.seek(22, 1)
                return int.from_bytes(fh.read(8), "little")
            return None
    except (OSError, ValueError):
        return None


def kuna_addr(binary_path: str | None, address: int | str | None) -> str | None:
    """The ``kuna --addr`` (VA) form of a mined case address.

    decbench records PE addresses as RVAs — ``function_results.json`` and every
    rival's ``// Function: <fn> @ 0x..`` marker do, while kuna's own artifacts
    use VAs — so a mined pool carries either form depending on where the address
    came from. kuna addresses a PE by VA, so an address below the image base can
    only be an RVA and is rebased; ELF (no image base) and VAs pass through.
    """
    if address is None:
        return None
    addr = int(address, 16) if isinstance(address, str) else int(address)
    base = image_base(binary_path)
    if base is not None and addr < base:
        addr += base
    return f"0x{addr:x}"
