#!/usr/bin/env python3
"""Declarations whose type changes class between two castbench arms.

    python3 demotions.py <off-arm> <on-arm>

Every parameter and local is matched by name within a function; a pointer that
becomes an integer (or the reverse) is listed, since a cast removed that way is
a type weakened, not a type made right.
"""
import collections, pathlib, re, sys
sys.path.insert(0, str(pathlib.Path(__file__).parent))
from hunks import split, parts

DECL = re.compile(r"^(?P<t>.*?)\s*\b(?P<n>[av]\d+)\s*(\[.*\])?;")


def decls(lines):
    out = {}
    if not lines:
        return out
    m = re.search(r"\((.*)\)", lines[1] if len(lines) > 1 else "")
    if m:
        for p in m.group(1).split(","):
            q = re.match(r"(.*?)\s*\b(a\d+)$", p.strip())
            if q:
                out[q.group(2)] = q.group(1).replace(" ", "")
    _, ds, _ = parts(lines)
    for d in ds:
        q = DECL.match(d.split("//")[0].strip())
        if q:
            where = re.search(r"//\s*(stack [-+] 0x[0-9a-f]+)", d)
            out[where.group(1) if where else q.group("n")] = q.group("t").replace(" ", "")
    return out


def kind(t):
    return "ptr" if t.endswith("*") else "int"


def main():
    off, on = pathlib.Path(sys.argv[1]), pathlib.Path(sys.argv[2])
    c, ex = collections.Counter(), collections.defaultdict(list)
    for f in sorted(off.rglob("*.c")):
        rel = f.relative_to(off)
        a, b = split(f), split(on / rel)
        for k in sorted(set(a) & set(b)):
            if a[k] == b[k]:
                continue
            da, db = decls(a[k]), decls(b[k])
            same_count = len(parts(a[k])[1]) == len(parts(b[k])[1])
            for n in set(da) & set(db):
                if n.startswith("v") and not same_count:
                    continue
                if kind(da[n]) != kind(db[n]):
                    key = f"{kind(da[n])} -> {kind(db[n])}"
                    c[key] += 1
                    ex[key].append(f"{rel} {k} {n}: {da[n]} -> {db[n]}")
    for key, n in c.most_common():
        print(f"{n:5}  {key}")
        for e in ex[key][:40]:
            print("        " + e)


if __name__ == "__main__":
    main()
