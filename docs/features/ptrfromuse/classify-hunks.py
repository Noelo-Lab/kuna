"""Classify every hunk of a ptrfromuse before/after `decompile-all` diff.

Run it from the directory holding the sweep output (`<bin>.off.c` beside
`<bin>.byte.c` / `<bin>.void.c`), with the arm as its one argument:
`python3 classify-hunks.py byte`.  It prints the per-binary hunk table as JSON
and writes `suspects.<arm>.txt` with every hunk that needs a human.


The pass renumbers locals wherever it changes a type (a merged high changes class),
so the raw text diff is dominated by renaming noise.  Every comparison here is on a
NORMALIZED line: local/parameter numbers erased, trailing `// ...` comment erased,
whitespace collapsed.  A hunk is then classified by which single normalizer makes
the two sides equal:

  declaration       a declaration's TYPE changed
  decl-set          a declaration line appeared or vanished (more/fewer locals)
  index-rewrite     `(p + k)` <-> `&p[k]` / `p[k]`  (RulePtrArith)
  cast-change       a C cast appeared or vanished
  stmt-fold         a statement vanished but every token it carried reappears in the
                    statements that stayed: a hoisted temp folded into its uses, or a
                    common subexpression recomputed (readability, not semantics)
  STATEMENT         a NON-declaration statement appeared or vanished and something it
                    carried is GONE                                   <-- the bug class
  OTHER             the tokens themselves moved                       <-- the bug class
"""
import re, sys, difflib, json, os, collections

SWEEP = os.path.dirname(os.path.abspath(__file__))
TYPES = r'(?:unsigned |signed |const |struct )*(?:char|short|int|long|long long|float|double|void|_Bool|undefined\d*|int\d|uint\d|xunknown\d|code|FILE|DIR|stat|tm|passwd|option)\b'

def strip_names(s):
    s = re.sub(r'//.*$', '', s)
    s = re.sub(r'\bv\d+\b', 'V', s)
    s = re.sub(r'\ba\d+\b', 'A', s)
    return re.sub(r'\s+', ' ', s).strip()

def is_sig(s):
    return re.match(r'^\S.*\w+\s*\(.*\)\s*(//.*)?$', s) is not None and ';' not in s

def is_decl(s):
    return re.match(r'^\s*' + TYPES + r'\s*\**\s*(V|\w+)\s*(\[[^\]]*\])?\s*;\s*$', s) is not None

def funcs(path):
    out, cur, buf = {}, None, []
    for ln in open(path, errors='replace').read().splitlines():
        m = re.match(r'// Function: (\S+) @ (0x[0-9a-f]+)', ln)
        if m:
            if cur: out[cur] = buf
            cur, buf = f"{m.group(1)}@{m.group(2)}", [ln]
        else:
            buf.append(ln)
    if cur: out[cur] = buf
    return out

CAST_RE = r'\((?:unsigned |signed |const |struct )*(?:char|short|int|long long|long|float|double|void|_Bool|undefined\d*|int\d|uint\d|xunknown\d|code|FILE|DIR|stat|tm|passwd|option)\s*\**\)'
IDX_RE  = r'&?\w+\[[^\]]*\]'

def casts(s):   return sorted(re.findall(CAST_RE, s))
def idxs(s):    return sorted(re.findall(IDX_RE, s))
def typetoks(s):return sorted(re.findall(TYPES, s))

def skeleton(s):
    """Erase every cast, every index spelling and every type name."""
    s = re.sub(CAST_RE, '', s)
    s = re.sub(r'&(\w+)\[([^\]]+)\]', r'(\1 + \2)', s)
    s = re.sub(r'\b(\w+)\[([^\]]+)\]', r'*(\1 + \2)', s)
    s = re.sub(TYPES + r'(\s*\*+)?', 'T', s)
    return re.sub(r'[()\s]', '', s)

def sk_sorted(s):
    return sorted(re.findall(r'\w+', skeleton(s)))

def classify(a, b):
    ja = [l for l in a if l.strip()]
    jb = [l for l in b if l.strip()]
    if len(ja) != len(jb):
        da, db = [l for l in ja if is_decl(l)], [l for l in jb if is_decl(l)]
        sa, sb = [l for l in ja if not is_decl(l)], [l for l in jb if not is_decl(l)]
        if len(sa) != len(sb):
            ta = collections.Counter(t for l in sa for t in sk_sorted(l))
            tb = collections.Counter(t for l in sb for t in sk_sorted(l))
            # Nothing the vanished statements carried is missing afterwards: the
            # statement was folded into its uses, not dropped.
            if all(tb[k] >= 1 for k in ta) and all(ta[k] >= 1 for k in tb):
                return 'stmt-fold'
            return 'STATEMENT'
        if len(da) != len(db):
            return 'decl-set'
        ja, jb = sa, sb
    cats = set()
    for x, y in zip(ja, jb):
        if x == y: continue
        if skeleton(x) != skeleton(y):
            # `AddTreeState` re-orders the terms of an add tree once its base is a
            # pointer: `a0 + 0x80 + v1` prints `(long)a0 + v1 + 0x80`.
            if sk_sorted(x) == sk_sorted(y):
                cats.add('addend-reorder')
                continue
            return 'OTHER'
        if idxs(x) != idxs(y):
            cats.add('index-rewrite')
        if casts(x) != casts(y):
            cats.add('cast-change')
        if (is_decl(x) or is_sig(x)) and typetoks(x) != typetoks(y):
            cats.add('declaration')
        elif typetoks(x) != typetoks(y) and casts(x) == casts(y) and idxs(x) == idxs(y):
            cats.add('declaration')
    return '+'.join(sorted(cats)) or 'equal-after-normalize'

def main(arm):
    tot = collections.Counter()
    changed_funcs = renamed_only = total_funcs = 0
    per_bin, suspects = {}, []
    for f in sorted(os.listdir(SWEEP)):
        if not f.endswith('.off.c'): continue
        base = f[:-6]
        A, B = os.path.join(SWEEP, f), os.path.join(SWEEP, f'{base}.{arm}.c')
        if not os.path.exists(B): continue
        fa, fb = funcs(A), funcs(B)
        total_funcs += len(fa)
        n = 0; cats = collections.Counter()
        for k in fa:
            if k not in fb or fa[k] == fb[k]: continue
            na = [strip_names(l) for l in fa[k]]
            nb = [strip_names(l) for l in fb[k]]
            if na == nb:
                renamed_only += 1
                continue
            n += 1
            sm = difflib.SequenceMatcher(None, na, nb, autojunk=False)
            for tag, i1, i2, j1, j2 in sm.get_opcodes():
                if tag == 'equal': continue
                c = classify(na[i1:i2], nb[j1:j2])
                cats[c] += 1; tot[c] += 1
                if c in ('STATEMENT', 'OTHER', 'stmt-fold'):
                    suspects.append((base, k, c, fa[k][i1:i2], fb[k][j1:j2]))
        changed_funcs += n
        la = sum(1 for l in open(A, errors='replace') if l.strip())
        lb = sum(1 for l in open(B, errors='replace') if l.strip())
        sa_ = sum(open(A, errors='replace').read().count(';') for _ in (0,))
        sb_ = sum(open(B, errors='replace').read().count(';') for _ in (0,))
        per_bin[base] = {'functions': len(fa), 'changed': n, 'hunks': dict(cats),
                         'lines_off': la, 'lines_on': lb, 'line_delta': lb - la,
                         'semicolons_off': sa_, 'semicolons_on': sb_,
                         'semicolon_delta': sb_ - sa_}
    print(json.dumps({'arm': arm, 'total_functions': total_funcs,
                      'functions_changed': changed_funcs,
                      'functions_renumbering_only': renamed_only,
                      'hunks': dict(tot), 'per_binary': per_bin}, indent=2))
    with open(os.path.join(SWEEP, f'suspects.{arm}.txt'), 'w') as fh:
        for b, k, c, x, y in suspects:
            fh.write(f'=== {b} {k} [{c}]\n--- off\n' + '\n'.join(x) + f'\n+++ {arm}\n' + '\n'.join(y) + '\n\n')
    print(f'suspect hunks written: {len(suspects)}', file=sys.stderr)

main(sys.argv[1])
