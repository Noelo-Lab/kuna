"""off vs static over decompile-all JSON: at every format call whose format the
rendered C names, the vararg count against the conversion count and each
vararg's class (floating or not) against its conversion's, plus function arity.

usage: argcount.py <dir> <armA> <armB>   (files <dir>/<bin>.<arm>.json)

A class is read off the rendered C: a declared variable's type, a cast, a
literal, or an arithmetic expression whose known operands agree. `?` (unknown)
never counts as a mismatch. LFLOAT rows list every printf-family site whose
format carries `l` on a floating conversion (%lf %le %lg %la), with the class
of the argument each such conversion received in armB.

CALLARITY rows cover every OTHER call in a changed function: calls to one
callee are paired in order of appearance, and a pair whose argument count
differs (and whose per-callee multiset of counts differs, so a reordered
body is not a change) is listed with the callee's own decompiled arity
(armA, armB) as the oracle: `toward` when armB now matches it and armA did not, `away` for the
reverse, `?` otherwise. CALLSHAPE counts functions where a callee's number of
call sites differs between the arms (restructured; not paired).
"""
import glob, json, os, re, sys
from collections import Counter

SLOT = {'printf': 0, 'fprintf': 1, '__printf_chk': 1, '__fprintf_chk': 2, 'sprintf': 1,
        'snprintf': 2, '__sprintf_chk': 3, '__snprintf_chk': 4, 'error': 2, 'syslog': 1,
        '__syslog_chk': 2, 'asprintf': 1, '__asprintf_chk': 2, 'dprintf': 1,
        '__dprintf_chk': 2, 'err': 1, 'warn': 0, 'errx': 1, 'warnx': 0,
        'error_at_line': 4, 'sscanf': 1, 'fscanf': 1, 'scanf': 0,
        '__isoc99_sscanf': 1, '__isoc99_fscanf': 1, '__isoc99_scanf': 0}
FN = re.compile(r'(?<![\w])(' + '|'.join(sorted(SLOT, key=len, reverse=True)) + r')\(')
CONV = re.compile(r"%(?:\d+\$)?[-+ #0']*(\*|\d+)?(?:\.(\*|\d+))?(hh|h|ll|l|L|q|j|z|t)?([diouxXeEfFgGaAcspnCS%])")
FLOATTY = re.compile(r'^(?:long double|double|float|float\d+)$')
INTTY = re.compile(r'^(?:(?:unsigned |signed )?(?:char|short|int|long|long long)|unsigned|bool|_Bool|'
                   r'u?int\d+|size_t|ssize_t|wchar_t)$')
DECL = re.compile(r'^\s+([A-Za-z_][\w ]*?[\w*])\s*(\**)\s*([av]\d+)\s*(?:\[[^\]]*\])?;')


def split_args(code, i):
    depth, args, cur, instr, j = 1, [], '', False, i
    while j < len(code) and depth > 0:
        c = code[j]
        if instr:
            cur += c
            if c == '\\' and j + 1 < len(code):
                cur += code[j + 1]; j += 1
            elif c == '"':
                instr = False
        elif c == '"':
            instr = True; cur += c
        elif c in '([':
            depth += 1; cur += c
        elif c in ')]':
            depth -= 1
            if depth > 0:
                cur += c
        elif c == ',' and depth == 1:
            args.append(cur.strip()); cur = ''
        else:
            cur += c
        j += 1
    if cur.strip():
        args.append(cur.strip())
    return args


def conversions(s, scan):
    """Expected class per consumed argument: F for a floating printf conversion."""
    out = []
    for m in CONV.finditer(s):
        if m.group(4) == '%':
            continue
        if not scan:
            out += ['I'] * ((m.group(1) == '*') + (m.group(2) == '*'))
        floating = m.group(4) in 'eEfFgGaA' and not scan
        out.append(('F' if floating else 'I', m.group(3) == 'l' and floating))
    return [x if isinstance(x, tuple) else (x, False) for x in out]


def unesc(s):
    return bytes(s, 'utf-8').decode('unicode_escape', errors='replace')


def resolve(code, fa, pos):
    m = re.match(r'^"((?:[^"\\]|\\.)*)"$', fa)
    if m:
        return unesc(m.group(1))
    m = re.match(r'^(?:\(char \*\))?dcgettext\(NULL,"((?:[^"\\]|\\.)*)"', fa)
    if m:
        return unesc(m.group(1))
    m = re.match(r'^(?:\(char \*\))?(v\d+)$', fa)
    if not m:
        return None
    v, best = m.group(1), None
    for a in re.finditer(r'(?<![\w])' + re.escape(v) +
                         r' = (?:\(char \*\))?(?:dcgettext\(NULL,|gettext\()?"((?:[^"\\]|\\.)*)"', code):
        if a.start() < pos:
            best = a.group(1)
    return unesc(best) if best is not None else None


def type_class(t):
    t = t.strip()
    if '*' in t or '[' in t:
        return 'I'
    t = re.sub(r'^(?:const|volatile) ', '', t)
    if FLOATTY.match(t):
        return 'F'
    return 'I' if INTTY.match(t) else '?'


def declarations(code):
    types = {}
    lines = code.splitlines()
    for l in lines:
        if l.startswith('//') or l.startswith(' ') or '(' not in l:
            continue
        for p in l[l.find('(') + 1:l.rfind(')')].split(','):
            m = re.match(r'^\s*(.*?[\w*])\s*(\**)\s*([av]\d+)\s*$', p)
            if m:
                types[m.group(3)] = m.group(1) + m.group(2)
        break
    for l in lines:
        m = DECL.match(l)
        if m and not l.lstrip().startswith(('return', 'if', 'while')):
            types[m.group(3)] = m.group(1) + m.group(2)
    return types


def strip_parens(e):
    while e.startswith('(') and e.endswith(')'):
        depth = 0
        for i, c in enumerate(e):
            depth += c == '('
            depth -= c == ')'
            if depth == 0 and i < len(e) - 1:
                return e
        e = e[1:-1].strip()
    return e


def top_level_ops(e):
    """Binary operators at depth 0 outside string/char literals."""
    ops, depth, instr, i = [], 0, None, 0
    while i < len(e):
        c = e[i]
        if instr:
            if c == '\\':
                i += 1
            elif c == instr:
                instr = None
        elif c in '"\'':
            instr = c
        elif c in '([':
            depth += 1
        elif c in ')]':
            depth -= 1
        elif depth == 0 and i > 0:
            two = e[i:i + 2]
            if two in ('==', '!=', '<=', '>=', '&&', '||', '<<', '>>', '->'):
                if two != '->':
                    ops.append((i, two))
                i += 2
                continue
            if c in '+-*/%&|^<>?' and e[i - 1] == ' ' and i + 1 < len(e) and e[i + 1] == ' ':
                ops.append((i, c))
        i += 1
    return ops


def expr_class(e, types, depth=0):
    e = strip_parens(e.strip())
    if depth > 8 or not e:
        return '?'
    if e.startswith('"') or e.startswith('&') or e == 'NULL' or e.startswith("'"):
        return 'I'
    if re.fullmatch(r'-?(?:0x[0-9a-fA-F]+|\d+)[uUlL]*', e):
        return 'I'
    if re.fullmatch(r'-?(?:\d+\.\d*|\.\d+)(?:[eE][-+]?\d+)?[fFlL]?', e):
        return 'F'
    ops = top_level_ops(e)
    if ops:
        kinds = {o for _, o in ops}
        if kinds & {'==', '!=', '<=', '>=', '<', '>', '&&', '||', '%', '&', '|', '^', '<<', '>>'}:
            return 'I'
        if '?' in kinds:
            return '?'
        parts, last = [], 0
        for i, o in ops:
            parts.append(e[last:i]); last = i + len(o)
        parts.append(e[last:])
        cls = {expr_class(p, types, depth + 1) for p in parts}
        if 'F' in cls:
            return 'F'
        return 'I' if cls == {'I'} else '?'
    if e.startswith('!') or e.startswith('~'):
        return 'I'
    if e.startswith('-'):
        return expr_class(e[1:], types, depth + 1)
    m = re.match(r'^\(([^()]+)\)(.*)$', e)
    if m and not top_level_ops(m.group(2)):
        if m.group(1).strip() == '*' or not m.group(2):
            return '?'
        return type_class(m.group(1))
    m = re.match(r'^\*\(([^()]+) \*\)', e)
    if m:
        return type_class(m.group(1))
    if re.fullmatch(r'[av]\d+', e):
        return type_class(types[e]) if e in types else '?'
    return '?'


def sites(code):
    """(name, fmt, nconv, nvar, class mismatches, lfloat classes) per format call."""
    out = []
    types = None
    for m in FN.finditer(code):
        name = m.group(1)
        args = split_args(code, m.end())
        s = SLOT[name]
        if len(args) <= s:
            continue
        fmt = resolve(code, args[s], m.start())
        if fmt is None:
            continue
        exp = conversions(fmt, 'scanf' in name)
        va = args[s + 1:]
        cm, lf = [], []
        if len(va) == len(exp):
            if types is None:
                types = declarations(code)
            for k, ((want, lfloat), a) in enumerate(zip(exp, va)):
                got = expr_class(a, types)
                if got != '?' and got != want:
                    cm.append((k, want, got, a[:40]))
                if lfloat:
                    lf.append(got)
        elif any(l for _, l in exp):
            lf.append('count')
        out.append((name, fmt[:48], len(exp), len(va), tuple(cm), tuple(lf)))
    return out


def mismatches(code):
    return [(n, f, nc, nv) for n, f, nc, nv, _, _ in sites(code) if nc != nv]


def class_mismatches(code):
    return [(n, f) + c for n, f, nc, nv, cm, _ in sites(code) for c in cm]


def sig_arity(code):
    for l in code.splitlines():
        if l.startswith('//') or l.startswith(' ') or '(' not in l:
            continue
        inner = l[l.find('(') + 1:l.rfind(')')]
        return 0 if inner.strip() in ('', 'void') else inner.count(',') + 1
    return None


CALLNAME = re.compile(r'(?<![\w.>])([A-Za-z_]\w*)\(')
NOTCALL = re.compile(r'^(?:if|while|for|switch|return|sizeof|[av]\d+|[A-Z0-9_]+)$')


def other_calls(code):
    """(callee, argument count) per non-format call, in order of appearance."""
    out = []
    code = code[code.find('{') + 1:]
    for m in CALLNAME.finditer(code):
        name = m.group(1)
        if name in SLOT or NOTCALL.match(name):
            continue
        out.append((name, len(split_args(code, m.end()))))
    return out


def callee_arities(fns):
    ar = {}
    for f in fns.values():
        n = sig_arity(f.get('code') or '')
        if n is not None and f.get('name'):
            ar[f['name']] = n
    return ar


def call_changes(ca, cb, ar_a, ar_b):
    """Paired per-callee argument-count changes, plus the restructured callees."""
    from collections import defaultdict
    xa, xb = defaultdict(list), defaultdict(list)
    for n, c in other_calls(ca):
        xa[n].append(c)
    for n, c in other_calls(cb):
        xb[n].append(c)
    changes, shape = [], 0
    for n in sorted(set(xa) | set(xb)):
        if len(xa[n]) != len(xb[n]):
            shape += 1
            continue
        if sorted(xa[n]) == sorted(xb[n]):
            continue
        for i, (p, q) in enumerate(zip(xa[n], xb[n])):
            if p == q:
                continue
            want = {ar_a.get(n), ar_b.get(n)} - {None}
            if q in want and p not in want:
                verdict = 'toward'
            elif p in want and q not in want:
                verdict = 'away'
            else:
                verdict = '?'
            changes.append((n, i, p, q, ar_a.get(n), ar_b.get(n), verdict))
    return changes, shape


def load(f):
    try:
        return {x['address']: x for x in json.load(open(f))['functions']}
    except Exception:
        return None


def main(d, a, b):
    tot = Counter()
    rows = []
    for fa in sorted(glob.glob(f'{d}/*.{a}.json')):
        n = os.path.basename(fa)[:-len(f'.{a}.json')]
        fb = f'{d}/{n}.{b}.json'
        A, B = load(fa), (load(fb) if os.path.exists(fb) else None)
        if A is None or B is None:
            tot['binaries_skipped'] += 1
            continue
        tot['binaries'] += 1
        tot['functions'] += len(B)
        per = Counter()
        ar_a, ar_b = callee_arities(A), callee_arities(B)
        for k, fbn in B.items():
            cb = fbn.get('code') or ''
            ca = (A[k].get('code') or '') if k in A else None
            for s in sites(cb):
                for cls in s[5]:
                    per[f'lfloat_{cls}'] += 1
                    rows.append(f'LFLOAT {n} {hex(k)} {s[0]} {s[1]!r} arg={cls}')
            for s in (sites(ca) if ca is not None else []):
                for cls in s[5]:
                    per[f'lfloat_{a}_{cls}'] += 1
            for arm, code in ((a, ca), (b, cb)):
                if code is not None:
                    per[f'class_mismatch_{arm}'] += len(class_mismatches(code))
            if ca is None or ca == cb:
                continue
            per['changed'] += 1
            ma, mb = mismatches(ca), mismatches(cb)
            for x in mb:
                if x not in ma:
                    per['new_mismatch'] += 1
                    rows.append(f'NEWMISMATCH {n} {hex(k)} {x}')
            for x in ma:
                if x not in mb:
                    per['fixed_mismatch'] += 1
            ca_cls, cb_cls = class_mismatches(ca), class_mismatches(cb)
            for x in cb_cls:
                if x not in ca_cls:
                    per['new_class_mismatch'] += 1
                    rows.append(f'NEWCLASS {n} {hex(k)} {x}')
            for x in ca_cls:
                if x not in cb_cls:
                    per['fixed_class_mismatch'] += 1
                    rows.append(f'FIXEDCLASS {n} {hex(k)} {x}')
            cc, shape = call_changes(ca, cb, ar_a, ar_b)
            per['callshape'] += shape
            for callee, i, p, q, oa, ob, verdict in cc:
                per['callarity_' + {'toward': 'toward', 'away': 'away', '?': 'unknown'}[verdict]] += 1
                rows.append(f'CALLARITY {n} {hex(k)} {fbn.get("name")} -> {callee}#{i} {p}->{q} '
                            f'callee={oa}/{ob} {verdict}')
            ra, rb = sig_arity(ca), sig_arity(cb)
            if ra is not None and rb is not None and ra != rb:
                per['arity_up' if rb > ra else 'arity_down'] += 1
                rows.append(f'ARITY {n} {hex(k)} {ra}->{rb}')
        tot.update(per)
        if per:
            rows.append(f'BIN {n} ' + json.dumps(dict(per)))
    print('\n'.join(rows))
    print('TOTAL', json.dumps(dict(sorted(tot.items()))))


if __name__ == '__main__':
    main(*sys.argv[1:4])
