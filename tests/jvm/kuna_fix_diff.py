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
import collections
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
def kt_constructor_diamond(old, new):
    """kt_constructor_diamond: token for token, a Kotlin constructor call's `C<>(` became `C(`."""
    a, b = tokens(old), tokens(new)
    out, i, found = [], 0, False
    while i < len(a):
        if a[i:i + 3] == ['<', '>', '('] and i > 0 and WORD.fullmatch(a[i - 1]):
            out.append('(')
            i, found = i + 3, True
            continue
        out.append(a[i])
        i += 1
    return found and out == b


KT_LOCAL_OLD = '// $VF: local visibility outside of methodSupplier'
KT_LOCAL_NEW = '/* $VF: local visibility outside of method */ '


def kt_local_visibility_comment(old, new):
    """kt_local_visibility_comment: the line comment before a local declaration's `internal` became an
    inline comment on the declaration's line."""
    kept = [x for x in old if x.strip() != KT_LOCAL_OLD]
    return (len(kept) < len(old) and any(KT_LOCAL_NEW in x for x in new)
            and tokens(kept) == tokens([x.replace(KT_LOCAL_NEW, '') for x in new]))


KT_ARRAYS = {p + 'Array': p for p in ('Boolean', 'Byte', 'Char', 'Short', 'Int', 'Long', 'Float', 'Double',
                                     'UByte', 'UShort', 'UInt', 'ULong')}


def kotlin_type_end(toks, j):
    """The index after the Kotlin type starting at toks[j] (a name, its type arguments, a `?`)."""
    while j + 2 < len(toks) and toks[j + 1] == '.' and WORD.fullmatch(toks[j + 2]):
        j += 2
    j += 1
IDENT = re.compile(r'[A-Za-z_$][\w$]*$')
TYPE_NAME = r'[\w$.]+(?:<[\w$.<>, ?\[\]]*>)?(?:\[\])*\??'
JAVA_DECL_INIT = re.compile(r'^(?:final )?' + TYPE_NAME + r' ([\w$]+) = (.*)$')
JAVA_DECL = re.compile(r'^(?:final )?' + TYPE_NAME + r' [\w$]+;$')
KT_DECL_INIT = re.compile(r'^(?:val|var) ([\w$`]+)(?:: ' + TYPE_NAME + r')? = (.*)$')
KT_DECL = re.compile(r'^(?:val|var) [\w$`]+: ' + TYPE_NAME + r'$')
CONTINUED = (';', '{', '}', '(', ',', '=', '?', ':', '.', '+', '-', '|', '&')
CONTINUING = ('?', ':', '.', ')', '|', '&', '+', '-')
NOT_CAST_BEFORE = {'if', 'while', 'for', 'switch', 'synchronized', 'catch', ')', ']'}


def type_end(toks, i):
    """The index past a type written from toks[i] (a dotted name, type arguments, array brackets, `?`)."""
    if i >= len(toks) or not IDENT.match(toks[i]):
        return None
    j = i + 1
    while j + 1 < len(toks) and toks[j] == '.' and IDENT.match(toks[j + 1]):
        j += 2
    if j < len(toks) and toks[j] == '<':
        depth = 0
        while j < len(toks):
            depth += {'<': 1, '>': -1}.get(toks[j], 0)
            j += 1
            if depth == 0:
                break
    while j + 1 < len(toks) and toks[j] == '[' and toks[j + 1] == ']':
        j += 2
    if j < len(toks) and toks[j] == '?':
        j += 1
    return j


def kt_vararg_from_metadata(old, new):
    """kt_vararg_from_metadata: token for token, parameters changed only as `vararg x: Any` ->
    `x: Array<..>` (an array parameter the metadata does not mark vararg) or `x: IntArray` ->
    `vararg x: Int` (a primitive vararg parameter)."""
    a, b = tokens(old), tokens(new)
    i = j = 0
    found = False
    while i < len(a) or j < len(b):
        if i < len(a) and j < len(b) and a[i] == b[j]:
            i, j = i + 1, j + 1
        elif (a[i:i + 1] == ['vararg'] and a[i + 1:i + 3] == b[j:j + 2] and a[i + 2:i + 3] == [':']
              and a[i + 3:i + 4] == ['Any'] and b[j + 2:j + 3] == ['Array']):
            i, j, found = i + 4, kotlin_type_end(b, j + 2), True
        elif (b[j:j + 1] == ['vararg'] and b[j + 1:j + 3] == a[i:i + 2] and a[i + 1:i + 2] == [':']
              and i + 2 < len(a) and j + 3 < len(b) and KT_ARRAYS.get(a[i + 2]) == b[j + 3]):
            i, j, found = i + 3, j + 4, True
        else:
            return False
    return found


def kt_default_args_ctor_descriptor(old, new):
    """kt_default_args_ctor_descriptor: token for token, a parameter's unknown default `= ...` became a
    value (the default found in an inner class's default-arguments constructor)."""
    a, b = tokens(old), tokens(new)
    i = j = 0
    found = False
    while i < len(a):
        if a[i:i + 4] == ['=', '.', '.', '.'] and b[j:j + 1] == ['='] and b[j + 1:j + 4] != ['.', '.', '.']:
            end, depth = j + 1, 0
            while end < len(b) and not (depth == 0 and b[end] in ',)'):
                depth += {'(': 1, '[': 1, '{': 1, ')': -1, ']': -1, '}': -1}.get(b[end], 0)
                end += 1
            if end == j + 1:
                return False
            i, j, found = i + 4, end, True
        elif j < len(b) and a[i] == b[j]:
            i, j = i + 1, j + 1
        else:
            return False
    return found and j == len(b)
def cast_type_end(toks, i):
    """The index past a cast's type list `A & B` written from toks[i]."""
    j = type_end(toks, i)
    while j is not None and j + 1 < len(toks) and toks[j] == '&':
        j = type_end(toks, j + 1)
    return j


def uncast_tokens(toks):
    """The tokens with every cast `(T)` before an operand and every Kotlin `as T` removed, then the
    parentheses around a lone name."""
    toks = [t for t in toks if t != '`']
    out, i = [], 0
    while i < len(toks):
        if toks[i] == '(' and (not out or (out[-1] not in NOT_CAST_BEFORE and not IDENT.match(out[-1])
                                            or out[-1] in ('return', 'throw', 'yield', 'else'))):
            j = cast_type_end(toks, i + 1)
            if j is not None and j + 1 < len(toks) and toks[j] == ')' and \
                    (IDENT.match(toks[j + 1]) or toks[j + 1] in ('(', '"', "'") or toks[j + 1][0].isdigit()):
                end = block_end(toks, j + 1) if toks[j + 1] == '(' else None
                if end is not None:
                    toks = toks[:end] + toks[end + 1:]
                    i = j + 2
                else:
                    i = j + 1
                continue
        if toks[i] == 'as':
            j = type_end(toks, i + 1)
            if j is not None:
                i = j
                continue
        out.append(toks[i])
        i += 1
    bare, i = [], 0
    while i < len(out):
        if out[i] == '(' and i + 2 < len(out) and IDENT.match(out[i + 1]) and out[i + 2] == ')' and \
                not (bare and (IDENT.match(bare[-1]) or bare[-1] in (')', ']', '>'))):
            bare.append(out[i + 1])
            i += 3
            continue
        bare.append(out[i])
        i += 1
    return bare


def undeclared(text_lines):
    """The tokens of some lines, each line a statement ended by `;`, with local declarations reduced to
    their assignment (`T v = E` and `val v: T = E` to `v = E`) and bare declarations dropped."""
    rows = []
    for line in text_lines:
        s = line.strip()
        if not s or JAVA_DECL.match(s) or KT_DECL.match(s):
            continue
        m = JAVA_DECL_INIT.match(s) or KT_DECL_INIT.match(s)
        if m and not s.startswith(('return ', 'throw ')):
            s = m.group(1) + ' = ' + m.group(2)
        rows.append(TOKEN.findall(s))
    out, depth = [], 0
    for k, toks in enumerate(rows):
        depth += sum(1 for t in toks if t in '([') - sum(1 for t in toks if t in ')]')
        nxt = rows[k + 1][0] if k + 1 < len(rows) and rows[k + 1] else ''
        out += toks
        if toks and depth <= 0 and toks[-1] not in CONTINUED and nxt not in CONTINUING:
            out.append(';')
    return explicit_type_args_dropped(uncast_tokens(out))


def explicit_type_args_dropped(toks):
    """The tokens without explicit type arguments of method calls (`.<T>m(` to `.m(`)."""
    out, i = [], 0
    while i < len(toks):
        if toks[i] == '<' and out and out[-1] == '.':
            depth, j = 0, i
            while j < len(toks):
                depth += {'<': 1, '>': -1}.get(toks[j], 0)
                j += 1
                if depth == 0:
                    break
            if j + 1 < len(toks) and IDENT.match(toks[j]) and toks[j + 1] == '(':
                i = j
                continue
        out.append(toks[i])
        i += 1
    return out


def without_self_copies(toks):
    out, i = [], 0
    while i < len(toks):
        if i + 3 < len(toks) and toks[i + 1] == '=' and toks[i] == toks[i + 2] and toks[i + 3] == ';' \
                and IDENT.match(toks[i]) and (i == 0 or toks[i - 1] in (';', '{', '}')):
            i += 4
            continue
        out.append(toks[i])
        i += 1
    return out


def var_merge_narrowing(old, new):
    """var_merge_narrowing: the same statements once a variable the fix keeps apart from another (its own
    name, declaration and copy `v = w`) is renamed back onto it, and casts, declarations and self-copies
    `v = v` are dropped."""
    a, b = without_self_copies(undeclared(old)), undeclared(new)
    if old == new:
        return False
    if a == b:
        return True
    mapping = {}
    for tag, i1, i2, j1, j2 in difflib.SequenceMatcher(None, a, b, autojunk=False).get_opcodes():
        if tag == 'replace' and i2 - i1 == j2 - j1:
            for x, y in zip(a[i1:i2], b[j1:j2]):
                if IDENT.match(x) and IDENT.match(y) and x != y:
                    if mapping.setdefault(y, x) != x:
                        return False
    names = set(a)
    for i in range(len(b) - 3):
        if b[i + 1] == '=' and b[i + 3] == ';' and b[i] not in names and IDENT.match(b[i]) \
                and IDENT.match(b[i + 2]) and (i == 0 or b[i - 1] in (';', '{', '}')):
            mapping.setdefault(b[i], b[i + 2])
    if not mapping:
        return False
    return without_self_copies([mapping.get(t, t) for t in b]) == a


def ternary_numeric_arms(old, new):
    """ternary_numeric_arms: token for token, casts `(T)` put before arms of conditionals (right after
    `?` or `:`), the lines possibly reflowed."""
    a, b = tokens(old), tokens(new)
    out, i, found = [], 0, False
    while i < len(b):
        if b[i] == '(' and out and out[-1] in ('?', ':'):
            j = type_end(b, i + 1)
            if j is not None and j < len(b) and b[j] == ')':
                end = block_end(b, j + 1) if j + 1 < len(b) and b[j + 1] == '(' else None
                if end is not None:
                    b = b[:end] + b[end + 1:]
                    j += 1
                i, found = j + 1, True
                continue
        out.append(b[i])
        i += 1
    return found and out == a


def switch_expression_poly_cast(old, new):
    """switch_expression_poly_cast: token for token, `(T)(switch ...)` became `switch ...`, the lines
    possibly reflowed."""
    a, b = tokens(old), tokens(new)
    out, i, found = [], 0, False
    while i < len(a):
        if a[i] == '(':
            j = type_end(a, i + 1)
            if j is not None and j + 2 < len(a) and a[j] == ')' and a[j + 1] == '(' and a[j + 2] == 'switch':
                end = block_end(a, j + 1)
                if end is not None:
                    out += a[j + 2:end]
                    i, found = end + 1, True
                    continue
        out.append(a[i])
        i += 1
    return found and out == b
UNREPRESENTABLE_INSTANCE = '<unrepresentable>.INSTANCE'
CALLABLE_REFERENCE = r'(?:[\w$.`]+)?::[\w$`]+'


def kt_callable_reference(old, new):
    """kt_reference_anonymous_super_call and kt_property_reference: the `INSTANCE` of a function- or
    property-reference class, printed `<unrepresentable>.INSTANCE`, became the reference (`Owner::name`,
    `::name`, `Receiver::name`) and nothing else changed, the lines possibly reflowed."""
    o, n = re.sub(r'\s+', '', ''.join(old)), re.sub(r'\s+', '', ''.join(new))
    if UNREPRESENTABLE_INSTANCE not in o:
        return False
    pattern = CALLABLE_REFERENCE.join(re.escape(p) for p in o.split(UNREPRESENTABLE_INSTANCE))
    return re.fullmatch(pattern, n) is not None
IDENT = re.compile(r'[A-Za-z_$][\w$]*$')
PRIMITIVES = {'boolean', 'byte', 'char', 'short', 'int', 'long', 'float', 'double'}


def nonpublic_nested_import(old, new):
    """nonpublic_nested_import: token for token, a class name `Inner` became `Outer.Inner` (one or more
    capitalized qualifiers put before a name not itself qualified), or a fully qualified `pkg.Outer.Inner`
    lost its package, the lines possibly reflowed."""
    a, b = tokens(old), tokens(new)
    i = j = 0
    found = False
    while j < len(b):
        if i < len(a) and a[i] == b[j]:
            i, j = i + 1, j + 1
        elif (i + 1 < len(a) and a[i + 1] == '.' and IDENT.match(a[i]) and a[i][0].islower()
              and (i == 0 or a[i - 1] != '.') and IDENT.match(b[j]) and b[j][0].isupper()):
            k = i
            while k + 1 < len(a) and a[k + 1] == '.' and IDENT.match(a[k]) and a[k][0].islower():
                k += 2
            if k < len(a) and a[k] == b[j]:
                i, found = k, True
            else:
                return False
        elif (j + 2 < len(b) and b[j + 1] == '.' and IDENT.match(b[j]) and b[j][0].isupper()
              and (j == 0 or b[j - 1] != '.')):
            k = j
            while k + 2 < len(b) and b[k + 1] == '.' and IDENT.match(b[k]) and b[k][0].isupper() and b[k + 2] != (a[i] if i < len(a) else None):
                k += 2
            if k + 2 < len(b) and b[k + 1] == '.' and i < len(a) and b[k + 2] == a[i]:
                j, found = k + 2, True
            else:
                return False
        else:
            return False
    return found and i == len(a)


def override_inaccessible_method(old, new):
    """override_inaccessible_method: `@Override` lines removed and nothing else changed."""
    return bool(old) and not any(x.strip() for x in new) and all(x.strip() == '@Override' for x in old)


def primitive_array_wildcard(old, new):
    """primitive_array_wildcard: token for token, `? extends`/`? super` put before a primitive array type
    argument, and a cast to a primitive array type (with parentheses) added, the lines possibly reflowed."""
    a, b = tokens(old), tokens(new)
    out, i, wild = [], 0, False
    while i < len(b):
        if (b[i] == '?' and i + 3 < len(b) and b[i + 1] in ('extends', 'super') and b[i + 2] in PRIMITIVES
                and b[i + 3] == '['):
            i, wild = i + 2, True
            continue
        if b[i] == '(' and i + 4 < len(b) and b[i + 1] in PRIMITIVES and b[i + 2] == '[':
            k = i + 2
            while k + 1 < len(b) and b[k] == '[' and b[k + 1] == ']':
                k += 2
            if k < len(b) and b[k] == ')':
                i = k + 1
                continue
        out.append(b[i])
        i += 1
    if not wild and out == b:
        return False
    return [t for t in out if t not in '()'] == [t for t in a if t not in '()']


PATTERN = re.compile(r'instanceof ([\w$.<>\[\], ?]+?) (?!extends\b|super\b)([A-Za-z_$][\w$]*)\b(?!\s*[.(<])')


def pattern_variable_scope(old, new):
    """pattern_variable_scope: a pattern `x instanceof T v` became `x instanceof T` with `T v;` declared
    and `v = (T)x;` assigned (the branches possibly inverted): every token that differs is the
    variable, the type, the operand, or punctuation, `if`, `else` and `!`."""
    a_text, b_text = '\n'.join(old), '\n'.join(new)
    patterns = PATTERN.findall(a_text)
    if not patterns:
        return False
    allowed = {'=', '(', ')', ';', '{', '}', 'else', '!', 'if', '.', ',', '<', '>', '[', ']', '?', 'extends',
               'super'}
    for type_, var in patterns:
        if re.search(r'instanceof %s %s\b' % (re.escape(type_), re.escape(var)), b_text):
            continue
        if not re.search(r'(?<![\w$])%s = \(' % re.escape(var), b_text):
            return False
        allowed |= {var} | set(TOKEN.findall(type_))
        for m in re.finditer(r'([\w$.()]+) instanceof %s %s\b' % (re.escape(type_), re.escape(var)), a_text):
            allowed |= set(TOKEN.findall(m.group(1)))
    ca, cb = collections.Counter(tokens(old)), collections.Counter(tokens(new))
    return set((ca - cb) | (cb - ca)) <= allowed


CASE_LABEL = re.compile(r'^(\s*(?:case .*|default):) \{$')


def switch_case_scope(old, new):
    """switch_case_scope: case labels opened with `{` and closed with `}` lines, and locals renamed between
    `name` and `name` with `x`s appended (a case written earlier keeps the name, a later clash is renamed
    or braced), and nothing else."""
    def norm(lines):
        out = []
        for line in lines:
            if line.strip() == '}':
                continue
            m = CASE_LABEL.match(line)
            out.append(m.group(1) if m else line)
        return [re.sub(r'x+$', '', t) if IDENT.match(t) else t for t in tokens(out)]
    return old != new and norm(old) == norm(new)


LOCAL_CLASS = re.compile(r'^\s*(?:(?:final|abstract|static|strictfp) )*(?:class|interface|enum|record) [\w$]+')


def local_class_anonymous_subclass(old, new):
    """local_class_anonymous_subclass: a local class declaration (its lines, and a blank line) moved: the
    hunk only deletes or only inserts lines that start with a local class header and end its body."""
    moved = [x for x in (old or new) if x.strip()]
    return (not old or not new) and bool(moved) and LOCAL_CLASS.match(moved[0]) is not None \
        and moved[-1].strip() == '}'


RESUGAR_NOTE = '// $VF: Unable to resugar Kotlin loop from Java for loop'
KT_DECLARATION = re.compile(r'\s*(?:val|var) [^\s=]+: [^=]+$')
KT_RUN_LABEL = re.compile(r'\s*run (label\d+)@\{$')
KT_DO_HEAD = re.compile(r'\s*(?:(label\d+)@ )?do \{$')
KT_DO_TAIL = re.compile(r'\s*\} while \(run \{ (.+); (.+) \}\)$')
KT_INC = re.compile(r'\s*(?:([^\s+-]+)(\+\+|--)|(\+\+|--)([^\s+-]+))$')
KT_UNTIL_HEAD = re.compile(r'(\s*(?:label\d+@ )?for \(\S+ in .+) until (\S+\) \{)$')
KT_REPEAT_HEAD = re.compile(r'(\s*repeat\()(-?\d+)(\) .*)$')
KT_LOOP_KINDS = {}
KT_TOKEN = re.compile(r'`[^`\n]+`|' + TOKEN.pattern)


def kt_tokens(text_lines):
    """`tokens`, with a backquoted Kotlin name one token."""
    return KT_TOKEN.findall('\n'.join(text_lines))


def indent_of(line):
    return len(line) - len(line.lstrip(' '))


def closing(lines, start, indent):
    """The index of the first line after `start` that is `}` at `indent`, or None."""
    for k in range(start + 1, len(lines)):
        if lines[k].strip() and indent_of(lines[k]) <= indent:
            return k if lines[k].strip().startswith('}') and indent_of(lines[k]) == indent else None
    return None


def kt_loop_segments(lines):
    """The lines of a new-text span as segments: ('line', text) or ('loop', parts) for each loop the
    Kotlin writer prints as `// $VF: Unable to resugar Kotlin loop from Java for loop` and a `while` or
    `do`-`while` (the bodies parsed recursively)."""
    segs, i = [], 0
    while i < len(lines):
        bare = (lines[i].lstrip().startswith('if (') and i + 1 < len(lines)
                and KT_DO_HEAD.match(lines[i + 1]) is not None)
        if lines[i].strip() != RESUGAR_NOTE and not bare:
            segs.append(('line', lines[i]))
            i += 1
            continue
        defs = []
        while not bare and segs and segs[-1][0] == 'line' and KT_DECLARATION.match(segs[-1][1]) \
                and indent_of(segs[-1][1]) == indent_of(lines[i]):
            defs.insert(0, segs.pop()[1])
        loop = {'defs': defs, 'label': None, 'run': False, 'init': [], 'do': False, 'bare': bare}
        j = i if bare else i + 1
        m = KT_RUN_LABEL.match(lines[j]) if j < len(lines) and not bare else None
        if m:
            loop['label'], loop['run'] = m.group(1), True
            j += 1
        while j < len(lines) and not lines[j].lstrip().startswith(('while (', 'if (')):
            loop['init'].append(lines[j])
            j += 1
        if j >= len(lines):
            segs += [('line', x) for x in lines[i:]]
            break
        head = j
        while j < len(lines) and not lines[j].rstrip().endswith(') {'):
            j += 1
        if j >= len(lines):
            segs += [('line', x) for x in lines[i:]]
            break
        cond = ' '.join(x.strip() for x in lines[head:j + 1])
        cond = cond[cond.index('(') + 1:-3].strip()
        loop['cond'], head_indent, body_start = cond, indent_of(lines[head]), j + 1
        if lines[head].lstrip().startswith('if ('):
            d = KT_DO_HEAD.match(lines[body_start]) if body_start < len(lines) else None
            if not d:
                segs += [('line', x) for x in lines[i:body_start]]
                i = body_start
                continue
            loop['do'], loop['label'] = True, d.group(1)
            end = closing(lines, body_start, indent_of(lines[body_start]))
            tail = KT_DO_TAIL.match(lines[end]) if end is not None else None
            if end is None or not tail or (end + 1 < len(lines) and lines[end + 1].strip() != '}'):
                segs += [('line', x) for x in lines[i:body_start + 1]]
                i = body_start + 1
                continue
            loop['inc'], loop['body'] = tail.group(1), kt_loop_segments(lines[body_start + 1:end])
            loop['closed'] = end + 1 < len(lines)
            i = end + 2
        else:
            end = closing(lines, j, head_indent)
            body = lines[body_start:end if end is not None else len(lines)]
            loop['closed'] = end is not None
            loop['inc'] = None
            if len(body) >= 2 and body[-2].strip() == '' and KT_INC.match(body[-1]):
                loop['inc'], body = body[-1].strip(), body[:-2]
            loop['body'] = kt_loop_segments(body)
            i = end + 1 if end is not None else len(lines)
            if loop['run'] and loop['closed']:
                if i < len(lines) and lines[i].strip() == '}':
                    i += 1
                else:
                    loop['closed'] = False
        segs.append(('loop', loop))
    return segs


KT_CONSTANT = re.compile(r"-?\d+L?|'(?:[^'\\]|\\.)+'|true|false|null")


def kt_step(text, delta):
    """A constant bound as Vineflower prints it: an int literal moved by `delta`, a long literal as it
    is; None for a constant whose moved text this check does not compute (a char)."""
    if re.fullmatch(r'-?\d+', text):
        return str(int(text) + delta)
    return text if re.fullmatch(r'-?\d+L', text) else None


def kt_range_header(loop):
    """What Vineflower's range resugaring prints for the loop's header, or None when it would not."""
    if len(loop['init']) != 1 or not loop['inc']:
        return None
    init = re.fullmatch(r'\s*(?:(?:val|var) )?(\S+?)(?:: [^=]+)? = (.+)', loop['init'][0])
    inc = KT_INC.match(loop['inc'])
    cond = re.fullmatch(r'(\S+) < (\S+)', loop['cond'])
    if not init or not inc or not cond:
        return None
    var, start = init.group(1), init.group(2)
    pp = (inc.group(2) or inc.group(3)) == '++'
    left, right = cond.group(1), cond.group(2)
    delta = -1 if pp else 1
    name = re.compile(r'`[^`]+`|[A-Za-z_$][\w$]*')
    if KT_CONSTANT.fullmatch(right) or KT_CONSTANT.fullmatch(left):
        bound = kt_step(right if KT_CONSTANT.fullmatch(right) else left, delta)
        if bound is None:
            return None
    elif left == var and name.fullmatch(right):
        bound = right
    elif right == var and name.fullmatch(left):
        bound = left
    else:
        return 'abandoned'
    label = loop['label']
    if pp and start == '0':
        return f'repeat({bound}) ' + (f'{label}@' if label else '') + '{' + ('' if var == 'it' else f' {var} ->')
    return (f'{label}@ ' if label else '') + f'for ({var} in {start}' + ('..' if pp else ' downTo ') + f'{bound}) {{'


def kt_loop_candidates(loop):
    """(kind, prefix tokens, keep body, suffix tokens) for each text Vineflower may have printed for a loop
    kuna prints as a `while`/`do`-`while`: its own `while (true) { if (C) break` form, its range or
    repeat form, or nothing (`break resugar`)."""
    init = kt_tokens(loop['init'])
    note = kt_tokens([RESUGAR_NOTE]) if not loop.get('bare') else []
    inc = kt_tokens([loop['inc']]) if loop['inc'] else []
    wrapped = loop['run'] or (loop['do'] and loop['label'])
    run = kt_tokens([f'run {loop["label"]}@{{']) if wrapped and not loop.get('bare') else []
    wrapped_end = kt_tokens(['}']) if wrapped else []
    head = kt_tokens([f'while (true) {{ if ({loop["cond"]}) break'])
    out = []
    end = kt_tokens(['}']) if loop['closed'] else []
    out.append(('kt_for_loop_exit_condition', note + run + init + head, True,
                inc + end + (wrapped_end if loop['closed'] else [])))
    header = kt_range_header(loop) if not loop.get('bare') else None
    if header == 'abandoned':
        out.append(('kt_for_range_unmatched_bound', [], False, []))
    elif header:
        out.append(('kt_for_range_counter', kt_tokens([header]), True, end))
    return out


def kt_eq(old, q, new, renames):
    """`renames` extended so that `new` reads as old[q:...] when each of its fresh names (`renames`'
    keys) stands for one old name; None when it cannot."""
    if old[q:q + len(new)] == new:
        return renames
    if len(old) < q + len(new):
        return None
    out = dict(renames)
    for o, n in zip(old[q:q + len(new)], new):
        if o == n:
            continue
        if n not in out or not KT_NAME.fullmatch(o) or (out[n] is not None and out[n] != o):
            return None
        out[n] = o
    return out


def kt_match(segs, old, p, kinds, renames=None):
    """The states (position in `old`, loop kinds used, renames) where the segments can end when matched
    from `p`; Vineflower's texts are tried for each loop."""
    states = [(p, kinds, renames or {})]
    for kind, seg in segs:
        nxt = []
        for q, ks, rn in states:
            if kind == 'line':
                for alt, alt_kind in kt_line_alternatives(seg):
                    t = kt_tokens([alt])
                    r2 = kt_eq(old, q, t, rn)
                    if r2 is not None:
                        nxt.append((q + len(t), ks + ([alt_kind] if alt_kind else []), r2))
                continue
            for defs in ([], seg['defs']) if seg['defs'] else ([],):
                for name, pre, body, post in kt_loop_candidates(seg):
                    pre = kt_tokens(defs) + pre
                    r1 = kt_eq(old, q, pre, rn)
                    if r1 is None:
                        continue
                    ends = kt_match(seg['body'], old, q + len(pre), [], r1) if body else [(q + len(pre), [], r1)]
                    for r, inner, r2 in ends:
                        r3 = kt_eq(old, r, post, r2)
                        if r3 is not None:
                            extra = ['kt_for_loop_definitions'] if seg['defs'] and not defs else []
                            extra += ['kt_for_loop_continue_update'] if seg['do'] else []
                            nxt.append((r + len(post), ks + inner + [name] + extra, r3))
        seen, states = set(), []
        for st in nxt:
            key = (st[0], tuple(st[1]), tuple(sorted(st[2].items(), key=str)))
            if key not in seen:
                seen.add(key)
                states.append(st)
        states = states[:64]
        if not states:
            return []
    return states


def kt_line_alternatives(line):
    """(text, kind) for each text Vineflower may have printed for one line kuna prints: the line itself
    (kind None) or, for an import a fixed method needs, nothing; the inclusive `..` / `repeat(N - 1)` of a range header kuna writes exclusive, or the
    `while (true) { if (C) break` of a loop kuna writes `while (C)`."""
    out = [(line, None)]
    if line.startswith('import '):
        out.append(('', None))
    m = KT_UNTIL_HEAD.match(line)
    if m:
        out.append((f'{m.group(1)}..{m.group(2)}', 'kt_for_range_exclusive_bound'))
    m = KT_REPEAT_HEAD.match(line)
    if m:
        out.append((f'{m.group(1)}{int(m.group(2)) - 1}{m.group(3)}', 'kt_for_range_exclusive_bound'))
    m = re.fullmatch(r'\s*while \((.+)\) \{', line)
    if m:
        out.append((f'while (true) {{ if ({m.group(1)}) break', 'kt_for_loop_exit_condition'))
    return out


KT_NAME = re.compile(r'`[^`]+`|[A-Za-z_$][\w$]*')
KT_DECLARED = re.compile(r'^(\s*)(?:val|var) (`[^`]+`|[\w$]+): [^=]+? = (.*)$')
KT_JAVA_CAST = re.compile(r'\((?:[A-Z][\w.$]*(?:<[^()]*>)?(?:\[\])*|char|byte|short|int|long|float|double|boolean)\)(?=[\w`(])')
KT_AS_CAST = re.compile(r' as [\w.$]+(?:<[^()]*?>)?\??')


def kt_unmerged(lines):
    """The lines with what a split variable changes besides its name taken out: declarations
    without a value, the `val`/`var` and type of a declaration with one, and casts."""
    out = []
    for line in lines:
        if KT_DECLARATION.match(line):
            continue
        m = KT_DECLARED.match(line)
        if m:
            line = f'{m.group(1)}{m.group(2)} = {m.group(3)}'
        out.append(KT_AS_CAST.sub('', kt_uncast(line)))
    return out


def kt_uncast(line):
    """The line without its Java-style casts, and the parentheses around a cast operand."""
    while True:
        m = KT_JAVA_CAST.search(line)
        if not m:
            return line
        rest = line[m.end():]
        if rest.startswith('('):
            depth = 0
            for k, c in enumerate(rest):
                depth += {'(': 1, ')': -1}.get(c, 0)
                if depth == 0:
                    rest = rest[1:k] + rest[k + 1:]
                    break
        line = line[:m.start()] + rest


def kt_fresh_names(old, new):
    """The names `new` uses more often than `old`: the variables a split gave their own name."""
    before, after = {}, {}
    for t in kt_tokens(old):
        before[t] = before.get(t, 0) + 1
    for t in kt_tokens(new):
        after[t] = after.get(t, 0) + 1
    return {t: None for t, n in after.items()
            if KT_NAME.fullmatch(t) and n > before.get(t, 0) and not t.lstrip('`')[:1].isupper()}


def var_merge_liveness(old, new):
    """var_merge_liveness: token for token, the new text is the old one with some uses of a merged
    variable renamed to a fresh name (each standing for one old name), declarations of that name added,
    a declaration's `val`/`var` and type and casts changed."""
    o, n = kt_unmerged(old), kt_unmerged(new)
    fresh = kt_fresh_names(old, new)
    if not fresh or kt_tokens(o) == kt_tokens(n):
        return False
    renames = kt_eq(kt_tokens(o), 0, kt_tokens(n), fresh)
    return renames is not None and len(kt_tokens(o)) == len(kt_tokens(n)) and any(v for v in renames.values())


def kt_for_loops(old, new):
    """kt_for_loop_*, kt_for_range_*: each `for` loop kuna writes differently is, token for token, the
    loop Vineflower printed: `while (true) { if (C) break` (exit condition), a range or `repeat` header
    without the initializer and update (counter, exclusive bound), nothing at all (unmatched bound),
    `..` / `repeat(N - 1)` for `until` / `repeat(N)` (exclusive bound), no declarations before the loop
    (definitions), or a `while` for a `do`-`while` (continue update)."""
    if not [x for x in old if x.strip()] and new and all(KT_DECLARATION.match(x) for x in new):
        KT_LOOP_KINDS['kt_for_loop_definitions'] = KT_LOOP_KINDS.get('kt_for_loop_definitions', 0) + 1
        return True
    segs = kt_loop_segments(new)
    a = kt_tokens(old)
    if kt_tokens(new) == a:
        return False
    for q, kinds, _ in kt_match(segs, a, 0, []):
        if q == len(a) and kinds:
            for k in kinds:
                KT_LOOP_KINDS[k] = KT_LOOP_KINDS.get(k, 0) + 1
            return True
    fresh = kt_fresh_names(old, new)
    if fresh:
        a = kt_tokens(kt_unmerged(old))
        for q, kinds, renames in kt_match(kt_loop_segments(kt_unmerged(new)), a, 0, [], fresh):
            if q == len(a) and kinds:
                for k in kinds + ['var_merge_liveness']:
                    KT_LOOP_KINDS[k] = KT_LOOP_KINDS.get(k, 0) + 1
                return True
    return False


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
    'kt_constructor_diamond': kt_constructor_diamond,
    'kt_local_visibility_comment': kt_local_visibility_comment,
    'kt_vararg_from_metadata': kt_vararg_from_metadata,
    'kt_default_args_ctor_descriptor': kt_default_args_ctor_descriptor,
    'switch_expression_poly_cast': switch_expression_poly_cast,
    'ternary_numeric_arms': ternary_numeric_arms,
    'var_merge_narrowing': var_merge_narrowing,
    'kt_callable_reference': kt_callable_reference,
    'nonpublic_nested_import': nonpublic_nested_import,
    'override_inaccessible_method': override_inaccessible_method,
    'primitive_array_wildcard': primitive_array_wildcard,
    'pattern_variable_scope': pattern_variable_scope,
    'switch_case_scope': switch_case_scope,
    'local_class_anonymous_subclass': local_class_anonymous_subclass,
    'kt_for_loops': kt_for_loops,
    'var_merge_liveness': var_merge_liveness,
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
        if all(is_import(x) for x in new):
            return 'import'
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
