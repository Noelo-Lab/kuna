"""Interleaved min-of-N A/B between two kuna builds on identical commands."""
import subprocess, time, sys, os, statistics, json
R="/home/mahaloz/github/decbench/results/full_run_address_2026-09-11"
REF="/home/mahaloz/kwt/libctypes-stdio-ref/decompiler/target/release/kuna"
NEW="/home/mahaloz/kwt/libctypes-stdio/decompiler/target/release/kuna"
SPECS_REF="/home/mahaloz/kwt/libctypes-stdio-ref/specs"
SPECS_NEW="/home/mahaloz/kwt/libctypes-stdio/specs"
CASES=[
 ("fmt decompile-all",      lambda k:[k,"decompile-all",f"{R}/O2/coreutils/stripped/fmt"]),
 ("useradd decompile-all",  lambda k:[k,"decompile-all",f"{R}/O2/shadow/stripped/useradd"]),
 ("libstdbuf.so decompile-all", lambda k:[k,"decompile-all",f"{R}/O2/coreutils/stripped/libstdbuf.so"]),
 ("fmt functions (load only)", lambda k:[k,"functions",f"{R}/O2/coreutils/stripped/fmt"]),
]
N=int(sys.argv[1]) if len(sys.argv)>1 else 15
def run(k, specs, argv):
    e=dict(os.environ); e["SLEIGHHOME"]=specs; e["KUNA_SPECS"]=specs
    t=time.perf_counter()
    subprocess.run(argv, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL, env=e, check=False)
    return (time.perf_counter()-t)*1000
out={}
for name,mk in CASES:
    a=[];b=[]
    for i in range(N):
        if i%2==0:
            a.append(run(REF,SPECS_REF,mk(REF))); b.append(run(NEW,SPECS_NEW,mk(NEW)))
        else:
            b.append(run(NEW,SPECS_NEW,mk(NEW))); a.append(run(REF,SPECS_REF,mk(REF)))
    ma,mb=min(a),min(b); da,db=statistics.median(a),statistics.median(b)
    out[name]=dict(min_ref=ma,min_new=mb,med_ref=da,med_new=db,
                   min_pct=(mb-ma)/ma*100, med_pct=(db-da)/da*100, n=N)
    print(f"{name:32s} min {ma:9.1f} -> {mb:9.1f} ms  ({(mb-ma)/ma*100:+.2f}%)   median {da:9.1f} -> {db:9.1f} ms ({(db-da)/da*100:+.2f}%)", flush=True)
json.dump(out, open(".scratch/timeab.json","w"), indent=2)
