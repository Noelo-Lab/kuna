"""Interleaved min-of-N `decompile-all` timing over three arms: main, this build at its defaults, this build with fieldtype on.
usage: speed3.py N OUT.json BIN[,BIN...]"""
import json, os, resource, statistics, subprocess, sys, time
R = "/home/mahaloz/github/decbench/results/full_run_address_2026-09-11"
BINS = {"fmt": f"{R}/O2/coreutils/stripped/fmt", "ls": f"{R}/O2/coreutils/stripped/ls",
        "sort": f"{R}/O2/coreutils/stripped/sort", "bash": f"{R}/O2/bash/stripped/bash"}
WT = "/home/mahaloz/kwt/fieldtype"; SP = f"{WT}/specs"
ARMS = {"main": (f"{WT}/.scratch/bin-main-4f037dae4/kuna", []),
        "default": (f"{WT}/.scratch/bin-v11/kuna", []),
        "on": (f"{WT}/.scratch/bin-v11/kuna", ["--option", "fieldtype", "on"])}
N = int(sys.argv[1]); outf = sys.argv[2]
LMAX = float(os.environ.get("SPEED_LOAD_MAX", "40")); WMAX = float(os.environ.get("SPEED_WAIT_MAX", "900"))
out = json.load(open(outf)) if os.path.exists(outf) else {}
env = dict(os.environ, SLEIGHHOME=SP, KUNA_SPECS=SP)
names = list(ARMS)
for name in sys.argv[3].split(","):
    b = BINS[name]
    for k, x in ARMS.values():
        subprocess.run([k, "decompile-all", b, "--json", "--max-fn-seconds", "120"] + x, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL, env=env)
    t = {a: [] for a in names}; c = {a: [] for a in names}; loads = []
    for i in range(N):
        w0 = time.time()
        while os.getloadavg()[0] >= LMAX and time.time() - w0 < WMAX:
            time.sleep(15)
        loads.append(round(os.getloadavg()[0], 1))
        order = names[i % 3:] + names[:i % 3]
        for lab in order:
            k, x = ARMS[lab]
            u0 = resource.getrusage(resource.RUSAGE_CHILDREN); t0 = time.perf_counter()
            p = subprocess.run([k, "decompile-all", b, "--json", "--max-fn-seconds", "120"] + x, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL, env=env)
            t[lab].append(round((time.perf_counter() - t0) * 1000, 1))
            u1 = resource.getrusage(resource.RUSAGE_CHILDREN)
            c[lab].append(round((u1.ru_utime - u0.ru_utime + u1.ru_stime - u0.ru_stime) * 1000, 1))
            if p.returncode != 0:
                print("RC", lab, name, p.returncode, flush=True)
    r = {lab: {"min_ms": min(v), "median_ms": statistics.median(v), "samples": v, "cpu_min_ms": min(c[lab]), "cpu_samples": c[lab]} for lab, v in t.items()}
    def d(a, bb):
        ratios = [x / y for x, y in zip(t[a], t[bb])]
        return {"min": round((r[a]["min_ms"] / r[bb]["min_ms"] - 1) * 100, 2),
                "median_of_ratios": round((statistics.median(ratios) - 1) * 100, 2),
                "cpu_min": round((r[a]["cpu_min_ms"] / r[bb]["cpu_min_ms"] - 1) * 100, 2)}
    r["delta_pct"] = {"default/main": d("default", "main"), "on/main": d("on", "main"), "on/default": d("on", "default")}
    r["round_loads"] = loads
    out[name] = r
    print(name, {a: r[a]["min_ms"] for a in names}, r["delta_pct"], flush=True)
    json.dump(out, open(outf, "w"), indent=1)
print("SPEED_DONE", flush=True)
