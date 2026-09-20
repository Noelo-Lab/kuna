"""Per-parameter layout accounting: default vs protoorder off, both directions."""
import sys
from pathlib import Path
sys.path.insert(0, "/home/mahaloz/kwt/layoutvote/.scratch/tools")
sys.path.insert(0, "/home/mahaloz/kwt/layoutvote")
import pindb
pindb.pin()
from scripts.decbench import structscore as SS


def arm(binary, options):
    return SS.run_json(binary, options, 1800), SS.header_layouts(SS.run_header(binary, options, 1800))


def rows(binary, functions, payload, layouts, fillerless):
    out = {}
    by_addr = {int(f.get("address") or -1): f for f in payload.get("functions") or []}
    for low, info in functions.items():
        fn = by_addr.get(low)
        for param in info["params"]:
            layout = param["struct"]
            if not layout or not fn:
                continue
            gt_keys = {(f["offset"], f["size"]) for f in layout["fields"] if f["kind"] != "pad"}
            pred = None
            for v in fn.get("variables") or []:
                if v.get("arg_index") == param["index"]:
                    rec = SS.parse_type(v.get("type"))
                    if rec["kind"] == "struct":
                        pred = (rec["base"], layouts.get(rec["base"]))
                    break
            if not pred or not pred[1]:
                continue
            fields = [f for f in pred[1]["fields"] if not (fillerless and f["name"] in fillerless.get(pred[0], set()))]
            tp = sum(1 for f in fields if (f["offset"], f["size"]) in gt_keys)
            out[(info["name"], param["index"])] = (layout["name"], len(gt_keys), pred[0], len(fields), tp)
    return out


def filler(binary, options):
    header = SS.run_header(binary, options, 1800)
    out = {}
    for name, body in SS.STRUCT_RE.findall(header):
        for line in body.split("\n"):
            m = SS.MEMBER_RE.match(line)
            if m and m.group("count"):
                out.setdefault(name, set()).add(m.group("name"))
    return out


def main():
    binary = Path(sys.argv[1])
    fillerless = "--fields-only" in sys.argv
    functions = SS.dwarf_functions(SS.twin_of(binary))
    fa = filler(binary, []) if fillerless else None
    fb = filler(binary, [["protoorder", "off"]]) if fillerless else None
    pa, la = arm(binary, [])
    pb, lb = arm(binary, [["protoorder", "off"]])
    ra, rb = rows(binary, functions, pa, la, fa), rows(binary, functions, pb, lb, fb)
    only_a = sorted(set(ra) - set(rb))
    print(f"struct-typed params: default {len(ra)}  off {len(rb)}  default-only {len(only_a)}  off-only {len(set(rb)-set(ra))}")
    tp = sum(ra[k][4] for k in only_a)
    pr = sum(ra[k][3] for k in only_a)
    print(f"default-only params: tp {tp} / predicted {pr} = {tp/pr if pr else 0:.4f}")
    both = set(ra) & set(rb)
    for tag, r in (("default", ra), ("off", rb)):
        t = sum(r[k][4] for k in both); p = sum(r[k][3] for k in both)
        print(f"shared params ({tag}): tp {t} / predicted {p} = {t/p if p else 0:.4f}")
    worse = sorted(((rb[k][4] - ra[k][4], k) for k in both if ra[k][4] < rb[k][4]), reverse=True)
    print(f"shared params where the default claims fewer true fields: {len(worse)}")
    for d, k in worse[:10]:
        print(f"  {k[0]}(arg {k[1]}): GT {rb[k][0]} {rb[k][1]}f | off -> {rb[k][2]} {rb[k][3]}f tp={rb[k][4]}"
              f" | default -> {ra[k][2]} {ra[k][3]}f tp={ra[k][4]}")
    bad = sorted(((ra[k][3] - ra[k][4], k) for k in only_a), reverse=True)
    print("worst default-only params (claimed - true):")
    for d, k in bad[:12]:
        print(f"  {k[0]}(arg {k[1]}): GT {ra[k][0]} {ra[k][1]}f | default -> {ra[k][2]} {ra[k][3]}f tp={ra[k][4]}")


main()
