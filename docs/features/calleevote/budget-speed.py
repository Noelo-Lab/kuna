"""Interleaved min-of-N and median-of-per-iteration-ratios, two BUILDS, defaults both."""
import json, os, statistics, subprocess, sys, time
D = "/home/mahaloz/github/decbench/results/full_run_address_2026-09-11"
ALL = {"fmt_O2": f"{D}/O2/coreutils/stripped/fmt", "ls_O2": f"{D}/O2/coreutils/stripped/ls",
       "sort_O2": f"{D}/O2/coreutils/stripped/sort", "bash_O2": f"{D}/O2/bash/stripped/bash",
       "kmod_O2ni": f"{D}/O2-noinline/kmod/stripped/kmod",
       "dpkgdivert_O2": f"{D}/O2/dpkg/stripped/dpkg-divert",
       "crontab_O2ni": f"{D}/O2-noinline/cronie/stripped/crontab",
       "cmp_O0": f"{D}/O0/diffutils/stripped/cmp"}
SP = "/home/mahaloz/kwt/calleevoteperf/specs"
BEFORE, AFTER, N, outf = sys.argv[1], sys.argv[2], int(sys.argv[3]), sys.argv[4]
which = sys.argv[5].split(",") if len(sys.argv) > 5 else list(ALL)
out = json.load(open(outf)) if os.path.exists(outf) else {}
for name in which:
    b = ALL[name]; t = {"before": [], "after": []}; ratios = []
    for i in range(N):
        order = ["before", "after"] if i % 2 == 0 else ["after", "before"]
        per = {}
        for lab in order:
            k = BEFORE if lab == "before" else AFTER
            env = dict(os.environ, SLEIGHHOME=SP, KUNA_SPECS=SP)
            t0 = time.perf_counter()
            p = subprocess.run([k, "decompile-all", b, "--json", "--max-fn-seconds", "120"],
                               stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL, env=env)
            ms = round((time.perf_counter() - t0) * 1000, 1)
            t[lab].append(ms); per[lab] = ms
            if p.returncode != 0: print("RC", lab, name, p.returncode, flush=True)
        ratios.append(per["after"] / per["before"])
    r = {lab: {"min_ms": min(v), "median_ms": statistics.median(v), "samples": v} for lab, v in t.items()}
    r["delta_min_pct"] = round((r["after"]["min_ms"] - r["before"]["min_ms"]) / r["before"]["min_ms"] * 100, 2)
    r["delta_median_of_ratios_pct"] = round((statistics.median(ratios) - 1) * 100, 2)
    r["loadavg"] = os.getloadavg()
    out[name] = r
    print(name, "min", r["delta_min_pct"], "% median-of-ratios", r["delta_median_of_ratios_pct"], "%",
          "before", r["before"]["min_ms"], "after", r["after"]["min_ms"], os.getloadavg(), flush=True)
    json.dump(out, open(outf, "w"), indent=1)
print("SPEED_DONE", flush=True)
