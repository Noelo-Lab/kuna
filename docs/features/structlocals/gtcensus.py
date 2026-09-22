"""DWARF census: struct-pointer params / locals / returns per build, by location class."""
import os, sys, collections, json
from pathlib import Path
sys.path.insert(0, os.environ.get("PINDB_DIR", str(Path(__file__).resolve().parents[2] / "decbench/typecampaign/final-c")))
import pindb; pindb.pin()
sys.path.insert(0, str(Path(__file__).resolve().parents[3]))
from scripts.decbench import structscore as SS
RES = Path(os.environ.get("KUNA_DECBENCH_RESULTS", "/home/mahaloz/github/decbench/results/full_run_address_2026-09-11"))

def loc_class(die):
    a = die.attributes.get("DW_AT_location")
    if a is None:
        return "none"
    if a.form == "exprloc" or isinstance(a.value, list):
        v = a.value
        if v and v[0] == 0x91: return "fbreg"
        if v and 0x50 <= v[0] <= 0x6f: return "reg"
        if v and v[0] == 0x03: return "addr"
        return "expr"
    return "loclist"

def walk(die, out, depth=0):
    for c in die.iter_children():
        if c.tag == "DW_TAG_variable":
            out.append(c)
        elif c.tag == "DW_TAG_lexical_block":
            walk(c, out, depth + 1)

def census(proj, b, o):
    twin = RES / o / proj / "compiled" / b
    from decbench.utils import binfmt
    info = binfmt.dwarf_info(twin)
    cnt = collections.Counter(); names = collections.Counter()
    for cu in info.iter_CUs():
        for die in cu.get_top_DIE().iter_children():
            if die.tag != "DW_TAG_subprogram" or SS._attr(die, "DW_AT_low_pc") is None:
                continue
            cnt["functions"] += 1
            rt, rp = SS._resolve(SS._type_die(die))
            if rp == 1 and rt is not None and rt.tag == "DW_TAG_structure_type":
                cnt["ret_structptr_fn"] += 1
            for c in die.iter_children():
                if c.tag == "DW_TAG_formal_parameter":
                    base, ptr = SS._resolve(SS._type_die(c))
                    if ptr == 1 and base is not None and base.tag == "DW_TAG_structure_type":
                        cnt["param_sp"] += 1
            vs = []; walk(die, vs)
            for v in vs:
                cnt["locals"] += 1
                base, ptr = SS._resolve(SS._type_die(v))
                lc = loc_class(v)
                cnt["locals_" + lc] += 1
                if ptr == 1 and base is not None and base.tag == "DW_TAG_structure_type":
                    cnt["local_sp"] += 1
                    cnt["local_sp_" + lc] += 1
                    names[SS.struct_identity(base)] += 1
                elif base is not None and base.tag == "DW_TAG_structure_type" and ptr == 0:
                    cnt["local_struct_value_" + lc] += 1
    return dict(cnt), names.most_common(8)

if __name__ == "__main__":
    bins = [tuple(x.split(":")) for x in sys.argv[1].split(",")]
    tot = collections.Counter()
    for p, b, o in bins:
        c, n = census(p, b, o)
        tot.update(c)
        print(f"{b} {o}: " + " ".join(f"{k}={v}" for k, v in sorted(c.items())))
        print("   top:", n)
    print("TOTAL " + " ".join(f"{k}={v}" for k, v in sorted(tot.items())))
