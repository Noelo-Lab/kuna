#!/usr/bin/env python3
"""Classify every function the element-pair rule changes in a `decompile-all` pair.

    python3 docs/features/structsynth/arrays_hunks.py <dir>/<prefix>...

reading `<prefix>.before.c` (structsynth param without the rule) and
`<prefix>.after.c` (with it). Buckets, first match wins:

* **ledger renumbering only** -- identical once every `struct_N` is spelled
  the same way: a pair that no longer mints a structure shifts the program-wide
  `struct_N` numbering of every later one;
* **skeleton delta** -- a control-flow keyword or a call token moved;
* **variable-count delta** -- the declaration block changed length;
* **element pointer restored** -- every differing line is a declaration or
  signature where a `struct_N *` became an integer pointer, or a body line where
  a `->field_0x0`/`->field_0x8` read became `*aN`/`aN[1]`;
* **other** -- read by hand; printed in full.

Unlike `hunks.py`, the whole function is compared, signature included.
"""
import collections
import difflib
import pathlib
import re
import sys

KEYWORDS = "if|else|while|do|for|goto|return|switch|case|default|break|continue"
NOTCALL = {"if", "while", "for", "switch", "sizeof", "return", "do", "case"}
STRUCT = re.compile(r"\bstruct_\d+\b")
FIELD_OF = re.compile(r"\b(a\d+)->field_0x([0-9a-f]+)")
PTR = re.compile(r"\b(?:struct_N|unsigned long|long|int8|uint8|undefined8) \*")
# Every declined layout is a pair of pointer-sized fields.
ELEMENT, PAIR = 8, 16
INDEX_OF = re.compile(r"&(a\d+)\[(\d+)\]")


def split(path):
    out, cur, buf = {}, None, []
    for line in pathlib.Path(path).read_text().splitlines():
        m = re.match(r"// Function: (\S+) @ (0x[0-9a-f]+)", line)
        if m:
            if cur:
                out[cur] = buf
            cur, buf = m.group(2), [line]
        else:
            buf.append(line)
    if cur:
        out[cur] = buf
    return out


def decl_count(lines):
    try:
        i = next(i for i, l in enumerate(lines) if l.rstrip() == "{") + 1
    except StopIteration:
        return 0
    n = 0
    while i < len(lines) and lines[i].strip():
        n, i = n + 1, i + 1
    return n


def skeleton(lines):
    toks = []
    for line in lines:
        for m in re.finditer(rf"\b({KEYWORDS})\b|\b([A-Za-z_]\w*)\s*\(", line.split("//")[0]):
            if m.group(1):
                toks.append(m.group(1))
            elif m.group(2) not in NOTCALL:
                toks.append(m.group(2) + "()")
    return toks


def restored_params(la, lb) -> set:
    """The parameters whose `struct_N *` became an integer pointer."""
    def params(lines):
        sig = next((l for l in lines if re.match(r"\S.*\(.*\)", l) and not l.startswith("//")), "")
        return dict((n, t) for t, n in re.findall(r"([\w ]+?) \*(a\d+)\b", sig))
    pa, pb = params(la), params(lb)
    return {n for n, t in pa.items() if t.strip().startswith("struct_")
            and n in pb and not pb[n].strip().startswith("struct_")}


def restored(before: str, after: str, names: set) -> bool:
    """Is this line pair exactly a structure turning back into its element
    pointer: `struct_N *aK` -> an integer `*aK` in a signature or declaration,
    `aK->field_0x0` -> `*aK`, `aK->field_0x8` -> `aK[1]` and `&aK[n]` over the
    16-byte structure -> `&aK[2n]` over the 8-byte element in a body, for a
    parameter `aK` the signature restored?"""
    def element(m):
        if m.group(1) not in names:
            return m.group(0)
        off = int(m.group(2), 16)
        return f"*{m.group(1)}" if off == 0 else f"{m.group(1)}[{off // ELEMENT}]"

    def index(m):
        if m.group(1) not in names:
            return m.group(0)
        return f"&{m.group(1)}[{int(m.group(2)) * PAIR // ELEMENT}]"
    b = PTR.sub("T *", INDEX_OF.sub(index, FIELD_OF.sub(element, before)))
    return b == PTR.sub("T *", after) and b != before


cls = collections.Counter()
other = []
per_binary = []
for prefix in sys.argv[1:]:
    a, b = split(f"{prefix}.before.c"), split(f"{prefix}.after.c")
    name = pathlib.Path(prefix).name
    changed = collections.Counter()
    for k in sorted(set(a) | set(b)):
        la, lb = a.get(k, []), b.get(k, [])
        if la == lb:
            continue
        if [STRUCT.sub("struct_N", l) for l in la] == [STRUCT.sub("struct_N", l) for l in lb]:
            bucket = "ledger renumbering only"
        elif skeleton(la) != skeleton(lb):
            bucket = "skeleton delta"
        elif decl_count(la) != decl_count(lb):
            bucket = "variable-count delta"
        else:
            na = [STRUCT.sub("struct_N", l).strip() for l in la]
            nb = [STRUCT.sub("struct_N", l).strip() for l in lb]
            pairs = []
            for tag, i1, i2, j1, j2 in difflib.SequenceMatcher(None, na, nb, autojunk=False).get_opcodes():
                if tag != "equal":
                    rem, add = na[i1:i2], nb[j1:j2]
                    pairs += [(rem[t] if t < len(rem) else "", add[t] if t < len(add) else "")
                              for t in range(max(len(rem), len(add)))]
            names = restored_params(la, lb)
            bad = [p for p in pairs if not restored(*p, names)]
            bucket = "element pointer restored" if not bad else "other"
            if bad:
                other.append((name, k, bad))
        cls[bucket] += 1
        changed[bucket] += 1
    per_binary.append((name, len(set(a) | set(b)), changed))

for name, total, changed in per_binary:
    print(f"{name:14s} {total:5d} functions  {sum(changed.values()):4d} changed  "
          + ", ".join(f"{k}={v}" for k, v in sorted(changed.items())))
print("\nchanged functions by class:")
for k, v in cls.most_common():
    print(f"  {v:5d}  {k}")
print(f"\nother ({len(other)}):")
for name, k, bad in other:
    print("  ", name, k)
    for r, d in bad[:8]:
        print("       -", r)
        print("       +", d)
