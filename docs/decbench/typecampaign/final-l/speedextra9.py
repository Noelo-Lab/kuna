"""Interleaved min-of-N on kmod and dpkg-divert -O2-noinline: round F (before calleevote), round I, final (round L).

speedextraF.py with the round-L arm in place of round G."""
import json, os, statistics, subprocess, sys, time
R = "/home/mahaloz/github/decbench/results/full_run_address_2026-09-11"
ALL = {"kmod-O2ni": f"{R}/O2-noinline/kmod/stripped/kmod",
       "dpkg-divert-O2ni": f"{R}/O2-noinline/dpkg/stripped/dpkg-divert",
       "crontab-O2ni": f"{R}/O2-noinline/cronie/stripped/crontab"}
K = {"roundF": ("/home/mahaloz/kwt/_final-f/kuna", "/home/mahaloz/kwt/_final-main/specs"),
     "roundI": ("/home/mahaloz/kwt/_final-i/kuna", "/home/mahaloz/kwt/_final-main/specs"),
     "roundL": ("/home/mahaloz/kwt/_final-l/kuna", "/home/mahaloz/kwt/_final-main/specs")}
ARMS = ["roundF", "roundI", "roundL"]
N = int(sys.argv[1]) if len(sys.argv) > 1 else 15
outf = sys.argv[2] if len(sys.argv) > 2 else "/home/mahaloz/kwt/_final-l/speed-extra.json"
which = sys.argv[3].split(",") if len(sys.argv) > 3 else ["kmod-O2ni", "dpkg-divert-O2ni"]
out = {}
for name in which:
    b = ALL[name]
    t = {a: [] for a in ARMS}
    for i in range(N):
        order = ARMS[i % 3:] + ARMS[:i % 3]
        for lab in order:
            kb, sp = K[lab]
            env = dict(os.environ, SLEIGHHOME=sp, KUNA_SPECS=sp)
            t0 = time.perf_counter()
            p = subprocess.run([kb, "decompile-all", b, "--json", "--max-fn-seconds", "120"],
                               stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL, env=env)
            t[lab].append(round((time.perf_counter() - t0) * 1000, 1))
            if p.returncode != 0:
                print("RC", lab, name, p.returncode, flush=True)
    r = {lab: {"min_ms": min(v), "median_ms": statistics.median(v), "samples": v} for lab, v in t.items()}
    r["delta_min_pct_L_vs_I"] = round((r["roundL"]["min_ms"] - r["roundI"]["min_ms"]) / r["roundI"]["min_ms"] * 100, 2)
    r["delta_median_pct_L_vs_I"] = round((r["roundL"]["median_ms"] - r["roundI"]["median_ms"]) / r["roundI"]["median_ms"] * 100, 2)
    r["delta_min_pct_L_vs_F"] = round((r["roundL"]["min_ms"] - r["roundF"]["min_ms"]) / r["roundF"]["min_ms"] * 100, 2)
    r["delta_min_pct_I_vs_F"] = round((r["roundI"]["min_ms"] - r["roundF"]["min_ms"]) / r["roundF"]["min_ms"] * 100, 2)
    out[name] = r
    print(name, {lab: r[lab]["min_ms"] for lab in ARMS}, "L/I min", r["delta_min_pct_L_vs_I"], "med", r["delta_median_pct_L_vs_I"], "L/F min", r["delta_min_pct_L_vs_F"], "I/F min", r["delta_min_pct_I_vs_F"], os.getloadavg(), flush=True)
    json.dump(out, open(outf, "w"), indent=1)
print("SPEEDEXTRA_DONE")
