#!/usr/bin/env python3
"""Offset-canonical comparison of structsynth off/on function pairs.

    python3 docs/features/structsynth/canon.py <tag>...

reading `sweep/<tag>.off.c` and `sweep/<tag>.on.c` (two `decompile-all` arms) from
the current directory and writing the unverified line pairs to
`canon_residue.txt`.

Every access through a retyped parameter is rewritten to @<byte offset> in both
arms (off: P[k], *P, *(T *)&P[k], *((long)P + K); on: P->field_0xK, P->field_0xK[j]),
casts are dropped and v/a numbering is collapsed. A changed line pair that is equal
after that is a pure field-access respelling; the rest is printed for reading.
"""
import re, sys, difflib, pathlib, collections
ELEM = {"char":1,"uchar":1,"unsigned char":1,"bool":1,"int1":1,"uint1":1,"byte":1,
        "short":2,"unsigned short":2,"int2":2,"uint2":2,
        "int":4,"unsigned int":4,"int4":4,"uint4":4,"float":4,
        "long":8,"unsigned long":8,"int8":8,"uint8":8,"double":8,"void *":8,"long long":8,"unsigned long long":8}
def split(path):
    out, cur, buf = {}, None, []
    for line in pathlib.Path(path).read_text(errors="replace").splitlines():
        m = re.match(r"// Function: (\S+) @ (0x[0-9a-f]+)", line)
        if m:
            if cur: out[cur] = buf
            cur, buf = m.group(2), []
        else: buf.append(line)
    if cur: out[cur] = buf
    return out
def params(sig):
    m = re.search(r"\((.*)\)", sig)
    if not m: return {}
    res = {}
    for p in m.group(1).split(","):
        p = p.strip()
        mm = re.match(r"(.*?)\s*(\**)\s*(a\d+)$", p)
        if mm: res[mm.group(3)] = (mm.group(1).strip(), mm.group(2))
    return res
CAST = re.compile(r"\((?:unsigned |signed |const |struct )*[A-Za-z_]\w*(?: \w+)?\s*\**\)(?=[\w(*&!~-])")
def strip_casts(l):
    prev = None
    while prev != l:
        prev = l; l = CAST.sub("", l)
    return l
def num(s): return int(s, 16) if s.lower().startswith("0x") else int(s)
def canon_off(l, P, s):
    l = strip_casts(l)
    l = re.sub(rf"\*&{P}\[(0x[0-9a-f]+|\d+)\]", lambda m: f"@{num(m.group(1))*s:#x}", l)
    l = re.sub(rf"&{P}\[(0x[0-9a-f]+|\d+)\]", lambda m: f"&@{num(m.group(1))*s:#x}", l)
    l = re.sub(rf"\*\({P} \+ (0x[0-9a-f]+|\d+)\)", lambda m: f"@{num(m.group(1)):#x}", l)
    l = re.sub(rf"\b{P}\[(0x[0-9a-f]+|\d+)\]", lambda m: f"@{num(m.group(1))*s:#x}", l)
    l = re.sub(rf"\*{P}\b(?!\[)", "@0x0", l)
    return l
def canon_on(l, P):
    l = strip_casts(l)
    l = re.sub(rf"\*&{P}->field_(0x[0-9a-f]+)", lambda m: f"@{num(m.group(1)):#x}", l)
    l = re.sub(rf"&{P}->field_(0x[0-9a-f]+)\[(0x[0-9a-f]+|\d+)\]", lambda m: f"&@{num(m.group(1))+num(m.group(2)):#x}", l)
    l = re.sub(rf"\b{P}->field_(0x[0-9a-f]+)\[(0x[0-9a-f]+|\d+)\]", lambda m: f"@{num(m.group(1))+num(m.group(2)):#x}", l)
    l = re.sub(rf"&{P}->field_(0x[0-9a-f]+)", lambda m: f"&@{num(m.group(1)):#x}", l)
    l = re.sub(rf"\b{P}->field_(0x[0-9a-f]+)", lambda m: f"@{num(m.group(1)):#x}", l)
    return l
REN = re.compile(r"\b[av]\d+\b")
tot = collections.Counter(); resid = []
tags = sys.argv[1:]
for t in tags:
    a, b = split(f"sweep/{t}.off.c"), split(f"sweep/{t}.on.c")
    arch32 = t.startswith(("chibios", "freertos"))
    for k in sorted(set(a) | set(b)):
        A, B = a.get(k, []), b.get(k, [])
        if A == B: continue
        pa, pb = params(A[0] if A else ""), params(B[0] if B else "")
        ret = [n for n in pb if pb[n][0].startswith("struct_") and not pa.get(n, ("",))[0].startswith("struct_")]
        ca, cb = A[1:], B[1:]
        for P in ret:
            ty, stars = pa.get(P, ("", "*"))
            if stars != "*" : s = 4 if arch32 else 8
            else: s = ELEM.get(ty, None)
            if ty in ("long","unsigned long") and arch32: s = 4
            if s is None: s = 1 if ty == "void" else 8
            ca = [canon_off(l, P, s) for l in ca]
            cb = [canon_on(l, P) for l in cb]
        ca = [REN.sub("V", strip_casts(l)).strip() for l in ca]
        cb = [REN.sub("V", strip_casts(l)).strip() for l in cb]
        # drop declaration lines (type spellings move with retyping)
        decl = re.compile(r"^[\w ]+[\s\*]+V( \[\w+\])?;( //.*)?$")
        ca = [l for l in ca if not decl.match(l)]; cb = [l for l in cb if not decl.match(l)]
        if ca == cb:
            tot["verified: field respelling only"] += 1; continue
        # line-level residue
        sm = difflib.SequenceMatcher(None, ca, cb, autojunk=False)
        rl = []
        for tag, i1, i2, j1, j2 in sm.get_opcodes():
            if tag != "equal": rl.append((ca[i1:i2], cb[j1:j2]))
        tot["residue"] += 1
        resid.append((t, k, rl))
print(dict(tot))
with open("canon_residue.txt", "w") as f:
    for t, k, rl in resid:
        f.write(f"=== {t} {k}\n")
        for x, y in rl:
            for l in x: f.write(f"  - {l}\n")
            for l in y: f.write(f"  + {l}\n")
            f.write("  ..\n")
