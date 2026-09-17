#!/usr/bin/env python3
"""For every function whose text changed between two decompile-all trees, run
`gcc -fsyntax-only` on the before and after text and report the diagnostic delta.

A prelude of `typedef char <t>;` lines is synthesized for the pseudo-types kuna
prints (`code`, `undefined4`, ...) so gcc parses past the signature and reaches
its label check; the same prelude is used for both sides.
"""
import sys, os, glob, subprocess, tempfile, re, importlib.util
here = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("orphan_scan", os.path.join(here, "orphan_scan.py"))
osc = importlib.util.module_from_spec(spec); spec.loader.exec_module(osc)

UNKNOWN = re.compile(r"unknown type name .(\w+).")

def load(path):
    return {f"{n}@{a}": b for n, a, b in osc.functions(open(path, errors='replace').read())}

def run(text):
    with tempfile.NamedTemporaryFile('w', suffix='.c', delete=False) as f:
        f.write(text); p = f.name
    out = subprocess.run(['gcc', '-fsyntax-only', '-w', p], capture_output=True, text=True).stderr
    os.unlink(p)
    return out

def errs(body, prelude):
    text = prelude + "\n".join(body) + "\n"
    out = run(text)
    return sorted(re.sub(r'^.*?error', 'error', l) for l in out.splitlines() if ' error' in l)

def prelude_for(bodies):
    """Typedef every pseudo-type gcc reports as unknown across the given bodies."""
    names, prelude = set(), ""
    for _ in range(8):
        new = set()
        for b in bodies:
            for m in UNKNOWN.finditer(run(prelude + "\n".join(b) + "\n")):
                if m.group(1) not in names:
                    new.add(m.group(1))
        if not new:
            break
        names |= new
        prelude = "".join(f"typedef char {n};\n" for n in sorted(names))
    return prelude

bdir, adir = sys.argv[1], sys.argv[2]
n_changed = n_ok = 0
rows = []
for bf in sorted(glob.glob(os.path.join(bdir, '*.c'))):
    af = os.path.join(adir, os.path.basename(bf))
    B, A = load(bf), load(af)
    for k in B:
        if B[k] == A[k]:
            continue
        n_changed += 1
        prelude = prelude_for([B[k], A[k]])
        eb, ea = errs(B[k], prelude), errs(A[k], prelude)
        gone = [e for e in eb if e not in ea]
        new = [e for e in ea if e not in eb]
        lab = [e for e in gone if 'used but not defined' in e]
        ok = bool(lab) and len(gone) == len(lab) and not new
        n_ok += ok
        rows.append((("OK   " if ok else "CHECK"), os.path.basename(bf), k, gone, new))
for r in rows:
    print(r[0], r[1], r[2], "gone=" + str(r[3]), "new=" + str(r[4]))
print(f"changed={n_changed} only-undefined-label-error-removed={n_ok}")
