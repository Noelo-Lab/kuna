"""Classify every changed line between two decompile-all outputs (castsign off -> on).

Per function, a variable is FLIPPED when its declaration line changed from
`unsigned T vN;` to `T vN;` (or `uintN` to `intN`) with nothing else on the line
changed.  Every other
changed line must be the old line with cast tokens deleted, each deleted cast
being one of
  sign-cast   `(T)vN` on a flipped vN, T its new declared type (the cast the new
              declaration makes a no-op),
  def-widen   an integer cast at the top of the right side of `vN = ...;` where
              vN is a local declared an integer of the cast's own width (its
              declaration re-signed here or by `signedness`): C's conversion on
              assignment yields the same bits, since a conversion to an N-bit
              integer depends only on the value modulo 2^N.
Anything else is OTHER, and OTHER must be empty.

  python3 hunkclass.py <off-dir> <on-dir>
"""
import collections
import difflib
import pathlib
import re
import sys

CAST = r'\((?:(?:unsigned |signed )?(?:char|short|int|long)|bool|u?int[1248])\)'
TOK = re.compile(CAST + r'|[A-Za-z_]\w*|0x[0-9a-fA-F]+|\d+|==|!=|<=|>=|->|&&|\|\||\S')
ISCAST = re.compile('^' + CAST + '$')
DECL = re.compile(r'^(\s*)(unsigned )?((?:char|short|int|long|u?int[1248]))( v\d+;.*)$')
FN = re.compile(r'^// Function: .* @ (0x[0-9a-f]+)$')


def functions(path):
    out, cur, addr = {}, [], None
    for line in path.read_text(errors='replace').split('\n'):
        m = FN.match(line)
        if m:
            if addr:
                out[addr] = cur
            addr, cur = m.group(1), []
        cur.append(line)
    if addr:
        out[addr] = cur
    return out


def pairs(a, b):
    sm = difflib.SequenceMatcher(None, a, b, autojunk=False)
    for tag, i1, i2, j1, j2 in sm.get_opcodes():
        if tag == 'equal':
            continue
        if tag == 'replace' and i2 - i1 == j2 - j1:
            yield from zip(a[i1:i2], b[j1:j2])
        else:
            yield ('\n'.join(a[i1:i2]), '\n'.join(b[j1:j2]))


def flipped_decl(o, n):
    mo, mn = DECL.match(o), DECL.match(n)
    if not (mo and mn) or mo.group(1) != mn.group(1) or mo.group(4) != mn.group(4):
        return None
    was = (mo.group(2) or '') + mo.group(3)
    now = (mn.group(2) or '') + mn.group(3)
    if was not in ('unsigned ' + now, 'u' + now):
        return None
    return mn.group(4).split(';')[0].strip(), now


WIDTH = {'char': 1, 'short': 2, 'int': 4, 'long': 8}


def width(t):
    t = t.strip('()').replace('unsigned ', '').replace('signed ', '')
    m = re.match(r'^u?int([1248])$', t)
    return int(m.group(1)) if m else WIDTH.get(t)


def classify(to, k, flips, widths):
    nxt = to[k + 1] if k + 1 < len(to) else ''
    if nxt in flips:
        return 'sign-cast'
    for j in range(k - 1, -1, -1):
        if ISCAST.match(to[j]) or to[j] == '(':
            continue
        if to[j] == '=' and j >= 1 and widths.get(to[j - 1]) == width(to[k]) and to[j - 1] in widths:
            return 'def-widen'
        break
    return None


def local_widths(lines):
    out = {}
    for line in lines:
        m = DECL.match(line)
        if m:
            out[m.group(4).split(';')[0].strip()] = width(m.group(3))
    return out


def main():
    off, on = map(pathlib.Path, sys.argv[1:3])
    cls = collections.Counter()
    ex = collections.defaultdict(list)
    other = []
    nfiles = nfuncs = 0
    for f in sorted(on.rglob('*.c')):
        b = off / f.relative_to(on)
        if not b.exists():
            continue
        nfiles += 1
        A, B = functions(b), functions(f)
        for addr in sorted(set(A) | set(B)):
            a, bb = A.get(addr, []), B.get(addr, [])
            if a == bb:
                continue
            nfuncs += 1
            hunks = list(pairs(a, bb))
            flips = {}
            for o, n in hunks:
                d = flipped_decl(o, n)
                if d:
                    flips[d[0]] = d[1]
            widths = local_widths(bb)
            where = f'{f.relative_to(on)}@{addr}'
            for o, n in hunks:
                if flipped_decl(o, n):
                    cls['decl'] += 1
                    continue
                to, tn = TOK.findall(o), TOK.findall(n)
                sm = difflib.SequenceMatcher(None, to, tn, autojunk=False)
                ok, kinds = True, []
                for tag, i1, i2, j1, j2 in sm.get_opcodes():
                    if tag == 'equal':
                        continue
                    if tag == 'delete':
                        for k in range(i1, i2):
                            if to[k] in '()':
                                continue
                            c = classify(to, k, flips, widths) if ISCAST.match(to[k]) else None
                            if c is None:
                                ok = False
                            else:
                                kinds.append(c)
                    elif all(t in '()' for t in to[i1:i2] + tn[j1:j2]):
                        continue
                    else:
                        ok = False
                if not ok or not kinds:
                    other.append((where, o, n))
                    continue
                for c in kinds:
                    cls[c] += 1
                    if len(ex[c]) < 3:
                        ex[c].append((where, o.strip()[:110], n.strip()[:110]))
    print(f'files {nfiles}  functions changed {nfuncs}  by class {dict(cls)}  OTHER lines {len(other)}')
    for c, e in ex.items():
        print('==', c)
        for x in e:
            print('   ', x[0], '|', x[1], '=>', x[2])
    for x in other[:20]:
        print('OTHER', x)


if __name__ == '__main__':
    main()
