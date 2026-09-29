"""Interleaved min-of-N whole-binary `decompile-all` timing over named arms.

usage: speed10.py N OUT.json BIN[,BIN...] ARM[,ARM...]
An arm is a pinned build (base, F, G, I, J, K, L) or `L-<opt>[=<value>][+...]`: the final build with those options off (or at <value>).
The arm order rotates every round; the table reports min and median per arm and each arm's delta against
every earlier-listed build arm (an option arm: the final build's delta against it, the cost of
those options). Each round of arms waits until the 1-minute load average is under SPEED_LOAD_MAX
(default 20; at most SPEED_WAIT_MAX seconds, default 1200) and records it; child CPU time (user+sys) is
kept beside wall time as the contention-robust cross-check."""
import json, os, resource, statistics, subprocess, sys, time
R = "/home/mahaloz/github/decbench/results/full_run_address_2026-09-11"
BINS = {"fmt": f"{R}/O2/coreutils/stripped/fmt", "ls": f"{R}/O2/coreutils/stripped/ls",
        "sort": f"{R}/O2/coreutils/stripped/sort", "bash": f"{R}/O2/bash/stripped/bash",
        "kmod-O2ni": f"{R}/O2-noinline/kmod/stripped/kmod",
        "dpkg-divert-O2ni": f"{R}/O2-noinline/dpkg/stripped/dpkg-divert",
        "crontab-O2ni": f"{R}/O2-noinline/cronie/stripped/crontab"}
SP = "/home/mahaloz/kwt/_final-main/specs"
BUILDS = {"base": ("/home/mahaloz/kwt/_baseline/kuna", "/home/mahaloz/github/kuna/specs"),
          "F": ("/home/mahaloz/kwt/_final-f/kuna", SP), "G": ("/home/mahaloz/kwt/_final-g/kuna", SP),
          "I": ("/home/mahaloz/kwt/_final-i/kuna", SP), "L": ("/home/mahaloz/kwt/_final-l/kuna", SP),
          "J": ("/home/mahaloz/kwt/castbench/bin-c960fb18d/kuna", SP),
          "K": ("/home/mahaloz/kwt/castbench/bin-0096e984d/kuna", SP)}
def arm(a):
    if a in BUILDS:
        kb, sp = BUILDS[a]
        return kb, sp, []
    base, _, opts = a.partition("-")
    kb, sp = BUILDS[base]
    extra = []
    for o in opts.split("+"):
        o, _, v = o.partition("=")
        extra += ["--option", o, v or "off"]
    return kb, sp, extra
N = int(sys.argv[1]); outf = sys.argv[2]
LMAX = float(os.environ.get("SPEED_LOAD_MAX", "20")); WMAX = float(os.environ.get("SPEED_WAIT_MAX", "1200"))
which = sys.argv[3].split(","); arms = sys.argv[4].split(",")
out = json.load(open(outf)) if os.path.exists(outf) else {}
for name in which:
    b = BINS[name]
    t = {a: [] for a in arms}
    c = {a: [] for a in arms}
    loads = []
    for i in range(N):
        w0 = time.time()
        while os.getloadavg()[0] >= LMAX and time.time() - w0 < WMAX:
            time.sleep(15)
        loads.append(round(os.getloadavg()[0], 1))
        order = arms[i % len(arms):] + arms[:i % len(arms)]
        for lab in order:
            kb, sp, extra = arm(lab)
            env = dict(os.environ, SLEIGHHOME=sp, KUNA_SPECS=sp)
            u0 = resource.getrusage(resource.RUSAGE_CHILDREN)
            t0 = time.perf_counter()
            p = subprocess.run([kb, "decompile-all", b, "--json", "--max-fn-seconds", "120"] + extra,
                               stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL, env=env)
            t[lab].append(round((time.perf_counter() - t0) * 1000, 1))
            u1 = resource.getrusage(resource.RUSAGE_CHILDREN)
            c[lab].append(round((u1.ru_utime - u0.ru_utime + u1.ru_stime - u0.ru_stime) * 1000, 1))
            if p.returncode != 0:
                print("RC", lab, name, p.returncode, flush=True)
    r = {lab: {"min_ms": min(v), "median_ms": statistics.median(v), "samples": v,
               "cpu_min_ms": min(c[lab]), "cpu_samples": c[lab]} for lab, v in t.items()}
    r["round_loads"] = loads
    d = {}
    pairs = [(a, b2) for j, a in enumerate(arms) for b2 in arms[:j] if a in BUILDS]
    pairs += [("L", a) for a in arms if a not in BUILDS and "L" in arms]
    for a, b2 in pairs:
        d[f"{a}/{b2}"] = [round((r[a][k] - r[b2][k]) / r[b2][k] * 100, 2)
                          for k in ("min_ms", "median_ms", "cpu_min_ms")]
    r["delta_min_median_pct"] = d
    r["loadavg_end"] = os.getloadavg()
    out[name] = r
    print(name, {lab: r[lab]["min_ms"] for lab in arms}, d, os.getloadavg(), flush=True)
    json.dump(out, open(outf, "w"), indent=1)
print("SPEED10_DONE", flush=True)
