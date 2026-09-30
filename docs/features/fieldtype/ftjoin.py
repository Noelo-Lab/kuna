import sys, re, collections
from pathlib import Path
def load(d):
    out = {}
    for f in d.glob('*.ft'):
        for L in f.read_text().splitlines():
            if not L.startswith('FT'): continue
            _, fa, b, off, w, decl, acc = L.split('\t', 6)
            out.setdefault((f.stem, fa, b, off), (decl, acc))
    return out
if __name__ == "__main__":
    base, new = Path(sys.argv[1]), Path(sys.argv[2])
    B, N = load(base), load(new)
    changed = [(k, B[k][0], N[k][0], N[k][1]) for k in N if k in B and B[k][0] != N[k][0]]
    print('changed fields', len(changed), 'of', len(N), '(only-in-new', len(set(N)-set(B)), 'only-in-base', len(set(B)-set(N)), ')')
    def ev(acc):
        tags = set()
        for a in acc.split(' | '):
            m = re.match(r'([^\[]*)\[(.*)\]$', a)
            if not m: continue
            t, u = m.group(1), m.group(2)
            if not t.endswith('*'): continue
            for x in u.split(','):
                if x.startswith('deref'): tags.add('deref')
                elif x.startswith('callL'): tags.add('declared')
                elif x == 'callee': tags.add('callee')
                elif x.startswith('call'): tags.add('call-unlocked')
                elif x == 'store': tags.add('stored-or-storeaddr')
                elif x: tags.add(x)
            if not u: tags.add('store-value')
        return tags
    c = collections.Counter()
    for k, o, n, acc in changed:
        c[(o, n)] += 1
    for k, v in c.most_common(30): print(v, k)
    if len(sys.argv) > 3:
        want = set(sys.argv[3].split(','))
        for k, o, n, acc in changed:
            stem, fa = k[0], k[1]
            tag = stem.split('__')[0] + '/' + stem.split('__')[2] + '@' + fa
            if tag in want: print(tag, 'off', k[3], o, '->', n, '::', sorted(ev(acc)), '::', acc[:300])
