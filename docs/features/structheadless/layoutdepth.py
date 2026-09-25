#!/usr/bin/env python3
"""Per-parameter layout precision against DWARF, top level and at depth.

    KUNA_BIN=<kuna> DECBENCH_PIN=<pinned decbench> ~/.virtualenvs/decbench/bin/python \
        layoutdepth.py [ARM_JSON]

`structscore.measure_layout` scores a claimed field only against the TOP-LEVEL
members of the ground-truth record, and keeps a named embedded record as one
member (`struct stat stat` at 0x18, 0x90 bytes). A reader of `f->stat.st_mode`
claims 4 bytes at 0x30, which is a member of the record at depth two and a miss
at the top level. This instrument scores the same claims both ways and sorts
every top-level miss by what DWARF has at that offset: a member of an embedded
record, an element of an array member, a union slot, padding, or another member
at the same offset with a different width.
"""
import collections, json, os, sys
from concurrent.futures import ProcessPoolExecutor
from pathlib import Path
sys.path.insert(0, "/home/mahaloz/kwt/structdeep/docs/decbench/typecampaign/final-c")
import pindb; pindb.pin()
sys.path.insert(0, str(Path(__file__).resolve().parents[3]))
from scripts.decbench import structscore as SS
from decbench.utils import binfmt

RES = Path("/home/mahaloz/github/decbench/results/full_run_address_2026-09-11")
BINS = [("coreutils", b, o) for b in ("fmt", "ls", "sort", "du") for o in ("O0", "O2")]
ARMS = json.loads(sys.argv[1]) if len(sys.argv) > 1 else {
    "param": [["structsynth", "param"]],
    "param+headless": [["structsynth", "param"], ["structheadless", "closed"]]}


def leaves(die, base_off=0, depth=0, out=None):
    """Every scalar/pointer member of a record at any depth, arrays as element runs."""
    out = [] if out is None else out
    if die is None or depth > 8:
        return out
    for m in die.iter_children():
        if m.tag != "DW_TAG_member":
            continue
        off = SS._member_offset(m)
        if off is None:
            continue
        b, ptr = SS._resolve(SS._type_die(m))
        size = SS._die_size(SS._type_die(m))
        at = base_off + off
        if not ptr and b is not None and b.tag in ("DW_TAG_structure_type", "DW_TAG_class_type"):
            out.append((at, size, "struct", depth))
            leaves(b, at, depth + 1, out)
        elif not ptr and b is not None and b.tag == "DW_TAG_union_type":
            out.append((at, size, "union", depth))
            for um in b.iter_children():
                if um.tag != "DW_TAG_member":
                    continue
                ub, uptr = SS._resolve(SS._type_die(um))
                usz = SS._die_size(SS._type_die(um))
                if not uptr and ub is not None and ub.tag in ("DW_TAG_structure_type", "DW_TAG_class_type"):
                    leaves(ub, at, depth + 1, out)
                else:
                    out.append((at, usz, "leaf", depth + 1))
        elif not ptr and b is not None and b.tag == "DW_TAG_array_type":
            out.append((at, size, "array", depth))
            eb, eptr = SS._resolve(SS._type_die(b))
            esz = SS._die_size(SS._type_die(b)) if eb is not None or eptr else None
            if esz and size:
                for k in range(min(size // esz, 64)):
                    if not eptr and eb is not None and eb.tag in ("DW_TAG_structure_type", "DW_TAG_class_type"):
                        leaves(eb, at + k * esz, depth + 1, out)
                    else:
                        out.append((at + k * esz, esz, "element", depth + 1))
        else:
            out.append((at, size, "leaf", depth))
    return out


def gt_params(twin):
    res = {}
    info = binfmt.dwarf_info(twin)
    for cu in info.iter_CUs():
        for die in cu.get_top_DIE().iter_children():
            if die.tag != "DW_TAG_subprogram":
                continue
            low = SS._attr(die, "DW_AT_low_pc")
            if low is None:
                continue
            for i, ch in enumerate(c for c in die.iter_children() if c.tag == "DW_TAG_formal_parameter"):
                b, ptr = SS._resolve(SS._type_die(ch))
                if ptr == 1 and b is not None and b.tag in ("DW_TAG_structure_type", "DW_TAG_class_type"):
                    res[(int(low), i)] = (SS.struct_identity(b), leaves(b), SS._attr(b, "DW_AT_byte_size"))
    return res


def one(job):
    proj, b, o, arm = job
    binary = RES / o / proj / "stripped" / b
    gt = gt_params(RES / o / proj / "compiled" / b)
    payload = SS.run_json(binary, ARMS[arm], 3600)
    header = SS.run_header(binary, ARMS[arm], 3600)
    layouts = SS.header_layouts(header)
    filler = set()
    for name, body in SS.STRUCT_RE.findall(header):
        for line in body.split("\n"):
            m = SS.MEMBER_RE.match(line)
            if m and m.group("count"):
                filler.add((name, m.group("name")))
    c = collections.Counter()
    ex = collections.defaultdict(list)
    rows = []
    for fn in payload.get("functions") or []:
        low = int(fn.get("address") or -1)
        for v in fn.get("variables") or []:
            idx = v.get("arg_index")
            if idx is None or (low, idx) not in gt:
                continue
            rec = SS.parse_type(v.get("type"))
            if rec["kind"] != "struct" or rec["base"] not in layouts:
                continue
            name, lv, gsize = gt[(low, idx)]
            rows.append({"bin": f"{b} {o}", "fn": fn["name"], "idx": idx, "gt": name, "rec": rec["base"], "claims": []})
            top = {(a, s) for a, s, k, d in lv if d == 0}
            deep = {(a, s) for a, s, k, d in lv if k == "leaf"}
            elems = {(a, s) for a, s, k, d in lv if k == "element"}
            for f in layouts[rec["base"]]["fields"]:
                if (rec["base"], f["name"]) in filler:
                    continue
                key = (f["offset"], f["size"])
                c["predicted"] += 1
                if key in top:
                    c["tp_top"] += 1
                    c["tp_deep"] += 1
                    rows[-1]["claims"].append((f["offset"], f["size"], "top"))
                    continue
                if key in deep:
                    c["tp_deep"] += 1
                    cls = "member of an embedded record, array element or union"
                elif key in elems:
                    cls = "element of an array member"
                else:
                    cover = [(a, s, k) for a, s, k, d in lv if s and a <= f["offset"] < a + s]
                    kinds = {k for a, s, k in cover}
                    if not cover:
                        cls = "past the record's end" if gsize and f["offset"] >= gsize else "padding"
                    elif "array" in kinds:
                        cls = "inside an array member"
                    elif "union" in kinds:
                        cls = "inside a union member"
                    elif any(a == f["offset"] for a, s, k in cover):
                        cls = "member at this offset, other width"
                    else:
                        cls = "inside a member"
                c[cls] += 1
                rows[-1]["claims"].append((f["offset"], f["size"], cls))
                if len(ex[cls]) < 3:
                    ex[cls].append(f"{b} {o} {fn['name']} a{idx} {name} +{f['offset']:#x}/{f['size']}")
    return arm, f"{b} {o}", c, dict(ex), rows


if __name__ == "__main__":
    jobs = [(p, b, o, a) for (p, b, o) in BINS for a in ARMS]
    tot = collections.defaultdict(collections.Counter)
    exs = collections.defaultdict(lambda: collections.defaultdict(list))
    with ProcessPoolExecutor(int(os.environ.get("W", "8"))) as ex:
        allrows = collections.defaultdict(list)
        for arm, name, c, e, rows in ex.map(one, jobs):
            allrows[arm] += rows
            tot[arm].update(c)
            for k, v in e.items():
                exs[arm][k] += v
    for arm, c in tot.items():
        p = c["predicted"]
        print(f"{arm}: predicted {p}  top-level TP {c['tp_top']} ({c['tp_top']/p:.4f})  "
              f"TP at depth {c['tp_deep']} ({c['tp_deep']/p:.4f})")
        for k, n in c.most_common():
            if k not in ("predicted", "tp_top", "tp_deep"):
                print(f"   {n:5}  {k}    e.g. {'; '.join(exs[arm][k][:2])}")
    if os.environ.get("OUT"):
        json.dump({"totals": {a: dict(c) for a, c in tot.items()}, "rows": allrows}, open(os.environ["OUT"], "w"), indent=1)
