"""Interleaved min-of-N wall clock, base build vs new build, same command."""
import json, os, subprocess, sys, time
R="/home/mahaloz/github/decbench/results/full_run_address_2026-09-11"
BINS=[("O2/tar/tar"),("O2/grep/grep"),("O2/coreutils/ls"),("O0/grep/grep"),("O2/coreutils/sort")]
N=int(sys.argv[1]) if len(sys.argv)>1 else 15
env=dict(os.environ, SLEIGHHOME="/home/mahaloz/kwt/libcstructs/specs",
         KUNA_SPECS="/home/mahaloz/kwt/libcstructs/specs")
out={}
for spec in BINS:
    opt,proj,b=spec.split("/")
    path=f"{R}/{opt}/{proj}/stripped/{b}"
    best={"base":1e9,"new":1e9}
    for i in range(N):
        for arm in ("base","new"):
            k=f"/home/mahaloz/kwt/libcstructs/.scratch/kuna-{arm}"
            t=time.time()
            subprocess.run([k,"decompile-all",path,"--max-fn-seconds","120"],
                           stdout=subprocess.DEVNULL,stderr=subprocess.DEVNULL,env=env)
            best[arm]=min(best[arm],time.time()-t)
    d=(best["new"]-best["base"])/best["base"]*100
    out[spec]={"base_s":round(best["base"],3),"new_s":round(best["new"],3),"delta_pct":round(d,2)}
    print(f"{spec:22s} base {best['base']:7.3f}s  new {best['new']:7.3f}s  {d:+6.2f}%", flush=True)
json.dump(out,open("/home/mahaloz/kwt/libcstructs/.scratch/speed.json","w"),indent=2)
