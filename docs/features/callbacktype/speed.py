"""Interleaved min-of-N, `callbacktype off` vs `on`, one kuna binary, both arms."""
import json, os, statistics, subprocess, sys, time
R = "/home/mahaloz/github/decbench/results/full_run_address_2026-09-11"
ALL = {"fmt-O2": f"{R}/O2/coreutils/stripped/fmt",
       "ls-O2": f"{R}/O2/coreutils/stripped/ls",
       "sort-O2": f"{R}/O2/coreutils/stripped/sort",
       "bash-O2": f"{R}/O2/bash/stripped/bash",
       "tar-O0": f"{R}/O0/tar/stripped/tar",
       "libselinux-O0": f"{R}/O0/libselinux/stripped/libselinux.so.1",
       "libselinux-O2-noinline": f"{R}/O2-noinline/libselinux/stripped/libselinux.so.1"}
K = os.environ.get("KUNA_BIN", "/home/mahaloz/kwt/callbacktype/decompiler/target/release/kuna")
SP = "/home/mahaloz/kwt/callbacktype/specs"
ARMS = ["off", "on"]
N = int(sys.argv[1]) if len(sys.argv) > 1 else 15
outf = sys.argv[2] if len(sys.argv) > 2 else "/home/mahaloz/kwt/callbacktype/.scratch/speed.json"
which = sys.argv[3].split(",") if len(sys.argv) > 3 else list(ALL)
out = {}
for name in which:
    b = ALL[name]
    t = {a: [] for a in ARMS}
    for i in range(N):
        order = ARMS[i % 2:] + ARMS[:i % 2]
        for lab in order:
            env = dict(os.environ, SLEIGHHOME=SP, KUNA_SPECS=SP)
            t0 = time.perf_counter()
            p = subprocess.run([K, "decompile-all", b, "--json", "--max-fn-seconds", "120",
                                "--option", "callbacktype", lab],
                               stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL, env=env)
            t[lab].append(round((time.perf_counter() - t0) * 1000, 1))
            if p.returncode != 0:
                print("RC", lab, name, p.returncode, flush=True)
    r = {lab: {"min_ms": min(v), "median_ms": statistics.median(v), "samples": v} for lab, v in t.items()}
    r["delta_min_pct"] = round((r["on"]["min_ms"] - r["off"]["min_ms"]) / r["off"]["min_ms"] * 100, 2)
    r["delta_median_pct"] = round((r["on"]["median_ms"] - r["off"]["median_ms"]) / r["off"]["median_ms"] * 100, 2)
    out[name] = r
    print(name, {lab: r[lab]["min_ms"] for lab in ARMS}, "on/off min", r["delta_min_pct"],
          "med", r["delta_median_pct"], os.getloadavg(), flush=True)
    json.dump(out, open(outf, "w"), indent=1)
print("SPEED_DONE")
