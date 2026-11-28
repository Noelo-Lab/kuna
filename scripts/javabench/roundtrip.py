"""Round trip of small sources: does a decompiler's text compile back to the same bytecode?

Each source is compiled alone with javac, every top-level class is decompiled, the text is
recompiled with the same flags, and the code of every method is compared (``javap -c -p`` with
constant-pool indexes dropped). A source round-trips when every method of every class is equal.
The verdict per source is one of::

    OK         every method equal
    DIFF       the text compiles, some method's code differs (or a class or method is missing)
    NOCOMPILE  javac rejects the text
    FAILED     the code is equal but the text holds a failure comment
    SKIP       the source itself does not compile with these flags

From the repo root::

    KUNA_BIN=decompiler/target/release/kuna python3 -m scripts.javabench.roundtrip
    python3 -m scripts.javabench.roundtrip --no-debug --decompiler vineflower --keep out/ SRC.java...

With no sources it runs the Java sources of kuna's own corpus. ``DIFF`` is a list to read, not a
count of defects: javac may lay out equivalent text differently, and without debug info it merges
lambda bodies that are equal.
"""

from __future__ import annotations

import argparse
import concurrent.futures
import os
import re
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path

from .decompilers import vineflower_jar

ROOT = Path(__file__).resolve().parents[2]
CORPUS_SOURCES = ROOT / "decompiler/managed/kuna-jvm/corpus/kuna-fixes/src/pkg"
DEBUG_FLAGS = ["-g", "--release", "21", "-nowarn", "-proc:none"]
NO_DEBUG_FLAGS = ["-parameters", "-g:none", "--release", "21", "-nowarn", "-proc:none"]
FAILURE = re.compile(r"\$VF:|\$FF:|couldn't be decompiled|kuna: (?:could not|couldn't) decompile")

_POOL_INDEX = re.compile(r"#\d+(,\s*\d+)?")
_OFFSET = re.compile(r"^\d+: ")
_SLOT = re.compile(
    r"^(?:(?P<op>[ailfd](?:load|store))_(?P<n>\d)"
    r"|(?P<op2>[ailfd](?:load|store)|ret)\s+(?P<n2>\d+)"
    r"|iinc\s+(?P<n3>\d+),\s*(?P<k>-?\d+))$"
)


def parse_javap(text: str) -> dict[str, list[str]]:
    """``javap -c -p`` output as member declaration -> instruction lines, offsets and pool indexes dropped."""
    members: dict[str, list[str]] = {}
    name = None
    for line in text.splitlines():
        if line.startswith("Compiled from") or line == "}":
            continue
        if line.startswith("  ") and not line.startswith("   ") and line.strip().endswith(";"):
            name = line.strip()
            members[name] = []
        elif name is not None and line.strip():
            members[name].append(_POOL_INDEX.sub("#", _OFFSET.sub("", line.strip())))
    return members


def renumber_slots(lines: list[str]) -> list[str]:
    """Local slots numbered by first appearance: javac may allocate a recompiled method's locals differently."""
    seen: dict[str, int] = {}
    out = []
    for line in lines:
        m = _SLOT.match(line)
        if not m:
            out.append(line)
            continue
        slot = seen.setdefault(m.group("n") or m.group("n2") or m.group("n3"), len(seen))
        if m.group("n3"):
            out.append(f"iinc v{slot}, {m.group('k')}")
        else:
            out.append(f"{m.group('op') or m.group('op2')} v{slot}")
    return out


def differing(original: dict[str, list[str]], recompiled: dict[str, list[str]]) -> list[str]:
    """The members of ``original`` whose code is missing from ``recompiled`` or differs beyond slot numbers."""
    return [
        m
        for m, code in original.items()
        if code != recompiled.get(m) and renumber_slots(code) != renumber_slots(recompiled.get(m, []))
    ]


def decompile(decompiler: str, classes: Path, top: str) -> str:
    """The text ``decompiler`` gives for the top-level class file ``top`` (relative to ``classes``)."""
    if decompiler == "vineflower":
        with tempfile.TemporaryDirectory() as out:
            stem = (classes / top).with_suffix("")
            nested = sorted(str(p) for p in stem.parent.glob(stem.name + "$*.class"))
            subprocess.run(
                ["java", "-jar", str(vineflower_jar()), str(classes / top), *nested, out],
                capture_output=True, timeout=600,
            )
            text = Path(out, stem.name + ".java")
            return text.read_text() if text.exists() else ""
    kuna = os.environ.get("KUNA_BIN") or str(ROOT / "decompiler/target/release/kuna")
    return subprocess.run([kuna, "decompile", str(classes / top)], capture_output=True, text=True, timeout=600).stdout


def javap(classes: Path, name: str) -> dict[str, list[str]]:
    return parse_javap(subprocess.run(["javap", "-c", "-p", "-cp", str(classes), name], capture_output=True, text=True).stdout)


def round_trip(source: Path, flags: list[str], decompiler: str, work: Path) -> tuple[str, str]:
    """``(verdict, note)`` for one source; ``work`` receives ``a`` (classes), ``t`` (text) and ``b`` (recompiled)."""
    a, t, b = work / "a", work / "t", work / "b"
    for d in (a, t, b):
        d.mkdir(parents=True)
    if subprocess.run(["javac", *flags, "-d", str(a), str(source)], capture_output=True).returncode != 0:
        return "SKIP", ""
    classes = sorted(str(p.relative_to(a)) for p in a.rglob("*.class"))
    texts, failed = [], False
    for top in (c for c in classes if "$" not in os.path.basename(c)):
        text = decompile(decompiler, a, top)
        path = (t / top).with_suffix(".java")
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(text)
        texts.append(str(path))
        failed |= bool(FAILURE.search(text))
    r = subprocess.run(["javac", *flags, "-cp", str(a), "-d", str(b), *texts], capture_output=True, text=True)
    if r.returncode != 0:
        return "NOCOMPILE", next((line.strip() for line in r.stderr.splitlines() if "error:" in line), "")[-160:]
    bad = []
    for c in classes:
        name = c[: -len(".class")].replace("/", ".")
        if not (b / c).exists():
            bad.append(f"{name} (class missing)")
        else:
            bad += [f"{name}#{m}" for m in differing(javap(a, name), javap(b, name))]
    if bad:
        return "DIFF", f"{len(bad)}: " + "; ".join(bad[:3])[:220]
    return ("FAILED" if failed else "OK"), ""


def main(argv: list[str] | None = None) -> int:
    ap = argparse.ArgumentParser(prog="python3 -m scripts.javabench.roundtrip", description=__doc__.split("\n\n")[0])
    ap.add_argument("sources", nargs="*", type=Path, help="Java sources (default: kuna's corpus sources)")
    ap.add_argument("--decompiler", choices=["kuna", "vineflower"], default="kuna")
    ap.add_argument("--no-debug", action="store_true", help="compile with -parameters -g:none in place of -g")
    ap.add_argument("--flags", help="javac flags for both compilations, in place of the defaults")
    ap.add_argument("--keep", type=Path, help="keep classes, text and recompiled classes of each failing source here")
    ap.add_argument("--jobs", type=int, default=8)
    args = ap.parse_args(argv)
    flags = args.flags.split() if args.flags else (NO_DEBUG_FLAGS if args.no_debug else DEBUG_FLAGS)
    sources = args.sources or sorted(CORPUS_SOURCES.glob("*.java"))

    def one(source: Path) -> tuple[str, str, str]:
        with tempfile.TemporaryDirectory() as tmp:
            verdict, note = round_trip(source, flags, args.decompiler, Path(tmp))
            if args.keep and verdict in ("DIFF", "NOCOMPILE", "FAILED"):
                shutil.copytree(tmp, args.keep / source.stem, dirs_exist_ok=True)
        return source.stem, verdict, note

    with concurrent.futures.ThreadPoolExecutor(args.jobs) as pool:
        results = sorted(pool.map(one, sources))
    counts: dict[str, int] = {}
    for stem, verdict, note in results:
        counts[verdict] = counts.get(verdict, 0) + 1
        if verdict not in ("OK", "SKIP"):
            print(f"{verdict:10s} {stem}  {note}")
    print("summary", " ".join(f"{k}={v}" for k, v in sorted(counts.items())))
    return 0


if __name__ == "__main__":
    sys.exit(main())
