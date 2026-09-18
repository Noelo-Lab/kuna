#!/usr/bin/env python3
"""Census of the layouts structsynth synthesizes, split by shape and joined with DWARF.

    KUNA_BIN=<census build> ~/.virtualenvs/decbench/bin/python \
        docs/features/structsynth/arrays_census.py [--out census.json]

The census build is the release build with `arrays_census.diff` applied: under
`KUNA_STRUCTSYNTH_CENSUS=1` it writes one stderr line per accepted base --
function entry, `struct_N`, size, the input register, and every claimed field as
`offset:width:metatype:pointee` (filler excluded). Each line is joined to the
`--json` argument that carries the name and to the DWARF twin's parameter at
that index.

A layout is HOMOGENEOUS-CONTIGUOUS when every claimed field has the same
metatype and width and the offsets are exactly 0, w, 2w, ... with no hole.
"""
import argparse, json, os, re, subprocess, sys
from concurrent.futures import ThreadPoolExecutor
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[3]))
from scripts.decbench import structscore as SS

RES = Path("/home/mahaloz/github/decbench/results/full_run_address_2026-09-11")
BINS = [(p, b, o) for o in ("O0", "O2") for p, b in (
    ("coreutils", "fmt"), ("coreutils", "ls"), ("coreutils", "sort"), ("coreutils", "du"),
    ("coreutils", "factor"), ("coreutils", "shred"), ("tar", "tar"), ("findutils", "find"))]
KUNA = os.environ.get("KUNA_BIN", "kuna")
# SysV x86-64 integer argument registers, by their register-space offset.
SYSV = {0x38: 0, 0x30: 1, 0x10: 2, 0x8: 3, 0x80: 4, 0x88: 5}
INTEGER = {12, 13, 14, 15}         # bool, uint, int, unknown
POINTER = {8, 9}                   # ptrrel, ptr
MT = {9: "ptr", 10: "float", 12: "bool", 13: "uint", 14: "int", 15: "unknown", 8: "ptrrel"}


def run(binary: Path, options, census: bool):
    env = dict(os.environ)
    if census:
        env["KUNA_STRUCTSYNTH_CENSUS"] = "1"
    cmd = [KUNA, "decompile-all", str(binary), "--json", "--max-fn-seconds", "120"]
    for k, v in options:
        cmd += ["--option", k, v]
    p = subprocess.run(cmd, capture_output=True, text=True, timeout=3600, env=env)
    return json.loads(p.stdout), p.stderr


def parse_fields(spec: str):
    out = []
    for part in filter(None, spec.split(";")):
        off, width, mt, pointee = part.split(":", 3)
        out.append({"offset": int(off), "width": int(width), "mt": int(mt), "pointee": pointee})
    return out


def shape(fields) -> str:
    """homogeneous-contiguous / other, plus the element class when homogeneous."""
    if not fields:
        return "empty"
    w, mt = fields[0]["width"], fields[0]["mt"]
    same = all(f["width"] == w and f["mt"] == mt for f in fields)
    contiguous = [f["offset"] for f in fields] == [i * w for i in range(len(fields))]
    if same and contiguous:
        return "homog-" + ("int" if mt in INTEGER else "ptr" if mt in POINTER else MT.get(mt, str(mt)))
    loose = all(f["width"] == w for f in fields) and contiguous and (
        all(f["mt"] in INTEGER for f in fields) or all(f["mt"] in POINTER for f in fields))
    return "loose-" + ("int" if fields[0]["mt"] in INTEGER else "ptr") if loose else "other"


def dwarf_params(twin: Path) -> dict:
    """{low_pc: [(kind, spelling)]} per formal parameter."""
    from decbench.utils import binfmt
    out = {}
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
            row = []
            for child in (c for c in die.iter_children() if c.tag == "DW_TAG_formal_parameter"):
                tdie = SS._type_die(child)
                base, ptr = SS._resolve(tdie)
                name = (SS._name(base) if base is not None else None) or (base.tag if base is not None else "void")
                if tdie is None:
                    row.append(("no DWARF type", "?"))
                elif ptr == 0:
                    row.append(("non-pointer", name))
                elif ptr >= 2:
                    row.append(("pointer to pointer", name + "*" * (ptr - 1)))
                elif base is None:
                    row.append(("void *", "void"))
                elif base.tag in ("DW_TAG_structure_type", "DW_TAG_class_type", "DW_TAG_union_type"):
                    row.append(("struct *", SS.struct_identity(base)))
                elif base.tag == "DW_TAG_base_type":
                    row.append(("scalar *", pointee_name(tdie)))
                else:
                    row.append((base.tag.replace("DW_TAG_", "") + " *", name))
            out[int(low)] = row
    return out


def pointee_name(tdie) -> str:
    """The first named DIE under the pointer, typedefs kept (`mp_limb_t`)."""
    while tdie is not None and tdie.tag != "DW_TAG_pointer_type":
        tdie = SS._type_die(tdie)
    tdie = SS._type_die(tdie) if tdie is not None else None
    while tdie is not None and tdie.tag in ("DW_TAG_const_type", "DW_TAG_volatile_type"):
        tdie = SS._type_die(tdie)
    return (SS._name(tdie) if tdie is not None else None) or "?"


def one(task):
    proj, b, o = task
    binary, twin = RES / o / proj / "stripped" / b, RES / o / proj / "compiled" / b
    on, err = run(binary, [("structsynth", "param")], True)
    off, _ = run(binary, [], False)
    params = dwarf_params(twin)
    off_by = {f["address"]: f for f in off.get("functions") or []}
    on_by = {f["address"]: f for f in on.get("functions") or []}
    rows = []
    for line in err.splitlines():
        if not line.startswith("STRUCTSYNTH_CENSUS\t"):
            continue
        _, addr, name, size, reg, spec = (line.split("\t") + [""])[:6]
        addr, reg = int(addr, 16), int(reg)
        idx = SYSV.get(reg)
        fn_on = on_by.get(addr) or {}
        args = {v["arg_index"]: v for v in fn_on.get("variables") or [] if v.get("kind") == "arg"}
        if idx is None:
            hits = [i for i, v in args.items() if v["type"] == f"{name} *"]
            idx = hits[0] if len(hits) == 1 else None
        installed = idx is not None and args.get(idx, {}).get("type") == f"{name} *"
        off_args = {v["arg_index"]: v for v in (off_by.get(addr) or {}).get("variables") or []
                    if v.get("kind") == "arg"}
        dw = params.get(addr)
        if dw is None:
            gt = ("not in DWARF", "")
        elif idx is None or idx >= len(dw):
            gt = ("no such DWARF param", "")
        else:
            gt = dw[idx]
        fields = parse_fields(spec)
        rows.append({"bin": f"{b} {o}", "addr": hex(addr), "name": name, "size": int(size),
                     "arg": idx, "installed": installed, "fields": fields, "shape": shape(fields),
                     "gt_kind": gt[0], "gt_spelling": gt[1],
                     "off_type": off_args.get(idx, {}).get("type") if idx is not None else None})
    return rows


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--out", default="arrays_census.json")
    ap.add_argument("--workers", type=int, default=8)
    a = ap.parse_args()
    with ThreadPoolExecutor(a.workers) as ex:
        rows = [r for rs in ex.map(one, BINS) for r in rs]
    json.dump(rows, open(a.out, "w"), indent=1)
    table = {}
    for r in rows:
        table.setdefault(r["shape"], {}).setdefault(r["gt_kind"], 0)
        table[r["shape"]][r["gt_kind"]] += 1
    print(f"{len(rows)} synthesized bases ({sum(r['installed'] for r in rows)} joined to a struct_N argument)")
    for s, kinds in sorted(table.items()):
        print(f"  {s:14s} {sum(kinds.values()):4d}  " +
              ", ".join(f"{k}={n}" for k, n in sorted(kinds.items(), key=lambda kv: -kv[1])))


if __name__ == "__main__":
    main()
