import os, re, sys, pathlib
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from canon import split
NOTCALL = {"if", "while", "for", "switch", "sizeof", "return", "do", "case"}
def skeleton(lines):
    toks = []
    for line in lines:
        code = line.split("//")[0]
        for m in re.finditer(r"\b(if|else|while|do|for|goto|return|switch|case|default|break|continue)\b|\b([A-Za-z_]\w*)\s*\(", code):
            if m.group(1): toks.append(m.group(1))
            elif m.group(2) not in NOTCALL: toks.append(m.group(2) + "()")
    return toks
d, A, B = sys.argv[1:4]; tot = ch = mv = 0
for f in sorted(pathlib.Path(d).glob(f"*.{A}.c")):
    t = f.name[:-len(f".{A}.c")]
    a, b = split(f), split(pathlib.Path(d) / f"{t}.{B}.c")
    for k in set(a) | set(b):
        tot += 1
        if a.get(k) == b.get(k): continue
        ch += 1
        if skeleton(a.get(k, [])) != skeleton(b.get(k, [])):
            mv += 1; print("SKELETON DELTA", t, k)
print(f"functions={tot} changed={ch} skeleton_deltas={mv}")
