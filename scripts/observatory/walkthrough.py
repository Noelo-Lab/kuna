"""Bytes -> C walkthrough: run one real function through the engine and keep
every intermediate artifact. Bytes and disassembly come from the kuna CLI;
the p-code/IR/region/C stages from one scripted decomp_dbg run using the
console's openfile-write redirection (no output parsing). Raw dumps land in
observatory/walkthrough/NN-*.txt beside walkthrough.json."""
import os
import subprocess
from pathlib import Path

from scripts.observatory import common
from scripts.paths import specs_dir

FIXTURE = "decompiler/crates/kuna-analysis/tests/fixtures/aliasoverlap_x86_64"
FUNCTION = "main"
TIMEOUT = 120

STAGES = (
    ("bytes", "01", "The bytes", ["P1"], "the mapped image: what the loader sees"),
    ("disasm", "02", "Disassembly", ["P1"], "SLEIGH decodes instructions over the code partition"),
    ("rawpcode", "03", "Raw p-code", ["P2"], "lifted ops + CFG, before any analysis pass runs"),
    ("finalir", "04", "Final IR", ["P3", "P4", "P5", "P6"],
     "the same ops after Band B: SSA, calls, types, variables"),
    ("regions", "05", "Region tree", ["P7"], "the structured-control skeleton over the CFG"),
    ("c", "06", "C", ["P8", "P9"], "structured AST rendered as source"),
)


def _kuna_bin(root: Path) -> Path:
    override = os.environ.get("KUNA_BIN")
    if override:
        return Path(override)
    profile = os.environ.get("KUNA_RUST_PROFILE", "release")
    return root / "decompiler" / "target" / profile / "kuna"


def _run(cmd: list, env: dict, cwd: Path, stdin: str | None = None) -> str:
    result = subprocess.run(
        cmd, cwd=cwd, env=env, input=stdin, capture_output=True, text=True, timeout=TIMEOUT
    )
    if result.returncode != 0:
        raise RuntimeError(f"walkthrough: {' '.join(map(str, cmd))} failed:\n{result.stderr}")
    return result.stdout


def console_script(binary: Path, function: str, out_dir: Path) -> str:
    sections = {
        "03-rawpcode": ["print raw"],
        "04-finalir": ["print raw"],
        "05-regions": ["region tree", "region blocks"],
        "06-c": ["print C"],
    }
    lines = [f"load file {binary}", f"load function {function}"]
    for name, commands in sections.items():
        if name == "04-finalir":
            lines.append("decompile")
        lines.append(f"openfile write {out_dir / (name + '.txt')}")
        lines.extend(commands)
        lines.append("closefile")
    lines.append("quit")
    return "\n".join(lines) + "\n"


def generate(root: Path | None = None, out: Path | None = None) -> dict:
    root = root or common.repo_root()
    binary = root / FIXTURE
    kuna = _kuna_bin(root)
    out_dir = (out or root / "observatory") / "walkthrough"
    out_dir.mkdir(parents=True, exist_ok=True)
    env = {**os.environ, "SLEIGHHOME": str(specs_dir())}

    texts = {
        "bytes": _run([kuna, "read", FIXTURE, FUNCTION], env, root),
        "disasm": _run([kuna, "disassemble", FIXTURE, FUNCTION], env, root),
    }
    _run([kuna, "decomp_dbg"], env, root, stdin=console_script(binary, FUNCTION, out_dir))
    for stage, number, *_ in STAGES:
        path = out_dir / f"{number}-{stage}.txt"
        if stage in texts:
            path.write_text(texts[stage], encoding="utf-8")
        else:
            texts[stage] = path.read_text(encoding="utf-8")

    produced = {
        "bytes": f"kuna read {FIXTURE} {FUNCTION}",
        "disasm": f"kuna disassemble {FIXTURE} {FUNCTION}",
        "rawpcode": "decomp_dbg: load function + print raw (before any action runs)",
        "finalir": "decomp_dbg: decompile + print raw",
        "regions": "decomp_dbg: region tree + region blocks",
        "c": "decomp_dbg: print C",
    }
    return {
        "schemaVersion": common.SCHEMA_VERSION,
        "meta": {
            **common.meta(root),
            "binary": FIXTURE,
            "function": FUNCTION,
        },
        "stages": [
            {
                "id": stage,
                "file": f"walkthrough/{number}-{stage}.txt",
                "title": title,
                "phases": phases,
                "role": role,
                "producedBy": produced[stage],
                "text": texts[stage],
            }
            for stage, number, title, phases, role in STAGES
        ],
    }
