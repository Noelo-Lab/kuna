"""Two-arm whole-corpus frame check: split stack objects, lost named locals, extra call arguments.

Usage: framecheck.py <dump-dir-A> <dump-dir-B> <keys-file> <comma-separated row names>
Each dump dir holds one `kuna decompile-all` output per binary, named <opt>__<project>__<bin>.c.
"""
import sys, os, json, collections, re
sys.path.insert(0, os.path.dirname(__file__))
from framecheck_lib import analyze, calls, SIZES, splits
A, B, keysfile = sys.argv[1], sys.argv[2], sys.argv[3]
names = sys.argv[4].split(',') if len(sys.argv) > 4 and sys.argv[4] else []
keys = [k.strip() for k in open(keysfile) if k.strip()]
tot = collections.Counter(); rows = []
extra_a = collections.Counter(); extra_b = collections.Counter(); extra_where = collections.defaultdict(list)
for k in keys:
    fa = f"{A}/{k.replace('::','__')}.c"; fb = f"{B}/{k.replace('::','__')}.c"
    if not (os.path.exists(fa) and os.path.exists(fb)): tot['missing'] += 1; continue
    ta, tb = open(fa).read(), open(fb).read()
    ra, pa = analyze(ta); rb, pb = analyze(tb)
    tot['bins'] += 1
    for addr in ra.keys() & rb.keys():
        a, b = ra[addr], rb[addr]
        tot['funcs'] += 1
        if a['body'] != b['body']: tot['changed'] += 1
        ua, ub = set(a['under']), set(b['under'])
        tot['under_a'] += len(ua); tot['under_b'] += len(ub)
        new_under = ub - ua
        sp = splits(a['body'], b['body']) if a['body'] != b['body'] else []
        tot['split_funcs'] += bool(sp)
        lost = {t: a['named'][t] - b['named'].get(t, 0) for t in a['named'] if a['named'][t] > b['named'].get(t, 0)}
        if sp or lost:
            called = sorted(set(re.findall(r'(?<![\w.>])(' + '|'.join(names) + r')\(', b['body']))) if names else []
            rows.append(dict(key=k, addr=addr, name=b['name'], split=sp, new_cast_on_small=sorted(new_under), lost=lost, calls=called))
    # extra args: reference = B's declared protos (superset)
    ca, wa = calls(ra, pb); cb, wb = calls(rb, pb)
    for n, c in ca.items(): extra_a[n] += c
    for n, c in cb.items(): extra_b[n] += c; extra_where[n] += [f"{k}@{x}" for x in wb[n]]
print(json.dumps(dict(totals=tot), indent=None))
print(f"functions where a base stack object >=16 bytes is SPLIT, or a named-aggregate local is LOST: {len(rows)}")
for r in rows: print(json.dumps(r))
print("EXTRA-ARG call sites (args > declared params, reference = B's declarations):")
print(f"  total A={sum(extra_a.values())} B={sum(extra_b.values())}")
for n in sorted(set(extra_a) | set(extra_b), key=lambda n: -(extra_b[n])):
    if extra_a[n] != extra_b[n] or n in names:
        print(f"  {n:24s} A={extra_a[n]:5d} B={extra_b[n]:5d}  {' '.join(extra_where[n][:4])}")
