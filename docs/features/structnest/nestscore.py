"""Layout P/R (fields only, filler counted) + nesting P/R/F1 per arm, both arms same build."""
import json, os, sys
from concurrent.futures import ProcessPoolExecutor
from pathlib import Path
# Run under the campaign metric pin: see docs/decbench/typecampaign/final-c/pindb.py.
sys.path.insert(0, os.environ.get("PINDB_DIR", str(Path(__file__).resolve().parents[2] / "decbench/typecampaign/final-c")))
import pindb; pindb.pin()
sys.path.insert(0, str(Path(__file__).resolve().parents[3]))
from scripts.decbench import structscore as SS
RES = Path(os.environ.get("KUNA_DECBENCH_RESULTS", "/home/mahaloz/github/decbench/results/full_run_address_2026-09-11"))
ARMS = {"param": [["structsynth", "param"]], "nest": [["structsynth", "nest"]]}
BINS = [("coreutils", b, o) for b in ("fmt", "ls", "sort", "du") for o in ("O0", "O2")] + \
       [("grep", "grep", "O2"), ("tar", "tar", "O2"), ("findutils", "find", "O2"), ("diffutils", "diff", "O2"),
        ("grep", "grep", "O0"), ("tar", "tar", "O0")]
if os.environ.get("BINS"):
    BINS = [tuple(x.split(":")) for x in os.environ["BINS"].split(",")]

_raw = SS.header_layouts
def mark_arrays(header):
    layouts = _raw(header)
    for name, body in SS.STRUCT_RE.findall(header):
        if name not in layouts: continue
        counts = {}
        for line in body.split("\n"):
            m = SS.MEMBER_RE.match(line)
            if m: counts[m.group("name")] = bool(m.group("count"))
        for f in layouts[name]["fields"]:
            f["array"] = counts.get(f["name"], False)
    return layouts
def drop_filler(header):
    return {n: {**l, "fields": [f for f in l["fields"] if not f.get("array")]} for n, l in mark_arrays(header).items()}

def one(job):
    proj, b, o, arm = job
    binary = RES / o / proj / "stripped" / b
    fns = SS.dwarf_functions(RES / o / proj / "compiled" / b)
    payload = SS.run_json(binary, ARMS[arm], 3600)
    header = SS.run_header(binary, ARMS[arm], 3600)
    out = {}
    for tag, hook in (("filler", mark_arrays), ("fields", drop_filler), ("raw", _raw)):
        SS.header_layouts = hook
        m = SS.measure_layout(payload, header, fns)
        out[tag] = m
    SS.header_layouts = _raw
    return (f"{b} {o}", arm, {"layout_fields": out["fields"]["layout"], "layout_filler": out["filler"]["layout"],
            "nesting": out["raw"]["nesting"], "structs": out["raw"]["structs_in_header"],
            "typed_params": out["raw"]["params_kuna_typed_as_a_struct"]})

if __name__ == "__main__":
    jobs = [(p, b, o, a) for (p, b, o) in BINS for a in ARMS]
    with ProcessPoolExecutor(int(os.environ.get("W", "8"))) as ex:
        res = list(ex.map(one, jobs))
    pooled = {}
    for name, arm, r in res:
        print(f"{name:10s} {arm:5s} layoutF P={r['layout_fields']['precision']} ({r['layout_fields']['tp']}/{r['layout_fields']['predicted']}) "
              f"R={r['layout_fields']['recall']} | nest tp={r['nesting']['tp']} pred={r['nesting']['predicted']} gt={r['nesting']['ground_truth']} F1={r['nesting']['f1']} | structs={r['structs']} typed={r['typed_params']}")
        for k in ("layout_fields", "layout_filler", "nesting"):
            p = pooled.setdefault(arm, {}).setdefault(k, {"tp": 0, "predicted": 0, "ground_truth": 0})
            for kk in p: p[kk] += r[k][kk]
        pooled[arm].setdefault("structs", 0); pooled[arm]["structs"] += r["structs"]
        pooled[arm].setdefault("typed", 0); pooled[arm]["typed"] += r["typed_params"]
    for arm, d in pooled.items():
        for k in ("layout_fields", "layout_filler", "nesting"):
            p = d[k]; pr = p["tp"]/p["predicted"] if p["predicted"] else 0; rc = p["tp"]/p["ground_truth"] if p["ground_truth"] else 0
            f1 = 2*pr*rc/(pr+rc) if pr+rc else 0
            print(f"POOLED {arm} {k}: P {p['tp']}/{p['predicted']}={pr:.4f} R {p['tp']}/{p['ground_truth']}={rc:.4f} F1 {f1:.4f}")
        print(f"POOLED {arm} structs={d['structs']} typed_params={d['typed']}")
    if os.environ.get("OUT"):
        json.dump({"rows": res, "pooled": pooled}, open(os.environ["OUT"], "w"), indent=1)
