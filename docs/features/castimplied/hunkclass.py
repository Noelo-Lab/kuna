"""Classify every removed cast between two decompile-all outputs.

Each changed line must be the old line with some integer-cast tokens deleted
(anything else is reported as OTHER). Each deleted token is classified by the
token in front of it: another cast (stacked inner), `return`, `=` (assignment),
`(`/`,` inside a call (call argument), else other."""
import re, sys, difflib, collections, pathlib
CAST = r'\((?:(?:unsigned |signed )?(?:char|short|int|long)|bool|u?int[1248])\)'
TOK = re.compile(CAST + r'|[A-Za-z_]\w*|0x[0-9a-fA-F]+|\d+|==|!=|<=|>=|->|&&|\|\||\S')
ISCAST = re.compile('^' + CAST + '$')
def toks(l): return TOK.findall(l)
def classify(ts, i):
    j = i - 1
    prev = ts[j] if j >= 0 else ''
    if ISCAST.match(prev): return 'stacked-inner'
    if prev == 'return': return 'return'
    if prev == '=': return 'assign'
    if prev in ('(', ','):
        depth = 0
        for k in range(i - 1, -1, -1):
            t = ts[k]
            if t == ')': depth += 1
            elif t == '(':
                if depth == 0:
                    return 'call-arg' if k > 0 and re.match(r'^[A-Za-z_]\w*$', ts[k-1]) and ts[k-1] not in ('if','while','for','switch','return','sizeof') else 'paren/comma'
                depth -= 1
        return 'paren/comma'
    return 'other:' + prev
def pairs(a_path, b_path):
    A = a_path.read_text(errors='replace').split('\n'); B = b_path.read_text(errors='replace').split('\n')
    sm = difflib.SequenceMatcher(None, A, B, autojunk=False)
    for tag, i1, i2, j1, j2 in sm.get_opcodes():
        if tag == 'equal': continue
        if tag == 'replace' and i2 - i1 == j2 - j1:
            yield from zip(A[i1:i2], B[j1:j2])
        else:
            yield ('\n'.join(A[i1:i2]), '\n'.join(B[j1:j2]))
cls = collections.Counter(); other = []; ex = collections.defaultdict(list); nfiles = 0
base, new = map(pathlib.Path, sys.argv[1:3])
for f in sorted(new.rglob('*.c')):
    b = base / f.relative_to(new)
    if not b.exists(): continue
    nfiles += 1
    for o, n in pairs(b, f):
        to, tn = toks(o), toks(n)
        sm = difflib.SequenceMatcher(None, to, tn, autojunk=False)
        ok = True; removed = []
        for tag, i1, i2, j1, j2 in sm.get_opcodes():
            if tag == 'equal': continue
            if tag == 'delete' and all(ISCAST.match(t) or t in '()' for t in to[i1:i2]) and any(ISCAST.match(t) for t in to[i1:i2]):
                removed += [k for k in range(i1, i2) if ISCAST.match(to[k])]
            elif all(t in '()' for t in to[i1:i2] + tn[j1:j2]):
                continue
            else:
                ok = False
        if not ok or not removed:
            other.append((str(f.relative_to(new)), o, n)); continue
        for k in removed:
            c = classify(to, k); cls[c] += 1
            if len(ex[c]) < 3: ex[c].append((str(f.relative_to(new)), o.strip()[:120], n.strip()[:120]))
print(f'files {nfiles}  removed casts by class: {dict(cls)}  total {sum(cls.values())}  OTHER lines {len(other)}')
for c, e in ex.items():
    print('==', c)
    for x in e: print('   ', x[0], '|', x[1], '=>', x[2])
for x in other[:10]: print('OTHER', x)
