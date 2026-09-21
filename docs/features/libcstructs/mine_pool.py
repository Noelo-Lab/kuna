"""Mine the libc/POSIX-named `ptr_struct` ground-truth pool from the DWARF twins.

For every GT variable decbench would score (same slice set, same function set,
same DIE walk as `final/gtclass.py`), record the pointee STRUCT TAG when the
variable's class is `ptr_struct`.  Output: one row per (slice, function,
variable) as JSON lines.
"""
from __future__ import annotations

import json
import os
import sys
from concurrent.futures import ProcessPoolExecutor, as_completed
from pathlib import Path

sys.path.insert(0, "/home/mahaloz/kwt/libcstructs")
sys.path.insert(0, "/home/mahaloz/kwt/libcstructs/docs/decbench/typecampaign/final-c")
import pindb  # noqa: E402
pindb.pin()

sys.path.insert(0, "/home/mahaloz/kwt/libcstructs/docs/decbench/typecampaign/final")
import gtclass  # noqa: E402
from scripts.decbench import config, typesweep as ts  # noqa: E402

PTR_TAGS = gtclass.PTR_TAGS
SKIP = gtclass.SKIP_TAGS


def pointee_name(var_die, dw):
    raw = gtclass._type_of(var_die, dw)
    t = gtclass._strip(raw, dw)
    if t is None or t.tag not in PTR_TAGS:
        return None
    # the pointee, with typedefs kept so the TYPEDEF spelling is recorded too
    inner = gtclass._type_of(t, dw)
    tdefs = []
    hops = 0
    while inner is not None and inner.tag in SKIP and hops < 32:
        if inner.tag == "DW_TAG_typedef":
            a = inner.attributes.get("DW_AT_name")
            if a:
                tdefs.append(a.value.decode("utf-8", "replace"))
        inner = gtclass._type_of(inner, dw)
        hops += 1
    if inner is None or inner.tag not in ("DW_TAG_structure_type", "DW_TAG_class_type"):
        return None
    a = inner.attributes.get("DW_AT_name")
    tag = a.value.decode("utf-8", "replace") if a else "<anon>"
    return tag, tdefs, gtclass._size(inner)


def walk(die, dw, out, is_arg=False, ai=None):
    v = gtclass._parse_variable_die(die, dw, is_arg=is_arg, arg_index=ai)
    if not v or v.get("cls") != "ptr_struct":
        return
    pn = pointee_name(die, dw)
    if pn is None:
        return
    tag, tdefs, sz = pn
    out.append({"name": v["name"], "is_arg": v["is_arg"], "tag": tag,
                "typedefs": tdefs, "psize": sz, "gt": (v["type"] or [None])[0]})


def blocks(die, dw, out):
    for c in die.iter_children():
        if c.tag == "DW_TAG_lexical_block":
            blocks(c, dw, out)
        elif c.tag in ("DW_TAG_formal_parameter", "DW_TAG_variable"):
            walk(c, dw, out)


def one(task):
    project, opt, stem, unstripped, a2n = task
    from decbench.utils import binfmt
    keep = set(a2n.values())
    rows = []
    try:
        dw = binfmt.dwarf_info(Path(unstripped))
        if dw is None:
            return project, opt, stem, rows
        for CU in dw.iter_CUs():
            for DIE in CU.get_top_DIE().iter_children():
                if DIE.tag != "DW_TAG_subprogram":
                    continue
                fn = binfmt.die_str_attr(DIE, "DW_AT_name")
                if fn is None or fn not in keep:
                    continue
                out = []
                ai = 0
                for c in DIE.iter_children():
                    if c.tag in ("DW_TAG_lexical_block", "DW_TAG_inlined_subroutine"):
                        blocks(c, dw, out)
                    elif c.tag == "DW_TAG_formal_parameter":
                        walk(c, dw, out, is_arg=True, ai=ai)
                        ai += 1
                    elif c.tag == "DW_TAG_variable":
                        walk(c, dw, out)
                for r in out:
                    r["fn"] = fn
                rows.extend(out)
    except Exception as e:  # noqa: BLE001
        return project, opt, stem, [{"error": str(e)[:200]}]
    return project, opt, stem, rows


def main():
    projects = set("coreutils grep gzip diffutils bzip2 findutils tar shadow x".split())
    opts = {"O0", "O2", "O2-noinline"}
    root = config.results_root()
    slices = ts.collect_slices(root, projects, opts)
    print(f"slices {len(slices)}", file=sys.stderr, flush=True)
    outf = open(sys.argv[1], "w")
    done = 0
    with ProcessPoolExecutor(max_workers=12) as ex:
        futs = [ex.submit(one, t) for t in slices]
        for f in as_completed(futs):
            project, opt, stem, rows = f.result()
            done += 1
            for r in rows:
                r["slice"] = f"{project}::{opt}::{stem}"
                outf.write(json.dumps(r) + "\n")
            if done % 50 == 0:
                print(f"[{done}/{len(slices)}]", file=sys.stderr, flush=True)
    outf.close()
    print("MINE_DONE", file=sys.stderr, flush=True)


if __name__ == "__main__":
    main()
