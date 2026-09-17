#!/usr/bin/env python3
"""Classify every function `structsynth` changes in a `decompile-all` pair.

    python3 docs/features/structsynth/hunks.py <sweep-dir>/<prefix>...

reading `<prefix>.off.c` and `<prefix>.on.c`. Four buckets:

* **field accesses only** -- every differing body line differs only where a
  field reference, a struct spelling or a cast appears;
* **variable renumbering only** -- the two bodies are identical once every
  `vN`/`aN` is spelled the same way;
* **variable-count delta** -- the declaration block declares a different number
  of variables (a type lock changes what `merge_test_adjacent` compares);
* **skeleton delta** -- a control-flow keyword or a call token moved, which a
  type change has no business doing;
* **other** -- read it by hand.

The declaration block is the run of lines between the opening brace and the
first blank line, which is exactly how the emitter writes it; matching
declarations by shape instead catches ordinary assignments whose right-hand
side is a bare variable.
"""
import re, sys, pathlib, collections, difflib

KEYWORDS = ("if|else|while|do|for|goto|return|switch|case|default|break|continue")
NOTCALL = {"if", "while", "for", "switch", "sizeof", "return", "do", "case"}
CAST = re.compile(r"\((?:unsigned |signed |const |struct )*[A-Za-z_]\w*\s*\**\)")
RENUM = re.compile(r"\b[av]\d+\b")


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


def parts(lines):
    """(declaration lines, body lines)."""
    try:
        start = next(i for i, l in enumerate(lines) if l.rstrip() == "{") + 1
    except StopIteration:
        return [], lines
    decls = []
    i = start
    while i < len(lines) and lines[i].strip():
        decls.append(lines[i].strip())
        i += 1
    return decls, lines[i:]


def skeleton(lines):
    toks = []
    for line in lines:
        for m in re.finditer(rf"\b({KEYWORDS})\b|\b([A-Za-z_]\w*)\s*\(", line.split("//")[0]):
            if m.group(1):
                toks.append(m.group(1))
            elif m.group(2) not in NOTCALL:
                toks.append(m.group(2) + "()")
    return toks


cls = collections.Counter()
detail = collections.defaultdict(list)
casts = 0
per_binary = []
for prefix in sys.argv[1:]:
    a, b = split(f"{prefix}.off.c"), split(f"{prefix}.on.c")
    name = pathlib.Path(prefix).name
    n_changed = 0
    for k in sorted(set(a) | set(b)):
        if a.get(k) == b.get(k):
            continue
        n_changed += 1
        da, ba = parts(a.get(k, []))
        db, bb = parts(b.get(k, []))
        if skeleton(ba) != skeleton(bb):
            cls["skeleton delta"] += 1
            detail["skeleton"].append((name, k))
            continue
        if len(da) != len(db):
            cls["variable-count delta"] += 1
            detail["vardelta"].append((name, k, len(db) - len(da)))
            continue
        # Variable numbering is not a change; compare the bodies with every
        # `vN`/`aN` spelled the same way.
        ba = [RENUM.sub("V", l) for l in ba]
        bb = [RENUM.sub("V", l) for l in bb]
        if ba == bb:
            cls["variable renumbering only"] += 1
            continue
        fields_only, other = True, []
        sm = difflib.SequenceMatcher(None, ba, bb, autojunk=False)
        for tag, i1, i2, j1, j2 in sm.get_opcodes():
            if tag == "equal":
                continue
            rem, add = ba[i1:i2], bb[j1:j2]
            for t in range(max(len(rem), len(add))):
                r = (rem[t] if t < len(rem) else "").strip()
                d = (add[t] if t < len(add) else "").strip()
                if r == d:
                    continue
                if "->field_0x" in d or "struct_" in d:
                    if len(CAST.findall(d)) > len(CAST.findall(r)):
                        casts += 1
                    continue
                fields_only = False
                other.append((r, d))
        if fields_only:
            cls["field accesses only"] += 1
        else:
            cls["other"] += 1
            detail["other"].append((name, k, other[:6]))
    per_binary.append((name, len(set(a) | set(b)), n_changed))

for name, total, changed in per_binary:
    print(f"{name:38s} {total:5d} functions  {changed:4d} changed")
print("\nchanged functions by class:")
for k, v in cls.most_common():
    print(f"  {v:5d}  {k}")
print(f"  {casts:5d}  lines where the field access gained a cast (width disagreement)")
print("\nvariable-count deltas:", collections.Counter(d[2] for d in detail["vardelta"]))
for d in detail["vardelta"]:
    print("   ", d)
print(f"\nskeleton deltas ({len(detail['skeleton'])}):")
for d in detail["skeleton"]:
    print("   ", d)
print(f"\nother ({len(detail['other'])}):")
for d in detail["other"]:
    print("   ", d[0], d[1])
    for x in d[2]:
        print("        ", x)
