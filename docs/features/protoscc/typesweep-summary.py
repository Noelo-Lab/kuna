#!/usr/bin/env python3
"""Per-project and per-class summary of a `typesweep --dump-decisions` run.

    python3 typesweep-summary.py <sweep-dir> <class-rows.json>

<sweep-dir>/rows.json is typesweep's output (arms `base` and `test`).
<class-rows.json> is a campaign `finalsweep` rows.json whose decisions carry the
DWARF class (`cls`) of every ground-truth variable; the ground truth does not
depend on the kuna build, so a variable is joined to its class by (slice,
function, position in the function's ground-truth list), checked by name.
"""
import collections
import json
import sys

rows = json.load(open(sys.argv[1] + "/rows.json"))
cls_rows = json.load(open(sys.argv[2]))

cls_of = {}
for k, v in cls_rows.items():
    per = collections.defaultdict(list)
    for d in (v.get("decisions") or {}).get("base") or []:
        per[d["fn"]].append((d.get("gt_name"), d.get("cls")))
    for fn, lst in per.items():
        for i, (name, c) in enumerate(lst):
            cls_of[(k, fn, i)] = (name, c)

proj = collections.defaultdict(lambda: collections.Counter())
cls_tp = {"base": collections.Counter(), "test": collections.Counter()}
cls_n = collections.Counter()
cls_moves = collections.Counter()
unjoined = 0
for k, v in rows.items():
    p = k.split("::")[0]
    b = (v.get("base") or {}).get("values") or {}
    t = (v.get("test") or {}).get("values") or {}
    for fn in b:
        if fn not in t:
            continue
        c = proj[p]
        c["n"] += 1
        c["perfect_off"] += b[fn] == 1.0
        c["perfect_on"] += t[fn] == 1.0
        c["sum_off"] += b[fn]
        c["sum_on"] += t[fn]
        c["improved"] += t[fn] > b[fn] + 1e-9
        c["worse"] += t[fn] < b[fn] - 1e-9
    dec = v.get("decisions") or {}
    pos = {}
    verdict = {}
    for arm in ("base", "test"):
        per = collections.defaultdict(int)
        for d in dec.get(arm) or []:
            i = per[d["fn"]]
            per[d["fn"]] += 1
            name, c = cls_of.get((k, d["fn"], i), (None, None))
            if c is None or name != d.get("gt_name"):
                unjoined += arm == "base"
                c = "unjoined"
            verdict[(arm, d["fn"], i)] = (c, d["ok"] is True)
    for (arm, fn, i), (c, ok) in verdict.items():
        if arm == "base":
            cls_n[c] += 1
        cls_tp[arm][c] += ok
        if arm == "test":
            before = verdict.get(("base", fn, i), (c, False))[1]
            if ok and not before:
                cls_moves[(c, "gain")] += 1
            if before and not ok:
                cls_moves[(c, "loss")] += 1

out = {"projects": {}, "pooled": collections.Counter()}
for p, c in sorted(proj.items()):
    out["projects"][p] = {"n": c["n"], "perfect_off": c["perfect_off"], "perfect_on": c["perfect_on"],
                          "mean_off": round(c["sum_off"] / c["n"], 4), "mean_on": round(c["sum_on"] / c["n"], 4),
                          "improved": c["improved"], "worse": c["worse"]}
    out["pooled"].update(c)
pc = out["pooled"]
out["pooled"] = {"n": pc["n"], "perfect_off": pc["perfect_off"], "perfect_on": pc["perfect_on"],
                 "aggregate_off": round(pc["sum_off"], 2), "aggregate_on": round(pc["sum_on"], 2),
                 "mean_off": round(pc["sum_off"] / pc["n"], 4), "mean_on": round(pc["sum_on"] / pc["n"], 4),
                 "improved": pc["improved"], "worse": pc["worse"]}
out["classes"] = {c: {"gt": cls_n[c], "tp_off": cls_tp["base"][c], "tp_on": cls_tp["test"][c],
                      "gained": cls_moves[(c, "gain")], "lost": cls_moves[(c, "loss")]}
                  for c in sorted(cls_n) if cls_tp["base"][c] != cls_tp["test"][c] or c == "ptr_char"}
out["unjoined_gt_vars"] = unjoined
print(json.dumps(out, indent=1))
