#!/usr/bin/env python3
"""Classify every function whose `code` differs between <dir>/<n>.off.json (main) and <n>.on.json (fix).

unfold-rhs : the fix spills one or more calls into `vN = CALL;` statements; folding each back
             into its single read reproduces main's body exactly (v-numbers renamed by first use,
             declarations set aside), and in main every such call sat in the right-hand operand of
             an `&&` or `||` (the GH-684 class).
unfold-rhs+operand : as unfold-rhs once main's own `vX = E;` temporaries that the fix prints in
             place are folded into their (at most two) reads: an operand of the call that main had to
             spill because the folded call's cover crossed it is implied again once the call is a
             statement.
unfold-lhs : as above, but some refolded call sat on the LEFT of its operator, or under no
             `&&`/`||` at all (a wrong decline: must be zero).
Braces are compared away: an `if` body that gains the spill statement gains `{ }`.
other      : anything else (printed for review).
"""
import json, re, sys, glob, os, difflib, collections

def split(code):
    L = code.split('\n')
    if '{' not in L: return [], L
    i = L.index('{'); j = i + 1
    while j < len(L) and L[j].strip(): j += 1
    return L[i+1:j], L[:i+1] + L[j:]

def rename(lines):
    m = {}
    def r(x):
        if x.group(0) not in m: m[x.group(0)] = f'V{len(m)}'
        return m[x.group(0)]
    return [re.sub(r'\bv\d+\b', r, l) for l in lines]

def balanced_call(rhs):
    m = re.match(r'^[A-Za-z_]\w*(?:->\w+)*\(', rhs)
    if not m: return False
    d = 0
    for i, ch in enumerate(rhs):
        if ch == '(': d += 1
        elif ch == ')':
            d -= 1
            if d == 0: return i == len(rhs) - 1
    return False

SPILL = re.compile(r'^(\s*)(v\d+) = (.+);$')
DECL = re.compile(r'^\s*(.+?)\s+(v\d+)(?:\s*\[\d+\])?;')

def ndiff(x, y):
    return sum(1 for l in difflib.ndiff(x, y) if l[:1] in '+-')

def refold_once(body, i, decls):
    """Fold spill line i into the one read of its variable before the variable is next assigned."""
    m = SPILL.match(body[i])
    if not m or not balanced_call(m.group(3)): return []
    v, call = m.group(2), m.group(3)
    pat = re.compile(r'\b' + v + r'\b')
    reads = []
    for j in range(i + 1, len(body)):
        if re.match(r'^\s*' + v + r' = ', body[j]):
            reads += [(j, len(pat.findall(body[j].split('=', 1)[1])))]
            break
        n = len(pat.findall(body[j]))
        if n: reads.append((j, n))
    reads = [(j, n) for j, n in reads if n]
    if len(reads) != 1 or reads[0][1] != 1: return []
    j = reads[0][0]
    out = []
    forms = [call] + ([f'({decls[v]}){call}'] if v in decls else [])
    for text in forms:
        nb = list(body)
        nb[j] = pat.sub(lambda _: text, nb[j], count=1)
        del nb[i]
        out.append((nb, j - 1, text))
    return out

def refold(body, target, decls):
    """Greedily fold spills into their read while that brings the body closer to `target`."""
    body = list(body); folded = []
    while True:
        best = None; cur = ndiff(rename(target), rename(body))
        for i in range(len(body)):
            for nb, j, text in refold_once(body, i, decls):
                d = ndiff(rename(target), rename(nb))
                if d < cur and (best is None or d < best[0]): best = (d, nb, j, text)
        if best is None: return body, folded
        _, body, j, text = best
        folded = [(jj - (1 if jj > j else 0), t) for jj, t in folded] + [(j, text)]

def in_rhs(line, pos):
    depth = 0; mind = 0; blocked = False; i = pos - 1
    while i >= 0:
        ch = line[i]
        two = line[i-1:i+1] if i >= 1 else ''
        if ch == ')': depth += 1
        elif ch == '(':
            depth -= 1
            if depth < mind: mind = depth; blocked = False
        elif ch == ',' and depth == mind: blocked = True
        elif two in ('&&', '||') and depth == mind and not blocked: return True
        i -= 1
    return False

def rename_map(lines):
    m = {}
    for l in lines:
        for x in re.findall(r'\bv\d+\b', l):
            if x not in m: m[x] = f'V{len(m)}'
    return m

def nobrace(lines):
    return [re.sub(r' \{$', '', l) for l in lines if l.strip() not in ('{', '}')]

def inline_temps(body, target):
    """Fold main's own `vX = E;` temporaries into their reads (at most two, within the next three
    lines, before vX is assigned again) while that brings it closer to `target`."""
    body = list(body)
    while True:
        cur = ndiff(nobrace(rename(target)), nobrace(rename(body))); best = None
        for i, l in enumerate(body):
            m = SPILL.match(l)
            if not m: continue
            v, e = m.group(2), m.group(3)
            pat = re.compile(r'\b' + v + r'\b')
            if pat.search(e): continue
            end = next((k for k in range(i + 1, len(body)) if re.match(r'^\s*' + v + r' = ', body[k])), len(body))
            reads = [k for k in range(i + 1, end) if pat.search(body[k])]
            n = sum(len(pat.findall(body[k])) for k in reads)
            if not 1 <= n <= 2 or reads[-1] > i + 3: continue
            nb = [pat.sub(lambda _: e, x) if i < k < end else x for k, x in enumerate(body)]
            del nb[i]
            d = ndiff(nobrace(rename(target)), nobrace(rename(nb)))
            if d < cur and (best is None or d < best[0]): best = (d, nb)
        if best is None: return body
        body = best[1]

def classify(a, b):
    da, ba = split(a); db, bb = split(b)
    decls = {m.group(2): m.group(1) for m in map(DECL.match, db) if m}
    fb, folded = refold(bb, ba, decls)
    if not folded: return 'other', []
    cls = 'unfold-rhs'
    if nobrace(rename(fb)) != nobrace(rename(ba)):
        ba = inline_temps(ba, fb)
        if nobrace(rename(fb)) != nobrace(rename(ba)): return 'other', folded
        cls = 'unfold-rhs+operand'
    mp = rename_map(fb); rf = rename(fb)
    for j, text in folded:
        t = re.sub(r'\bv\d+\b', lambda x: mp.get(x.group(0), x.group(0)), text)
        p = rf[j].find(t)
        if p < 0 or not in_rhs(rf[j], p): return 'unfold-lhs', folded
    return cls, folded

if __name__ == "__main__":
    d = sys.argv[1]; show = sys.argv[2:]  # classes whose hunks to print (v-numbers renamed by first use)
    tot = collections.Counter(); rows = []
    for offp in sorted(glob.glob(f'{d}/*.off.json')):
        n = os.path.basename(offp)[:-len('.off.json')]
        A = {f['address_hex']: f for f in json.load(open(offp))['functions']}
        B = {f['address_hex']: f for f in json.load(open(f'{d}/{n}.on.json'))['functions']}
        c = collections.Counter(fns=len(A), missing=len(set(A) ^ set(B)))
        for k in A:
            a, b = A[k].get('code') or '', (B.get(k) or {}).get('code') or ''
            if a == b: continue
            cl, folded = classify(a, b); c['changed'] += 1; c[cl] += 1
            c['calls'] += len(folded)
            rows.append((n, k, A[k]['name'], cl, a, b))
        tot.update(c)
        print(f'{n:14s}', {x: y for x, y in c.items() if x != 'missing' or y})
    print('TOTAL', dict(tot))
    for n, k, name, cl, a, b in rows:
        if cl not in show: continue
        print(f'\n### {n} {k} {name} [{cl}]')
        _, ba = split(a); _, bb = split(b)
        for l in difflib.unified_diff(rename(ba), rename(bb), lineterm='', n=0):
            if not l.startswith(('+++', '---', '@@')): print('   ', l)
