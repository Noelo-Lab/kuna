#!/usr/bin/env python3
"""Run decompile-all with KUNA_GLOBALREF_TRACE over the castbench corpus; save C (castbench format) + trace."""
import concurrent.futures as cf, json, os, subprocess, sys
from pathlib import Path
sys.path.insert(0, "/home/mahaloz/kwt/castbench")
import castbench as CB
kbin, out, which = sys.argv[1], Path(sys.argv[2]), sys.argv[3]
opts = sys.argv[4:]
env = dict(os.environ, KUNA_GLOBALREF_TRACE="1")
def one(j):
    opt, proj, b = j
    cmd = [kbin, "decompile-all", str(CB.RES / opt / proj / "stripped" / b), "--json"]
    for i in range(0, len(opts), 2): cmd += ["--option", opts[i], opts[i + 1]]
    p = subprocess.run(cmd, capture_output=True, text=True, env=env, timeout=3600)
    o = out / opt / proj; o.mkdir(parents=True, exist_ok=True)
    (o / f"{b}.trace").write_text("\n".join(l for l in p.stderr.splitlines() if l.startswith("[globalref")))
    if p.returncode: return f"{j} rc={p.returncode}"
    d = json.loads(p.stdout)
    (o / f"{b}.c").write_text("\n".join(f"// Function: {f['name']} @ {hex(f['address'])}\n{f['code']}\n" for f in d.get("functions", []) if f.get("code")))
    return None
jobs = list(CB.corpus(which))
with cf.ThreadPoolExecutor(12) as ex:
    for e in ex.map(one, jobs):
        if e: print("FAIL", e, flush=True)
print("done", len(jobs))
