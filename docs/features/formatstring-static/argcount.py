"""off vs static over decompile-all JSON: vararg count against conversion count
at every format call whose format the rendered C names, plus function arity.

usage: argcount.py <dir> <armA> <armB>   (files <dir>/<bin>.<arm>.json)
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


def nconv(s, scan):
    n = 0
    for m in CONV.finditer(s):
        if m.group(4) == '%':
            continue
        n += 1 + (m.group(1) == '*' and not scan) + (m.group(2) == '*' and not scan)
    return n


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


def mismatches(code):
    out = []
    for m in FN.finditer(code):
        name = m.group(1)
        args = split_args(code, m.end())
        s = SLOT[name]
        if len(args) <= s:
            continue
        fmt = resolve(code, args[s], m.start())
        if fmt is None:
            continue
        nv, nc = len(args) - s - 1, nconv(fmt, 'scanf' in name)
        if nv != nc:
            out.append((name, fmt[:48], nc, nv))
    return out


def sig_arity(code):
    for l in code.splitlines():
        if l.startswith('//') or l.startswith(' ') or '(' not in l:
            continue
        inner = l[l.find('(') + 1:l.rfind(')')]
        return 0 if inner.strip() in ('', 'void') else inner.count(',') + 1
    return None


def load(f):
    try:
        return {x['address']: x for x in json.load(open(f))['functions']}
    except Exception:
        return None


d, a, b = sys.argv[1:4]
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
    for k, fbn in B.items():
        if k not in A:
            continue
        ca, cb = A[k].get('code') or '', fbn.get('code') or ''
        if ca == cb:
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
        ra, rb = sig_arity(ca), sig_arity(cb)
        if ra is not None and rb is not None and ra != rb:
            per['arity_up' if rb > ra else 'arity_down'] += 1
            rows.append(f'ARITY {n} {hex(k)} {ra}->{rb}')
    tot.update(per)
    if per:
        rows.append(f'BIN {n} ' + json.dumps(dict(per)))
print('\n'.join(rows))
print('TOTAL', json.dumps(dict(tot)))
