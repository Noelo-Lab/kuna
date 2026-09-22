"""o2census analysis over .scratch/sweep-o2/rows.json (provenance-tagged decisions)."""
import collections, json, re, sys
from pathlib import Path
from elftools.elf.elffile import ELFFile

RES = Path("/home/mahaloz/github/decbench/results/full_run_address_2026-09-11")
ROWS = Path(sys.argv[1] if len(sys.argv) > 1 else "/home/mahaloz/kwt/o2census/.scratch/sweep-o2/rows.json")
OUT = Path("/home/mahaloz/kwt/o2census/.scratch/census.json")
R = json.loads(ROWS.read_text())
C = collections.Counter
_symcache = {}


def syms(project, opt, stem):
    key = (project, opt, stem)
    if key in _symcache:
        return _symcache[key]
    m = collections.defaultdict(list)
    d = RES / opt / project / "compiled"
    cands = [p for p in d.iterdir() if p.is_file() and not p.is_symlink() and (p.name == stem or p.stem == stem)
             and open(p, "rb").read(4) == b"\x7fELF"]
    cands.sort(key=lambda p: p.name != stem)
    for p in cands[:1]:
        with open(p, "rb") as f:
            e = ELFFile(f)
            st = e.get_section_by_name(".symtab")
            if st:
                for s in st.iter_symbols():
                    if s["st_info"]["type"] == "STT_FUNC" and s["st_value"]:
                        m[s["st_value"]].append(s.name)
    _symcache[key] = m
    return m


def norm_sp(t):
    if t is None:
        return None
    return re.sub(r"struct_\d+", "struct_N", t)


def storage(d):
    return "arg" if d["gt_is_arg"] else ("stack" if d["gt_has_off"] else "reg")


def regsub(d):
    p, lk = d.get("prov") or "", d.get("lockind") or ""
    if p.startswith("inl"):
        return "inlined-callee " + ("param" if p == "inl_param" else "local")
    if "addr" in lk:
        return "static-local(DW_OP_addr)"
    if lk in ("const", "value", "const+value") or (lk and all(x in ("const", "value") for x in lk.split("+"))):
        return "constant/value-only"
    return "own register local"


def main():
    rep = {}
    per_opt = {}
    onesh = collections.defaultdict(list)
    nonTP = collections.defaultdict(C)
    spell = collections.defaultdict(C)
    unreach = collections.defaultdict(C)
    fclass = collections.defaultdict(C)
    deficit = collections.defaultdict(C)
    reach_rows = collections.defaultdict(list)
    examples = collections.defaultdict(list)
    for key, v in R.items():
        project, opt, stem = key.split("::")
        if "decisions" not in v:
            continue
        vals = v["base"]["values"]
        funcs = {f["fn"]: f for f in v.get("funcs", [])}
        sm = syms(project, opt, stem)
        byfn = collections.defaultdict(list)
        for d in v["decisions"]["base"]:
            byfn[d["fn"]].append(d)
        po = per_opt.setdefault(opt, C())
        for fn, ds in byfn.items():
            if fn not in vals:
                continue
            fi = funcs.get(fn, {})
            kaddr = fi.get("kaddr")
            symnames = sm.get(kaddr, []) if kaddr is not None else []
            clone = any(re.search(r"\.(isra|constprop|part|cold|lto_priv)", s) for s in symnames)
            inst_mis = fi.get("gt_lowpc") is not None and kaddr is not None and fi.get("gt_lowpc") != kaddr
            kv = fi.get("kvars") or []
            karity = 1 + max([x[4] for x in kv if x[4] is not None], default=-1)
            kstack = [(x[3], x[2] or 0, x[1]) for x in kv if x[3] is not None]
            k = fi.get("k") or 0
            perfect = vals[fn] == 1.0
            po["fn"] += 1
            po["perfect"] += perfect
            bl = []
            fun_unr = set()
            for d in ds:
                s = storage(d)
                ok = d["ok"]
                po[f"gt_{s}"] += 1
                po[f"tp_{s}"] += ok is True
                restrict = bool(d.get("restrict")) or ((d.get("cls") or "").startswith("ptr") and d.get("gt_type") == "void")
                # unreachable taxonomy (per GT var, independent of outcome)
                if s == "reg":
                    u = "reg-only: " + regsub(d)
                elif restrict:
                    u = "restrict-void GT (%s)" % s
                elif s == "arg" and (inst_mis or clone):
                    u = "arg of clone/split instance"
                else:
                    u = "reachable-" + s
                unreach[opt][u] += 1
                unreach[opt]["tp|" + u] += ok is True
                if not u.startswith("reachable"):
                    fun_unr.add(u.split(":")[0].split(" (")[0])
                if ok is True:
                    continue
                outcome = "wrong" if ok is False else "miss"
                if ok is None:
                    if s == "arg":
                        ai = d.get("arg_index")
                        why = ("no kuna arg (arity %d<=idx %d)" % (karity, ai)) if karity <= (ai or 0) else "arg slot claimed elsewhere"
                    elif s == "stack":
                        offs = d.get("gt_off") or []
                        ov = [t for (ko, ksz, t) in kstack for g in offs if ko + k <= g < ko + k + max(ksz, 1)
                              or g <= ko + k < g + max(d.get("gt_size") or 1, 1)]
                        why = ("overlapping kuna slot %s" % norm_sp(ov[0])) if ov else ("no kuna stack var" if not kstack else "no kuna slot at offset")
                    else:
                        why = "reg-only (never exported)"
                    sp = why
                else:
                    sp = norm_sp(d.get("kuna_type"))
                if not perfect:
                    nonTP[opt][(s, outcome, d.get("cls"))] += 1
                    spell[opt][(s, d.get("cls"), sp)] += 1
                bl.append((s, outcome, d.get("cls"), sp, u, d))
            if perfect:
                fclass[opt]["perfect"] += 1
                continue
            # function class
            if any(x.startswith("reg-only") for x in fun_unr):
                allinl = all(regsub(d).startswith("inlined") for d in ds if storage(d) == "reg")
                fc = "has reg-only GT (all inlined-callee)" if allinl else "has own reg-only GT"
            elif any(x.startswith("restrict") for x in fun_unr):
                fc = "restrict-void GT (no reg-only)"
            elif any(x.startswith("arg of clone") for x in fun_unr):
                fc = "clone/split instance (no reg-only)"
            else:
                fc = "reachable"
            fclass[opt][fc] += 1
            deficit[opt][(fc, min(len(bl), 5))] += 1
            if len(bl) == 1:
                s, outcome, cls, sp, u, d = bl[0]
                onesh[opt].append({"slice": key, "fn": fn, "storage": s, "outcome": outcome, "cls": cls,
                                   "spell": sp, "u": u, "gt_name": d.get("gt_name"), "gt_type": d.get("gt_type"),
                                   "nvars": d.get("gt_vars"), "kaddr": kaddr, "prov": d.get("prov"),
                                   "fc": fc})
            if fc == "reachable":
                for (s, outcome, cls, sp, u, d) in bl:
                    reach_rows[opt].append((s, outcome, cls, sp))
                    examples[(s, outcome, cls, sp)].append((len(bl), d.get("gt_vars"), key, fn, kaddr,
                                                            d.get("gt_name"), d.get("gt_type")))
    out = {"per_opt": {o: dict(c) for o, c in per_opt.items()},
           "fclass": {o: dict(c) for o, c in fclass.items()},
           "deficit": {o: {"%s|%d" % kk: n for kk, n in c.items()} for o, c in deficit.items()},
           "unreach": {o: dict(c) for o, c in unreach.items()},
           "nonTP": {o: {"|".join(map(str, kk)): n for kk, n in c.most_common()} for o, c in nonTP.items()},
           "spell": {o: {"|".join(map(str, kk)): n for kk, n in c.most_common(200)} for o, c in spell.items()},
           "onesh": onesh,
           "reach_shapes": {o: {"|".join(map(str, kk)): n for kk, n in C(r).most_common(60)} for o, r in reach_rows.items()},
           "examples": {"|".join(map(str, kk)): sorted(v)[:8] for kk, v in examples.items()}}
    OUT.write_text(json.dumps(out, indent=1, default=str))
    print("wrote", OUT)


if __name__ == "__main__":
    main()
