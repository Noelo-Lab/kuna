"""Interleaved min-of-N wall clock, base build (main eaa19ebbb) vs #706, same command."""
import json, os, subprocess, sys, time
R="/home/mahaloz/github/decbench/results/full_run_address_2026-09-11"
BINS=sys.argv[2].split(",") if len(sys.argv)>2 else ["O2/grep/grep","O2/coreutils/sort","O2/coreutils/wc","O0/grep/grep","O2/rsyslog/rsyslogd"]
N=int(sys.argv[1]) if len(sys.argv)>1 else 15
S="/home/mahaloz/kwt/libcstructs/.scratch/r6"
env=dict(os.environ, SLEIGHHOME="/home/mahaloz/kwt/libcstructs/specs", KUNA_SPECS="/home/mahaloz/kwt/libcstructs/specs")
out={}
for spec in BINS:
    opt,proj,b=spec.split("/")
    path=f"{R}/{opt}/{proj}/stripped/{b}"
    best={"base":1e9,"new":1e9}
    for i in range(N):
        for arm in ("base","new"):
            k={"base":f"{S}/base/kuna","new":f"{S}/final/kuna"}[arm]
            t=time.time()
            subprocess.run([k,"decompile-all",path,"--max-fn-seconds","120"],stdout=subprocess.DEVNULL,stderr=subprocess.DEVNULL,env=env)
            best[arm]=min(best[arm],time.time()-t)
    d=(best["new"]-best["base"])/best["base"]*100
    out[spec]={"base_s":round(best["base"],3),"new_s":round(best["new"],3),"delta_pct":round(d,2)}
    print(f"{spec:22s} base {best['base']:7.3f}s  new {best['new']:7.3f}s  {d:+6.2f}%", flush=True)
json.dump({"n":N,"method":"interleaved min-of-N, kuna decompile-all --max-fn-seconds 120","rows":out},open(f"{S}/speed.json","w"),indent=2)
print("SPEED_DONE")
