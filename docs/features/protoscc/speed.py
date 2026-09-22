"""Interleaved min-of-N decompile-all timing of protoorder arms on one build."""
import json, os, statistics, subprocess, sys, time
R = "/home/mahaloz/github/decbench/results/full_run_address_2026-09-11/O2"
ALL = {"fmt": f"{R}/coreutils/stripped/fmt", "ls": f"{R}/coreutils/stripped/ls",
       "sort": f"{R}/coreutils/stripped/sort", "bash": f"{R}/bash/stripped/bash"}
K = sys.argv[1]
N = int(sys.argv[2])
outf = sys.argv[3]
which = sys.argv[4].split(",")
ARMS = {"types": (["--option", "protoorder", "types"], {}),
        "cycles": (["--option", "protoorder", "cycles"], {}),
        "cycles_norepass": (["--option", "protoorder", "cycles"], {"KUNA_EXP_PROTOSCC": "norepass"})}
arms = sys.argv[5].split(",") if len(sys.argv) > 5 else list(ARMS)
sp = "/home/mahaloz/kwt/protoscc/specs"
out = {}
for name in which:
    t = {a: [] for a in arms}
    for i in range(N):
        order = arms[i % len(arms):] + arms[:i % len(arms)]
        for lab in order:
            opts, extra = ARMS[lab]
            env = dict(os.environ, SLEIGHHOME=sp, KUNA_SPECS=sp, **extra)
            env.pop("KUNA_EXP_PROTOSCC", None) if not extra else None
            t0 = time.perf_counter()
            p = subprocess.run([K, "decompile-all", ALL[name], "--json", "--max-fn-seconds", "120"] + opts,
                               stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL, env=env)
            t[lab].append(round((time.perf_counter() - t0) * 1000, 1))
            if p.returncode != 0:
                print("RC", lab, name, p.returncode, flush=True)
    r = {lab: {"min_ms": min(v), "median_ms": statistics.median(v), "samples": v} for lab, v in t.items()}
    for lab in arms[1:]:
        r[f"delta_min_pct_{lab}_vs_{arms[0]}"] = round((r[lab]["min_ms"] - r[arms[0]]["min_ms"]) / r[arms[0]]["min_ms"] * 100, 2)
        r[f"delta_median_pct_{lab}_vs_{arms[0]}"] = round((r[lab]["median_ms"] - r[arms[0]]["median_ms"]) / r[arms[0]]["median_ms"] * 100, 2)
    out[name] = r
    print(name, {lab: r[lab]["min_ms"] for lab in arms}, {k: v for k, v in r.items() if k.startswith("delta")}, os.getloadavg(), flush=True)
    json.dump(out, open(outf, "w"), indent=1)
print("SPEED_DONE", flush=True)
