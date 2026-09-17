#!/usr/bin/env python3
"""Does structsynth move a statement anywhere in the corpus?

    python3 docs/features/structsynth/skeleton.py <sweep-dir>/<prefix>...

For each function in a `decompile-all` off/on pair, reduce the body to its
skeleton -- the sequence of control-flow keywords and call tokens -- and report
every function whose skeleton differs. A type change may not move a statement;
if it does, the hunk is not "a field access" however it is spelled.
"""
import re, sys, pathlib, collections

KEYWORD = re.compile(r"\b(if|else|while|do|for|goto|return|switch|case|default|"
                     r"break|continue)\b")
CALL = re.compile(r"\b([A-Za-z_]\w*)\s*\(")
NOTCALL = {"if", "while", "for", "switch", "sizeof", "return", "do", "case"}


def split(path):
    out, cur, buf = {}, None, []
    for line in pathlib.Path(path).read_text().splitlines():
        m = re.match(r"// Function: (\S+) @ (0x[0-9a-f]+)", line)
        if m:
            if cur:
                out[cur] = buf
            cur, buf = m.group(2), []
        else:
            buf.append(line)
    if cur:
        out[cur] = buf
    return out


def skeleton(lines):
    toks = []
    for line in lines:
        code = line.split("//")[0]
        for m in re.finditer(r"\b(if|else|while|do|for|goto|return|switch|case|default|"
                             r"break|continue)\b|\b([A-Za-z_]\w*)\s*\(", code):
            if m.group(1):
                toks.append(m.group(1))
            elif m.group(2) not in NOTCALL:
                toks.append(m.group(2) + "()")
    return toks


total = changed = moved = 0
for prefix in sys.argv[1:]:
    a, b = split(f"{prefix}.off.c"), split(f"{prefix}.on.c")
    n_changed = n_moved = 0
    for k in sorted(set(a) | set(b)):
        if a.get(k) == b.get(k):
            continue
        n_changed += 1
        if skeleton(a.get(k, [])) != skeleton(b.get(k, [])):
            n_moved += 1
            print(f"  SKELETON DELTA {pathlib.Path(prefix).name} {k}")
    total += len(set(a) | set(b))
    changed += n_changed
    moved += n_moved
    print(f"{pathlib.Path(prefix).name}: {len(a)} functions, {n_changed} changed, "
          f"{n_moved} with a skeleton delta")
print(f"TOTAL: {total} functions, {changed} changed, {moved} with a skeleton delta")
