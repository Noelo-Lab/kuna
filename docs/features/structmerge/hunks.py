#!/usr/bin/env python3
"""Classify every function `structmerge` changes in a `decompile-all` pair.

Four buckets, in order: a skeleton delta (a control-flow keyword or call token
moved), a declaration-count delta, `record name only` (the two functions are
identical once every `struct_N` number and every `vN`/`aN` is spelled the same
way), `field accesses only` (every differing body line differs only where a
field reference, a struct spelling or a cast appears), and `read` for the rest.
"""
import re, sys, pathlib, collections, difflib
sys.path.insert(0, "/home/mahaloz/kwt/layoutrecall/docs/features/structsynth")
KEYWORDS = "if|else|while|do|for|goto|return|switch|case|default|break|continue"
NOTCALL = {"if", "while", "for", "switch", "sizeof", "return", "do", "case"}
CAST = re.compile(r"\((?:unsigned |signed |const |struct )*[A-Za-z_]\w*\s*\**\)")
RENUM = re.compile(r"\b[av]\d+\b")
SNUM = re.compile(r"\bstruct_\d+\b")

def split(path):
    out, cur, buf = {}, None, []
    for line in pathlib.Path(path).read_text().splitlines():
        m = re.match(r"// Function: (\S+) @ (0x[0-9a-f]+)", line)
        if m:
            if cur: out[cur] = buf
            cur, buf = m.group(2), [line]
        else: buf.append(line)
    if cur: out[cur] = buf
    return out

def parts(lines):
    try: start = next(i for i, l in enumerate(lines) if l.rstrip() == "{") + 1
    except StopIteration: return lines[:1], [], lines
    head = lines[:start]
    decls, i = [], start
    while i < len(lines) and lines[i].strip():
        decls.append(lines[i].strip()); i += 1
    return head, decls, lines[i:]

def skeleton(lines):
    toks = []
    for line in lines:
        for m in re.finditer(rf"\b({KEYWORDS})\b|\b([A-Za-z_]\w*)\s*\(", line.split("//")[0]):
            if m.group(1): toks.append(m.group(1))
            elif m.group(2) not in NOTCALL: toks.append(m.group(2) + "()")
    return toks

def norm(ls): return [SNUM.sub("S", RENUM.sub("V", l)) for l in ls]

cls = collections.Counter(); rows = []; other = []
for prefix in sys.argv[1:]:
    a, b = split(f"{prefix}.off.c"), split(f"{prefix}.on.c")
    name = pathlib.Path(prefix).name
    per = collections.Counter(); changed = 0
    for k in sorted(set(a) | set(b)):
        if a.get(k) == b.get(k): continue
        changed += 1
        ha, da, ba = parts(a.get(k, [])); hb, db, bb = parts(b.get(k, []))
        if skeleton(ba) != skeleton(bb): per["skeleton delta"] += 1; continue
        if len(da) != len(db): per["declaration-count delta"] += 1; continue
        if norm(ha + da + ba) == norm(hb + db + bb):
            per["record name only"] += 1; continue
        ok = True; diffs = []
        for tag, i1, i2, j1, j2 in difflib.SequenceMatcher(
                None, norm(ba), norm(bb), autojunk=False).get_opcodes():
            if tag == "equal": continue
            rem, add = norm(ba)[i1:i2], norm(bb)[j1:j2]
            for t in range(max(len(rem), len(add))):
                r = (rem[t] if t < len(rem) else "").strip()
                d = (add[t] if t < len(add) else "").strip()
                if r == d: continue
                if "->field_0x" in d or "->field_0x" in r or "S" in d:
                    continue
                ok = False; diffs.append((r, d))
        if ok: per["field accesses only"] += 1
        else:
            per["read"] += 1; other.append((name, k, diffs[:4]))
    rows.append((name, len(set(a) | set(b)), changed, per))
    cls.update(per)
w = max(len(r[0]) for r in rows)
print(f"| {'binary':{w}s} | functions | changed | record name only | field accesses only | read |")
print(f"|{'-'*(w+2)}|---:|---:|---:|---:|---:|")
tot = [0, 0, 0, 0, 0]
for name, n, ch, per in rows:
    print(f"| {name:{w}s} | {n} | {ch} | {per['record name only']} | "
          f"{per['field accesses only']} | {per['read'] + per['skeleton delta'] + per['declaration-count delta']} |")
    tot = [tot[0]+n, tot[1]+ch, tot[2]+per['record name only'], tot[3]+per['field accesses only'],
           tot[4]+per['read']+per['skeleton delta']+per['declaration-count delta']]
print(f"| **total** | {tot[0]} | {tot[1]} | {tot[2]} | {tot[3]} | {tot[4]} |")
print("\nall classes:", dict(cls))
for o in other:
    print("READ", o[0], o[1]); [print("    ", x) for x in o[2]]
