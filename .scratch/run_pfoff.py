import sys, json, os, subprocess, collections, concurrent.futures as cf
from pathlib import Path
sys.path.insert(0,".")
import run_census as RC, castcount as CC
KUNA="/home/mahaloz/kwt/castcensus/decompiler/target/release/kuna"
def job(a):
    o,p,b=a
    out=Path("out/pfoff")/o/p/f"{b}.c"
    r=RC.kuna_c(KUNA, RC.RES/o/p/"stripped"/b, out, extra=("--option","ptrfromuse","off"))
    return o,p,b,r[1]
jobs=[(o,p,b) for o in RC.OPTS for p,bs in RC.TARGETS.items() for b in bs
      if (RC.RES/o/p/"stripped"/b).exists()]
with cf.ThreadPoolExecutor(max_workers=10) as ex:
    for o,p,b,err in ex.map(job,jobs):
        if err: print("FAIL",o,p,b,err)
print("done", len(jobs))
