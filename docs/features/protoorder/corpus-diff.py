#!/usr/bin/env python3
"""Whole-corpus before/after diff for `protoorder types`, every changed function classified.

    python3 corpus-diff.py <dir> [<dir> ...] > report.json

Each <dir> holds `<slice>.off.json` beside `<slice>.on.json` (`kuna decompile-all
--json` with `--option protoorder off` and with the default).  `review.txt` is
written next to this script: every function whose change is not explained by a
normalizer below, with its diff, for reading by hand.

Per changed function (its `code` or its `variables[]` differ) three things are
decided independently:

  layout     the stack frame's shape, as `variables[]` states it: every stack row
             (named locals and `local_` frame slots) by (offset, size).
               split   an OFF row is covered by two or more ON rows
               merge   an ON row covers two or more OFF rows
               moved   rows appear or vanish without a clean split or merge
  unwritten  use-before-def, two ways, each counted only where ON has more than
             OFF: a stack local the body reads and never writes (no assignment,
             no increment, no `&v`, no element or field store, not passed whole
             to a call), and one whose first use in the text is a read.
  class      the FIRST normalizer under which the two bodies agree:
               metadata      the code is identical; only variables[] moved
               struct-number the code is identical once each `struct_N` is
                             renamed by its order of appearance: the run minted
                             the synthesized structures in another order
               declarations  local numbering, declaration types and which
                             locals share a name -- the statements are the same
               casts         + C casts (and the parentheses they leave), NULL/0,
                             character, negative and hex spellings of the same
                             constant, `x += y` spelled `x = x + y`
               pointer-arith + `&p[k]` <-> `p + k`, `p[k]` <-> `*(p + k)`: the
                             same arithmetic spelled for a new pointer type
               literal       + a number became a string/float literal or the
                             reverse (`0xa13b` -> `""`, `0x3f000000` -> `0.5`)
               merge         + a parameter and a local trade which one holds a
                             value (`v2 = *a0` -> `a0 = *a0`)
               member-reach  + the same bytes reached through a retyped pointer:
                             `*(int *)(a0 + 0x28)` <-> `a0[10]` for `int *a0`
                             <-> `a0->field_0x28`, the index scaled by each
                             name's declared pointee
               REVIEW        none of the above; listed in review.txt
"""
import collections
import difflib
import json
import os
import re
import sys

HERE = os.path.dirname(os.path.abspath(__file__))

TYPE_WORDS = (r'unsigned|signed|const|struct|union|enum|char|short|int|long|float|double|void|bool|_Bool'
              r'|undefined\d*|u?int\d+|float\d+|xunknown\d+|code|FILE|DIR|stat|passwd|group|tm|option'
              r'|dirent|timespec|timeval|sigaction|sigset_t|mbstate_t|termios|sockaddr|obstack|regex_t'
              r'|size_t|\w+_t|struct_\w+')
TYPE = r'(?:(?:' + TYPE_WORDS + r')\b\s*)+(?:\**|\(\*+\)\s*\[\d*\])\s*'
CAST = re.compile(r'(?<![\w\]\)])\(\s*' + TYPE + r'\)(?=\s*[\w(&*\-!~"\'])')
DECL = re.compile(r'^\s*(?:(?:' + TYPE_WORDS + r')\b\s*)+(?:\**\s*[A-Za-z_]\w*\s*(?:\[[^\]]*\])?'
                  r'|\(\*+\s*[A-Za-z_]\w*\)\s*\[\d*\])\s*;\s*$')
NAME = [(re.compile(r'\bv\d+\b'), 'V'), (re.compile(r'\ba\d+\b'), 'A')]


def split(code):
    lines = [re.sub(r'\s+', ' ', re.sub(r'//.*$', '', l)).strip() for l in (code or '').splitlines()]
    lines = [l for l in lines if l]
    if not lines:
        return '', [], []
    sig, rest = lines[0], lines[1:]
    decls = [l for l in rest if DECL.match(l)]
    body = [l for l in rest if not DECL.match(l)]
    return sig, decls, body


def names(l):
    for rx, rep in NAME:
        l = rx.sub(rep, l)
    return l


def _int(t):
    return int(t, 16) if t.lower().startswith('0x') else int(t)


def _num(m):
    t = m.group(0)
    v = int(t, 16) if t.lower().startswith('0x') else int(t)
    if 0xffff0000 <= v < 1 << 32:
        v += (1 << 64) - (1 << 32)
    return str(v & 0xffffffffffffffff)


def casts(l):
    l = names(l)
    prev = None
    while prev != l:
        prev = l
        l = CAST.sub('', l)
    l = re.sub(r'\bNULL\b', '0', l)
    l = re.sub(r"'\\x([0-9a-fA-F]{2})'", lambda m: str(int(m.group(1), 16)), l)
    l = re.sub(r"'\\([nrtv0'\\\"abf])'", lambda m: str(ord({'n': '\n', 'r': '\r', 't': '\t', 'v': '\v', '0': '\0',
               'a': '\a', 'b': '\b', 'f': '\f'}.get(m.group(1), m.group(1)))), l)
    l = re.sub(r"'\\0'", '0', l)
    l = re.sub(r"'(.)'", lambda m: str(ord(m.group(1))), l)
    l = re.sub(r'\+ -(\w)', r'- \1', l)
    l = re.sub(r'(?<![\w.])-(0x[0-9a-fA-F]+|\d+)\b(?!\.)',
               lambda m: str((-int(m.group(1), 0)) & 0xffffffffffffffff), l)
    l = re.sub(r'\b(0x[0-9a-fA-F]+|\d+)[uU]?[lL]{0,2}\b', r'\1', l)
    l = re.sub(r'(?<![\w.])(0x[0-9a-fA-F]+|\d+)\b(?!\.)', _num, l)
    l = re.sub(r'(?<![\w\]\)])\(\s*([A-Za-z_]\w*|\d+)\s*\)', r'\1', l)
    return parens(l)


def strip_argument_parens(l):
    """Drop a parenthesis pair that is a whole call argument, a whole right-hand
    side, or a whole operand of a comparison whose inside has no bitwise or
    logical operator (arithmetic binds tighter than any comparison)."""
    out = list(l)
    stack, pairs = [], []
    for i, ch in enumerate(l):
        if ch == '(':
            stack.append(i)
        elif ch == ')' and stack:
            pairs.append((stack.pop(), i))
    drop = set()
    for a, b in pairs:
        before = l[:a].rstrip()[-2:]
        after = l[b + 1:].lstrip()[:2]
        inner = l[a + 1:b]
        if a > 0 and (l[a - 1].isalnum() or l[a - 1] in '_]'):
            continue
        whole_arg = before[-1:] in ('(', ',') and after[:1] in (',', ')')
        whole_rhs = before[-1:] == '=' and before[-2:] not in ('==', '!=', '<=', '>=') and after[:1] == ';'
        cmp_ops = ('<=', '>=', '==', '!=')
        simple = not re.search(r'[&|^?:]|<(?!<)|>(?!>)', inner.replace('<<', '').replace('>>', ''))
        is_cmp = lambda t, side: t in cmp_ops or (t[side:] if side else t[:1]) in ('<', '>') and t not in ('<<', '>>')
        left_whole = before[-1:] in ('(', ',', '') or before in ('&&', '||')
        right_whole = after[:1] in (')', ',', ';', '') or after in ('&&', '||')
        cmp_operand = simple and ((is_cmp(before, -1) and right_whole) or (is_cmp(after, 0) and left_whole))
        if whole_arg or whole_rhs or cmp_operand:
            drop.update((a, b))
    return ''.join(ch for i, ch in enumerate(out) if i not in drop)


def parens(l):
    m = re.match(r'^(\w+) ([-+*/%&|^]|<<|>>)= (.*);$', l)
    if m:
        l = f'{m.group(1)} = {m.group(1)} {m.group(2)} ({m.group(3)});'
    prev = None
    while prev != l:
        prev = l
        l = strip_argument_parens(l)
        l = re.sub(r'\(\s*\(([^()]*)\)\s*\)', r'(\1)', l)
        l = re.sub(r'(?<=[(,])\s*\(([^()]*)\)\s*(?=[,)])', r'\1', l)
        l = re.sub(r'(?<==) \(([^()]*)\);$', r' \1;', l)
        l = re.sub(r'^return \(([^()]*)\);$', r'return \1;', l)
        l = re.sub(r'(?<![\w\]\)])\(\s*([A-Za-z_]\w*|\d+)\s*\)', r'\1', l)
    return l


def merged(l):
    return re.sub(r'(^|[^A-Za-z0-9_]|if|while|return|else)A(?![A-Za-z0-9_])', r'\1V', l)


def pointer_arith(l):
    l = casts(l)
    prev = None
    while prev != l:
        prev = l
        l = re.sub(r'&\s*\(?\s*(&?[A-Za-z_]\w*(?:\[[^\[\]]*\])?)\s*\)?\s*\[\s*-\s*([^\[\]]+?)\s*\]', r'(\1 - \2)', l)
        l = re.sub(r'&\s*([A-Za-z_]\w*)\s*\[\s*([^\[\]]+?)\s*\]', r'(\1 + \2)', l)
        l = re.sub(r'(?<![\w&])([A-Za-z_]\w*)\s*\[\s*([^\[\]]+?)\s*\]', r'*(\1 + \2)', l)
        l = re.sub(r'\(\s*\(([^()]*)\)\s*\)', r'(\1)', l)
        l = re.sub(r'(?<![\w\]\)])\(\s*([A-Za-z_]\w*|\d+)\s*\)', r'\1', l)
    return re.sub(r'\s+', '', parens(l))


SCALAR = [(r'\bchar\b|\bu?int1\b|\bbool\b|undefined1\b', 1), (r'\bshort\b|\bu?int2\b|undefined2\b', 2),
          (r'\bint\b|\bu?int4\b|\bfloat4?\b|undefined4\b', 4),
          (r'\blong\b|\bu?int8\b|\bdouble\b|\bfloat8\b|undefined8\b', 8)]


def pointee_sizes(code, ptr):
    """name -> the byte size of what it points at, from its declaration."""
    sizes = {}
    text = '\n'.join(re.sub(r'//.*$', '', l) for l in (code or '').splitlines())
    for m in re.finditer(r'([A-Za-z_][\w ]*?)\s*(\*+)\s*([av]\d+)\b', text):
        base, stars, name = m.group(1), m.group(2), m.group(3)
        if len(stars) > 1:
            sizes[name] = ptr
            continue
        for rx, n in SCALAR:
            if re.search(rx, base):
                sizes[name] = n
                break
    return sizes


def member_reach(l, sizes):
    """`X[k]`, `&X[k]`, `X->field_0xK`, `X->field_0xK[j]` (a byte-array member) and
    `*(T *)(X + K)` to one byte-offset spelling."""
    l = re.sub(r'(&\s*)?\b([av]\d+)->field_0x([0-9a-fA-F]+)\[([^\[\]]+)\](?!\s*[\[.]|->)',
               lambda m: f'{"" if m.group(1) else "*"}({m.group(2)} + {int(m.group(3), 16)} + ({m.group(4)}))', l)
    l = re.sub(r'&\s*\b([av]\d+)->field_0x([0-9a-fA-F]+)\b(?!\s*[\[.]|->)',
               lambda m: f'({m.group(1)} + {int(m.group(2), 16)})', l)
    l = re.sub(r'\b([av]\d+)->field_0x([0-9a-fA-F]+)\b(?!\s*[\[.]|->)',
               lambda m: f'*({m.group(1)} + {int(m.group(2), 16)})', l)
    def scaled(m):
        name, idx, amp = m.group(2), m.group(3), m.group(1)
        n = sizes.get(name)
        if n is None:
            return m.group(0)
        return f'{"" if amp else "*"}({name} + ({idx}) * {n})'
    l = re.sub(r'\b0x([0-9a-fA-F]+)\b', lambda m: str(int(m.group(1), 16)), l)
    prev = None
    while prev != l:
        prev = l
        l = re.sub(r'(&?)\b([av]\d+)\s*\[\s*([^\[\]]+?)\s*\]', scaled, l)
    l = casts(l)
    l = re.sub(r'\*\(\s*([av]\d+|[VA])\s*\)', r'*(\1 + 0)', l)
    l = re.sub(r'(?<![\w)\]])\*\s*([av]\d+|[VA])\b(?!\s*[\[(])', r'*(\1 + 0)', l)
    return linear(re.sub(r'\s+', '', parens(l)))


def _signed(v):
    v &= 0xffffffffffffffff
    return v - (1 << 64) if v >= 1 << 63 else v


def _canon_sum(inner):
    """`inner` as a canonical sum of `V`/`A` terms and a constant, or None."""
    acc = collections.Counter()
    for t in re.findall(r'[+-]?[^+-]+', inner):
        sign = -1 if t.startswith('-') else 1
        t = t.lstrip('+-')
        cc = re.fullmatch(r'(\d+)\*(\d+)', t)
        if cc:
            acc['#'] += sign * _signed(int(cc.group(1))) * _signed(int(cc.group(2)))
            continue
        vk = re.fullmatch(r'([VAK])((?:\*\d+)+)', t) or re.fullmatch(r'((?:\d+\*)+)([VAK])((?:\*\d+)*)', t)
        if vk and len(vk.groups()) == 2 and vk.group(2).count('*') > 1:
            k = 1
            for d in vk.group(2).split('*')[1:]:
                k *= _signed(int(d))
            acc[vk.group(1)] += sign * k
            continue
        if vk and len(vk.groups()) == 3 and (vk.group(1) + vk.group(3)).count('*') > 1:
            k = 1
            for d in re.findall(r'\d+', vk.group(1) + vk.group(3)):
                k *= _signed(int(d))
            acc[vk.group(2)] += sign * k
            continue
        f = (re.fullmatch(r'(\d+)', t) or re.fullmatch(r'([VAK])', t)
             or re.fullmatch(r'([VAK])\*(\d+)', t) or re.fullmatch(r'(\d+)\*([VAK])', t))
        if not f:
            return None
        g = f.groups()
        if len(g) == 1 and g[0].isdigit():
            acc['#'] += sign * _signed(int(g[0]))
        elif len(g) == 1:
            acc[g[0]] += sign
        elif g[0].isdigit():
            acc[g[1]] += sign * _signed(int(g[0]))
        else:
            acc[g[0]] += sign * _signed(int(g[1]))
    parts = [k if v == 1 else f'{k}*{v & 0xffffffffffffffff}' for k, v in sorted(acc.items()) if k != '#' and v]
    c = acc['#'] & 0xffffffffffffffff
    if c or not parts:
        parts.append(str(c))
    return '+'.join(parts)


def linear(l):
    """Canonicalise every pure additive sum -- `V`, `V*k`, `k*V`, `k*k` and
    constants, with `(e)*k` distributed and signed arithmetic modulo 2^64 -- so
    `p + i*8`, `p + (i)*8` and `8*i + p` agree; then drop parentheses around a
    pure sum wherever every neighbouring operator binds more loosely than `+`."""
    prev = None
    while prev != l:
        prev = l
        l = re.sub(r'\(([\w*+-]+)\)\*(\d+)', lambda m: '(' + '+'.join(
            (f'{t}*{m.group(2)}' if not t.isdigit() else str(int(t) * int(m.group(2))))
            for t in m.group(1).split('+')) + ')' if '-' not in m.group(1) else m.group(0), l)
        l = re.sub(r'\(([^()]*)\)', lambda m: '(' + (_canon_sum(m.group(1)) or m.group(1)) + ')', l)
        l = re.sub(r'(?<=[+(=,?:<>|^])([VA0-9][\w*+-]*)(?=[)+,;?:<>|^=!]|$)',
                   lambda m: _canon_sum(m.group(1)) or m.group(1), l)
        l = re.sub(r'(?<![\w\])\]!~*/%&+-])\(([\w*+]+)\)(?![\[(\w*/%])', lambda m: m.group(1)
                   if _canon_sum(m.group(1)) is not None else m.group(0), l)
        l = re.sub(r'\(\(([^()]*)\)\)', r'(\1)', l)
        l = re.sub(r'([VA][\w*]*)\+\(([\w*+]+)\)(?![\[(\w*/%])', lambda m: m.group(1) + '+' + m.group(2)
                   if _canon_sum(m.group(2)) is not None else m.group(0), l)
    return l


def _addr_like(t):
    v = _int(t)
    if v >= (1 << 64) - (1 << 32):
        v &= 0xffffffff
        return 0x1000 <= v < 0xffff0000
    return 0x1000 <= v < 1 << 48


def literal(l):
    l = re.sub(r'\*\s*&\s*', '*', l)
    l = re.sub(r'&?\b([av]\d+|[VA])->field_0x([0-9a-fA-F]+)', lambda m: f'*({m.group(1)} + {int(m.group(2), 16)})', l)
    l = re.sub(r'\b_DT_\w+\b|\b_(?:end|edata|etext|start)\b', 'K', l)
    l = re.sub(r'(?<![\w.])-?0\.0+\b(?![\d.e])', '0', l)
    l = re.sub(r'(?<![\w.])(0x[0-9a-fA-F]+|\d+)\b(?![.\w])', lambda m: 'K' if _addr_like(m.group(1)) else m.group(1), l)
    l = pointer_arith(l)
    l = re.sub(r'"(?:[^"\\]|\\.)*"', 'K', l)
    l = re.sub(r'-?\b\d+\.\d*(?:e-?\d+)?\b|-?INFINITY|-?NAN', 'K', l)
    return re.sub(r'&?\b(?:sub|dat|DAT|FUN)_[0-9a-fA-F]+(?:\+\d+)?', 'K', l)


CLASSES = [('declarations', names), ('casts', casts), ('pointer-arith', pointer_arith), ('literal', literal),
           ('merge', lambda l: merged(literal(l)))]


def renumber(code):
    """Each `struct_N` renamed by its order of first appearance in `code`: a
    callee-first run mints synthesized structures in another order."""
    seen = {}
    return re.sub(r'\bstruct_\d+\b', lambda m: seen.setdefault(m.group(0), f'struct_z{len(seen)}'), code or '')


def classify(off, on, ptr):
    if off.get('code') == on.get('code'):
        return 'metadata'
    off, on = dict(off, code=renumber(off.get('code'))), dict(on, code=renumber(on.get('code')))
    if off.get('code') == on.get('code'):
        return 'struct-number'
    s0, d0, b0 = split(off.get('code'))
    s1, d1, b1 = split(on.get('code'))
    for cls, norm in CLASSES:
        if [norm(x) for x in b0] == [norm(x) for x in b1]:
            return cls
    z0, z1 = pointee_sizes(off.get('code'), ptr), pointee_sizes(on.get('code'), ptr)
    raw = lambda c: [x for x in (re.sub(r'\s+', ' ', re.sub(r'//.*$', '', l)).strip()
                                 for l in (c or '').splitlines()[1:]) if x and not DECL.match(x)]
    m0 = [merged(literal(member_reach(x, z0))) for x in raw(off.get('code'))]
    m1 = [merged(literal(member_reach(x, z1))) for x in raw(on.get('code'))]
    if m0 == m1:
        return 'member-reach'
    return 'REVIEW'


def stack_rows(f):
    return sorted((v['stack_offset'], v['size']) for v in f.get('variables') or []
                  if v.get('kind') == 'stack' and v.get('stack_offset') is not None and v.get('size'))


def covered(row, rows):
    o, s = row
    return [r for r in rows if r[0] >= o and r[0] + r[1] <= o + s and r != row]


def layout(off, on):
    a, b = stack_rows(off), stack_rows(on)
    if a == b:
        return None
    gone = [r for r in a if r not in b]
    new = [r for r in b if r not in a]
    if any(len(covered(r, b)) >= 2 for r in gone):
        return 'split'
    if any(len(covered(r, a)) >= 2 for r in new):
        return 'merge'
    return 'moved'


def stack_locals(code, f):
    """(name, uses, writes) for every declared stack local of `code`."""
    lines = (code or '').splitlines()
    body = '\n'.join(re.sub(r'//.*$', '', l) for l in lines[1:])
    out = []
    for l in lines[1:]:
        m = re.match(r'^\s*[^;(=]*?\b(v\d+)\s*(?:\[[^\]]*\])?\s*;\s*//\s*stack', l)
        if not m:
            continue
        n = m.group(1)
        uses = [u.start() for u in re.finditer(r'\b' + n + r'\b', body)][1:]
        w = re.compile(r'\b' + n + r'\b(?:\s*\[[^\]]*\]|\.\w+)*\s*(?:[-+*/%&|^]|<<|>>)?=(?!=)'
                       r'|\b' + n + r'\b\s*(?:\+\+|--)|(?:\+\+|--)\s*' + n + r'\b|&\s*' + n + r'\b'
                       r'|[(,]\s*(?:\((?:[^()]|\([^()]*\))*\)\s*)?' + n + r'\s*[,)]')
        writes = [x.start() for x in w.finditer(body)]
        out.append((n, uses, writes))
    return out


def unwritten(code, f):
    """Stack locals read but never written, and stack locals whose first use is a read."""
    never, first = [], []
    for n, uses, writes in stack_locals(code, f):
        if not uses:
            continue
        if not writes:
            never.append(n)
        elif min(writes) > uses[0] + 1 and not any(abs(w - uses[0]) <= len(n) + 24 for w in writes):
            first.append(n)
    return never, first


def main(dirs):
    report = collections.OrderedDict()
    totals = collections.Counter()
    review = []
    for d in dirs:
        for fn in sorted(os.listdir(d)):
            if not fn.endswith('.off.json'):
                continue
            stem = fn[:-len('.off.json')]
            onp = os.path.join(d, stem + '.on.json')
            if not os.path.exists(onp):
                continue
            try:
                offdoc = json.load(open(os.path.join(d, fn)))
                off = {f['address']: f for f in offdoc['functions']}
                on = {f['address']: f for f in json.load(open(onp))['functions']}
                ptr = next((v.get('size') for f in offdoc['functions'] for v in f.get('variables') or []
                            if str(v.get('type') or '').endswith('*') and v.get('size')), 8)
            except Exception as e:  # noqa: BLE001
                report[stem] = {'error': str(e)}
                continue
            row = collections.Counter()
            row['functions'] = len(off)
            for addr, f0 in off.items():
                f1 = on.get(addr)
                if f1 is None:
                    row['missing'] += 1
                    continue
                if f0.get('code') == f1.get('code') and f0.get('variables') == f1.get('variables'):
                    continue
                row['changed'] += 1
                cls = classify(f0, f1, ptr)
                row['class:' + cls] += 1
                lay = layout(f0, f1)
                if lay:
                    row['layout:' + lay] += 1
                (n0, r0), (n1, r1) = unwritten(f0.get('code'), f0), unwritten(f1.get('code'), f1)
                flag = []
                if len(n1) > len(n0):
                    row['unwritten'] += 1
                    flag.append(f'never-written={n1}')
                if len(r1) > len(r0):
                    row['read-first'] += 1
                    flag.append(f'read-first={r1}')
                if cls == 'REVIEW' or lay in ('split', 'merge') or flag:
                    diff = [l for l in difflib.unified_diff((f0.get('code') or '').splitlines(),
                                                            (f1.get('code') or '').splitlines(),
                                                            lineterm='', n=0)
                            if not l.startswith(('---', '+++'))]
                    review.append(f"### {stem} {f0.get('name')} 0x{addr:x} class={cls} layout={lay} "
                                  f"{' '.join(flag)}\n" + '\n'.join(diff[:80]))
            report[stem] = dict(sorted(row.items()))
            totals.update(row)
    report['TOTAL'] = dict(sorted(totals.items()))
    with open(os.path.join(HERE, 'review.txt'), 'w') as fh:
        fh.write('\n\n'.join(review) + '\n')
    json.dump(report, sys.stdout, indent=1)
    print()


if __name__ == '__main__':
    main(sys.argv[1:])
