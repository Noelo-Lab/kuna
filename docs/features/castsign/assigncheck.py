"""Re-check every `lhs = (T)rhs;` castsign turned into `lhs = rhs;`.

castimplied drops that cast for a re-signed `lhs` only when `lhs` is declared an
integer as wide as `T` and `rhs` is an integer: C's conversion of `rhs` to the
declaration then yields the bits `(T)rhs` followed by the assignment did.  This
reads the printed declarations and flags anything else (a pointer or float
`rhs`, a width mismatch).
  python3 assigncheck.py <off-dir> <on-dir>
"""
import collections
import difflib
import pathlib
import re
import sys

INT = r'(?:unsigned )?(?:char|short|int|long)|bool|u?int[1248]'
W = {'char': 1, 'short': 2, 'int': 4, 'long': 8, 'bool': 1}


def width(s):
    s = (s or '').replace('unsigned ', '').strip()
    m = re.match(r'^u?int([1248])$', s)
    return int(m.group(1)) if m else W.get(s)


def fmap(lines):
    out, cur = {}, ({}, {})
    for i, line in enumerate(lines):
        if line.startswith('// Function:'):
            hdr = lines[i + 1] if i + 1 < len(lines) else ''
            params = {}
            m = re.search(r'\((.*)\)', hdr)
            if m:
                for p in m.group(1).split(','):
                    pm = re.match(r'\s*(.*?)\s*\**\s*(\w+)\s*$', p)
                    if pm:
                        t = p.strip()
                        params[pm.group(2)] = t[:len(t) - len(pm.group(2))].strip()
            locs, j = {}, i + 3
            while j < len(lines) and lines[j].strip():
                dm = re.match(r'^  ([A-Za-z_][\w ]*?(?: ?\*+)?) ?(\w+)( \[\d+\])?;( //.*)?$', lines[j])
                if dm and '=' not in dm.group(1):
                    locs[dm.group(2)] = dm.group(1).strip()
                j += 1
            cur = (params, locs)
        out[i] = cur
    return out


stats, flagged = collections.Counter(), []
off, on = map(pathlib.Path, sys.argv[1:3])
for f in sorted(on.rglob('*.c')):
    A = (off / f.relative_to(on)).read_text(errors='replace').split('\n')
    B = f.read_text(errors='replace').split('\n')
    fb = fmap(B)
    sm = difflib.SequenceMatcher(None, A, B, autojunk=False)
    for tag, i1, i2, j1, j2 in sm.get_opcodes():
        if tag != 'replace' or i2 - i1 != j2 - j1:
            continue
        for k in range(i2 - i1):
            m = re.match(r'\s*(\w+) = \((' + INT + r')\)(\w+);( //.*)?$', A[i1 + k])
            n = re.match(r'\s*(\w+) = (\w+);( //.*)?$', B[j1 + k])
            if not (m and n and m.group(1) == n.group(1) and m.group(3) == n.group(2)):
                continue
            lhs, t, rhs = m.group(1), m.group(2), m.group(3)
            params, locs = fb.get(j1 + k) or ({}, {})
            dl, dr = locs.get(lhs) or params.get(lhs), locs.get(rhs) or params.get(rhs)
            stats['lhs = (T)rhs dropped'] += 1
            if width(dl) != width(t):
                flagged.append(('WIDTH', str(f.relative_to(on)), A[i1 + k].strip(), dl, dr))
            elif dr is not None and width(dr) is None:
                flagged.append(('RHS-NONINT', str(f.relative_to(on)), A[i1 + k].strip(), dl, dr))
            else:
                stats['ok' if dr is not None else 'ok (rhs a global)'] += 1
print(dict(stats))
print('flagged', len(flagged))
for x in flagged[:25]:
    print(x)
