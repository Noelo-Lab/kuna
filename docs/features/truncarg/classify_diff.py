"""Per-function before/after diff of `kuna decompile-all --json` pairs (<bin>.base.json / <bin>.fix.json).

Every changed line must be the insertion of an unsigned narrow cast at a call argument
(optionally replacing a signed narrow cast); anything else, and any change to a JSON field
other than `code`, is reported as OTHER.
"""
import json, sys, glob, os, re, difflib
O = sys.argv[1] if len(sys.argv) > 1 else 'corpus'
CAST = re.compile(r'\((unsigned short|unsigned char|uint2|uint1|ushort|uchar|byte)\)')
SCAST = re.compile(r'\((char|short|int1|int2)\)')
def norm(s):
    return re.sub(r'[\s()]', '', SCAST.sub('', CAST.sub('', s)))
tot_fn = tot_changed = tot_sites = 0
nretype = [0]
other = []
examples = []
per = []
for bf in sorted(glob.glob(O + '/*.base.json')):
    ff = bf.replace('.base.json', '.fix.json')
    if not os.path.exists(ff): continue
    try:
        b = json.load(open(bf)); f = json.load(open(ff))
    except Exception as e:
        print('LOADFAIL', bf, e); continue
    bm = {x.get('address'): x for x in b.get('functions', [])}
    fm = {x.get('address'): x for x in f.get('functions', [])}
    name = os.path.basename(bf).replace('.base.json', '')
    nfn = len(bm); nchg = 0; nsites = 0
    if set(bm) != set(fm):
        other.append((name, 'function set differs', len(set(bm) ^ set(fm))))
    for a in bm:
        if a not in fm: continue
        x, y = bm[a], fm[a]
        for k in set(x) | set(y):
            if k == 'code' or k in ('elapsed_ms', 'ms', 'time', 'seconds'): continue
            if x.get(k) != y.get(k):
                other.append((name, x.get('name'), 'field ' + k))
        cx, cy = x.get('code') or '', y.get('code') or ''
        if cx == cy: continue
        nchg += 1
        ol = cx.splitlines(); nl = cy.splitlines()
        sm = difflib.SequenceMatcher(None, ol, nl, autojunk=False)
        for tag, i1, i2, j1, j2 in sm.get_opcodes():
            if tag == 'equal': continue
            if tag != 'replace' or (i2 - i1) != (j2 - j1):
                other.append((name, x.get('name'), 'structural', ol[i1:i2], nl[j1:j2])); continue
            for o, n in zip(ol[i1:i2], nl[j1:j2]):
                casts = len(CAST.findall(n)) - len(CAST.findall(o))
                if casts > 0 and norm(n) == norm(o) and 0 <= len(SCAST.findall(o)) - len(SCAST.findall(n)) <= casts:
                    nsites += casts
                    nretype[0] += max(0, len(SCAST.findall(o)) - len(SCAST.findall(n)))
                    examples.append((name, x.get('name'), o.strip(), n.strip()))
                else:
                    other.append((name, x.get('name'), 'line', o, n))
    per.append((name, nfn, nchg, nsites))
    tot_fn += nfn; tot_changed += nchg; tot_sites += nsites
for p in per: print('%-45s fns=%5d changed=%4d cast_sites=%4d' % p)
print('TOTAL fns=%d changed_fns=%d cast_sites=%d retyped_existing_casts=%d other=%d' % (tot_fn, tot_changed, tot_sites, nretype[0], len(other)))
for o in other[:40]: print('OTHER', o)
json.dump({'per': per, 'examples': examples, 'other': other}, open(os.path.join(os.path.dirname(os.path.abspath(O)), 'classify.json'), 'w'), indent=1)
