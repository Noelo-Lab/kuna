import sys, re, collections
from pathlib import Path
d = Path(sys.argv[1])
cat = collections.Counter(); ex = collections.defaultdict(list); ptrsets = collections.Counter()
def kind(t):
    if t.endswith('*'): return 'ptr'
    if t in ('int1','int2','int4','int8','char'): return 'sint'
    if t in ('uint1','uint2','uint4','uint8','bool'): return 'uint'
    if t.startswith('xunknown') or t.startswith('undefined') or t == '-': return 'unk'
    if t.startswith('float'): return 'float'
    return 'other:' + t
for f in sorted(d.glob('*.ft')):
    for L in f.read_text().splitlines():
        if not L.startswith('FT'): continue
        _, fa, base, off, w, decl, acc = L.split('\t', 6)
        accs = [a for a in acc.split(' | ')]
        types = [re.match(r'([^\[]*)\[', a).group(1) for a in accs]
        kinds = [kind(t) for t in types]
        ptrs = sorted(set(t for t in types if t.endswith('*')))
        spec = sorted(set(t for t in ptrs if t not in ('void*', 'xunknown1*', 'VOID*', 'UNKNOWN*') and not t.startswith('xunknown')))
        dk = kind(decl)
        if not ptrs: c = 'noptr'
        elif dk == 'ptr' and len(set(ptrs)) == 1: c = 'ptr-agree'
        elif dk == 'ptr': c = f'ptr-decl-mixed(spec={len(spec)})'
        else: c = f'{dk}-decl-with-ptr(spec={len(spec)})'
        cat[c] += 1
        if c != 'noptr' and c != 'ptr-agree':
            ptrsets[(decl, tuple(ptrs))] += 1
            if len(ex[c]) < 8: ex[c].append(f'{f.stem} {fa} off={off} decl={decl} :: {acc[:260]}')
for k, v in cat.most_common(): print(f'{v:6d} {k}')
print('--- (decl, pointer set) most common')
for k, v in ptrsets.most_common(40): print(f'{v:5d} {k}')
for k in ex:
    print('==', k); [print('   ', e) for e in ex[k]]
