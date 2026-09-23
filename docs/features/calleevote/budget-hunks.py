"""Whole-corpus before/after decompile-all diff for the redo budget.

`corpus-hunks.py` compares the option off against the default with one build;
this compares two BUILDS running their defaults -- the flat 32-line refusal
against the budget that replaced it -- and classifies every function whose body
moved.

usage: budget-hunks.py <kuna-before> <kuna-after> <outdir> <bin>...
"""
import collections
import json
import os
import re
import subprocess
import sys
from concurrent.futures import ThreadPoolExecutor

BEFORE, AFTER, OUT = sys.argv[1], sys.argv[2], sys.argv[3]
BINS = sys.argv[4:]
os.makedirs(OUT, exist_ok=True)
CALL = re.compile(r"\b([A-Za-z_]\w*)\(")
CHAR_STORE = re.compile(r"= '(?:\\.|[^'\\])+';")
KW = re.compile(r"\b(if|while|for|do|goto|return|switch|case|break|continue|else)\b")


def run(b, arm):
    tag = b.replace("/", "_")
    f = f"{OUT}/{tag}.{arm}.json"
    if not os.path.exists(f):
        k = BEFORE if arm == "before" else AFTER
        subprocess.run([k, "decompile-all", b, "--json", "--max-fn-seconds", "120"],
                       stdout=open(f, "w"), stderr=subprocess.DEVNULL)
    return {x["address"]: x for x in json.load(open(f))["functions"]}


def shape(code):
    body = (code or "").split("{", 1)[1] if "{" in (code or "") else ""
    stmts = [l.strip() for l in body.split("\n")
             if l.strip() and not re.match(r"^[\w\s\*\[\]]+\s\**v\d+(\s*\[\w+\])?;(\s*//.*)?$", l.strip())]
    calls = [m.group(1) for l in stmts for m in CALL.finditer(l)
             if m.group(1) not in ("if", "while", "for", "switch", "return", "sizeof")]
    kws = [k for l in stmts for k in KW.findall(l)]
    return len(stmts), calls, kws


def args_count(code):
    out = []
    for m in re.finditer(r"\b(sub_[0-9a-f]+|[A-Za-z_]\w*)\(", code or ""):
        i = m.end()
        d, j = 1, i
        n = 1 if code[i:i + 1] != ")" else 0
        while j < len(code) and d:
            c = code[j]
            if c == "(":
                d += 1
            elif c == ")":
                d -= 1
            elif c == "," and d == 1:
                n += 1
            j += 1
        out.append((m.group(1), n))
    return out


def one(b):
    before, after = run(b, "before"), run(b, "after")
    out = []
    for a in sorted(set(before) | set(after)):
        x, y = before.get(a), after.get(a)
        if not x or not y:
            out.append((b, a, "missing"))
            continue
        if (x.get("error") is None) != (y.get("error") is None):
            out.append((b, a, "error-flip"))
            continue
        cx, cy = x.get("code") or "", y.get("code") or ""
        if cx == cy:
            continue
        if len(x["variables"]) != len(y["variables"]):
            out.append((b, a, "var-count"))
            continue
        if len(CHAR_STORE.findall(cy)) > len(CHAR_STORE.findall(cx)) and shape(cy)[0] > shape(cx)[0]:
            out.append((b, a, "CHAR-SPLIT"))
            continue
        bx = cx.split("\n", 1)[1:] or [""]
        by = cy.split("\n", 1)[1:] or [""]
        if bx == by:
            out.append((b, a, "signature-only"))
            continue
        if shape(cx) == shape(cy) and args_count(cx.split("\n", 1)[-1]) == args_count(cy.split("\n", 1)[-1]):
            out.append((b, a, "types-and-spelling"))
            continue
        out.append((b, a, "SHAPE"))
    return out


rows = []
with ThreadPoolExecutor(6) as ex:
    for r in ex.map(one, BINS):
        rows += r
print(dict(collections.Counter(k for _, _, k in rows)))
json.dump(rows, open(f"{OUT}/hunks.json", "w"))
for b, a, k in rows:
    if k not in ("signature-only", "types-and-spelling"):
        print(k, b.split("results/")[-1], hex(a))
