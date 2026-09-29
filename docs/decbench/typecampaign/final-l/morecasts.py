"""Every function with MORE casts between two castbench arms, as a unified diff of its text.

  morecasts.py DIR_A DIR_B [--ida]
Reuses castbench.funcs/corpus; prints each function's cast delta and the changed lines."""
import difflib, sys
from pathlib import Path
sys.path.insert(0, "/home/mahaloz/kwt/castbench")
import castbench as CB, castcount as CC

A, B = sys.argv[1], sys.argv[2]
tot = 0
for opt, proj, b in CB.corpus("full"):
    fa, fb = CB.funcs(Path(A) / opt / proj / f"{b}.c"), CB.funcs(Path(B) / opt / proj / f"{b}.c")
    ida = CB.funcs(CB.RES / opt / proj / "decompiled" / f"ida_{b}.c")
    if not (fa and fb and ida):
        continue
    ta = {x: t for _, x, t in CC.split_functions((Path(A) / opt / proj / f"{b}.c").read_text(errors="replace"))}
    tb = {x: t for _, x, t in CC.split_functions((Path(B) / opt / proj / f"{b}.c").read_text(errors="replace"))}
    for a in sorted(set(fa) & set(fb) & set(ida)):
        if fb[a][2] > fa[a][2]:
            tot += fb[a][2] - fa[a][2]
            print(f"##### {opt}/{proj}/{b} @{hex(a)}  casts {fa[a][2]} -> {fb[a][2]} (IDA {ida[a][2]})")
            for L in difflib.unified_diff(ta[a].splitlines(), tb[a].splitlines(), lineterm="", n=0):
                if not L.startswith(("---", "+++", "@@")):
                    print("   " + L)
print(f"TOTAL +{tot}")
