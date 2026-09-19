"""How many call sites render FEWER arguments than the callee's own emitted
declaration takes -- the shape a C compiler rejects with "too few arguments".
Both arms, whole binary, over a directory of `<name>.{off,on}.json` dumps.

Usage: python3 underarity.py <dir-of-decompile-all-json>
"""
import collections, json, os, sys, types

HERE = os.path.dirname(os.path.abspath(__file__))
# The parsing helpers live in callsite-metrics.py, whose name is not an
# identifier; take everything above its `main` and exec it as a module.
src = open(os.path.join(HERE, 'callsite-metrics.py')).read().split("def main(")[0]
mod = types.ModuleType('cm')
exec(compile(src, 'callsite-metrics.py', 'exec'), mod.__dict__)

d = sys.argv[1]
names = sorted({f.rsplit('.', 2)[0] for f in os.listdir(d) if f.endswith('.json')})
tot = collections.Counter(); rows = []
for n in names:
    arms = {}
    for arm in ('off', 'on'):
        fns = json.load(open(f'{d}/{n}.{arm}.json'))['functions']
        decl = {}
        for f in fns:
            nm, k = mod.declared(f.get('code') or '')
            if nm is not None and k is not None:
                decl[nm] = k
        under = 0; usites = []
        for f in fns:
            code = f.get('code') or ''
            for callee, lst in mod.sites(code).items():
                k = decl.get(callee)
                if k is None or k == 0:
                    continue
                for site in lst:
                    if len(site) < k:
                        under += 1
                        usites.append((f.get('name'), callee, len(site), k))
        arms[arm] = (under, usites)
    tot['under_' + 'off'] += arms['off'][0]; tot['under_on'] += arms['on'][0]
    rows.append((n, arms['off'][0], arms['on'][0]))
    if arms['on'][0] > arms['off'][0]:
        seen = set(arms['off'][1])
        for s in arms['on'][1]:
            if s not in seen:
                print('  NEW', n, s)
for r in rows:
    print(f"{r[0]:42s} off={r[1]:5d} on={r[2]:5d}")
print("TOTAL under-arity sites: off", tot['under_off'], "on", tot['under_on'])
