import json, subprocess, sys, pathlib, concurrent.futures as cf
R = "/home/mahaloz/github/decbench/results/full_run_address_2026-09-11"
BINS = ["O0/bzip2/stripped/bzip2", "O2/bash/stripped/bash", "O2/kmod/stripped/kmod",
        "O2/dpkg/stripped/dpkg-divert", "O0/shadow/stripped/chage", "O2/cronie/stripped/crontab",
        "O2-noinline/dash/stripped/dash", "O0/cronie/stripped/crond"]
kuna, out = sys.argv[1], pathlib.Path(sys.argv[2])
arms = {"on": [], "off": ["--option", "castimplied", "off"]}
def run(job):
    arm, b = job
    p = subprocess.run([kuna, "decompile-all", f"{R}/{b}", "--json", *arms[arm]], capture_output=True, text=True, timeout=7200)
    if p.returncode != 0:
        return f"{arm} {b} rc={p.returncode} {p.stderr[-200:]}"
    d = json.loads(p.stdout)
    o = out / arm / (b.replace("/stripped/", "/") + ".c")
    o.parent.mkdir(parents=True, exist_ok=True)
    o.write_text("\n".join(f"// Function: {f['name']} @ {hex(f['address'])}\n{f['code']}\n" for f in d.get("functions", []) if f.get("code")))
    return f"{arm} {b} ok {len(d.get('functions', []))} functions"
with cf.ThreadPoolExecutor(max_workers=6) as ex:
    for r in ex.map(run, [(a, b) for b in BINS for a in arms]):
        print(r, flush=True)
