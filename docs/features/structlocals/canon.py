"""Offset-canonical comparison of two decompile-all arms for EVERY pointer variable.

    python3 canon2.py <dir> <armA> <armB> [--residue out.txt]

Each access through any pointer variable (param or local) is rewritten to
<var>@<byte offset> in both arms (P[k], *P, *(T *)&P[k], *(T *)((long)P + K),
P->field_0xK, P->field_0xK[j]); casts are dropped, v/a numbering collapsed and
declarations dropped.  A changed function equal after that is a pure respelling.
"""
import re, sys, difflib, pathlib, collections
ELEM = {"char":1,"unsigned char":1,"bool":1,"byte":1,"undefined":1,"undefined1":1,
        "short":2,"unsigned short":2,"undefined2":2,
        "int":4,"unsigned int":4,"float":4,"undefined4":4,
        "long":8,"unsigned long":8,"double":8,"void":1,"undefined8":8,"long long":8,"unsigned long long":8,
        "int1":1,"uint1":1,"int2":2,"uint2":2,"int4":4,"uint4":4,"int8":8,"uint8":8,"float4":4,"float8":8,"xunknown1":1,"xunknown2":2,"xunknown4":4,"xunknown8":8}
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
DECL = re.compile(r"^\s*((?:unsigned |signed |const |struct )*[A-Za-z_]\w*)\s*(\**)\s*([av]\d+)(\s*\[\w+\])?;")
def varmap(lines):
    res = {}
    sig = next((l for l in lines if "(" in l), "")
    m = re.search(r"\((.*)\)", sig)
    if m:
        for p in m.group(1).split(","):
            mm = re.match(r"\s*(.*?)\s*(\**)\s*(a\d+)\s*$", p)
            if mm: res[mm.group(3)] = (mm.group(1).strip(), len(mm.group(2)))
    for l in lines:
        mm = DECL.match(l)
        if mm and not mm.group(4) and mm.group(1) not in ("return", "goto", "case", "else") and mm.group(3) not in res:
            res[mm.group(3)] = (mm.group(1).strip(), len(mm.group(2)))
    return res
CAST = re.compile(r"\((?:unsigned |signed |const |struct )*[A-Za-z_]\w*(?: \w+)?\s*\**\)(?=[\w(*&!~-])")
def strip_casts(l):
    prev = None
    while prev != l:
        prev = l; l = CAST.sub("", l)
    return l
def num(s): return int(s, 16) if s.lower().startswith("0x") else int(s)
T = r"\((?:unsigned |signed |const |struct )*[A-Za-z_]\w*(?: \w+)?\s*\**\)"
def canon(l, vm):
    for P, (ty, stars) in vm.items():
        if stars < 1: continue
        if ty.startswith("struct_") and stars == 1:
            l = re.sub(rf"\*&{P}->field_(0x[0-9a-f]+)", lambda m: f"{P}@{num(m.group(1)):#x}", l)
            l = re.sub(rf"\*(?:{T})?&{P}->field_(0x[0-9a-f]+)\[(0x[0-9a-f]+|\d+)\]", lambda m: f"{P}@{num(m.group(1))+num(m.group(2)):#x}", l)
            l = re.sub(rf"&{P}->field_(0x[0-9a-f]+)\[(0x[0-9a-f]+|\d+)\]", lambda m: f"&{P}@{num(m.group(1))+num(m.group(2)):#x}", l)
            l = re.sub(rf"\b{P}->field_(0x[0-9a-f]+)\[(0x[0-9a-f]+|\d+)\]", lambda m: f"{P}@{num(m.group(1))+num(m.group(2)):#x}", l)
            l = re.sub(rf"&{P}->field_(0x[0-9a-f]+)", lambda m: f"&{P}@{num(m.group(1)):#x}", l)
            l = re.sub(rf"\b{P}->field_(0x[0-9a-f]+)", lambda m: f"{P}@{num(m.group(1)):#x}", l)
            continue
        s = (8 if stars > 1 else ELEM.get(ty))
        if s is None: continue
        l = re.sub(rf"\*(?:{T})?\(\((?:long|int|int8|uint8|int4)\){P} \+ (0x[0-9a-f]+|\d+)\)", lambda m: f"{P}@{num(m.group(1)):#x}", l)
        l = re.sub(rf"(?:{T})?\(\((?:long|int|int8|uint8|int4)\){P} \+ (0x[0-9a-f]+|\d+)\)", lambda m: f"&{P}@{num(m.group(1)):#x}", l)
        l = re.sub(rf"\((?:long|int|int8|uint8|int4)\){P} \+ (0x[0-9a-f]+|\d+)\b", lambda m: f"&{P}@{num(m.group(1)):#x}", l)
        l = re.sub(rf"\*(?:{T})?&{P}\[(0x[0-9a-f]+|\d+)\]", lambda m: f"{P}@{num(m.group(1))*s:#x}", l)
        l = re.sub(rf"&{P}\[(0x[0-9a-f]+|\d+)\]", lambda m: f"&{P}@{num(m.group(1))*s:#x}", l)
        l = re.sub(rf"\b{P}\[(0x[0-9a-f]+|\d+)\]", lambda m: f"{P}@{num(m.group(1))*s:#x}", l)
        l = re.sub(rf"\*(?:{T})?{P}\b(?!\[|@|->)", f"{P}@0x0", l)
    return strip_casts(l)
REN = re.compile(r"\b[av]\d+\b")
decl = re.compile(r"^[\w ]+[\s\*]+V( \[\w+\])?;( //.*)?$")
def canonical(lines):
    vm = varmap(lines)
    body = [l for l in lines[1:] if not (DECL.match(l) and DECL.match(l).group(1) not in ("return", "goto", "case", "else"))]
    out = [REN.sub("V", canon(l, vm)).strip() for l in body]
    out = [re.sub(r"\s*//.*$", "", l) for l in out]
    return [l for l in out if l]
if __name__ == "__main__":
    d, A, B = sys.argv[1:4]
    resout = sys.argv[5] if len(sys.argv) > 5 and sys.argv[4] == "--residue" else None
    tot = collections.Counter(); resid = []
    for f in sorted(pathlib.Path(d).glob(f"*.{A}.c")):
        t = f.name[:-len(f".{A}.c")]
        a, b = split(f), split(pathlib.Path(d) / f"{t}.{B}.c")
        for k in sorted(set(a) | set(b)):
            X, Y = a.get(k, []), b.get(k, [])
            if X == Y: continue
            if [re.sub(r"\bstruct_\d+\b", "S", l) for l in X] == [re.sub(r"\bstruct_\d+\b", "S", l) for l in Y]:
                tot["numbering only"] += 1; continue
            ca, cb = canonical(X), canonical(Y)
            if ca == cb:
                tot["respelling only"] += 1; continue
            sm = difflib.SequenceMatcher(None, ca, cb, autojunk=False)
            rl = [(ca[i1:i2], cb[j1:j2]) for tag, i1, i2, j1, j2 in sm.get_opcodes() if tag != "equal"]
            tot["residue"] += 1; resid.append((t, k, rl))
    print(dict(tot))
    if resout:
        with open(resout, "w") as fo:
            for t, k, rl in resid:
                fo.write(f"=== {t} {k}\n")
                for x, y in rl:
                    for l in x: fo.write(f"  - {l}\n")
                    for l in y: fo.write(f"  + {l}\n")
                    fo.write("  ..\n")
