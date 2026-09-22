"""Where rivals are perfect at O2/O2-noinline and kuna is not."""
import collections, json, re
RV = json.load(open("/home/mahaloz/kwt/_final/rivals/rivals.json"))
K = json.load(open("/home/mahaloz/kwt/o2census/.scratch/sweep-o2/rows.json"))
out = {}
for opt in ("O2", "O2-noinline"):
    row = {}
    for r in ("binja", "ida", "ghidra", "angr"):
        both = only_r = only_k = 0
        shapes = collections.Counter()
        ex = collections.defaultdict(list)
        for key, v in K.items():
            if key.split("::")[1] != opt or key not in RV:
                continue
            kv = v["base"]["values"]
            rv = (RV[key].get(r) or {}).get("values") or {}
            kd = collections.defaultdict(list)
            for d in v["decisions"]["base"]:
                kd[d["fn"]].append(d)
            rd = collections.defaultdict(list)
            for d in (RV[key].get(r) or {}).get("decisions") or []:
                rd[d["fn"]].append(d)
            for fn, x in rv.items():
                if fn not in kv:
                    continue
                if x == 1.0 and kv[fn] == 1.0:
                    both += 1
                elif x == 1.0:
                    only_r += 1
                    for d in kd[fn]:
                        if d["ok"] is True:
                            continue
                        s = "arg" if d["gt_is_arg"] else ("stack" if d["gt_has_off"] else "reg")
                        rt = next((q.get("kuna_type") for q in rd[fn] if q["gt_name"] == d["gt_name"]), None)
                        sh = (s, d["cls"], re.sub(r"struct_\d+", "struct_N", str(d.get("kuna_type"))), str(rt))
                        shapes[sh] += 1
                        ex[sh].append((key, fn, d["gt_name"], d["gt_type"]))
                elif kv[fn] == 1.0:
                    only_k += 1
        row[r] = {"both": both, "rival_only": only_r, "kuna_only": only_k,
                  "shapes": [[list(s), n, ex[s][:3]] for s, n in shapes.most_common(15)]}
    out[opt] = row
json.dump(out, open("/home/mahaloz/kwt/o2census/.scratch/rivals.json", "w"), indent=1)
for opt, row in out.items():
    for r, x in row.items():
        print(opt, r, x["both"], x["rival_only"], x["kuna_only"])
