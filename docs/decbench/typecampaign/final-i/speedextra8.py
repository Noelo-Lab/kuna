"""Interleaved min-of-N on the three binaries #711's review flagged (round G vs round H vs final)."""
import json, os, statistics, subprocess, sys, time
R = "/home/mahaloz/github/decbench/results/full_run_address_2026-09-11"
ALL = {"kmod-O2ni": f"{R}/O2-noinline/kmod/stripped/kmod",
       "crontab-O2ni": f"{R}/O2-noinline/cronie/stripped/crontab",
       "dpkg-divert-O2ni": f"{R}/O2-noinline/dpkg/stripped/dpkg-divert"}
K = {"roundG": ("/home/mahaloz/kwt/_final-g/kuna", "/home/mahaloz/kwt/_final-main/specs"),
     "roundH": ("/home/mahaloz/kwt/_final-i/bh/kuna", "/home/mahaloz/kwt/_final-main/specs"),
     "roundI": ("/home/mahaloz/kwt/_final-i/kuna", "/home/mahaloz/kwt/_final-main/specs")}
ARMS = ["roundG", "roundH", "roundI"]
N = int(sys.argv[1]) if len(sys.argv) > 1 else 15
outf = sys.argv[2] if len(sys.argv) > 2 else "/home/mahaloz/kwt/_final-i/speed-extra.json"
which = sys.argv[3].split(",") if len(sys.argv) > 3 else list(ALL)
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
    r["delta_min_pct"] = round((r["roundI"]["min_ms"] - r["roundG"]["min_ms"]) / r["roundG"]["min_ms"] * 100, 2)
    r["delta_median_pct"] = round((r["roundI"]["median_ms"] - r["roundG"]["median_ms"]) / r["roundG"]["median_ms"] * 100, 2)
    r["delta_min_pct_I_vs_H"] = round((r["roundI"]["min_ms"] - r["roundH"]["min_ms"]) / r["roundH"]["min_ms"] * 100, 2)
    out[name] = r
    print(name, {lab: r[lab]["min_ms"] for lab in ARMS}, "I/G min", r["delta_min_pct"], "med", r["delta_median_pct"], "I/H min", r["delta_min_pct_I_vs_H"], os.getloadavg(), flush=True)
    json.dump(out, open(outf, "w"), indent=1)
print("SPEEDEXTRA_DONE")
