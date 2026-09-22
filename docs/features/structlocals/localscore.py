"""Layout score for records synthesized over CALL-RETURNED locals, joined to DWARF at the call site.

For every local declared `struct_N *` whose defining line is a call, the call instruction is found
through `line_mappings`, and the ground-truth variable that receives the returned value is read from
the unstripped twin's DWARF at the call's return address: at -O0 the `mov %rax,-K(%rbp)` that follows
the call names the stack slot (DW_OP_fbreg K-16); at -O2 a location-list entry that starts at the
return address in `rax` (DW_OP_reg0). Failing both, a callee whose DWARF return type is a struct
pointer answers. Fields are compared by (offset, size), filler arrays excluded.
"""
import os, re, sys, json, collections, subprocess
from pathlib import Path
from concurrent.futures import ProcessPoolExecutor
sys.path.insert(0, os.environ.get("PINDB_DIR", str(Path(__file__).resolve().parents[2] / "decbench/typecampaign/final-c")))
import pindb; pindb.pin()
sys.path.insert(0, str(Path(__file__).resolve().parents[3]))
from scripts.decbench import structscore as SS
RES = Path(os.environ.get("KUNA_DECBENCH_RESULTS", "/home/mahaloz/github/decbench/results/full_run_address_2026-09-11"))

def insns(binary):
    out = {}
    p = subprocess.run(["objdump", "-d", "--no-show-raw-insn", str(binary)], capture_output=True, text=True)
    prev = None
    for line in p.stdout.splitlines():
        m = re.match(r"^\s*([0-9a-f]+):\s+(\S.*)$", line)
        if not m: continue
        a = int(m.group(1), 16); out[a] = [m.group(2), None]
        if prev is not None: out[prev][1] = a
        prev = a
    return out

def walk_vars(die, out):
    for c in die.iter_children():
        if c.tag in ("DW_TAG_variable", "DW_TAG_formal_parameter"): out.append(c)
        if c.tag in ("DW_TAG_lexical_block", "DW_TAG_inlined_subroutine"): walk_vars(c, out)

def dwarf_index(twin):
    from decbench.utils import binfmt
    from elftools.dwarf.locationlists import LocationParser, LocationEntry, BaseAddressEntry
    info = binfmt.dwarf_info(twin)
    lp = LocationParser(info.location_lists())
    subs = []; rets = {}; complete = {}
    for cu in info.iter_CUs():
        for d in cu.iter_DIEs():
            if d.tag == "DW_TAG_structure_type" and SS._name(d) and not SS._attr(d, "DW_AT_declaration"):
                complete.setdefault(SS._name(d), d)
    for cu in info.iter_CUs():
        top = cu.get_top_DIE(); cubase = SS._attr(top, "DW_AT_low_pc") or 0
        for die in top.iter_children():
            if die.tag != "DW_TAG_subprogram": continue
            low = SS._attr(die, "DW_AT_low_pc"); nm = SS._name(die)
            base, ptr = SS._resolve(SS._type_die(die))
            if nm and ptr == 1 and base is not None and base.tag == "DW_TAG_structure_type":
                if SS._attr(base, "DW_AT_declaration") and complete.get(SS._name(base)) is not None:
                    base = complete[SS._name(base)]
                rets.setdefault(nm, SS.struct_layout(base))
            if low is None: continue
            hi = SS._attr(die, "DW_AT_high_pc")
            hi = low + hi if isinstance(hi, int) and hi < low else hi
            vs = []; walk_vars(die, vs); rows = []
            for v in vs:
                a = v.attributes.get("DW_AT_location")
                org = v
                if "DW_AT_abstract_origin" in v.attributes:
                    try: org = v.get_DIE_from_attribute("DW_AT_abstract_origin")
                    except Exception: org = v
                vb, vp = SS._resolve(SS._type_die(org))
                if vb is not None and vb.tag == "DW_TAG_structure_type" and SS._attr(vb, "DW_AT_declaration"):
                    full = complete.get(SS._name(vb))
                    vb = full if full is not None else vb
                lay = SS.struct_layout(vb) if vp == 1 and vb is not None and vb.tag == "DW_TAG_structure_type" else None
                tname = (SS._name(vb) if vb is not None else "void") or (vb.tag if vb is not None else "void")
                rec = {"name": SS._name(org), "type": tname + "*" * vp, "layout": lay, "fbreg": None, "entries": []}
                if a is None: rows.append(rec); continue
                if a.form in ("DW_FORM_exprloc", "exprloc", "DW_FORM_block1") or isinstance(a.value, list):
                    e = a.value
                    if e and e[0] == 0x91:
                        val, sh = 0, 0
                        for b in e[1:]:
                            val |= (b & 0x7f) << sh; sh += 7
                            if not b & 0x80:
                                if b & 0x40: val -= 1 << sh
                                break
                        rec["fbreg"] = val
                else:
                    try:
                        ll = lp.parse_from_attribute(a, cu["version"], v)
                    except Exception:
                        ll = []
                    b0 = cubase
                    for ent in ll if isinstance(ll, list) else []:
                        if isinstance(ent, BaseAddressEntry): b0 = ent.base_address; continue
                        if isinstance(ent, LocationEntry):
                            beg = ent.begin_offset + (0 if getattr(ent, "is_absolute", False) else b0)
                            rec["entries"].append((beg, list(ent.loc_expr)))
                rows.append(rec)
            subs.append((low, hi or low, nm, rows))
    return subs, rets

DECL = re.compile(r"^\s*(struct_\d+)\s*\*\s*(v\d+)\s*;")
def one(job):
    proj, b, o, arm = job
    stripped = RES / o / proj / "stripped" / b; twin = RES / o / proj / "compiled" / b
    payload = SS.run_json(stripped, [["structsynth", arm]], 3600)
    header = SS.run_header(stripped, [["structsynth", arm]], 3600)
    layouts = SS.header_layouts(header)
    arrays = {}
    for name, body in SS.STRUCT_RE.findall(header):
        for line in body.split("\n"):
            m = SS.MEMBER_RE.match(line)
            if m: arrays[(name, m.group("name"))] = bool(m.group("count"))
    ins = insns(twin); subs, rets = dwarf_index(twin)
    names = {}
    for line in subprocess.run(["objdump", "-d", str(twin)], capture_output=True, text=True).stdout.splitlines():
        m = re.match(r"^([0-9a-f]+) <([^>]+)>:", line)
        if m: names[int(m.group(1), 16)] = m.group(2).split("@")[0]
    rows = []
    for fn in payload.get("functions") or []:
        code = (fn.get("code") or "").split("\n")
        decls = {m.group(2): m.group(1) for m in (DECL.match(l) for l in code) if m}
        if not decls: continue
        lm = {e["line_number"]: e["addresses"] for e in fn.get("line_mappings") or []}
        for ln, line in enumerate(code, 1):
            m = re.match(r"^\s*(v\d+) = (?:\(struct_\d+ \*\))?\s*([A-Za-z_]\w*)\(", line)
            if not m or m.group(1) not in decls: continue
            calls = [a for a in lm.get(ln, []) if a in ins and ins[a][0].startswith("call")]
            if len(calls) != 1: continue
            ca = calls[0]; ret = ins[ca][1]
            sub = next((s for s in subs if s[0] <= ca < s[1]), None)
            gt = None; how = None
            if sub:
                nxt = ins.get(ret, [""])[0]
                mm = re.match(r"mov\s+%rax,(-?0x[0-9a-f]+)\(%rbp\)", nxt)
                if mm:
                    fb = int(mm.group(1), 16) - 16
                    gt = next((r for r in sub[3] if r["fbreg"] == fb), None); how = "fbreg" if gt else None
                if gt is None:
                    gt = next((r for r in sub[3] if any(beg == ret and ex[:1] == [0x50] for beg, ex in r["entries"])), None)
                    how = "loclist" if gt else None
            tgt = re.search(r"<([^>+]+)", ins[ca][0]); callee = tgt.group(1).split("@")[0] if tgt else None
            if gt is None and callee in rets:
                gt = {"name": "<ret>", "type": "ret", "layout": rets[callee]}; how = "callee-ret"
            pred = layouts.get(decls[m.group(1)])
            pf = {(f["offset"], f["size"]) for f in (pred or {}).get("fields", []) if not arrays.get((decls[m.group(1)], f["name"]))}
            row = {"bin": f"{b} {o}", "fn": fn.get("name"), "call": hex(ca), "callee": callee, "how": how,
                   "gt_var": gt and gt["name"], "gt_type": gt and gt["type"], "pred": sorted(pf)}
            if gt and gt["layout"]:
                gf = {(f["offset"], f["size"]) for f in gt["layout"]["fields"] if f["kind"] != "pad"}
                row["tp"] = len(pf & gf); row["gt_fields"] = len(gf); row["cls"] = "struct"
            elif gt:
                row["cls"] = "nonstruct"
            else:
                row["cls"] = "unjoined"
            rows.append(row)
    fns = SS.dwarf_functions(twin)
    raw = SS.header_layouts
    def drop_filler(h):
        lay = raw(h)
        return {n: {**l, "fields": [f for f in l["fields"] if not arrays.get((n, f["name"]))]} for n, l in lay.items()}
    SS.header_layouts = drop_filler
    fo = SS.measure_layout(payload, header, fns)
    SS.header_layouts = raw
    fr = SS.measure_layout(payload, header, fns)
    rows.append({"param_layout": fo["layout"], "param_nesting": fr["nesting"], "structs": fr["structs_in_header"],
                 "typed_params": fr["params_kuna_typed_as_a_struct"]})
    return (f"{b} {o}", arm, rows)

if __name__ == "__main__":
    bins = [tuple(x.split(":")) for x in os.environ["BINS"].split(",")]
    arms = os.environ.get("ARMS", "locals").split(",")
    jobs = [(p, b, o, a) for (p, b, o) in bins for a in arms]
    with ProcessPoolExecutor(int(os.environ.get("W", "8"))) as ex:
        res = list(ex.map(one, jobs))
    for arm in arms:
        c = collections.Counter()
        for name, a, rows in res:
            if a != arm: continue
            for r in rows:
                if "param_layout" in r:
                    for k in ("tp", "predicted", "ground_truth"):
                        c["P_" + k] += r["param_layout"][k]; c["N_" + k] += r["param_nesting"][k]
                    c["structs"] += r["structs"]; c["typed_params"] += r["typed_params"]
                    continue
                c["records"] += 1; c[r["cls"]] += 1
                if r["cls"] == "struct":
                    c["tp"] += r["tp"]; c["pred_fields"] += len(r["pred"]); c["gt_fields"] += r["gt_fields"]
                elif r["cls"] == "nonstruct":
                    c["fp_fields"] += len(r["pred"])
        p = c["tp"] / (c["pred_fields"] + c["fp_fields"]) if c["pred_fields"] + c["fp_fields"] else 0
        rc = c["tp"] / c["gt_fields"] if c["gt_fields"] else 0
        print(f"ARM {arm}: " + " ".join(f"{k}={v}" for k, v in sorted(c.items())) + f" field_precision={p:.4f} recall_of_joined={rc:.4f}")
        pp = c["P_tp"] / c["P_predicted"] if c["P_predicted"] else 0; pr = c["P_tp"] / c["P_ground_truth"] if c["P_ground_truth"] else 0
        npp = c["N_tp"] / c["N_predicted"] if c["N_predicted"] else 0; nr = c["N_tp"] / c["N_ground_truth"] if c["N_ground_truth"] else 0
        print(f"ARM {arm} PARAM layout fields-only P={pp:.4f} R={pr:.4f} | nesting P={npp:.4f} R={nr:.4f} F1={(2*npp*nr/(npp+nr) if npp+nr else 0):.4f}")
    for name, a, rows in res:
        for r in rows:
            if "cls" not in r: continue
            if r["cls"] != "struct" or r.get("tp", 0) < len(r["pred"]):
                print(a, json.dumps(r))
    if os.environ.get("OUT"): json.dump(res, open(os.environ["OUT"], "w"), indent=1)
