"""Functions decompile-all emits (and how many carry an error) per build, for the speed binaries."""
import json, os, subprocess, sys
R = "/home/mahaloz/github/decbench/results/full_run_address_2026-09-11"
BINS = {"fmt": f"{R}/O2/coreutils/stripped/fmt", "ls": f"{R}/O2/coreutils/stripped/ls",
        "sort": f"{R}/O2/coreutils/stripped/sort", "bash": f"{R}/O2/bash/stripped/bash",
        "kmod-O2ni": f"{R}/O2-noinline/kmod/stripped/kmod",
        "dpkg-divert-O2ni": f"{R}/O2-noinline/dpkg/stripped/dpkg-divert",
        "crontab-O2ni": f"{R}/O2-noinline/cronie/stripped/crontab"}
SP = "/home/mahaloz/kwt/_final-main/specs"
BUILDS = {"base": ("/home/mahaloz/kwt/_baseline/kuna", "/home/mahaloz/github/kuna/specs"),
          "G": ("/home/mahaloz/kwt/_final-g/kuna", SP), "L": ("/home/mahaloz/kwt/_final-l/kuna", SP)}
out = {}
for name, b in BINS.items():
    out[name] = {}
    for lab, (kb, sp) in BUILDS.items():
        env = dict(os.environ, SLEIGHHOME=sp, KUNA_SPECS=sp)
        p = subprocess.run([kb, "decompile-all", b, "--json", "--max-fn-seconds", "120"], capture_output=True, env=env)
        d = json.loads(p.stdout)
        fs = d["functions"]
        err = sum(1 for f in fs if f.get("error"))
        lines = sum(len((f.get("code") or "").splitlines()) for f in fs)
        out[name][lab] = {"functions": len(fs), "errors": err, "lines": lines}
    print(name, out[name], flush=True)
json.dump(out, open("/home/mahaloz/kwt/_final-l/fncount10.json", "w"), indent=1)
print("FNCOUNT_DONE")
