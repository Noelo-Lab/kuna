"""decompile-all, castsign off vs on (default), over binaries outside the castbench set.

Writes <out>/{off,on}/<name>.c in castbench's per-function layout so
hunkclass.py can classify every changed line.
  python3 corpus-extra.py <kuna> <out>
"""
import concurrent.futures as cf
import json
import subprocess
import sys
from pathlib import Path

D = "/home/mahaloz/github/decbench/results/full_run_address_2026-09-11"
BINS = {
    "bash_O2": f"{D}/O2/bash/stripped/bash",
    "dash_O2ni": f"{D}/O2-noinline/dash/stripped/dash",
    "cf2_O2ni": f"{D}/O2-noinline/crazyflie/stripped/cf2.elf",
    "kmod_O2ni": f"{D}/O2-noinline/kmod/stripped/kmod",
    "bzip2_O0": f"{D}/O0/bzip2/stripped/bzip2",
    "crontab_O0": f"{D}/O0/cronie/stripped/crontab",
}
K, OUT = sys.argv[1], Path(sys.argv[2])


def one(job):
    name, arm = job
    cmd = [K, "decompile-all", BINS[name], "--json"] + (["--option", "castsign", "off"] if arm == "off" else [])
    p = subprocess.run(cmd, capture_output=True, text=True)
    if p.returncode != 0:
        return f"{name} {arm} rc={p.returncode}"
    d = json.loads(p.stdout)
    parts = [f"// Function: {f['name']} @ {hex(f['address'])}\n{f['code']}\n" for f in d["functions"] if f.get("code")]
    (OUT / arm).mkdir(parents=True, exist_ok=True)
    (OUT / arm / f"{name}.c").write_text("\n".join(parts))
    vars_ = {f["address"]: f.get("variables") for f in d["functions"]}
    (OUT / arm / f"{name}.vars.json").write_text(json.dumps(vars_, sort_keys=True))
    return f"{name} {arm} ok {len(parts)} functions"


with cf.ThreadPoolExecutor(4) as ex:
    for r in ex.map(one, [(n, a) for n in BINS for a in ("off", "on")]):
        print(r, flush=True)
print("CORPUS_DONE")
