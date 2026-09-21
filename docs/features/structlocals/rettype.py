"""Functions whose return type becomes struct_N * in armB, with the DWARF return type."""
import os, re, sys, subprocess, pathlib, collections
from pathlib import Path
sys.path.insert(0, os.environ.get("PINDB_DIR", str(Path(__file__).resolve().parents[2] / "decbench/typecampaign/final-c")))
import pindb; pindb.pin()
sys.path.insert(0, str(Path(__file__).resolve().parents[3]))
from scripts.decbench import structscore as SS
R = pathlib.Path(os.environ.get("KUNA_DECBENCH_RESULTS", "/home/mahaloz/github/decbench/results/full_run_address_2026-09-11"))
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from canon import split
def gt_rets(twin):
    from decbench.utils import binfmt
    info = binfmt.dwarf_info(twin); out = {}
    if info is None: return out
    for cu in info.iter_CUs():
        for die in cu.get_top_DIE().iter_children():
            if die.tag != "DW_TAG_subprogram": continue
            low = SS._attr(die, "DW_AT_low_pc")
            if low is None: continue
            base, ptr = SS._resolve(SS._type_die(die))
            out[int(low)] = ((SS._name(die) or "?"), ("void" if base is None else (SS._name(base) or base.tag)) + "*" * ptr, base is not None and base.tag == "DW_TAG_structure_type" and ptr == 1)
    return out
d, A, B = sys.argv[1:4]
tot = collections.Counter()
for f in sorted(pathlib.Path(d).glob(f"*.{A}.c")):
    t = f.name[:-len(f".{A}.c")]
    o = t.rsplit("-", 1)[1]; p, b = t.rsplit("-", 1)[0].split("-", 1)
    twin = R / o / p / "compiled" / b
    rets = gt_rets(twin) if twin.exists() else {}
    a, bb = split(f), split(pathlib.Path(d) / f"{t}.{B}.c")
    for k in sorted(set(a) & set(bb)):
        sa = a[k][0] if a[k] else ""; sb = bb[k][0] if bb[k] else ""
        ra = sa.split("(")[0].rsplit(" ", 1)[0]; rb = sb.split("(")[0].rsplit(" ", 1)[0]
        if ra == rb or "struct_" not in rb or "struct_" in ra: continue
        name, gt, gts = rets.get(int(k, 16), ("?", "?", None))
        cls = "gt_structptr" if gts else ("gt_unknown" if gts is None else "gt_other")
        tot[cls] += 1
        print(f"{t:28s} {k:9s} {name:28s} {ra:22s} -> {rb:14s} GT {gt}")
print(dict(tot))
