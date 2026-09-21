import re, sys, difflib, pathlib
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
S = re.compile(r"\bstruct_\d+\b"); V = re.compile(r"\b[av]\d+\b")
for prefix in sys.argv[1:]:
    a, b = split(f"{prefix}.param.c"), split(f"{prefix}.nest.c")
    ren = real = 0; reals = []
    for k in sorted(set(a) | set(b)):
        if a.get(k) == b.get(k): continue
        na = [S.sub("struct_N", l) for l in a.get(k, [])]; nb = [S.sub("struct_N", l) for l in b.get(k, [])]
        if na == nb: ren += 1; continue
        real += 1; reals.append(k)
    print(f"{prefix:30s} functions={len(set(a)|set(b))} struct-renumbering-only={ren} real={real} {reals}")
