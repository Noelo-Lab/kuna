"""Stage-3 re-measure driver (read-only on the decbench results tree).

  kuna   : typesweep --baseline-only over the slice set, with per-GT-variable
           class-tagged decisions; writes <out>/rows.json in typesweep's format
           (re-report it with `typesweep --report-only --out <out>`).
  rivals : replay decbench's metric over the checkpoint's stored results of
           the named decompilers for the same slices; writes <out>/rivals.json.
"""
from __future__ import annotations

import argparse
import json
import os
import pickle
import sys
from concurrent.futures import ProcessPoolExecutor, as_completed
from pathlib import Path

sys.path.insert(0, os.environ.get("KUNA_TREE", "/home/mahaloz/kwt/_final-main"))
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

import pindb  # noqa: E402

pindb.pin()

import gtclass  # noqa: E402
from scripts.decbench import config  # noqa: E402
from scripts.decbench import typesweep as ts  # noqa: E402


def kuna_work(task):
    sink: list = []
    gtclass.install(sink)
    t = list(task)
    t[8] = False
    key, out = ts.score_slice(tuple(t))
    out["decisions"] = {"base": list(sink)}
    return key, out


def rival_work(task):
    project, opt, stem, results = task
    from decbench.metrics.type_match import TypeMatchMetric
    out = {}
    for dname, res in results.items():
        sink: list = []
        gtclass.install(sink)
        try:
            mr = TypeMatchMetric().compute_for_binary(res)
        except Exception as e:  # noqa: BLE001
            out[dname] = {"error": str(e)[:200]}
            continue
        out[dname] = {
            "values": {fn: float(mv.value) for fn, mv in mr.function_results.items()},
            "decisions": sink,
        }
    return ts.slice_key(project, opt, stem), out


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("mode", choices=["kuna", "rivals"])
    ap.add_argument("--opt", action="append", default=[])
    ap.add_argument("--project", action="append", default=[])
    ap.add_argument("--workers", type=int, default=24)
    ap.add_argument("--timeout", type=int, default=1800)
    ap.add_argument("--rival", action="append", default=[])
    ap.add_argument("--out", type=Path, required=True)
    a = ap.parse_args()
    a.out.mkdir(parents=True, exist_ok=True)
    root = config.results_root()

    if a.mode == "kuna":
        cachef = a.out / "rows.json"
        rows = json.loads(cachef.read_text()) if cachef.exists() else {}
        slices = ts.collect_slices(root, set(a.project), set(a.opt))
        todo = [(p, o, s, b, x, [], a.timeout, ["base"], False)
                for (p, o, s, b, x) in slices if ts.slice_key(p, o, s) not in rows]
        print(f"KUNA_BIN={config.kuna_bin()} slices {len(slices)} todo {len(todo)}", flush=True)
        done = 0
        with ProcessPoolExecutor(max_workers=a.workers) as ex:
            futs = [ex.submit(kuna_work, t) for t in todo]
            for f in as_completed(futs):
                done += 1
                try:
                    k, v = f.result()
                except Exception as e:  # noqa: BLE001
                    print(f"[{done}/{len(todo)}] WORKER-ERR {e}", flush=True)
                    continue
                rows[k] = v
                if done % 20 == 0:
                    cachef.write_text(json.dumps(rows))
                print(f"[{done}/{len(todo)}] {k}", flush=True)
        cachef.write_text(json.dumps(rows))
        print("FINALSWEEP_DONE", flush=True)
        return

    import decbench.decompilers  # noqa: F401
    rivals = a.rival or ["ida", "binja", "ghidra"]
    outf = a.out / "rivals.json"
    res = json.loads(outf.read_text()) if outf.exists() else {}
    tasks = []
    for pk in sorted((root / "checkpoints").glob("*.pkl")):
        if a.project and pk.stem not in a.project:
            continue
        data = pickle.loads(pk.read_bytes())
        for opt, bins in (data.get("decompile") or {}).items():
            optn = getattr(opt, "value", str(opt))
            if a.opt and optn not in a.opt:
                continue
            for stem, decs in bins.items():
                k = decs.get("kuna")
                if k is None or not getattr(k, "functions", None):
                    continue
                key = ts.slice_key(pk.stem, optn, stem)
                if key in res:
                    continue
                sel = {d: decs[d] for d in rivals + ["kuna"]
                       if decs.get(d) is not None and getattr(decs[d], "functions", None)}
                tasks.append((pk.stem, optn, stem, sel))
        del data
    print(f"rival slices todo {len(tasks)}", flush=True)
    done = 0
    with ProcessPoolExecutor(max_workers=a.workers) as ex:
        futs = [ex.submit(rival_work, t) for t in tasks]
        for f in as_completed(futs):
            done += 1
            try:
                k, v = f.result()
            except Exception as e:  # noqa: BLE001
                print(f"[{done}] WORKER-ERR {e}", flush=True)
                continue
            res[k] = v
            if done % 20 == 0:
                outf.write_text(json.dumps(res))
            print(f"[{done}/{len(tasks)}] {k}", flush=True)
    outf.write_text(json.dumps(res))
    print("FINALSWEEP_DONE", flush=True)


if __name__ == "__main__":
    main()
