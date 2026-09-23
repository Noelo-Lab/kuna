import sys,collections
sys.path.insert(0,".")
import castcount as CC
from pathlib import Path
RES=Path("/home/mahaloz/github/decbench/results/full_run_address_2026-09-11")
OUT=Path("shared"); OUT.mkdir(exist_ok=True)
T={"coreutils":["fmt","ls","sort","du","cp","tail","wc"],"grep":["grep"],"gzip":["gzip"],
   "diffutils":["cmp","diff","diff3","sdiff"],"tar":["tar"],"findutils":["find"]}
def fn(p):
    try: s=Path(p).read_text(errors="replace")
    except OSError: return None
    return {a:t for n,a,t in CC.split_functions(s)}
for opt in ("O0","O2","O2-noinline"):
    for proj,bins in T.items():
        for b in bins:
            d=RES/opt/proj/"decompiled"
            k=fn(f"out/new/{opt}/{proj}/{b}.c"); i=fn(d/f"ida_{b}.c")
            if not k or not i: continue
            sh=sorted(set(k)&set(i))
            for tag,src in (("kuna",k),("ida",i),("ghidra",fn(d/f"ghidra_{b}.c")),
                            ("angr",fn(d/f"angr_{b}.c")),("kbase",fn(f"out/base/{opt}/{proj}/{b}.c"))):
                if not src: continue
                p=OUT/tag/opt/proj; p.mkdir(parents=True,exist_ok=True)
                (p/f"{b}.c").write_text("\n".join(src[a] for a in sh if a in src))
print("ok")
