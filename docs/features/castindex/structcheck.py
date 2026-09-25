import sys, re, collections
sys.path.insert(0, "/home/mahaloz/kwt/castbench")
import castbench as CB, castcount as CC
from pathlib import Path
base, arm = Path(sys.argv[1]), Path(sys.argv[2])
keys = ["goto ", "label_", "return-dupe", "while", "for (", "do {", "break;", "continue;", "return", "if ("]
changed = 0; structural = []
for opt, proj, b in CB.corpus(sys.argv[3] if len(sys.argv) > 3 else "full"):
    try:
        bs = (base/opt/proj/f"{b}.c").read_text(errors="replace"); ks = (arm/opt/proj/f"{b}.c").read_text(errors="replace")
    except OSError: continue
    bf = {a: t for _, a, t in CC.split_functions(bs)}; kf = {a: t for _, a, t in CC.split_functions(ks)}
    if set(bf) != set(kf): structural.append((opt, b, "function set differs"))
    for a in bf:
        if a not in kf or bf[a] == kf[a]: continue
        changed += 1
        cb = [bf[a].count(k) for k in keys]; ck = [kf[a].count(k) for k in keys]
        lb = len(bf[a].splitlines()); lk = len(kf[a].splitlines())
        sb = bf[a].count(";"); sk = kf[a].count(";")
        if cb != ck or lb != lk or sb != sk:
            structural.append((opt, b, hex(a), dict((k, (x, y)) for k, x, y in zip(keys, cb, ck) if x != y), (lb, lk), (sb, sk)))
print("changed functions:", changed, " with a structural difference:", len(structural))
for s in structural[:40]: print(s)
