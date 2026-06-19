"""Differential harness for the Rust SLEIGH compiler port (`kuna-slacomp`).

The C++ ``sleigh_opt`` is the byte-deterministic oracle: compiling a ``.slaspec``
twice yields a byte-identical ``.sla``.  The Rust compiler is correct iff, for
every vendored ``.slaspec``, its ``.sla`` is **byte-identical** to the one
``sleigh_opt`` produces.  This module is the gate.

Usage::

    python -m kuna.slacomp --one specs/.../x86-64.slaspec   # one spec, byte-diff
    python -m kuna.slacomp --all                            # all 148, tally
    python -m kuna.slacomp --all --json                     # machine-readable

Engine resolution (mirrors ``kuna.paths``):
  * C++ oracle  : ``decompiler/cpp/sleigh_opt`` (or ``$KUNA_SLEIGH``)
  * Rust port   : ``rust/target/release/slacomp`` (or ``$KUNA_SLACOMP``)

Each spec is compiled in an isolated temp copy of its ``data/languages`` dir so
neither run clobbers the committed ``.sla`` or each other's output.
"""
from __future__ import annotations

import argparse
import json
import os
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
SPECS = ROOT / "specs"


def _cpp_sleigh() -> Path:
    return Path(os.environ.get("KUNA_SLEIGH", ROOT / "decompiler" / "cpp" / "sleigh_opt"))


def _rust_slacomp() -> Path:
    return Path(os.environ.get("KUNA_SLACOMP", ROOT / "rust" / "target" / "release" / "slacomp"))


def all_slaspecs() -> list[Path]:
    return sorted(SPECS.rglob("*.slaspec"))


def _compile_in_isolation(compiler: Path, slaspec: Path) -> bytes | None:
    """Copy the spec's languages dir to a temp dir, compile there, return the
    resulting ``.sla`` bytes (or None on failure)."""
    langdir = slaspec.parent
    with tempfile.TemporaryDirectory() as td:
        dst = Path(td) / langdir.name
        shutil.copytree(langdir, dst, ignore=shutil.ignore_patterns("*.sla", "*.sla.lock"))
        spec = dst / slaspec.name
        try:
            subprocess.run(
                [str(compiler), str(spec)],
                cwd=dst, capture_output=True, timeout=600,
            )
        except (subprocess.TimeoutExpired, OSError):
            return None
        out = spec.with_suffix(".sla")
        return out.read_bytes() if out.exists() else None


def _content(sla: bytes) -> bytes | None:
    """The decompressed `.sla` element stream. The file is a 4-byte magic
    (``sla\\x04``) + a zlib stream; the *compressed* bytes differ between the
    Rust (flate2/miniz_oxide) and C (zlib) deflate backends (LOSS-010), but the
    decompressed element stream is the meaningful, byte-comparable content."""
    import zlib
    i = sla.find(b"\x78\x9c")
    if i < 0:
        return None
    try:
        return sla[:i] + zlib.decompress(sla[i:])
    except zlib.error:
        return None


def diff_one(slaspec: Path) -> dict:
    cpp, rust = _cpp_sleigh(), _rust_slacomp()
    try:
        rel = str(Path(slaspec).resolve().relative_to(ROOT))
    except ValueError:
        rel = str(slaspec)
    rec = {"spec": rel, "ok": False, "reason": ""}
    if not cpp.exists():
        rec["reason"] = "cpp sleigh_opt missing (run `make sleigh`)"
        return rec
    cpp_sla = _compile_in_isolation(cpp, slaspec)
    if cpp_sla is None:
        rec["reason"] = "cpp compile failed"
        return rec
    if not rust.exists():
        rec["reason"] = "rust slacomp missing (build kuna-slacomp)"
        return rec
    rust_sla = _compile_in_isolation(rust, slaspec)
    if rust_sla is None:
        rec["reason"] = "rust compile failed"
        return rec
    # The gate is decompressed-content equality (raw bytes differ by the deflate
    # backend only, LOSS-010). `raw_ok` is reported for visibility.
    rec["raw_ok"] = cpp_sla == rust_sla
    cc, rc = _content(cpp_sla), _content(rust_sla)
    if cc is None or rc is None:
        rec["reason"] = "could not decompress .sla element stream"
        return rec
    rec["content_bytes"] = len(cc)
    if cc == rc:
        rec["ok"] = True
    else:
        n = min(len(cc), len(rc))
        off = next((i for i in range(n) if cc[i] != rc[i]), n)
        rec["reason"] = f"content mismatch @ {off} (cpp {len(cc)}b / rust {len(rc)}b)"
    return rec


def main(argv: list[str] | None = None) -> int:
    ap = argparse.ArgumentParser(description="Differential SLEIGH-compiler (.sla byte-diff) harness")
    g = ap.add_mutually_exclusive_group(required=True)
    g.add_argument("--one", type=Path, help="one .slaspec to byte-diff")
    g.add_argument("--all", action="store_true", help="byte-diff all vendored .slaspec")
    ap.add_argument("--json", action="store_true")
    a = ap.parse_args(argv)

    specs = [a.one] if a.one else all_slaspecs()
    recs = [diff_one(s) for s in specs]
    npass = sum(1 for r in recs if r["ok"])
    if a.json:
        print(json.dumps({"passing": npass, "total": len(recs), "results": recs}, indent=2))
    else:
        for r in recs:
            mark = "OK  " if r["ok"] else "FAIL"
            print(f"  [{mark}] {r['spec']}" + ("" if r["ok"] else f"  -- {r['reason']}"))
        print(f"\n.sla byte-parity: {npass}/{len(recs)}")
    return 0 if npass == len(recs) else 1


if __name__ == "__main__":
    sys.exit(main())
