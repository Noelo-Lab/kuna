"""Classify every function calleevote changes, from corpus-hunks.py's off/on decompile-all JSONs.

usage: corpus-classes.py <outdir of corpus-hunks.py>  -> prints corpus-hunks.json to stdout

Classes, first match wins: error-flip, var-count (exported variables), renumber-only (equal once every
struct_N is spelled struct_N), signature-only, types-and-spelling (same statements, calls and argument
counts; corpus-hunks.py shape()), SHAPE. Tags counted on top of the class: byte-split-stores (a new run of
consecutive character stores through one pointer), local-pointer-to-integer (a local declared as a
pointer before is a plain integer after), local-count-moved (a different number of declared locals).
"""
import json, re, sys, os, glob, collections
D = sys.argv[1]
here = os.path.dirname(os.path.abspath(__file__))
src = open(os.path.join(here, "corpus-hunks.py")).read()
ns = {"re": re, "collections": collections}
exec(src[src.index("CALL = "):src.index("res = collections.Counter()")], ns)
shape, args_count = ns["shape"], ns["args_count"]
DECL = re.compile(r"^\s+([\w ]+?)\s*(\*+)?\s*(v\d+)(\[\w+\])?;")
INT = {"long", "unsigned long", "int", "unsigned int", "char", "unsigned char", "short", "unsigned short",
       "long long", "unsigned long long", "int8", "uint8", "int4", "uint4"}
def decls(code):
    out = {}
    for l in code.split("\n")[1:]:
        m = DECL.match(l)
        if m:
            out[m.group(3)] = (m.group(1).strip() + (" " + m.group(2) if m.group(2) else "")).strip()
    return out
def runs(code):
    n = 0; prev = None
    for l in code.split("\n"):
        m = re.match(r"\s*(\w+)\[(0x[0-9a-f]+|\d+)\] = '", l)
        cur = (m.group(1), int(m.group(2), 0)) if m else None
        if cur and prev and cur[0] == prev[0] and cur[1] == prev[1] + 1:
            n += 1
        prev = cur
    return n
norm = lambda c: re.sub(r"struct_\d+", "struct_N", c)
out = {"binaries": {}, "total": collections.Counter(), "tags": collections.Counter(), "examples": collections.defaultdict(list)}
for fon in sorted(glob.glob(f"{D}/*.on.json")):
    name = os.path.basename(fon)[:-len(".on.json")].split("2026-09-11_")[-1].replace("_stripped_", "/").replace("_", "/", 1)
    on = {x["address"]: x for x in json.load(open(fon))["functions"]}
    off = {x["address"]: x for x in json.load(open(fon[:-len(".on.json")] + ".off.json"))["functions"]}
    row = collections.Counter(functions=len(off))
    for a in sorted(set(off) | set(on)):
        x, y = off.get(a), on.get(a)
        if not x or not y:
            k = "missing"
        elif (x.get("error") is None) != (y.get("error") is None):
            k = "error-flip"
        else:
            cx, cy = x.get("code") or "", y.get("code") or ""
            if cx == cy and x["variables"] == y["variables"]:
                continue
            if len(x["variables"]) != len(y["variables"]):
                k = "var-count"
            elif norm(cx) == norm(cy):
                k = "renumber-only"
            elif cx.split("\n", 1)[1:] == cy.split("\n", 1)[1:]:
                k = "signature-only"
            elif shape(cx) == shape(cy) and args_count(cx.split("\n", 1)[-1]) == args_count(cy.split("\n", 1)[-1]):
                k = "types-and-spelling"
            else:
                k = "SHAPE"
            dx, dy = decls(cx), decls(cy)
            tags = []
            if runs(cy) > runs(cx):
                tags.append("byte-split-stores")
            if any(v in dy and "*" in dx[v] and dy[v] in INT for v in dx):
                tags.append("local-pointer-to-integer")
            if len(dx) != len(dy):
                tags.append("local-count-moved")
            for t in tags:
                out["tags"][t] += 1
                out["examples"][t].append(f"{name} {hex(a)}")
        row[k] += 1
        out["total"][k] += 1
        if k not in ("renumber-only", "signature-only", "types-and-spelling"):
            out["examples"][k].append(f"{name} {hex(a)}")
    out["binaries"][name] = dict(row)
out["total"]["changed"] = sum(v for k, v in out["total"].items())
out["total"]["functions"] = sum(r["functions"] for r in out["binaries"].values())
print(json.dumps(out, indent=1))
