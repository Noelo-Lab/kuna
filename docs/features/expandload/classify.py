#!/usr/bin/env python3
"""Classify every function whose `code` differs between <dir>/<n>.off.json (main) and <n>.on.json (fix).

L1 respell   : equal once each narrow-load spelling is one token: main's cast of the wider
               element `(T)p[k]` / `(T)*p` and the fix's narrow read `*(T *)&p[k]` / `*(T *)p`.
L2 renumber  : L1 after renaming v1..vN by first use in the body (declarations set aside).
L3 casts     : L2 after erasing scalar casts, spelling a narrow read `*(T *)&E` as `E` and
               ignoring signedness in type names: the narrow value reaches its uses without
               the casts main needed, and a same-width store through it is spelled unsigned.
L4 temps     : L3 with every register-temp name erased: the narrow value lives in its own
               temporary instead of sharing main's wider one (or is used directly).
other        : none of the above (printed for review).
"""
import json, re, sys, glob, os, difflib, collections
T = r'(?:unsigned |signed )?(?:char|short|int|long long|long|bool|int1|int2|int4|int8|uint1|uint2|uint4|uint8|undefined[1248]?|float|double)'
IDX = r'([A-Za-z_]\w*(?:->\w+|\.\w+)*)\[([^\[\]]+)\]'
fix_idx = re.compile(r'\*\(' + T + r' \*\)&' + IDX)
fix_der = re.compile(r'\*\(' + T + r' \*\)([A-Za-z_]\w*)\b(?!\s*[\[(+-])')
main_idx = re.compile(r'\(' + T + r'\)' + IDX)
main_der = re.compile(r'\(' + T + r'\)\*\s*([A-Za-z_]\w*)\b')
cast = re.compile(r'\(' + T + r'\)')
narrow_amp = re.compile(r'\*\(' + T + r' \*\)&')
def l1(s):
    s = fix_idx.sub(lambda m: f'N({m.group(1)}[{m.group(2)}])', s)
    s = main_idx.sub(lambda m: f'N({m.group(1)}[{m.group(2)}])', s)
    s = fix_der.sub(lambda m: f'N(*{m.group(1)})', s)
    return main_der.sub(lambda m: f'N(*{m.group(1)})', s)
def split(code):
    L = code.split('\n')
    if '{' not in L: return [], L
    i = L.index('{'); j = i + 1
    while j < len(L) and L[j].strip(): j += 1
    return L[i+1:j], L[:i+1] + L[j:]
def rename(body):
    m = {}
    def r(x):
        if x.group(0) not in m: m[x.group(0)] = f'V{len(m)}'
        return m[x.group(0)]
    return '\n'.join(re.sub(r'\bv\d+\b', r, l) for l in body)
def l3(s):
    s = narrow_amp.sub('', s); s = cast.sub('', s)
    s = re.sub(r'\b(?:unsigned|signed) (char|short|int|long)\b', r'\1', s)
    s = re.sub(r'\buint([1248])\b', r'int\1', s)
    return re.sub(r'N\((.*?)\)', r'\1', s)
def l4(s):
    s = re.sub(r'\bV\d+\b', 'V', s)
    # a temp read in place of the load it held: `V = E; ... f(V)` vs `f(E)` is left to review
    return s
def classify(a, b):
    if l1(a) == l1(b): return 'L1-respell'
    _, ba = split(a); _, bb = split(b)
    ra, rb = rename(ba), rename(bb)
    if l1(ra) == l1(rb): return 'L2-renumber'
    if l3(l1(ra)) == l3(l1(rb)): return 'L3-casts'
    if l4(l3(l1(ra))) == l4(l3(l1(rb))): return 'L4-temps'
    return 'other'
if __name__ == "__main__":
    d = sys.argv[1]; show = sys.argv[2:]  # classes to print
    tot = collections.Counter(); rows = []
    for offp in sorted(glob.glob(f'{d}/*.off.json')):
        n = os.path.basename(offp)[:-len('.off.json')]
        A = {f['address_hex']: f for f in json.load(open(offp))['functions']}
        B = {f['address_hex']: f for f in json.load(open(f'{d}/{n}.on.json'))['functions']}
        c = collections.Counter(fns=len(A), missing=len(set(A) ^ set(B)))
        for k in A:
            a, b = A[k]['code'] or '', (B.get(k) or {}).get('code') or ''
            if a == b: continue
            cl = classify(a, b); c['changed'] += 1; c[cl] += 1
            rows.append((n, k, A[k]['name'], cl, a, b))
        tot.update(c)
        print(f'{n:45s}', {x: y for x, y in c.items() if x != 'missing' or y})
    print('TOTAL', dict(tot))
    for n, k, name, cl, a, b in rows:
        if cl not in show: continue
        print(f'\n### {n} {k} {name} [{cl}]')
        _, ba = split(a); _, bb = split(b)
        for l in difflib.unified_diff(rename(ba).splitlines(), rename(bb).splitlines(), lineterm='', n=0):
            if not l.startswith(('+++', '---', '@@')): print('   ', l)
