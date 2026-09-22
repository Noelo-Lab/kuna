import json, collections, sys
sys.path.insert(0, '/home/mahaloz/kwt/o2census/.scratch/py')
from extra import LIBC  # noqa
R = json.load(open('/home/mahaloz/kwt/o2census/.scratch/sweep-o2/rows.json'))
RD = json.load(open('/home/mahaloz/kwt/_final-d/sweep-d/rows.json'))


def tier(d):
    s = "arg" if d["gt_is_arg"] else ("stack" if d["gt_has_off"] else "reg")
    if s == "reg":
        return "U"
    if (d["cls"] or "").startswith("ptr") and d["gt_type"] == "void":
        return "U"
    if d["cls"] == "ptr_struct" and d["gt_type"] not in LIBC:
        return "S"
    return "P"


res = {}
for opt in ("O2", "O2-noinline"):
    V = collections.Counter()
    F = collections.Counter()
    rx = collections.Counter()
    for k, v in R.items():
        if k.split("::")[1] != opt:
            continue
        vals = v["base"]["values"]
        by = collections.defaultdict(list)
        for d in v["decisions"]["base"]:
            by[d["fn"]].append(d)
            t = tier(d)
            V[t] += 1
            V[t + "_tp"] += d["ok"] is True
        for fn, ds in by.items():
            if fn not in vals:
                continue
            ts = {tier(d) for d in ds}
            p = vals[fn] == 1.0
            if any(d["pass"].startswith("rx") for d in ds):
                rx["fn"] += 1
                rx["gt"] += len(ds)
                rx["gt_arg"] += sum(d["gt_is_arg"] for d in ds)
                rx["no_reg"] += all(tier(d) != "U" for d in ds)
            if "U" in ts:
                F["U"] += 1
                F["U_perfect"] += p
            elif "S" in ts:
                F["S"] += 1
                F["S_perfect"] += p
                if all(d["ok"] is True or tier(d) == "S" and d["kuna_type"] and d["kuna_type"].startswith("struct_") for d in ds):
                    F["S_credit93"] += 1
            else:
                F["P"] += 1
                F["P_perfect"] += p
    res[opt] = {"vars": dict(V), "fns": dict(F), "rx": dict(rx)}
# O0 comparison from round-D rows (storage only)
F0 = collections.Counter()
for k, v in RD.items():
    if k.split("::")[1] != "O0":
        continue
    vals = v["base"]["values"]
    by = collections.defaultdict(list)
    for d in v["decisions"]["base"]:
        by[d["fn"]].append(d)
    for fn, ds in by.items():
        if fn not in vals:
            continue
        ts = {tier(d) for d in ds}
        p = vals[fn] == 1.0
        key = "U" if "U" in ts else ("S" if "S" in ts else "P")
        F0[key] += 1
        F0[key + "_perfect"] += p
res["O0"] = {"fns": dict(F0)}
print(json.dumps(res, indent=1))
json.dump(res, open('/home/mahaloz/kwt/o2census/.scratch/ceiling.json', 'w'), indent=1)
