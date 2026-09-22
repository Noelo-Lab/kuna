"""o2census sweep: typesweep --baseline-only with provenance-tagged GT and per-function kuna vars."""
import argparse, json, os, sys
from concurrent.futures import ProcessPoolExecutor, as_completed
from pathlib import Path
sys.path.insert(0, os.environ["KUNA_TREE"])
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import pindb  # noqa
pindb.pin()
import o2gt  # noqa
from scripts.decbench import config  # noqa
from scripts.decbench import typesweep as ts  # noqa


def work(task):
    sink, fsink = [], []
    o2gt.install(sink, fsink)
    t = list(task)
    t[8] = False
    key, out = ts.score_slice(tuple(t))
    out["decisions"] = {"base": list(sink)}
    out["funcs"] = list(fsink)
    out["addr2name"] = {str(a): n for a, n in task[4].items()}
    return key, out


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--opt", action="append", default=[])
    ap.add_argument("--project", action="append", default=[])
    ap.add_argument("--workers", type=int, default=12)
    ap.add_argument("--out", type=Path, required=True)
    a = ap.parse_args()
    a.out.mkdir(parents=True, exist_ok=True)
    cachef = a.out / "rows.json"
    rows = json.loads(cachef.read_text()) if cachef.exists() else {}
    slices = ts.collect_slices(config.results_root(), set(a.project), set(a.opt))
    todo = [(p, o, s, b, x, [], 1800, ["base"], False) for (p, o, s, b, x) in slices
            if ts.slice_key(p, o, s) not in rows]
    print(f"KUNA_BIN={config.kuna_bin()} slices {len(slices)} todo {len(todo)}", flush=True)
    done = 0
    with ProcessPoolExecutor(max_workers=a.workers) as ex:
        futs = [ex.submit(work, t) for t in todo]
        for f in as_completed(futs):
            done += 1
            try:
                k, v = f.result()
            except Exception as e:  # noqa
                print(f"[{done}] WORKER-ERR {e}", flush=True)
                continue
            rows[k] = v
            if done % 20 == 0:
                cachef.write_text(json.dumps(rows))
            print(f"[{done}/{len(todo)}] {k}", flush=True)
    cachef.write_text(json.dumps(rows))
    print("O2SWEEP_DONE", flush=True)


if __name__ == "__main__":
    main()
