#!/usr/bin/env python3
"""Scan `kuna decompile-all` C output for orphan gotos: an emitted `goto label_X;`
whose `label_X:` never appears in the same function body (invalid C).

Usage: orphan_scan.py <out.c> [out.c ...]
Prints `<file>\t<function>\t<label>` per orphan plus a SUMMARY line.
"""
import re, sys

MARK = re.compile(r'^// Function: (\S+) @ (0x[0-9a-fA-F]+)')
GOTO = re.compile(r'\bgoto\s+([A-Za-z_]\w*)\s*;')
LABEL = re.compile(r'^\s*([A-Za-z_]\w*)\s*:(?!:)')

def functions(text):
    name = addr = None
    body = []
    for ln in text.splitlines():
        m = MARK.match(ln)
        if m:
            if name is not None:
                yield name, addr, body
            name, addr, body = m.group(1), m.group(2), []
        else:
            body.append(ln)
    if name is not None:
        yield name, addr, body

def scan(path):
    text = open(path, errors='replace').read()
    nfunc = 0
    orphans = []
    for name, addr, body in functions(text):
        nfunc += 1
        labels = {m.group(1) for ln in body for m in [LABEL.match(ln)] if m}
        gotos = {g.group(1) for ln in body for g in GOTO.finditer(ln)}
        for g in sorted(gotos - labels - {'break', 'continue'}):
            orphans.append((name, addr, g))
    return nfunc, orphans

def main():
    tf = to = tg = 0
    for path in sys.argv[1:]:
        nfunc, orphans = scan(path)
        tf += nfunc
        seen = set()
        for name, addr, lab in orphans:
            print(f"{path}\t{name}@{addr}\t{lab}")
            seen.add(name)
        to += len(seen); tg += len(orphans)
    print(f"SUMMARY files={len(sys.argv)-1} functions={tf} orphan_functions={to} orphan_gotos={tg}")

if __name__ == '__main__':
    main()
