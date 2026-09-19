#!/usr/bin/env python3
"""Compare two `typesweep --baseline-only` runs (main build vs fix build) function by function:
type_match, perfect, and the tp/fp/fn and variable-count counters beside it.
Usage: tscompare.py <main-out-dir> <fix-out-dir>"""
import json, sys, collections
A = json.load(open(f'{sys.argv[1]}/rows.json')); B = json.load(open(f'{sys.argv[2]}/rows.json'))
proj = collections.defaultdict(collections.Counter); moved = []
for sl in sorted(set(A) | set(B)):
    a, b = (A.get(sl) or {}).get('base') or {}, (B.get(sl) or {}).get('base') or {}
    p = sl.split('::')[0]
    av, bv = a.get('values', {}), b.get('values', {})
    for fn in sorted(set(av) | set(bv)):
        x, y = av.get(fn), bv.get(fn)
        c = proj[p]; c['n'] += 1
        if x is None or y is None: c['missing'] += 1; continue
        c['perfect_off'] += x == 1.0; c['perfect_on'] += y == 1.0
        c['sum_off'] += x; c['sum_on'] += y
        if y > x: c['improved'] += 1; moved.append((sl, fn, x, y))
        if y < x: c['worse'] += 1; moved.append((sl, fn, x, y))
        ma, mb = a['meta'][fn], b['meta'][fn]
        for k in ('tp', 'fp', 'fn'):
            c[k + '_off'] += ma[k]; c[k + '_on'] += mb[k]
        c['nvars_off'] += a['nvars'][fn]; c['nvars_on'] += b['nvars'][fn]
tot = collections.Counter()
for p in sorted(proj):
    c = proj[p]; tot.update(c)
    print(f"{p:10s} n={c['n']} perfect {c['perfect_off']}->{c['perfect_on']} mean {c['sum_off']/max(c['n'],1):.4f}->{c['sum_on']/max(c['n'],1):.4f} improved={c['improved']} worse={c['worse']}")
print(f"POOLED slices={len(set(A)|set(B))} n={tot['n']} missing={tot['missing']} perfect {tot['perfect_off']}->{tot['perfect_on']} "
      f"agg {tot['sum_off']:.2f}->{tot['sum_on']:.2f} improved={tot['improved']} worse={tot['worse']} "
      f"tp {tot['tp_off']}->{tot['tp_on']} fp {tot['fp_off']}->{tot['fp_on']} fn {tot['fn_off']}->{tot['fn_on']} nvars {tot['nvars_off']}->{tot['nvars_on']}")
for m in moved: print('MOVED', *m)
