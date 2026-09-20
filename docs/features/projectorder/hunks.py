"""Classify every function `decompile-project` renders differently after the fix."""
import collections
import difflib
import re
import sys
from pathlib import Path

ROOT = Path("/home/mahaloz/kwt/layoutvote/.scratch/export")
BUILDS = [(o, b) for o in ("O0", "O2") for b in ("fmt", "ls", "sort", "du")]
FN = re.compile(r"^[A-Za-z_][\w \*]*\**(?P<name>\w+)\([^;]*\)\s*(//.*)?$")


def functions(path: Path) -> dict:
    """`.c` text split into per-function bodies, keyed by the `sub_<addr>` name."""
    out, cur, buf = {}, None, []
    for line in path.read_text().split("\n"):
        m = re.match(r"^(?:[\w ]+[ \*] *)?(\w+)\(.*\)(?: //.*)?$", line)
        if m and not line.startswith(" ") and not line.endswith(";"):
            if cur:
                out[cur] = "\n".join(buf)
            cur, buf = m.group(1), [line]
            continue
        if cur:
            buf.append(line)
    if cur:
        out[cur] = "\n".join(buf)
    return out


STRUCT = re.compile(r"\bstruct_\d+\b")


def classify(a: str, b: str) -> str:
    if STRUCT.sub("struct_N", a) == STRUCT.sub("struct_N", b):
        return "struct_N renumbered only"
    an, bn = STRUCT.sub("struct_N", a).split("\n"), STRUCT.sub("struct_N", b).split("\n")
    moved = [l for l in difflib.unified_diff(an, bn, n=0) if l[:1] in "+-" and l[:3] not in ("---", "+++")]
    kinds = set()
    for l in moved:
        t = l[1:].strip()
        if re.match(r"^(struct_N|\w[\w ]*) \**\w+;$", t) or "field_0x" in t:
            kinds.add("declaration/field spelling")
        elif t.startswith("//") or not t:
            kinds.add("comment/blank")
        else:
            kinds.add("statement text")
    if kinds <= {"declaration/field spelling", "comment/blank"}:
        return "declarations and field names only"
    return "statement text changed"


def main():
    totals = collections.Counter()
    per_build = []
    examples = collections.defaultdict(list)
    for opt, b in BUILDS:
        a = functions(ROOT / f"base-{opt}-{b}" / f"{b}.c")
        c = functions(ROOT / f"fix-{opt}-{b}" / f"{b}.c")
        common = set(a) & set(c)
        changed = [k for k in common if a[k] != c[k]]
        row = collections.Counter(classify(a[k], c[k]) for k in changed)
        for k in changed:
            examples[classify(a[k], c[k])].append(f"{opt}/{b}:{k}")
        per_build.append((opt, b, len(common), len(set(a) ^ set(c)), len(changed), row))
        totals.update(row)
        totals["functions"] += len(common)
        totals["changed"] += len(changed)
        totals["only in one arm"] += len(set(a) ^ set(c))
    print("| build | functions | only in one arm | changed | " +
          " | ".join(sorted(totals for totals in {k for _, _, _, _, _, r in per_build for k in r})) + " |")
    for opt, b, n, only, ch, row in per_build:
        cells = " | ".join(str(row.get(k, 0)) for k in sorted({k for _, _, _, _, _, r in per_build for k in r}))
        print(f"| {opt} {b} | {n} | {only} | {ch} | {cells} |")
    print()
    for k, v in totals.items():
        print(f"{k}: {v}")
    for k, v in examples.items():
        if k != "struct_N renumbered only":
            print(f"\n{k} ({len(v)}): {v[:40]}")


main()
