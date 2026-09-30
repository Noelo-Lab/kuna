"""Classify every function `fieldtype` changes: castbench arm OFF vs ON (whole-binary decompile-all).

hunks.py OFF_ARM ON_ARM [--show CLASS]
Normalizations tried in order; the first under which the function is equal names its class:
  RENAME  struct_N numbering and v-variable numbering only (the ledger minted in another order)
  DECL    as RENAME, plus declaration lines (a local's declared type / a merged or split register local)
  CAST    as DECL, plus every cast removed
  ARITH   as CAST, and each line compared as the bag of its identifiers and numbers: pointer
          arithmetic re-spelled (`&X[Y]` for `X + Y`, operands commuted), `NULL` for `0`, a constant
          address named `&dat_N` for `0xN`
  MERGE   as ARITH with every variable name erased and self-copies dropped: a register local
          merged with or split from another (declaration count printed by the decl census)
  SIG     as MERGE with the signature line's parameter types, `&X->field_0x0` read as `X`, and
          statements that only copy one variable to another dropped: a parameter whose type follows a
          retyped field it is stored into, a pointer to a record spelled as the address of its first
          member, a copy the merge absorbed
  OTHER   anything else: printed for reading
"""
import collections, difflib, re, sys
from pathlib import Path
sys.path.insert(0, '/home/mahaloz/kwt/castbench')
import castcount as CC, castbench as CB

TYPE = r'(?:const\s+)?(?:unsigned\s+|signed\s+)?(?:struct\s+)?[A-Za-z_]\w*(?:\s+(?:long|int|char|short))*(?:\s*\*+)?'
CAST = re.compile(r'\((?:' + TYPE + r')\)(?=[\w(*&\-!~"\'])')
DECL = re.compile(r'^\s+[A-Za-z_][\w ]*?(?:[\s*]+|\s*\(\*)[va]\d+\)?(?:\s*\[\d+\])?;.*$')

def canon(lines):
    names = {}
    def sub(m):
        names.setdefault(m.group(0), 'V%d' % len(names))
        return names[m.group(0)]
    return [re.sub(r'\bv\d+\b', sub, l) for l in lines]

def norm(text, level):
    t = re.sub(r'struct_\d+', 'struct_#', text)
    lines = t.splitlines()
    if level >= 1:
        lines = [l for l in lines if not DECL.match(l)]
    lines = canon(lines)
    if level >= 2:
        prev = None
        out = []
        for l in lines:
            while True:
                n = CAST.sub('', l)
                if n == l: break
                l = n
            out.append(l)
        lines = out
    if level >= 3:
        out = []
        for l in lines:
            l = l.replace('NULL', '0')
            l = re.sub(r'&dat_([0-9a-f]+)', r'0x\1', l)
            toks = sorted(re.findall(r'[A-Za-z_]\w*|0x[0-9a-fA-F]+|\d+', l))
            if level >= 4:
                toks = sorted('v' if re.fullmatch(r'V\d+', t) else t for t in toks)
                if toks == ['v', 'v']:
                    continue
            if level >= 5:
                if re.match(r'^\S.*\(.*\)', l) and not l.startswith(' '):
                    toks = [t for t in toks if not re.fullmatch(r'a\d+|unsigned|signed|long|int|char|short|int8|uint8|int4|uint4|void', t)]
                l2 = re.sub(r'&(\w+)->field_0x0\b', r'\1', l)
                toks = sorted('v' if re.fullmatch(r'V\d+', t) else t for t in re.findall(r'[A-Za-z_]\w*|0x[0-9a-fA-F]+|\d+', re.sub(r'&dat_([0-9a-f]+)', r'0x\1', l2.replace('NULL', '0'))))
                if toks in (['v', 'v'], ['a0', 'v'], ['a1', 'v'], ['a2', 'v']) and '=' in l and '==' not in l:
                    continue
            out.append(' '.join(toks))
        lines = out
    return lines

def main():
    off, on = Path(sys.argv[1]), Path(sys.argv[2])
    show = sys.argv[sys.argv.index('--show') + 1] if '--show' in sys.argv else None
    tot = collections.Counter(); per = {}
    others = []
    for opt, proj, b in CB.corpus('full'):
        fa = {a: t for _, a, t in CC.split_functions((off / opt / proj / f'{b}.c').read_text(errors='replace'))}
        fb = {a: t for _, a, t in CC.split_functions((on / opt / proj / f'{b}.c').read_text(errors='replace'))}
        c = collections.Counter()
        for a in sorted(set(fa) & set(fb)):
            if fa[a] == fb[a]:
                continue
            cls = 'OTHER'
            for lvl, name in enumerate(['RENAME', 'DECL', 'CAST', 'ARITH', 'MERGE', 'SIG']):
                if norm(fa[a], lvl) == norm(fb[a], lvl):
                    cls = name
                    break
            c[cls] += 1
            if cls == (show or 'OTHER'):
                others.append((f'{opt}/{b}@{hex(a)}', fa[a], fb[a]))
        per[f'{opt}/{proj}/{b}'] = dict(c)
        tot.update(c)
    for k, v in per.items():
        if v: print(k, v)
    print('TOTAL', dict(tot), 'changed functions', sum(tot.values()))
    for tag, x, y in others:
        print('=====', tag)
        print('\n'.join(difflib.unified_diff(canon(re.sub(r'struct_\d+', 'struct_#', x).splitlines()),
                                             canon(re.sub(r'struct_\d+', 'struct_#', y).splitlines()), lineterm='', n=1)))

main()
