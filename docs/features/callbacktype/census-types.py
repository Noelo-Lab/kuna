"""callbacktype census, step B: for each callback target decbench scores, what
does the DWARF twin declare, what does kuna recover today, and what is the
function's published type_match?"""
from __future__ import annotations

import json
import os
import subprocess
import sys
from concurrent.futures import ProcessPoolExecutor, as_completed
from pathlib import Path

sys.path.insert(0, "/home/mahaloz/kwt/callbacktype")
sys.path.insert(0, "/home/mahaloz/kwt/callbacktype/docs/decbench/typecampaign/final-g")
os.environ.setdefault("DECBENCH_PIN", "/home/mahaloz/kwt/_final-d/db625")
os.environ.setdefault("KUNA_DECBENCH_RESULTS",
                      "/home/mahaloz/github/decbench/results/full_run_address_2026-09-11")
import pindb  # noqa: E402

pindb.pin()

from scripts.decbench import config  # noqa: E402

KUNA = os.environ.get("KUNA_BIN", "/home/mahaloz/kwt/callbacktype/decompiler/target/release/kuna")


def kuna_vars(stripped, options=()):
    cmd = [KUNA, "decompile-all", str(stripped), "--json", "--max-fn-seconds", "120"]
    for n, v in options:
        cmd += ["--option", n, v]
    p = subprocess.run(cmd, capture_output=True, text=True, timeout=1800)
    try:
        doc = json.loads(p.stdout)
    except Exception:  # noqa: BLE001
        return {}
    out = {}
    for f in doc.get("functions", []):
        try:
            a = int(str(f.get("address")), 0)
        except Exception:  # noqa: BLE001
            continue
        out[a] = {"name": f.get("name"), "vars": f.get("variables") or []}
    return out


def dwarf_params(unstripped):
    """addr -> (return type, [ (name, type, is_param) ]) for every subprogram."""
    from elftools.elf.elffile import ELFFile

    out = {}
    with open(unstripped, "rb") as fh:
        elf = ELFFile(fh)
        if not elf.has_dwarf_info():
            return out
        dw = elf.get_dwarf_info()
        for cu in dw.iter_CUs():
            top = cu.get_top_DIE()
            for die in top.iter_children():
                if die.tag != "DW_TAG_subprogram":
                    continue
                lo = die.attributes.get("DW_AT_low_pc")
                if lo is None:
                    continue
                params = []
                for c in die.iter_children():
                    if c.tag == "DW_TAG_formal_parameter":
                        nm = c.attributes.get("DW_AT_name")
                        params.append((
                            nm.value.decode() if nm else "?",
                            type_name(cu, c.attributes.get("DW_AT_type")),
                        ))
                out[lo.value] = {
                    "ret": type_name(cu, die.attributes.get("DW_AT_type")),
                    "params": params,
                }
    return out


def type_name(cu, attr, depth=0):
    if attr is None or depth > 8:
        return "void" if attr is None else "?"
    try:
        die = cu.get_DIE_from_refaddr(attr.value + cu.cu_offset)
    except Exception:  # noqa: BLE001
        return "?"
    t = die.tag
    nm = die.attributes.get("DW_AT_name")
    if t == "DW_TAG_pointer_type":
        return type_name(cu, die.attributes.get("DW_AT_type"), depth + 1) + " *"
    if t in ("DW_TAG_const_type", "DW_TAG_volatile_type"):
        return type_name(cu, die.attributes.get("DW_AT_type"), depth + 1)
    if t == "DW_TAG_typedef":
        return nm.value.decode() if nm else "?"
    if t in ("DW_TAG_structure_type", "DW_TAG_union_type", "DW_TAG_enumeration_type"):
        return (nm.value.decode() if nm else "anon")
    if nm:
        return nm.value.decode()
    return t


def work(item):
    slice_key, binpath, targets = item
    stripped = binpath
    unstripped = stripped.replace("/stripped/", "/compiled/")
    kv = kuna_vars(stripped)
    dw = dwarf_params(unstripped)
    rows = []
    for t, info in targets.items():
        k = kv.get(t)
        rows.append({
            "slice": slice_key, "target": t, "name": info["name"],
            "callee": info["callee"], "sig": info["sig"], "ret": info["ret"],
            "dwarf": dw.get(t),
            "kuna": [{"n": v.get("name"), "t": v.get("type"), "k": v.get("kind"),
                      "loc": v.get("location")} for v in (k or {}).get("vars", [])],
        })
    return rows


def main():
    scan = json.loads(Path(".scratch/cbscan.json").read_text())
    jobs = {}
    for r in scan:
        for h in r.get("hits", []):
            if not h["scored"]:
                continue
            jobs.setdefault((r["slice"], r["bin"]), {})[h["target"]] = h
    items = [(k[0], k[1], v) for k, v in jobs.items()]
    print(f"binaries: {len(items)}", file=sys.stderr)
    out = []
    with ProcessPoolExecutor(max_workers=10) as ex:
        futs = [ex.submit(work, it) for it in items]
        for f in as_completed(futs):
            try:
                out.extend(f.result())
            except Exception as e:  # noqa: BLE001
                print("ERR", e, file=sys.stderr)
    Path(sys.argv[1]).write_text(json.dumps(out, indent=1))
    print(f"rows: {len(out)}", file=sys.stderr)


if __name__ == "__main__":
    main()
