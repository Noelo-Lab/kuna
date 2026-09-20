#!/usr/bin/env python3
"""flip-ndiff.py <dir> <slice> <addr> <arm>: diff of one function's body after the corpus-diff normalizers
(names, casts, pointer-arith, member-reach, literal), printing the RAW lines of the hunks that survive."""
import json, sys, difflib, os, re, importlib.util
spec = importlib.util.spec_from_file_location('cd', os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', 'protoorder', 'corpus-diff.py'))
cd = importlib.util.module_from_spec(spec); spec.loader.exec_module(cd)
d, s, a, arm = sys.argv[1:5]; a = int(a, 16)
g = lambda arm: {f['address']: f for f in json.load(open(f'{d}/{s}.{arm}.json'))['functions']}[a]['code']
c0, c1 = g('off'), g(arm)
ptr = 4 if ('chibios' in s or 'freertos' in s or 'nuttx' in s or 'betaflight' in s) else 8
z0, z1 = cd.pointee_sizes(c0, ptr), cd.pointee_sizes(c1, ptr)
def body(c):
    out = []
    for l in (c or '').splitlines()[1:]:
        x = re.sub(r'\s+', ' ', re.sub(r'//.*$', '', l)).strip()
        if x and not cd.DECL.match(x): out.append(x)
    return out
b0, b1 = body(c0), body(c1)
n0 = [cd.names(cd.merged(cd.literal(cd.member_reach(x, z0)))) for x in b0]
n1 = [cd.names(cd.merged(cd.literal(cd.member_reach(x, z1)))) for x in b1]
def bag(l):
    l = cd.names(cd.casts(l))
    l = re.sub(r'\*\s*&', '', l)
    toks = re.findall(r'0x[0-9a-fA-F]+|\d+|\w+|->|<<|>>|<=|>=|==|!=|&&|\|\||[-+*/%&|^<>=!~?:,;\[\]]', l)
    toks = [str(int(t, 16)) if t.startswith('0x') else t for t in toks]
    return sorted(toks)
REORDER_OK = os.environ.get('SHOWALL') is None
sm = difflib.SequenceMatcher(a=n0, b=n1, autojunk=False)
for tag, i1, i2, j1, j2 in sm.get_opcodes():
    if tag == 'equal': continue
    if REORDER_OK and tag == 'replace' and (i2 - i1) == (j2 - j1) and all(bag(b0[i1+k]) == bag(b1[j1+k]) for k in range(i2 - i1)):
        print(f'@@ reorder-only off[{i1}:{i2}]'); continue
    print(f'@@ {tag} off[{i1}:{i2}] on[{j1}:{j2}]')
    for x in b0[max(0,i1-1):i1]: print('  ', x)
    for x in b0[i1:i2]: print(' -', x)
    for x in b1[j1:j2]: print(' +', x)
    for x in b0[i2:i2+1]: print('  ', x)
