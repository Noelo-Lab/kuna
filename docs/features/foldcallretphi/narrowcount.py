"""Count folds that drop a narrowing, and uncast `f(..) == -1` compares, between two decompile-all dumps.

Usage: narrowcount.py <dir>...   where each <dir> holds <tag>-off.json / <tag>-on.json pairs
(`kuna decompile-all <bin> --json`, with and without `--option foldcallretphi off`).

A site counts when, in a changed function, a callee's `vN = f(` assignment count drops, the callee is
printed with an 8-byte return, the spilled local was declared 1-4 bytes, and no remaining call text
carries a narrowing cast.
"""
import collections
import json
import os
import re
import sys

ASG = re.compile(r'\b(v\d+) = ([A-Za-z_]\w*)\(')
DECL = re.compile(r'^\s+([A-Za-z_][\w ]*?)\s*(\**)\s*(v\d+)\s*(\[[^\]]*\])?\s*;')
SIG = re.compile(r'^([A-Za-z_][\w ]*?)\s*(\**)\s*([A-Za-z_]\w*)\(')
SZ = {'char': 1, 'unsigned char': 1, 'bool': 1, 'short': 2, 'unsigned short': 2, 'int': 4,
      'unsigned int': 4, 'long': 8, 'unsigned long': 8}
CAST = r'\((?:char|unsigned char|bool|int|unsigned int|short|unsigned short)\)'
EQM1 = re.compile(r'(?<![\w)])(' + CAST + r')?([A-Za-z_]\w*)\((?:[^()]|\([^()]*\))*\)\s*[!=]= -1\b')


def returns(fs):
    ret = {}
    for f in fs:
        s = f.get('code', '').strip()
        m = SIG.match(s.splitlines()[0]) if s else None
        if m:
            ret[m.group(3)] = (m.group(1).strip(), m.group(2))
    return ret


def wide(rt):
    return rt is not None and (rt[1] or rt[0] in ('long', 'unsigned long'))


def decls(code):
    d = {}
    for line in code.splitlines():
        m = DECL.match(line)
        if m and not m.group(4):
            d[m.group(3)] = (m.group(1).strip(), m.group(2))
    return d


def eq_minus1(fs, ret):
    return sum(1 for f in fs for m in EQM1.finditer(f.get('code', ''))
               if not m.group(1) and wide(ret.get(m.group(2))))


def main(dirs):
    tot = collections.Counter()
    rows = []
    for d in dirs:
        for tag in sorted({f.rsplit('-', 1)[0] for f in os.listdir(d) if f.endswith('-on.json')}):
            on_fs = json.load(open(os.path.join(d, tag + '-on.json')))['functions']
            off_fs = json.load(open(os.path.join(d, tag + '-off.json')))['functions']
            ret = returns(off_fs)
            tot['eq_minus1_off'] += eq_minus1(off_fs, ret)
            tot['eq_minus1_on'] += eq_minus1(on_fs, returns(on_fs))
            on = {f['address']: f for f in on_fs}
            for f in off_fs:
                g = on.get(f['address'])
                if not g or 'code' not in f or 'code' not in g or f['code'] == g['code']:
                    continue
                da = decls(f['code'])
                ca, cb, ta = collections.Counter(), collections.Counter(), collections.defaultdict(list)
                for v, c in ASG.findall(f['code']):
                    ca[c] += 1
                    ta[c].append(da.get(v))
                for _v, c in ASG.findall(g['code']):
                    cb[c] += 1
                for c in ca:
                    n = ca[c] - cb[c]
                    rt = ret.get(c)
                    if n <= 0 or not rt:
                        continue
                    rsz = 8 if rt[1] else SZ.get(rt[0])
                    narrow = [x for x in ta[c] if x and not x[1] and x[0] in SZ and rsz and SZ[x[0]] < rsz]
                    if not narrow:
                        continue
                    k = min(n, len(narrow))
                    uses = [x.strip() for x in g['code'].splitlines() if re.search(r'\b' + re.escape(c) + r'\(', x)]
                    uncast = [u for u in uses if not re.search(CAST + re.escape(c) + r'\(', u) and not ASG.search(u)]
                    if not uncast:
                        tot['cast_printed'] += k
                        continue
                    tot['lost'] += k
                    for x in narrow[:k]:
                        tot['lost_' + x[0].replace(' ', '_')] += 1
                    m1 = sum(1 for u in uncast if re.search(re.escape(c) + r'\([^;]*?\)\s*[!=]= -1\b', u))
                    tot['lost_eq_minus1'] += min(m1, k)
                    rows.append((tag, f['name'], c, '%s%s' % rt, [x[0] for x in narrow[:k]], uncast[0][:110]))
    print(json.dumps(dict(sorted(tot.items()))))
    for r in rows:
        print(r)


if __name__ == '__main__':
    main(sys.argv[1:])
