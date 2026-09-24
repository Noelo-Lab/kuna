#!/usr/bin/env python3
"""Structural-hunk sweep for castarith: per function, compare the control-flow
shape of two arms of one build (`--option castarith off` vs default).

The signature counts gotos, labels, `// return-dupe` tails, `return`
statements and the control keywords. castarith only rewrites expressions, so a
function whose signature differs changed a P8 decision; the known cause is a
post-cast duplication budget (taildup, gotoreduce, crossjumprevert) counting
the CAST ops the rewrite removed.

  structural.py OFF_DIR ON_DIR     # castbench-style trees of <bin>.c files
"""
import re, sys
from collections import Counter
from pathlib import Path

HDR = re.compile(r"^// Function: (\S+) @ (0x[0-9a-f]+)$", re.M)
KEYS = {
    "goto": re.compile(r"\bgoto\s+\w+\s*;"),
    "label": re.compile(r"^\s*\w+:\s*$", re.M),
    "return-dupe": re.compile(r"//.*\breturn-dupe\b"),
    "return": re.compile(r"\breturn\b[^;]*;"),
    "if": re.compile(r"\bif\s*\("),
    "else": re.compile(r"\belse\b"),
    "while": re.compile(r"\bwhile\s*\("),
    "for": re.compile(r"\bfor\s*\("),
    "switch": re.compile(r"\bswitch\s*\("),
    "break": re.compile(r"\bbreak\s*;"),
    "continue": re.compile(r"\bcontinue\s*;"),
}


def funcs(path):
    src = path.read_text(errors="replace")
    heads = list(HDR.finditer(src))
    out = {}
    for i, m in enumerate(heads):
        end = heads[i + 1].start() if i + 1 < len(heads) else len(src)
        out[m.group(2)] = (m.group(1), src[m.end():end])
    return out


def sig(text):
    return tuple(len(r.findall(text)) for r in KEYS.values())


def main():
    off, on = Path(sys.argv[1]), Path(sys.argv[2])
    nfun = nbin = 0
    moved = []
    for f in sorted(off.rglob("*.c")):
        g = on / f.relative_to(off)
        if not g.exists():
            continue
        nbin += 1
        a, b = funcs(f), funcs(g)
        for addr in sorted(set(a) & set(b)):
            nfun += 1
            sa, sb = sig(a[addr][1]), sig(b[addr][1])
            if sa != sb:
                d = {k: y - x for k, x, y in zip(KEYS, sa, sb) if x != y}
                moved.append((str(f.relative_to(off))[:-2], addr, a[addr][0], d))
    print(f"{nbin} binaries, {nfun} functions, {len(moved)} with a different control-flow signature")
    tot = Counter()
    for rel, addr, name, d in moved:
        tot.update(d)
        print(f"  {rel} {name} @{addr}: " + ", ".join(f"{k} {v:+d}" for k, v in d.items()))
    if moved:
        print("  net: " + ", ".join(f"{k} {v:+d}" for k, v in sorted(tot.items())))


if __name__ == "__main__":
    main()
