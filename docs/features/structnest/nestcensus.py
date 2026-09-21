"""Census of predicted struct-pointer members of parameter records vs DWARF, per arm."""
import json, os, sys, collections
from concurrent.futures import ProcessPoolExecutor
from pathlib import Path
# Run under the campaign metric pin: see docs/decbench/typecampaign/final-c/pindb.py.
sys.path.insert(0, os.environ.get("PINDB_DIR", str(Path(__file__).resolve().parents[2] / "decbench/typecampaign/final-c")))
import pindb; pindb.pin()
sys.path.insert(0, str(Path(__file__).resolve().parents[3]))
from scripts.decbench import structscore as SS
RES = Path(os.environ.get("KUNA_DECBENCH_RESULTS", "/home/mahaloz/github/decbench/results/full_run_address_2026-09-11"))
ARMS = {a: [["structsynth", a]] for a in os.environ.get("ARMS", "param,nest").split(",")}
EXTRA = [kv.split(" ", 1) for kv in os.environ.get("EXTRA", "").split(",") if kv.strip()]
BINS = [tuple(x.split(":")) for x in os.environ["BINS"].split(",")]
DIES = {}
_ident = SS.struct_identity
def ident(die):
    i = _ident(die); DIES.setdefault(i, die); return i
SS.struct_identity = ident

def fields_only(layout):
    return {(f["offset"], f["size"]) for f in layout["fields"] if not f.get("array") and f["kind"] != "pad"}

def mark(header):
    layouts = SS.header_layouts(header)
    for name, body in SS.STRUCT_RE.findall(header):
        counts = {}
        for line in body.split("\n"):
            m = SS.MEMBER_RE.match(line)
            if m: counts[m.group("name")] = bool(m.group("count"))
        for f in layouts.get(name, {}).get("fields", []):
            f["array"] = counts.get(f["name"], False)
    return layouts

def one(job):
    proj, b, o, arm = job
    binary = RES / o / proj / "stripped" / b
    fns = SS.dwarf_functions(RES / o / proj / "compiled" / b)
    payload = SS.run_json(binary, ARMS[arm] + EXTRA, 3600)
    header = SS.run_header(binary, ARMS[arm] + EXTRA, 3600)
    layouts = mark(header)
    by_addr = {int(f.get("address") or -1): f for f in payload.get("functions") or []}
    rows = []
    for low, info in fns.items():
        fn = by_addr.get(low)
        if not fn: continue
        for p in info["params"]:
            G = p["struct"]
            if not G: continue
            pred = None
            for v in fn.get("variables") or []:
                if v.get("arg_index") == p["index"]:
                    r = SS.parse_type(v.get("type"))
                    if r["kind"] == "struct": pred = r["base"]
                    break
            S = layouts.get(pred) if pred else None
            if not S: continue
            for f in S["fields"]:
                if not f["pointee"]: continue
                gt = next((g for g in G["fields"] if g["offset"] == f["offset"] and g["kind"] != "pad"), None)
                inner = layouts.get(f["pointee"])
                row = {"bin": f"{b} {o}", "fn": info["name"], "param": p["index"], "outer": pred, "off": f["offset"],
                       "pointee": f["pointee"], "self": f["pointee"] == pred, "gt_outer": G["name"]}
                if gt is None: row["cls"] = "gt_none"
                elif gt["pointee"]:
                    row["cls"] = "gt_struct_ptr"; row["gt_pointee"] = gt["pointee"]
                    row["gt_self"] = gt["pointee"] == G["name"]
                    die = DIES.get(gt["pointee"]); GL = SS.struct_layout(die) if die is not None else None
                    if GL and inner:
                        pf = fields_only(inner); gf = fields_only(GL)
                        row["inner_tp"] = len(pf & gf); row["inner_pred"] = len(pf)
                elif gt["ptr"]: row["cls"] = "gt_other_ptr"
                else: row["cls"] = "gt_nonptr"
                rows.append(row)
    return (f"{b} {o}", arm, rows)

if __name__ == "__main__":
    jobs = [(p, b, o, a) for (p, b, o) in BINS for a in ARMS]
    with ProcessPoolExecutor(int(os.environ.get("W", "8"))) as ex:
        res = list(ex.map(one, jobs))
    agg = {a: collections.Counter() for a in ARMS}
    keyed = {a: {} for a in ARMS}
    for name, arm, rows in res:
        for r in rows:
            c = agg[arm]; c["pred"] += 1; c[r["cls"]] += 1
            if r["cls"] == "gt_struct_ptr":
                c["inner_tp"] += r.get("inner_tp", 0); c["inner_pred"] += r.get("inner_pred", 0)
                if r["self"]: c["self_ok" if r["gt_self"] else "self_wrong"] += 1
            elif r["self"]: c["self_wrong"] += 1
            keyed[arm][(r["bin"], r["fn"], r["param"], r["off"])] = r
    for a, c in agg.items():
        ip = c["inner_tp"] / c["inner_pred"] if c["inner_pred"] else 0
        print(f"ARM {a}: " + " ".join(f"{k}={v}" for k, v in sorted(c.items())) + f" inner_precision={ip:.4f}")
    arms = list(ARMS)
    if len(arms) == 2:
        a, b = arms
        for k in sorted(set(keyed[b]) - set(keyed[a])):
            r = keyed[b][k]; print("NEW", json.dumps(r))
        for k in sorted(set(keyed[a]) - set(keyed[b])):
            r = keyed[a][k]; print("GONE", json.dumps(r))
    if os.environ.get("OUT"):
        json.dump(res, open(os.environ["OUT"], "w"), indent=1)
