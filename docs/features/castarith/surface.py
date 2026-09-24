#!/usr/bin/env python3
"""JSON-surface check for castarith: run `decompile-all --json` with the option
off and on (one build) and count, per function, the instruction addresses that
`line_mappings` and `variables[].addresses` carry in one arm and not the other.

  surface.py KUNA OUTDIR OPT/PROJECT/BIN...

A converted access prints as a subscript, and a subscript is a surround token
that carries no op (upstream PrintLanguage::emitOp), so an instruction whose
only printed ops were the add and the LOAD/STORE through it maps to no line.
"""
import json, subprocess, sys
from pathlib import Path

R = Path("/home/mahaloz/github/decbench/results/full_run_address_2026-09-11")


def load(kuna, binary, out, arm):
    p = out / f"{binary.parent.parent.parent.name}-{binary.name}-{arm}.json"
    if not p.exists():
        extra = ["--option", "castarith", "off"] if arm == "off" else []
        r = subprocess.run([kuna, "decompile-all", str(binary), "--json"] + extra,
                           capture_output=True, text=True, check=True)
        p.write_text(r.stdout)
    return {f["address"]: f for f in json.loads(p.read_text()).get("functions", [])}


def lines(f):
    return {a for e in f.get("line_mappings") or [] for a in e.get("addresses", [])}


def varaddrs(f):
    return {(v.get("name"), a) for v in f.get("variables") or [] for a in v.get("addresses") or []}


def main():
    kuna, out = sys.argv[1], Path(sys.argv[2])
    out.mkdir(parents=True, exist_ok=True)
    tot = dict(funcs=0, mapped_off=0, lm_lost=0, lm_gained=0, lm_funcs=0, va_lost=0, va_gained=0, va_funcs=0)
    for spec in sys.argv[3:]:
        binary = R / spec.split("/")[0] / spec.split("/")[1] / "stripped" / spec.split("/")[2]
        off, on = load(kuna, binary, out, "off"), load(kuna, binary, out, "on")
        for addr in sorted(set(off) & set(on)):
            a, b = off[addr], on[addr]
            tot["funcs"] += 1
            la, lb = lines(a), lines(b)
            tot["mapped_off"] += len(la)
            if la != lb:
                tot["lm_funcs"] += 1
                tot["lm_lost"] += len(la - lb)
                tot["lm_gained"] += len(lb - la)
                print(f"LINEMAP {spec} {hex(addr)} lost {[hex(x) for x in sorted(la - lb)]} gained {[hex(x) for x in sorted(lb - la)]}")
            va, vb = varaddrs(a), varaddrs(b)
            if va != vb:
                tot["va_funcs"] += 1
                tot["va_lost"] += len(va - vb)
                tot["va_gained"] += len(vb - va)
                print(f"VARADDR {spec} {hex(addr)} lost {len(va - vb)} gained {len(vb - va)}")
    print(json.dumps(tot))


if __name__ == "__main__":
    main()
