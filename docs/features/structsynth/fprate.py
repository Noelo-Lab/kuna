#!/usr/bin/env python3
"""Classify every parameter structsynth retypes by what DWARF says it is.

    KUNA_BIN=<release kuna> ~/.virtualenvs/decbench/bin/python \
        docs/features/structsynth/fprate.py [census|arm]

Two readings of the same 8-binary census:

* the class table -- is the ground truth a struct pointer, some other pointer,
  or a non-pointer (the only shape that is a plain false positive);
* the pointee spelling histogram -- what the ground truth actually *is*, which
  is the reading that says whether a named-type pass would have owned the
  parameter (a `FILE *` taken by this pass is a precedence question, not a
  false positive in the class table).

Needs the decbench venv (pyelftools) and the pinned decbench results tree.
"""
import json, os, re, sys
from pathlib import Path
sys.path.insert(0, str(Path(__file__).resolve().parents[3]))
from scripts.decbench import structscore as SS

RES = Path("/home/mahaloz/github/decbench/results/full_run_address_2026-09-11")
# (project, binary name, optimization) -- the census set is x86-64 coreutils at
# both optimizations; `arm` is the 32-bit arm of the same census.
SETS = {
    "census": [("coreutils", b, o) for b in ("fmt", "ls", "sort", "du")
               for o in ("O0", "O2")],
    "arm": [("chibios", "ch.elf", "O2"), ("freertos", "RTOSDemo.out", "O2")],
}
BINS = SETS[sys.argv[1] if len(sys.argv) > 1 else "census"]
SYNTH = re.compile(r"struct_\d+")
if os.environ.get("KUNA_BIN"):
    SS.KUNA = os.environ["KUNA_BIN"]


def param_pointees(twin: Path) -> dict:
    """{low_pc: {param index: DWARF spelling of the pointee}}."""
    from decbench.utils import binfmt
    out: dict[int, dict] = {}
    info = binfmt.dwarf_info(twin)
    if info is None:
        return out
    for cu in info.iter_CUs():
        for die in cu.get_top_DIE().iter_children():
            if die.tag != "DW_TAG_subprogram":
                continue
            low = SS._attr(die, "DW_AT_low_pc")
            if low is None:
                continue
            row = {}
            for index, child in enumerate(
                    c for c in die.iter_children() if c.tag == "DW_TAG_formal_parameter"):
                base, ptr = SS._resolve(SS._type_die(child))
                if ptr == 0:
                    row[index] = "<not a pointer: %s>" % (SS._name(base) or base.tag
                                                          if base is not None else "?")
                    continue
                if base is None:
                    row[index] = "void" + "*" * (ptr - 1)
                    continue
                if base.tag in ("DW_TAG_structure_type", "DW_TAG_class_type",
                                "DW_TAG_union_type"):
                    name = SS.struct_identity(base)
                else:
                    name = SS._name(base) or base.tag
                row[index] = name + "*" * (ptr - 1)
            out[int(low)] = row
    return out


tot = {}
pointees: dict[str, int] = {}
witnesses: dict[str, list] = {}
rows = []
for proj, b, o in BINS:
    binary = RES / o / proj / "stripped" / b
    twin = RES / o / proj / "compiled" / b
    fns = SS.dwarf_functions(twin)
    spell = param_pointees(twin)
    payload = SS.run_json(binary, [["structsynth", "param"]], 900)
    by_addr = {int(f.get("address") or -1): f for f in payload.get("functions") or []}
    c = {"struct*": 0, "void*/other ptr": 0, "no DWARF type": 0, "not in DWARF": 0, "non-pointer": 0}
    names = {"void*/other ptr": [], "non-pointer": []}
    for low, fn in by_addr.items():
        for v in fn.get("variables") or []:
            if v.get("arg_index") is None:
                continue
            rec = SS.parse_type(v.get("type"))
            # Only the names this pass mints. `libctypes` is default-on since
            # #658 and types a `FILE *` parameter with a named composite too,
            # so "kind == struct" alone counts 289 parameters this pass never
            # touched.
            if rec["kind"] != "struct" or not SYNTH.fullmatch(rec["base"]):
                continue
            info = fns.get(low)
            if not info:
                c["not in DWARF"] += 1; continue
            p = next((p for p in info["params"] if p["index"] == v["arg_index"]), None)
            if p is None:
                c["not in DWARF"] += 1; continue
            sp = spell.get(low, {}).get(v["arg_index"], "<no DWARF>")
            pointees[sp] = pointees.get(sp, 0) + 1
            witnesses.setdefault(sp, []).append(
                f"{b} {o} {info['name']} @{low:#x} arg{v['arg_index']}")
            if p["struct"]:
                c["struct*"] += 1
            elif p["ptr"] >= 1:
                c["void*/other ptr"] += 1
                names["void*/other ptr"].append(info["name"])
            elif p["ptr"] == 0 and p.get("name") is None:
                c["no DWARF type"] += 1
            else:
                c["non-pointer"] += 1
                names["non-pointer"].append(info["name"])
    rows.append((f"{b} {o}", dict(c)))
    for k, v in c.items(): tot[k] = tot.get(k, 0) + v
    print(f"{b} {o}: {c}")
    if names["void*/other ptr"]:
        print("    void*/other-ptr params:", sorted(set(names['void*/other ptr']))[:12])
    if names["non-pointer"]:
        print("    non-pointer params:", sorted(set(names['non-pointer']))[:12])
print("TOTAL", tot, "sum", sum(tot.values()))
print("\nDWARF pointee spellings of every retyped parameter:")
for sp, n in sorted(pointees.items(), key=lambda kv: -kv[1]):
    print(f"  {n:4d}  {sp}")
    if n <= 12:
        for w in witnesses[sp][:12]:
            print(f"          {w}")
json.dump({"rows": rows, "total": tot, "pointees": pointees,
           "witnesses": {k: v[:12] for k, v in witnesses.items()}},
          open(os.environ.get("FPRATE_OUT", "fprate.json"), "w"), indent=1)
