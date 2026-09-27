import sys, os, subprocess, concurrent.futures as cf
from pathlib import Path
sys.path.insert(0, "/home/mahaloz/kwt/castbench")
import castbench as CB
kuna = sys.argv[1]; out = Path(sys.argv[2]); which = sys.argv[3] if len(sys.argv) > 3 else "full"
extra = sys.argv[4:]
out.mkdir(parents=True, exist_ok=True)
env = dict(os.environ, KUNA_CW_CENSUS="1")
def one(j):
    opt, proj, b = j
    binary = CB.RES / opt / proj / "stripped" / b
    p = subprocess.run([kuna, "decompile-all", str(binary), "--json", *extra], capture_output=True, text=True, env=env, timeout=3600)
    lines = sorted(set(l for l in p.stderr.splitlines() if l.startswith("CWCENSUS")))
    (out / f"{opt}__{proj}__{b}.tsv").write_text("\n".join(lines) + "\n")
    return f"{opt}/{b} rc={p.returncode} n={len(lines)}"
with cf.ThreadPoolExecutor(12) as ex:
    for r in ex.map(one, list(CB.corpus(which))):
        print(r, flush=True)
