#!/usr/bin/env python3
"""Count functions with a `goto L;` whose `L:` is not emitted, per arm.

    python3 docs/features/structsynth/gotolabel.py <sweep-dir>/<prefix>...

reading `<prefix>.off.c` and `<prefix>.on.c`.
"""
import re, sys, pathlib
def split(path):
    out, cur, buf = {}, None, []
    for line in pathlib.Path(path).read_text(errors="replace").splitlines():
        m = re.match(r"// Function: (\S+) @ (0x[0-9a-f]+)", line)
        if m:
            if cur: out[cur] = buf
            cur, buf = m.group(2), []
        else:
            buf.append(line)
    if cur: out[cur] = buf
    return out
GOTO = re.compile(r"\bgoto (\w+);")
tot = {}
for prefix in sys.argv[1:]:
    name = pathlib.Path(prefix).name
    for arm in ("off", "on"):
        fs = split(f"{prefix}.{arm}.c")
        bad = []
        for k, lines in fs.items():
            text = "\n".join(lines)
            labels = set(re.findall(r"^\s*(\w+):", text, re.M))
            miss = sorted({g for g in GOTO.findall(text) if g not in labels})
            if miss: bad.append((k, miss))
        tot[arm] = tot.get(arm, 0) + len(bad)
        print(f"{name:12s} {arm:3s} {len(bad):3d} {bad}")
print("TOTAL", tot)
