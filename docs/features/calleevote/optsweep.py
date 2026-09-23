"""finalsweep kuna mode with extra --option pairs (arm 'base' carries them)."""
import argparse, json, os, sys
from concurrent.futures import ProcessPoolExecutor, as_completed
from pathlib import Path
sys.path.insert(0, os.environ.get("KUNA_TREE"))
sys.path.insert(0, "/home/mahaloz/kwt/_final-e/tools")
import pindb  # noqa
pindb.pin()
import gtclass  # noqa
from scripts.decbench import config  # noqa
from scripts.decbench import typesweep as ts  # noqa

def kuna_work(task):
    sink = []
    gtclass.install(sink)
    key, out = ts.score_slice(task)
    out["decisions"] = {"base": list(sink)}
    return key, out

def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--opt", action="append", default=[])
    ap.add_argument("--project", action="append", default=[])
    ap.add_argument("--option", nargs=2, action="append", default=[])
    ap.add_argument("--workers", type=int, default=12)
    ap.add_argument("--timeout", type=int, default=1800)
    ap.add_argument("--out", type=Path, required=True)
    a = ap.parse_args()
    a.out.mkdir(parents=True, exist_ok=True)
    root = config.results_root()
    opts = [tuple(p) for p in a.option]
    cachef = a.out / "rows.json"
    rows = json.loads(cachef.read_text()) if cachef.exists() else {}
    slices = ts.collect_slices(root, set(a.project), set(a.opt))
    todo = [(p, o, s, b, x, [], a.timeout, {"base": opts}, False)
            for (p, o, s, b, x) in slices if ts.slice_key(p, o, s) not in rows]
    print(f"KUNA_BIN={config.kuna_bin()} opts={opts} slices {len(slices)} todo {len(todo)}", flush=True)
    done = 0
    with ProcessPoolExecutor(max_workers=a.workers) as ex:
        futs = [ex.submit(kuna_work, t) for t in todo]
        for f in as_completed(futs):
            done += 1
            try:
                k, v = f.result()
            except Exception as e:  # noqa
                print(f"[{done}/{len(todo)}] WORKER-ERR {e}", flush=True); continue
            rows[k] = v
            if done % 20 == 0: cachef.write_text(json.dumps(rows))
            print(f"[{done}/{len(todo)}] {k}", flush=True)
    cachef.write_text(json.dumps(rows))
    print("FINALSWEEP_DONE", flush=True)

if __name__ == "__main__":
    main()
