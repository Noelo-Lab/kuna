"""decompile-all, castsign off vs on (default), over a third set of binaries outside the castbench set
(the review's disjoint set; corpus-extra.py is the second).

Writes <out>/{off,on}/<name>.c in castbench's per-function layout so
hunkclass.py can classify every changed line.
  python3 corpus-third.py <kuna> <out>
"""
import concurrent.futures as cf
import json
import subprocess
import sys
from pathlib import Path

D = "/home/mahaloz/github/decbench/results/full_run_address_2026-09-11"
BINS = {
    "factor_O0": f"{D}/O0/coreutils/stripped/factor",
    "factor_O2": f"{D}/O2/coreutils/stripped/factor",
    "dd_O0": f"{D}/O0/coreutils/stripped/dd",
    "date_O0": f"{D}/O0/coreutils/stripped/date",
    "expr_O0": f"{D}/O0/coreutils/stripped/expr",
    "cksum_O0": f"{D}/O0/coreutils/stripped/cksum",
    "stat_O0": f"{D}/O0/coreutils/stripped/stat",
    "seq_O0": f"{D}/O0/coreutils/stripped/seq",
    "init_O0": f"{D}/O0/sysvinit/stripped/init",
    "groupadd_O0": f"{D}/O0/shadow/stripped/groupadd",
    "mirai_O0": f"{D}/O0/mirai/stripped/mirai",
    "certtool_O0": f"{D}/O0/gnutls/stripped/certtool",
    "e2fsck_O0": f"{D}/O0/e2fsprogs/stripped/e2fsck",
    "ip_O2": f"{D}/O2/iproute2/stripped/ip",
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


with cf.ThreadPoolExecutor(8) as ex:
    for r in ex.map(one, [(n, a) for n in BINS for a in ("off", "on")]):
        print(r, flush=True)
print("CORPUS_DONE")
