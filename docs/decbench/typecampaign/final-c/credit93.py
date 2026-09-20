"""Counterfactual replay of the Noelo-Lab/decbench#93 crediting rule.

Rule: a decompiler variable that is a pointer to a structure counts as a match
for a ground-truth pointer-to-struct, whatever the structure is named.  The
score of a function is tp / gt_vars, so re-crediting only moves tp.
"""
import json
import sys

PRIM = {"char", "signed char", "unsigned char", "short", "unsigned short", "int",
        "unsigned int", "long", "unsigned long", "long long", "unsigned long long",
        "float", "double", "long double", "void", "bool", "_Bool", "code", "size_t",
        "wchar_t", "undefined", "undefined1", "undefined2", "undefined4", "undefined8",
        "undefined16", "int1", "int2", "int4", "int8", "int16",
        "uint1", "uint2", "uint4", "uint8", "uint16"}


def is_struct_ptr(t):
    if not t:
        return False
    s = t.strip()
    if not s.endswith("*") or s.count("*") != 1:
        return False
    base = s[:-1].strip()
    if base.startswith("struct "):
        base = base[len("struct "):].strip()
    return bool(base) and base not in PRIM


def replay(rows):
    tp_added = 0
    per_fn = {}
    for k, v in rows.items():
        for d in (v.get("decisions") or {}).get("base") or []:
            key = (k, d["fn"])
            r = per_fn.setdefault(key, [0, 0, 0])   # gt vars, tp, credited
            r[0] += 1
            r[1] += d["ok"] is True
            if d["ok"] is False and d["cls"] == "ptr_struct" and is_struct_ptr(d["kuna_type"]):
                r[2] += 1
                tp_added += 1
    perfect = sum(1 for n, tp, cr in per_fn.values() if n and tp == n)
    perfect_c = sum(1 for n, tp, cr in per_fn.values() if n and tp + cr == n)
    mean = sum(tp / n for n, tp, cr in per_fn.values() if n) / max(1, len(per_fn))
    mean_c = sum(min(1.0, (tp + cr) / n) for n, tp, cr in per_fn.values() if n) / max(1, len(per_fn))
    return {"functions_with_gt": len(per_fn), "tp_added": tp_added,
            "perfect": perfect, "perfect_credited": perfect_c,
            "onto_perfect": perfect_c - perfect,
            "mean": round(mean, 4), "mean_credited": round(mean_c, 4)}


if __name__ == "__main__":
    out = {}
    for label, path in (a.split("=", 1) for a in sys.argv[1:]):
        out[label] = replay(json.loads(open(path).read()))
        print(label, json.dumps(out[label]), flush=True)
    print(json.dumps(out))
