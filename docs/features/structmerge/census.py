#!/usr/bin/env python3
"""Where structsynth's 8,626 unclaimed ground-truth fields are.

Joins three views of the same eight builds:

  * DWARF ground truth for every pointer-to-struct PARAMETER (structscore's own
    `dwarf_functions`, the layout instrument's denominator);
  * kuna's `decompile-all --json` + `decompile-project` header, which says which
    of those parameters kuna typed as a record and with which fields
    (structscore's `measure_layout`, field for field);
  * an `SSCENSUS` stderr line per candidate base from a census build of the
    pass, which says what the function actually dereferenced (pre-prune), what
    the layout prune left, and the exact decline reason.

Usage:  KUNA_CENSUS_BIN=.scratch/kuna-census ~/.virtualenvs/decbench/bin/python .scratch/census.py
"""
import json, os, re, subprocess, sys, tempfile, shutil
from pathlib import Path
from concurrent.futures import ProcessPoolExecutor

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / ".scratch"))
import pindb  # noqa: E402
pindb.pin()
sys.path.insert(0, str(ROOT))
from scripts.decbench import structscore as SS

RES = Path("/home/mahaloz/github/decbench/results/full_run_address_2026-09-11")
BINS = [("fmt", "O0"), ("fmt", "O2"), ("ls", "O0"), ("ls", "O2"),
        ("sort", "O0"), ("sort", "O2"), ("du", "O0"), ("du", "O2")]
CENSUS_BIN = os.environ.get("KUNA_CENSUS_BIN", str(ROOT / ".scratch/kuna-census"))
OPTS = ["--option", "structsynth", os.environ.get("CENSUS_MODE", "param")]

# x86-64 SLEIGH register-space offsets of the SysV integer argument registers,
# in argument order.  The census runs before the FuncProto has a store, so the
# base varnode's storage is all there is to join an argument index on.
INT_ARG_REGS = [0x38, 0x30, 0x10, 0x8, 0x80, 0x88]   # rdi rsi rdx rcx r8 r9
LINE = re.compile(r"SSCENSUS fn=0x(?P<fn>[0-9a-f]+) arg=(?P<arg>-?\d+) input=(?P<input>\w+) "
                  r"reason=(?P<reason>\w+) ty=(?P<ty>\S*) vn=(?P<vn>[-\d]+:[0-9a-f]+:\d+) "
                  r"store=\w+ np=-?\d+ raw=(?P<raw>\S*) pruned=(?P<pruned>\S*) "
                  r"dropped=\[(?P<dropped>[^\]]*)\] dyn=(?P<dyn>\w+) int=(?P<int>\w+) "
                  r"phi=(?P<phi>\w+)")


def slots(text):
    out = []
    for part in text.split(","):
        if not part.strip():
            continue
        off, w = part.split(":")
        out.append((int(off), int(w)))
    return out


def run(binary):
    """(json payload, header text, census lines by function address)."""
    cmd = [CENSUS_BIN, "decompile-all", str(binary), "--json", "--max-fn-seconds", "120"] + OPTS
    env = dict(os.environ, KUNA_SS_CENSUS="1")
    p = subprocess.run(cmd, capture_output=True, text=True, timeout=3600, env=env)
    payload = json.loads(p.stdout)
    census = {}
    gate = {}
    for m in re.finditer(r"SSGATE fn=0x([0-9a-f]+) started=(\w+) exceeded=(\w+) settled=(\w+)", p.stderr):
        gate[int(m.group(1), 16)] = {"started": m.group(2) == "true",
                                     "exceeded": m.group(3) == "true",
                                     "settled": m.group(4) == "true"}
    for m in LINE.finditer(p.stderr):
        fn = int(m.group("fn"), 16)
        sp, off, size = m.group("vn").split(":")
        census.setdefault(fn, []).append({
            "reason": m.group("reason"), "input": m.group("input") == "true",
            "ty": m.group("ty"), "space": int(sp), "off": int(off, 16), "size": int(size),
            "raw": slots(m.group("raw")), "pruned": slots(m.group("pruned")),
            "dropped": slots(m.group("dropped")),
            "dyn": m.group("dyn") == "true", "int": m.group("int") == "true",
            "phi": m.group("phi") == "true"})
    out = Path(tempfile.mkdtemp(prefix="recallcensus-"))
    try:
        subprocess.run([CENSUS_BIN, "decompile-project", str(binary), "-o", str(out),
                        "--max-fn-seconds", "120"] + OPTS,
                       capture_output=True, text=True, timeout=3600)
        headers = sorted(out.glob("*.h"))
        header = headers[0].read_text() if headers else ""
    finally:
        shutil.rmtree(out, ignore_errors=True)
    return payload, header, census, gate


def arg_of_base(base, fnvars):
    """The kuna argument index of a census base, or None.

    kuna's own parameter list says how many register-class integer arguments the
    function has and in which source positions; the census base's register
    offset says which of those it is.
    """
    if not base["input"]:
        return None
    regs = [v for v in fnvars if v.get("arg_index") is not None
            and v.get("stack_offset") is None]
    regs.sort(key=lambda v: v["arg_index"])
    if not regs:
        return None
    ints = [v for v in regs if "double" not in (v.get("type") or "")
            and "float" not in (v.get("type") or "")]
    try:
        slot = INT_ARG_REGS.index(base["off"])
    except ValueError:
        return None
    if slot >= len(ints):
        return None
    return ints[slot].get("arg_index")


def touched(raw, gt_fields):
    """GT fields any pre-prune access of this base overlaps."""
    hit = set()
    for off, w in raw:
        for i, f in enumerate(gt_fields):
            fo, fs = f.get("offset"), f.get("size")
            if fo is None or fs is None:
                continue
            if off < fo + fs and fo < off + w:
                hit.add(i)
    return hit


def mark_arrays(header):
    """`header_layouts` with every member tagged filler (array-typed) or not."""
    layouts = SS.header_layouts(header)
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


def measure(binary, twin):
    payload, header, census, gate = run(binary)
    layouts = mark_arrays(header)
    fns = SS.dwarf_functions(twin)
    by_addr = {int(f.get("address") or -1): f for f in payload.get("functions") or []}
    rows = []
    for low, info in fns.items():
        fn = by_addr.get(low)
        fnvars = (fn.get("variables") or []) if fn else []
        bases = census.get(low, [])
        byarg = {}
        for b in bases:
            a = arg_of_base(b, fnvars)
            if a is not None and a not in byarg:
                byarg[a] = b
        for param in info["params"]:
            gtl = param["struct"]
            if not gtl:
                continue
            gt_real = [f for f in gtl["fields"] if f["kind"] != "pad"]
            gt_keys_l = [[f.get("offset"), f.get("size")] for f in gt_real]
            row = {"binary": binary.name, "fn": low, "arg": param["index"],
                   "gt_name": gtl.get("name"), "gt_keys": gt_keys_l,
                   "gt_fields": len(gt_real), "gt_size": gtl.get("size"),
                   "kuna_saw_fn": fn is not None, "reached_pass": low in census}
            predicted = None
            if fn:
                for v in fnvars:
                    if v.get("arg_index") == param["index"]:
                        rec = SS.parse_type(v.get("type"))
                        row["kuna_type"] = v.get("type")
                        if rec["kind"] == "struct":
                            predicted = layouts.get(rec["base"])
                        break
            base = byarg.get(param["index"])
            if base:
                row["reason"] = base["reason"]
            elif low in census:
                row["reason"] = "no_base"
            elif low in gate:
                g = gate[low]
                row["reason"] = ("gate_not_started" if not g["started"]
                                 else "gate_exceeded" if g["exceeded"]
                                 else "gate_unsettled")
            else:
                row["reason"] = "no_line"
            row["raw"] = base["raw"] if base else []
            row["pruned"] = base["pruned"] if base else []
            row["dropped"] = base["dropped"] if base else []
            hit_raw = touched(row["raw"], gt_real)
            hit_pruned = touched(row["pruned"], gt_real)
            row["touched"] = len(hit_raw)
            row["touched_keys"] = [gt_keys_l[i] for i in sorted(hit_raw)]
            row["touched_after_prune"] = len(hit_pruned)
            row["typed"] = predicted is not None
            if predicted:
                gt_keys = {(f.get("offset"), f.get("size")) for f in gt_real}
                claimed = [f for f in predicted["fields"]]
                row["claimed"] = len(claimed)
                row["claimed_keys"] = [[f["offset"], f["size"]] for f in claimed]
                fo = [f for f in claimed if not f.get("array")]
                row["claimed_fields_only"] = len(fo)
                row["tp_fields_only"] = sum(
                    1 for f in fo if (f["offset"], f["size"]) in gt_keys)
                row["struct_name"] = SS.parse_type(row.get("kuna_type") or "")["base"]
                row["tp"] = sum(1 for f in claimed if (f["offset"], f["size"]) in gt_keys)
                # a GT field the reader touched, kept through the prune, yet the
                # installed record does not claim at its (offset,size)
                cl = {(f["offset"], f["size"]) for f in claimed}
                row["touched_not_claimed"] = sum(
                    1 for i in hit_pruned
                    if (gt_real[i].get("offset"), gt_real[i].get("size")) not in cl)
            rows.append(row)
    return rows


def job(spec):
    b, o = spec
    binary = RES / o / "coreutils" / "stripped" / b
    twin = RES / o / "coreutils" / "compiled" / b
    rows = measure(binary, twin)
    for r in rows:
        r["opt"] = o
        r["prog"] = b
    return rows


if __name__ == "__main__":
    allrows = []
    with ProcessPoolExecutor(max_workers=8) as ex:
        for rows in ex.map(job, BINS):
            allrows += rows
    out = os.environ.get("CENSUS_OUT", str(ROOT / ".scratch/census.json"))
    json.dump(allrows, open(out, "w"))
    print(f"{len(allrows)} ptr-to-struct parameters, {sum(r['gt_fields'] for r in allrows)} GT fields -> {out}")
