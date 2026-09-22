"""Whole-corpus before/after decompile-all diff for calleevote: classify every changed function.

usage: hunks.py <kuna> <outdir> <bin>...   (off = --option calleevote off, on = default)
"""
import json, os, re, subprocess, sys, collections
from concurrent.futures import ThreadPoolExecutor
K = sys.argv[1]; OUT = sys.argv[2]; BINS = sys.argv[3:]
os.makedirs(OUT, exist_ok=True)
CALL = re.compile(r"\b([A-Za-z_]\w*)\(")
KW = re.compile(r"\b(if|while|for|do|goto|return|switch|case|break|continue|else)\b")
def run(b, arm):
    tag = b.replace("/", "_")
    f = f"{OUT}/{tag}.{arm}.json"
    if not os.path.exists(f):
        extra = ["--option", "calleevote", "off"] if arm == "off" else []
        subprocess.run([K, "decompile-all", b, "--json", "--max-fn-seconds", "120"] + extra, stdout=open(f, "w"), stderr=subprocess.DEVNULL)
    return {x["address"]: x for x in json.load(open(f))["functions"]}
def body_lines(code):
    ls = [l for l in (code or "").split("\n")]
    # drop declaration lines (type + name + ;) at the top of the body
    return ls
def shape(code):
    body = (code or "").split("{", 1)[1] if "{" in (code or "") else ""
    stmts = [l.strip() for l in body.split("\n") if l.strip() and not re.match(r"^[\w\s\*\[\]]+\s\**v\d+(\s*\[\w+\])?;(\s*//.*)?$", l.strip())]
    calls = []
    for l in stmts:
        for m in CALL.finditer(l):
            if m.group(1) not in ("if", "while", "for", "switch", "return", "sizeof"):
                calls.append(m.group(1))
    kws = [k for l in stmts for k in KW.findall(l)]
    nargs = []
    for l in stmts:
        for m in re.finditer(r"\b(sub_[0-9a-f]+|[a-z_]\w*)\((.*)\)", l):
            pass
    return len(stmts), calls, kws
def args_count(code):
    out = []
    for m in re.finditer(r"\b(sub_[0-9a-f]+|[A-Za-z_]\w*)\(", code or ""):
        i = m.end(); d = 1; j = i; n = 1 if code[i:i+1] != ")" else 0
        while j < len(code) and d:
            c = code[j]
            if c == "(": d += 1
            elif c == ")": d -= 1
            elif c == "," and d == 1: n += 1
            j += 1
        out.append((m.group(1), n))
    return out
res = collections.Counter(); rows = []
def one(b):
    off = run(b, "off"); on = run(b, "on")
    out = []
    for a in sorted(set(off) | set(on)):
        x, y = off.get(a), on.get(a)
        if not x or not y:
            out.append((b, a, "missing")); continue
        if (x.get("error") is None) != (y.get("error") is None):
            out.append((b, a, "error-flip")); continue
        cx, cy = x.get("code") or "", y.get("code") or ""
        if cx == cy:
            continue
        if len(x["variables"]) != len(y["variables"]):
            out.append((b, a, "var-count")); continue
        px, py = cx.split("\n", 1)[0], cy.split("\n", 1)[0]
        bx, by = cx.split("\n", 1)[1:] or [""], cy.split("\n", 1)[1:] or [""]
        if bx == by:
            out.append((b, a, "signature-only")); continue
        if shape(cx) == shape(cy) and args_count(cx.split("\n",1)[-1]) == args_count(cy.split("\n",1)[-1]):
            out.append((b, a, "types-and-spelling")); continue
        out.append((b, a, "SHAPE"))
    return out
with ThreadPoolExecutor(6) as ex:
    for r in ex.map(one, BINS):
        rows += r
c = collections.Counter(k for _, _, k in rows)
print(dict(c))
json.dump(rows, open(f"{OUT}/hunks.json", "w"))
for b, a, k in rows:
    if k not in ("signature-only", "types-and-spelling"):
        print(k, b.split("results/")[-1], hex(a))
