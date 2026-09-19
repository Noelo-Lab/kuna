#!/usr/bin/env python3
"""After the ledger converges, no emitted layout may be contained in another."""
import json, re, subprocess, sys, os
KUNA = sys.argv[1]
MEM = re.compile(r'^\s{4}(.*?)([A-Za-z_]\w*)\s*(\[(\d+)\])?;\s*$')
def claimed(defn):
    out = []
    for ln in defn.splitlines():
        m = MEM.match(ln)
        if not m:
            continue
        ty, nm, arr = m.group(1).strip(), m.group(2), m.group(3)
        if arr or '_0x' not in nm:
            continue                      # filler members are not claims
        out.append((int(nm.split('_0x')[1], 16), ty))
    return frozenset(out)
# The ledger's own rule (ledger.rs `answers_for`): a container that says strictly
# more may answer for a layout it contains only while it claims at most
# CLAIM_GROWTH times the fields and is at most SIZE_GROWTH times the size, over a
# layout claiming at least MIN_CLAIMS fields of which one is a typed pointer, or at
# least MIN_UNTYPED_CLAIMS without one. A pointer to code, `void` or an undefined
# type is not typed; a layout whose every claim past offset 0 is such a pointer
# is a table and answered only by its own shape; no structure answers with a
# member inside the alignment padding a layout leaves between two of its claims;
# a layout with no typed pointer is answered only by a structure of its own size
# that claims nothing past its last field. A contained layout outside those rules
# keeps its own name by design, so it is not a convergence residual.
CLAIM_GROWTH = 2
SIZE_GROWTH = 4
MIN_CLAIMS = 2
MIN_UNTYPED_CLAIMS = 3
OPAQUE = re.compile(r'^(code|void|undefined\d*)$')


def opaque(t):
    return '*' in t and bool(OPAQUE.match(t.replace('*', '').strip()))


WIDTH = {'char': 1, 'bool': 1, 'uchar': 1, 'short': 2, 'ushort': 2, 'int': 4, 'uint': 4,
         'float': 4, 'long': 8, 'ulong': 8, 'double': 8}


def width(t):
    """Bytes a member of this x86-64 type spelling occupies."""
    if '*' in t:
        return 8
    words = t.replace('unsigned', '').replace('signed', '').split()
    last = words[-1] if words else 'int'
    m = re.match(r'^(?:u?int|undefined)(\d+)$', last)
    return int(m.group(1)) if m else WIDTH.get(last, 1)


def padding(claims):
    """The `[lo, hi)` alignment padding a layout leaves between two claims."""
    ws = sorted((o, width(t)) for o, t in claims)
    out = []
    for (o1, w1), (o2, w2) in zip(ws, ws[1:]):
        lo, a = o1 + w1, (w2 if w2 in (1, 2, 4, 8) else 1)
        hi = min(o2, (lo + a - 1) // a * a)
        if lo < hi:
            out.append((lo, hi))
    return out


def answers_for(big, small):
    """Would the ledger have handed `big` out to a reader who measured `small`?"""
    (bsize, bclaims), (ssize, sclaims) = big, small
    if bsize < ssize or not sclaims <= bclaims:
        return False
    if bsize == ssize and len(bclaims) == len(sclaims):
        return False                      # the same shape: one name already
    over = lambda lo, hi: any(o < hi and o + width(t) > lo for o, t in bclaims)
    if any(over(lo, hi) for lo, hi in padding(sclaims)):
        return False
    typed = any('*' in t and not opaque(t) for _, t in sclaims)
    slots = [t for o, t in sclaims if o != 0]
    table = bool(slots) and all(opaque(t) for t in slots)
    end = max((o + width(t) for o, t in sclaims), default=0)
    evidence = (not table
                and (len(sclaims) >= MIN_UNTYPED_CLAIMS or (len(sclaims) >= MIN_CLAIMS and typed))
                and (typed or (bsize == ssize and not over(end, bsize))))
    return (evidence and bsize <= ssize * SIZE_GROWTH
            and len(bclaims) <= len(sclaims) * CLAIM_GROWTH)

tot = 0
for b in sys.argv[2:]:
    tag = b.split('/')[-4] + '/' + os.path.basename(b)
    p = subprocess.run([KUNA, 'decompile-all', b, '--option', 'structsynth', 'param',
                        '--option', 'structdefs', 'on', '--json'], capture_output=True, text=True)
    d = json.loads(p.stdout)
    st = {}
    for f in d['functions']:
        for t in f.get('types', []):
            if t['name'].startswith('struct_'):
                st[t['name']] = (t['size'], claimed(t['definition']))
    bad = [(a, c) for a in st for c in st
           if a != c and answers_for(st[c], st[a])]
    tot += len(bad)
    print(f'{tag:14s} emitted={len(st):4d} residual_contained={len(bad):3d} {bad[:3]}')
print('TOTAL residual', tot)
