#!/usr/bin/env python3
"""Check that kuna's fixes change only what they may: compare two `kuna decompile-project` trees.

  kuna decompile-project IN.jar -o COMPAT --option jvm_vineflower_compat on
  kuna decompile-project IN.jar -o DEFAULT
  python3 -I tests/jvm/kuna_fix_diff.py COMPAT DEFAULT

Every difference between the trees must be one of: a method that failed in compat mode (its
`// $VF: Couldn't be decompiled` comment block replaced by code), the tracker note of a failure
comment (fix `report_to_kuna`), or import lines, or a hunk a fix of wrong output recognises
(RECOGNISERS: the before/after shape of that fix, counted under the fix's name), or a reflow of the
same tokens (`layout`, which such a fix causes where a line grows past the wrap width). Anything else
is printed as collateral and the exit status is 1. The counts it prints are the evidence a fix's commit quotes
(docs/jvm.md, "Fixing a Vineflower bug").
"""
import difflib
import os
import re
import sys

MARKER = "// $VF: Couldn't be decompiled"
VINEFLOWER_NOTE = '// Please report this to the Vineflower issue tracker'
KUNA_NOTE = '// Please report this to the kuna issue tracker'


def tree(root):
    out = {}
    for dirpath, _, files in os.walk(root):
        for name in files:
            path = os.path.join(dirpath, name)
            out[os.path.relpath(path, root)] = path
    return out


def lines(path):
    with open(path, encoding='utf-8', errors='replace') as f:
        return f.read().split('\n')


def failure_blocks(text):
    """The indices of the lines of every failure comment block, with the method's header line
    (a failed method's parameters are printed `paramN`, a decompiled one's by their names)."""
    marked, inside = set(), False
    for i, line in enumerate(text):
        stripped = line.strip()
        if stripped == MARKER:
            inside = True
            if i > 0 and text[i - 1].rstrip().endswith('{'):
                marked.add(i - 1)
        elif not stripped.startswith('//'):
            inside = False
        if inside:
            marked.add(i)
    return marked


def is_import(line):
    s = line.strip()
    return s == '' or s.startswith('import ')


TOKEN = re.compile(r'"(?:[^"\\\n]|\\.)*"|\'(?:[^\'\\\n]|\\.)*\'|[\w$]+|\S')


def tokens(text_lines):
    """The Java tokens of some lines: string and char literals whole, whitespace dropped."""
    return TOKEN.findall('\n'.join(text_lines))


def same_tokens(old, new):
    """Only line breaks and indentation changed (a reflow): the same tokens."""
    return old != new and tokens(old) == tokens(new)


def external_class_diamond(old, new):
    """external_class_diamond: token for token, `new C(` became `new C<>(`, the lines possibly reflowed."""
    a, b = tokens(old), tokens(new)
    out, i, found = [], 0, False
    while i < len(b):
        if b[i] == 'new':
            j = i + 1
            while j + 2 < len(b) and b[j + 1] == '.':
                j += 2
            if j + 3 < len(b) and b[j + 1:j + 4] == ['<', '>', '(']:
                out += b[i:j + 1] + ['(']
                i, found = j + 4, True
                continue
        out.append(b[i])
        i += 1
    return found and out == a


def block_end(toks, i):
    """The index of the token that closes the bracket at toks[i]."""
    pairs = {'(': ')', '{': '}', '[': ']'}
    depth = 0
    for j in range(i, len(toks)):
        if toks[j] in pairs:
            depth += 1
        elif toks[j] in pairs.values():
            depth -= 1
            if depth == 0:
                return j
    return None


def top_level(toks, start, end):
    """The indices in toks[start:end] at bracket depth 0 (relative to start)."""
    depth, out = 0, []
    for j in range(start, end):
        if depth == 0:
            out.append(j)
        if toks[j] in '({[':
            depth += 1
        elif toks[j] in ')}]':
            depth -= 1
    return out


def unblock_lambda_asserts(toks):
    """`-> { assert X ; }` written back as `-> assert X`."""
    out, i, found = [], 0, False
    while i < len(toks):
        if toks[i:i + 4] == ['-', '>', '{', 'assert']:
            end = block_end(toks, i + 2)
            if end is not None and toks[end - 1] == ';' and \
                    [j for j in top_level(toks, i + 3, end) if toks[j] == ';'] == [end - 1]:
                out += ['-', '>'] + toks[i + 3:end - 1]
                i, found = end + 1, True
                continue
        out.append(toks[i])
        i += 1
    return out, found


def statement_start(toks, end, floor):
    """The first token of the statement that ends just before toks[end], in a block starting at floor."""
    start, depth = floor, 0
    for j in range(floor, end):
        if toks[j] in '({[':
            depth += 1
        elif toks[j] in ')}]':
            depth -= 1
            if depth == 0 and toks[j] == '}':
                start = j + 1
        elif toks[j] == ';' and depth == 0:
            start = j + 1
    return start


def declarations(toks, end):
    """The (start, semicolon) of each top-level statement before toks[end] shaped `T v = E ;`."""
    start, depth, out = 0, 0, []
    for j in range(end):
        if toks[j] in '({[':
            depth += 1
        elif toks[j] in ')}]':
            depth -= 1
            if depth == 0 and toks[j] == '}':
                start = j + 1
        elif toks[j] == ';' and depth == 0:
            eq = [k for k in top_level(toks, start, j) if toks[k] == '=']
            if eq and eq[0] - start >= 2 and toks[eq[0] - 1] not in '.)]>':
                out.append((start, j))
            start = j + 1
    return out


def unwhile_assert_updates(toks):
    """`INIT ; while ( C ) { ... assert X ; }` written back as `for ( INIT ; C ; assert X ) { ... }`
    (the closing brace may lie past the end of the hunks); INIT may also be the initializer of an
    earlier declaration `T v = E ;`, which the for form writes as `T v ;` and `for ( v = E ; ...`."""
    for w in range(len(toks)):
        if toks[w] != 'while' or w + 1 >= len(toks) or toks[w + 1] != '(':
            continue
        cond_end = block_end(toks, w + 1)
        if cond_end is None or cond_end + 1 >= len(toks) or toks[cond_end + 1] != '{':
            continue
        body_end = block_end(toks, cond_end + 1)
        closed = body_end is not None
        if not closed:
            body_end = len(toks)
        if toks[body_end - 1] != ';':
            continue
        upd = statement_start(toks, body_end - 1, cond_end + 2)
        if toks[upd] != 'assert':
            continue
        cond, update, body = toks[w + 2:cond_end], toks[upd:body_end - 1], toks[cond_end + 2:upd]
        tail = body + (['}'] + toks[body_end + 1:] if closed else [])
        for init_start in ([statement_start(toks, w - 1, 0)] if w > 0 and toks[w - 1] == ';' else []) + [w]:
            init = toks[init_start:w - 1] if init_start < w else []
            yield toks[:init_start] + ['for', '('] + init + [';'] + cond + [';'] + update + [')', '{'] + tail
        for start, end in declarations(toks, w):
            eq = toks.index('=', start)
            init = toks[eq - 1:end]
            yield (toks[:eq] + [';'] + toks[end + 1:w] + ['for', '('] + init + [';'] + cond + [';'] + update
                   + [')', '{'] + tail)


def statement_as_expression(old, new):
    """statement_as_expression: token for token, an expression lambda `-> assert X` became
    `-> { assert X; }`, or `for (INIT; C; assert X) { ... }` became `INIT; while (C) { ... assert X; }`,
    the lines possibly reflowed."""
    a, b = tokens(old), tokens(new)
    b, found = unblock_lambda_asserts(b)
    if found and b == a:
        return True
    return any(candidate == a for candidate in unwhile_assert_updates(b))


UNREPRESENTABLE = '<unrepresentable>.'
ASSERT_THROW = re.compile(r'\s*throw new AssertionError\(')
WORD = re.compile(r'[A-Za-z_$][\w$]*')
ENUM_HEADER = re.compile(r'^\s*(?:[\w$]+ )*enum [\w$]+(?: implements [^{]*)? \{$')


def anonymous_static_qualifier(old, new):
    """anonymous_static_qualifier: line for line, `<unrepresentable>.` dropped and nothing else changed."""
    if len(old) != len(new) or old == new:
        return False
    return all(o == n or (UNREPRESENTABLE in o and o.replace(UNREPRESENTABLE, '') == n) for o, n in zip(old, new))


def assert_throws(old):
    """The `throw new AssertionError(...)` statements of `old` when it holds nothing else but blank lines and
    `return;` (the explicit return Vineflower prints before them), else None."""
    throws, current = [], None
    for line in old:
        s = line.strip()
        if current is not None:
            current.append(s)
            if s.endswith(';'):
                throws.append(' '.join(current))
                current = None
        elif ASSERT_THROW.match(line):
            current = [s]
            if s.endswith(';'):
                throws.append(s)
                current = None
        elif s not in ('', 'return;'):
            return None
    return throws if throws and current is None else None


def assert_dead_throw(old, new):
    """assert_dead_throw: dead `throw new AssertionError(...)` statements (with the `return;` before
    them) removed and nothing added."""
    return not any(x.strip() for x in new) and assert_throws(old) is not None




def enum_body_separator(a, b, i1, i2, j1, j2, throw_names):
    """enum_body_separator: a `;` line (and a blank line) inserted right after an enum's header."""
    return (i1 == i2 and i1 > 0 and ENUM_HEADER.match(a[i1 - 1]) is not None
            and [x.strip() for x in b[j1:j2]] == [';', ''] and b[j1].startswith(' '))


def declared_in_scope(o, n, names):
    """`n` is `o` with a type put before one use of a name of `names`, or with every use of one such name
    renamed to one other identifier (and a type put before the first)."""
    for name in names:
        word = r'(?<![\w$])%s(?![\w$])' % re.escape(name)
        for m in re.finditer(word, o):
            head, tail = o[:m.start()], o[m.start():]
            if (n.startswith(head) and n.endswith(tail)
                    and re.fullmatch(r'[\w$.<>\[\], ?]+ ', n[len(head):len(n) - len(tail)])):
                return True
        pieces = re.split(word, o)
        if len(pieces) > 1:
            pattern = r'(?:[\w$.<>\[\], ?]+ )?([A-Za-z_$][\w$]*)'.join(re.escape(p) for p in pieces[:2])
            pattern += ''.join(r'\1' + re.escape(p) for p in pieces[2:])
            if re.fullmatch(pattern, n):
                return True
    return False


def assert_dead_throw_scope(a, b, i1, i2, j1, j2, throw_names):
    """assert_dead_throw, in a file where it removed throws: a variable the removed throws read is
    declared where it is used (its separate declaration removed, a type put before its first assignment or
    for-each header) or keeps its own name instead of being merged with another local."""
    old, new = a[i1:i2], b[j1:j2]
    if not throw_names or old == new:
        return False
    while old and len(old) > len(new):
        m = re.fullmatch(r'\s*[\w$.<>\[\], ?]+ ([\w$]+);', old[0])
        if not m or m.group(1) not in throw_names:
            return False
        old = old[1:]
    if len(old) != len(new):
        return False
    return all(o == n or declared_in_scope(o, n, throw_names) for o, n in zip(old, new))


JAVA_CAST = re.compile(r'\([A-Z][\w.$]*(?:<[^()]*>)?(?:\[\])*\)(?=[\w`$(])')
KT_AS = re.compile(r' as [\w.$]+(?:<[^()]*?>)?\??')


def uncast(line):
    """`line` without Java casts `(T)x` and Kotlin casts `x as T` (a `return` is written with the casts
    of the method's return type, a bare value without them)."""
    return KT_AS.sub('', JAVA_CAST.sub('', line))


def return_added(o, n):
    """`n` is the statement `o` (a line, or a `when` branch `C -> X`) as `return X`, casts aside."""
    o, n = uncast(o), uncast(n)
    if not o.strip():
        return False
    starts = [len(o) - len(o.lstrip())] + [m.end() for m in re.finditer(r'-> ', o)]
    return any(n == o[:k] + 'return ' + o[k:] for k in starts)


def kt_return_outside_lambda(old, new):
    """kt_return_outside_lambda: line for line, a statement `X` became `return X` at the same indentation
    (casts aside), and lines of the returned expression changed only in their casts; or, reflowed, the same
    tokens (casts aside) with `return`s added."""
    if old == new:
        return False
    if len(old) == len(new):
        pairs = [(o, n) for o, n in zip(old, new) if o != n]
        if (any(return_added(o, n) for o, n in pairs)
                and all(return_added(o, n) or uncast(o) == uncast(n) for o, n in pairs)):
            return True
    a, b = tokens([uncast(x) for x in old]), tokens([uncast(x) for x in new])
    return b.count('return') > a.count('return') and [t for t in a if t != 'return'] == [t for t in b if t != 'return']


def kt_return_outside_lambda_scope(a, b, i1, i2, j1, j2, throw_names):
    """kt_return_outside_lambda: lines of a multi-line returned expression changed only in their casts
    (and reflowed by it); the statement they belong to (up through less indented lines, not past a `{`)
    is a `return` the fix added."""
    old, new = a[i1:i2], b[j1:j2]
    if old == new or not new or tokens([uncast(x) for x in old]) != tokens([uncast(x) for x in new]):
        return False
    depth = len(new[0]) - len(new[0].lstrip())
    for k in range(j1 - 1, -1, -1):
        line = b[k]
        if not line.strip() or len(line) - len(line.lstrip()) >= depth:
            continue
        if line.lstrip().startswith('return '):
            return line not in a
        if line.rstrip().endswith('{'):
            return False
        depth = len(line) - len(line.lstrip())
    return False


def balanced_end(text, start):
    """The index of the `)` closing the `(` at `start`, skipping string and char literals, or None."""
    depth, i = 0, start
    while i < len(text):
        c = text[i]
        if c in '"\'':
            m = re.compile(r'"(?:[^"\\]|\\.)*"|\'(?:[^\'\\]|\\.)*\'').match(text, i)
            if not m:
                return None
            i = m.end()
            continue
        depth += {'(': 1, ')': -1}.get(c, 0)
        if depth == 0:
            return i
        i += 1
    return None


def object_super_arguments(o, n):
    """`n` is the `object :` header line `o` with one argument list `(...)` put right after a supertype."""
    if ' object : ' not in ' ' + o.lstrip() and 'object : ' not in o:
        return False
    k = next((i for i, (x, y) in enumerate(zip(o, n)) if x != y), len(o))
    if k >= len(n) or n[k] != '(' or k == 0 or not re.match(r'[\w$>]', n[k - 1]):
        return False
    end = balanced_end(n, k)
    return end is not None and n[:k] + n[end + 1:] == o


def kt_object_super_call(old, new):
    """kt_object_super_call: line for line, an `object :` header gained its superclass constructor's arguments."""
    if len(old) != len(new) or old == new:
        return False
    return all(o == n or object_super_arguments(o, n) for o, n in zip(old, new))


def method_block(block):
    """`block` (blank lines at its ends aside) is whole Kotlin function declarations: annotations, a `fun`
    header ending in `{`, a body, and `}` at the header's indentation."""
    rest = list(block)
    while rest and not rest[0].strip():
        rest = rest[1:]
    while rest and not rest[-1].strip():
        rest = rest[:-1]
    if not rest:
        return False
    while rest:
        i = 0
        while i < len(rest) and rest[i].lstrip().startswith(('@', '/**')):
            i += 1
        if i >= len(rest) or not re.search(r'(?<![\w$])fun ', rest[i]) or not rest[i].rstrip().endswith('{'):
            return False
        indent = rest[i][:len(rest[i]) - len(rest[i].lstrip())]
        close = next((j for j in range(i + 1, len(rest)) if rest[j] == indent + '}'), None)
        if close is None:
            return False
        rest = rest[close + 1:]
        while rest and not rest[0].strip():
            rest = rest[1:]
    return True


def kt_failed_method_hidden(a, b, i1, i2, j1, j2, throw_names):
    """kt_failed_method_hidden: whole functions inserted (a member Vineflower hides because its body
    failed, now printed with its failure comment, or decompiled where another fix removed the failure);
    the inserted lines may be rotated by the lines that follow them (the diff's choice)."""
    if i1 != i2 or j1 == j2:
        return False
    block = b[j1:j2]
    for r in range(len(block)):
        if r and b[j2 + r - 1:j2 + r] != [block[r - 1]]:
            break
        if method_block(block[r:] + block[:r]):
            return True
    for r in range(1, len(block)):
        if j1 - r < 0 or b[j1 - r] != block[-r]:
            break
        if method_block(block[-r:] + block[:-r]):
            return True
    return False


def kt_failed_method_hidden_reflow(old, new):
    """kt_failed_method_hidden, in an object written as an argument: whole functions inserted (ones whose
    lines `old` does not hold) and the rest the same tokens, reflowed (the longer object reflowed its call),
    with any `return`s kt_return_outside_lambda added."""
    held = '\n'.join(x.strip() for x in old)
    rest, s, inserted = list(new), 0, False
    while s < len(rest):
        line = rest[s]
        if line.lstrip().startswith(('@', '/**')) or re.search(r'(?<![\w$])fun ', line):
            e = next((e for e in range(s + 1, len(rest) + 1) if method_block(rest[s:e])), None)
            if e is not None and '\n'.join(x.strip() for x in rest[s:e]) not in held:
                rest, inserted = rest[:s] + rest[e:], True
                continue
        s += 1
    return inserted and (tokens(rest) == tokens(old) or kt_return_outside_lambda(old, rest))


CONTEXT_RECOGNISERS = {'enum_body_separator': enum_body_separator,
                       'assert_dead_throw': assert_dead_throw_scope,
                       'kt_failed_method_hidden': kt_failed_method_hidden,
                       'kt_return_outside_lambda': kt_return_outside_lambda_scope}


RECOGNISERS = {
    'external_class_diamond': external_class_diamond,
    'statement_as_expression': statement_as_expression,
    'anonymous_static_qualifier': anonymous_static_qualifier,
    'assert_dead_throw': assert_dead_throw,
    'kt_return_outside_lambda': kt_return_outside_lambda,
    'kt_object_super_call': kt_object_super_call,
    'kt_failed_method_hidden': kt_failed_method_hidden_reflow,
}


def classify(a, b, i1, i2, j1, j2, marked):
    kind = classify_base(a, b, i1, i2, j1, j2, marked)
    if kind != 'other':
        return kind
    old, new = a[i1:i2], b[j1:j2]
    for fix, recognises in RECOGNISERS.items():
        if recognises(old, new):
            return fix
    return 'layout' if same_tokens(old, new) else 'other'


def classify_base(a, b, i1, i2, j1, j2, marked):
    old, new = a[i1:i2], b[j1:j2]
    for fix, recognises in RECOGNISERS.items():
        if recognises(old, new):
            return fix
    if old and new and all(is_import(x) for x in old + new):
        return 'import'
    if not old and all(is_import(x) for x in new) and any(x.strip() for x in new):
        return 'import'
    if (len(old) == len(new) and old
            and all(o.strip().startswith(VINEFLOWER_NOTE) and n.strip().startswith(KUNA_NOTE)
                    for o, n in zip(old, new))):
        return 'note'
    if not old:
        return 'fixed' if (i1 - 1 in marked or i1 in marked) else 'other'
    if all(i in marked for i in range(i1, i2)):
        return 'fixed'
    if all(is_import(x) for x in old + new):
        return 'import'
    return 'other'


def reflowed(a, b, hunks):
    """The hunks again, where a run of neighbouring other, layout and recognised hunks that holds an
    other one is one reflowed change a recogniser accepts as a whole (line matching can pair a line of
    the old layout with a different line of the new one, splitting the change's tokens across hunks)."""
    out, run = [], []
    for hunk in hunks + [('end', 0, 0, 0, 0)]:
        if hunk[0] in RECOGNISERS or hunk[0] in ('other', 'layout'):
            run.append(hunk)
            continue
        if any(k == 'other' for k, *_ in run):
            old, new = a[run[0][1]:run[-1][2]], b[run[0][3]:run[-1][4]]
            fix = next((f for f, recognises in RECOGNISERS.items() if recognises(old, new)), None)
            if fix:
                run = [(fix,) + tuple(h[1:]) for h in run]
        out += run
        run = []
        if hunk[0] != 'end':
            out.append(hunk)
    return out


def main():
    if len(sys.argv) != 3:
        sys.exit(__doc__)
    compat, default = tree(sys.argv[1]), tree(sys.argv[2])
    counts = {'files': 0, 'differing': 0, 'import': 0, 'note': 0, 'fixed hunks': 0, 'other': 0}
    counts.update((fix, 0) for fix in list(RECOGNISERS) + list(CONTEXT_RECOGNISERS) + ['layout'])
    failed_compat = failed_default = 0
    bad = []
    for rel in sorted(set(compat) | set(default)):
        if rel not in compat or rel not in default:
            bad.append(f'{rel}: only in {"default" if rel in default else "compat"}')
            continue
        counts['files'] += 1
        a, b = lines(compat[rel]), lines(default[rel])
        failed_compat += sum(1 for x in a if x.strip() == MARKER)
        failed_default += sum(1 for x in b if x.strip() == MARKER)
        if a == b:
            continue
        counts['differing'] += 1
        marked = failure_blocks(a)
        hunks = [(classify(a, b, i1, i2, j1, j2, marked), i1, i2, j1, j2)
                 for tag, i1, i2, j1, j2 in difflib.SequenceMatcher(None, a, b, autojunk=False).get_opcodes()
                 if tag != 'equal']
        hunks = reflowed(a, b, hunks)
        throw_names = {w for kind, i1, i2, _, _ in hunks if kind == 'assert_dead_throw'
                       for t in assert_throws(a[i1:i2]) or [] for w in WORD.findall(t.split('(', 1)[1])}
        for kind, i1, i2, j1, j2 in hunks:
            if kind == 'other':
                kind = next((fix for fix, recognises in CONTEXT_RECOGNISERS.items()
                             if recognises(a, b, i1, i2, j1, j2, throw_names)), 'other')
            counts['fixed hunks' if kind == 'fixed' else kind] += 1
            if kind == 'other':
                bad.append(f'{rel}:{i1 + 1}: ' + ' | '.join((a[i1:i2] + ['->'] + b[j1:j2])[:6]))
    print(f"files {counts['files']}, differing {counts['differing']}; failed methods: {failed_compat} in compat, "
          f"{failed_default} by default ({failed_compat - failed_default} fixed); hunks: {counts['fixed hunks']} in "
          f"failed methods, {counts['note']} tracker notes, {counts['import']} imports, {counts['other']} other"
          + ''.join(f", {counts[fix]} {fix}"
                    for fix in dict.fromkeys(list(RECOGNISERS) + list(CONTEXT_RECOGNISERS) + ['layout'])))
    for line in bad:
        print('COLLATERAL', line)
    sys.exit(1 if bad else 0)


if __name__ == '__main__':
    main()
