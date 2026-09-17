#!/usr/bin/env python3
"""Layout precision for structsynth, filler counted and filler excluded.

    KUNA_BIN=<release kuna> ~/.virtualenvs/decbench/bin/python \
        docs/features/structsynth/layoutscore.py

A synthesized layout has two kinds of member: a field the pass claims (a scalar
or a pointer at an offset something dereferenced) and `undefined1
field_0x<hex>[N]` filler, which exists only so the printer cannot name an
offset the header does not declare.  Filler is padding, which is exactly what
`structscore --layout` already drops on the ground-truth side, so the honest
precision is over the claimed fields; the filler-counted number is printed
beside it so the difference is visible rather than assumed.

Filler is recognized by its array type: this version of the pass never claims an
array-typed field.
"""
import json, os, sys
from pathlib import Path
sys.path.insert(0, str(Path(__file__).resolve().parents[3]))
from scripts.decbench import structscore as SS

RES = Path("/home/mahaloz/github/decbench/results/full_run_address_2026-09-11")
BINS = [("fmt","O0"),("fmt","O2"),("ls","O0"),("ls","O2"),("sort","O0"),("sort","O2"),
        ("du","O0"),("du","O2")]
if os.environ.get("KUNA_BIN"):
    SS.KUNA = os.environ["KUNA_BIN"]

_raw = SS.header_layouts


def mark_arrays(header: str) -> dict:
    """`header_layouts`, each member tagged with whether it is an array."""
    layouts = _raw(header)
    for name, body in SS.STRUCT_RE.findall(header):
        if name not in layouts:
            continue
        counts = {}
        for line in body.split("\n"):
            m = SS.MEMBER_RE.match(line)
            if m:
                counts[m.group("name")] = bool(m.group("count"))
        for f in layouts[name]["fields"]:
            f["array"] = counts.get(f["name"], False)
    return layouts


def drop_filler(header: str) -> dict:
    """`mark_arrays` with every array-typed member removed."""
    return {n: {**l, "fields": [f for f in l["fields"] if not f.get("array")]}
            for n, l in mark_arrays(header).items()}


rows, pooled = [], {}
for b, o in BINS:
    binary = RES / o / "coreutils" / "stripped" / b
    twin = RES / o / "coreutils" / "compiled" / b
    fns = SS.dwarf_functions(twin)
    payload = SS.run_json(binary, [["structsynth", "param"]], 1800)
    header = SS.run_header(binary, [["structsynth", "param"]], 1800)
    row = {}
    for tag, hook in (("filler counted", mark_arrays), ("fields only", drop_filler)):
        SS.header_layouts = hook
        row[tag] = SS.measure_layout(payload, header, fns)["layout"]
    SS.header_layouts = _raw
    rows.append((f"{b} {o}", row))
    for tag in row:
        p = pooled.setdefault(tag, {"tp": 0, "predicted": 0, "ground_truth": 0})
        for k in p:
            p[k] += row[tag][k]
    print(f"{b} {o}: fields-only P={row['fields only']['precision']} "
          f"({row['fields only']['tp']}/{row['fields only']['predicted']}), "
          f"filler-counted P={row['filler counted']['precision']} "
          f"({row['filler counted']['tp']}/{row['filler counted']['predicted']}), "
          f"gt={row['fields only']['ground_truth']}")
for tag, p in pooled.items():
    prec = p["tp"] / p["predicted"] if p["predicted"] else 0.0
    rec = p["tp"] / p["ground_truth"] if p["ground_truth"] else 0.0
    f1 = 2 * prec * rec / (prec + rec) if prec + rec else 0.0
    print(f"POOLED {tag}: precision {p['tp']}/{p['predicted']} = {prec:.4f}  "
          f"recall {p['tp']}/{p['ground_truth']} = {rec:.4f}  F1 {f1:.4f}")
out = os.environ.get("LAYOUTSCORE_OUT")
if out:
    json.dump({"rows": rows, "pooled": pooled}, open(out, "w"), indent=1)
