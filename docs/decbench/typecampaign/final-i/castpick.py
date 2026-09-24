"""List short real functions whose casts fell G -> final, with their DWARF names (from the unstripped twin)."""
import subprocess, sys
from pathlib import Path
sys.path.insert(0, "/home/mahaloz/kwt/castbench")
import castbench as CB, castcount as CC
R = CB.RES
def names(opt, proj, b):
    out = subprocess.run(["nm", str(R / opt / proj / "compiled" / b)], capture_output=True, text=True).stdout
    return {int(l.split()[0], 16): l.split()[2] for l in out.splitlines() if len(l.split()) == 3 and l.split()[1] in "tT"}
def texts(path):
    return {a: t for _, a, t in CC.split_functions(Path(path).read_text(errors="replace"))}
maxlines = int(sys.argv[1]) if len(sys.argv) > 1 else 20
rows = []
for opt, proj, b in CB.corpus("full"):
    g = CB.funcs(f"cb-g/{opt}/{proj}/{b}.c"); i = CB.funcs(f"cb-i/{opt}/{proj}/{b}.c")
    ida = CB.funcs(R / opt / proj / "decompiled" / f"ida_{b}.c")
    if not (g and i and ida): continue
    nm = None
    for a in set(g) & set(i) & set(ida):
        if i[a][2] < g[a][2] and i[a][0] <= maxlines:
            nm = nm or names(opt, proj, b)
            rows.append((g[a][2] - i[a][2], g[a][2], i[a][2], ida[a][2], i[a][0], opt, b, hex(a), nm.get(a, "?")))
rows.sort(reverse=True)
for r in rows[:60]:
    print(*r)
