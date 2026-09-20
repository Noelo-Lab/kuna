#!/usr/bin/env python3
"""flip-stmtdelta.py: per changed function: statement / call / store / goto / return / label counts off vs arm.
usage: stmtdelta.py <dir> <arm>   (dir holds <slice>.off.json and <slice>.<arm>.json)"""
import json, os, re, sys, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import importlib.util
spec = importlib.util.spec_from_file_location('cd', os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', 'protoorder', 'corpus-diff.py'))
cd = importlib.util.module_from_spec(spec); spec.loader.exec_module(cd)
CALL = re.compile(r'\b([A-Za-z_]\w*)\s*\(')
KW = set('if while for switch return sizeof do else char short int long float double void signed unsigned bool _Bool code struct union'.split())
def counts(code):
    s0, decls, body = cd.split(code)
    c = collections.Counter()
    for l in body:
        if l.endswith(';'): c['stmt'] += 1
        c['goto'] += len(re.findall(r'\bgoto\b', l))
        c['return'] += len(re.findall(r'\breturn\b', l))
        c['label'] += 1 if re.match(r'^\w+:$', l) else 0
        for m in CALL.finditer(re.sub(r'"(?:[^"\\]|\\.)*"', '""', l)):
            n = m.group(1)
            if n in KW or re.match(r'^(u?int\d+|undefined\d*|CONCAT\d+|SUB\d+|ZEXT\d+|SEXT\d+|struct_\w+)$', n): continue
            c['call:' + n] += 1
    c['decl'] = len(decls)
    return c
d, arm = sys.argv[1], sys.argv[2]
tot = collections.Counter(); rows = []
for fn in sorted(os.listdir(d)):
    if not fn.endswith('.off.json'): continue
    stem = fn[:-9]
    off = {f['address']: f for f in json.load(open(os.path.join(d, fn)))['functions']}
    on = {f['address']: f for f in json.load(open(os.path.join(d, f'{stem}.{arm}.json')))['functions']}
    for a, f0 in off.items():
        f1 = on.get(a)
        if not f1 or f0.get('code') == f1.get('code'): continue
        c0, c1 = counts(f0.get('code')), counts(f1.get('code'))
        delta = {k: c1[k] - c0[k] for k in set(c0) | set(c1) if c1[k] != c0[k]}
        sig0 = (f0.get('code') or '').splitlines()[:1]; sig1 = (f1.get('code') or '').splitlines()[:1]
        ar = lambda s: len([x for x in re.sub(r'.*?\((.*)\).*', r'\1', s[0] if s else '').split(',') if x.strip() and x.strip() != 'void'])
        if ar(sig0) != ar(sig1): delta['ARITY'] = ar(sig1) - ar(sig0)
        nv0 = len(f0.get('variables') or []); nv1 = len(f1.get('variables') or [])
        if nv0 != nv1: delta['vars[]'] = nv1 - nv0
        for k, v in delta.items(): tot[k.split(':')[0] if k.startswith('call:') else k] += v
        if any(k != 'decl' for k in delta):
            rows.append((stem, f0.get('name'), hex(a), delta))
for r in rows: print(*r)
print('TOTAL', dict(tot))
