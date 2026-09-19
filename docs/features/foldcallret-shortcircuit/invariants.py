#!/usr/bin/env python3
"""Per-function invariants of the decbench-scored surface, main vs fix: variables[] as a multiset of
(kind, type, stack offset, size, arg index) with names set aside, the argument rows, the signature
line, and the calls made (callee names counted in the body)."""
import json, re, sys, glob, os, collections
d = sys.argv[1]
tot = collections.Counter()
for offp in sorted(glob.glob(f'{d}/*.off.json')):
    n = os.path.basename(offp)[:-len('.off.json')]
    A = {f['address_hex']: f for f in json.load(open(offp))['functions']}
    B = {f['address_hex']: f for f in json.load(open(f'{d}/{n}.on.json'))['functions']}
    for k, a in A.items():
        b = B.get(k)
        if b is None: tot['missing'] += 1; continue
        tot['fns'] += 1
        key = lambda f: sorted((v['kind'], v['type'], str(v['stack_offset']), v['size'], str(v['arg_index'])) for v in f.get('variables') or [])
        if key(a) == key(b): tot['vars_same'] += 1
        else: tot['vars_diff'] += 1; print('VARS', n, k, a['name'])
        args = lambda f: sum(1 for v in f.get('variables') or [] if v['kind'] == 'arg')
        tot['args_off'] += args(a); tot['args_on'] += args(b)
        ca, cb = a.get('code') or '', b.get('code') or ''
        if ca.split('\n', 1)[0] != cb.split('\n', 1)[0]: tot['sig_diff'] += 1; print('SIG', n, k)
        calls = lambda c: collections.Counter(re.findall(r'\b([A-Za-z_]\w*)\(', c))
        if calls(ca) != calls(cb): tot['calls_diff'] += 1; print('CALLS', n, k)
        tot['goto_off'] += ca.count('goto '); tot['goto_on'] += cb.count('goto ')
print(dict(tot))
