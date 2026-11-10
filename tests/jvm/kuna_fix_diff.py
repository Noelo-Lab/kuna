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

A recogniser's table key is its fix's `KunaFix` id, or a FIX_GROUPS key listing the ids it stands for. The
tool checks itself before it runs (kuna_fix_check.py: no name defined twice, every table entry a function
of this file naming a fix); `make test-tools` runs that check and every recogniser on small hunks.
"""
import collections
import difflib
import functools
import importlib.util
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
    return list(_tokens(tuple(text_lines)))


@functools.lru_cache(maxsize=4096)
def _tokens(text_lines):
    return tuple(TOKEN.findall('\n'.join(text_lines)))


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
    if j < len(toks) and toks[j] == '<':
        depth = 0
        while j < len(toks):
            depth += {'<': 1, '>': -1}.get(toks[j], 0)
            j += 1
            if depth == 0:
                break
    if j < len(toks) and toks[j] == '?':
        j += 1
    return j


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
            end = kotlin_type_end(b, j + 2)
            if end is None:
                return False
            i, j, found = i + 4, end, True
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


CAST_BEFORE = {'(', ',', '=', 'return', ':', '?', '&', '|', '!', '+', '-', '*', '/', '%', '<', '>', '{', ';', '['}
BOXES = {'Boolean', 'Byte', 'Character', 'Short', 'Integer', 'Long', 'Float', 'Double'}


def cast_type_close(toks, i):
    """The index of the `)` closing a type that starts at toks[i] (a cast's type), or None."""
    depth, expect_name, j = 0, True, i
    while j < len(toks):
        t = toks[j]
        if t == ')' and depth == 0 and not expect_name:
            return j
        if WORD.fullmatch(t):
            if not expect_name and t not in ('extends', 'super'):
                return None
            expect_name = t in ('extends', 'super')
        elif t == '.' or t == '&':
            expect_name = True
        elif t == '<':
            depth, expect_name = depth + 1, True
        elif t == '>':
            depth -= 1
            if depth < 0:
                return None
        elif t == ',' and depth > 0:
            expect_name = True
        elif t == '?' and depth > 0:
            expect_name = False
        elif t in ('[', ']'):
            expect_name = False
        else:
            return None
        j += 1
    return None


def operand_start(t):
    return t not in ('+', '-') and (WORD.fullmatch(t) is not None or t[0] in '("\'!~' or t[0].isdigit())


def cast_at(toks, i):
    """When toks[i] opens a cast `( Type )` followed by its operand, the index of the `)`."""
    if toks[i] != '(' or (i > 0 and toks[i - 1] not in CAST_BEFORE):
        return None
    j = cast_type_close(toks, i + 1)
    return j if j is not None and j + 1 < len(toks) and operand_start(toks[j + 1]) else None


def unparen(toks):
    """`( X )` dropped where X is a primary (no operator at its top level) and the parentheses are not a
    call's or a statement's."""
    toks, i = list(toks), 0
    while i < len(toks):
        if toks[i] != '(' or (i > 0 and toks[i - 1] not in CAST_BEFORE) or cast_at(toks, i) is not None:
            i += 1
            continue
        end = block_end(toks, i)
        if end is None:
            i += 1
            continue
        inner = [toks[k] for k in top_level(toks, i + 1, end)]
        if inner and not any(x in ('?', '=', ',', '+', '-', '*', '/', '<', '>', 'instanceof', ':', '%', '&',
                                   '|', '^', '!') for x in inner):
            del toks[end]
            del toks[i]
            continue
        i += 1
    return toks


def strip_casts(toks):
    """The tokens with every cast removed and the parentheses that only grouped a cast's operand dropped."""
    return list(_strip_casts(tuple(toks)))


@functools.lru_cache(maxsize=4096)
def _strip_casts(toks):
    out, i = [], 0
    while i < len(toks):
        j = cast_at(toks, i)
        if j is not None:
            i = j + 1
            continue
        out.append(toks[i])
        i += 1
    return tuple(unparen(out))


def cast_only(old, new):
    """The hunk only adds, drops or changes casts (and the parentheses around a cast's operand)."""
    a, b = tokens(old), tokens(new)
    return a != b and strip_casts(a) == strip_casts(b)


def generic_patterns(toks):
    """The variables of the type patterns `instanceof T<...> v` (or of a type variable, `instanceof T v`)."""
    out = []
    for i, t in enumerate(toks):
        if t != 'instanceof':
            continue
        j = i + 1
        while j + 2 < len(toks) and toks[j + 1] == '.':
            j += 2
        generic = j + 1 < len(toks) and toks[j + 1] == '<'
        if generic:
            depth = 0
            for k in range(j + 1, len(toks)):
                depth += {'<': 1, '>': -1}.get(toks[k], 0)
                if depth == 0:
                    j = k
                    break
        if j + 1 < len(toks) and WORD.fullmatch(toks[j + 1]) and toks[j + 1] not in ('instanceof',):
            out.append((generic, toks[j + 1]))
    return out


def unchecked_type_pattern(old, new):
    """unchecked_type_pattern: type patterns of a generic type or a type variable are gone, each now an
    `instanceof` of the class and a declaration of the pattern's variable from a cast of the operand."""
    a, b = tokens(old), tokens(new)
    gone = [v for v in generic_patterns(a) if v not in generic_patterns(b)]
    if not gone or 'instanceof' not in b:
        return False
    return all(any(b[k] == var and b[k + 1:k + 3] == ['=', '('] for k in range(len(b) - 2)) for _, var in gone)


def unbox_casts(toks):
    """`( T ) Box . valueOf ( X )` written `( T ) X`."""
    out, i = [], 0
    while i < len(toks):
        if (toks[i] in BOXES and i + 3 < len(toks) and toks[i + 1:i + 4] == ['.', 'valueOf', '(']
                and out and out[-1] == ')'):
            end = block_end(toks, i + 3)
            if end is not None:
                out += toks[i + 4:end]
                i = end + 1
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


def renamable(a, b):
    """A renaming of names in `b` and dropping `v = v ;` copies can give `a`: it keeps every other token,
    and each copy drops two names, one `=` and one `;`."""
    copies, rest = divmod(len(b) - len(a), 4)
    if copies < 0 or rest:
        return False
    other_a = collections.Counter(t for t in a if not IDENT.match(t))
    other_b = collections.Counter(t for t in b if not IDENT.match(t))
    other_b.subtract({'=': copies, ';': copies})
    return +other_b == other_a and min(other_b.values(), default=0) >= 0


def var_merge_narrowing(old, new):
    """var_merge_narrowing: the same statements once a variable the fix keeps apart from another (its own
    name, declaration and copy `v = w`) is renamed back onto it, and casts, declarations and self-copies
    `v = v` are dropped."""
    a, b = without_self_copies(undeclared(old)), undeclared(new)
    if old == new:
        return False
    if a == b:
        return True
    if not renamable(a, b):
        return False
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


def boxed_type_variable_cast(old, new):
    """boxed_type_variable_cast: a cast's operand `X` became `Box.valueOf(X)`, nothing else changed."""
    a, b = tokens(old), tokens(new)
    return a != b and unbox_casts(b) == a


def lambda_argument_overload(old, new):
    """lambda_argument_overload: a lambda or method reference argument became `(Interface)(lambda)`."""
    a, b = tokens(old), tokens(new)
    out, i, found = [], 0, False
    while i < len(b):
        j = cast_at(b, i)
        if j is not None and b[j + 1] == '(':
            end = block_end(b, j + 1)
            inner = [b[k] for k in top_level(b, j + 2, end)] if end is not None else []
            if any(x in ('-', ':') and y in ('>', ':') for x, y in zip(inner, inner[1:])):
                out += b[j + 2:end]
                i, found = end + 1, True
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


def x_renamed(old, new):
    """The hunk's tokens are equal once trailing `x`s are stripped from identifiers, and differ."""
    def norm(lines):
        return [re.sub(r'x+$', '', t) if IDENT.match(t) else t for t in tokens(lines)]
    return old != new and len(old) == len(new) and norm(old) == norm(new)


def changed_lines(old, new):
    return [o for o, n in zip(old, new) if o != n] + [n for o, n in zip(old, new) if o != n]


def lambda_catch_var_rename(old, new):
    """lambda_catch_var_rename: a catch clause's variable renamed (`e` -> `ex`), nothing else."""
    return x_renamed(old, new) and all('} catch (' in line for line in changed_lines(old, new))


def nested_param_rename(old, new):
    """nested_param_rename: a method's parameter declared without the `x`s its uses do not have."""
    return x_renamed(old, new) and not any('catch (' in line for line in changed_lines(old, new))


QUALIFIER = re.compile(r'(?<![\w$.])(?:[\w$]+\.)+(?=@|[A-Z])')


def type_annotation_qualified_name(old, new):
    """type_annotation_qualified_name: in lines with a type annotation, qualifiers (`Map.`, `a.b.`)
    inserted before the annotation or before a type name inside the annotated type."""
    return old != new and len(old) == len(new) and all(
        o == n or ('@' in n and QUALIFIER.sub('', n) == QUALIFIER.sub('', o)) for o, n in zip(old, new))


ANNOTATION = re.compile(r'@[\w$.]+ ')


def type_annotation_argument_scope(old, new):
    """type_annotation_argument_scope: the same annotations, moved between the arguments of a type."""
    return old != new and len(old) == len(new) and all(
        ANNOTATION.sub('', o) == ANNOTATION.sub('', n) and
        sorted(ANNOTATION.findall(o)) == sorted(ANNOTATION.findall(n)) for o, n in zip(old, new))


def assert_throw_after_if(old, new):
    """assert_throw_after_if: `$assertionsDisabled` (with its resugar note) replaced by an `assert`, or
    the leftover `throw new AssertionError(..)` of such an `if` dropped."""
    o, n = '\n'.join(old), '\n'.join(new)
    if '$assertionsDisabled' in n:
        return False
    if '$assertionsDisabled' in o:
        return 'assert ' in n
    if 'Could not resugar all assert statements' in o and not n.strip():
        return True
    rest = [line.strip() for line in old if 'throw new AssertionError' not in line]
    return 'throw new AssertionError' in o and 'throw new AssertionError' not in n and \
        all(r in ('', '}', '} else {') or r.startswith(('return', 'continue', 'break')) for r in rest)


def sealed_private_permits(old, new):
    """sealed_private_permits: a sealed class header loses its `permits` clause."""
    return len(old) == len(new) == 1 and ' sealed ' in old[0] and ' permits ' in old[0] and \
        re.sub(r' permits [^{]+(?= \{$)', '', old[0]) == new[0]


def switch_expression_assignments(old, new):
    """switch_expression_assignments: a switch expression with `yield`s or arms becomes the switch
    statement assigning a variable (`case X:`, `v = ...;`, `break;`)."""
    o, n = '\n'.join(old), '\n'.join(new)
    arms = ('yield ' in o or ' -> ' in o or 'switch (' in o)
    statement = ('break;' in n or re.search(r'case [^\n]*:$', n, re.M) or re.search(r'\bvar\d+ = ', n) or 'default:' in n)
    return bool(arms and statement) and 'yield ' not in n


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


KT_SEMAPHORE_OLD = (re.compile(r'\s*val var\d+: (?:Boolean|<unknown>)$'), re.compile(r'\s*val var\d+: Boolean = true$'))
KT_SEMAPHORE_NEW = (re.compile(r'\s*var var\d+: Boolean = false /\* VF: Semaphore variable \*/$'),
                    re.compile(r'\s*var\d+ = (?:true|false)$'))
KT_SEMAPHORE_STORE = re.compile(r'\s*(var\d+) = (?:true|false)$')


def kt_semaphore_lines(lines, patterns):
    return [x for x in lines if x.strip() and not any(p.match(x) for p in patterns)]


def kt_semaphore_store_liveness(old, new):
    """kt_semaphore_store_liveness: a finally semaphore's stores kept. The new lines are the old ones
    without the semaphore's orphan declaration after its guard (`val varN: Boolean`, `val varN: <unknown>`)
    and the misdirected first store (`val varM: Boolean = true`), plus the semaphore's declaration
    (`var varN: Boolean = false /* VF: Semaphore variable */`) and its stores (`varN = true`/`false`)."""
    return (old != new and any(p.match(x) for x in old + new for p in KT_SEMAPHORE_OLD + KT_SEMAPHORE_NEW)
            and kt_semaphore_lines(old, KT_SEMAPHORE_OLD) == kt_semaphore_lines(new, KT_SEMAPHORE_NEW))


def kt_semaphore_store_liveness_scope(a, b, i1, i2, j1, j2, throw_names):
    """kt_semaphore_store_liveness, where a kept store changes the statements around it: the Java
    `break labelN;` Vineflower left inside a nested try becomes `varS = false` + `return@labelN` after
    it (one hunk each), or a copy of a variable the Kotlin pass inlined into a semaphore's guard,
    `(E as T)`, is read through the variable `V = E as T` assigned before it."""
    old, new = a[i1:i2], b[j1:j2]
    near_a, near_b = a[max(0, i1 - 12):i2 + 12], b[max(0, j1 - 12):j2 + 12]
    brk = re.fullmatch(r'\s*break (label\d+);', old[0]) if len(old) == 1 else None
    if brk and not new:
        return (any(x.strip() == f'return@{brk.group(1)}' for x in near_b)
                and any(KT_SEMAPHORE_STORE.match(x) for x in near_b))
    ret = [x for x in new if x.strip()]
    if not old and len(ret) == 2 and KT_SEMAPHORE_STORE.match(ret[0]):
        m = re.fullmatch(r'\s*return@(label\d+)', ret[1])
        return bool(m) and any(x.strip() == f'break {m.group(1)};' for x in near_a)
    if len(old) == 1 and len(new) == 1:
        for line in b[max(0, j1 - 40):j1]:
            m = re.fullmatch(r'\s*([\w$`]+) = (.+)', line)
            if m and old[0] in (new[0].replace(m.group(1), f'({m.group(2)})'),
                                new[0].replace(m.group(1), m.group(2))) and old[0] != new[0]:
                return any(p.match(x) for x in near_b for p in KT_SEMAPHORE_NEW)
    return False
def raw_receiver_erasure(old, new):
    """raw_receiver_erasure: casts only, and among them an array cast of a `toArray` call or the cast of a
    for-each's iterable (`for (T x : (C<T>) e)`)."""
    if not cast_only(old, new):
        return False
    b = tokens(new)
    for i in range(len(b)):
        j = cast_at(b, i)
        if j is None:
            continue
        if b[j - 1] == ']' and 'toArray' in b[j + 1:]:
            return True
        if i > 0 and b[i - 1] == ':' and '<' in b[i:j]:
            return True
    return False


def undiamond(toks):
    """`new C<>(` written `new C(`."""
    out = []
    for i, t in enumerate(toks):
        if t == '<' and toks[i + 1:i + 3] == ['>', '('] and 'new' in toks[max(0, i - 8):i]:
            continue
        if t == '>' and i > 0 and toks[i - 1] == '<' and toks[i + 1:i + 2] == ['(']:
            continue
        out.append(t)
    return out


def unchecked_generic_cast(old, new):
    """unchecked_generic_cast: casts added, each of `this`, a field or a variable (no call in its operand)."""
    if not cast_only(old, new):
        return False
    a, b = tokens(old), tokens(new)
    casts = [i for i in range(len(b)) if cast_at(b, i) is not None]
    if len(casts) <= sum(cast_at(a, i) is not None for i in range(len(a))):
        return False
    for i in casts:
        k = cast_at(b, i) + 1
        while k < len(b) and b[k] not in (';', ')', ',', '?', ':', '='):
            if b[k] == '(' and cast_at(b, k) is None:
                break
            k += 1
        if k < len(b) and b[k] == '(':
            return False
    return True


def static_type_variable_scope(old, new):
    """static_type_variable_scope: casts dropped (none added) and diamonds added, nothing else changed."""
    a, b = tokens(old), undiamond(tokens(new))
    if a == b or strip_casts(a) != strip_casts(b):
        return False
    return sum(cast_at(b, i) is not None for i in range(len(b))) < sum(cast_at(a, i) is not None for i in range(len(a)))


def inner_method_argument_shift(old, new):
    """inner_method_argument_shift: casts only."""
    return cast_only(old, new)
# --- kuna fixes of the Kotlin printer (ktprint): the inverse of each fix's change, line by line ---

KT_CONVERSIONS = [  # longest first: the Kotlin conversion and the Java cast it stands for
    ('.code.toLong()', 'long'), ('.code.toFloat()', 'float'), ('.code.toDouble()', 'double'),
    ('.code.toByte()', 'byte'), ('.code.toShort()', 'short'),
    ('.toInt().toChar()', 'char'), ('.toInt().toByte()', 'byte'), ('.toInt().toShort()', 'short'),
    ('.toChar()', 'char'), ('.toByte()', 'byte'), ('.toShort()', 'short'), ('.toInt()', 'int'),
    ('.toLong()', 'long'), ('.toFloat()', 'float'), ('.toDouble()', 'double'), ('.code', 'int'),
]
KT_CLOSE = {')': '(', ']': '['}


def kt_operand_start(s, end):
    """The start of the postfix operand that ends at `s[end]`: a parenthesised expression, or a chain of
    names, calls, indexing and member accesses (backquoted names and literals included)."""
    i = end
    while i > 0:
        c = s[i - 1]
        if c in KT_CLOSE:
            depth, j = 0, i - 1
            while j >= 0:
                if s[j] in '"\'':
                    q, j = s[j], j - 1
                    while j >= 0 and not (s[j] == q and (j == 0 or s[j - 1] != '\\')):
                        j -= 1
                elif s[j] in KT_CLOSE:
                    depth += 1
                elif s[j] in '([':
                    depth -= 1
                    if depth == 0:
                        break
                j -= 1
            if j < 0:
                return None
            group, i = c, j
            if group == ')' and (i == 0 or not (s[i - 1].isalnum() or s[i - 1] in '_$`>)]/')):
                return i
            continue
        if c == '/' and i > 1 and s[i - 2] == '*':
            j = s.rfind('/*', 0, i - 2)
            if j < 0:
                return None
            i = j
            continue
        if c == '`':
            j = s.rfind('`', 0, i - 1)
            if j < 0:
                return None
            i = j
        elif c.isalnum() or c in '_$':
            while i > 0 and (s[i - 1].isalnum() or s[i - 1] in '_$'):
                i -= 1
        elif c in '"\'':
            j = i - 2
            while j >= 0 and not (s[j] == c and (j == 0 or s[j - 1] != '\\')):
                j -= 1
            if j < 0:
                return None
            i = j
        else:
            return i if i < end else None
        if i > 1 and s[i - 1] == '.' and (s[i - 2].isalnum() or s[i - 2] in '_$`)]"\''):
            i -= 1
            continue
        if i > 2 and s[i - 2:i] == '::':
            i -= 2
            continue
        return i
    return i


def kt_uncast_line(line, old_text):
    """`line` with each Kotlin conversion `X.toInt()` written as Java's cast `(int)X`, and each cast to a
    type variable `X as T` as `(T)X` when the old text has that cast (fix kt_java_cast_syntax)."""
    changed = True
    while changed:
        changed = False
        best = None
        for suffix, java in KT_CONVERSIONS:
            k = line.find(suffix)
            while k >= 0:
                after = line[k + len(suffix):k + len(suffix) + 1]
                if not after or not (after.isalnum() or after in '_$('):
                    break
                k = line.find(suffix, k + 1)
            if k >= 0 and (best is None or k < best[0] or (k == best[0] and len(suffix) > len(best[1]))):
                best = (k, suffix, java)
        if best:
            k, suffix, java = best
            start = kt_operand_start(line, k)
            if start is not None:
                operand = line[start:k]
                if re.fullmatch(r'\(-[\d.]+[LlFfDd]?\)', operand):
                    operand = operand[1:-1]
                line = line[:start] + f'({java})' + operand + line[k + len(suffix):]
                changed = True
                continue
        for m in re.finditer(r' as ([A-Za-z_]\w*)(?![\w.<?])', line):
            if f'({m.group(1)})' not in old_text:
                continue
            start = kt_operand_start(line, m.start())
            if start is not None and line[:start].endswith(' as '):
                inner = kt_operand_start(line, start - 4)
                if inner is not None:
                    line = (line[:inner] + f'({m.group(1)})(' + line[inner:m.start()] + ')'
                            + line[m.end():])
                    changed = True
                    break
            if start is not None:
                line = line[:start] + f'({m.group(1)})' + line[start:m.start()] + line[m.end():]
                changed = True
                break
    return line


KT_CHAR = re.compile(r"'(?:[^'\\]|\\.|\\u[0-9a-fA-F]{4})'")


def kt_char_value(tok):
    body = tok[1:-1]
    if body.startswith('\\u'):
        return int(body[2:], 16)
    if body.startswith('\\'):
        return {'b': 8, 't': 9, 'n': 10, 'f': 12, 'r': 13, "'": 39, '"': 34, '\\': 92, '$': 36}.get(body[1])
    return ord(body) if len(body) == 1 else None


def kt_same_constant(o, n):
    """Old and new token are one constant, as an int and as a char (fix kt_when_case_type)."""
    for x, y in ((o, n), (n, o)):
        if KT_CHAR.fullmatch(x) and re.fullmatch(r'-?\d+', y):
            return kt_char_value(x) == int(y)
    return False


def kt_token_undo(old, new, fired):
    """`new` with the tokens fixes kt_type_parameter_id (`T` for `Any`) and kt_when_case_type (`':'` for
    `58` on a case label) changed put back, when the two lines differ only there."""
    a, b = kt_tokens([old]), kt_tokens([new])
    if len(a) != len(b) or a == b:
        return None
    kinds = set()
    for x, y in zip(a, b):
        if x == y:
            continue
        if x == 'Any' and re.fullmatch(r'[A-Za-z_]\w*', y):
            kinds.add('kt_type_parameter_id')
        elif kt_same_constant(x, y) and ' -> ' in new:
            kinds.add('kt_when_case_type')
        else:
            return None
    fired.update(kinds)
    return old


def kt_paren_undo(old, new, fired):
    """`new` without the parentheses fix kt_range_operand_precedence puts around a range bound."""
    for m in re.finditer(r'(?<=\bin )\(|(?<=\.\.)\(|(?<= until )\(|(?<= downTo )\(', new):
        depth = 0
        for j in range(m.start(), len(new)):
            depth += {'(': 1, ')': -1}.get(new[j], 0)
            if depth == 0:
                cand = new[:m.start()] + new[m.start() + 1:j] + new[j + 1:]
                if cand == old:
                    fired.add('kt_range_operand_precedence')
                    return old
                break
    return None


def kt_line_undo(old, new, fired):
    """One line of new text with the ktprint fixes undone, as close to `old` as they allow."""
    if old == new:
        return new
    line = new
    m = re.search(r'\blambda_\w+@(?=\{)', line)
    if m:
        line = line[:m.start()] + line[m.end():]
        fired.add('kt_lambda_singleton_returns')
    line2 = re.sub(r'return@lambda_\w+ ?', '', line)
    if line2 != line:
        line = line2
        fired.add('kt_lambda_singleton_returns')
    if line == old:
        return line
    m = re.fullmatch(r'(.*\bfun .*\)|\s*\)): [^{]+( \{)', line)
    if m and m.group(1) + m.group(2) == old:
        fired.add('kt_method_object_return_type')
        return old
    uncast = kt_uncast_line(line, old)
    if uncast != line:
        fired.add('kt_java_cast_syntax')
        line = uncast
    if line == old:
        return line
    for undo in (kt_token_undo, kt_paren_undo):
        got = undo(old, line, fired)
        if got is not None:
            return got
    return line


def ktprint_undo(old, new):
    """The ktprint fixes whose change undone line by line turns `new` into `old`, else None."""
    fired = _ktprint_undo(tuple(old), tuple(new))
    return None if fired is None else set(fired)


@functools.lru_cache(maxsize=1024)
def _ktprint_undo(old, new):
    old, new = list(old), list(new)
    fired = set()
    plain = kt_unjava_char_casts(old)
    if plain != old:
        fired.add('kt_java_cast_syntax')
        old = plain
    if len(old) == len(new):
        if [kt_line_undo(o, n, fired) for o, n in zip(old, new)] == old:
            return frozenset(fired)
    fired = kt_joined_undo(old, new, fired)
    return None if fired is None else frozenset(fired)


def kt_joined_undo(old, new, fired):
    """`ktprint_undo` over a hunk whose lines the fixes reflowed: undone as one text, then compared token
    by token, where a type parameter's name may stand for `Any`, a character for its code and a
    parenthesised range bound for the bare one."""
    lines_ = []
    for n in new:
        n2 = re.sub(r'\blambda_\w+@(?=\{)', '', n)
        n2 = re.sub(r'return@lambda_\w+ ?', '', n2)
        if n2 != n:
            fired.add('kt_lambda_singleton_returns')
        lines_.append(n2)
    old_text = '\n'.join(old)
    joined = kt_uncast_line('\n'.join(lines_), old_text)
    if joined != '\n'.join(lines_):
        fired.add('kt_java_cast_syntax')
    a, b = kt_tokens(old), kt_tokens([joined])
    if len(a) != len(b):
        return fired if a == b else None
    for x, y in zip(a, b):
        if x == y:
            continue
        if x == 'Any' and re.fullmatch(r'[A-Za-z_]\w*', y):
            fired.add('kt_type_parameter_id')
        elif kt_same_constant(x, y):
            fired.add('kt_when_case_type')
        else:
            return None
    return fired or None


KTPRINT_ORDER = ['kt_type_parameter_id', 'kt_java_cast_syntax', 'kt_when_case_type',
                 'kt_method_object_return_type', 'kt_range_operand_precedence',
                 'kt_lambda_singleton_returns', 'kt_secondary_constructor_line']


def ktprint_recogniser(fix):
    """The recogniser of one ktprint fix: the hunk is undone by the ktprint fixes, the first of which
    (in KTPRINT_ORDER) is `fix`; a hunk several of them change counts under the first."""
    def recognise(old, new):
        fired = ktprint_undo(old, new)
        return bool(fired) and next(f for f in KTPRINT_ORDER if f in fired) == fix
    recognise.__name__ = fix
    return recognise


def kt_secondary_constructor_line(old, new):
    """kt_secondary_constructor_line: `constructor(..) : this(..)}` becomes the constructor line and
    the brace on the next (and a blank line follows a bodiless constructor that is not the last
    member); the same tokens."""
    return (any(re.search(r'\bconstructor\(.*\) : (this|super)\(.*\)\s*\}$', x) for x in old)
            and tokens(old) == tokens(new))


def kt_unjava_char_casts(text_lines):
    """Old text with Java's identity cast of a character literal, `(char)'x'`, written as the literal,
    which fix kt_java_cast_syntax writes without a conversion."""
    return [re.sub(r"\(char\)(" + KT_CHAR.pattern + ")", r'\1', x) for x in text_lines]


def ktprint_context(a, b, i1, i2, j1, j2, throw_names):
    """A ktprint hunk that only its surroundings explain: a cast whose operand starts or ends on a
    line next to the hunk (undone over the hunk and up to 8 lines on each side, as one text), or the
    blank line kt_secondary_constructor_line writes after a bodiless secondary constructor, followed
    by other ktprint changes. Returns the fix, or None."""
    old, new = a[i1:i2], b[j1:j2]
    lead = None
    if new and new[0] == '' and j1 > 0 and re.search(r'\bconstructor\(.*\) : (this|super)\(.*\)$', b[j1 - 1]):
        lead, new = 'kt_secondary_constructor_line', new[1:]
        if not old and not new:
            return lead
        if kt_secondary_constructor_line(old, new) or ktprint_undo(old, new):
            return lead
    for k in range(0, 9):
        if i1 - k < 0 or j1 - k < 0:
            break
        for m in range(0, 9):
            if i2 + m > len(a) or j2 + m > len(b):
                break
            old_w, new_w = a[i1 - k:i2 + m], b[j1 - k:j2 + m]
            if lead:
                new_w = new_w[:k] + new_w[k + 1:]
            fired = set()
            plain = kt_unjava_char_casts(old_w)
            if plain != old_w:
                fired.add('kt_java_cast_syntax')
            fired = kt_joined_undo(plain, new_w, fired)
            if fired:
                return lead or next(f for f in KTPRINT_ORDER if f in fired)
    return None


def ktprint_canon(old, new):
    """`new` with the ktprint fixes' conversions, casts and lambda labels written back line by line
    (the casts only where `old` has them), for the recognisers of other fixes whose hunks hold them."""
    old_text = '\n'.join(old)
    out = []
    for k, line in enumerate(new):
        line = re.sub(r'\blambda_\w+@(?=\{)', '', line)
        line = re.sub(r'return@lambda_\w+ ?', '', line)
        line = kt_uncast_line(line, old_text)
        if len(old) == len(new):
            if old[k].endswith(') {'):
                line = re.sub(r'\): [^{]+ \{$', ') {', line)
            plain = kt_token_undo(old[k], line, set())
            if plain is not None:
                line = plain
        out.append(line)
    return out


KTPRINT_CONTEXT_LAST = [None, None]


def ktprint_context_once(a, b, i1, i2, j1, j2, throw_names):
    """`ktprint_context`, computed once for the hunk the recognisers of all its fixes ask about in turn."""
    key = (a, b, i1, i2, j1, j2)
    last = KTPRINT_CONTEXT_LAST[0]
    if last is None or last[0] is not a or last[1] is not b or last[2:] != key[2:]:
        KTPRINT_CONTEXT_LAST[:] = [key, ktprint_context(a, b, i1, i2, j1, j2, throw_names)]
    return KTPRINT_CONTEXT_LAST[1]


def ktprint_context_recogniser(fix):
    def recognise(a, b, i1, i2, j1, j2, throw_names):
        return ktprint_context_once(a, b, i1, i2, j1, j2, throw_names) == fix
    recognise.__name__ = fix + '_context'
    return recognise


# --- wrong2 fixes ---

JUMP_LINE = re.compile(r'^\s*(?:(?:break|continue)(?: [\w$]+)?;|[\w$]+:)\s*$')


def loop_body_exit_explicit(old, new):
    """loop_body_exit_explicit: only `break;`/`continue label;` lines (or a label) added."""
    keep = [x for x in new if not JUMP_LINE.match(x)]
    return len(keep) < len(new) and tokens(keep) == tokens(old)


def boolean_literals_as_ints(old, new, before):
    """Token for token, `false`/`true` became `0`/`1`, each right after a token in `before`."""
    a, b = tokens(old), tokens(new)
    if len(a) != len(b) or a == b:
        return False
    return all(x == y or (x, y) in (('false', '0'), ('true', '1')) and i > 0 and a[i - 1] in before
               for i, (x, y) in enumerate(zip(a, b)))


INTEGRAL_DECLARATION = re.compile(r'^\s*(?:final )?(?:int|short|byte|char) [\w$]+ = [01];\s*$')


def compound_shift_count_type(old, new):
    """compound_shift_count_type: a comparison's `false`/`true` operand became `0`/`1`."""
    return (boolean_literals_as_ints(old, new, ('=',))
            and all(('!=' in n or '==' in n) and not INTEGRAL_DECLARATION.match(n)
                    for o, n in zip(old, new) if o != n))


def definition_const_type(old, new):
    """definition_const_type: an integral declaration's `false`/`true` initializer became `0`/`1`."""
    return (boolean_literals_as_ints(old, new, ('=',))
            and all(INTEGRAL_DECLARATION.match(n) for o, n in zip(old, new) if o != n))


def diamond_argument_conflict(old, new):
    """diamond_argument_conflict: diamonds `<>` removed from `new C<>(`, nothing else changed."""
    a, b = tokens(old), tokens(new)
    out, i = [], 0
    while i < len(a):
        if a[i] == '<' and i + 2 < len(a) and a[i + 1] == '>' and a[i + 2] == '(':
            i += 2
            continue
        out.append(a[i])
        i += 1
    return a != b and out == b


def catch_tail_inline(old, new):
    """catch_tail_inline: one closing `}` line moved down past statements (into the catch clause)."""
    a, b = [x for x in old if x.strip()], [x for x in new if x.strip()]
    if len(a) != len(b) or a == b:
        return False
    for i, x in enumerate(a):
        if x.strip() == '}':
            rest = a[:i] + a[i + 1:]
            for j in range(i + 1, len(b)):
                if b[j].strip() == '}' and [y.strip() for y in b[:j] + b[j + 1:]] == [y.strip() for y in rest]:
                    return True
    return False


KT_DEFAULT_LITERAL = re.compile(r"""(?:null|true|false|-?\d+(?:\.\d+)?[LFDfd]?(?:\.to(?:Char|Byte|Short)\(\))?|'(?:[^'\\]|\\.+)'"""
                                r"""|\([\w$.]+\)null|null as [\w$.]+\??)""")


def kt_default_split(toks, i):
    """The top-level arguments (token lists) of the call whose `(` is toks[i], and the index after its `)`."""
    args, cur, depth, j = [], [], 0, i + 1
    while j < len(toks):
        t = toks[j]
        if t in '([{':
            depth += 1
        elif t in ')]}':
            if depth == 0:
                if cur or args:
                    args.append(cur)
                return args, j + 1
            depth -= 1
        elif t == ',' and depth == 0:
            args.append(cur)
            cur = []
            j += 1
            continue
        cur.append(t)
        j += 1
    if cur:
        args.append(cur)
    return args, j


def kt_default_chain(toks, i):
    """The end of the qualifier chain `W(.W)*` starting at toks[i] when it is followed by `(`, else None."""
    j = i
    while j < len(toks) and WORD.fullmatch(toks[j]):
        if j + 1 < len(toks) and toks[j + 1] == '(':
            return j
        if j + 1 < len(toks) and toks[j + 1] == '.':
            j += 2
            continue
        return None
    return None


def kt_default_args_match(a, b):
    """Whether the tokens `b` are `a` with calls through default-arguments bridges resugared (see
    kt_default_args_call). A call cut off by the end of the lines counts as ending there; string
    templates are compared by their tokens."""
    return _kt_default_match(kt_default_templates(a), kt_default_templates(b))


def kt_default_templates(toks):
    """The tokens with each string literal that holds a template (`${`) split into its tokens."""
    out = []
    for t in toks:
        if t.startswith('"') and '${' in t:
            out += ['"'] + TOKEN.findall(t[1:-1]) + ['"']
        else:
            out.append(t)
    return tuple(out)


@functools.lru_cache(maxsize=65536)
def _kt_default_match(a, b):
    i = 0
    while True:
        if i == len(a) or i == len(b):
            return i == len(a) and i == len(b)
        if WORD.fullmatch(a[i]) and (i == 0 or a[i - 1] != '.'):
            end = kt_default_chain(a, i)
            if end is not None:
                args, after = kt_default_split(a, end + 1)
                if args is not None and len(args) >= 2 and args[-1] == ['null'] and len(args[-2]) == 1 \
                        and re.fullmatch(r'-?\d+', args[-2][0]):
                    for taken in _kt_default_rewrites(a[i:end], a[end], args[:-2], int(args[-2][0]), b[i:]):
                        if _kt_default_match(a[after:], b[i + taken:]):
                            return True
        if a[i] != b[i]:
            return False
        i += 1


def _kt_default_rewrites(qualifier, name, values, mask, b):
    """For each way `b` may start with the call `qualifier.name(values, mask, null)` resugared, how many
    tokens of `b` it takes."""
    suffix = '$default'
    forms = []
    if name.endswith(suffix) and len(name) > len(suffix):
        base = name[:-len(suffix)]
        forms.append((tuple(qualifier) + (base,), values, None))
        if values:
            forms.append(((base,), values[1:], values[0]))
    else:
        forms.append((tuple(qualifier) + (name,), values, None))
    for head, rest, instance in forms:
        start = 0
        if instance is not None:
            k = next((k for k in range(len(b) - len(head) - 1)
                      if b[k] == '.' and tuple(b[k + 1:k + 1 + len(head)]) == head and b[k + 1 + len(head)] == '('),
                     None)
            if k is None:
                continue
            inst = b[:k]
            if inst[:1] == ('(',) and inst[-1:] == (')',):
                inst = inst[1:-1]
            if not _kt_default_match(tuple(instance), tuple(inst)):
                continue
            start = k + 1
        if tuple(b[start:start + len(head)]) != head or start + len(head) >= len(b) or b[start + len(head)] != '(':
            continue
        new_args, after = kt_default_split(b, start + len(head))
        if new_args is None:
            continue
        stripped = [x[2:] if len(x) > 2 and WORD.fullmatch(x[0]) and x[1] == '=' else x for x in new_args]
        for offset in (0, 1, 2):
            dropped = {offset + bit for bit in range(32) if mask >> bit & 1}
            if not dropped or max(dropped) >= len(rest) or any(
                    not KT_DEFAULT_LITERAL.fullmatch(''.join(rest[d])) for d in dropped):
                continue
            kept = [x for d, x in enumerate(rest) if d not in dropped]
            if len(kept) == len(stripped) and all(
                    _kt_default_match(tuple(o), tuple(n)) for o, n in zip(kept, stripped)):
                yield after
                break


def kt_default_args_call(old, new):
    """kt_default_args_call: token for token, calls of `f$default(.., mask, null)` became `f(..)` (a member's
    `C.f$default(x, ..)` became `x.f(..)`) and constructor calls lost their `mask, null`, the arguments the
    mask defaults (constants) left out and the others kept in order, possibly by name."""
    a, b = tokens(old), tokens(new)
    return a != b and kt_default_args_match(a, b)


def kt_default_casts_removed(toks):
    """The tokens without casts: `as T` / `as? T` (a Kotlin type, function types included), Java's `(a.b.T)`
    before an operand, and explicit type arguments `.<T>m(`."""
    out, i = [], 0
    while i < len(toks):
        t = toks[i]
        if t == 'as' and out:
            j = i + 1
            if j < len(toks) and toks[j] == '?':
                j += 1
            j = kt_default_type_end(toks, j)
            if j is not None:
                i = j
                continue
        if t == '(' and kt_default_grouping(out):
            j = kt_default_type_end(toks, i + 1, java=True)
            if j is not None and j < len(toks) - 1 and toks[j] == ')' and (
                    WORD.fullmatch(toks[j + 1]) or toks[j + 1] in '(`') and '.' in toks[i + 1:j]:
                i = j + 1
                continue
        if t == '<' and out and out[-1] == '.':
            depth, j = 0, i
            while j < len(toks):
                depth += {'<': 1, '>': -1}.get(toks[j], 0)
                j += 1
                if depth == 0:
                    break
            i = j
            continue
        out.append(t)
        i += 1
    return out


def kt_default_type_end(toks, j, java=False):
    """The index after the type starting at toks[j], else None."""
    if j >= len(toks):
        return None
    if toks[j] == '(' and not java:
        depth, k = 0, j
        while k < len(toks):
            depth += {'(': 1, ')': -1}.get(toks[k], 0)
            k += 1
            if depth == 0:
                break
        if k + 1 < len(toks) and toks[k] == '-' and toks[k + 1] == '>':
            k = kt_default_type_end(toks, k + 2)
            if k is not None and k < len(toks) and toks[k] == '?':
                k += 1
            return k
        return None
    if not WORD.fullmatch(toks[j]):
        return None
    k = j + 1
    while k + 1 < len(toks) and toks[k] == '.' and WORD.fullmatch(toks[k + 1]):
        k += 2
    if k < len(toks) and toks[k] == '<':
        depth = 0
        while k < len(toks):
            depth += {'<': 1, '>': -1}.get(toks[k], 0)
            k += 1
            if depth == 0:
                break
    if k < len(toks) and toks[k] == '?' and not java:
        k += 1
    return k


KT_DEFAULT_KEYWORDS = {'return', 'throw', 'else', 'in', 'is', 'if', 'while', 'when', 'for', 'and', 'or'}


def kt_default_grouping(before):
    """Whether a `(` after the tokens `before` opens a group (or a cast), not an argument list."""
    if before[-2:] == ['*', '/']:
        return False
    return not before or before[-1] in KT_DEFAULT_KEYWORDS or not (WORD.fullmatch(before[-1])
                                                                  or before[-1] in ')]')


def kt_default_ungrouped(toks):
    """The tokens without grouping parentheses (those not after a name, `)` or `]`)."""
    out, stack = [], []
    for t in toks:
        if t == '(':
            grouping = kt_default_grouping(out)
            stack.append(grouping)
            if not grouping:
                out.append(t)
        elif t == ')' and stack:
            if not stack.pop():
                out.append(t)
        else:
            out.append(t)
    return out


def kt_default_args_call_casts(old, new):
    """kt_default_args_call, with what the call's new descriptor changes around it: the casts the writer
    adds to or drops from its arguments and its value (and the parentheses they need), and explicit type
    arguments; the call itself resugared as kt_default_args_call recognises."""
    a, b = tokens(old), tokens(new)
    if a == b or kt_default_args_match(a, b):
        return False
    a2 = kt_default_ungrouped(kt_default_casts_removed(a))
    b2 = kt_default_ungrouped(kt_default_casts_removed(b))
    return a2 != b2 and kt_default_args_match(a2, b2)


KT_GENERATED_CTOR = re.compile(r'(?:@\w+(?:\([^)]*\))?\n)*(?:(?:public|private|protected|internal) )?fun ([A-Z][\w$]*)\([^\n]*\) \{\n'
                               r'this\([^\n]*\)\n\}\n')


KT_GENERATED_CTOR_SHIFTED = re.compile(r'\}\n(?:@\w+(?:\([^)]*\))?\n)*(?:(?:public|private|protected|internal) )?'
                                       r'fun ([A-Z][\w$]*)\([^\n]*\) \{\nthis\([^\n]*\)\n(?=\}\n|$)')


def kt_generated_constructor_hidden(old, new):
    """kt_generated_constructor_hidden: whole constructors printed as functions named after the class whose
    body is only `this(..)` (with the annotations above them) were removed, nothing else changed (a class
    body left empty loses its braces)."""
    kept = kt_generated_constructors_removed(old)
    if kept is None:
        return False
    stripped = [x.strip() for x in new if x.strip()]
    if len(kept) >= 2 and kept[-1] == '}' and kept[-2].endswith(' {'):
        if stripped == kept[:-2] + [kept[-2][:-2]]:
            return True
    return kept == stripped


def kt_generated_constructors_removed(old):
    """The stripped non-blank lines of `old` without its generated constructors, None if it has none."""
    text = '\n'.join(x.strip() for x in old if x.strip()) + '\n'
    kept, n = KT_GENERATED_CTOR.subn('', text)
    if not n:
        kept, n = KT_GENERATED_CTOR_SHIFTED.subn('', text)
    return kept.split('\n')[:-1] if n else None


def kt_default_args_both(old, new):
    """kt_generated_constructor_hidden and kt_default_args_call in one hunk: generated constructors removed
    and, in the lines left, calls through default-arguments bridges resugared."""
    kept = kt_generated_constructors_removed(old)
    return kept is not None and kt_default_args_call(kept, [x.strip() for x in new if x.strip()])


def kt_default_args_call_scope(a, b, i1, i2, j1, j2, failed):
    """kt_default_args_call (or its casts) with up to 12 lines of the unchanged context around the hunk:
    a call whose first or last lines did not change."""
    for k1 in range(13):
        if k1 and (i1 - k1 < 0 or j1 - k1 < 0 or a[i1 - k1] != b[j1 - k1]):
            break
        for k2 in range(13):
            if k2 and (i2 + k2 > len(a) or j2 + k2 > len(b) or a[i2 + k2 - 1] != b[j2 + k2 - 1]):
                break
            if not k1 and not k2:
                continue
            old, new = a[i1 - k1:i2 + k2], b[j1 - k1:j2 + k2]
            if kt_default_args_call(old, new) or kt_default_args_call_casts(old, new):
                return True
    return False


CONTEXT_RECOGNISERS = {'enum_body_separator': enum_body_separator,
                       'assert_dead_throw': assert_dead_throw_scope,
                       'kt_failed_method_hidden': kt_failed_method_hidden,
                       'kt_return_outside_lambda': kt_return_outside_lambda_scope,
                       'kt_semaphore_store_liveness': kt_semaphore_store_liveness_scope,
                       'kt_default_args_call': kt_default_args_call_scope}
CONTEXT_RECOGNISERS.update((fix, ktprint_context_recogniser(fix)) for fix in KTPRINT_ORDER)


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
    'kt_semaphore_store_liveness': kt_semaphore_store_liveness,
    'lambda_catch_var_rename': lambda_catch_var_rename,
    'nested_param_rename': nested_param_rename,
    'type_annotation_qualified_name': type_annotation_qualified_name,
    'type_annotation_argument_scope': type_annotation_argument_scope,
    'assert_throw_after_if': assert_throw_after_if,
    'sealed_private_permits': sealed_private_permits,
    'switch_expression_assignments': switch_expression_assignments,
    'unchecked_type_pattern': unchecked_type_pattern,
    'boxed_type_variable_cast': boxed_type_variable_cast,
    'lambda_argument_overload': lambda_argument_overload,
    'raw_receiver_erasure': raw_receiver_erasure,
    'unchecked_generic_cast': unchecked_generic_cast,
    'static_type_variable_scope': static_type_variable_scope,
    'inner_method_argument_shift': inner_method_argument_shift,
    'kt_default_args_call': kt_default_args_call,
    'kt_default_args_call_casts': kt_default_args_call_casts,
    'kt_generated_constructor_hidden': kt_generated_constructor_hidden,
    'kt_default_args_both': kt_default_args_both,
}
RECOGNISERS.update((fix, ktprint_recogniser(fix)) for fix in KTPRINT_ORDER[:-1])
RECOGNISERS['kt_secondary_constructor_line'] = kt_secondary_constructor_line
RECOGNISERS['loop_body_exit_explicit'] = loop_body_exit_explicit
RECOGNISERS['type_variable_shadowing'] = cast_only
RECOGNISERS['compound_shift_count_type'] = compound_shift_count_type
RECOGNISERS['definition_const_type'] = definition_const_type
RECOGNISERS['catch_tail_inline'] = catch_tail_inline
RECOGNISERS['diamond_argument_conflict'] = diamond_argument_conflict

# A table key that is not one fix's id names the fixes its recogniser stands for.
FIX_GROUPS = {
    'kt_default_args_call_casts': ('kt_default_args_call',),
    'kt_default_args_both': ('kt_default_args_call', 'kt_generated_constructor_hidden'),
    'kt_callable_reference': ('kt_reference_anonymous_super_call', 'kt_property_reference'),
    'kt_for_loops': ('kt_for_loop_exit_condition', 'kt_for_range_counter', 'kt_for_range_exclusive_bound',
                     'kt_for_range_unmatched_bound', 'kt_for_loop_definitions', 'kt_for_loop_continue_update'),
    'ktprint4_file': ('kt_constructor_default_signature', 'kt_abstract_class_constructor',
                      'kt_inline_class_constructor_accessor', 'kt_constructor_body_after_first_block',
                      'kt_jump_to_non_loop_label'),
}


def without_type_arguments(toks, qualifiers):
    """The tokens without the explicit type arguments of method calls (`.<T>m(` to `.m(`), and, with
    `qualifiers`, without the `C.` or `this.` written before them (`C.<T>m(` to `m(`)."""
    out, i, found = [], 0, False
    while i < len(toks):
        if toks[i] == '<' and out and out[-1] == '.':
            depth, j = 0, i
            while j < len(toks):
                depth += {'<': 1, '>': -1}.get(toks[j], 0)
                j += 1
                if depth == 0:
                    break
            if j + 1 < len(toks) and IDENT.match(toks[j]) and toks[j + 1] == '(':
                found = True
                if qualifiers and len(out) >= 2 and IDENT.match(out[-2]) and (len(out) < 3 or out[-3] != '.'):
                    out = out[:-2]
                i = j
                continue
        out.append(toks[i])
        i += 1
    return out if found else None


def lambda_pertinence(old, new):
    """lambda_pertinence: calls got explicit type arguments, `x.<T>m(` (or `C.<T>m(` / `this.<T>m(`
    for an unqualified `m(`), the lines possibly reflowed."""
    a, b = tokens(old), tokens(new)
    return a != b and a in (without_type_arguments(b, False), without_type_arguments(b, True))


def lambda_type_variable_target(old, new):
    """lambda_type_variable_target: a lambda argument of a call that is itself an argument became
    `(Interface)(lambda)`, the lines possibly reflowed."""
    return lambda_argument_overload(old, new)


RECOGNISERS['lambda_pertinence'] = lambda_pertinence


def unchecked_call_result_cast(old, new):
    """unchecked_call_result_cast: casts added (none dropped) to returned or assigned values, nothing
    else changed (the operand a call, which unchecked_generic_cast's recogniser leaves alone)."""
    if not cast_only(old, new):
        return False
    a, b = tokens(old), tokens(new)
    casts = [i for i in range(len(b)) if cast_at(b, i) is not None]
    if len(casts) <= sum(cast_at(a, i) is not None for i in range(len(a))):
        return False
    old_casts = [a[j:cast_at(a, j) + 1] for j in range(len(a)) if cast_at(a, j) is not None]
    return all(i > 0 and b[i - 1] in ('return', '=') for i in casts if b[i:cast_at(b, i) + 1] not in old_casts)


RECOGNISERS['unchecked_call_result_cast'] = unchecked_call_result_cast
RECOGNISERS['lambda_type_variable_target'] = lambda_type_variable_target


KT_OPERAND_KEYWORDS = ('return', 'throw', 'else', 'in', 'is', 'when')


def kt_as_operand_unparen(toks):
    """The tokens with the parentheses around the operand of each Kotlin `X as T` dropped, where they only
    group it (not a call's), so `(a + b) as T` reads like the operand of Java's `(T)(a + b)`."""
    toks = list(toks)
    for i in range(len(toks) - 1, 0, -1):
        if toks[i] != 'as' or toks[i - 1] != ')':
            continue
        depth, k = 0, i - 1
        while k >= 0:
            depth += {')': 1, '(': -1}.get(toks[k], 0)
            if depth == 0:
                break
            k -= 1
        if k < 0 or (k > 0 and ((IDENT.match(toks[k - 1]) and toks[k - 1] not in KT_OPERAND_KEYWORDS)
                                or toks[k - 1] in (')', ']', '>', '?'))):
            continue
        del toks[i - 1]
        del toks[k]
    return toks


def kt_as_casts_dropped(toks):
    """The tokens without each Kotlin cast `as T` (`as T?`) and the parentheses that only grouped its
    operand."""
    toks = kt_as_operand_unparen(toks)
    out, i = [], 0
    while i < len(toks):
        if toks[i] == 'as' and out:
            j = type_end(toks, i + 1)
            if j is None and i + 1 < len(toks) and toks[i + 1] == '(':
                m = block_end(toks, i + 1)
                if m is not None and toks[m + 1:m + 3] == ['-', '>']:
                    j = type_end(toks, m + 3)
            if j is not None:
                i = j
                continue
        out.append(toks[i])
        i += 1
    return out


def kt_code_tokens(text_lines):
    """The tokens of some Kotlin lines, string templates split like code (`"${f(x)}"`), backticks dropped."""
    return [t for t in tokens([x.replace('"', ' ') for x in text_lines]) if t != '`']


def kt_java_reference_cast(old, new):
    """kt_java_reference_cast: Java's casts `(T)X` and instance casts `((T)X).m` became Kotlin's `X as T`
    (`as T?`, `as (A) -> R`) and `(X as T).m`, or were dropped, token for token otherwise (casts aside);
    the new text has fewer Java casts."""
    a, b = kt_code_tokens(old), kt_code_tokens(new)
    if a != b and java_casts(b) < java_casts(a) and \
            strip_casts(kt_as_casts_dropped(a)) == strip_casts(kt_as_casts_dropped(b)):
        return True
    return kt_java_reference_cast_mixed(old, new)


def java_casts(toks):
    return sum(1 for i in range(len(toks)) if cast_at(toks, i) is not None)


def kotlin_type_args_dropped(toks):
    """The tokens without the type arguments written after a called method's name (`.m<T>(` to `.m(`)."""
    out, i, found = [], 0, False
    while i < len(toks):
        if toks[i] == '<' and len(out) >= 2 and IDENT.match(out[-1]) and out[-2] == '.':
            depth, j = 0, i
            while j < len(toks):
                depth += {'<': 1, '>': -1}.get(toks[j], 0)
                j += 1
                if depth == 0:
                    break
            if j < len(toks) and toks[j] == '(':
                found = True
                i = j
                continue
        out.append(toks[i])
        i += 1
    return out if found else None


def kt_explicit_type_arguments(old, new):
    """kt_explicit_type_arguments: Java's explicit type arguments `.<T>m(` moved after the name, `.m<T>(`
    (or were dropped), the lines otherwise the same."""
    a, b = kt_code_tokens(old), kt_code_tokens(new)
    plain = explicit_type_args_dropped(a)
    if a == b or plain == a:
        return False
    return plain in (b, kotlin_type_args_dropped(b))


RECOGNISERS['kt_java_reference_cast'] = kt_java_reference_cast


def kt_java_reference_cast_tail(a, b, i1, i2, j1, j2, throw_names):
    """kt_java_reference_cast: the last line of a cast's operand that spans lines got the Kotlin cast
    `as T` (`}` -> `} as T`), where the operand's first line, above, has Java's cast `(T)X`."""
    if i2 - i1 != 1 or j2 - j1 != 1 or not b[j1].startswith(a[i1] + ' as '):
        return False
    rest = kt_code_tokens([b[j1][len(a[i1]) + 1:]])
    if not rest or rest[0] != 'as' or type_end(rest, 1) != len(rest):
        return False
    for k in range(i1 - 1, max(-1, i1 - 400), -1):
        toks = kt_code_tokens([a[k]])
        for c in (c for c in range(len(toks)) if cast_at(toks, c) is not None):
            opened = toks[cast_at(toks, c) + 1:]
            if sum(t in '({' for t in opened) > sum(t in ')}' for t in opened):
                return True
    return False


CONTEXT_RECOGNISERS['kt_java_reference_cast'] = kt_java_reference_cast_tail


KT_JAVA_REF_CAST = re.compile(r'(?<![\w$>\]?])\((?:[\w$.]+(?:<[^()]*?>)?(?:\[\])+|(?:[\w$]+\.)*[A-Z][\w$]*(?:<[^()]*?>)?)\)'
                              r'(?=[\w$`("\'])')


KT_TYPE_ARGS = re.compile(r'\.[\w$]+<[^()]*>\(')


def kt_balanced_end(text, i, open_, close):
    """The index past the bracket closing the one at text[i]."""
    depth = 0
    for j in range(i, len(text)):
        depth += (text[j] == open_) - (text[j] == close)
        if depth == 0:
            return j + 1
    return None


def kt_type_text_end(text, i):
    """The index past a Kotlin type written from text[i] (a dotted name, type arguments, `?`, or a
    function type `(A) -> R`), or None."""
    if i < len(text) and text[i] == '(':
        j = kt_balanced_end(text, i, '(', ')')
        if j is None or not text.startswith(' -> ', j):
            return None
        return kt_type_text_end(text, j + 4)
    j = i
    while j < len(text) and (text[j].isalnum() or text[j] in '_$.'):
        j += 1
    if j == i:
        return None
    if j < len(text) and text[j] == '<':
        j = kt_balanced_end(text, j, '<', '>')
        if j is None:
            return None
    return j + 1 if j < len(text) and text[j] == '?' else j


def kt_neutral_line(line):
    """`line` without reference casts in either language and without explicit type arguments in either
    place (`.<T>m(`, `.m<T>(`): what kt_java_reference_cast and kt_explicit_type_arguments change."""
    line = KT_JAVA_REF_CAST.sub('', line)
    out, i = [], 0
    while i < len(line):
        if line.startswith(' as ', i):
            j = kt_type_text_end(line, i + 4)
            if j is not None:
                i = j
                continue
        if line.startswith('.<', i):
            j = kt_balanced_end(line, i + 1, '<', '>')
            if j is not None:
                out.append('.')
                i = j
                continue
        if line[i] == '<' and out and (out[-1].isalnum() or out[-1] in '_$'):
            k = len(out)
            while k > 0 and (out[k - 1].isalnum() or out[k - 1] in '_$'):
                k -= 1
            j = kt_balanced_end(line, i, '<', '>')
            if k > 0 and out[k - 1] == '.' and j is not None and line.startswith('(', j):
                i = j
                continue
        out.append(line[i])
        i += 1
    return ''.join(out)


def kt_java_reference_cast_mixed(old, new):
    """kt_java_reference_cast or kt_explicit_type_arguments in a hunk another fix changes too: with the
    casts and explicit type arguments of both texts dropped, the texts are the same or another fix's
    recogniser accepts them."""
    def java(lines):
        text = '\n'.join(lines)
        return len(KT_JAVA_REF_CAST.findall(text)) + text.count('.<')

    def kotlin(lines):
        return '\n'.join(lines).count(' as ')

    def kotlin_type_args(lines):
        return len(KT_TYPE_ARGS.findall('\n'.join(lines)))

    dropped = java(old) - java(new) + kotlin_type_args(new) - kotlin_type_args(old)
    if dropped <= 0 or kotlin(new) - kotlin(old) > dropped:
        return False
    a, b = [kt_neutral_line(x) for x in old], [kt_neutral_line(x) for x in new]
    if tokens(a) == tokens(b):
        return True
    return any(r(a, b) for fix, r in RECOGNISERS.items()
               if fix not in ('kt_java_reference_cast', 'kt_explicit_type_arguments'))

RECOGNISERS['kt_explicit_type_arguments'] = kt_explicit_type_arguments


# --- ktprint4 fixes ---

KT_VISIBILITY = ('public', 'protected', 'private', 'internal')
KT_HEADER_END = {'{', '}', 'fun', 'val', 'var', 'class', 'object', 'interface', 'init', 'constructor', '@', 'by',
                 'public', 'protected', 'private', 'internal', 'open', 'abstract', 'override', 'final'}


def kt_paren_end(toks, i):
    """The index after the balanced `( .. )` that starts at toks[i], None if it does not close."""
    depth = 0
    for j in range(i, len(toks)):
        depth += {'(': 1, ')': -1}.get(toks[j], 0)
        if depth == 0:
            return j + 1
    return None


def kt_headers_canon(toks, drop_called):
    """The tokens with each class header's primary constructor (`(..)`, `vis constructor(..)`) dropped and
    its supertype calls `S(..)` written `S` (or, with `drop_called`, dropped with their comma)."""
    out, i = [], 0
    while i < len(toks):
        out.append(toks[i])
        i += 1
        if out[-1] != 'class' or i >= len(toks):
            continue
        out.append(toks[i])
        i += 1
        if i < len(toks) and toks[i] == '<':
            depth = 0
            while i < len(toks):
                depth += {'<': 1, '>': -1}.get(toks[i], 0)
                out.append(toks[i])
                i += 1
                if depth == 0:
                    break
        j = i + 1 if i < len(toks) and toks[i] in KT_VISIBILITY else i
        j = j + 1 if j < len(toks) and toks[j] == 'constructor' else j
        if j < len(toks) and toks[j] == '(':
            i = kt_paren_end(toks, j) or len(toks)
        if i >= len(toks) or toks[i] != ':':
            continue
        entries, cur = [], []
        i += 1
        while i < len(toks) and toks[i] not in KT_HEADER_END:
            if toks[i] == '(':
                i = kt_paren_end(toks, i) or len(toks)
                cur.append('()')
                continue
            if toks[i] == ',':
                entries.append(cur)
                cur = []
            else:
                cur.append(toks[i])
            i += 1
        entries.append(cur)
        kept = [kt_type_args_dropped([t for t in e if t != '()']) for e in entries if not (drop_called and '()' in e)]
        if kept:
            out.append(':')
            for k, e in enumerate(kept):
                out += ([','] if k else []) + e
    return out


def kt_type_args_dropped(toks):
    """The tokens without `<..>` groups (a supertype's type arguments, which the header's call leaves out)."""
    out, depth = [], 0
    for t in toks:
        depth += t == '<'
        if not depth:
            out.append(t)
        depth -= t == '>' and depth > 0
    return out


KT_CTOR_FUN = re.compile(r'^(open )?fun ([A-Z][\w$]*)\((.*)\) \{$')
KT_SECONDARY_CTOR = re.compile(r'^(?:(?:public|protected|private|internal) )?constructor\(.*?\) : (this\(.*)$')


def kt_ctor_funs_undone(lines, open_fun):
    """The stripped non-blank lines with each constructor written as a function named after its class
    (`open fun` when `open_fun`) in the form it has as a constructor: an empty one dropped, one that only
    calls `this(..)` written `constructor : this(..)`, any other `init {` without a leading `super(..)`
    (moved to the class header); and how many there were."""
    s = [x.strip() for x in lines if x.strip()]
    out, i, n = [], 0, 0
    while i < len(s):
        m = KT_CTOR_FUN.match(s[i])
        if not m or bool(m.group(1)) != open_fun:
            out.append(KT_SECONDARY_CTOR.sub(r'constructor : \1', s[i]))
            i += 1
            continue
        n += 1
        if i + 1 < len(s) and s[i + 1] == '}':
            i += 2
        elif i + 2 < len(s) and s[i + 1].startswith('this(') and s[i + 2] == '}':
            out.append('constructor : ' + s[i + 1])
            i += 3
        else:
            out.append('init {')
            i += 1
            if i < len(s) and s[i].startswith('super('):
                depth = 0
                while i < len(s):
                    depth += s[i].count('(') - s[i].count(')')
                    i += 1
                    if depth <= 0:
                        break
                if i < len(s) and s[i] == '}':
                    out.pop()
                    i += 1
    return out, n


def kt_ctor_funs_written(old, new, open_fun, drop_called):
    """Constructors written as functions in `old` (see kt_ctor_funs_undone) are constructors in `new`,
    the class headers aside (kt_headers_canon), and nothing else changed."""
    plain, n = kt_ctor_funs_undone(old, open_fun)
    if not n and not drop_called and not re.search(r'\babstract class ', '\n'.join(old)):
        return False
    canon_new = [KT_SECONDARY_CTOR.sub(r'constructor : \1', x.strip()) for x in new if x.strip()]
    a = kt_headers_canon(kt_code_tokens(plain), drop_called)
    b = kt_headers_canon(kt_code_tokens(canon_new), drop_called)
    return a == b and (n > 0 or kt_code_tokens(old) != kt_code_tokens(new))


def kt_abstract_class_constructor(old, new):
    """kt_abstract_class_constructor: an abstract class's constructors, written as `open fun C(..)`, are
    its primary constructor in the class header (with the supertype call) and `init`, or `constructor(..)`
    lines; nothing else changed."""
    return kt_ctor_funs_written(old, new, True, False)


def kt_self_supertype_call(old):
    """Whether a class header in `old` names its own class as the supertype it calls (`class C(..) : C(..)`)."""
    toks = kt_code_tokens(old)
    for i in range(len(toks) - 2):
        if toks[i] == 'class':
            name = toks[i + 1]
            j = toks.index(':', i) if ':' in toks[i:] else -1
            while 0 <= j < len(toks) - 1 and toks[j + 1] not in KT_HEADER_END:
                if toks[j] == name and toks[j + 1] == '(':
                    return True
                j += 1
    return False


def kt_inline_class_constructor_accessor(old, new):
    """kt_inline_class_constructor_accessor: a class whose header called itself as its supertype (the
    accessor's `this(..)`) and whose constructor was written as `fun C(..)`: the header calls the real
    supertype or none, the function is the primary constructor's `init` (or gone); nothing else changed."""
    if not kt_self_supertype_call(old) and not kt_ctor_funs_undone(old, False)[1]:
        return False
    return kt_ctor_funs_written(old, new, False, True)


def kt_constructor_default_signature(old, new):
    """kt_constructor_default_signature: an empty constructor written as `fun C() {}` (not `open`) is gone,
    nothing else changed."""
    s = [x.strip() for x in old if x.strip()]
    plain = [x.strip() for x in new if x.strip()]
    for i in range(len(s) - 1):
        if re.match(r'^fun [A-Z][\w$]*\(\) \{$', s[i]) and s[i + 1] == '}' and s[:i] + s[i + 2:] == plain:
            return True
    return False


def kt_brace_block_end(toks, i):
    """The index after the balanced `{ .. }` that starts at toks[i], None if it does not close."""
    depth = 0
    for j in range(i, len(toks)):
        depth += {'{': 1, '}': -1}.get(toks[j], 0)
        if depth == 0:
            return j + 1
    return None


def kt_constructor_body_after_first_block(old, new):
    """kt_constructor_body_after_first_block: only whole bodies added: an `init { .. }` block, or a body
    `{ .. }` after a constructor's `this(..)` or a class header's supertype call; nothing else changed."""
    a, b = kt_code_tokens(old), kt_code_tokens(new)
    i = j = added = 0
    while i < len(a) or j < len(b):
        if i < len(a) and j < len(b) and a[i] == b[j]:
            i += 1
            j += 1
            continue
        if j >= len(b):
            return False
        k = j + 1 if b[j] == 'init' else j
        if k < len(b) and b[k] == '{' and (k > j or (j and b[j - 1] == ')')):
            end = kt_brace_block_end(b, k)
            if end is None or end - k <= 2:
                return False
            j = end
            added += 1
            continue
        return False
    return added > 0


def kt_jump_tokens_undone(a, b, free_closes):
    """Whether `b` is `a` with kt_jump_to_non_loop_label's spellings: `labelN@` written `run labelN@{` (its
    extra `}` later, or up to `free_closes` closing a block opened before), `break@labelN` or
    `continue@labelN` written `return@labelN`; and how many such changes."""
    i = j = changed = 0
    opened = 0
    while i < len(a) or j < len(b):
        if i < len(a) and j < len(b) and a[i] == b[j]:
            i += 1
            j += 1
            continue
        if j + 3 < len(b) and b[j] == 'run' and b[j + 2:j + 4] == ['@', '{'] and b[j + 1].startswith('label') \
                and a[i:i + 2] == [b[j + 1], '@']:
            i, j, opened, changed = i + 2, j + 4, opened + 1, changed + 1
            continue
        if b[j:j + 2] == ['return', '@'] and i < len(a) and a[i] in ('break', 'continue') \
                and a[i + 1:i + 3] == b[j + 1:j + 3] and b[j + 2].startswith('label'):
            i, j, changed = i + 3, j + 3, changed + 1
            continue
        if j < len(b) and b[j] == '}' and (opened or free_closes):
            if opened:
                opened -= 1
            else:
                free_closes -= 1
                changed += 1
            j += 1
            continue
        return False, 0
    return True, changed


def kt_jump_to_non_loop_label(old, new):
    """kt_jump_to_non_loop_label: a labelled `if` wrapped in `run labelN@{ .. }` (re-indented), jumps to
    such a label written `return@labelN`; nothing else changed."""
    ok, changed = kt_jump_tokens_undone(kt_code_tokens(old), kt_code_tokens(new), 0)
    return ok and changed > 0


KTPRINT4_FIXES = ('kt_constructor_default_signature', 'kt_abstract_class_constructor',
                  'kt_inline_class_constructor_accessor', 'kt_constructor_body_after_first_block',
                  'kt_jump_to_non_loop_label')
KT_JUMP = re.compile(r'\b(?:break|continue|return)@(label\d+)\b')
_KTPRINT4_FILES = {}


def kt_enclosing_class(lines, k):
    """The name of the class whose header encloses lines[k] (the nearest less indented `class` line)."""
    ind = indent_of(lines[k])
    for m in range(k - 1, -1, -1):
        if lines[m].strip() and indent_of(lines[m]) < ind:
            found = re.search(r'\bclass ([\w$]+)', lines[m])
            if found:
                return found.group(1)
            ind = indent_of(lines[m])
    return None


def kt_header_joined(lines, k):
    """A class header that starts at lines[k] as one line (its continuation lines joined), and the index
    after it."""
    text, e = lines[k].rstrip(), k + 1
    while e < len(lines) and (text.count('(') > text.count(')') or text.endswith((',', ':'))):
        text += ' ' + lines[e].strip()
        e += 1
    return text, e


def kt_ktprint4_canon(lines, other, old):
    """The lines with the ktprint4 fixes' changes written alike on both sides: the headers of abstract
    classes and of classes that call themselves as their supertype as their kt_headers_canon tokens, the
    secondary constructors in them `constructor : this(..)`, jumps to labels the new text leaves with
    `return@labelN` and the old with `break@`/`continue@` as `jump@labelN`; in the old text (`old`) constructors written as functions (in a class of their name) in their
    constructor form (kt_ctor_funs_undone); in the new text each `run labelN@{` around an `if` whose label
    the old text (`other`) writes bare and never as `run` back as `labelN@` and the `if`, one level out.
    Also, per input line, the indices of the lines it became, and the input lines this changed."""
    out, where, touched, k = [], [[] for _ in lines], set(), 0
    other_text = '\n'.join(other)
    unwrapped, closing = {}, set()
    a, b = (lines, other) if old else (other, lines)
    a_text, b_text = '\n'.join(a), '\n'.join(b)
    classes = {m.group(1) for m in re.finditer(r'\babstract class ([\w$]+)(?:<[^\n]*?>)? [:{]', a_text)} | \
        {x for x in re.findall(r'\bclass ([\w$]+)\b', a_text) if kt_self_supertype_call(
            [y for y in a if re.search(r'\bclass ' + re.escape(x) + r'\b', y)])}
    labels = {x for x in re.findall(r'\breturn@(label\d+)\b', b_text)
              if re.search(r'\b(?:break|continue)@' + x + r'\b', a_text)}

    def emit(src, text):
        where[src].append(len(out))
        if text != lines[src]:
            touched.add(src)
        out.append(text)

    while k < len(lines):
        line = lines[k]
        m = KT_CTOR_FUN.match(line.strip())
        if old and m and kt_enclosing_class(lines, k) == m.group(2):
            ind = indent_of(line)
            end = next((e for e in range(k + 1, len(lines)) if lines[e] == ' ' * ind + '}'), None)
            if end is not None:
                touched.update(range(k, end + 1))
                plain, _ = kt_ctor_funs_undone(lines[k:end + 1], bool(m.group(1)))
                if plain[:1] == ['init {']:
                    emit(k, ' ' * ind + 'init {')
                    e, depth = k + 1, 0
                    while e < end and (lines[e].strip().startswith('super(') or depth > 0):
                        depth += lines[e].count('(') - lines[e].count(')')
                        e += 1
                    for x in range(e, end + 1):
                        emit(x, lines[x])
                else:
                    for x in plain:
                        emit(k, ' ' * ind + x)
                k = end + 1
                continue
        m = re.match(r'^(\s*)run (label\d+)@\{$', line)
        if not old and m and k + 1 < len(lines) and lines[k + 1].strip().startswith('if (') \
                and re.search(r'^\s*' + m.group(2) + '@$', other_text, re.M) \
                and not re.search(r'\brun ' + m.group(2) + r'@\{', other_text):
            end = next((e for e in range(k + 1, len(lines)) if lines[e] == m.group(1) + '}'), None)
            if end is not None:
                emit(k, m.group(1) + m.group(2) + '@')
                touched.update((k, end))
                closing.add(end)
                for x in range(k + 1, end):
                    unwrapped[x] = unwrapped.get(x, 0) + 3
                k += 1
                continue
        if k in closing:
            k += 1
            continue
        header = re.search(r'\bclass ([\w$]+)', line)
        if header and header.group(1) in classes and not line.strip().startswith(('//', '*', '@')):
            text, e = kt_header_joined(lines, k)
            emit(k, ' ' * indent_of(line) + ' '.join(kt_headers_canon(kt_code_tokens([text]), False)))
            touched.update(range(k, e))
            k = e
            continue
        stripped = line.strip()
        if KT_SECONDARY_CTOR.match(stripped) and kt_enclosing_class(lines, k) in classes:
            stripped = KT_SECONDARY_CTOR.sub(r'constructor : \1', stripped)
        stripped = KT_JUMP.sub(lambda j: 'jump@' + j.group(1) if j.group(1) in labels else j.group(0), stripped)
        emit(k, ' ' * max(0, indent_of(line) - unwrapped.get(k, 0)) + stripped if stripped else line)
        k += 1
    return out, where, touched


def kt_ktprint4_file(a, b, i1, i2, j1, j2, throw_names):
    """The ktprint4 fixes in a file other fixes change too, where line matching pairs their hunks with other
    changes: with kt_ktprint4_canon applied to both texts (which changed some line of the file), each line
    of the hunk became a line that is the same in both texts or in a difference a recogniser accepts
    (another fix's, or kt_constructor_body_after_first_block's added bodies)."""
    key = (id(a), id(b), len(a), len(b))
    if key not in _KTPRINT4_FILES:
        ca, wa, ta = kt_ktprint4_canon(a, b, True)
        cb, wb, tb = kt_ktprint4_canon(b, a, False)
        ok_a, ok_b = set(), set()
        for tag, x1, x2, y1, y2 in difflib.SequenceMatcher(None, ca, cb, autojunk=False).get_opcodes():
            if tag == 'equal' or classify(ca, cb, x1, x2, y1, y2, set()) != 'other' or \
                    any(r(ca, cb, x1, x2, y1, y2, set()) for fix, r in CONTEXT_RECOGNISERS.items()
                        if fix != 'ktprint4_file'):
                ok_a.update(range(x1, x2))
                ok_b.update(range(y1, y2))
        _KTPRINT4_FILES.clear()
        _KTPRINT4_FILES[key] = (wa, ta, ok_a), (wb, tb, ok_b)
    (wa, ta, ok_a), (wb, tb, ok_b) = _KTPRINT4_FILES[key]
    if not ta and not tb:
        return False
    return all(all(c in ok_a for c in wa[x]) for x in range(i1, i2)) and \
        all(all(c in ok_b for c in wb[y]) for y in range(j1, j2))


RECOGNISERS['kt_constructor_default_signature'] = kt_constructor_default_signature
RECOGNISERS['kt_abstract_class_constructor'] = kt_abstract_class_constructor
RECOGNISERS['kt_inline_class_constructor_accessor'] = kt_inline_class_constructor_accessor
RECOGNISERS['kt_constructor_body_after_first_block'] = kt_constructor_body_after_first_block
RECOGNISERS['kt_jump_to_non_loop_label'] = kt_jump_to_non_loop_label
CONTEXT_RECOGNISERS['ktprint4_file'] = kt_ktprint4_file


KT_CONTINUATION_OBJECT = re.compile(r'\bobject : (?:Restricted)?ContinuationImpl\(')


def kt_failure_comment_lines(lines):
    """The indices of the lines of every failure comment (the marker and the comment lines after it)."""
    out, inside = set(), False
    for i, line in enumerate(lines):
        s = line.strip()
        if s == MARKER:
            inside = True
        elif not s.startswith('//'):
            inside = False
        if inside:
            out.add(i)
    return out


def kt_continuation_functions(lines):
    """The (opening line, closing line) of every function or lambda of `lines` that holds a continuation
    object: the nearest enclosing line with `fun` or ending in `->`, and the line of the brace that closes
    it; None when one is not whole in `lines`."""
    def indent(i):
        return len(lines[i]) - len(lines[i].lstrip(' '))

    def opener(i):
        j = i - 1
        while j >= 0 and not (lines[j].strip() and indent(j) < indent(i)
                              and lines[j].rstrip().endswith(('{', '->'))):
            j -= 1
        return j

    def braces(i):
        toks = [] if lines[i].lstrip().startswith('//') else tokens([lines[i]])
        return toks.count('{') - toks.count('}')

    spans = set()
    for k, line in enumerate(lines):
        if not KT_CONTINUATION_OBJECT.search(line):
            continue
        h = opener(k)
        while h >= 0 and not (re.search(r'(?<![\w$`])fun ', lines[h]) or lines[h].rstrip().endswith('->')):
            h = opener(h)
        if h < 0:
            return None
        depth, e = braces(h), h + 1
        while e < len(lines):
            depth += braces(e)
            if depth <= 0:
                break
            e += 1
        if e == len(lines):
            return None
        spans.add((h, e))
    return sorted(spans)


def kt_without_return_type(line):
    """A function header line without its return type (a function that failed may be written with another)."""
    return re.sub(r'\)\s*:\s*[^(){}]*\{\s*$', ') {', line)


def kt_continuation_residue_ok(old, new):
    """The difference of two texts with kt_synthetic_object_super_call's changes set aside: a reflow, or
    differences other fixes' recognisers accept."""
    if tokens(old) == tokens(new):
        return True
    others = [r for fix, r in RECOGNISERS.items() if fix != 'kt_synthetic_object_super_call']
    for tag, i1, i2, j1, j2 in difflib.SequenceMatcher(None, old, new, autojunk=False).get_opcodes():
        o, n = old[i1:i2], new[j1:j2]
        if tag != 'equal' and not (tokens(o) == tokens(n) or all(is_import(x) for x in o + n)
                                   or any(r(o, n) for r in others)):
            return False
    return True


def kt_synthetic_object_super_call(old, new):
    """kt_synthetic_object_super_call: failure comments of functions and lambdas replaced by their bodies,
    which write a suspend function's continuation as `object : ContinuationImpl(..)` (or, where `old` has no
    failure comment, such functions inserted whole: members compat mode hides because they failed); with
    those set aside (and the return types of their headers) the rest is the same tokens, reflowed, or
    changes other fixes' recognisers accept."""
    blocks, spans = kt_failure_comment_lines(old), kt_continuation_functions(new)
    if not spans:
        return False
    body = {i for h, e in spans for i in range(h + (1 if blocks else 0), e + (0 if blocks else 1))}
    headers = {h for h, _ in spans}
    rest_old = [kt_without_return_type(x) if i + 1 in blocks and i not in blocks else x
                for i, x in enumerate(old) if i not in blocks]
    rest_new = [kt_without_return_type(x) if i in headers else x for i, x in enumerate(new) if i not in body]
    return kt_continuation_residue_ok(rest_old, rest_new)


_KT_CONTINUATION_FILES = {}
_KT_CONTINUATION_BUSY = []


def kt_continuation_header(line):
    """What names a function or lambda in its opening line: from its last `fun` (return type aside), or its
    lambda's `{ params ->`."""
    toks = tokens([kt_without_return_type(line)])
    if 'fun' in toks:
        return tuple(toks[len(toks) - 1 - toks[::-1].index('fun'):])
    return tuple(toks[len(toks) - 1 - toks[::-1].index('{'):]) if '{' in toks else tuple(toks)


def kt_synthetic_object_super_call_file(a, b, i1, i2, j1, j2, throw_names):
    """kt_synthetic_object_super_call where line matching pairs its new bodies with other changes: with
    each failure comment of `a` and the body that replaces it in `b` set aside (a function of `b` whose
    header no failure comment of `a` follows, whole: a member compat mode hides because it failed), each
    line of the hunk is the same in both texts or in a difference that is a reflow or that a recogniser
    accepts."""
    key = (id(a), id(b), len(a), len(b))
    if _KT_CONTINUATION_BUSY:
        return False
    if key not in _KT_CONTINUATION_FILES:
        _KT_CONTINUATION_BUSY.append(key)
        spans = kt_continuation_functions(b) or []
        blocks = kt_failure_comment_lines(a)
        starts = [i for i in sorted(blocks) if i - 1 not in blocks and i > 0]
        failed = [(kt_continuation_header(a[i - 1]), i) for i in starts]
        drop_a, drop_b, headers_a, headers_b = set(), set(), set(), set()
        for h, e in spans:
            name = kt_continuation_header(b[h])
            match = next((f for f in failed if f[0] == name), None)
            if match:
                failed.remove(match)
                t = match[1]
                while t in blocks:
                    drop_a.add(t)
                    t += 1
                headers_a.add(match[1] - 1)
                headers_b.add(h)
                drop_b.update(range(h + 1, e))
            else:
                drop_b.update(range(h - (1 if h and not b[h - 1].strip() else 0), e + 1))
        ra = [i for i in range(len(a)) if i not in drop_a]
        rb = [i for i in range(len(b)) if i not in drop_b]
        ta = [kt_without_return_type(a[i]) if i in headers_a else a[i] for i in ra]
        tb = [kt_without_return_type(b[i]) if i in headers_b else b[i] for i in rb]
        ok_a, ok_b = set(drop_a), set(drop_b)
        for tag, x1, x2, y1, y2 in difflib.SequenceMatcher(None, ta, tb, autojunk=False).get_opcodes():
            if tag == 'equal' or classify(ta, tb, x1, x2, y1, y2, set()) != 'other' or \
                    any(r(ta, tb, x1, x2, y1, y2, set()) for fix, r in CONTEXT_RECOGNISERS.items()
                        if fix != 'kt_synthetic_object_super_call'):
                ok_a.update(ra[x] for x in range(x1, x2))
                ok_b.update(rb[y] for y in range(y1, y2))
        _KT_CONTINUATION_BUSY.clear()
        _KT_CONTINUATION_FILES.clear()
        _KT_CONTINUATION_FILES[key] = bool(spans), ok_a, ok_b
    found, ok_a, ok_b = _KT_CONTINUATION_FILES[key]
    return found and all(x in ok_a for x in range(i1, i2)) and all(y in ok_b for y in range(j1, j2))


RECOGNISERS['kt_synthetic_object_super_call'] = kt_synthetic_object_super_call
CONTEXT_RECOGNISERS['kt_synthetic_object_super_call'] = kt_synthetic_object_super_call_file


def classify(a, b, i1, i2, j1, j2, marked):
    kind = classify_base(a, b, i1, i2, j1, j2, marked)
    if kind != 'other':
        return kind
    old, new = a[i1:i2], b[j1:j2]
    plain = ktprint_canon(old, new)
    if plain != new:
        for fix, recognises in RECOGNISERS.items():
            if fix not in KTPRINT_ORDER and recognises(old, plain):
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
    if not old and new and all(is_import(x) for x in new):
        return 'import'
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


def self_check():
    """The problems kuna_fix_check.py finds in this file (two functions of one name, a table entry with no
    recogniser or naming no KunaFix id)."""
    here = os.path.dirname(os.path.abspath(__file__))
    spec = importlib.util.spec_from_file_location('kuna_fix_check', os.path.join(here, 'kuna_fix_check.py'))
    check = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(check)
    with open(os.path.abspath(__file__), encoding='utf-8') as f:
        return check.problems(f.read(), check.fix_ids())


def main():
    if sys.argv[1:] in (['-h'], ['--help']):
        print(__doc__)
        return
    if len(sys.argv) != 3:
        sys.exit(__doc__)
    broken = self_check()
    if broken:
        sys.exit('kuna_fix_diff.py fails its self-check:\n  ' + '\n  '.join(broken))
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
