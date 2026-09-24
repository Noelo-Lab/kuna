"""Print one function from two castbench arms (and IDA) side by side: castshow.py ARM_A ARM_B OPT PROJ BIN ADDR"""
import sys
from pathlib import Path
sys.path.insert(0, "/home/mahaloz/kwt/castbench")
import castbench as CB, castcount as CC
a, b, opt, proj, binn, addr = sys.argv[1:7]
addr = int(addr, 16)
def get(p):
    for _, x, t in CC.split_functions(Path(p).read_text(errors="replace")):
        if x == addr:
            return t
for lab, p in ((a, f"{a}/{opt}/{proj}/{binn}.c"), (b, f"{b}/{opt}/{proj}/{binn}.c"),
               ("ida", CB.RES / opt / proj / "decompiled" / f"ida_{binn}.c")):
    print(f"===== {lab}"); print(get(p))
