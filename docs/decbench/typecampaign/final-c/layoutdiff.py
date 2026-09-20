"""Per-parameter layout diff between two option arms (why layout precision moved)."""
import sys
from pathlib import Path

sys.path.insert(0, "/home/mahaloz/kwt/_final-c/tools")
sys.path.insert(0, "/home/mahaloz/kwt/_final-main")

import pindb  # noqa: E402

pindb.pin()

from scripts.decbench import structscore as SS  # noqa: E402


def arm(binary, options):
    payload = SS.run_json(binary, options, 1800)
    header = SS.run_header(binary, options, 1800)
    return payload, SS.header_layouts(header)


def rows(binary, functions, payload, layouts):
    out = {}
    by_addr = {int(f.get("address") or -1): f for f in payload.get("functions") or []}
    for low, info in functions.items():
        fn = by_addr.get(low)
        for param in info["params"]:
            layout = param["struct"]
            if not layout or not fn:
                continue
            gt_real = [f for f in layout["fields"] if f["kind"] != "pad"]
            gt_keys = {(f["offset"], f["size"]) for f in gt_real}
            pred = None
            for v in fn.get("variables") or []:
                if v.get("arg_index") == param["index"]:
                    rec = SS.parse_type(v.get("type"))
                    if rec["kind"] == "struct":
                        pred = (rec["base"], layouts.get(rec["base"]))
                    break
            if not pred or not pred[1]:
                continue
            tp = sum(1 for f in pred[1]["fields"] if (f["offset"], f["size"]) in gt_keys)
            out[(info["name"], param["index"])] = (
                layout["name"], len(gt_real), pred[0], len(pred[1]["fields"]), tp)
    return out


def main():
    binary = Path(sys.argv[1])
    functions = SS.dwarf_functions(SS.twin_of(binary))
    pa, la = arm(binary, [])
    pb, lb = arm(binary, [["protoorder", "off"]])
    ra, rb = rows(binary, functions, pa, la), rows(binary, functions, pb, lb)
    worse = []
    for k in set(ra) & set(rb):
        if ra[k][4] < rb[k][4]:
            worse.append((rb[k][4] - ra[k][4], k, rb[k], ra[k]))
    worse.sort(reverse=True)
    print(f"params typed as a struct: default {len(ra)}  protoorder-off {len(rb)}")
    print(f"params where the default claims fewer true fields: {len(worse)}")
    for d, k, b, a in worse[:20]:
        print(f"  {k[0]}(arg {k[1]}): GT {b[0]} {b[1]}f | off -> {b[2]} {b[3]}f tp={b[4]}"
              f" | default -> {a[2]} {a[3]}f tp={a[4]}")


if __name__ == "__main__":
    main()
