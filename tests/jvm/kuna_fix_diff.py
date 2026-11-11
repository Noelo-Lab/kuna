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
import ast
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
    'ktprint6_file': ('kt_legacy_constructor_parameter_checks', 'kt_hidden_empty_super_call'),
    'kt_unboxed_cast': ('kt_unboxing_cast_type', 'kt_operator_precedence'),
    'twr_resugar': ('twr_unresolved_resource', 'twr_class_version', 'twr_desugaring_shape'),
    'source_locals_diamond': ('jvm_source_locals', 'external_class_diamond'),
    'source_locals_kt_narrow': ('jvm_source_locals', 'kt_narrow_bitwise_operand'),
    'ktprint7_file': ('kt_default_accessor_flags', 'kt_unescaped_name', 'kt_reserved_underscore_name',
                      'kt_java_type_syntax'),
    'ktvalid_file': ('kt_accessor_backing_field', 'kt_string_literal_escape', 'kt_abstract_function_body',
                     'kt_misplaced_modality', 'kt_companion_property_initializer', 'kt_default_property_initializer',
                     'kt_missed_override', 'kt_outer_this', 'kt_extension_receiver', 'kt_nested_annotation_argument',
                     'kt_throws_class_name', 'kt_throwable_cast', 'kt_mapped_java_member', 'kt_mapped_java_type',
                     'kt_object_instance', 'kt_super_property_access', 'kt_setter_parameter_name',
                     'kt_java_class_literal'),
    'instanceof_cast': ('pattern_binding_shape', 'jvm_source_patterns'),
    'pattern_subject_local': ('jvm_source_locals',),
    'instanceof_cast_subject_local': ('pattern_binding_shape', 'jvm_source_patterns', 'jvm_source_locals'),
    'instanceof_forms': ('pattern_binding_shape', 'jvm_source_patterns', 'jvm_source_locals', 'jvm_javac_layout'),
    'ktvalid2': ('local_final_writes', 'kt_not_null_assertion', 'kt_expression_value_check',
                 'kt_static_initializer', 'kt_object_instance_local', 'kt_property_initializer_key',
                 'kt_synchronized_accessor'),
    'ktfacade_calls': ('kt_facade_call', 'kt_extension_call', 'kt_jvm_name_call', 'kt_synthetic_accessor_call'),
    'ktnullable': ('kt_nullable_local', 'kt_nullable_cast', 'kt_field_read_reuse', 'kt_partial_inline'),
    'kt_extension_call_type_argument': ('kt_extension_call',),
    'source_scopes': ('jvm_source_locals',),
    'source_increments': ('jvm_source_locals',),
    'compound_increment': ('compound_assignment_shape',),
    'switch_filler_labels': ('switch_table_fillers',),
    'ktdefaults_composed': ('kt_default_args_bridge_lookup', 'kt_default_value_names', 'kt_default_value_coercion_cast',
                            'kt_delegated_member_default'),
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

KT_FILE_CTOR = re.compile(r'^open fun [\w$]+\(\) \{$')


def kt_file_class_constructor(old, new):
    """kt_file_class_constructor: empty top-level `open fun C() {}` functions (a file class's constructor)
    are gone; nothing else changed."""
    if any(x.strip() for x in new):
        return False
    s = [x for x in old if x.strip()]
    return bool(s) and len(s) % 2 == 0 and all(KT_FILE_CTOR.match(s[k]) and s[k + 1] == '}'
                                               for k in range(0, len(s), 2))


def kt_when_break_undone(a, b):
    """Whether tokens `b` are tokens `a` with kt_when_break's changes: an unlabelled `break` (or Java's
    `break;`) that ends a block left out, or written `return@labelN` inside a `run labelN@{ .. }` added
    around the `when` (its `}` later); and how many changes."""
    i = j = changed = opened = 0
    labels = set()
    while i < len(a) or j < len(b):
        if i < len(a) and j < len(b) and a[i] == b[j]:
            i, j = i + 1, j + 1
            continue
        if b[j:j + 1] == ['run'] and j + 3 < len(b) and b[j + 1].startswith('label') and b[j + 2:j + 4] == ['@', '{']:
            labels.add(b[j + 1])
            i, j, opened, changed = i, j + 4, opened + 1, changed + 1
            continue
        if a[i:i + 1] == ['break']:
            k = i + 2 if a[i + 1:i + 2] == [';'] else i + 1
            if a[k:k + 1] in (['}'], []):
                if b[j:j + 2] == ['return', '@'] and b[j + 2:j + 3] and b[j + 2] in labels:
                    i, j, changed = k, j + 3, changed + 1
                    continue
                i, changed = k, changed + 1
                continue
        if b[j:j + 1] == ['}'] and opened:
            j, opened = j + 1, opened - 1
            continue
        return False, 0
    return opened == 0, changed


def kt_when_break(old, new):
    """kt_when_break: `break`s out of a `when` left out where they end a block, or written `return@labelN`
    with the `when` in an added `run labelN@{ .. }` (re-indented); nothing else changed."""
    ok, changed = kt_when_break_undone(kt_code_tokens(old), kt_code_tokens(new))
    return ok and changed > 0


def kt_java_statement_jump(old, new):
    """kt_java_statement_jump: jumps in Java's syntax (`break;`, `continue label5;`) written in Kotlin's
    (`break`, `continue@label5`, `return@label5` out of a `run label5@{`); nothing else changed."""
    a, b = kt_code_tokens(old), kt_code_tokens(new)
    i = j = changed = 0
    while i < len(a) or j < len(b):
        if i < len(a) and j < len(b) and a[i] == b[j] and a[i] not in ('break', 'continue'):
            i, j = i + 1, j + 1
            continue
        if i < len(a) and a[i] in ('break', 'continue') and j < len(b):
            label = a[i + 1] if i + 1 < len(a) and a[i + 1].startswith('label') else None
            k = i + (2 if label else 1)
            if a[k:k + 1] != [';']:
                if a[i:i + 1] == b[j:j + 1]:
                    i, j = i + 1, j + 1
                    continue
                return False
            if label is None and b[j] == a[i] and b[j + 1:j + 2] != ['@']:
                i, j, changed = k + 1, j + 1, changed + 1
                continue
            if label and b[j] in (a[i], 'return') and b[j + 1:j + 3] == ['@', label]:
                i, j, changed = k + 1, j + 3, changed + 1
                continue
        return False
    return changed > 0


KT_JUMP_LINE = re.compile(r'^(?:(?:break|continue)(?:@label\d+)?|return@label\d+)$')


def kt_catch_block_jump(old, new):
    """kt_catch_block_jump: a jump line (`continue`, `break`, `return@labelN`) added, nothing else."""
    s = [x.strip() for x in new if x.strip()]
    return not any(x.strip() for x in old) and len(s) == 1 and bool(KT_JUMP_LINE.match(s[0]))


RECOGNISERS['kt_file_class_constructor'] = kt_file_class_constructor
RECOGNISERS['kt_when_break'] = kt_when_break
RECOGNISERS['kt_java_statement_jump'] = kt_java_statement_jump
RECOGNISERS['kt_catch_block_jump'] = kt_catch_block_jump


KT_SUPER_TYPE_ARGS = re.compile(r'((?:^|\s):\s[\w.$]+)<(?:[^()<>]|<(?:[^()<>]|<[^()<>]*>)*>)*>\(')


def kt_supertype_type_arguments_added(a, b):
    """Whether tokens `b` are tokens `a` with type arguments `<..>` added between names and their `(`."""
    i = j = changed = 0
    while i < len(a) or j < len(b):
        if i < len(a) and j < len(b) and a[i] == b[j]:
            i, j = i + 1, j + 1
            continue
        if b[j:j + 1] == ['<'] and a[i:i + 1] == ['('] and j > 0 and b[j - 1] not in ('<', ',', ':'):
            depth, k = 0, j
            while k < len(b):
                depth += {'<': 1, '>': -1}.get(b[k], 0)
                if depth == 0:
                    break
                k += 1
            if k + 1 < len(b) and b[k + 1] == '(' and '(' not in b[j:k]:
                j, changed = k + 1, changed + 1
                continue
        return False
    return changed > 0


def kt_supertype_type_arguments(old, new):
    """kt_supertype_type_arguments: type arguments `<..>` added between a class header's supertype name and
    its call's `(`, the lines possibly reflowed; or, in a header another fix changes too, with them dropped
    again (`: S<..>(` back to `: S(`) the texts are the same or another fix's recogniser accepts them."""
    a, b = kt_code_tokens(old), kt_code_tokens(new)
    if ':' in a and kt_supertype_type_arguments_added(a, b):
        return True
    plain = [KT_SUPER_TYPE_ARGS.sub(r'\1(', x) for x in new]
    if plain == new:
        return False
    return tokens(old) == tokens(plain) or any(r(old, plain) for fix, r in RECOGNISERS.items()
                                               if fix != 'kt_supertype_type_arguments')


RECOGNISERS['kt_supertype_type_arguments'] = kt_supertype_type_arguments


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
    if _KT_CONTINUATION_BUSY or (key not in _KT_CONTINUATION_FILES
                                 and not any(KT_CONTINUATION_OBJECT.search(x) for x in b)):
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


ANON_CAPTURE = '\x00captured'


def block_ends(toks):
    """`block_end(toks, i)` for every i, in one pass: the first later token where the running bracket depth is
    back at the depth before toks[i] (None where there is none, or toks[i] opens no bracket)."""
    depths, depth = [], 0
    for t in toks:
        depth += 1 if t in ('(', '{', '[') else -1 if t in (')', '}', ']') else 0
        depths.append(depth)
    ends, nearest = [None] * len(toks), {}
    for i in range(len(toks) - 1, -1, -1):
        if toks[i] in ('(', '{', '['):
            ends[i] = nearest.get(depths[i] - 1)
        nearest[depths[i]] = i
    return ends


def anon_capture_reads(toks, old):
    """The tokens with each read of an object's capture as `ANON_CAPTURE`: in `old` (Vineflower's mapping) an
    enclosing instance `X.this`/`X.super` (cast `(X.this as T)`/`((T)X.this)`, read field `X.super.$f`) or the
    object's own field `this.$f`; in the new text a field of the creating instance, `R.this.$f$inlined`."""
    out, i = [], 0
    ends = block_ends(toks)
    while i < len(toks):
        t = toks[i]
        if old and t == '(' and toks[i + 2:i + 5] in (['.', 'this', 'as'], ['.', 'super', 'as']) \
                and IDENT.match(toks[i + 1]):
            j = ends[i]
            if j is not None:
                out.append(ANON_CAPTURE)
                i = j + 1
                continue
        if old and t == '(' and toks[i + 1:i + 2] == ['(']:
            j, k = ends[i], ends[i + 1]
            if j is not None and k is not None and k + 4 == j and IDENT.match(toks[k + 1]) \
                    and toks[k + 2:k + 4] in (['.', 'this'], ['.', 'super']):
                out.append(ANON_CAPTURE)
                i = j + 1
                continue
        if IDENT.match(t) and t[:1].isupper() and toks[i + 1:i + 2] == ['.'] \
                and toks[i + 2:i + 3] in ([['this'], ['super']] if old else [['this']]):
            j = i + 3
            if toks[j:j + 1] == ['.'] and toks[j + 1:j + 2] and toks[j + 1].startswith('$'):
                j += 2
            elif not old:
                out.append(t)
                i += 1
                continue
            out.append(ANON_CAPTURE)
            i = j
            continue
        if old and t == 'this' and (not out or out[-1] != '.') and toks[i + 1:i + 2] == ['.'] \
                and toks[i + 2:i + 3] and toks[i + 2].startswith('$'):
            out.append(ANON_CAPTURE)
            i += 3
            continue
        out.append(t)
        i += 1
    return out


def anonymous_class_created_elsewhere(old, new):
    """anonymous_class_created_elsewhere: an object kotlinc's `$$inlined$` class creates reads its captures at
    the creating method: where Vineflower read the declaring class's `X.this`/`X.super` or the object's own
    field `this.$f`, the new text reads a variable or the creating instance's `R.this.$f$inlined`; the
    captured variables are `val` (were `var`), and calls of the declaring class's private members keep their
    `access$` accessor (the object is no longer nested in that class). Nothing else changed."""
    flat = re.sub(r'\s+', '', '\n'.join(old))
    if '.this' not in flat and '.super' not in flat and 'this.$' not in flat:
        return False
    a, b = anon_capture_reads(tokens(old), True), anon_capture_reads(tokens(new), False)
    if old == new or ANON_CAPTURE not in a or len(a) != len(b):
        return False
    for tag, a1, a2, b1, b2 in difflib.SequenceMatcher(None, a, b, autojunk=False).get_opcodes():
        x, y = a[a1:a2], b[b1:b2]
        if tag == 'equal' or (x == ['var'] and y == ['val']):
            continue
        if len(x) == len(y) and all(o == ANON_CAPTURE and (n == ANON_CAPTURE or IDENT.match(n)) or
                                    n == 'access$' + o for o, n in zip(x, y)):
            continue
        return False
    return True


RECOGNISERS['anonymous_class_created_elsewhere'] = anonymous_class_created_elsewhere


_KT_DELEGATING = []


def kt_other_recogniser_accepts(old, plain, me):
    """Whether a recogniser other than `me` accepts the hunk with `me`'s change undone (`plain`); one level
    deep: these recognisers do not delegate again from here."""
    if _KT_DELEGATING:
        return False
    _KT_DELEGATING.append(me)
    try:
        return any(r(old, plain) for fix, r in RECOGNISERS.items() if fix != me)
    finally:
        _KT_DELEGATING.pop()


KT_LEGACY_CHECK = re.compile(r'^Intrinsics\.checkParameterIsNotNull\([\w$`]+, "(?:[^"\\]|\\.)*"\)$')
KT_CTOR_HEAD = r'((?:(?:public|protected|private|internal) )?constructor\(.*\))'
KT_CHECKED_CTOR = re.compile(r'^' + KT_CTOR_HEAD + r' : Intrinsics\.checkParameterIsNotNull\([\w$`]+, "(?:[^"\\]|\\.)*"\) \{$')
KT_CALL_CTOR = re.compile(r'^' + KT_CTOR_HEAD + r' : (?!this\(|super\(|Intrinsics\.checkParameterIsNotNull\()(.+?\))( \{)?$')
KT_SPACED_CTOR = re.compile(r'^' + KT_CTOR_HEAD + r'  \{$')
KT_BARE_CTOR = re.compile(r'^' + KT_CTOR_HEAD + r'( \{)?$')
KT_CLASS_HEADER = re.compile(r'^\s*(?:[@\w$]+(?:\([^)]*\))? +)*(?:class|object)\b')


def kt_statement_end(lines, k):
    """The index after the statement that starts at lines[k] (its parentheses balanced), None if they do not."""
    depth = 0
    for e in range(k, len(lines)):
        depth += lines[e].count('(') - lines[e].count(')')
        if depth <= 0:
            return e + 1
    return None


def kt_delegation(lines, k, names):
    """The `super(..)`/`this(..)` call (one of `names`) that starts at lines[k]: its name, its argument text and
    the index after it; None if lines[k] starts none."""
    m = re.match(r'^\s*(super|this)\(', lines[k]) if k < len(lines) else None
    end = kt_statement_end(lines, k) if m and m.group(1) in names else None
    if end is None:
        return None
    text = ' '.join(x.strip() for x in lines[k:end])
    return (m.group(1), text[len(m.group(1)) + 1:-1], end) if text.endswith(')') else None


def kt_ktprint6_undo(lines, headers=None):
    """Old text rewritten as fixes kt_legacy_constructor_parameter_checks and kt_hidden_empty_super_call write it:
    an `init` block's leading parameter checks (`Intrinsics.checkParameterIsNotNull(p, "p")`) and the `super(..)`
    after them dropped (the block too when nothing is left; with `headers`, the call's arguments recorded for
    the class header the block belongs to, `headers[index of its first line] = args`; without, checks and the
    call at the start of the lines are taken for such a block's); a secondary constructor `constructor(..) :
    Intrinsics.checkParameterIsNotNull(..) {` with its checks and the delegation call after them written
    `constructor(..) : super(args)` / `: this(args)` (no call for an empty `super()`), without braces when
    nothing is left; and a secondary constructor that takes a call for its delegation (`constructor(..) :
    f(..)`) or has none (`constructor(..)  {`) written with the call as its first statement. Returns the lines,
    the output indices per input line, and the fixes whose rules fired."""
    out, where, fired, k = [], [[] for _ in lines], set(), 0

    def emit(src, text):
        where[src].append(len(out))
        out.append(text)

    def skip(start, end, to):
        for x in range(start, end):
            where[x].append(to)

    while k < len(lines):
        line, stripped = lines[k], lines[k].strip()
        ind = indent_of(line)
        bare = headers is None and k == 0 and KT_LEGACY_CHECK.match(stripped)
        if stripped == 'init {' or bare:
            e = k if bare else k + 1
            while e < len(lines) and KT_LEGACY_CHECK.match(lines[e].strip()):
                e += 1
            call = kt_delegation(lines, e, ('super',)) if e > k + 1 or (bare and e > k) else None
            if call:
                fired.add('kt_legacy_constructor_parameter_checks')
                end = call[2]
                if bare:
                    skip(k, end, len(out))
                    k = end
                    continue
                if headers is not None:
                    header = next((h for h in range(k - 1, -1, -1) if KT_CLASS_HEADER.match(lines[h])
                                   and indent_of(lines[h]) < ind), None)
                    if header is not None:
                        headers[header] = call[1]
                if end < len(lines) and lines[end] == ' ' * ind + '}':
                    end += 1
                    if end < len(lines) and not lines[end].strip():
                        end += 1
                    skip(k, end, len(out))
                    k = end
                    continue
                emit(k, line)
                skip(k + 1, end, len(out) - 1)
                k = end
                continue
        m = KT_CHECKED_CTOR.match(stripped)
        if m:
            e = k + 1
            while e < len(lines) and KT_LEGACY_CHECK.match(lines[e].strip()):
                e += 1
            call = kt_delegation(lines, e, ('super', 'this'))
            if call:
                fired.add('kt_legacy_constructor_parameter_checks')
                head = m.group(1)
                if call[0] == 'this' or call[1]:
                    head += f' : {call[0]}({call[1]})'
                else:
                    fired.add('kt_hidden_empty_super_call')
                end = call[2]
                if end < len(lines) and lines[end] == ' ' * ind + '}':
                    emit(k, ' ' * ind + head)
                    skip(k + 1, end + 1, len(out) - 1)
                    k = end + 1
                    continue
                emit(k, ' ' * ind + head + ' {')
                skip(k + 1, end, len(out) - 1)
                k = end
                continue
        m = KT_SPACED_CTOR.match(stripped)
        if m:
            fired.add('kt_hidden_empty_super_call')
            emit(k, ' ' * ind + m.group(1) + ' {')
            k += 1
            continue
        m = KT_CALL_CTOR.match(stripped)
        if m:
            fired.add('kt_hidden_empty_super_call')
            emit(k, ' ' * ind + m.group(1) + ' {')
            emit(k, ' ' * (ind + 3) + m.group(2))
            if not m.group(3):
                emit(k, ' ' * ind + '}')
            k += 1
            continue
        emit(k, line)
        k += 1
    return out, where, fired


def kt_first_supertype_span(toks):
    """The token span (start, end) of a class header's first supertype, its call included: after the `:` that
    follows the class name, type parameters and primary constructor (without `class` or `object` in the
    tokens, after the first `) :`); None if there is none."""
    start = None
    for i, t in enumerate(toks):
        if t in ('class', 'object') and i + 1 < len(toks):
            j = i + 1 if toks[i + 1] in ('{', ':') else i + 2
            if j < len(toks) and toks[j] == '<':
                depth = 0
                while j < len(toks):
                    depth += {'<': 1, '>': -1}.get(toks[j], 0)
                    j += 1
                    if depth == 0:
                        break
            while j < len(toks) and (toks[j] in KT_VISIBILITY or toks[j] in ('constructor', '@') or toks[j - 1] == '@'):
                j += 1
            if j < len(toks) and toks[j] == '(':
                j = kt_paren_end(toks, j) or len(toks)
            start = j
            break
    if start is None:
        start = next((i + 1 for i in range(len(toks) - 1) if toks[i] == ')' and toks[i + 1] == ':'), None)
    if start is None or start >= len(toks) or toks[start] != ':':
        return None
    e = start + 1
    while e < len(toks) and toks[e] not in (',', '(', '{') and toks[e] not in KT_HEADER_END:
        if toks[e] == '<':
            depth = 0
            while e < len(toks):
                depth += {'<': 1, '>': -1}.get(toks[e], 0)
                e += 1
                if depth == 0:
                    break
            continue
        e += 1
    if e < len(toks) and toks[e] == '(':
        e = kt_paren_end(toks, e) or len(toks)
    return start + 1, e


def kt_supertype_call_canon(toks, args=None):
    """The tokens with the first supertype of a class header written without type arguments (given `args`, a
    supertype without a call gets that one), and the call's argument tokens (None for no call)."""
    span = kt_first_supertype_span(toks)
    if not span:
        return toks, None
    s, e = span
    sup, call = toks[s:e], None
    if '(' in sup:
        p = sup.index('(')
        call, sup = sup[p + 1:-1], sup[:p]
    elif args is not None:
        call = args
    sup = kt_type_args_dropped(sup)
    return toks[:s] + sup + ([] if call is None else ['('] + call + [')']) + toks[e:], call


def kt_legacy_constructor_parameter_checks(old, new):
    """kt_legacy_constructor_parameter_checks: Kotlin 1.3's parameter checks before a constructor's delegation
    call dropped: an `init` block's leading checks with the `super(..)` after them (the block too when empty),
    or a secondary constructor's checks with the call after them written as its delegation (kt_ktprint6_undo),
    the same tokens otherwise; a class header's call is kt_ktprint6_file's (it checks the arguments)."""
    plain, _, fired = kt_ktprint6_undo(old)
    return 'kt_legacy_constructor_parameter_checks' in fired and kt_code_tokens(plain) == kt_code_tokens(new)


KT_EMPTY_SUPER_CALL = re.compile(r'((?:^|\s|\)) :\s[\w.$`]+(?:<(?:[^()<>]|<(?:[^()<>]|<[^()<>]*>)*>)*>)?)\(\)')


def kt_hidden_empty_super_call(old, new):
    """kt_hidden_empty_super_call: a class header's superclass gained an empty call `()` (with the metadata's
    type arguments: kt_supertype_type_arguments), the lines possibly reflowed, or another fix's recogniser
    accepts the hunk once the call is taken out again; a secondary constructor that took
    a call for its delegation, or had none (`)  {`), written with the call as its first statement; or a
    secondary constructor added that Vineflower left out (an empty one, or one whose statements a sibling's
    differing field initialiser keeps)."""
    a, b = kt_code_tokens(old), kt_code_tokens(new)
    ca, call_a = kt_supertype_call_canon(a, [])
    cb, call_b = kt_supertype_call_canon(b)
    if a != b and ca == cb and call_a == [] and call_b == [] and '(' not in a[slice(*kt_first_supertype_span(a))]:
        return True
    plain = [KT_EMPTY_SUPER_CALL.sub(r'\1', x, count=1) for x in new]
    if plain != new and kt_supertype_call_canon(kt_code_tokens(plain))[1] is None and \
            kt_other_recogniser_accepts(old, plain, 'kt_hidden_empty_super_call'):
        return True
    plain, _, fired = kt_ktprint6_undo(old)
    if fired == {'kt_hidden_empty_super_call'} and kt_code_tokens(plain) == b:
        return True
    body = [x for x in new if x.strip()]
    if any(x.strip() for x in old) or not body:
        return False
    m = KT_BARE_CTOR.match(body[0].strip())
    return bool(m) and ((not m.group(2) and len(body) == 1) or
                        (bool(m.group(2)) and body[-1] == ' ' * indent_of(body[0]) + '}'))


KT_INIT_PROPERTY = re.compile(r'^((?:[\w$]+ )*(?:val|var) ([\w$`]+): .+?) = (.+)$')


def kt_ktprint6_moved_initialiser(old, new, b):
    """A property initialiser that `InitializerProcessor` now leaves in the constructors, because their first
    statements agree on a super call no longer (kt_legacy_constructor_parameter_checks): `val x: T = v` became
    `val x: T`, and the new text (`b`) assigns `this.x = v`."""
    if len(old) != len(new) or old == new:
        return False
    text = '\n'.join(b)
    for o, n in zip(old, new):
        if o == n:
            continue
        m = KT_INIT_PROPERTY.match(o.strip())
        if not m or n.strip() != m.group(1) or f'this.{m.group(2).strip("`")} = {m.group(3)}' not in text:
            return False
    return True


KT_THIS_ASSIGNMENT = re.compile(r'^this\.([\w$]+) = (.+)$')


def kt_property_lines(lines, name):
    """The declaration lines of the properties named `name` in some lines, stripped."""
    pattern = re.compile(r'^(?:[\w$]+ )*(?:val|var) `?' + re.escape(name) + r'`?: ')
    return [x.strip() for x in lines if pattern.match(x.strip())]


def kt_ktprint6_moved_assignments(old, new, a, b):
    """Constructor assignments `this.x = v` that `InitializerProcessor` now keeps in the constructors or takes
    out as the property's initialiser, because the constructors' first statements now agree (or no longer do)
    on a super call (kt_legacy_constructor_parameter_checks): inserted where the old text (`a`) declares the
    property with initialiser `v` and the new text (`b`) without one, or removed where the new text declares
    it with an initialiser that starts with `v`."""
    old, new = [x for x in old if x.strip()], [x for x in new if x.strip()]
    if bool(old) == bool(new):
        return False
    for line in old or new:
        m = KT_THIS_ASSIGNMENT.match(line.strip())
        if not m:
            return False
        before, after = kt_property_lines(a, m.group(1)), kt_property_lines(b, m.group(1))
        if new and not (any(x.endswith(' = ' + m.group(2)) for x in before) and any(' = ' not in x for x in after)):
            return False
        if old and not any(' = ' + m.group(2).replace('<>(', '(') in x for x in after):
            return False
    return True


def kt_import_qualification(old, new, b):
    """Only qualified names `p.q.N` written `N`, each imported in the new text (`b`): the import collector's
    choice of a short name, which a changed header (which no longer imports a homonym first) leaves free."""
    imported = {x.strip()[len('import '):] for x in b if x.startswith('import ')}
    plain = [re.sub(r'\b(?:[a-z]\w*\.)+[A-Z][\w$]*(?:\.[A-Z][\w$]*)*\b',
                    lambda m: m.group(0).split('.')[-1] if m.group(0) in imported else m.group(0), x) for x in old]
    return plain != old and (tokens(plain) == tokens(new) or kt_other_recogniser_accepts(plain, new, None))


def kt_ktprint6_header_name(text):
    """The name of the class or object a header line declares."""
    m = re.search(r'\b(?:class|object) ([\w$`]+)', text)
    return m.group(1) if m else None


def kt_ktprint6_canon(lines, args=None):
    """The lines with every class header joined on one line and its first supertype written by
    kt_supertype_call_canon (`args`: the call arguments to give the header starting at an index, by index or
    by the class's name); the output index of each input line, and the inputs that got a call."""
    out, where, given_at, k = [], [], set(), 0
    while k < len(lines):
        line = lines[k]
        if KT_CLASS_HEADER.match(line) and not line.strip().startswith(('//', '*')):
            text, e = line.rstrip(), k + 1
            while e < len(lines) and (text.count('(') > text.count(')') or text.endswith((',', ':'))
                                      or lines[e].strip().startswith(':')):
                text += ' ' + lines[e].strip()
                e += 1
            given = None if args is None else args.get(k, args.get(kt_ktprint6_header_name(text)))
            toks = kt_code_tokens([text])
            canon, _ = kt_supertype_call_canon(toks, None if given is None else kt_code_tokens([given]))
            if given is not None and canon != kt_supertype_call_canon(toks)[0]:
                given_at.update(range(k, e))
            where += [len(out)] * (e - k)
            out.append(' ' * indent_of(line) + ' '.join(canon))
            k = e
            continue
        where.append(len(out))
        out.append(line)
        k += 1
    return out, where, given_at


_KTPRINT6_FILES = {}


def kt_ktprint6_file(a, b, i1, i2, j1, j2, throw_names):
    """The kt_legacy_constructor_parameter_checks and kt_hidden_empty_super_call changes in a file, where line
    matching splits or mixes them: with kt_ktprint6_undo applied to the old text (the arguments of each `init`
    block's `super(..)` it drops given to the block's class header) and both texts' class headers written by
    kt_ktprint6_canon, each line of the hunk became a line that is the same in both texts, or in a difference
    a recogniser accepts (kt_hidden_empty_super_call's added constructors among them), or a property
    initialiser moved into the constructors or out of them (kt_ktprint6_moved_initialiser,
    kt_ktprint6_moved_assignments), or a qualified name the import collector now writes short
    (kt_import_qualification)."""
    key = (id(a), id(b), len(a), len(b))
    if key not in _KTPRINT6_FILES:
        headers = {}
        ua, wa, fired = kt_ktprint6_undo(a, headers)
        args = {o: text for h, text in headers.items() for o in wa[h][:1]}
        empty = {kt_ktprint6_header_name(x): '' for x in b if KT_CLASS_HEADER.match(x) and
                 kt_supertype_call_canon(kt_code_tokens([x]))[1] == []}
        cb, wcb, _ = kt_ktprint6_canon(b)
        ca, wca, given_at = kt_ktprint6_canon(ua, {**empty, **args})
        touched = {x for x, w in enumerate(wa) if len(w) != 1 or ua[w[0]] != a[x] or w[0] in given_at}
        back = collections.defaultdict(list)
        for x, w in enumerate(wa):
            for o in w:
                back[wca[o]].append(x)
        ok_a, ok_b = set(), set()
        for tag, x1, x2, y1, y2 in difflib.SequenceMatcher(None, ca, cb, autojunk=False).get_opcodes():
            old, new = ca[x1:x2], cb[y1:y2]
            caused = ((kt_ktprint6_moved_initialiser(old, new, b) or kt_ktprint6_moved_assignments(old, new, a, b))
                      and 'kt_legacy_constructor_parameter_checks' in fired) \
                or (kt_import_qualification(old, new, b) and bool(given_at))
            if tag == 'equal' or caused or kt_code_tokens(old) == kt_code_tokens(new) or \
                    kt_hidden_empty_super_call(old, new) or classify(ca, cb, x1, x2, y1, y2, set()) not in ('other', 'layout'):
                ok_a.update(range(x1, x2))
                ok_b.update(range(y1, y2))
            if caused:
                touched.update(x for c in range(x1, x2) for x in back[c])
        _KTPRINT6_FILES.clear()
        _KTPRINT6_FILES[key] = ([[wca[o] for o in w] for w in wa], wcb, ok_a, ok_b, touched)
    wa, wcb, ok_a, ok_b, touched = _KTPRINT6_FILES[key]
    return any(x in touched for x in range(i1, i2)) and \
        all(all(c in ok_a for c in wa[x]) for x in range(i1, i2)) and all(wcb[y] in ok_b for y in range(j1, j2))


RECOGNISERS['kt_legacy_constructor_parameter_checks'] = kt_legacy_constructor_parameter_checks
RECOGNISERS['kt_hidden_empty_super_call'] = kt_hidden_empty_super_call
CONTEXT_RECOGNISERS['ktprint6_file'] = kt_ktprint6_file


KT_COMPOUND_BITWISE = re.compile(r'^(\s*)(.+?) (\|=|&=|\^=|<<=|>>=|>>>=) (.+)$')
KT_INFIX_NAMES = {'|=': 'or', '&=': 'and', '^=': 'xor', '<<=': 'shl', '>>=': 'shr', '>>>=': 'ushr'}


KT_INFIX_ASSIGNMENT = re.compile(r'^(\s*)(.+?) = (.+?) (or|and|xor|shl|shr|ushr) (.+)$')


def kt_compound_bitwise_undone(new, old_text):
    """The new lines with each `x = x or y` the old text (`old_text`) writes `x |= y` written that way again."""
    ops = {v: k for k, v in KT_INFIX_NAMES.items()}
    out = []
    for n in new:
        m = KT_INFIX_ASSIGNMENT.match(n)
        if m and m.group(2) == m.group(3):
            ind, left, _, name, right = m.groups()
            for r in ([right[1:-1]] if right.startswith('(') and right.endswith(')') else []) + [right]:
                if f'{left} {ops[name]} {r}' in old_text:
                    n = f'{ind}{left} {ops[name]} {r}'
                    break
        out.append(n)
    return out


def kt_compound_bitwise_assignment(old, new):
    """kt_compound_bitwise_assignment: line for line, `x |= y` (`&=`, `^=`, `<<=`, `>>=`, `>>>=`) became
    `x = x or y` (`and`, `xor`, `shl`, `shr`, `ushr`), `y` possibly in parentheses; other lines the same, or,
    with those lines written as before, the same tokens or another fix's recogniser accepts the hunk."""
    if len(old) == len(new) and old != new:
        for o, n in zip(old, new):
            if o == n:
                continue
            m = KT_COMPOUND_BITWISE.match(o)
            if not m:
                break
            ind, left, op, right = m.groups()
            name = KT_INFIX_NAMES[op]
            if n not in (f'{ind}{left} = {left} {name} {right}', f'{ind}{left} = {left} {name} ({right})'):
                break
        else:
            return True
    plain = kt_compound_bitwise_undone(new, '\n'.join(old))
    return plain != new and (tokens(old) == tokens(plain) or
                             kt_other_recogniser_accepts(old, plain, 'kt_compound_bitwise_assignment'))


RECOGNISERS['kt_compound_bitwise_assignment'] = kt_compound_bitwise_assignment


def kt_switch_head_indent(old, new):
    """kt_switch_head_indent: line for line, lines written in column 0 (the statements before a `when` but the
    first) indented; other lines the same, or with that undone another fix's recogniser accepts the hunk."""
    if len(old) == len(new) and old != new and all(
            o == n or (o.strip() and o == o.lstrip() and n.strip() == o and n != o) for o, n in zip(old, new)):
        return True
    plain = [o if len(old) == len(new) and o.strip() and o == o.lstrip() and n.strip() == o else n
             for o, n in zip(old, new)] if len(old) == len(new) else new
    return plain != new and kt_other_recogniser_accepts(old, plain, 'kt_switch_head_indent')


RECOGNISERS['kt_switch_head_indent'] = kt_switch_head_indent


NOT_A_READ_BEFORE = {'.', 'val', 'var', '>', ']'}
READ_KEYWORDS = {'return', 'throw', 'else', 'in', 'is', 'as', 'new', 'case'}


def captured_var_clash_name(old, new):
    """captured_var_clash_name: a captured variable read by the clashing name of the creating method's variable it
    holds (`first` -> `firstx`): the tokens are equal once trailing `x`s are stripped from identifiers, and every
    renamed token is a read (not a declared name, a parameter or a member after `.`). Nothing else changed."""
    if not x_renamed(old, new):
        return False
    b = tokens(new)
    for i, (o, n) in enumerate(zip(tokens(old), b)):
        if o == n:
            continue
        before = b[i - 1] if i else ''
        after = b[i + 1] if i + 1 < len(b) else ''
        if before in NOT_A_READ_BEFORE or after == ':' or (IDENT.match(before) and before not in READ_KEYWORDS):
            return False
    return True


RECOGNISERS['captured_var_clash_name'] = captured_var_clash_name

OWN_FIELD_INIT = re.compile(r'^\{(this\.(\$[\w$]+)=`\2`)+\}$')


def captured_var_through_cast(old, new):
    """captured_var_through_cast: an object's or lambda's read of its own field `this.$f` became a read of a
    variable, the one the creating `new` passed through a cast; or the Kotlin initializer block that stored the
    field, `{ this.$f = `$f` }`, is gone. Nothing else changed."""
    a, b = tokens(old), tokens(new)
    if not b:
        return bool(a) and OWN_FIELD_INIT.match(''.join(a)) is not None
    out, i = [], 0
    while i < len(a):
        if a[i] == 'this' and (not out or out[-1] != '.') and a[i + 1:i + 2] == ['.'] \
                and a[i + 2:i + 3] and a[i + 2].startswith('$'):
            out.append(ANON_CAPTURE)
            i += 3
            continue
        out.append(a[i])
        i += 1
    return ANON_CAPTURE in out and len(out) == len(b) and all(
        o == n or (o == ANON_CAPTURE and IDENT.match(n) and not n.startswith('$') and n != 'this')
        for o, n in zip(out, b))


RECOGNISERS['captured_var_through_cast'] = captured_var_through_cast

# --- ktops fixes: kt_operator_precedence, kt_narrow_bitwise_operand, kt_reference_inequality ---

KT_PREC_TOKEN = re.compile(r'`[^`\n]+`|"(?:[^"\\\n]|\\.)*"|\'(?:[^\'\\\n]|\\.)*\'|[\w$]+|\S')
KT_PREC_JOIN = sorted(('===', '!==', '?.', '?:', '!!', '::', '..', '->', '==', '!=', '<=', '>=', '&&', '||', '++',
                       '--', '+=', '-=', '*=', '/=', '%='), key=len, reverse=True)
KT_PREC_KOTLIN = {'*': 6, '/': 6, '%': 6, '+': 8, '-': 8, '..': 10, '?:': 14, 'in': 16, '!in': 16,
                  '<': 18, '>': 18, '<=': 18, '>=': 18, '==': 20, '!=': 20, '===': 20, '!==': 20,
                  '&&': 22, '||': 24}
KT_PREC_KOTLIN_TYPE = {'as': 4, 'as?': 4, 'is': 16, '!is': 16}
KT_PREC_JAVA = {'*': 4, '/': 4, '%': 4, '+': 6, '-': 6, '..': 7, 'shl': 8, 'shr': 8, 'ushr': 8,
                '<': 10, '>': 10, '<=': 10, '>=': 10, 'in': 12, '==': 12, '!=': 12, '===': 12, '!==': 12,
                'and': 14, 'xor': 16, 'or': 18, '&&': 20, '||': 22, '?:': 22}
KT_PREC_JAVA_TYPE = {'as': 3, 'as?': 3, 'is': 12, '!is': 3}
KT_PREC_KEYWORDS = {'val', 'var', 'fun', 'if', 'else', 'while', 'do', 'for', 'when', 'try', 'catch', 'finally',
                    'class', 'object', 'interface', 'package', 'import', 'is', 'in', 'as', 'public', 'private',
                    'internal', 'protected', 'open', 'override', 'abstract', 'final', 'const', 'lateinit',
                    'inline', 'suspend', 'companion', 'sealed', 'enum', 'constructor', 'operator', 'infix',
                    'tailrec', 'external', 'vararg', 'crossinline', 'noinline', 'reified', 'out', 'return',
                    'throw', 'break', 'continue', 'typealias'}
KT_PREC_PREFIX = {'-', '+', '!', '++', '--'}
KT_PREC_OPEN = {'(': ')', '[': ']', '{': '}'}
KT_PREC_NAME = re.compile(r'[\w$`]')


KT_PREC_CASTS = {'byte', 'short', 'char', 'int', 'long', 'float', 'double'}
KT_PREC_CONVERSIONS = {'toByte': 'byte', 'toShort': 'short', 'toChar': 'char', 'toInt': 'int', 'toLong': 'long',
                       'toFloat': 'float', 'toDouble': 'double'}


class KtPrecError(Exception):
    pass


def kt_prec_conversion(to, operand):
    """A numeric conversion, Java's cast `(int)x` or Kotlin's `x.toInt()` (`c.code`), as one node; a conversion
    through `Int` (`d.toInt().toByte()`, `c.code.toLong()`) is the one conversion Java writes."""
    while isinstance(operand, tuple) and operand[:2] == ('conv', 'int') and to != 'int':
        operand = operand[2]
    return ('conv', to, operand)


def kt_prec_spans(text_lines):
    """Kotlin tokens of some lines as (token, line, start, end): operators of several characters joined when
    written together (`?:`, `!==`, `!is`, `as?`), string templates one token."""
    out = []
    for n, line in enumerate(text_lines):
        raw = [(m.group(), m.start(), m.end()) for m in KT_PREC_TOKEN.finditer(line)]
        k = 0
        while k < len(raw):
            tok, s, e = raw[k]
            for j in KT_PREC_JOIN:
                seq = raw[k:k + len(j)]
                if len(seq) == len(j) and ''.join(t[0] for t in seq) == j and \
                        all(seq[x][2] == seq[x + 1][1] for x in range(len(seq) - 1)):
                    tok, e, k = j, seq[-1][2], k + len(j) - 1
                    break
            else:
                nxt = raw[k + 1] if k + 1 < len(raw) else None
                if nxt and nxt[1] == e and ((tok == '!' and nxt[0] in ('is', 'in')) or (tok == 'as' and nxt[0] == '?')):
                    tok, e, k = tok + nxt[0], nxt[2], k + 1
            out.append((tok, n, s, e))
            k += 1
    return out


def kt_prec_tokens(text_lines):
    return [(t, n) for t, n, _, _ in kt_prec_spans(text_lines)]


class KtPrecParser:
    """Reads Kotlin tokens as expressions under one operator precedence, Kotlin's or the Java one Vineflower
    parenthesises by; `canon()` is the token sequence with each expression it reads replaced by its tree
    (grouping parentheses gone) and the rest kept as it is."""

    def __init__(self, toks, kotlin):
        self.t = [x for x, _ in toks]
        self.line = [n for _, n in toks]
        self.kotlin = kotlin
        self.i = 0
        self.binary = KT_PREC_KOTLIN if kotlin else KT_PREC_JAVA
        self.typed = KT_PREC_KOTLIN_TYPE if kotlin else KT_PREC_JAVA_TYPE

    def peek(self, k=0):
        return self.t[self.i + k] if self.i + k < len(self.t) else None

    def take(self):
        tok = self.peek()
        if tok is None:
            raise KtPrecError('end')
        self.i += 1
        return tok

    def inner(self):
        """The canonical form of the bracketed tokens at the cursor, which it passes."""
        depth = 0
        for end in range(self.i, len(self.t)):
            depth += 1 if self.t[end] in KT_PREC_OPEN else -1 if self.t[end] in (')', ']', '}') else 0
            if depth == 0:
                break
        else:
            raise KtPrecError('unbalanced')
        sub = KtPrecParser(list(zip(self.t[self.i + 1:end], self.line[self.i + 1:end])), self.kotlin)
        self.i = end + 1
        return tuple(sub.canon())

    def canon(self):
        out = []
        while self.i < len(self.t):
            tok = self.t[self.i]
            if tok == ':':
                self.i += 1
                out.append(':')
                start = self.i
                try:
                    out.append(('type', self.type_()))
                except KtPrecError:
                    self.i = start
                continue
            if tok in ('[', '{'):
                try:
                    out += [tok, self.inner(), KT_PREC_OPEN[tok]]
                except KtPrecError:
                    out.append(tok)
                    self.i += 1
                continue
            if self.starts(tok) and tok not in KT_PREC_KEYWORDS:
                start = self.i
                try:
                    out.append(self.expr(99))
                    continue
                except KtPrecError:
                    self.i = start
            if tok == '(':
                try:
                    out += ['(', self.inner(), ')']
                    continue
                except KtPrecError:
                    pass
            out.append(tok)
            self.i += 1
        return out

    @staticmethod
    def starts(tok):
        return tok is not None and (tok in KT_PREC_PREFIX or tok in ('(', '{', '::') or tok[0] in '"\'`' or
                                    KT_PREC_NAME.match(tok) is not None)

    def type_(self):
        out = []
        if self.peek() == '(':
            out += ['(', self.inner(), ')']
            if self.peek() == '->':
                out += [self.take(), self.type_()]
            return tuple(out)
        if self.peek() == '*':
            return (self.take(),)
        while self.peek() in ('suspend', 'out', 'in', '@'):
            out.append(self.take())
        if self.peek() is None or not KT_PREC_NAME.match(self.peek()):
            raise KtPrecError('type')
        out.append(self.take())
        while self.peek() == '.' and self.peek(1) and KT_PREC_NAME.match(self.peek(1)):
            out += [self.take(), self.take()]
        if self.peek() == '<':
            depth = 0
            while True:
                tok = self.take()
                out.append(tok)
                depth += {'<': 1, '>': -1}.get(tok, 0)
                if depth == 0:
                    break
        if self.peek() == '?':
            out.append(self.take())
        return tuple(out)

    def expr(self, limit):
        left = self.prefix()
        while True:
            op = self.peek()
            if op is None:
                return left
            if op in self.typed and self.typed[op] <= limit:
                self.i += 1
                left = ('t', op, left, self.type_())
                continue
            if op in self.binary:
                level = self.binary[op]
            elif re.match(r'[A-Za-z_$`][\w$`]*$', op) and op not in KT_PREC_KEYWORDS \
                    and self.line[self.i] == self.line[self.i - 1] and self.starts(self.peek(1)) \
                    and self.peek(1) not in KT_PREC_KEYWORDS:
                level = 12 if self.kotlin else KT_PREC_JAVA.get(op, 18)
            else:
                return left
            if level > limit:
                return left
            self.i += 1
            if not self.kotlin and op not in KT_PREC_JAVA and op not in self.binary:
                left = ('b', op, left, self.expr(98))
                continue
            left = ('b', op, left, self.expr(level - 1))

    def prefix(self):
        tok = self.peek()
        if tok in KT_PREC_PREFIX:
            self.i += 1
            return ('p', tok, self.prefix())
        return self.postfix(self.primary())

    def primary(self):
        tok = self.peek()
        if tok == '(' and self.peek(1) in KT_PREC_CASTS and self.peek(2) == ')' and self.starts(self.peek(3)):
            to = self.peek(1)
            self.i += 3
            return kt_prec_conversion(to, self.prefix())
        if tok == '(':
            self.i += 1
            node = self.expr(99)
            if self.take() != ')':
                raise KtPrecError(')')
            return node
        if tok == '{':
            return ('lambda', self.inner())
        if tok == 'if':
            self.i += 1
            if self.peek() != '(':
                raise KtPrecError('if')
            cond = self.primary()
            then = self.expr(99)
            if self.peek() == 'else':
                self.i += 1
                return ('if', cond, then, self.expr(99))
            return ('if', cond, then)
        if tok in ('when', 'object', 'try'):
            out = [self.take()]
            while self.peek() is not None and self.peek() != '{':
                out.append(self.take())
            out.append(self.inner())
            while tok == 'try' and self.peek() in ('catch', 'finally'):
                out.append(self.take())
                if self.peek() == '(':
                    out.append(self.inner())
                out.append(self.inner())
            return ('block', tuple(map(str, out)))
        if tok in ('return', 'throw', 'break', 'continue'):
            n = self.line[self.i]
            self.i += 1
            label = ()
            if self.peek() == '@':
                label = (self.take(), self.take())
            if tok in ('return', 'throw') and self.peek() is not None and self.line[self.i] == n \
                    and self.starts(self.peek()) and self.peek() != '{':
                return ('jump', tok, label, self.expr(99))
            return ('jump', tok, label)
        if tok == '::':
            self.i += 1
            return ('ref', self.take())
        if tok is None or tok in KT_PREC_KEYWORDS or not self.starts(tok):
            raise KtPrecError('primary')
        self.i += 1
        return ('a', tok)

    def postfix(self, node):
        while True:
            tok = self.peek()
            if tok == '.' and self.peek(1) in KT_PREC_CONVERSIONS and self.t[self.i + 2:self.i + 4] == ['(', ')']:
                node = kt_prec_conversion(KT_PREC_CONVERSIONS[self.peek(1)], node)
                self.i += 4
            elif tok == '.' and self.peek(1) == 'code':
                node = kt_prec_conversion('int', node)
                self.i += 2
            elif tok in ('.', '?.', '::', '@') and self.peek(1) is not None and KT_PREC_NAME.match(self.peek(1)):
                self.i += 2
                node = ('m', tok, node, self.t[self.i - 1])
            elif tok in ('!!', '++', '--'):
                self.i += 1
                node = ('x', tok, node)
            elif tok == '(' and self.line[self.i] == self.line[self.i - 1]:
                node = ('call', node, self.inner())
            elif tok == '[':
                node = ('index', node, self.inner())
            elif tok == '<' and self.type_arguments():
                args, depth = [], 0
                while True:
                    t = self.take()
                    args.append(t)
                    depth += {'<': 1, '>': -1}.get(t, 0)
                    if depth == 0:
                        break
                node = ('targs', node, tuple(args))
            else:
                return node

    def type_arguments(self):
        depth = 0
        for k in range(self.i, len(self.t)):
            tok = self.t[k]
            depth += {'<': 1, '>': -1}.get(tok, 0)
            if depth == 0:
                return k + 1 < len(self.t) and self.t[k + 1] in ('(', '::', '.')
            if not (tok in ('<', '>', ',', '?', '*', '.', 'out', 'in') or KT_PREC_NAME.match(tok)):
                return False
        return False


def kt_prec_same_meaning(old, new):
    """Whether the new text read with Kotlin's operator precedence holds the same expressions as the old text read
    with the Java precedence Vineflower parenthesised it by (grouping parentheses aside, a Java cast and the
    Kotlin conversion call that stands for it one node)."""
    return KtPrecParser(kt_prec_tokens(old), False).canon() == KtPrecParser(kt_prec_tokens(new), True).canon()


KT_PREC_INFIX_OPS = {'and', 'or', 'xor', 'shl', 'shr', 'ushr', '?:', 'is', '!is'}


def kt_prec_operator_at(names, k):
    """The binary operator `names[k]` is (an operator token, or a name between two operands: an infix call), with
    its Kotlin and Java levels, or None."""
    tok = names[k]
    if tok in KT_PREC_KOTLIN or tok in KT_PREC_KOTLIN_TYPE:
        return tok, KT_PREC_KOTLIN.get(tok, KT_PREC_KOTLIN_TYPE.get(tok)), KT_PREC_JAVA.get(tok, KT_PREC_JAVA_TYPE.get(tok))
    if k and k + 1 < len(names) and re.match(r'[A-Za-z_$`][\w$`]*$', tok) and tok not in KT_PREC_KEYWORDS \
            and (re.match(r'[\w$`\'"]', names[k - 1]) or names[k - 1] in (')', ']')) \
            and names[k - 1] not in KT_PREC_KEYWORDS and KtPrecParser.starts(names[k + 1]):
        return tok, 12, KT_PREC_JAVA.get(tok, 18)
    return None


def kt_prec_undone(new):
    """The new lines without the grouping parentheses kt_operator_precedence adds: around an operation that is an
    operand of a binary operator, where Kotlin's precedence needs them and the Java precedence Vineflower writes
    by does not."""
    toks = kt_prec_spans(new)
    names = [t[0] for t in toks]
    close, stack = {}, []
    for k, tok in enumerate(names):
        if tok in KT_PREC_OPEN:
            stack.append(k)
        elif tok in (')', ']', '}') and stack:
            close[stack.pop()] = k
    drop = []
    for i, j in close.items():
        if names[i] != '(' or (i and (re.match(r'[\w$`\'"]', names[i - 1]) or names[i - 1] in (')', ']', '>'))
                                    and kt_prec_operator_at(names, i - 1) is None):
            continue
        depth, inner = 0, None
        for k in range(i + 1, j):
            depth += 1 if names[k] in KT_PREC_OPEN else -1 if names[k] in (')', ']', '}') else 0
            op = kt_prec_operator_at(names, k) if depth == 0 else None
            if op and (inner is None or op[1] >= inner[1]):
                inner = op
        if inner is None:
            continue
        before = kt_prec_operator_at(names, i - 1) if i else None
        after = kt_prec_operator_at(names, j + 1) if j + 1 < len(names) else None
        right = before is not None and (after is None or before[1] <= after[1])
        parent = before if right else after
        if parent is None:
            continue
        kotlin = inner[1] > parent[1] or (inner[1] == parent[1] and right and inner[0] != parent[0])
        eq = right or parent[0] in KT_PREC_INFIX_OPS
        java = inner[2] > parent[2] or (inner[2] == parent[2] and eq and not (
            inner[0] == parent[0] and inner[0] in ('and', 'or', 'xor', '+', '*', '&&', '||')))
        if right and parent[0] not in KT_PREC_JAVA and parent[0] not in KT_PREC_JAVA_TYPE:
            java = False
        if kotlin and not java:
            drop += [toks[i], toks[j]]
    lines = list(new)
    for _, n, s_, e in sorted(drop, key=lambda t: (t[1], -t[2])):
        lines[n] = lines[n][:s_] + lines[n][e:]
    return lines


def kt_operator_precedence(old, new):
    """kt_operator_precedence: the old text read with Kotlin's operator precedence means something else than read
    with Java's, by which Vineflower parenthesised it, and the new text read with Kotlin's means what the old
    meant (`x or 1 shl k` -> `x or (1 shl k)`, `a == b xor true` -> `(a == b) xor true`); parentheses and casts
    written as conversion calls aside, nothing else changed. Or, in a hunk other fixes change too, with the
    parentheses only Kotlin's precedence needs taken out (kt_prec_undone), another Kotlin fix's recogniser accepts
    it (one level deep)."""
    a = kt_prec_tokens(old)
    java = KtPrecParser(a, False).canon()
    if old == new or java == KtPrecParser(a, True).canon():
        return False
    if java == KtPrecParser(kt_prec_tokens(new), True).canon():
        return True
    return kt_ops_delegate(old, new, 'kt_operator_precedence')


RECOGNISERS['kt_operator_precedence'] = kt_operator_precedence


def kt_prec_depths(text_lines):
    """Per line, the bracket depth it ends at relative to its start and the lowest it reaches."""
    out = []
    for line in text_lines:
        depth = low = 0
        for tok, _ in kt_prec_tokens([line]):
            if tok in KT_PREC_OPEN:
                depth += 1
            elif tok in (')', ']', '}'):
                depth -= 1
                low = min(low, depth)
        out.append((depth, low))
    return out


def kt_prec_needs(depths, x1, x2):
    """Whether lines x1..x2 need lines before them (a bracket closed that opened earlier) and after them (one
    left open)."""
    depth = low = 0
    for d, line_low in depths[x1:x2]:
        low = min(low, depth + line_low)
        depth += d
    return low < 0, depth > low


_KT_PREC_FILES = {}


def kt_operator_precedence_context(a, b, i1, i2, j1, j2, throw_names):
    """The ktops fixes for a hunk inside a statement other hunks change too (a long expression the line diff
    cuts up): the smallest range of whole statements around it (brackets balanced on both sides, grown over
    shared lines and neighbouring hunks) that kt_operator_precedence, kt_narrow_bitwise_operand or
    kt_reference_inequality accepts."""
    key = (id(a), id(b))
    if key not in _KT_PREC_FILES or _KT_PREC_FILES[key][0] is not a or _KT_PREC_FILES[key][1] is not b:
        _KT_PREC_FILES.clear()
        ops = difflib.SequenceMatcher(None, a, b, autojunk=False).get_opcodes()
        _KT_PREC_FILES[key] = (a, b, ops, kt_prec_depths(a), kt_prec_depths(b), [])
    _, _, ops, da, db, accepted = _KT_PREC_FILES[key]
    if any(x1 <= i1 and i2 <= x2 and y1 <= j1 and j2 <= y2 for x1, x2, y1, y2 in accepted):
        return True
    k = next((n for n, op in enumerate(ops) if op[1] <= i1 and i2 <= op[2] and op[3] <= j1 and j2 <= op[4]), None)
    if k is None:
        return False
    lo = hi = k
    x1, x2, y1, y2 = i1, i2, j1, j2
    while True:
        before_a, after_a = kt_prec_needs(da, x1, x2)
        before_b, after_b = kt_prec_needs(db, y1, y2)
        if not (before_a or after_a or before_b or after_b):
            old, new = a[x1:x2], b[y1:y2]
            if kt_operator_precedence(old, new) or kt_narrow_bitwise_operand(old, new) or \
                    kt_reference_inequality(old, new):
                accepted.append((x1, x2, y1, y2))
                return True
            return False
        if before_a or before_b:
            if ops[lo][0] == 'equal' and x1 > ops[lo][1]:
                x1, y1 = x1 - 1, y1 - 1
            elif lo > 0:
                lo -= 1
                x1, y1 = (ops[lo][2] - 1, ops[lo][4] - 1) if ops[lo][0] == 'equal' else (ops[lo][1], ops[lo][3])
            else:
                return False
        if after_a or after_b:
            if ops[hi][0] == 'equal' and x2 < ops[hi][2]:
                x2, y2 = x2 + 1, y2 + 1
            elif hi + 1 < len(ops):
                hi += 1
                x2, y2 = (ops[hi][1] + 1, ops[hi][3] + 1) if ops[hi][0] == 'equal' else (ops[hi][2], ops[hi][4])
            else:
                return False


CONTEXT_RECOGNISERS['kt_operator_precedence'] = kt_operator_precedence_context


def kt_ops_delegate(old, new, me):
    """Whether, with the changes of kt_operator_precedence, kt_narrow_bitwise_operand and kt_reference_inequality
    undone (`!==`/`===` written `!=`/`==` and the widening conversions taken out on both sides, the parentheses
    only Kotlin's precedence needs taken out of the new text), the recogniser of another Kotlin output fix (`kt_`
    id) accepts the hunk; one level deep. The Java fixes' recognisers, some of which take any added parentheses,
    are not asked."""
    if _KT_DELEGATING:
        return False
    norm = lambda lines: [re.sub(r' (!=|==)= ', r' \1 ', x) for x in lines]
    old_text = '\n'.join(old)
    plain_old = kt_narrow_stripped(norm(old))[0]
    plain_new = kt_prec_undone(kt_narrow_stripped(norm([kt_narrow_compound(x, old_text) for x in new]))[0])
    if plain_new == list(new) and plain_old == list(old):
        return False
    _KT_DELEGATING.append(me)
    try:
        return any(r(plain_old, plain_new) for fix, r in RECOGNISERS.items()
                   if fix.startswith('kt_') and fix not in KT_OPS)
    finally:
        _KT_DELEGATING.pop()


KT_OPS = ('kt_operator_precedence', 'kt_narrow_bitwise_operand', 'kt_reference_inequality')


KT_NARROW_SUFFIXES = (('.', 'code', '.', 'toLong', '(', ')'), ('.', 'toLong', '(', ')'), ('.', 'toInt', '(', ')'),
                      ('.', 'code'))
KT_NARROW_OPS = {'and', 'or', 'xor', 'shl', 'shr', 'ushr'}
KT_NARROW_COMPOUND = re.compile(r'^(\s*)(.+?) = \((.+?)(?:\.toInt\(\)|\.code) (or|and|xor|shl|shr|ushr) (.+)\)'
                                r'\.(?:toByte|toShort|toChar)\(\)$')
KT_NARROW_ASSIGN = {'or': '|=', 'and': '&=', 'xor': '^=', 'shl': '<<=', 'shr': '>>=', 'ushr': '>>>='}


def kt_narrow_after_op(before, end):
    """Whether the operand that ends the tokens `before[:end]` is the right operand of a bitwise or shift
    operator (or of the compound assignment `op=`)."""
    depth = 0
    for k in range(end - 1, -1, -1):
        tok = before[k]
        if tok in (')', ']'):
            depth += 1
        elif tok in ('(', '['):
            if depth == 0:
                return False
            depth -= 1
        elif depth == 0 and tok in KT_NARROW_OPS:
            return True
        elif depth == 0 and tok == '=' and k > 0 and before[k - 1] in ('^', '|', '&', '<', '>'):
            return True
        elif depth == 0 and not (re.match(r'[\w$`\'"]', tok) or tok in ('.', '?.', '!!', '++', '--', '::', '@')):
            return False
    return False


def kt_narrow_stripped(text_lines):
    """The lines without the widening conversions of operands of bitwise and shift operators (`b.toInt() and
    255` -> `b and 255`, `x shl n.toInt()` -> `x shl n`, `c.code.inv()`), and how many it took out."""
    toks = kt_prec_spans(text_lines)
    names = [t[0] for t in toks]
    drop, j = [], 0
    while j < len(toks):
        group = next((g for g in KT_NARROW_SUFFIXES if tuple(names[j:j + len(g)]) == g), None)
        if group:
            after = names[j + len(group)] if j + len(group) < len(names) else ''
            nxt = names[j + len(group) + 1] if j + len(group) + 1 < len(names) else ''
            if after in KT_NARROW_OPS or (after == '.' and nxt == 'inv') or kt_narrow_after_op(names, j):
                drop += toks[j:j + len(group)]
                j += len(group)
                continue
        j += 1
    lines = list(text_lines)
    for _, n, s, e in sorted(drop, key=lambda t: (t[1], -t[2])):
        lines[n] = lines[n][:s] + lines[n][e:]
    return lines, sum(1 for t in drop if t[0] in ('toInt', 'toLong') or (t[0] == 'code'))


def kt_narrow_compound(line, old_text):
    """`x = (x.toInt() op y).toByte()` written `x op= y` again where the old text has that."""
    m = KT_NARROW_COMPOUND.match(line)
    if not m or m.group(2) != m.group(3):
        return line
    ind, left, _, name, right = m.groups()
    return f'{ind}{left} {KT_NARROW_ASSIGN[name]} {right}' if f'{left} {KT_NARROW_ASSIGN[name]} ' in old_text else line


def kt_narrow_bitwise_operand(old, new):
    """kt_narrow_bitwise_operand: `.toInt()`, `.code`, `.toLong()` (`.code.toLong()`) added to operands of `and or
    xor shl shr ushr` and before `.inv()` (possibly in parentheses), and `x op= y` to a Byte, Short or Char place
    written `x = (x.toInt() op y).toByte()`: with those conversions taken out of both sides (and the compound
    written back) the text is the same, or means the same read as kt_operator_precedence reads it (parentheses
    added, casts written as conversion calls), or another fix's recogniser accepts it (one level deep)."""
    old_text = '\n'.join(old)
    plain_old, n_old = kt_narrow_stripped(old)
    plain_new, n_new = kt_narrow_stripped([kt_narrow_compound(x, old_text) for x in new])
    compound = [kt_narrow_compound(x, old_text) for x in new] != list(new)
    if n_new <= n_old and not compound:
        return False
    if kt_prec_tokens(plain_old) == kt_prec_tokens(plain_new) or kt_prec_same_meaning(plain_old, plain_new):
        return True
    return kt_ops_delegate(old, new, 'kt_narrow_bitwise_operand')


RECOGNISERS['kt_narrow_bitwise_operand'] = kt_narrow_bitwise_operand


KT_IDENTITY = {('!=', '!=='), ('==', '===')}
KT_IDENTITY_OP = re.compile(r' (!==|===|!=|==) ')


def kt_identity_same(a, b):
    """Whether token `b` is token `a` with some `!=`/`==` made `!==`/`===` (in a string template too)."""
    if a == b or (a, b) in KT_IDENTITY:
        return True
    if not (a.startswith('"') and b.startswith('"')):
        return False
    pa, pb = KT_IDENTITY_OP.split(a), KT_IDENTITY_OP.split(b)
    return len(pa) == len(pb) and all(x == y or (x, y) in KT_IDENTITY for x, y in zip(pa, pb))


def kt_reference_inequality(old, new):
    """kt_reference_inequality: `!=` became `!==` (a negated identity `==` `===`), in a string template too;
    nothing else, or with those written back kt_operator_precedence or kt_narrow_bitwise_operand accepts the hunk,
    or (kt_ops_delegate) another Kotlin fix's recogniser."""
    a, b = [t for t, _ in kt_prec_tokens(old)], [t for t, _ in kt_prec_tokens(new)]
    if len(a) == len(b) and a != b and all(kt_identity_same(x, y) for x, y in zip(a, b)):
        return True
    added = sum(len(re.findall(r' (?:!==|===) ', x)) for x in new) - sum(len(re.findall(r' (?:!==|===) ', x)) for x in old)
    if added <= 0 or _KT_DELEGATING:
        return False
    norm = lambda lines: [re.sub(r' (!=|==)= ', r' \1 ', x) for x in lines]
    plain_old, plain_new = norm(old), norm(new)
    if plain_old == plain_new or kt_operator_precedence(plain_old, plain_new) or \
            kt_narrow_bitwise_operand(plain_old, plain_new):
        return True
    return kt_ops_delegate(old, new, 'kt_reference_inequality')


RECOGNISERS['kt_reference_inequality'] = kt_reference_inequality


# ---- kuna fixes of ktvalid: Kotlin output kotlinc accepts ----

KTVALID_ORDER = ['kt_accessor_backing_field', 'kt_string_literal_escape', 'kt_abstract_function_body',
                 'kt_misplaced_modality', 'kt_companion_property_initializer', 'kt_default_property_initializer',
                 'kt_missed_override', 'kt_outer_this', 'kt_extension_receiver', 'kt_nested_annotation_argument',
                 'kt_throws_class_name', 'kt_throwable_cast', 'kt_mapped_java_member', 'kt_mapped_java_type',
                 'kt_object_instance', 'kt_super_property_access', 'kt_setter_parameter_name', 'kt_java_class_literal']
KTV_MAPPED_TYPES = {'String': 'String', 'CharSequence': 'CharSequence', 'Throwable': 'Throwable', 'Number': 'Number',
                    'Long': 'Long', 'Short': 'Short', 'Byte': 'Byte', 'Character': 'Char', 'Float': 'Float',
                    'Double': 'Double', 'Boolean': 'Boolean'}
KTV_MAPPED_TYPE = re.compile(r'\bjava\.lang\.(' + '|'.join(KTV_MAPPED_TYPES) + r')\b(?![\w$])')
KTV_MEMBERS = [(re.compile(r'\.length\(\)'), '.size'), (re.compile(r'\.length\b(?![\w$(])'), '.size'),
               (re.compile(r'\.size\(\)'), '.size'), (re.compile(r'\.keySet\(\)'), '.keys'),
               (re.compile(r'\.values\(\)'), '.values'), (re.compile(r'\.entrySet\(\)'), '.entries'),
               (re.compile(r'\.getKey\(\)'), '.key'), (re.compile(r'\.getValue\(\)'), '.value'),
               (re.compile(r'\.getMessage\(\)'), '.message'), (re.compile(r'\.getCause\(\)'), '.cause'),
               (re.compile(r'\.name\(\)'), '.name'), (re.compile(r'\.ordinal\(\)'), '.ordinal'),
               (re.compile(r'\.getClass\(\)'), '.javaClass'), (re.compile(r'\.intValue\(\)'), '.toInt()'),
               (re.compile(r'\.longValue\(\)'), '.toLong()'), (re.compile(r'\.shortValue\(\)'), '.toShort()'),
               (re.compile(r'\.byteValue\(\)'), '.toByte()'), (re.compile(r'\.floatValue\(\)'), '.toFloat()'),
               (re.compile(r'\.doubleValue\(\)'), '.toDouble()'), (re.compile(r'\.remove\('), '.removeAt(')]
KTV_DEFAULT_LITERALS = ('0', '0L', '0.0', '0.0F', 'false', 'null', "'\\u0000'")
KTV_PROPERTY = re.compile(r'^\s*(?:@[\w.]+(?:\(.*\))? )*(?:(?:public|private|protected|internal|final|open|override|'
                          r'abstract|const|lateinit|external) )*(?:val|var) [^=]+$')
KTV_ACCESSOR_MODALITY = re.compile(r'^(\s*(?:(?:public|private|protected|internal) )?)(?:final|open) '
                                   r'(?=(?:external |inline )*(?:get|set)\b)')
KTV_TOP_FINAL = re.compile(r'^(\s*(?:@\S+ )*(?:(?:public|private|protected|internal) )?)final (?=val |var )')
KTV_ABSTRACT_HEAD = re.compile(r'^(\s*)(?:.*\b(?:abstract|external)\b.*\bfun\b.*\S|\).*) \{$')
KTV_THROWS = re.compile(r'@Throws\((.*)\)')


def ktv_char_at(text):
    """`x.charAt(e)` written `x[e]`, the argument's brackets balanced."""
    out, i = [], 0
    while True:
        k = text.find('.charAt(', i)
        if k < 0:
            out.append(text[i:])
            return ''.join(out)
        depth, j = 0, k + 7
        while j < len(text):
            depth += {'(': 1, ')': -1}.get(text[j], 0)
            if depth == 0:
                break
            j += 1
        if j >= len(text):
            out.append(text[i:])
            return ''.join(out)
        out.append(text[i:k] + '[' + text[k + 8:j] + ']')
        i = j + 1


def ktv_unwrap_throw(line):
    """A trailing cast to `Throwable` dropped (a thrown value's, also where the statement wraps), and the
    parentheses it needed."""
    m = re.match(r'^(.*?\bthrow )(.*) as Throwable$', line)
    if not m:
        return line[:-len(' as Throwable')] if line.endswith(' as Throwable') else line
    value = m.group(2)
    if value.startswith('(') and block_end(list(value), 0) == len(value) - 1:
        value = value[1:-1]
    return m.group(1) + value


def ktv_merge_abstract(text_lines):
    out, k = [], 0
    while k < len(text_lines):
        line = text_lines[k]
        m = KTV_ABSTRACT_HEAD.match(line)
        if m and k + 1 < len(text_lines) and text_lines[k + 1] == m.group(1) + '}':
            out.append(line[:-2])
            k += 2
            continue
        out.append(line)
        k += 1
    return out


def ktv_throws_names(line):
    m = KTV_THROWS.search(line)
    if not m:
        return line
    names = [re.split(r'[/$.]', n.strip()[:-len('::class')])[-1] if n.strip().endswith('::class') else n.strip()
             for n in m.group(1).split(',')]
    return line[:m.start()] + '@Throws(' + ', '.join(n + '::class' for n in names) + ')' + line[m.end():]


KTV_STEPS = [
    ('kt_string_literal_escape', lambda s: re.sub(r'\$\{(`[^`\n]+`|[A-Za-z_]\w*)\}', r'$\1',
                                                   s.replace('\\$', '$').replace('\\u000c', '\\f')
                                                   .replace('\\"', '"').replace('\\n', '\n')
                                                   .replace('\\t', '\t'))),
    ('kt_missed_override', lambda s: re.sub(r'\b(?:override|open) (?=(?:\w+ )*(?:fun|val|var)\b)', '', s)),
    ('kt_misplaced_modality', lambda s: '\n'.join(KTV_TOP_FINAL.sub(r'\1', KTV_ACCESSOR_MODALITY.sub(r'\1', x))
                                                 for x in s.split('\n'))),
    ('kt_extension_receiver', lambda s: re.sub(r'`\$this\$[\w$]+`', 'this', s)),
    ('kt_outer_this', lambda s: re.sub(r'\$\{(this(?:@[\w$]+)?)\}', r'$\1',
                                       re.sub(r'\bthis@[\w$`]+', 'this',
                                              re.sub(r'(?<![\w.])[A-Za-z_][\w$]*\.this\b', 'this', s)))),
    ('kt_nested_annotation_argument', lambda s: re.sub(r'= @(?=[A-Z])', '= ', s)),
    ('kt_throws_class_name', lambda s: '\n'.join(ktv_throws_names(x) for x in s.split('\n'))),
    ('kt_mapped_java_type', lambda s: re.sub(r'(?<![\w$.])Character\b(?![\w$(])', 'Char',
                                             KTV_MAPPED_TYPE.sub(lambda m: KTV_MAPPED_TYPES[m.group(1)], s))),
    ('kt_throwable_cast', lambda s: '\n'.join(ktv_unwrap_throw(x) for x in s.split('\n'))),
    ('kt_mapped_java_member', lambda s: functools.reduce(lambda t, p: p[0].sub(p[1], t), KTV_MEMBERS,
                                                         ktv_char_at(s))),
    ('kt_object_instance', lambda s: re.sub(r'\.INSTANCE\b(?![\w$])', '', s)),
    ('kt_super_property_access', lambda s: re.sub(r'\bsuper(?:<[\w.]+>)?\.', 'this.', s)),
    ('kt_setter_parameter_name', lambda s: s.replace('set(<set-?>)', 'set(value)')),
    ('kt_java_class_literal', lambda s: re.sub(
        r'(?<![\w$.:])([A-Za-z_][\w$]*(?:\.[A-Za-z_][\w$]*)*(?:\[\])*)\.class\b(?![\w$])',
        lambda m: ktv_class_literal_type(m.group(1)) + '::class.java', s)),
]
def ktv_class_literal_type(name):
    """The type a Java class literal `name` (`byte[]`, `String[][]`) takes in Kotlin: an array class literal is
    written by fix kt_java_type_syntax in Kotlin's array syntax (`ByteArray`, `Array<Array<String>>`)."""
    dims = name.count('[]')
    if not dims:
        return name
    base = name[:-2 * dims]
    if base in KTV_PRIMITIVE_ARRAYS:
        base, dims = KTV_PRIMITIVE_ARRAYS[base], dims - 1
    else:
        base = {'Object': 'Any', 'java.lang.Object': 'Any'}.get(base, KTV_MAPPED_TYPES.get(base.removeprefix('java.lang.'), base))
    return 'Array<' * dims + base + '>' * dims


KTV_PRIMITIVE_ARRAYS = {'boolean': 'BooleanArray', 'byte': 'ByteArray', 'char': 'CharArray', 'short': 'ShortArray',
                        'int': 'IntArray', 'long': 'LongArray', 'float': 'FloatArray', 'double': 'DoubleArray'}


KTV_SIGNATURES = {
    'kt_string_literal_escape': r'\\\$|\\u000c|\$\{|\\[nt"]',
    'kt_misplaced_modality': r'(?:final|open) (?:external |inline )*(?:get|set)\b|^\s*(?:\S+ )?final (?:val|var) ',
    'kt_missed_override': r'\boverride\b',
    'kt_extension_receiver': r'`\$this\$|\bthis\b',
    'kt_outer_this': r'\.this\b|this@',
    'kt_nested_annotation_argument': r'= @[A-Z]',
    'kt_throws_class_name': r'@Throws\(.*\)',
    'kt_mapped_java_type': r'java\.lang\.|\bChar(?:acter)?\b',
    'kt_throwable_cast': r'as (?:java\.lang\.)?Throwable',
    'kt_mapped_java_member': r'\.(?:length|size|keySet|values|entrySet|getKey|getValue|getMessage|getCause|name|ordinal|'
                             r'getClass|intValue|longValue|shortValue|byteValue|floatValue|doubleValue|charAt|remove)\(|'
                             r'\.(?:length|size|keys|entries|key|value|message|cause|javaClass)\b|\[',
    'kt_object_instance': r'\bINSTANCE\b',
    'kt_super_property_access': r'\bsuper\.',
    'kt_setter_parameter_name': r'\bset\([^)]*\)|\bvalue\b',
    'kt_java_class_literal': r'\.class\b|::class\.java',
}


def ktv_canon(text_lines):
    """Lines with the ktvalid fixes' forms written alike on both sides: the text, and the fixes whose step
    changed it."""
    text = '\n'.join(ktv_merge_abstract(list(text_lines)))
    changed = set()
    if text != '\n'.join(text_lines):
        changed.add('kt_abstract_function_body')
    for fix, step in KTV_STEPS:
        new_text = step(text)
        if new_text != text:
            changed.add(fix)
        text = new_text
    return text.split('\n'), changed


KTV_DECLARATION = re.compile(r'^\s*(?:@|(?:(?:public|private|protected|internal|final|open|override|const|lateinit|'
                             r'abstract|inner|data|inline) )*(?:val|var|fun|class|object(?! :)|interface|init|get|set)\b)')


def ktv_decl(line):
    """A declaration line without indentation and modality (which kt_misplaced_modality and kt_missed_override
    change)."""
    return re.sub(r'\b(?:final|open|override) ', '', line.strip())


KTV_PROPERTY_KEY = re.compile(r'^((?:[\w@.]+ )*(?:val|var) `?[\w$]+`?)\s*:')


def ktv_property_key(line):
    """A property declaration line's modifiers, kind and name (ktv_decl), without its type."""
    m = KTV_PROPERTY_KEY.match(ktv_decl(line))
    return m.group(1) if m else None


def ktv_initialized(line, keys):
    """(the line without its initializer, the initializer's text) when `line` declares, with an initializer, a
    property one of `keys` (ktv_property_key) declares without; else None."""
    if ' = ' not in line and not line.endswith(' ='):
        return None
    key = ktv_property_key(line)
    if key is None or key not in keys:
        return None
    cut = line.find(' = ') if ' = ' in line else len(line) - 2
    return line[:cut], line[cut + 2:].strip()


def ktv_depth(text):
    """The brackets `text` opens less those it closes."""
    return sum(text.count(c) for c in '({[') - sum(text.count(c) for c in ')}]')


def ktv_undo_initializers(old, new):
    """`new` with each initializer added to a property line of `old` (and its continuation lines) taken out,
    and the fixes that added them."""
    olds = set(ktv_property_key(o) for o in old if KTV_PROPERTY.match(o))
    out, fired, k = [], set(), 0
    while k < len(new):
        line = new[k]
        split = ktv_initialized(line, olds)
        if split is None:
            out.append(line)
            k += 1
            continue
        head, init = split
        fired.add('kt_default_property_initializer' if init in KTV_DEFAULT_LITERALS
                  else 'kt_companion_property_initializer')
        out.append(head)
        k += 1
        indent = len(head) - len(head.lstrip())
        depth = ktv_depth(init)
        if not init and k < len(new):
            depth += ktv_depth(new[k])
            k += 1
        while k < len(new) and (depth > 0 or not init and new[k].strip() and
                                len(new[k]) - len(new[k].lstrip()) > indent and
                                depth + ktv_depth(new[k]) >= 0 and not KTV_DECLARATION.match(new[k])):
            depth += ktv_depth(new[k])
            k += 1
    return out, fired


KTV_RECEIVER_NAME = re.compile(r'^(?:var\d+|\$receiver|`<this>`)$')


def ktv_field_equal(a, b):
    """Token lists equal where `b` may have `field` for `a`'s `this.x`/`Owner.x` (a backing field access),
    and `this` for one variable `a` names `varN` (an extension receiver whose debug name is not an
    identifier): whether they are, and which of the two it took."""
    i = j = 0
    used, receiver, setter = set(), set(), set()
    while i < len(a) and j < len(b):
        if a[i] == b[j]:
            i += 1
            j += 1
        elif b[j] == 'field' and i + 2 < len(a) and a[i + 1] == '.' and IDENT.match(a[i]) and IDENT.match(a[i + 2]):
            i += 3
            j += 1
            used.add('kt_accessor_backing_field')
        elif b[j] == 'this' and KTV_RECEIVER_NAME.match(a[i]) and receiver <= {a[i]}:
            receiver.add(a[i])
            i += 1
            j += 1
            used.add('kt_extension_receiver')
        elif b[j] == 'value' and re.match(r'^var\d+$', a[i]) and setter <= {a[i]}:
            setter.add(a[i])
            i += 1
            j += 1
            used.add('kt_setter_parameter_name')
        elif a[i] == 'INSTANCE' and IDENT.match(b[j]) and (i == 0 or a[i - 1] != '.'):
            i += 1
            j += 1
            used.add('kt_object_instance')
        elif a[i].startswith('"') and b[j].startswith('"') and \
                re.sub(r'(?<![\w$])var\d+(?![\w$])', 'this', a[i]) == b[j]:
            i += 1
            j += 1
            used.add('kt_extension_receiver')
        else:
            return False, used
    return i == len(a) and j == len(b), used


def ktvalid_undo(old, new):
    """The ktvalid fixes that explain the hunk (in KTVALID_ORDER), or an empty list; with the canonical texts."""
    plain_new, fired = ktv_undo_initializers(old, new)
    a, ca = ktv_canon(old)
    b, cb = ktv_canon(plain_new)
    ta, tb = tokens(a), tokens(b)
    same, used = (True, set()) if ta == tb else ktv_field_equal(ta, tb)
    fired |= used
    old_text, new_text = '\n'.join(old), '\n'.join(new)
    for fix in ca | cb:
        sig = KTV_SIGNATURES.get(fix)
        if fix == 'kt_abstract_function_body' or sig is None or \
                len(re.findall(sig, old_text, re.M)) != len(re.findall(sig, new_text, re.M)) or \
                re.findall(sig, old_text, re.M) != re.findall(sig, new_text, re.M):
            fired.add(fix)
    return [f for f in KTVALID_ORDER if f in fired], same, a, b


def ktvalid_recogniser(fix):
    """The recogniser of one ktvalid fix: with the ktvalid fixes' forms written alike (ktv_canon), the hunk's
    two sides are the same tokens, or another fix's recogniser accepts them; the hunk counts under the first
    ktvalid fix (KTVALID_ORDER) that changed it."""
    def recognise(old, new):
        if old == new:
            return False
        fired, same, a, b = ktvalid_undo(old, new)
        if not fired or fired[0] != fix:
            return False
        return same or (a != list(old) or b != list(new)) and kt_other_recogniser_accepts(a, b, fix)
    recognise.__name__ = fix
    return recognise


RECOGNISERS.update((fix, ktvalid_recogniser(fix)) for fix in KTVALID_ORDER)


_KTVALID_FILES = {}
_KTVALID_ACTIVE = []


def ktv_file_canon(a, b):
    """Both files with the ktvalid forms written alike, line by line: the old text with abstract functions'
    empty bodies merged into their headers, the new one with the initializers added to the old text's property
    lines taken out (their continuation lines folded into the head), then every line through ktv_canon; with
    the index of each original line's canonical line, and the original lines a ktvalid fix touched."""
    wa, merged = [], []
    k = 0
    while k < len(a):
        m = KTV_ABSTRACT_HEAD.match(a[k])
        wa.append(len(merged))
        if m and k + 1 < len(a) and a[k + 1] == m.group(1) + '}':
            merged.append(a[k][:-2])
            wa.append(len(merged) - 1)
            k += 2
            continue
        merged.append(a[k])
        k += 1
    olds = set(ktv_property_key(o) for o in a if KTV_PROPERTY.match(o))
    wb, undone, k = [], [], 0
    touched_b = set()
    while k < len(b):
        line = b[k]
        split = ktv_initialized(line, olds) if '=' in line else None
        if split is None:
            wb.append(len(undone))
            undone.append(line)
            k += 1
            continue
        head, init = split
        start = k
        wb.append(len(undone))
        undone.append(head)
        k += 1
        depth = ktv_depth(init)
        if not init and k < len(b):
            depth += ktv_depth(b[k])
            wb.append(len(undone) - 1)
            k += 1
        indent = len(line) - len(line.lstrip())
        while k < len(b) and (depth > 0 or not init and b[k].strip() and len(b[k]) - len(b[k].lstrip()) > indent and
                              depth + ktv_depth(b[k]) >= 0 and not KTV_DECLARATION.match(b[k])):
            depth += ktv_depth(b[k])
            wb.append(len(undone) - 1)
            k += 1
        touched_b.update(range(start, k))
    ca = ['\n'.join(ktv_canon([x])[0]) for x in merged]
    cb = ['\n'.join(ktv_canon([x])[0]) for x in undone]
    touched_a = {x for x in range(len(a)) if a[x] != ca[wa[x]]}
    touched_b |= {y for y in range(len(b)) if b[y] != cb[wb[y]]}
    return ca, cb, wa, wb, touched_a, touched_b


def ktvalid_file(a, b, i1, i2, j1, j2, throw_names):
    """The ktvalid fixes' changes in a hunk that also holds other fixes' (or that line matching split): with the
    ktvalid forms written alike over both files (ktv_file_canon), a ktvalid fix touched one of the hunk's lines,
    and each of its lines is a line equal in both canonical texts or in a canonical difference another
    recogniser (or context recogniser) accepts. Only the canonical differences a hunk reaches are classified."""
    if _KTVALID_ACTIVE:
        return False
    key = (id(a), id(b), len(a), len(b))
    if key not in _KTVALID_FILES:
        ca, cb, wa, wb, touched_a, touched_b = ktv_file_canon(a, b)
        ops = [op for op in difflib.SequenceMatcher(None, ca, cb, autojunk=False).get_opcodes() if op[0] != 'equal']
        _KTVALID_FILES.clear()
        _KTVALID_FILES[key] = (ca, cb, wa, wb, touched_a, touched_b, ops, {})
    ca, cb, wa, wb, touched_a, touched_b, ops, verdicts = _KTVALID_FILES[key]
    if not (any(x in touched_a for x in range(i1, i2)) or any(y in touched_b for y in range(j1, j2))):
        return False
    lines_a = {wa[x] for x in range(i1, i2)}
    lines_b = {wb[y] for y in range(j1, j2)}
    for k, (_, x1, x2, y1, y2) in enumerate(ops):
        if not (any(x1 <= x < x2 for x in lines_a) or any(y1 <= y < y2 for y in lines_b)):
            continue
        if k not in verdicts:
            _KTVALID_ACTIVE.append(key)
            try:
                verdicts[k] = tokens(ca[x1:x2]) == tokens(cb[y1:y2]) or \
                    classify(ca, cb, x1, x2, y1, y2, set()) not in ('other', 'layout') or \
                    any(r(ca, cb, x1, x2, y1, y2, throw_names) for fix, r in CONTEXT_RECOGNISERS.items()
                        if fix != 'ktvalid_file')
            finally:
                _KTVALID_ACTIVE.pop()
        if not verdicts[k]:
            return False
    return True


CONTEXT_RECOGNISERS['ktvalid_file'] = ktvalid_file


KT2_NEGATED = {'==': '!=', '!=': '==', '===': '!==', '!==': '==='}
KT2_BOXES = {('java', '.', 'lang', '.', name): (name,) for name in ('Boolean', 'Byte', 'Short', 'Long', 'Float', 'Double')}
KT2_BOXES[('Character',)] = ('Char',)
KT2_PROMOTED = {'+', '-', '*', '/', '%', '<', '>', '<=', '>=', '==', '!='}
KT2_WIDE = ('int', 'long', 'float', 'double')
KT2_BREAK_OPS = {'+', '-', '*', '/', '%', '<', '>', '<=', '>=', '==', '!=', '===', '!=='}


def kt2_canon(text_lines):
    """The lines read with Kotlin's operator precedence (KtPrecParser), or None."""
    try:
        return KtPrecParser(kt_prec_tokens(text_lines), True).canon()
    except (KtPrecError, RecursionError, IndexError):
        return None


def kt2_map(x, f):
    """`x` (a canonical form, its trees and lists) with `f` applied to every tuple, innermost first."""
    if isinstance(x, list):
        return [kt2_map(y, f) for y in x]
    if isinstance(x, tuple):
        return f(tuple(kt2_map(y, f) for y in x))
    return x


def kt2_negate(node):
    """kt_negated_equality: `(a == b) xor true` read as `a != b` (and the other comparisons)."""
    if len(node) == 4 and node[:2] == ('b', 'xor') and node[3] == ('a', 'true') and \
            isinstance(node[2], tuple) and len(node[2]) == 4 and node[2][0] == 'b' and node[2][1] in KT2_NEGATED:
        return ('b', KT2_NEGATED[node[2][1]], node[2][2], node[2][3])
    return node


def kt2_unbox_name(node):
    """kt_unboxing_cast_type: `x as java.lang.Boolean` read as `x as Boolean` (`Character` as `Char`)."""
    if len(node) == 4 and node[:2] == ('t', 'as') and node[3] in KT2_BOXES:
        return ('t', 'as', node[2], KT2_BOXES[node[3]])
    return node


def kt2_unbox_reparse(node):
    """kt_operator_precedence (an unboxing call not written): `!x as T`, which Vineflower means as `!(x as T)`,
    read so."""
    if len(node) == 4 and node[:2] == ('t', 'as') and isinstance(node[2], tuple) and len(node[2]) == 3 and \
            node[2][0] == 'p' and node[2][1] in ('!', '-'):
        return ('p', node[2][1], ('t', 'as', node[2][2], node[3]))
    return node


def kt2_identity_default(node):
    """kt_copied_equality: `a === b` read as `a == b`."""
    if len(node) == 4 and node[:2] == ('b', '==='):
        return ('b', '==', node[2], node[3])
    return node


def kt2_conversions(x):
    """How many widening conversions (`.code`, `.toInt()`, `.toLong()`, ...) are direct operands of arithmetic,
    comparison and equality operators and of a unary minus in `x`."""
    count = 0

    def visit(y):
        nonlocal count
        if isinstance(y, list):
            for z in y:
                visit(z)
        elif isinstance(y, tuple):
            if y[:1] == ('b',) and len(y) == 4 and y[1] in KT2_PROMOTED or y[:2] == ('p', '-') and len(y) == 3:
                count += sum(1 for z in y[2:] if isinstance(z, tuple) and z[:1] == ('conv',) and z[1] in KT2_WIDE)
            for z in y:
                visit(z)
    visit(x)
    return count


def kt2_unpromote(node):
    """kt_binary_numeric_promotion: the widening conversions of the operands of arithmetic, comparison and
    equality operators and of a unary minus taken out."""
    strip = lambda z: z[2] if isinstance(z, tuple) and z[:1] == ('conv',) and z[1] in KT2_WIDE else z
    if len(node) == 4 and node[0] == 'b' and node[1] in KT2_PROMOTED:
        return ('b', node[1], strip(node[2]), strip(node[3]))
    if len(node) == 3 and node[:2] == ('p', '-'):
        return ('p', '-', strip(node[2]))
    return node


KT2_FORWARD = (('kt_negated_equality', kt2_negate), ('kt_unboxed_cast', kt2_unbox_reparse),
               ('kt_unboxing_cast_type', kt2_unbox_name), ('kt_copied_equality', kt2_identity_default))


def kt2_default_value_or_supertype(text_lines):
    """Whether the lines are a function header with a default value or an object expression's supertype call,
    where a copied `==` is written (kt_copied_equality)."""
    text = ' '.join(text_lines)
    return bool(re.search(r'\bfun\b[^{]*\([^)]*=', text) or re.search(r'\bobject\s*:', text))


def kt2_recognise(old, new, me):
    """Whether kt2_fix gives the hunk to `me`."""
    return kt2_fix(tuple(old), tuple(new), bool(_KT_DELEGATING), bool(_KT2_DELEGATING)) == me


@functools.lru_cache(maxsize=8192)
def kt2_fix(old, new, delegating, delegating2):
    """The ktops2 fix whose change, with the others', turns `old` into `new`, computed once for the
    recognisers of all of them (and per delegation state, on which it depends), or None: the old text read
    with Kotlin's precedence (or with the Java precedence Vineflower parenthesises by), with `(a == b) xor
    true` read `a != b`, `!x as T` read `!(x as T)`, the box classes of casts read as Kotlin's types, `===`
    read `==` (in a default value or supertype call only), holds the expressions of the new text read with
    Kotlin's, the widening conversions of arithmetic, comparison and equality operands taken out of both; the
    first change in KT2_FORWARD order that applies. Or what kt2_delegate gives."""
    old, new = list(old), list(new)
    if old == new:
        return None
    n = kt2_canon(new)
    if n is None:
        return None
    for kotlin in (True, False):
        try:
            o = KtPrecParser(kt_prec_tokens(old), kotlin).canon()
        except (KtPrecError, RecursionError, IndexError):
            continue
        fix = kt2_applied(old, o, n)
        if fix:
            return fix
    return kt2_delegate(old, new)


_KT2_DELEGATING = []


def kt2_delegate(old, new):
    """With the new lines that ktops2's changes make of an old line (aligned by difflib) written back, the
    fix of the first such line, if the hunk is then the old one or another Kotlin fix's recogniser accepts it
    (which may delegate in turn, as kt_ops_delegate does, but not back here); else None."""
    if _KT2_DELEGATING:
        return None
    _KT2_DELEGATING.append(True)
    try:
        undone, mine = list(new), []
        for tag, i1, i2, j1, j2 in difflib.SequenceMatcher(None, old, new, autojunk=False).get_opcodes():
            if tag != 'replace' or i2 - i1 != j2 - j1:
                continue
            for k in range(i2 - i1):
                fix = kt2_fix((old[i1 + k],), (new[j1 + k],), bool(_KT_DELEGATING), True)
                if fix:
                    mine.append(fix)
                    undone[j1 + k] = old[i1 + k]
        if not mine:
            return None
        if undone == old or any(r(old, undone) for fix, r in RECOGNISERS.items()
                                if fix.startswith('kt') and fix not in KT2_FIXES):
            return mine[0]
        return None
    finally:
        _KT2_DELEGATING.pop()


def kt2_applied(old, o, n):
    """The first ktops2 change that, with the later ones, makes `o` (the old text's canonical form) `n`, or
    None."""
    applied = []
    for fix, f in KT2_FORWARD:
        if fix == 'kt_copied_equality' and not kt2_default_value_or_supertype(old):
            continue
        t = kt2_map(o, f)
        if t != o:
            applied.append(fix)
            o = t
    if kt2_conversions(n) > kt2_conversions(o):
        applied.append('kt_binary_numeric_promotion')
    if not applied or kt2_map(o, kt2_unpromote) != kt2_map(n, kt2_unpromote):
        return None
    return applied[0]


KT2_FIXES = ('kt_negated_equality', 'kt_unboxed_cast', 'kt_unboxing_cast_type', 'kt_copied_equality',
             'kt_binary_numeric_promotion', 'kt_operator_line_break')


def kt_negated_equality(old, new):
    """kt_negated_equality: `(a == b) xor true` (an `==`, `!=`, `===` or `!==` that `xor true` negates) written
    `a != b` (the negated operator); with the other ktops2 changes (kt2_recognise)."""
    return kt2_recognise(old, new, 'kt_negated_equality')


def kt_unboxed_cast(old, new):
    """kt_operator_precedence with kt_unboxing_cast_type: `!x as java.lang.Boolean`, the cast an unwritten
    `booleanValue()` unboxes under `!`, written `!(x as Boolean)`; with the other ktops2 changes."""
    return kt2_recognise(old, new, 'kt_unboxed_cast')


def kt_unboxing_cast_type(old, new):
    """kt_unboxing_cast_type: `x as java.lang.Boolean` (`as Character`, ...) written `x as Boolean` (`as Char`);
    with the other ktops2 changes."""
    return kt2_recognise(old, new, 'kt_unboxing_cast_type')


def kt_copied_equality(old, new):
    """kt_copied_equality: `a === b` in a default value or an object's supertype call written `a == b`."""
    return kt2_recognise(old, new, 'kt_copied_equality')


def kt_binary_numeric_promotion(old, new):
    """kt_binary_numeric_promotion: `.code`, `.toInt()`, `.toLong()`, `.toFloat()`, `.toDouble()` added to
    operands of `+ - * / %`, comparisons, `==`/`!=` and unary minus (`c.code - 48`, `b.toInt() == 0`,
    `l != i.toLong()`), parentheses aside; with the other ktops2 changes."""
    return kt2_recognise(old, new, 'kt_binary_numeric_promotion')


def kt_operator_line_break(old, new):
    """kt_operator_line_break: the same tokens, a line that started with a binary operator other than `&&` and
    `||` broken after it instead (`a` / `+ b` -> `a +` / `b`)."""
    if old == new or [t for t, _ in kt_prec_tokens(old)] != [t for t, _ in kt_prec_tokens(new)]:
        return False
    starts = [x.split()[0] for x in old if x.split() and x.split()[0] in KT2_BREAK_OPS]
    ends = [x.split()[-1] for x in new if x.split() and x.split()[-1] in KT2_BREAK_OPS]
    lead = [x.split()[0] for x in new if x.split() and x.split()[0] in KT2_BREAK_OPS]
    return bool(starts) and bool(ends) and not lead


RECOGNISERS['kt_negated_equality'] = kt_negated_equality
RECOGNISERS['kt_unboxed_cast'] = kt_unboxed_cast
RECOGNISERS['kt_unboxing_cast_type'] = kt_unboxing_cast_type
RECOGNISERS['kt_copied_equality'] = kt_copied_equality
RECOGNISERS['kt_binary_numeric_promotion'] = kt_binary_numeric_promotion
RECOGNISERS['kt_operator_line_break'] = kt_operator_line_break


_KT2_FILES = {}


def kt2_context_fix(a, b, i1, i2, j1, j2):
    """For a hunk inside a statement or region other hunks change too: the ktops2 fix kt2_recognise gives the
    smallest range of whole statements around it (brackets balanced on both sides, grown over shared lines and
    neighbouring hunks as kt_operator_precedence_context grows it, a few times past the first balanced
    range), or None; a hunk inside a range accepted before is that range's."""
    key = (id(a), id(b))
    if key not in _KT2_FILES or _KT2_FILES[key][0] is not a or _KT2_FILES[key][1] is not b:
        _KT2_FILES.clear()
        ops = difflib.SequenceMatcher(None, a, b, autojunk=False).get_opcodes()
        _KT2_FILES[key] = (a, b, ops, kt_prec_depths(a), kt_prec_depths(b), {}, [])
    _, _, ops, da, db, done, accepted = _KT2_FILES[key]
    if (i1, i2, j1, j2) in done:
        return done[(i1, i2, j1, j2)]
    inside = next((f for x1, x2, y1, y2, f in accepted if x1 <= i1 and i2 <= x2 and y1 <= j1 and j2 <= y2), None)
    if inside:
        return inside
    found, tries = None, 0
    k = next((n for n, op in enumerate(ops) if op[1] <= i1 and i2 <= op[2] and op[3] <= j1 and j2 <= op[4]), None)
    lo = hi = k
    x1, x2, y1, y2 = i1, i2, j1, j2
    while k is not None and tries < 6:
        before_a, after_a = kt_prec_needs(da, x1, x2)
        before_b, after_b = kt_prec_needs(db, y1, y2)
        if not (before_a or after_a or before_b or after_b):
            found = kt2_fix(tuple(a[x1:x2]), tuple(b[y1:y2]), bool(_KT_DELEGATING), bool(_KT2_DELEGATING))
            if found:
                accepted.append((x1, x2, y1, y2, found))
                break
            tries += 1
            before_a = after_a = True
        if before_a or before_b:
            if ops[lo][0] == 'equal' and x1 > ops[lo][1]:
                x1, y1 = x1 - 1, y1 - 1
            elif lo > 0:
                lo -= 1
                x1, y1 = (ops[lo][2] - 1, ops[lo][4] - 1) if ops[lo][0] == 'equal' else (ops[lo][1], ops[lo][3])
            else:
                break
        if after_a or after_b:
            if ops[hi][0] == 'equal' and x2 < ops[hi][2]:
                x2, y2 = x2 + 1, y2 + 1
            elif hi + 1 < len(ops):
                hi += 1
                x2, y2 = (ops[hi][1] + 1, ops[hi][3] + 1) if ops[hi][0] == 'equal' else (ops[hi][2], ops[hi][4])
            else:
                break
    done[(i1, i2, j1, j2)] = found
    return found


def kt2_context_recogniser(fix):
    return lambda a, b, i1, i2, j1, j2, throw_names: kt2_context_fix(a, b, i1, i2, j1, j2) == fix


CONTEXT_RECOGNISERS.update((fix, kt2_context_recogniser(fix)) for fix in KT2_FIXES[:-1])

KT2_BREAK_END = re.compile(r'^(.*\S) (' + '|'.join(re.escape(op) for op in sorted(KT2_BREAK_OPS, key=len, reverse=True))
                           + r')$')
_KT2_UNBROKEN = {}


def kt2_unbroken(text_lines):
    """The lines with kt_operator_line_break undone: a line that ends with a binary operator (other than `&&`
    and `||`, outside comments) gives it to the start of the next line, after its indentation."""
    out = list(text_lines)
    for k in range(len(out) - 1):
        m = KT2_BREAK_END.match(out[k])
        if m and not out[k].lstrip().startswith(('//', '*', '/*')) and out[k + 1].strip():
            nxt = out[k + 1]
            ind = len(nxt) - len(nxt.lstrip())
            out[k], out[k + 1] = m.group(1), nxt[:ind] + m.group(2) + ' ' + nxt[ind:]
    return out


def kt_operator_line_break_context(a, b, i1, i2, j1, j2, throw_names):
    """kt_operator_line_break in or next to (8 lines, as far as ktprint_context looks) a hunk other fixes change
    too: with the line breaks undone in the new file (kt2_unbroken, which keeps its line count), the hunk is
    the old one or another fix's recogniser or context recogniser accepts it."""
    key = (id(b), len(b))
    if key not in _KT2_UNBROKEN or _KT2_UNBROKEN[key][0] is not b:
        _KT2_UNBROKEN.clear()
        _KT2_UNBROKEN[key] = (b, kt2_unbroken(b))
    bu = _KT2_UNBROKEN[key][1]
    if bu[max(0, j1 - 9):j2 + 9] == b[max(0, j1 - 9):j2 + 9]:
        return False
    old, new = a[i1:i2], bu[j1:j2]
    return old == new or any(r(old, new) for fix, r in RECOGNISERS.items() if fix not in KT2_FIXES) or \
        any(r(a, bu, i1, i2, j1, j2, throw_names) for fix, r in CONTEXT_RECOGNISERS.items() if fix not in KT2_FIXES)


CONTEXT_RECOGNISERS['kt_operator_line_break'] = kt_operator_line_break_context

KT_JAVA_FIELD = re.compile(r'^(\s*)((?:(?:open|public|protected|private|static|final|volatile|transient) )*)'
                           r'([\w$.]+(?:<.*>)?(?:\[\])*) ([\w$]+)(?: = (.*))?;$')
KT_FIELD_PROPERTY = re.compile(r'^(\s*)(@JvmField )?((?:public|protected|private|internal) )?(val|var) '
                               r'(`?)([\w$]+)\5: (.+?)(?: = (.*))?$')
KT_PRIMITIVE_TYPES = {'int': 'Int', 'long': 'Long', 'short': 'Short', 'byte': 'Byte', 'char': 'Char',
                      'boolean': 'Boolean', 'float': 'Float', 'double': 'Double'}
KT_FIELD_DEFAULTS = {'Int': '0', 'Short': '0', 'Byte': '0', 'Long': '0L', 'Float': '0.0F', 'Double': '0.0',
                     'Char': "'\\u0000'", 'Boolean': 'false'}


def kt_field_type_matches(java, kotlin):
    """Whether the Kotlin property type `kotlin` is the Kotlin form of the Java field type `java`: a primitive's
    Kotlin name, else a nullable type of the same (simple) name, `Any?` for `Object`, an array type for an array."""
    if java in KT_PRIMITIVE_TYPES:
        return kotlin == KT_PRIMITIVE_TYPES[java]
    if not kotlin.endswith('?'):
        return False
    kotlin = kotlin[:-1]
    if kotlin.startswith('(') and kotlin.endswith(')'):
        kotlin = kotlin[1:-1]
    if java.endswith('[]'):
        return kotlin.startswith('Array<') or kotlin.endswith('Array')
    simple = lambda t: t.split('<', 1)[0].rsplit('.', 1)[-1]
    return simple(kotlin) == ('Any' if simple(java) == 'Object' else simple(java)) or '->' in kotlin


def kt_field_as_property(old, new):
    """Whether the Java field declaration `old` is written as the property `new` (kt_field_without_property)."""
    mo, mn = KT_JAVA_FIELD.match(old), KT_FIELD_PROPERTY.match(new)
    if not mo or not mn or mo.group(1) != mn.group(1) or mo.group(4) != mn.group(6):
        return False
    mods = mo.group(2).split()
    if 'static' in mods or ('private' in mods) != (mn.group(2) is None):
        return False
    vis = next((v for v in ('private', 'protected', 'public') if v in mods), 'internal')
    if (mn.group(3) or 'public ').strip() != vis or (mn.group(4) == 'val') != ('final' in mods):
        return False
    if not kt_field_type_matches(mo.group(3), mn.group(7)):
        return False
    if mo.group(5) is not None:
        return mn.group(8) is not None and kt_code_tokens([mo.group(5)]) == kt_code_tokens([mn.group(8)])
    if mn.group(4) == 'val':
        return mn.group(8) is None
    return mn.group(8) == KT_FIELD_DEFAULTS.get(mn.group(7), 'null')


def kt_field_without_property(old, new):
    """kt_field_without_property: Java field declarations (`open int label;`) became Kotlin properties line for
    line (`@JvmField internal var label: Int = 0`): same indentation and name, `val` for a final field, the Kotlin
    type (nullable for a reference), the visibility (package-private `internal`, `@JvmField` unless private),
    the initializer kept or, for a `var` without one, the type's JVM default."""
    o, n = [x for x in old if x.strip()], [x for x in new if x.strip()]
    return (bool(o) and len(o) == len(n) and len(old) == len(new)
            and all(kt_field_as_property(a, b) for a, b in zip(o, n)))


RECOGNISERS['kt_field_without_property'] = kt_field_without_property

KT_CTOR_VISIBILITY = ('public', 'internal', 'private', 'protected')


def kt_balanced(toks, i):
    """The index after the bracket group that opens at `toks[i]` (`(`), or None."""
    depth = 0
    for k in range(i, len(toks)):
        depth += {'(': 1, ')': -1}.get(toks[k], 0)
        if depth == 0:
            return k + 1
    return None


def kt_ctor_parts(toks, new):
    """(prefix, params, delegation, rest) of a constructor head in tokens: the old text's `[open] fun Name(..) {`
    followed by an optional `super(..)`/`this(..)` statement, the new text's `[vis] constructor(..) [: super(..)] {`;
    None when the tokens do not start that way after the prefix."""
    start = next((i for i, t in enumerate(toks) if t == ('constructor' if new else 'fun')), None)
    if start is None:
        return None
    prefix = toks[:start]
    if new and prefix and prefix[-1] in KT_CTOR_VISIBILITY:
        prefix = prefix[:-1]
    if not new and prefix and prefix[-1] == 'open':
        prefix = prefix[:-1]
    i = start + 1 if new else start + 2
    if i >= len(toks) or toks[i] != '(':
        return None
    end = kt_balanced(toks, i)
    if end is None:
        return None
    params, delegation = toks[i:end], None
    if new and end < len(toks) and toks[end] == ':':
        if end + 2 >= len(toks) or toks[end + 1] not in ('super', 'this') or toks[end + 2] != '(':
            return None
        stop = kt_balanced(toks, end + 2)
        if stop is None:
            return None
        delegation, end = toks[end + 1:stop], stop
    if end >= len(toks) or toks[end] != '{':
        return None
    end += 1
    if not new and end + 1 < len(toks) and toks[end] in ('super', 'this') and toks[end + 1] == '(':
        stop = kt_balanced(toks, end + 1)
        if stop is None:
            return None
        delegation, end = toks[end:stop], stop
    return prefix, params, delegation, toks[end:]


def kt_delegation_resugared(old, new):
    """Whether the delegation call `new` is `old` with a default-arguments constructor's mask and marker
    resugared away (kt_default_args_super_call): `super(a, false, 2, null)` became `super(a)`."""
    if old[:2] != new[:2] or len(old) < 6 or old[-3:-1] != [',', 'null'] or not old[-4].isdigit():
        return False
    args = [t for t in new[2:-1] if t != ',']
    rest = iter(t for t in old[2:-1] if t != ',')
    return all(any(t == x for x in rest) for t in args)


KT_FUN_CONSTRUCTOR = re.compile(r'^\s*(?:open )?fun `?[\w$]+`?\(')
KT_SECONDARY_CONSTRUCTOR = re.compile(r'^\s*(?:(?:public|internal|private|protected) )?constructor\(')


def kt_constructor_without_metadata(old, new):
    """kt_constructor_without_metadata: an anonymous class's initializer `{` became `init {`; a constructor written
    as a function, `[open] fun Cls(..) {` with the superclass or `this` call as its first statement (or none:
    the hidden empty `super()`), became a secondary constructor `[vis] constructor(..) : super(..) {` (or with no
    delegation), parameters and the rest token for token, the call's default-arguments mask and marker possibly
    resugared away; the lines before the constructor equal, or (one level deep) accepted by another fix's
    recogniser (the class header a neighbouring fix changed)."""
    if kt_constructor_as_secondary(old, new):
        return True
    if len(old) == len(new):
        init = [a.strip() == '{' and b == a.replace('{', 'init {', 1) for a, b in zip(old, new)]
        if any(init):
            plain = [a if k else b for a, b, k in zip(old, new, init)]
            return kt_other_recogniser_accepts(old, plain, 'kt_constructor_without_metadata')
    i = next((k for k, x in enumerate(old) if KT_FUN_CONSTRUCTOR.match(x)), None)
    j = next((k for k, x in enumerate(new) if KT_SECONDARY_CONSTRUCTOR.match(x)), None)
    if i is None or j is None or (i == 0 and j == 0):
        return False
    return (kt_constructor_as_secondary(old[i:], new[j:]) and
            (old[:i] == new[:j] or kt_other_recogniser_accepts(old[:i], new[:j], 'kt_constructor_without_metadata')))


def kt_constructor_as_secondary(old, new):
    """The whole of kt_constructor_without_metadata's change, with nothing before the constructor."""
    if len(old) == len(new) and old != new and all(
            a == b or (a.strip() == '{' and b == a.replace('{', 'init {', 1)) for a, b in zip(old, new)):
        return True
    po, pn = kt_ctor_parts(kt_code_tokens(old), False), kt_ctor_parts(kt_code_tokens(new), True)
    if po is None or pn is None or po[0] != pn[0] or po[1] != pn[1] or po[3] != pn[3]:
        return False
    if po[2] is None or pn[2] is None:
        return po[2] is None and pn[2] is None
    return po[2] == pn[2] or kt_delegation_resugared(po[2], pn[2])


RECOGNISERS['kt_constructor_without_metadata'] = kt_constructor_without_metadata

KT_TEMP_DEFINITION = re.compile(r'^(?:val|var) (var\d+): .+? = (.+)$')
KT_NULL_CHECK = re.compile(r'^Intrinsics\.(?:checkNotNullExpressionValue|checkExpressionValueIsNotNull|checkNotNullParameter|'
                           r'checkParameterIsNotNull)\([\w$`]+, "(?:[^"\\]|\\.)*"\)$')
KT_COMPUTED_DELEGATION_HEAD = re.compile(r'^(\s*(?:(?:public|protected|private|internal) )?constructor\(.*\))'
                                         r'(?:  \{| : Intrinsics\.checkParameterIsNotNull\([\w$`]+, "(?:[^"\\]|\\.)*"\) \{)$')


def kt_delegation_call_arguments(old, new):
    """kt_delegation_call_arguments: a secondary constructor written `constructor(..)  {` (or with a Kotlin 1.3
    parameter check taken for its delegation) whose body computes the delegation call's arguments in stack
    variables (`val var10001: T = e`), with kotlinc's null checks, before `this(var10001)`/`super(..)`, became
    `constructor(..) : this(e)`: each variable replaced by its value, the checks gone, the statements after the
    call kept as the body."""
    if len(old) < 3 or not new or old[-1].strip() != '}':
        return False
    m = KT_COMPUTED_DELEGATION_HEAD.match(old[0])
    if not m:
        return False
    temps, body, delegation = {}, [], None
    for line in old[1:-1]:
        s = line.strip()
        if delegation is None:
            d = KT_TEMP_DEFINITION.match(s)
            if d:
                temps[d.group(1)] = d.group(2)
                continue
            if KT_NULL_CHECK.match(s):
                continue
            if s.startswith(('this(', 'super(')):
                delegation = s
                continue
            return False
        body.append(line)
    if delegation is None or not temps:
        return False
    call = []
    for t in kt_code_tokens([delegation]):
        call += kt_code_tokens([temps[t]]) if t in temps else [t]
    tail = ['{'] + kt_code_tokens(body) + ['}'] if body else []
    return kt_code_tokens(new) == kt_code_tokens([m.group(1)]) + [':'] + call + tail


RECOGNISERS['kt_delegation_call_arguments'] = kt_delegation_call_arguments


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
    the old layout with a different line of the new one, splitting the change's tokens across hunks); or
    the run without the recognised hunks at its ends, which another fix may have changed nearby."""
    out, run = [], []
    for hunk in hunks + [('end', 0, 0, 0, 0)]:
        if hunk[0] in RECOGNISERS or hunk[0] in ('other', 'layout'):
            run.append(hunk)
            continue
        if any(k == 'other' for k, *_ in run):
            lo, hi = 0, len(run)
            while run[lo][0] in RECOGNISERS:
                lo += 1
            while run[hi - 1][0] in RECOGNISERS:
                hi -= 1
            for x, y in dict.fromkeys(((0, len(run)), (lo, hi))):
                old, new = a[run[x][1]:run[y - 1][2]], b[run[x][3]:run[y - 1][4]]
                fix = next((f for f, recognises in RECOGNISERS.items() if recognises(old, new)), None)
                if fix:
                    run = run[:x] + [(fix,) + tuple(h[1:]) for h in run[x:y]] + run[y:]
                    break
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


JS_PRIMITIVES = {'byte', 'short', 'char', 'int', 'long', 'float', 'double'}
JS_COMPOUND_OPS = [['>', '>', '>'], ['<', '<'], ['>', '>'], ['+'], ['-'], ['*'], ['/'], ['%'], ['&'], ['|'], ['^']]
JS_COMMUTATIVE = (['+'], ['&'], ['|'], ['^'])


def js_strip_casts(toks):
    """The tokens without parentheses and primitive cast types (`(byte)(x & y)` to `x & y`)."""
    out, i = [], 0
    while i < len(toks):
        if toks[i] == '(' and i + 2 < len(toks) and toks[i + 1] in JS_PRIMITIVES and toks[i + 2] == ')':
            i += 3
            continue
        if toks[i] not in ('(', ')'):
            out.append(toks[i])
        i += 1
    return out


def js_assignment(stmt):
    """(left, op, right) of an assignment statement's tokens, `op` None for a plain `=`."""
    if stmt and stmt[-1] == ';':
        stmt = stmt[:-1]
    depth = 0
    for i, t in enumerate(stmt):
        if t in ('(', '[', '{'):
            depth += 1
        elif t in (')', ']', '}'):
            depth -= 1
        elif t == '=' and depth == 0 and i > 0 and (i + 1 >= len(stmt) or stmt[i + 1] != '='):
            if stmt[i - 1] in ('=', '!'):
                return None
            for op in JS_COMPOUND_OPS:
                if i - len(op) >= 1 and stmt[i - len(op):i] == op:
                    return stmt[:i - len(op)], op, stmt[i + 1:]
            if stmt[i - 1] in ('<', '>'):
                return None
            return stmt[:i], None, stmt[i + 1:]
    return None


def js_expansions(stmt):
    """The readings of an assignment statement as `left = left op right` (casts and parentheses dropped):
    `[(left, op, right)]` for `left op= right`, every split of `left = left op right` or (commutative `op`)
    `left = right op left` for a plain one."""
    parts = js_assignment(stmt)
    if parts is None:
        return []
    left, op, right = parts
    if op is not None:
        return [(left, op, js_strip_casts(right))]
    right, n, out = js_strip_casts(right), len(left), []
    for op in JS_COMPOUND_OPS:
        if right[:n] == left and right[n:n + len(op)] == op:
            out.append((left, op, right[n + len(op):]))
        if op in JS_COMMUTATIVE and right[-n:] == left and right[-n - len(op):-n] == op:
            out.append((left, op, right[:-n - len(op)]))
    return out


def js_statements(toks):
    out, cur = [], []
    for t in toks:
        cur.append(t)
        if t == ';':
            out.append(cur)
            cur = []
    if cur:
        out.append(cur)
    return out


def compound_assignment_shape(old, new):
    """compound_assignment_shape: statements `x = x op e` (through a cast to x's type, or written
    `x = e op x`) became `x op= e`, or `x op= e` became `x = x op e` or `x = e op x`, operands possibly
    parenthesised; every other statement is unchanged."""
    a, b = js_statements(tokens(old)), js_statements(tokens(new))
    if len(a) != len(b):
        a, b = js_inline_temps(a), js_inline_temps(b)
    if len(a) != len(b) or a == b:
        return False
    for x, y in zip(a, b):
        if x == y:
            continue
        px, py = js_assignment(x), js_assignment(y)
        if px is not None and py is not None and (px[1] is None) != (py[1] is None):
            compound, plain = (x, y) if px[1] is not None else (y, x)
            if js_expansions(compound)[0] in js_expansions(plain):
                continue
        if not js_inner_compound(x, y):
            return False
    return True


def js_inline_temps(stmts):
    """The statements with each `[T] varN = e;` whose variable the next statement uses written into it
    (Vineflower's stack variable for a compound assignment's receiver it does not re-sugar)."""
    out = []
    for stmt in stmts:
        prev = out[-1] if out else None
        if prev and prev[-1] == ';':
            parts = js_assignment(prev)
            if parts and parts[1] is None and re.match(r'var\d+$', parts[0][-1]) and parts[0][-1] in stmt \
                    and all(IDENT.match(t) or t in '.<>[],?' for t in parts[0][:-1]):
                name, value = parts[0][-1], parts[2]
                out[-1] = [x for t in stmt for x in (value if t == name else [t])]
                continue
        out.append(stmt)
    return out


def js_inner_compound(x, y):
    """Whether two statements differ only in one assignment inside them (a lambda's body) written
    `l op= e` in one and `l = l op e` in the other, casts and parentheses dropped."""
    a, b = js_strip_casts(x), js_strip_casts(y)
    p = 0
    while p < min(len(a), len(b)) and a[p] == b[p]:
        p += 1
    q = 0
    while q < min(len(a), len(b)) - p and a[-1 - q] == b[-1 - q]:
        q += 1
    ma, mb = a[p:len(a) - q], b[p:len(b) - q]
    for op in JS_COMPOUND_OPS:
        for n in range(1, p + 1):
            left = a[p - n:p]
            if sorted((ma, mb)) == sorted((op + ['='], ['='] + left + op)):
                return True
    return False


def js_new_simple(toks):
    """The tokens with `Outer.this.new Inner(` and `new A.B.Inner(` both written `new Inner(`."""
    out, i = [], 0
    while i < len(toks):
        if i + 4 < len(toks) and toks[i + 1:i + 5] == ['.', 'this', '.', 'new'] and IDENT.match(toks[i]):
            i += 4
            continue
        if toks[i] == 'new':
            j = i + 1
            while j + 2 < len(toks) and IDENT.match(toks[j]) and toks[j + 1] == '.' and IDENT.match(toks[j + 2]):
                j += 2
            out += ['new', toks[j]]
            i = j + 1
            continue
        out.append(toks[i])
        i += 1
    return out


def implicit_outer_instance(old, new):
    """implicit_outer_instance: `Outer.this.new Inner(` became `new Outer.Inner(` (the lines possibly
    reflowed), and nothing else."""
    a, b = tokens(old), tokens(new)
    qualified = sum(1 for i in range(len(a) - 3) if a[i:i + 4] == ['.', 'this', '.', 'new'])
    left = sum(1 for i in range(len(b) - 3) if b[i:i + 4] == ['.', 'this', '.', 'new'])
    return qualified > left and a != b and js_new_simple(a) == js_new_simple(b)


JS_CONTROL = re.compile(r'^\s*(?:\}\s*)?(?:(?:if|else|for|while|do|try|catch|finally|switch|case|return|throw|yield)\b|'
                        r'synchronized\s*\(|default\s*(?::|->)|[\w$]+:)')
JS_TWR_TOKENS = {'try', 'catch', 'Throwable', '(', ')', '{', '}', 'if', 'else', '!', '=', 'null', '.', 'close',
                 'addSuppressed', 'throw', ';', 'return', 'break', 'continue', ':', ',', '?', 'true', 'while', '&',
                 '|'}
JS_LOCAL_DECL = re.compile(r'(?:^\s*(?:final )?|\(|catch \()([\w$.]+(?:<[^;()=]*>)?(?:\[\])*) ([a-z_$][\w$]*)(?= = |;$|\) \{| : )')
JS_TEMP_CAST = re.compile(r'\breturn \(([\w$.<>\[\], ?]+)\)var\d+;')
JS_TEMP_DECL = re.compile(r'^\s*(?:final )?([\w$.<>\[\], ?]+) ([a-z_$][\w$]*)(?: = [\w$.]+)?;$')
JS_TWR_HEAD = re.compile(r'(?:try \(|^\s*)(?:final )?[\w$.<>\[\], ?]+ ([\w$]+) = ')


JS_FIELD_LAMBDA = re.compile(r'^\s*(?:(?:public|protected|private|static|final) )*(?:public|protected|private|static) '
                             r'(?:(?:public|protected|private|static|final) )*[\w$.<>\[\], ?]+ [\w$]+ = .*-> \{$')


def js_method_head(line):
    """Whether a line opening a block (or the first line of a header continued on later lines) starts a
    method or constructor (not a statement, lambda, class, initializer or labeled block)."""
    s = ' ' + line.strip()
    if JS_FIELD_LAMBDA.match(line) or re.match(r'^\s*static \{$', line):
        return True
    return not JS_CONTROL.match(line) and '->' not in s and not line.lstrip().startswith(('}', ')')) \
        and not re.search(r' (?:class|interface|enum|record|new) ', s) and not re.match(r'^\s*(?:static\s*)?\{$', line) \
        and not line.rstrip().endswith('= {') and not line.rstrip().endswith(']{') and '(' in line


def js_header_start(lines, k):
    """The first line of the header the block opened on line `k` belongs to (`k` itself unless the line
    closes parentheses opened above)."""
    if not lines[k].lstrip().startswith(')'):
        return k
    depth = 0
    for h in range(k, -1, -1):
        for t in reversed(_tokens((lines[h],))):
            depth += (t == ')') - (t == '(')
        if depth <= 0:
            return h
    return k


def js_methods(lines):
    """Every method of a file: (header line text, start, end) with [start, end) its lines from its
    header's first line, in order (a method nested in another's anonymous class comes after it)."""
    out, stack = [], []
    for k, line in enumerate(lines):
        for t in _tokens((line,)):
            if t == '{':
                stack.append(k)
            elif t == '}' and stack:
                opener = stack.pop()
                start = js_header_start(lines, opener)
                if js_method_head(lines[start]):
                    out.append((lines[start].strip(), start, k + 1))
    return sorted(out, key=lambda m: (m[1], -m[2]))


def js_canonical(lines):
    """A method's tokens with its compound assignments written out (`x op= e` as `x = x op e`), primitive
    casts and parentheses dropped, and `Outer.this.new Inner(` and `new Outer.Inner(` both `new Inner(`."""
    out, stmts, cur = [], [], []
    for t in js_new_simple(tokens(lines)):
        cur.append(t)
        if t in (';', '{', '}'):
            stmts.append(cur)
            cur = []
    for stmt in stmts + [cur]:
        parts = js_assignment(stmt) if stmt and stmt[-1] == ';' else None
        if parts is not None and parts[1] is not None:
            left, op, right = parts
            stmt = left + ['='] + left + op + right + ([';'] if stmt[-1] == ';' else [])
        out += js_strip_casts(stmt)
    return out


JS_TWR_CACHE = {}


def js_twr_spans(a, b):
    """The [start, end) spans of the methods of `a` and of `b` (paired by name and its occurrence: another
    fix may change a header's types) that js_twr_method accepts."""
    if JS_TWR_CACHE.get('a') is a and JS_TWR_CACHE.get('b') is b:
        return JS_TWR_CACHE['spans']
    def keyed(lines):
        seen, out = collections.Counter(), {}
        for head, start, end in js_methods(lines):
            name = re.search(r'([\w$]+)\(', head)
            name = name.group(1) if name else head
            seen[name] += 1
            out[(name, seen[name])] = (start, end)
        return out
    ma, mb = keyed(a), keyed(b)
    spans = ([], [])
    for key in ma.keys() & mb.keys():
        (sa, ea), (sb, eb) = ma[key], mb[key]
        if a[sa:ea] != b[sb:eb] and js_twr_method(a[sa:ea], b[sb:eb]):
            spans[0].append((sa, ea))
            spans[1].append((sb, eb))
    JS_TWR_CACHE.update(a=a, b=b, spans=spans)
    return spans


def twr_resugar(a, b, i1, i2, j1, j2, throw_names):
    """twr_unresolved_resource, twr_class_version, twr_desugaring_shape: the hunk's lines lie in methods
    where javac's try-with-resources desugaring (`catch (Throwable t) { [if (r != null)] try { r.close();
    } catch (Throwable x) { t.addSuppressed(x); } throw t; }`, the closes at the exits, the result
    temporaries) became `try (R r = ..) {..}`, or back: the method has more `try (` and fewer
    `addSuppressed` (or the reverse), and every token that differs, compound assignments and qualified
    inner creations written out, is the desugaring's (its keywords and punctuation, `varN`, `labelN`, the
    temporaries' types and casts), a local (renamed when a temporary's name is freed), or the `else`,
    `!`, `&&`, `||`, `?`, `:` and `while (true)` of the exits and loops Vineflower then merges."""
    sa, sb = js_twr_spans(a, b)
    def inside(lines, lo, hi, spans):
        return all(lines[k].strip() in ('', '}') or re.match(r'\s*@[\w$.]+(?:\(.*\))?$', lines[k])
                   or any(s <= k < e for s, e in spans) for k in range(lo, hi))
    return bool(sa) and inside(a, i1, i2, sa) and inside(b, j1, j2, sb) and (i1 < i2 or j1 < j2)


def js_resources(text):
    """The resources of the `try (...)` headers of a method's text: the declared names and the variables
    or fields written as a resource."""
    return set(js_resource_list(text))


def js_resource_list(text):
    """js_resources as a list, one entry per resource."""
    out = []
    for m in re.finditer(r'\btry \(', text):
        depth, k, parts, cur = 1, m.end(), [], []
        while k < len(text) and depth:
            ch = text[k]
            depth += (ch in '({[') - (ch in ')}]')
            if depth == 1 and ch == ';':
                parts.append(''.join(cur))
                cur = []
            elif depth:
                cur.append(ch)
            k += 1
        for res in parts + [''.join(cur)]:
            res = res.strip()
            d = re.match(r'(?:final )?[\w$.<>\[\], ?]+? ([\w$]+) = ', res)
            if d:
                out.append(d.group(1))
            elif re.match(r'[\w$.]+$', res):
                out.append(res)
    return out


def js_closed(name, text):
    """Whether a desugared method's text closes resource `name` (a variable or field), directly or
    through the variable javac's desugaring copies it to."""
    if re.search(r'(?<![\w$.])%s\.close\(\)' % re.escape(name), text):
        return True
    return any(re.search(r'(?<![\w$])%s\.close\(\)' % re.escape(m.group(1)), text)
               for m in re.finditer(r'(?<![\w$.])([\w$]+) = %s;' % re.escape(name), text))


def js_calls(lines):
    """The methods a method's text calls and the string literals it holds (as a set)."""
    toks = [t for i, t in enumerate(js_new_simple(tokens(lines)))]
    toks = [t for i, t in enumerate(toks) if not (t in '<>' and (toks[i - 1:i + 1] == ['<', '>'] or toks[i:i + 2] == ['<', '>']))]
    return {t for i, t in enumerate(toks) if t.startswith('"') or (IDENT.match(t) and i + 1 < len(toks)
            and toks[i + 1] == '(' and t not in ('if', 'while', 'for', 'switch', 'catch', 'synchronized', 'try', 'return',
                                         'throw', 'new', 'yield', 'case', 'else', 'super', 'this'))}


def js_twr_method(ma, mb):
    """twr_resugar's test of one method, before (`ma`) and after (`mb`): it has more `try (` and fewer
    `addSuppressed` calls (or the reverse), every resource of a `try (` it gained is closed and its close
    failure `addSuppressed` in the desugared version, and both versions call the same methods and hold the
    same string literals, apart from `close` and `addSuppressed`."""
    ta, tb = '\n'.join(ma), '\n'.join(mb)
    d_res = len(js_resource_list(tb)) - len(js_resource_list(ta))
    d_supp = ta.count('.addSuppressed(') - tb.count('.addSuppressed(')
    if d_res == 0 or (d_res > 0) != (d_supp > 0) or abs(d_supp) < abs(d_res):
        return False
    sugared, desugared = (tb, ta) if d_res > 0 else (ta, tb)
    for name in js_resources(sugared) - js_resources(desugared):
        if not js_closed(name, desugared):
            return False
    return js_calls(ma) - {'close', 'addSuppressed'} == js_calls(mb) - {'close', 'addSuppressed'}

JS_TWR_ACCESS = re.compile(r'^(\s*(?:\} )?try \()([\w$.]+)(\) \{)$')
JS_TWR_DECL = re.compile(r'^(\s*(?:\} )?try \()[\w$.<>\[\], ?]+ [\w$]+ = ([\w$.]+)(\) \{)$')


def twr_field_resource(old, new):
    """twr_desugaring_shape: a resource written as a field (`try (this.f)`), which Java allows only for a
    final field, became the variable javac's desugaring assigns it to (`try (T v = this.f)`)."""
    if len(old) != len(new) or old == new:
        return False
    for o, n in zip(old, new):
        if o == n:
            continue
        mo, mn = JS_TWR_ACCESS.match(o), JS_TWR_DECL.match(n)
        if not mo or not mn or (mo.group(1), mo.group(2), mo.group(3)) != mn.groups():
            return False
    return True


RECOGNISERS['compound_assignment_shape'] = compound_assignment_shape
RECOGNISERS['implicit_outer_instance'] = implicit_outer_instance
RECOGNISERS['twr_desugaring_shape'] = twr_field_resource
CONTEXT_RECOGNISERS['twr_resugar'] = twr_resugar


def name_chain_end(toks, i):
    """The index after `W(.W)*` starting at toks[i], else None."""
    if i >= len(toks) or not WORD.fullmatch(toks[i]):
        return None
    j = i + 1
    while j + 1 < len(toks) and toks[j] == '.' and WORD.fullmatch(toks[j + 1]):
        j += 2
    return j


def without_length_arrays(toks):
    """The tokens with each last argument `new T[n]..` of a call (and the `,` before it) dropped, and how
    many were dropped."""
    out, i, n = [], 0, 0
    while i < len(toks):
        if toks[i] == 'new' and out and out[-1] in ('(', ','):
            j = name_chain_end(toks, i + 1)
            if j is not None and j + 1 < len(toks) and toks[j] == '[' and toks[j + 1] != ']':
                while j is not None and j < len(toks) and toks[j] == '[':
                    j = block_end(toks, j)
                    j = None if j is None else j + 1
                if j is not None and j < len(toks) and toks[j] == ')':
                    if out[-1] == ',':
                        out.pop()
                    i, n = j, n + 1
                    continue
        out.append(toks[i])
        i += 1
    return out, n


def vararg_array_length(old, new):
    """vararg_array_length: token for token, a call's last argument `new T[n]` (an array created by its
    length) was written as no argument at all; the lines possibly reflowed."""
    a, b = tokens(old), tokens(new)
    stripped, n = without_length_arrays(b)
    return n > 0 and stripped == a


def without_null_checks(toks):
    """The tokens with each statement `Objects.requireNonNull(x);` or `x.getClass();` (x a name, a field
    or `this`) dropped, and how many were dropped."""
    out, i, n = [], 0, 0
    while i < len(toks):
        if not out or out[-1] in (';', '{', '}'):
            j = name_chain_end(toks, i)
            if j is not None:
                names = toks[i:j]
                if [t for t in names if t != '.'][-1:] == ['requireNonNull'] and j < len(toks) and toks[j] == '(' \
                        and ''.join(names) in ('Objects.requireNonNull', 'java.util.Objects.requireNonNull'):
                    k = name_chain_end(toks, j + 1)
                    if k is not None and toks[k:k + 2] == [')', ';']:
                        i, n = k + 2, n + 1
                        continue
                if names[-2:] == ['.', 'getClass'] and toks[j:j + 3] == ['(', ')', ';']:
                    i, n = j + 3, n + 1
                    continue
        out.append(toks[i])
        i += 1
    return out, n


def synthetic_null_check_shape(old, new):
    """synthetic_null_check_shape: statements `Objects.requireNonNull(x);` / `x.getClass();` inserted, nothing
    else changed."""
    stripped, n = without_null_checks(tokens(new))
    return n > 0 and stripped == tokens(old)


def var_merge_declared_type(old, new):
    """var_merge_declared_type: a variable the fix keeps apart from an earlier one of its slot (its own
    name and declaration, `Box<Integer> boxx = ..` for `box = ..`) is renamed back onto it, and the casts
    the merge needed are dropped: var_merge_narrowing's shape."""
    return var_merge_narrowing(old, new)


def without_this_stores(toks):
    """The tokens with each statement `this.f = E;` dropped."""
    out, i = [], 0
    while i < len(toks):
        if (not out or out[-1] in (';', '{', '}')) and toks[i:i + 2] == ['this', '.'] and i + 3 < len(toks) \
                and WORD.fullmatch(toks[i + 2]) and toks[i + 3] == '=':
            depth, j = 0, i + 4
            while j < len(toks) and not (toks[j] == ';' and depth == 0):
                depth += {'(': 1, '[': 1, '{': 1, ')': -1, ']': -1, '}': -1}.get(toks[j], 0)
                j += 1
            i = j + 1
            continue
        out.append(toks[i])
        i += 1
    return out


def compact_record_constructor(old, new):
    """compact_record_constructor: a record's canonical constructor written whole: inserted where it was
    hidden (a body of `this.f = E;` stores only), or a compact header `R {` given its parameters `R(..) {` and
    the `this.f = E;` stores the compact form had removed put back."""
    a, b = tokens(old), tokens(new)
    if not any(t == 'this' for t in b) or a == b:
        return False
    rest = without_this_stores(b)
    if not a:
        mods = 0
        while mods < len(rest) and rest[mods] in ('public', 'protected', 'private'):
            mods += 1
        if mods < len(rest) and WORD.fullmatch(rest[mods]) and rest[mods + 1:mods + 2] == ['(']:
            end = block_end(rest, mods + 1)
            return end is not None and rest[end + 1:] == ['{', '}']
        return False
    for i in range(len(rest) - 1):
        if WORD.fullmatch(rest[i]) and rest[i + 1] == '(' and rest[i] in a and (i == 0 or rest[i - 1] in (
                'public', 'protected', 'private', '}', ';')):
            end = block_end(rest, i + 1)
            if end is not None and end + 1 < len(rest) and rest[end + 1] == '{' \
                    and rest[:i + 1] + rest[end + 1:] == without_this_stores(a):
                return True
    return False


def raw_cast_types(toks):
    """The tokens with the type arguments of every cast's class type dropped (`(A<B>) x` to `(A) x`), and
    how many were dropped."""
    out, i, n = [], 0, 0
    while i < len(toks):
        j = cast_at(toks, i)
        if j is not None and '<' in toks[i + 1:j] and toks[j - 1] == '>' and '&' not in toks[i + 1:j]:
            k = toks.index('<', i + 1)
            out += toks[i:k] + [')']
            i, n = j + 1, n + 1
            continue
        out.append(toks[i])
        i += 1
    return out, n


def disjoint_generic_cast(old, new):
    """disjoint_generic_cast: token for token, casts to a parameterized class `(A<B>) x` became casts to the
    raw class `(A) x`; the lines possibly reflowed."""
    a, b = tokens(old), tokens(new)
    stripped, n = raw_cast_types(a)
    return n > 0 and a != b and stripped == b


def without_thrown_type_variables(toks):
    """The tokens with `throw (T)x` written `throw x` (T a bare name) and `new C<RuntimeException, ..>(`
    written `new C(`, and how many were changed."""
    out, i, n = [], 0, 0
    while i < len(toks):
        if out and out[-1] == 'throw' and toks[i] == '(' and i + 2 < len(toks) and WORD.fullmatch(toks[i + 1]) \
                and toks[i + 2] == ')':
            i, n = i + 3, n + 1
            continue
        if out and toks[i] == '<' and len(out) >= 2 and WORD.fullmatch(out[-1]) and 'new' in out[-6:]:
            j = i + 1
            while j + 1 < len(toks) and toks[j] == 'RuntimeException' and toks[j + 1] in (',', '>'):
                j += 2
                if toks[j - 1] == '>':
                    break
            if j > i + 1 and toks[j - 1] == '>' and j < len(toks) and toks[j] == '(':
                i, n = j, n + 1
                continue
        out.append(toks[i])
        i += 1
    return out, n


def thrown_type_variable(old, new):
    """thrown_type_variable: token for token, `throw x` became `throw (T)x` and a raw receiver `new C()` became
    `new C<RuntimeException>()`."""
    stripped, n = without_thrown_type_variables(tokens(new))
    return n > 0 and stripped == tokens(old)


RECOGNISERS['vararg_array_length'] = vararg_array_length
RECOGNISERS['synthetic_null_check_shape'] = synthetic_null_check_shape
RECOGNISERS['var_merge_declared_type'] = var_merge_declared_type
RECOGNISERS['compact_record_constructor'] = compact_record_constructor
RECOGNISERS['disjoint_generic_cast'] = disjoint_generic_cast
RECOGNISERS['thrown_type_variable'] = thrown_type_variable


def statement_end(toks, i):
    """The index of the `;` that ends the statement starting at toks[i] (at bracket depth 0), else None."""
    depth = 0
    for j in range(i, len(toks)):
        depth += {'(': 1, '[': 1, '{': 1, ')': -1, ']': -1, '}': -1}.get(toks[j], 0)
        if depth < 0:
            return None
        if toks[j] == ';' and depth == 0:
            return j
    return None


def without_outer_null_checks(toks):
    """The tokens with each statement `Objects.requireNonNull(E);`, each stack-variable declaration
    `T varN = E;` and each initializer block `{ }` left empty by them dropped, and how many were dropped."""
    out, i, n = [], 0, 0
    while i < len(toks):
        if not out or out[-1] in (';', '{', '}'):
            if toks[i:i + 4] == ['Objects', '.', 'requireNonNull', '('] or \
                    toks[i:i + 6] == ['java', '.', 'util', '.', 'Objects', '.'] and toks[i + 6:i + 8] == ['requireNonNull', '(']:
                j = statement_end(toks, i)
                if j is not None:
                    i, n = j + 1, n + 1
                    continue
            j = name_chain_end(toks, i)
            if j is not None and j + 2 < len(toks) and re.fullmatch(r'var\d+', toks[j]) and toks[j + 1] == '=':
                k = statement_end(toks, j)
                if k is not None:
                    i, n = k + 1, n + 1
                    continue
        if toks[i] == '}' and out and out[-1] == '{' and (len(out) < 2 or out[-2] in (';', '{', '}')):
            out.pop()
            i += 1
            continue
        out.append(toks[i])
        i += 1
    return out, n


def unqualified_super_calls(toks):
    """The tokens with each constructor call `Q.super(` written `super(` and each anonymous class creation
    `Q.new S(A) {` written `new S(Q, A) {`, the `S` of the second form standing for any `X.S`; how many were
    rewritten."""
    out, i, n = [], 0, 0
    while i < len(toks):
        if toks[i] == '.' and toks[i + 1:i + 3] == ['super', '('] or toks[i] == '.' and toks[i + 1:i + 2] == ['new']:
            s = len(out)
            depth = 0
            while s > 0:
                t = out[s - 1]
                if t in (')', ']'):
                    depth += 1
                elif t in ('(', '['):
                    if depth == 0:
                        break
                    depth -= 1
                elif depth == 0 and t in (';', '{', '}', '=', ',', 'return'):
                    break
                s -= 1
            q = out[s:]
            if q and toks[i + 1] == 'super':
                del out[s:]
                i, n = i + 1, n + 1
                continue
            j = name_chain_end(toks, i + 2) if q else None
            if j is not None and toks[j:j + 1] == ['(']:
                del out[s:]
                out += ['new', '\0', toks[j - 1], '('] + q + ([] if toks[j + 1] == ')' else [','])
                i, n = j + 1, n + 1
                continue
        out.append(toks[i])
        i += 1
    return out, n


def qualified_super_call(old, new):
    """qualified_super_call: `super(` became `Q.super(` (an anonymous class `new X.S(Q, A) {` became
    `Q.new S(A) {`), and javac's null checks `Objects.requireNonNull(Q);` before them (with the stack copies
    `T varN = Q;` and the initializer blocks holding them) were dropped; nothing else changed."""
    a, _ = without_outer_null_checks(tokens(old))
    b, n = unqualified_super_calls(tokens(new))
    if n > 0 and simple_new_names(a) == simple_new_names(b):
        return True
    t = tokens(new)
    return a != t and (without_first_super_args(a) == t or n > 0 and super_qualifiers_as_arguments(t) == a)


def super_qualifiers_as_arguments(toks):
    """The tokens with each `Q.super(A)` written `super(Q, A)`."""
    out, i = [], 0
    while i < len(toks):
        if toks[i] == '.' and toks[i + 1:i + 3] == ['super', '(']:
            s, depth = len(out), 0
            while s > 0:
                t = out[s - 1]
                if t in (')', ']'):
                    depth += 1
                elif t in ('(', '['):
                    if depth == 0:
                        break
                    depth -= 1
                elif depth == 0 and t in (';', '{', '}'):
                    break
                s -= 1
            q = out[s:]
            del out[s:]
            out += ['super', '('] + q + ([] if toks[i + 3:i + 4] == [')'] else [','])
            i += 3
            continue
        out.append(toks[i])
        i += 1
    return out


def without_first_super_args(toks):
    """The tokens with the first argument of each `super(A, ..)` (the outer instance Vineflower writes for a
    superclass outside the input) dropped."""
    out, i = [], 0
    while i < len(toks):
        if toks[i:i + 2] == ['super', '('] and (not out or out[-1] in (';', '{', '}', '.')):
            depth, j = 0, i + 2
            while j < len(toks) and not (depth == 0 and toks[j] in (',', ')')):
                depth += {'(': 1, '[': 1, '{': 1, ')': -1, ']': -1, '}': -1}.get(toks[j], 0)
                j += 1
            if j < len(toks) and toks[j] == ',':
                out += ['super', '(']
                i = j + 1
                continue
        out.append(toks[i])
        i += 1
    return out


def simple_new_names(toks):
    """The tokens with each `new X.S(` written `new \\0 S(` (and `new S(` as is)."""
    out, i = [], 0
    while i < len(toks):
        if toks[i] == 'new' and toks[i + 1:i + 2] != ['\0']:
            j = name_chain_end(toks, i + 1)
            if j is not None and toks[j:j + 1] == ['('] and j - i > 2:
                out += ['new', '\0', toks[j - 1]]
                i = j
                continue
        out.append(toks[i])
        i += 1
    return out


RECOGNISERS['qualified_super_call'] = qualified_super_call



def without_kt_conversions(lines):
    """The lines with Kotlin's numeric conversions `.toChar()`, `.code` (what a narrowing `(char)` cast and
    its widening back print as) dropped."""
    return [re.sub(r'\.code\b', '', re.sub(r'\.toChar\(\)', '', x)) for x in lines]


def var_merge_declared_types(old, new):
    """var_merge_declared_type: var_merge_narrowing's shape (a variable kept apart renamed back onto the one
    it was merged into, the casts the merge needed dropped), the casts written as Kotlin's conversions too."""
    return var_merge_narrowing(old, new) or (old != new and var_merge_narrowing(without_kt_conversions(old), new))


RECOGNISERS['var_merge_declared_type'] = var_merge_declared_types



def names_qualified(old, new):
    """The tokens of `new` are those of `old` with some simple class names `N` written as dotted names
    `a.b.N`, and how many; None where they are not."""
    a, b = tokens(old), tokens(new)
    i = j = n = 0
    while i < len(a) and j < len(b):
        if a[i] == b[j] and not (j + 1 < len(b) and b[j + 1] == '.' and WORD.fullmatch(a[i])
                                 and (i + 1 >= len(a) or a[i + 1] != '.')):
            i, j = i + 1, j + 1
            continue
        k = name_chain_end(b, j)
        if k is None or k - j < 3 or b[k - 1] != a[i] or (j > 0 and b[j - 1] == '.'):
            return None
        i, j, n = i + 1, k, n + 1
    return n if i == len(a) and j == len(b) else None


def external_inherited_member_type(old, new):
    """external_inherited_member_type: imports dropped and simple class names written qualified
    (`Bucket` to `pkglib.Filters.Bucket`, `Reader` to `java.io.Reader`); nothing else changed."""
    a = [x for x in old if not is_import(x)]
    b = [x for x in new if not is_import(x)]
    if not a and not b:
        return False
    n = names_qualified(a, b)
    return bool(n)


RECOGNISERS['external_inherited_member_type'] = external_inherited_member_type



def without_call_type_arguments(toks):
    """The tokens with the explicit type arguments of each call `.<A>m(` dropped, and how many."""
    out, i, n = [], 0, 0
    while i < len(toks):
        if toks[i] == '<' and out and out[-1] == '.':
            depth, j = 0, i
            while j < len(toks):
                depth += {'<': 1, '>': -1}.get(toks[j], 0)
                if depth == 0:
                    break
                j += 1
            if j + 2 < len(toks) and WORD.fullmatch(toks[j + 1]) and toks[j + 2] == '(':
                i, n = j + 1, n + 1
                continue
        out.append(toks[i])
        i += 1
    return out, n


def lambda_return_type_argument(old, new):
    """lambda_return_type_argument: token for token, a call's explicit type arguments `.<A>m(` dropped."""
    a, b = tokens(old), tokens(new)
    stripped, n = without_call_type_arguments(a)
    return n > 0 and stripped == b


RECOGNISERS['lambda_return_type_argument'] = lambda_return_type_argument



def without_outer_type_arguments(toks):
    """The tokens with the type arguments of each qualifier class of an annotated type (`Outer<V>.@A Inner`)
    dropped, and how many."""
    out, i, n = [], 0, 0
    while i < len(toks):
        if toks[i] == '<' and out and WORD.fullmatch(out[-1]):
            depth, j = 0, i
            while j < len(toks):
                depth += {'<': 1, '>': -1}.get(toks[j], 0)
                if depth == 0:
                    break
                j += 1
            if toks[j + 1:j + 3] == ['.', '@']:
                i, n = j + 1, n + 1
                continue
        out.append(toks[i])
        i += 1
    return out, n


def type_annotation_outer_arguments(old, new):
    """type_annotation_outer_arguments: token for token, `Outer.@A Inner` became `Outer<V>.@A Inner`."""
    stripped, n = without_outer_type_arguments(tokens(new))
    return n > 0 and stripped == tokens(old)


RECOGNISERS['type_annotation_outer_arguments'] = type_annotation_outer_arguments


LAYOUT_SKIP = {('{',), (';',), ('return', ';'), ('continue', ';'), ('break', ';'), ('try', '{')}
LAYOUT_NOT_TYPES = {'return', 'throw', 'new', 'else', 'case', 'break', 'continue', 'yield', 'assert', 'do',
                    'try', 'synchronized', 'if', 'while', 'for', 'switch', 'catch', 'finally', 'default'}
LAYOUT_COMPARE = {'==': '=', '!=': '=', '<': '<', '>=': '<', '>': '>', '<=': '>'}
LAYOUT_TYPE_TOKENS = {'?', 'extends', 'super', ',', '.', '[', ']', '<', '>', '&'}
LAYOUT_OPEN, LAYOUT_CLOSE = ('(', '['), (')', ']')


def layout_join(toks):
    """Operators the tokenizer splits (`&&`, `||`, `==`, `!=`, `<=`, `>=`, `--`, `++`, `->`, `::`) as one
    token."""
    out = []
    for t in toks:
        if out and (out[-1], t) in (('&', '&'), ('|', '|'), ('=', '='), ('!', '='), ('<', '='), ('>', '='),
                                    ('-', '-'), ('+', '+'), ('-', '>'), (':', ':')):
            out[-1] += t
        else:
            out.append(t)
    return out


def layout_close(toks, i):
    """The index of the bracket closing the one at toks[i]."""
    depth = 0
    for j in range(i, len(toks)):
        if toks[j] in LAYOUT_OPEN or toks[j] == '{':
            depth += 1
        elif toks[j] in LAYOUT_CLOSE or toks[j] == '}':
            depth -= 1
            if depth == 0:
                return j
    return None


LAYOUT_BLOCK_STATEMENTS = {'if', 'else', 'for', 'while', 'do', 'switch', 'synchronized', 'try', 'catch',
                           'finally', 'static', '}'}


def layout_embedded(unit, depth):
    """Whether a `{` after `unit` opens a block inside an expression (a lambda body, an anonymous class,
    an array initializer) rather than a statement's."""
    if depth > 0:
        return True
    unit = layout_unlabel(unit)
    if not unit or unit[0] in LAYOUT_BLOCK_STATEMENTS:
        return False
    top, d = [], 0
    for t in unit:
        if t in LAYOUT_OPEN:
            d += 1
        elif t in LAYOUT_CLOSE:
            d -= 1
        elif d == 0:
            top.append(t)
    return unit[0] in ('return', 'throw', 'yield') or any(t in ('->', '=', 'new', '?', ':') for t in top)


def layout_unlabel(unit):
    """unit without the `case x:`, `default:` and `label:` in front."""
    while True:
        if len(unit) > 1 and unit[0] == 'default' and unit[1] == ':':
            unit = unit[2:]
        elif unit and unit[0] == 'case' and ':' in unit:
            unit = unit[list(unit).index(':') + 1:]
        elif len(unit) > 1 and IDENT.match(unit[0]) and unit[1] == ':':
            unit = unit[2:]
        else:
            return unit


def layout_lambda_blocks(toks):
    """toks with every expression lambda, `-> e`, written as the block `-> { return e ; }` (a body that
    returns twice is a block)."""
    out, i = [], 0
    while i < len(toks):
        if toks[i] == '->' and i + 1 < len(toks) and toks[i + 1] != '{':
            depth, j, pending = 0, i + 1, 0
            while j < len(toks):
                t = toks[j]
                if t in LAYOUT_OPEN or t == '{':
                    depth += 1
                elif t in LAYOUT_CLOSE or t == '}':
                    if depth == 0:
                        break
                    depth -= 1
                elif depth == 0 and t in (',', ';'):
                    break
                elif depth == 0 and t == '?' and not layout_wildcard(toks, j):
                    pending += 1
                elif depth == 0 and t == ':':
                    if pending == 0:
                        break
                    pending -= 1
                j += 1
            body = toks[i + 1:j]
            if body:
                out += ['->', '{', 'return'] + layout_lambda_blocks(body) + [';', '}']
                i = j
                continue
        out.append(toks[i])
        i += 1
    return out


def layout_flatten(toks, blocks):
    """toks with every block inside an expression replaced by `{}`; the blocks' own tokens go to `blocks`."""
    out, unit, depth, i = [], [], 0, 0
    while i < len(toks):
        t = toks[i]
        if t == '{' and layout_embedded(unit, depth):
            end = layout_close(toks, i)
            if end is None:
                end = len(toks)
            blocks.append(layout_flatten(toks[i + 1:end], blocks))
            out.append('{}')
            unit.append('{}')
            i = end + 1
            continue
        out.append(t)
        unit.append(t)
        if t in LAYOUT_OPEN:
            depth += 1
        elif t in LAYOUT_CLOSE:
            depth = max(depth - 1, 0)
        elif depth == 0 and t in ('{', '}', ';'):
            unit = []
        i += 1
    return out


def layout_units(toks):
    """The statement units of a flattened token stream: runs ending at `{`, `}` or a `;` outside brackets."""
    units, cur, depth = [], [], 0
    for t in toks:
        cur.append(t)
        if t in LAYOUT_OPEN:
            depth += 1
        elif t in LAYOUT_CLOSE:
            depth = max(depth - 1, 0)
        elif depth == 0 and t in ('{', '}', ';'):
            units.append(tuple(cur))
            cur = []
    if cur:
        units.append(tuple(cur))
    return units


def layout_split(toks, ops):
    """toks split at the top-level operators in ops; None if there are none."""
    parts, cur, depth = [], [], 0
    for t in toks:
        if t in LAYOUT_OPEN:
            depth += 1
        elif t in LAYOUT_CLOSE:
            depth -= 1
        if depth == 0 and t in ops:
            parts.append(cur)
            cur = []
        else:
            cur.append(t)
    parts.append(cur)
    return parts if len(parts) > 1 else None


def layout_unparen(toks):
    """toks without enclosing parentheses."""
    while len(toks) > 1 and toks[0] == '(' and layout_close(toks, 0) == len(toks) - 1:
        toks = toks[1:-1]
    return toks


def layout_uncast(toks):
    """toks without enclosing parentheses and the casts in front: `(T)x` and `(T)(x)` become `x`."""
    toks = layout_unparen(list(toks))
    while toks and toks[0] == '(':
        end = layout_close(toks, 0)
        inner = toks[1:end] if end else None
        if (not inner or end == len(toks) - 1
                or not all(IDENT.match(t) or t in LAYOUT_TYPE_TOKENS for t in inner)
                or not (IDENT.match(toks[end + 1]) or toks[end + 1] == '(' or toks[end + 1][0] in '"\'0123456789')):
            return toks
        toks = layout_unparen(toks[end + 1:])
    return toks


def layout_wildcard(toks, i):
    """Whether the `?` at toks[i] is a type argument's wildcard."""
    return i > 0 and toks[i - 1] in ('<', ',') and i + 1 < len(toks) and toks[i + 1] in ('>', ',', 'extends', 'super')


def layout_ternary(toks):
    """(condition, then, else) of a top-level `c ? a : b`, else None."""
    depth, q, pend = 0, None, 0
    for i, t in enumerate(toks):
        if t in LAYOUT_OPEN:
            depth += 1
        elif t in LAYOUT_CLOSE:
            depth -= 1
        elif depth == 0 and t == '?' and not layout_wildcard(toks, i):
            if q is None:
                q = i
            else:
                pend += 1
        elif depth == 0 and t == ':' and q is not None:
            if pend == 0:
                return toks[:q], toks[q + 1:i], toks[i + 1:]
            pend -= 1
    return None


def layout_atoms(toks):
    """The leaves of a boolean expression, each up to negation; a ternary's condition and arms are leaves."""
    toks = layout_uncast(list(toks))
    while toks and toks[0] == '!':
        toks = layout_uncast(toks[1:])
    tern = layout_ternary(toks)
    if tern:
        return [a for p in tern for a in layout_atoms(p)]
    parts = layout_split(toks, ('&&', '||'))
    if parts:
        return [a for p in parts for a in layout_atoms(p)]
    for op in ('==', '!=', '<', '>=', '>', '<='):
        found = layout_split(toks, (op,))
        if found and len(found) == 2:
            return [layout_text(found[0]) + ' ' + LAYOUT_COMPARE[op] + ' ' + layout_text(found[1])]
    return [layout_text(toks)]


def layout_text(toks):
    """toks as text with every ternary inside written up to its orientation."""
    toks = list(toks)
    parts = layout_split(toks, ('=',))
    if parts and len(parts) == 2:
        return layout_text(parts[0]) + ' = ' + layout_text(parts[1])
    if toks and toks[-1] == ';':
        return layout_text(toks[:-1]) + ' ;'
    tern = layout_ternary(toks)
    if tern:
        arms = sorted(layout_text(layout_uncast(x)) for x in tern[1:])
        return '?:( ' + ' | '.join(sorted(layout_atoms(tern[0]))) + ' : ' + ' | '.join(arms) + ' )'
    out, i = [], 0
    while i < len(toks):
        if toks[i] in LAYOUT_OPEN:
            end = layout_close(toks, i)
            if end is not None:
                inner = toks[i + 1:end]
                args = layout_split(inner, (',',)) or [inner]
                out.append(toks[i] + ' ' + ' , '.join(layout_text(a) for a in args) + ' ' + toks[end])
                i = end + 1
                continue
        out.append(layout_token(toks[i]))
        i += 1
    return ' '.join(out)


def layout_token(t):
    """A token with the numbers of `labelN` and `varN` names dropped (the order structuring numbers them
    in) and a char literal as its code (a comparison's constant is typed by where it ends up)."""
    if re.fullmatch(r'(label|var)\d+', t):
        return t.rstrip('0123456789') + '#'
    if len(t) > 2 and t[0] == t[-1] == "'":
        try:
            value = ast.literal_eval(t)
        except (ValueError, SyntaxError):
            return t
        if len(value) == 1:
            return str(ord(value))
    return t


def layout_declared(unit):
    """The assignment of a declaration `T v = e ;` (`T` possibly qualified, `a.B<..>[]`), `()` for a bare
    `T v ;`, None for anything else."""
    i = 1 if unit and unit[0] == 'final' else 0
    if i >= len(unit) or not IDENT.match(unit[i]) or unit[i] in LAYOUT_NOT_TYPES:
        return None
    j, depth = i + 1, 0
    while j < len(unit):
        t = unit[j]
        if t == '<':
            depth += 1
        elif t == '>':
            depth -= 1
        elif depth == 0 and t == '.' and j + 1 < len(unit) and IDENT.match(unit[j + 1]):
            j += 1
        elif depth == 0 and t in ('.', '[', ']'):
            pass
        elif depth == 0:
            break
        elif not (IDENT.match(t) or t in LAYOUT_TYPE_TOKENS):
            return None
        j += 1
    if depth != 0 or j + 1 >= len(unit) or not IDENT.match(unit[j]):
        return None
    if unit[j + 1] == ';' and j + 2 == len(unit):
        return ()
    return unit[j:] if unit[j + 1] == '=' else None


def layout_loop_header(u):
    """A loop header unit as (statement units, condition or None): `while (c) {` and `} while (c);` give
    their condition (none for `true`), `do {` nothing, and a `for (init; c; update) {` its init and update
    as statements and its condition (a for-each header is a statement of its own); None for anything else."""
    if u == ('do', '{'):
        return [], None
    if len(u) < 4 or u[1] != '(' or u[0] not in ('while', 'for') or u[-1] not in ('{', ';'):
        return None
    close = layout_close(u, 1)
    if close != len(u) - 2 or (u[0] == 'for' and u[-1] != '{'):
        return None
    inner = list(u[2:close])
    if u[0] == 'while':
        return [], (None if inner == ['true'] else inner)
    pieces = layout_split(inner, (';',))
    if pieces is None or len(pieces) != 3:
        return None
    init, cond, update = pieces
    units = [tuple(x) + (';',) for x in layout_split(init, (',',)) or [init] if x]
    units += [tuple(x) + (';',) for x in layout_split(update, (',',)) or [update] if x]
    return units, (cond or None)


def layout_summary(lines):
    """(statements, condition leaves, returned values) of some lines, without braces, `else`, `try {` and
    bare exits."""
    stmts, conds, rets = collections.Counter(), collections.Counter(), []
    blocks = []
    flat = layout_flatten(layout_lambda_blocks(layout_join(tokens(lines))), blocks)
    for toks in [flat] + blocks:
        for unit in layout_units(toks):
            u = unit
            while u and u[0] in ('}', 'else'):
                u = u[1:]
            u = layout_unlabel(u)
            if not u or u in LAYOUT_SKIP:
                continue
            if u[0] == 'if' and len(u) > 3 and u[1] == '(' and u[-1] == '{' and layout_close(u, 1) == len(u) - 2:
                conds.update(layout_atoms(u[2:-2]))
                continue
            loop = layout_loop_header(u)
            if loop is not None:
                parts, cond = loop
                if cond is not None:
                    conds.update(layout_atoms(cond))
                for part in parts:
                    decl = layout_declared(part)
                    if decl != ():
                        stmts[layout_text(decl if decl is not None else part)] += 1
                continue
            if u[0] == 'return':
                rets.append(list(u[1:-1] if u[-1] == ';' else u[1:]))
                continue
            decl = layout_declared(u)
            if decl == ():
                continue
            stmts[layout_text(decl if decl is not None else u)] += 1
    return stmts, conds, rets


def layout_returns(rets, conds, boolean):
    """The returned values with ternaries split into their condition's leaves and both values; with
    `boolean` (a side returns `true` or `false`) every value is split into its leaves."""
    out = collections.Counter()
    work = list(rets)
    while work:
        r = layout_uncast(work.pop())
        tern = layout_ternary(r)
        if tern:
            conds.update(layout_atoms(tern[0]))
            work += [tern[1], tern[2]]
        elif boolean:
            if r not in (['true'], ['false']):
                conds.update(layout_atoms(r))
        else:
            out[layout_text(r)] += 1
    return out


def javac_layout(old, new):
    """jvm_javac_layout (the option, on by default): the same statements with ifs and exits laid out anew: the statements of both
    sides are the same (a declaration may be split from or joined to its first assignment, a ternary's
    arms swapped with its condition negated), the conditions of their ifs have the same leaves up to
    negation (so an if may be negated, its branches swapped, and `a && b` written `!a || !b`),
    `return c ? a : b;` may be an if of `c` returning `a` or `b` (a boolean return, with `true`/`false`
    returns on either side, the if of its leaves), a return may be written once more (an exit javac
    wrote where Vineflower shared one), and bare `return;`, `continue;`, `break;` and `try {` lines and
    `else` may come or go. A loop's condition counts as an if's (`while (a && b)` and `while (a)` with
    `if (!b) break;`, `do { .. } while (c);` and `while (true)` with `if (!c) break;` are the same), and a
    `for` loop's init and update as statements (`for (i = 0; c; i++)` against `i = 0; while (c) { ..; i++; }`)."""
    if tokens(old) == tokens(new):
        return False
    s1, c1, r1 = layout_summary(old)
    s2, c2, r2 = layout_summary(new)
    if s1 != s2:
        return False
    boolean = any(r in (['true'], ['false']) for r in r1 + r2)
    return set(layout_returns(r1, c1, boolean)) == set(layout_returns(r2, c2, boolean)) and c1 == c2


RECOGNISERS['jvm_javac_layout'] = javac_layout


# --- jvm_source_locals, a kuna-form option on by default: the new form keeps the locals the class file records.
# The recognisers fold them back into Vineflower's form (a kept local into its one use, element stores into an
# array initializer, a named iterable or iterator into a for-each) and compare spellings both forms share: casts,
# default values, a ternary return as if/return, no `else` after an exit, package and outer-class qualifiers, the
# clash renamer's `x` suffix, pattern bindings, Kotlin's line ends; declarations compare wherever they stand.

_SL_TOKEN = re.compile(r'"(?:[^"\\\n]|\\.)*"|\'(?:[^\'\\\n]|\\.)*\'|[\w$]+|\S')
_SL_IDENT = re.compile(r'[A-Za-z_$][\w$]*$')
_SL_PRIMS = {'int', 'long', 'short', 'byte', 'char', 'boolean', 'float', 'double'}
_SL_COMPOUND = {'+', '-', '*', '/', '%', '&', '|', '^', '<<', '>>', '>>>'}
_SL_DEFAULTS = {'0', '0L', '0.0F', '0.0', 'false', 'null', "'\\u0000'", '0.0D'}


def _sl_toks_of(lines):
    return _SL_TOKEN.findall('\n'.join(lines))


def _sl_join_ops(t):
    """`>` `>` `>` `=` etc. as written: re-join shift and compound operators."""
    out, i = [], 0
    while i < len(t):
        for op in ('>>>=', '<<=', '>>=', '>>>', '<<', '>>', '+=', '-=', '*=', '/=', '%=', '&=', '|=', '^=',
                   '==', '!=', '<=', '>=', '&&', '||', '->', '::', '++', '--'):
            if t[i:i + len(op)] == list(op):
                out.append(op)
                i += len(op)
                break
        else:
            out.append(t[i])
            i += 1
    return out


def _sl_type_end(t, i):
    """The index after a type starting at t[i] (`a.b.C<..>[]`), or None."""
    if i >= len(t) or not _SL_IDENT.match(t[i]) or t[i] in ('return', 'new', 'this', 'super', 'final', 'else'):
        return None
    j = i + 1
    while j + 1 < len(t) and t[j] == '.' and _SL_IDENT.match(t[j + 1]):
        j += 2
    if j < len(t) and t[j] == '<':
        depth = 0
        while j < len(t):
            if t[j] == '<':
                depth += 1
            elif t[j] == '>':
                depth -= 1
            elif t[j] == '>>':
                depth -= 2
            elif t[j] == '>>>':
                depth -= 3
            elif t[j] in (';', '{', '}', '(', ')', '='):
                return None
            j += 1
            if depth <= 0:
                break
        if depth != 0:
            return None
        if j + 1 < len(t) and t[j] == '.' and _SL_IDENT.match(t[j + 1]):
            return _sl_type_end(t, j + 1)
    while j + 1 < len(t) and t[j] == '[' and t[j + 1] == ']':
        j += 2
    return j


def _sl_expr_end(t, i, stops=(';',)):
    """The index of the first top-level stop token at or after i."""
    depth = 0
    for j in range(i, len(t)):
        if t[j] in ('(', '[', '{'):
            depth += 1
        elif t[j] in (')', ']', '}'):
            if depth == 0:
                return j
            depth -= 1
        elif depth == 0 and t[j] in stops:
            return j
    return None


def _sl_match_close(t, i):
    """The index of the bracket closing t[i]."""
    depth = 0
    for j in range(i, len(t)):
        if t[j] in ('(', '[', '{'):
            depth += 1
        elif t[j] in (')', ']', '}'):
            depth -= 1
            if depth == 0:
                return j
    return None


def _sl_strip_casts(t):
    """Drop `(Type)` casts: a parenthesised type followed by an operand."""
    out, i = [], 0
    while i < len(t):
        if t[i] == '(':
            e = _sl_type_end(t, i + 1)
            if e is not None and e < len(t) and t[e] == ')' and e + 1 < len(t) and (
                    _SL_IDENT.match(t[e + 1]) or t[e + 1] in ('(', '"') or t[e + 1][0] in '\'"0123456789'
                    or t[e + 1] in ('!', '-', '~')) and t[e + 1] not in ('instanceof',):
                if t[i + 1] in _SL_PRIMS and e == i + 2:
                    out.append(t[i]); i += 1; continue
                i = e + 1
                continue
        out.append(t[i])
        i += 1
    return out


def _sl_decl(t, i):
    """`[final] TYPE x =` or `x =` at t[i]: (name, index after `=`), or None. Also `[final] TYPE x ;` -> (name, None)."""
    j = i
    if j < len(t) and t[j] == 'final':
        j += 1
    e = _sl_type_end(t, j)
    if e is not None and e < len(t) and _SL_IDENT.match(t[e]) and e + 1 < len(t) and t[e + 1] in ('=', ';'):
        return t[e], (e + 2 if t[e + 1] == '=' else None), e + 1
    if j < len(t) and _SL_IDENT.match(t[j]) and j + 1 < len(t) and t[j + 1] == '=':
        return t[j], j + 2, j + 1
    return None


def _sl_stmt_start(t, i):
    return i == 0 or t[i - 1] in (';', '{', '}', ')', ':', 'else')


_SL_KOTLIN = [False]


def _sl_fold_once(t):
    n = len(t)
    for i in range(n):
        # -> { return E ; }  ->  -> E
        if t[i] == '{' and i > 0 and t[i - 1] == '->' and i + 1 < n and t[i + 1] == 'return':
            e = _sl_expr_end(t, i + 2)
            if e is not None and t[e + 1:e + 2] == ['}']:
                return t[:i] + t[i + 2:e] + t[e + 2:]
        if not _sl_stmt_start(t, i):
            continue
        d = _sl_decl(t, i)
        # x = E ; return x ;   ->  return E ;
        if d and d[1] is not None:
            name, s = d[0], d[1]
            e = _sl_expr_end(t, s)
            if e is not None and t[e + 1:e + 4] == ['return', name, ';']:
                return t[:i] + ['return'] + t[s:e] + t[e + 3:]
            if e is not None and t[e + 1:e + 3] == ['return', '(']:
                c = _sl_type_end(t, e + 3)
                if c is not None and t[c:c + 3] == [')', name, ';']:
                    return t[:i] + t[e + 1:c + 1] + t[s:e] + t[c + 2:]
            if _SL_KOTLIN[0] and e is not None and t[e + 1:e + 2] == ['return']:
                r = _sl_expr_end(t, e + 2)
                uses = [k for k in range(e + 2, r or e + 2) if t[k] == name and t[k - 1] != '.']
                if r is not None and len(uses) == 1:
                    return t[:i] + t[e + 1:uses[0]] + t[s:e] + t[uses[0] + 1:]
            # T x = E ; for ( U u : x )
            if e is not None and t[e + 1:e + 3] == ['for', '(']:
                c = _sl_expr_end(t, e + 3, (':',))
                if c is not None and t[c:c + 3] == [':', name, ')']:
                    return t[:i] + t[e + 1:c + 1] + t[s:e] + t[c + 2:]
            # x = new T [ N ] ; x [ k ] = A ; ...
            if e is not None and t[s] == 'new':
                te = _sl_type_end(t, s + 1)
                tail = te + 3
                while tail + 1 < e and t[tail:tail + 2] == ['[', ']']:
                    tail += 2
                if te is not None and te + 3 <= e and t[te] == '[' and t[te + 2] == ']' and tail == e and t[te + 1].isdigit():
                    size, k, elems = int(t[te + 1]), e + 1, {}
                    while t[k:k + 2] == [name, '['] and k + 4 < len(t) and t[k + 2].isdigit() and t[k + 3:k + 5] == [']', '=']:
                        f = _sl_expr_end(t, k + 5)
                        if f is None:
                            break
                        elems[int(t[k + 2])] = t[k + 5:f]
                        k = f + 1
                    if elems and max(elems) < size:
                        init = []
                        for x in range(size):
                            init += (elems.get(x) or ['DEFAULT']) + [',']
                        return t[:s] + ['new'] + t[s + 1:te] + ['[', ']'] + t[te + 3:e] + ['{'] + init[:-1] + ['}'] + t[e:e + 1] + t[k:]
        # return ++x ;  ->  return x + 1 ;
        if t[i] == 'return' and i + 2 < n and t[i + 1] in ('++', '--') and _SL_IDENT.match(t[i + 2]) and t[i + 3:i + 4] in ([';'], []):
            return t[:i] + ['return', t[i + 2], t[i + 1][0], '1'] + t[i + 3:]
        # for ( Iterator it = E . iterator ( ) ; it . hasNext ( ) ; UPDATE ) { U u = it . next ( ) ; BODY }
        if t[i:i + 2] == ['for', '(']:
            d = _sl_decl(t, i + 2)
            if d and d[1] is not None:
                name, s0 = d[0], d[1]
                e = _sl_expr_end(t, s0)
                if (e is not None and t[e - 4] == '.' and t[e - 3] in ('iterator', 'listIterator') and t[e - 2:e] == ['(', ')']
                        and t[e + 1:e + 8] == [name, '.', 'hasNext', '(', ')', ';'][:6] + [t[e + 7]]):
                    u = e + 7
                    c = _sl_expr_end(t, u)
                    if c is not None and t[c + 1:c + 2] == ['{']:
                        d2 = _sl_decl(t, c + 2)
                        f = d2 and d2[1] is not None and _sl_expr_end(t, d2[1])
                        if f and t[f - 5:f] == [name, '.', 'next', '(', ')']:
                            close = _sl_match_close(t, c + 1)
                            if close is not None:
                                update = t[u:c] + [';'] if c > u else []
                                return (t[:i] + ['for', '('] + t[c + 2:d2[2]] + [':'] + t[s0:e - 4] + [')', '{']
                                        + t[f + 1:close] + update + t[close:])
        # x OP= E ; return x ;  ->  return x OP E ;
        if i + 1 < n and _SL_IDENT.match(t[i]) and t[i + 1].endswith('=') and t[i + 1][:-1] in _SL_COMPOUND:
            name, op = t[i], t[i + 1][:-1]
            e = _sl_expr_end(t, i + 2)
            if e is not None and t[e + 1:e + 4] == ['return', name, ';']:
                return t[:i] + ['return', name, op, '('] + t[i + 2:e] + [')'] + t[e + 3:]
        # Iterator it = E . iterator ( ) ; while ( it . hasNext ( ) ) { U u = it . next ( ) ;
        if d and d[1] is not None:
            name, s = d[0], d[1]
            e = _sl_expr_end(t, s)
            if (e is not None and t[e - 4] == '.' and t[e - 3] in ('iterator', 'listIterator') and t[e - 2:e] == ['(', ')']
                    and t[e + 1:e + 9] == ['while', '(', name, '.', 'hasNext', '(', ')', ')'] and t[e + 9] == '{'):
                d2 = _sl_decl(t, e + 10)
                if d2 and d2[1] is not None:
                    f = _sl_expr_end(t, d2[1])
                    if f is not None and t[f - 5:f] == [name, '.', 'next', '(', ')']:
                        head = t[e + 10:d2[2]]
                        return t[:i] + ['for', '('] + head + [':'] + t[s:e - 4] + [')', '{'] + t[f + 1:]
    return None


def _sl_ternary_split(t, s, e):
    """For an expression t[s:e]: (index of its top-level `?`, index of the matching `:`), or None."""
    depth, q, nest = 0, None, 0
    for k in range(s, e):
        x = t[k]
        if x in ('(', '[', '{'):
            depth += 1
        elif x in (')', ']', '}'):
            depth -= 1
        elif depth == 0 and x == '?':
            if q is None:
                q = k
            else:
                nest += 1
        elif depth == 0 and x == ':' and q is not None:
            if nest == 0:
                return q, k
            nest -= 1
        elif depth == 0 and x == '->':
            return None
    return None


def _sl_ends_in_exit(t, close):
    """The block closed by t[close] ends with `return ...;` or `throw ...;`."""
    if close < 1 or t[close - 1] != ';':
        return False
    depth = 0
    for k in range(close - 2, -1, -1):
        if t[k] in (')', ']', '}'):
            depth += 1
        elif t[k] in ('(', '[', '{'):
            if depth == 0:
                return t[k + 1] in ('return', 'throw') if t[k] == '{' else False
            depth -= 1
        elif depth == 0 and t[k] == ';':
            return t[k + 1] in ('return', 'throw')
    return depth == 0 and t[0] in ('return', 'throw')


def _sl_unpattern(t):
    """`E instanceof T x` as `E instanceof T`, with x read as E where it is used after."""
    for i in range(len(t)):
        if t[i] == 'instanceof':
            e = _sl_type_end(t, i + 1)
            if e is not None and e < len(t) and _SL_IDENT.match(t[e]) and t[e] not in ('instanceof', '&&', '||'):
                k = i - 1
                while k >= 0:
                    if t[k] in (')', ']') and t[k + 1] in ('.', 'instanceof', '['):
                        depth = 0
                        while k >= 0:
                            if t[k] in (')', ']'):
                                depth += 1
                            elif t[k] in ('(', '['):
                                depth -= 1
                                if depth == 0:
                                    break
                            k -= 1
                        k -= 1
                    elif (_SL_IDENT.match(t[k]) and t[k] not in ('return', 'new', 'case', 'throw', 'yield')) or t[k] == '.':
                        k -= 1
                    else:
                        break
                subject = t[k + 1:i]
                name = t[e]
                rest = []
                for x in t[e + 1:]:
                    rest += subject if x == name else [x]
                return _sl_unpattern(t[:e] + rest)
    return t


def _sl_raw_instanceof(t):
    """`instanceof T<..>` as `instanceof T`: the type a pattern names keeps its arguments."""
    out, i = [], 0
    while i < len(t):
        out.append(t[i])
        e = _sl_type_end(t, i + 1) if t[i] == 'instanceof' else None
        if e is not None:
            depth = 0
            for x in t[i + 1:e]:
                step = {'<': 1, '>': -1, '>>': -2, '>>>': -3}.get(x, 0)
                if depth == 0 and step == 0:
                    out.append(x)
                depth += step
            i = e
            continue
        i += 1
    return out


def _sl_untyped_calls(t):
    """`.<T>m(` as `.m(`: explicit type arguments of a call."""
    out, i = [], 0
    while i < len(t):
        if t[i] == '<' and i > 0 and t[i - 1] == '.':
            depth, j = 0, i
            while j < len(t):
                depth += {'<': 1, '>': -1, '>>': -2, '>>>': -3}.get(t[j], 0)
                if depth <= 0 or t[j] in (';', '{', '}', '='):
                    break
                j += 1
            if depth == 0 and j + 2 < len(t) and _SL_IDENT.match(t[j + 1]) and t[j + 2] == '(':
                i = j + 1
                continue
        out.append(t[i])
        i += 1
    return out


_SL_VAR_N = re.compile(r'var\d+$')


def _sl_untemp(t):
    """A `varN` temporary (no name in the LocalVariableTable) assigned and then tested by `instanceof`,
    read as the value it holds: javac's pattern matching on an expression."""
    for i in range(len(t)):
        if _sl_stmt_start(t, i):
            d = _sl_decl(t, i)
            if d and d[1] is not None and _SL_VAR_N.match(d[0]):
                e = _sl_expr_end(t, d[1])
                if e is not None and t[e + 1:e + 5] == ['if', '(', d[0], 'instanceof']:
                    value = t[d[1]:e]
                    rest = []
                    for x in t[e + 1:]:
                        rest += value if x == d[0] else [x]
                    return _sl_untemp(t[:i] + rest)
    return t


def _sl_unclash(t):
    """`namex` as `name` where `name` occurs too: the clash renamer's suffix, which moves with the locals."""
    names = set(x for x in t if _SL_IDENT.match(x))
    out = []
    for x in t:
        y = x
        while y.endswith('x') and _SL_IDENT.match(y) and y[:-1] in names:
            y = y[:-1]
        out.append(y)
    return out


def _sl_indexed_loops(t):
    """`T[] a = E; int n = a.length; for (int i = 0; i < n; i++) { .. a[i] .. }` (a for-each Vineflower did
    not rebuild) as `for (T $e : E) { .. $e .. }`."""
    for i in range(len(t)):
        if not _sl_stmt_start(t, i):
            continue
        e = _sl_type_end(t, i)
        if e is None or e < 2 or t[e - 2:e] != ['[', ']'] or e + 2 >= len(t) or t[e + 1] != '=':
            continue
        a = t[e]
        s0 = e + 2
        f = _sl_expr_end(t, s0)
        if f is None or t[f + 1:f + 2] != ['int'] or t[f + 3:f + 8] != ['=', a, '.', 'length', ';']:
            continue
        n = t[f + 2]
        k = f + 8
        if t[k:k + 4] != ['for', '(', 'int', t[k + 3] if k + 3 < len(t) else None] or k + 15 > len(t):
            continue
        ix = t[k + 3]
        if t[k + 4] != '=' or t[k + 5] not in ('0', 'DEFAULT') or t[k + 6:k + 14] != [';', ix, '<', n, ';', ix, '++', ')']:
            continue
        body = k + 14
        close = _sl_match_close(t, body)
        if t[body] != '{' or close is None:
            continue
        inner, j = [], body + 1
        while j < close:
            if t[j:j + 4] == [a, '[', ix, ']']:
                inner.append('$e')
                j += 4
            else:
                inner.append(t[j])
                j += 1
        return _sl_indexed_loops(t[:i] + ['for', '('] + t[i:e - 2] + ['$e', ':'] + t[s0:f] + [')', '{'] + inner + t[close:])
    return t


def _sl_foreach_vars(t):
    """Each for-each variable renamed `@k` (k-th loop) in its loop."""
    out, count = list(t), 0
    for i in range(len(out)):
        if out[i:i + 2] == ['for', '(']:
            c = _sl_expr_end(out, i + 2, (':', ';'))
            if c is None or out[c] != ':' or c - 1 <= i + 2 or not _SL_IDENT.match(out[c - 1]):
                continue
            close = _sl_match_close(out, i + 1)
            if close is None or close + 1 >= len(out) or out[close + 1] != '{':
                continue
            end = _sl_match_close(out, close + 1)
            if end is None:
                end = len(out)
            count += 1
            name, new = out[c - 1], f'@{count}'
            out[c - 1] = new
            for k in range(close + 1, end):
                if out[k] == name and out[k - 1] != '.':
                    out[k] = new
    return out


def _sl_unqualify(t):
    """`a.b.C` (lower-case package segments before a capitalised name) as `C`, and `Outer.Inner` as `Inner`:
    the import collector picks which of two same-named classes gets the simple name by the order it meets them."""
    out, i, fresh = [], 0, False
    while i < len(t):
        start = fresh or i == 0 or t[i - 1] != '.'
        fresh = False
        j = i
        while start and j + 2 < len(t) and re.match(r'[a-z][\w$]*$', t[j]) and t[j + 1] == '.':
            j += 2
        if j > i and re.match(r'[A-Z][\w$]*$', t[j]) and t[i] not in ('this', 'super'):
            i, fresh = j, True
            continue
        if (start and re.match(r'[A-Z][a-z][\w$]*$', t[i]) and i + 2 < len(t) and t[i + 1] == '.'
                and re.match(r'[A-Z][a-z][\w$]*$', t[i + 2])):
            i, fresh = i + 2, True
            continue
        out.append(t[i])
        i += 1
    return out


def _sl_canon(t):
    """A spelling both forms share: casts dropped, default values, `return (E);` unwrapped, `return C ? A : B;`
    as `if (C) { return A; } return B;`, and no `else` after a branch that returns or throws."""
    t = [('DEFAULT' if x in _SL_DEFAULTS else x) for x in _sl_strip_casts(_sl_raw_instanceof(_sl_unpattern(_sl_untemp(_sl_untyped_calls(_sl_unqualify(t))))))]
    t = _sl_foreach_vars(_sl_indexed_loops(_sl_unclash(t)))
    for _ in range(256):
        changed = False
        for i in range(len(t)):
            if t[i] == 'return' and i + 1 < len(t) and t[i + 1] == '(':
                e = _sl_expr_end(t, i + 2)
                if e is not None and t[e + 1:e + 2] == [';']:
                    t = t[:i + 1] + t[i + 2:e] + t[e + 1:]
                    changed = True
                    break
            if t[i] == 'return':
                e = _sl_expr_end(t, i + 1)
                q = e is not None and _sl_ternary_split(t, i + 1, e)
                if q:
                    c, a = q
                    t = t[:i] + ['if', '('] + t[i + 1:c] + [')', '{', 'return'] + t[c + 1:a] + [';', '}', 'return'] + t[a + 1:]
                    changed = True
                    break
            if t[i] == 'else' and i > 0 and t[i - 1] == '}' and _sl_ends_in_exit(t, i - 1):
                if t[i + 1:i + 2] == ['{']:
                    depth, k = 0, i + 1
                    while k < len(t):
                        if t[k] == '{':
                            depth += 1
                        elif t[k] == '}':
                            depth -= 1
                            if depth == 0:
                                break
                        k += 1
                    t = t[:i] + t[i + 2:k] + t[k + 1:]
                    changed = True
                    break
                else:
                    t = t[:i] + t[i + 1:]
                    changed = True
                    break
        if not changed:
            break
    return t


def _sl_declarations(t):
    """The locals declared at statement starts: {name: type tokens}, and t without the declarations
    (`T x;` dropped, `T x = E;` as `x = E;`)."""
    types, out, i = {}, [], 0
    while i < len(t):
        if _sl_stmt_start(t, i) and (i == 0 or t[i - 1] != '('):
            j = i + 1 if t[i] == 'final' else i
            e = _sl_type_end(t, j)
            if e is not None and e < len(t) and _SL_IDENT.match(t[e]) and e + 1 < len(t) and t[e + 1] in ('=', ';') \
                    and t[j] not in ('return', 'throw', 'yield', 'else', 'case', 'new', 'assert'):
                if types.get(t[e], t[j:e]) != t[j:e]:
                    return None, None
                types[t[e]] = t[j:e]
                if t[e + 1] == ';':
                    i = e + 2
                else:
                    out.append(t[e])
                    i = e + 1
                continue
        out.append(t[i])
        i += 1
    return types, out


def _sl_fold(b):
    for _ in range(256):
        f = _sl_fold_once(b)
        if f is None:
            break
        b = f
    return b


def _sl_patterns(t):
    """The number of `instanceof T x` patterns."""
    count = 0
    for i in range(len(t)):
        e = _sl_type_end(t, i + 1) if t[i] == 'instanceof' else None
        count += e is not None and e < len(t) and bool(_SL_IDENT.match(t[e])) and t[e] != 'instanceof'
    return count


def _sl_same(a, b):
    """Whether b, its kept locals folded, gives a's spelling, where the option explains the change: a fold, a
    kept local that became a pattern binding, a for-each rebuilt over a kept element local, or only the
    qualification of names (the import collector meets the kept locals' types first)."""
    fb = _sl_fold(b)
    if fb == b and _sl_patterns(b) <= _sl_patterns(a) and _sl_indexed_loops(a) == a \
            and _sl_unqualify(a) != _sl_unqualify(b):
        return False
    ca, cb = _sl_canon(a), _sl_canon(fb)
    return ca == cb or (len(ca) != len(cb) and [x for x in ca if x not in '()'] == [x for x in cb if x not in '()'])


def _sl_decl_order(t):
    """The declarations at statement starts, in order: (name, whether it has an initializer)."""
    out = []
    for i in range(len(t)):
        if _sl_stmt_start(t, i) and (i == 0 or t[i - 1] != '('):
            j = i + 1 if t[i] == 'final' else i
            e = _sl_type_end(t, j)
            if e is not None and e + 1 < len(t) and _SL_IDENT.match(t[e]) and t[e + 1] in ('=', ';') \
                    and t[j] not in ('return', 'throw', 'yield', 'else', 'case', 'new', 'assert'):
                out.append((t[e], t[e + 1] == '='))
    return out


_SL_KT_CONT = {'(', ',', '{', '=', '->', '+', '-', '*', '/', '&&', '||', '?:', '.', '?.', 'and', 'or', 'shl', 'shr',
           'ushr', 'xor', '['}
_SL_KT_ARRAYS = {'CharArray': 'char', 'IntArray': 'int', 'ByteArray': 'byte', 'LongArray': 'long', 'ShortArray': 'short',
             'FloatArray': 'float', 'DoubleArray': 'double', 'BooleanArray': 'boolean'}


def _sl_is_kotlin(lines):
    """Kotlin text: statement lines without a closing `;`."""
    body = [x.strip() for x in lines if x.strip()]
    return bool(body) and not any(x.endswith(';') for x in body) and any(
        x.startswith(('val ', 'var ', 'return')) or ' = ' in x for x in body)


def _sl_kt_tokens(lines):
    """Kotlin lines as the Java-like tokens the folds read: `;` at a statement's end, `val x: T = e` as
    `T x = e`, `CharArray(n)` as `new char[n]` and `charArrayOf(a, b)` as `new char[]{a, b}`."""
    rows = [_sl_join_ops(_sl_toks_of([x])) for x in lines if x.strip()]
    t = []
    for k, row in enumerate(rows):
        t += row
        nxt = rows[k + 1][0] if k + 1 < len(rows) else None
        if row[-1] not in _SL_KT_CONT and row != ['}'] and nxt not in ('.', '?.', ')', ']', '?:', ',', '?'):
            t.append(';')
    out, i = [], 0
    while i < len(t):
        if t[i] in ('val', 'var') and i + 1 < len(t) and _SL_IDENT.match(t[i + 1]):
            if t[i + 2:i + 3] == [':']:
                e = _sl_type_end(t, i + 3)
                if e is not None and t[e:e + 1] == ['=']:
                    out += t[i + 3:e] + [t[i + 1], '=']
                    i = e + 1
                    continue
            out.append(t[i + 1])
            i += 2
            continue
        c = _sl_match_close(t, i + 1) if t[i + 1:i + 2] == ['('] else None
        if t[i] in _SL_KT_ARRAYS and c is not None:
            out += ['new', _SL_KT_ARRAYS[t[i]], '['] + t[i + 2:c] + [']']
            i = c + 1
            continue
        if t[i].endswith('ArrayOf') and t[i][:-7].capitalize() + 'Array' in _SL_KT_ARRAYS and c is not None:
            out += ['new', t[i][:-7], '[', ']', '{'] + t[i + 2:c] + ['}']
            i = c + 1
            continue
        out.append(t[i])
        i += 1
    return out


def source_locals(old, new):
    """jvm_source_locals: the new hunk's kept locals folded back into their use give the old hunk, the
    declarations wherever they are. A hunk cut where brackets do not balance is not recognised."""
    try:
        return _sl_source_locals(old, new)
    except (IndexError, TypeError):
        return False


def _sl_source_locals(old, new):
    _SL_KOTLIN[0] = _sl_is_kotlin(old + new)
    if _SL_KOTLIN[0]:
        a, b = _sl_kt_tokens(old), _sl_kt_tokens(new)
    else:
        a, b = _sl_join_ops(_sl_toks_of(old)), _sl_join_ops(_sl_toks_of(new))
    if a == b:
        return False
    if _sl_same(a, b):
        return True
    ta, da = _sl_declarations(a)
    tb, db = _sl_declarations(b)
    if ta is None or tb is None or any(tb.get(n, ty) != ty for n, ty in ta.items()):
        return False
    if any(n not in ta and not any(db[k:k + 2] == [n, '='] for k in range(len(db))) for n in tb):
        return False
    if _sl_decl_order(a) == _sl_decl_order(b):
        return _sl_same(da, db)
    fdb = _sl_fold(db)
    ca, cb = _sl_canon(da), _sl_canon(fdb)
    return ca == cb or (len(ca) != len(cb) and [x for x in ca if x not in '()'] == [x for x in cb if x not in '()'])


_SL_HEADER = re.compile(r'^\s*(?:\)|[\w@<].*\().*\{$')
_SL_NOT_HEADER = re.compile(r'^\s*(?:if|for|while|switch|try|else|catch|synchronized|do|return|case|default|\}|.*->|.*\s=\s)')


def _sl_method_span(lines, i):
    """The lines of the method around line i: from its header's `{` line to its closing `}`."""
    for k in range(min(i, len(lines) - 1), -1, -1):
        line = lines[k]
        if _SL_HEADER.match(line) and not _SL_NOT_HEADER.match(line):
            ind = len(line) - len(line.lstrip())
            for m in range(k + 1, len(lines)):
                x = lines[m]
                if x.strip() and len(x) - len(x.lstrip()) <= ind:
                    if x.strip() == '}' and len(x) - len(x.lstrip()) == ind and m >= i:
                        return k, m + 1
                    break
    return None


def _sl_method_locals(a, b, i1, j1):
    """jvm_source_locals, a change spread over several hunks: the whole method around the hunk."""
    ra, rb = _sl_method_span(a, i1), _sl_method_span(b, j1)
    if not ra or not rb or a[ra[0]].strip() != b[rb[0]].strip():
        return False
    return source_locals(a[ra[0]:ra[1]], b[rb[0]:rb[1]])


_SL_DECL_LINE = re.compile(r'^(?:final\s+)?(?:@\w+\s+)*[\w$.]+(?:<.*>)?(?:\[\])*\s+([\w$]+)(?:\s*=\s*(.*))?;$')
_SL_ASSIGN_LINE = re.compile(r'^([\w$]+)\s*=\s*(.*);$')
_SL_WINDOW = 400


def _sl_line_decl(line):
    m = _SL_DECL_LINE.match(line.strip())
    if not m or line.strip().split()[0] in ('return', 'throw', 'new', 'else', 'case', 'yield'):
        return None
    return m.group(1), m.group(2)


def _sl_moved_in(line, there):
    """`line` (a declaration) occurs, stripped, among `there` (the other side's lines nearby)."""
    return line.strip() in {x.strip() for x in there}


def source_locals_context(a, b, i1, i2, j1, j2, throw_names):
    """jvm_source_locals across hunks: declarations moved within the method, or the whole method recognised."""
    try:
        return _sl_moved_declarations(a, b, i1, i2, j1, j2) or _sl_method_locals(a, b, i1, j1)
    except (IndexError, TypeError):
        return False


def _sl_moved_declarations(a, b, i1, i2, j1, j2):
    """jvm_source_locals, declarations: lines the hunk inserts are declarations moved up from below in the old
    text (or split off a declaration below: `T x = e;` became `x = e;`), lines it deletes are declarations moved
    up into the new text above (or such a split's `T x = e;`); what is left is equal or a fold of kept locals."""
    old, new = [x for x in a[i1:i2] if x.strip()], [x for x in b[j1:j2] if x.strip()]
    below_a, below_b = a[i2:i2 + _SL_WINDOW], b[j2:j2 + _SL_WINDOW]
    above_a, above_b = a[max(0, i1 - _SL_WINDOW):i1], b[max(0, j1 - _SL_WINDOW):j1]
    keep_new, moved = [], False
    for line in new:
        d = _sl_line_decl(line)
        if d and (_sl_moved_in(line, below_a) or _sl_moved_in(line, above_a)
                  or d[1] is None and any(_sl_line_decl(x) and _sl_line_decl(x)[0] == d[0] and _sl_line_decl(x)[1] for x in below_a)):
            moved = True
            continue
        if not d and _SL_ASSIGN_LINE.match(line.strip()):
            name, value = _SL_ASSIGN_LINE.match(line.strip()).groups()
            if any(_sl_line_decl(x) == (name, value) for x in old) and any((_sl_line_decl(x) or ('', 0))[0] == name and _sl_line_decl(x)[1] is None
                                                                   for x in above_b + new):
                moved = True
                old = [x for x in old if _sl_line_decl(x) != (name, value)]
                continue
        keep_new.append(line)
    keep_old = []
    for line in old:
        d = _sl_line_decl(line)
        if d and (_sl_moved_in(line, above_b) or _sl_moved_in(line, below_b)):
            moved = True
            continue
        if d and d[1] is not None and any(_SL_ASSIGN_LINE.match(x.strip()) and _SL_ASSIGN_LINE.match(x.strip()).groups() == d
                                          for x in below_b[:3] + new) and any(_sl_line_decl(x) == (d[0], None) for x in above_b):
            moved = True
            keep_new = [x for x in keep_new if not (_SL_ASSIGN_LINE.match(x.strip()) and _SL_ASSIGN_LINE.match(x.strip()).groups() == d)]
            continue
        keep_old.append(line)
    if not moved:
        return False
    if [x.strip() for x in keep_old] == [x.strip() for x in keep_new]:
        return True
    return bool(keep_old or keep_new) and source_locals(keep_old, keep_new)


def source_locals_diamond(old, new):
    """jvm_source_locals and external_class_diamond in one hunk: a kept local whose `new C<>(..)` the other
    fix gave its diamond."""
    plain = [re.sub(r'(\bnew [\w$.]+)<>\(', r'\1(', x) for x in new]
    return plain != new and source_locals(old, plain)


RECOGNISERS['jvm_source_locals'] = source_locals
CONTEXT_RECOGNISERS['jvm_source_locals'] = source_locals_context
RECOGNISERS['source_locals_diamond'] = source_locals_diamond


def source_locals_kt_narrow(old, new):
    """jvm_source_locals and kt_narrow_bitwise_operand in one Kotlin hunk: a kept local whose element stores
    the other fix wrote with the operands' promotion (`b.toInt() shr 4`)."""
    plain = [re.sub(r'\.(?:toInt|toLong)\(\)|\.code\b', '', x) for x in new]
    narrow = [re.sub(r'\.(?:toInt|toLong)\(\)|\.code\b', '', x) for x in old]
    return plain != new and narrow == old and source_locals(old, plain)


RECOGNISERS['source_locals_kt_narrow'] = source_locals_kt_narrow


def kt_names_delegate(old, new, me):
    """Whether the recogniser of another Kotlin output fix (`kt_` id) accepts the hunk; one level deep. The Java
    fixes' recognisers, some of which take any renamed identifier, are not asked."""
    if _KT_DELEGATING:
        return False
    _KT_DELEGATING.append(me)
    try:
        return any(r(old, new) for fix, r in RECOGNISERS.items() if fix.startswith('kt_') and fix != me)
    finally:
        _KT_DELEGATING.pop()


KT_HARD_KEYWORDS = {'as', 'break', 'class', 'continue', 'do', 'else', 'false', 'for', 'fun', 'if', 'in', 'interface', 'is',
                    'null', 'object', 'package', 'return', 'super', 'this', 'throw', 'true', 'try', 'typealias', 'typeof',
                    'val', 'var', 'when', 'while'}
JAVA_KEYWORDS = {'abstract', 'assert', 'boolean', 'break', 'byte', 'case', 'catch', 'char', 'class', 'const', 'continue',
                 'default', 'do', 'double', 'else', 'enum', 'extends', 'false', 'final', 'finally', 'float', 'for', 'goto',
                 'if', 'implements', 'import', 'instanceof', 'int', 'interface', 'long', 'native', 'new', 'null', 'package',
                 'private', 'protected', 'public', 'return', 'short', 'static', 'strictfp', 'super', 'switch',
                 'synchronized', 'throw', 'throws', 'transient', 'true', 'try', 'void', 'volatile', 'while'}
KT_KEYWORD_NAME_TOKEN = re.compile(r'`[^`\n]+`|' + TOKEN.pattern)
KT_GENERATED_VAR = re.compile(r'^var\d+(?:_\d+)?$')


def kt_java_keyword_name(token):
    """Whether `token` writes a Java keyword other than `this` as a Kotlin name (a Kotlin keyword in backticks)."""
    name = token[1:-1] if len(token) > 2 and token[0] == token[-1] == '`' else token
    return name in JAVA_KEYWORDS and name != 'this' and (name in KT_HARD_KEYWORDS) == (name != token)


def kt_java_keyword_renames(old, new):
    """The generated names (`var2`) of the hunk renamed to a Java keyword name, aligned token by token (equal-length
    replaced runs of the two token lists), as a dict; None when a name maps two ways."""
    a, b = KT_KEYWORD_NAME_TOKEN.findall('\n'.join(old)), KT_KEYWORD_NAME_TOKEN.findall('\n'.join(new))
    renames, back = {}, {}
    for tag, i1, i2, j1, j2 in difflib.SequenceMatcher(None, a, b, autojunk=False).get_opcodes():
        if tag != 'replace' or i2 - i1 != j2 - j1:
            continue
        for o, n in zip(a[i1:i2], b[j1:j2]):
            if KT_GENERATED_VAR.match(o) and kt_java_keyword_name(n):
                if renames.setdefault(o, n) != n or back.setdefault(n, o) != o:
                    return None
    return renames


def kt_java_keyword_variable_name(old, new):
    """kt_java_keyword_variable_name: variables printed by their generated names (`var2`, `var3_1`) now have their
    debug names, Java keywords Kotlin allows (`new`, `default`, `char`; a Kotlin keyword in backticks), one name per
    generated name; nothing else, or with the names written back another recogniser accepts the hunk."""
    names = lambda lines: collections.Counter(t for t in KT_KEYWORD_NAME_TOKEN.findall('\n'.join(lines))
                                              if kt_java_keyword_name(t))
    if not names(new) - names(old) or not any(KT_GENERATED_VAR.match(t) for t in TOKEN.findall('\n'.join(old))):
        return False
    renames = kt_java_keyword_renames(old, new)
    if not renames:
        return False
    a, b = KT_KEYWORD_NAME_TOKEN.findall('\n'.join(old)), KT_KEYWORD_NAME_TOKEN.findall('\n'.join(new))
    if len(a) == len(b) and all(o == n or renames.get(o) == n for o, n in zip(a, b)):
        return True
    back = {n: o for o, n in renames.items()}
    pattern = re.compile(r'(?<![\w$`])(' + '|'.join(re.escape(n) for n in back) + r')(?![\w$`])')
    plain = [pattern.sub(lambda m: back[m.group(1)], x) for x in new]
    return plain != new and (plain == list(old) or kt_names_delegate(old, plain, 'kt_java_keyword_variable_name'))


RECOGNISERS['kt_java_keyword_variable_name'] = kt_java_keyword_variable_name

KT_DEFAULT_SETTER = re.compile(r'^\s+(?:internal )?(?:final )?set$')


def kt_default_accessor_flags(old, new):
    """kt_default_accessor_flags: setter lines a setter read with flags 0 printed (`internal set`, `internal final
    set`, `final set`) left out, with blank lines next to them; the remaining lines the same, or the same tokens with
    the other ktprint7 fixes' changes undone too (kt_ktprint7_canon)."""
    setters = lambda lines: sum(1 for x in lines if KT_DEFAULT_SETTER.match(x) and x.strip() != 'set')
    if setters(old) <= setters(new):
        return False
    j = 0
    for o in old:
        if j < len(new) and o == new[j]:
            j += 1
        elif not (KT_DEFAULT_SETTER.match(o) and o.strip() != 'set') and o.strip():
            break
    else:
        if j == len(new):
            return True
    return tokens(kt_ktprint7_canon(old, True)[0]) == tokens(kt_ktprint7_canon(new, False)[0])


RECOGNISERS['kt_default_accessor_flags'] = kt_default_accessor_flags


def kt_escaped_name(name):
    """Whether kt_unescaped_name writes `name` in backticks: a Kotlin hard keyword or a name with `$`."""
    return name in KT_HARD_KEYWORDS or '$' in name


def kt_underscore_name(name):
    """Whether kt_reserved_underscore_name writes `name` in backticks: underscores alone."""
    return name != '' and set(name) == {'_'}


KT_QUOTED_NAME = re.compile(r'(?<![\w$`])`([\w$]+)`(?![\w$`])')


KT_ANONYMOUS_PARAMETER = re.compile(r'<anonymous parameter \d+>')


def kt_backticked(old, new, needs, me):
    """The hunk's new text puts more names `needs` accepts in backticks than the old one (or, for kt_unescaped_name,
    writes a lambda's `<anonymous parameter N>` `_`); with the ktprint7 fixes' changes undone on both sides
    (kt_ktprint7_canon: backticks out, Java's type syntax, the dropped setters) the tokens are the same (a reflow
    aside). A hunk another fix changed too is left to the context recogniser ktprint7_file."""
    count = lambda lines: sum(1 for x in lines for m in KT_QUOTED_NAME.finditer(x) if needs(m.group(1)))
    renamed = [KT_ANONYMOUS_PARAMETER.sub('_', x) for x in old] if me == 'kt_unescaped_name' else list(old)
    if renamed == list(old) and count(new) <= count(old):
        return False
    return tokens(kt_ktprint7_canon(renamed, True)[0]) == tokens(kt_ktprint7_canon(new, False)[0])


def kt_unescaped_name(old, new):
    """kt_unescaped_name: names with `$` and Kotlin hard keywords (`$this$asList`, `L$0`, `access$getX$p`, `in`) printed
    by the writers Kotlin shares with Java are in backticks; nothing else changed (kt_backticked)."""
    return kt_backticked(old, new, kt_escaped_name, 'kt_unescaped_name')


RECOGNISERS['kt_unescaped_name'] = kt_unescaped_name


def kt_reserved_underscore_name(old, new):
    """kt_reserved_underscore_name: a name of underscores alone (`_`) is in backticks; nothing else changed
    (kt_backticked)."""
    return kt_backticked(old, new, kt_underscore_name, 'kt_reserved_underscore_name')


RECOGNISERS['kt_reserved_underscore_name'] = kt_reserved_underscore_name


KT_TYPE_NAMES = [('java.util.Map.Entry', 'MutableMap.MutableEntry'), ('Map.Entry', 'MutableMap.MutableEntry'),
                 ('java.util.Map', 'MutableMap'), ('java.util.HashMap', 'HashMap'), ('java.util.List', 'MutableList'),
                 ('java.util.ArrayList', 'ArrayList'), ('java.util.Set', 'MutableSet'), ('java.util.HashSet', 'HashSet'),
                 ('java.util.Collection', 'MutableCollection'), ('java.util.Iterator', 'MutableIterator'),
                 ('java.lang.Iterable', 'MutableIterable'), ('java.lang.Integer', 'Int'), ('java.lang.Object', 'Any'),
                 ('Integer', 'Int'), ('Object', 'Any')]
KT_TYPE_NAME = re.compile(r'(?<![\w$.`])(' + '|'.join(re.escape(j) for j, _ in KT_TYPE_NAMES) + r')(?![\w$`])')
KT_PRIMITIVE_ARRAYS = {'boolean': 'BooleanArray', 'byte': 'ByteArray', 'char': 'CharArray', 'short': 'ShortArray',
                       'int': 'IntArray', 'long': 'LongArray', 'float': 'FloatArray', 'double': 'DoubleArray'}
KT_JAVA_ARRAY = re.compile(r'(?<![\w$.])([\w$.]+)\[\]')
KT_TYPE_PARAMETER_EXTENDS = re.compile(r'(?<=[<,] )(\w+) extends |(?<=<)(\w+) extends ')


KT_TRANSLATED_GENERIC = re.compile(r'(?<![\w$.`])(?:MutableMap\.MutableEntry|MutableMap|HashMap|MutableList|ArrayList|MutableSet|'
                                   r'HashSet|MutableCollection|MutableIterator|MutableIterable)<')


def kt_translated_projections_dropped(line):
    """`line` with the projections (`in `, `out `) of the type arguments of the collection types getKotlinType
    translates taken out: Vineflower dropped those wildcards."""
    pos = 0
    while True:
        m = KT_TRANSLATED_GENERIC.search(line, pos)
        if not m:
            return line
        depth, j, at_argument = 1, m.end(), True
        while j < len(line) and depth:
            if at_argument and depth == 1:
                for projection in ('in ', 'out '):
                    if line.startswith(projection, j):
                        line = line[:j] + line[j + len(projection):]
                at_argument = False
            c = line[j]
            depth += {'<': 1, '>': -1}.get(c, 0)
            if c == ',' and depth == 1:
                at_argument = True
                j += 1
                while j < len(line) and line[j] == ' ':
                    j += 1
                continue
            j += 1
        pos = m.end()


def kt_java_type_canon(line):
    """A line with Java's type syntax rewritten as kt_java_type_syntax writes it, leniently: wildcards written as
    projections and the projections of the translated collection types' arguments dropped (kt_translated_projections_
    dropped), Java's collection and boxed names mapped, `X[]` an `Array`, a type parameter's `extends` bound `:`,
    and a `where` clause or a second `&` bound dropped."""
    line = KT_TYPE_NAME.sub(lambda m: dict(KT_TYPE_NAMES)[m.group(1)], line)
    line = line.replace('? extends ', 'out ').replace('? super ', 'in ')
    line = kt_translated_projections_dropped(line)
    line = re.sub(r'(?<=<)\?(?=[,>])|(?<=, )\?(?=[,>])', '*', line)
    line = re.sub(r' where \w+ : .*?(?= \{$|$)', '', line)
    line = re.sub(r'(\w+) extends ([^<>&,]+(?:<[^&]*?>)?)(?: & [^<>,]+(?:<.*?>)?)+(?=[,>])', r'\1', line)
    line = KT_TYPE_PARAMETER_EXTENDS.sub(lambda m: (m.group(1) or m.group(2)) + ' : ', line)
    line = KT_JAVA_ARRAY.sub(lambda m: KT_PRIMITIVE_ARRAYS.get(m.group(1)) or f'Array<{m.group(1)}>', line)
    return line


def kt_java_type_syntax(old, new):
    """kt_java_type_syntax: types in Java's syntax (`Function0<T[]>`, `Continuation<? super java.lang.Boolean>`,
    `<T extends Comparable<? super T>>`, `Object[]::class.java`, `KClass<Object>`) became Kotlin's, or a collection
    type's argument gained the projection the old text dropped: with both written by kt_java_type_canon and import
    lines (an import the old type needed) left out, the tokens are the same (a reflow aside). A hunk another fix
    changed too is left to the context recogniser ktprint7_file."""
    plain_old = [kt_java_type_canon(x) for x in old if not is_import(x)]
    plain_new = [kt_java_type_canon(x) for x in new if not is_import(x)]
    if plain_old == [x for x in old if not is_import(x)] and plain_new == [x for x in new if not is_import(x)]:
        return False
    return tokens(kt_ktprint7_canon([x for x in old if not is_import(x)], True)[0]) == \
        tokens(kt_ktprint7_canon([x for x in new if not is_import(x)], False)[0])


RECOGNISERS['kt_java_type_syntax'] = kt_java_type_syntax


def kt_ktprint7_canon(lines, drop_setters):
    """Lines with the kt_unescaped_name, kt_reserved_underscore_name and kt_java_type_syntax changes undone (the
    backticks taken out, an anonymous lambda parameter `_`, types written by kt_java_type_canon) and, when
    `drop_setters`, the setter lines kt_default_accessor_flags leaves out dropped; and the index of each line's
    result (None for a dropped line)."""
    out, where = [], []
    for x in lines:
        if drop_setters and KT_DEFAULT_SETTER.match(x) and x.strip() != 'set':
            where.append(None)
            continue
        y = KT_ANONYMOUS_PARAMETER.sub('_', x)
        y = KT_QUOTED_NAME.sub(lambda m: m.group(1) if kt_escaped_name(m.group(1)) or kt_underscore_name(m.group(1))
                               else m.group(0), y)
        where.append(len(out))
        out.append(kt_java_type_canon(y))
    return out, where


_KTPRINT7_FILES = {}


def kt_ktprint7_touched(old, new):
    """Whether one of fixes 2-5 changed the hunk: the old text has a setter line kt_default_accessor_flags drops, an
    `<anonymous parameter N>` or Java's type syntax (kt_java_type_canon rewrites a line); or it has more bare names
    with `$` (a class's binary name aside) or of underscores alone than the new one; or a Kotlin keyword it has bare
    is in backticks in the new one."""
    if any((KT_DEFAULT_SETTER.match(x) and x.strip() != 'set') or KT_ANONYMOUS_PARAMETER.search(x) or
           kt_java_type_canon(x) != x for x in old if not is_import(x)):
        return True
    def bare(lines):
        names = re.findall(r'(?<![\w$`])[\w$]+(?![\w$`])', '\n'.join(KT_QUOTED_NAME.sub('', x) for x in lines))
        return (sum(1 for x in names if '$' in x and not (x[:1].isupper() and len(x.split('$')[0]) > 1)),
                sum(1 for x in names if kt_underscore_name(x)), set(names) & KT_HARD_KEYWORDS)
    (old_dollar, old_underscore, old_keywords), (new_dollar, new_underscore, _) = bare(old), bare(new)
    quoted = {m.group(1) for x in new for m in KT_QUOTED_NAME.finditer(x)}
    return old_dollar > new_dollar or old_underscore > new_underscore or bool(quoted & old_keywords)


def kt_inside_other_context_recogniser():
    """Whether another context recogniser's pass over its own rewritten texts is asking: ktprint7_file answers only
    for the compared files, so a hunk another fix's recogniser could not place does not pass through it."""
    codes = {r.__code__ for fix, r in CONTEXT_RECOGNISERS.items() if fix != 'ktprint7_file'}
    frame = sys._getframe(2)
    while frame is not None:
        if frame.f_code in codes:
            return True
        frame = frame.f_back
    return False


def kt_ktprint7_file(a, b, i1, i2, j1, j2, throw_names):
    """The kt_default_accessor_flags, kt_unescaped_name, kt_reserved_underscore_name and kt_java_type_syntax changes
    in a file, where they share a hunk or a reflowed run with another fix's change and line matching splits it: in a
    file those fixes changed (kt_ktprint7_canon changes a line of it), with both texts written by kt_ktprint7_canon
    (the old one's dropped setter lines left out), every line of the hunk in the canonical texts' difference is in no
    hunk the recognisers (with reflowed runs and the other context recognisers) leave as other; for a hunk at most
    three lines from one these fixes changed (kt_ktprint7_touched), also in a group of such hunks at most three
    equal lines apart that a recogniser accepts as a whole (line matching pairs a line such as `.add(` across a
    reflow). The other context recognisers' passes over their own rewritten texts get no
    answer from this one (kt_inside_other_context_recogniser)."""
    if kt_inside_other_context_recogniser():
        return False
    key = (id(a), id(b), len(a), len(b))
    if key not in _KTPRINT7_FILES:
        ca, wa = kt_ktprint7_canon(a, True)
        cb, wb = kt_ktprint7_canon(b, False)
        changed = any(w is None or ca[w] != x for w, x in zip(wa, a)) or any(cb[w] != x for w, x in zip(wb, b))
        bad_a, bad_b, plain_a, plain_b = set(), set(), set(), set()
        if changed:
            marked = failure_blocks(ca)
            hunks = [(classify(ca, cb, x1, x2, y1, y2, marked), x1, x2, y1, y2)
                     for tag, x1, x2, y1, y2 in difflib.SequenceMatcher(None, ca, cb, autojunk=False).get_opcodes()
                     if tag != 'equal']
            hunks = [('context' if kind == 'other' and any(
                fix != 'ktprint7_file' and r(ca, cb, x1, x2, y1, y2, throw_names)
                for fix, r in CONTEXT_RECOGNISERS.items()) else kind, x1, x2, y1, y2)
                for kind, x1, x2, y1, y2 in reflowed(ca, cb, hunks)]
            for kind, x1, x2, y1, y2 in hunks:
                if kind == 'other':
                    plain_a.update(range(x1, x2))
                    plain_b.update(range(y1, y2))
            groups = []
            for hunk in hunks:
                if groups and hunk[1] - groups[-1][-1][2] <= 3 and hunk[3] - groups[-1][-1][4] <= 3:
                    groups[-1].append(hunk)
                else:
                    groups.append([hunk])
            for group in groups:
                if all(kind != 'other' for kind, *_ in group):
                    continue
                old, new = ca[group[0][1]:group[-1][2]], cb[group[0][3]:group[-1][4]]
                if same_tokens(old, new) or any(r(old, new) for fix, r in RECOGNISERS.items()):
                    continue
                for kind, x1, x2, y1, y2 in group:
                    if kind == 'other':
                        bad_a.update(range(x1, x2))
                        bad_b.update(range(y1, y2))
        touched = [(x1, x2) for tag, x1, x2, y1, y2 in difflib.SequenceMatcher(None, a, b, autojunk=False).get_opcodes()
                   if changed and tag != 'equal' and kt_ktprint7_touched(a[x1:x2], b[y1:y2])]
        if len(_KTPRINT7_FILES) >= 4:
            _KTPRINT7_FILES.clear()
        _KTPRINT7_FILES[key] = (changed, wa, wb, bad_a, bad_b, plain_a, plain_b, touched, a, b)
    changed, wa, wb, bad_a, bad_b, plain_a, plain_b, touched, _, _ = _KTPRINT7_FILES[key]
    if not changed:
        return False
    if not any(x1 - 3 <= i2 and i1 <= x2 + 3 for x1, x2 in touched):
        bad_a, bad_b = plain_a, plain_b
    return all(wa[x] is None or wa[x] not in bad_a for x in range(i1, i2)) and \
        all(wb[y] not in bad_b for y in range(j1, j2))


CONTEXT_RECOGNISERS['ktprint7_file'] = kt_ktprint7_file

# --- ktinline fixes: field_read_all_paths, kt_inline_evaluation_order ---
# Both keep a local Vineflower's Kotlin inliner (INLINE_ALL_VARS) moved to its use: the new text declares
# (or assigns) a local the old one has fewer uses of and uses it where the old text has its value. The
# recogniser writes the values back: the hunk, or a run of up to four neighbouring hunks of the function,
# must then be the old text token for token (parentheses, conversion calls, casts, `val`/`var` and types,
# and numbered names `varN` aside), or, where the kept local stopped a later rebuilding (an `if` written as
# a conditional expression, a condition joined with `&&`), have the same tokens once the control structure
# (`if`, `else`, braces, `return`, `&&`, labels) and the numbered temporaries are dropped.

KT_KEPT_HEAD = re.compile(r'^(\s*)(?:(?:val|var) )?(`[^`]+`|[A-Za-z_$][\w$]*)(?:: [^=]+?)? = (.*)$')
KT_KEPT_BARE = re.compile(r'^\s*(?:val|var) (`[^`]+`|[A-Za-z_$][\w$]*): [^=]+$')
KT_KEPT_FUN = re.compile(r'^\s*(?:[\w@]+(?:\([^)]*\))? )*(?:fun|constructor|init)\b|^\s*(?:get|set)\(')
KT_KEPT_CONV = re.compile(r'\.to(?:Int|Long|Byte|Short|Char|Float|Double)\(\)')
KT_KEPT_NUMBERED = re.compile(r'`?var\d+x?`?$')
KT_KEPT_OPEN = {'(': 1, '[': 1, '{': 1, ')': -1, ']': -1, '}': -1}
KT_KEPT_WORD = re.compile(r'`[^`\n]+`|[A-Za-z_$][\w$]*')
KT_KEPT_STRUCTURE = {'if', 'else', '{', '}', '(', ')', 'return', '&', '|', 'val', 'var', ':', '=', ';'}
KT_KEPT_ANY_DECL = re.compile(r'\b(?:val|var) (`[^`]+`|[A-Za-z_$][\w$]*)(?:: [\w$.<>?*\[\], ]+?)? = ')
KT_KEPT_LABEL = re.compile(r'label\d+@?$|@')
KT_KEPT_CALL = re.compile(r'[\w$`]\s*\(|\[')
KT_KEPT_HUNKS = {}


def kt_kept_depth(line):
    return sum(KT_KEPT_OPEN.get(t, 0) for t in kt_tokens([line]))


def kt_kept_statement(lines, i):
    """The end (exclusive) of the statement that starts at line i: up to balanced brackets, with the
    deeper-indented lines that continue it."""
    depth, j, ind = kt_kept_depth(lines[i]), i + 1, indent_of(lines[i])
    while j < len(lines) and (depth > 0 or (lines[j].strip() and indent_of(lines[j]) > ind
                                             and not lines[i].rstrip().endswith('{'))):
        if depth <= 0 and lines[j].lstrip().startswith(('}', 'else', 'catch', 'finally')):
            break
        depth += kt_kept_depth(lines[j])
        j += 1
    return j


def kt_kept_method(lines, k):
    """(start, end) of the innermost function, constructor, init block or accessor around line k."""
    for i in range(min(k, len(lines) - 1), -1, -1):
        if KT_KEPT_FUN.match(lines[i]):
            head = i
            while head < len(lines) - 1 and not lines[head].rstrip().endswith('{') and head - i < 20:
                head += 1
            ind, end = indent_of(lines[i]), head + 1
            while end < len(lines) and not (lines[end].strip() and indent_of(lines[end]) <= ind):
                end += 1
            if end < len(lines) and lines[end].strip().startswith('}'):
                end += 1
            if end > k:
                return i, end
    return 0, len(lines)


def kt_kept_tokens(text_lines):
    """The tokens of a text without parentheses, conversion calls, casts, bare declarations and the
    `val`/`var` and type of a declaration."""
    out = []
    for x in text_lines:
        if KT_KEPT_BARE.match(x):
            continue
        m = KT_KEPT_HEAD.match(x)
        if m and x.lstrip().startswith(('val ', 'var ')):
            x = f'{m.group(1)}{m.group(2)} = {m.group(3)}'
        out.append(KT_KEPT_CONV.sub('', KT_AS_CAST.sub('', kt_uncast(x))))
    return [t for t in kt_tokens(out) if t not in ('(', ')')]


def kt_kept_same(a, b):
    """Whether two token lists are equal up to a one-to-one renaming of numbered names `varN`."""
    if len(a) != len(b):
        return False
    fwd, back = {}, {}
    for x, y in zip(a, b):
        if x != y and not (KT_KEPT_NUMBERED.match(x) and KT_KEPT_NUMBERED.match(y)
                           and fwd.setdefault(x, y) == y and back.setdefault(y, x) == x):
            return False
    return True


def kt_kept_free(text_lines):
    """How often a text names each name other than as a member (`x.name`), string templates' `$name` included."""
    toks = kt_tokens(text_lines)
    count = collections.Counter(t for k, t in enumerate(toks)
                                if (k == 0 or toks[k - 1] != '.') and KT_NAME.fullmatch(t))
    count.update(re.findall(r'\$([A-Za-z_]\w*)', '\n'.join(t for t in toks if t.startswith('"'))))
    return count


def kt_kept_names(a_method, b_method):
    """The names the new function uses more often than the old one: the locals it keeps."""
    old_count, new_count = kt_kept_free(a_method), kt_kept_free(b_method)
    return {n for n, c in new_count.items() if c > old_count[n]}


def kt_kept_assignments(lines, kept):
    """(start, end, name, value) of each statement that declares or assigns one of the kept names."""
    out, i = [], 0
    while i < len(lines):
        m = KT_KEPT_HEAD.match(lines[i])
        if m and m.group(2) in kept:
            j = kt_kept_statement(lines, i)
            out.append((i, j, m.group(2), ' '.join([m.group(3)] + [x.strip() for x in lines[i + 1:j]])))
            i = j
            continue
        i += 1
    return out


def kt_kept_sub_text(text, subst):
    """`text` with each name of `subst` replaced by its value: in code as `(value)` (not as a member
    `.name` or an assignment's target), in a string template as `${value}`."""
    out, i, stack = [], 0, ['code']
    while i < len(text):
        c = text[i]
        if stack[-1] == 'string':
            if c == '\\':
                out.append(text[i:i + 2])
                i += 2
                continue
            if c == '"':
                stack.pop()
            elif text.startswith('${', i):
                stack.append('code')
                out.append('${')
                i += 2
                continue
            elif c == '$':
                m = KT_KEPT_WORD.match(text, i + 1)
                if m and m.group(0) in subst:
                    out.append('${' + subst[m.group(0)] + '}')
                    i = m.end()
                    continue
            out.append(c)
            i += 1
            continue
        m = KT_KEPT_WORD.match(text, i) if i == 0 or not re.match(r'[\w$`.]', text[i - 1]) else None
        if m:
            name = m.group(0)
            if name in subst and not re.match(r' [-+*/]?= ', text[m.end():]):
                out.append('(' + subst[name] + ')')
            elif name.startswith('$') and name[1:] in subst:
                out.append('${' + subst[name[1:]] + '}')
            else:
                out.append(name)
            i = m.end()
            continue
        if c == '"':
            stack.append('string')
        elif c == '{':
            stack.append('code')
        elif c == '}' and len(stack) > 1:
            stack.pop()
        out.append(c)
        i += 1
    return ''.join(out)


def kt_kept_inline(new, kept, subst, drop=True):
    """The new lines without the kept names' assignments (with `drop`), each kept name replaced by the
    value `subst` gives it or the last assignment dropped before it gives it."""
    subst = dict(subst)
    groups, i = [([], dict(subst))], 0
    local = {s: (e, n, v) for s, e, n, v in kt_kept_assignments(new, kept)} if drop else {}
    while i < len(new):
        if i in local:
            e, n, v = local[i]
            subst[n] = v
            groups.append(([], dict(subst)))
            i = e
            continue
        if not (KT_KEPT_BARE.match(new[i]) and KT_KEPT_BARE.match(new[i]).group(1) in kept):
            groups[-1][0].append(new[i])
        i += 1
    out = []
    for lines, values in groups:
        text = '\n'.join(lines)
        for _ in range(4 if values else 0):
            done, text = text, kt_kept_sub_text(text, values)
            if done == text:
                break
        out += text.split('\n') if lines else []
    return out


def kt_kept_choices(kept, b_method, new):
    """The ways to give the kept names `new` uses one of the values their assignments in the function
    give them, a name possibly left as it is (at most 32)."""
    used = set(kt_kept_free(new)) | {t[1:] for t in re.findall(r'\$[A-Za-z_][\w]*', '\n'.join(new))}
    values = collections.defaultdict(list)
    for _, _, n, v in kt_kept_assignments(b_method, kept & used):
        if v not in values[n]:
            values[n].append(v)
    choices = [{}]
    for n, vs in values.items():
        choices = [dict(c, **{n: v}) if v is not None else c for c in choices for v in vs + [None]][:32]
    return choices


def kt_kept_loose(text_lines, temps):
    """The tokens of a text, counted, without its control structure, declarations' keywords and types,
    numbered names and the temporaries `temps`."""
    toks = []
    for x in text_lines:
        if not KT_KEPT_BARE.match(x):
            x = KT_KEPT_ANY_DECL.sub(r'\1 = ', x)
            toks += re.findall(r'`[^`\n]+`|\w+|\S', KT_KEPT_CONV.sub('', KT_AS_CAST.sub('', kt_uncast(x))))
    return collections.Counter(t for t in toks if t not in KT_KEPT_STRUCTURE and t not in temps
                               and not KT_KEPT_LABEL.match(t) and not KT_KEPT_NUMBERED.match(t))


def kt_kept_read(lines, kept):
    """The kept names the lines read somewhere other than as an assignment's target."""
    reads = kt_kept_free(lines)
    reads.subtract(n for _, _, n, _ in kt_kept_assignments(lines, kept))
    return {n for n in kept if reads[n] > 0}


def kt_kept_scope(a, b, i1, j1):
    """The functions around old line i1 and new line j1, and the locals the new one keeps: read, unless the
    function holds a failure comment (a lambda that failed reads them unprinted). Memoised per file pair, as
    kt_kept_context's hunks."""
    memo = KT_KEPT_HUNKS.setdefault('scopes', {}) if KT_KEPT_HUNKS.get('a') is a else {}
    if (i1, j1) not in memo:
        ma, ea = kt_kept_method(a, i1)
        mb, eb = kt_kept_method(b, j1)
        kept = kt_kept_names(a[ma:ea], b[mb:eb])
        unprinted = any(x.strip().startswith((MARKER, KUNA_NOTE, VINEFLOWER_NOTE)) for x in b[mb:eb])
        memo[(i1, j1)] = ((ma, ea), (mb, eb), kept if unprinted else kept & kt_kept_read(b[mb:eb], kept))
    return memo[(i1, j1)]


def kt_kept_span(a, b, i1, i2, j1, j2):
    """The values of the kept locals the span's new text, with them written back, is the old text with
    (a dict, empty for a renaming only), or None."""
    old, new = a[i1:i2], b[j1:j2]
    (ma, ea), (mb, eb), kept = kt_kept_scope(a, b, i1, j1)
    if not kept:
        return None
    old_toks = kt_kept_tokens(old)
    if kt_kept_same(old_toks, kt_kept_tokens(new)):
        return {}
    candidates = []
    for subst in kt_kept_choices(kept, b[mb:eb], new):
        for drop in (True, False):
            inl = kt_kept_inline(new, kept, subst, drop)
            dropped = {n: v for _, _, n, v in kt_kept_assignments(new, kept)} if drop else {}
            if not any(x.strip() for x in old) and not any(x.strip() for x in inl):
                return dict(subst, **dropped)
            if kt_kept_same(old_toks, kt_kept_tokens(inl)):
                return dict(subst, **dropped)
            if len(candidates) < 2 and inl != new:
                candidates.append((inl, dict(subst, **dropped)))
    for inl, values in candidates:
        if any(recognises(old, inl) for recognises in RECOGNISERS.values()):
            return values
    temps = {n for n in kept if sum(1 for _, _, m, _ in kt_kept_assignments(b[mb:eb], kept) if m == n) > 1}
    for subst in kt_kept_choices(kept - temps, b[mb:eb], new):
        inl = kt_kept_inline(new, kept - temps, subst)
        defined = {n: v for _, _, n, v in kt_kept_assignments(new, kept)}
        if (subst or defined) and kt_kept_loose(old, temps) == kt_kept_loose(inl, temps):
            return dict(subst, **defined)
    return None


def kt_kept_context(a, b, i1, i2, j1, j2, fields):
    """Whether the hunk, alone or with up to three neighbouring hunks on either side, is a run whose new
    text keeps locals (`fields`: whose values all read fields only, else: one of which does more)."""
    if KT_KEPT_HUNKS.get('a') is not a or KT_KEPT_HUNKS.get('b') is not b:
        KT_KEPT_HUNKS.clear()
        KT_KEPT_HUNKS.update(a=a, b=b, hunks=[h[1:] for h in difflib.SequenceMatcher(None, a, b, autojunk=False)
                                               .get_opcodes() if h[0] != 'equal'])
    hunks = KT_KEPT_HUNKS['hunks']
    k = next((n for n, h in enumerate(hunks) if h == (i1, i2, j1, j2)), None)
    spans = [(k, k)] if k is None else [(k - x, k + y) for x in range(4) for y in range(4)
                                         if k - x >= 0 and k + y < len(hunks)]
    for lo, hi in spans:
        x1, _, y1, _ = hunks[lo] if lo is not None else (i1, i2, j1, j2)
        _, x2, _, y2 = hunks[hi] if hi is not None else (i1, i2, j1, j2)
        values = kt_kept_span(a, b, x1, x2, y1, y2)
        if values is None:
            continue
        _, (mb, eb), kept = kt_kept_scope(a, b, x1, y1)
        all_values = [v for _, _, _, v in kt_kept_assignments(b[mb:eb], kept)]
        field_like = all(not KT_KEPT_CALL.search(KT_AS_CAST.sub('', v)) for v in all_values)
        if field_like == fields:
            return True
    return False


def field_read_all_paths_scope(a, b, i1, i2, j1, j2, throw_names):
    """field_read_all_paths: a local holding a field read is kept where Vineflower read the field again
    past a join or into a catch or finally body (the values written back read fields only)."""
    return kt_kept_context(a, b, i1, i2, j1, j2, True)


def kt_inline_evaluation_order_scope(a, b, i1, i2, j1, j2, throw_names):
    """kt_inline_evaluation_order: a local whose value calls, allocates or reads an array element is kept
    where Vineflower moved the value past what it could observe, into a conditional, or out of order."""
    return kt_kept_context(a, b, i1, i2, j1, j2, False)


CONTEXT_RECOGNISERS['field_read_all_paths'] = field_read_all_paths_scope
CONTEXT_RECOGNISERS['kt_inline_evaluation_order'] = kt_inline_evaluation_order_scope


# --- ktops2 fixes: kt_negated_equality, kt_unboxing_cast_type (with kt_operator_precedence's unwritten unboxing
# calls), kt_binary_numeric_promotion, kt_copied_equality, kt_operator_line_break ---

# --- pattern_binding_shape (a fix) and jvm_source_patterns (a kuna-form option, on by default): an `instanceof`
# pattern the bytecode does not show is the source's test and cast, `if (x instanceof T) { T v = (T)x; ..}`;
# jvm_source_locals keeps a named local Vineflower folded into a pattern's subject. The recognisers rebuild
# Vineflower's pattern from the new hunk (the cast's declaration taken into the test as its binding, the kept
# local folded into the subject, two nested ifs merged) and compare the spellings, parentheses and the type
# arguments a pattern names aside.

def _jp_toks(lines):
    return _sl_join_ops(_sl_toks_of(lines))


def _jp_binding_end(t, k):
    """The index after the type of `instanceof` at t[k] when no binding follows it, else None."""
    e = _sl_type_end(t, k + 1)
    if e is None or (e < len(t) and _SL_IDENT.match(t[e]) and t[e] not in ('instanceof', 'true', 'false', 'null')):
        return None
    return e


def _jp_uncast_once(t):
    """The first `T v = (T')E;` at a statement start whose E is tested earlier by `E instanceof T''` without a
    binding: the declaration removed and the test written `E instanceof T'' v`."""
    for i in range(len(t)):
        if not _sl_stmt_start(t, i):
            continue
        d = _sl_decl(t, i)
        if not d or d[1] is None or d[1] >= len(t) or t[d[1]] != '(':
            continue
        name, s = d[0], d[1]
        c = _sl_type_end(t, s + 1)
        if c is None or c >= len(t) or t[c] != ')':
            continue
        e = _sl_expr_end(t, c + 1)
        subject = t[c + 1:e] if e is not None else []
        if not subject:
            continue
        for k in range(i - 1, len(subject) - 1, -1):
            if t[k] == 'instanceof' and t[k - len(subject):k] == subject:
                end = _jp_binding_end(t, k)
                if end is not None:
                    return t[:end] + [name] + t[end:i] + t[e + 1:]
                break
    return None


def _jp_uncast(t):
    for _ in range(64):
        u = _jp_uncast_once(t)
        if u is None:
            break
        t = u
    return t


def _jp_simple(expr):
    """Whether an expression needs no parentheses as an operand of `instanceof`."""
    depth = 0
    for x in expr:
        if x in ('(', '[', '{'):
            depth += 1
        elif x in (')', ']', '}'):
            depth -= 1
        elif depth == 0 and x in ('?', ':', '&&', '||', '==', '!=', '+', '-', '*', '/', '%', '<', '>', '<=', '>=',
                                  '&', '|', '^', '=', 'instanceof'):
            return False
    return True


def _jp_statement_head_end(t):
    """The index of the first top-level `;` or `{` of t (the end of its first statement or block head)."""
    depth = 0
    for j, x in enumerate(t):
        if depth == 0 and x in (';', '{'):
            return j
        if x in ('(', '['):
            depth += 1
        elif x in (')', ']'):
            depth -= 1
    return len(t)


def _jp_fold_once(t, loose=False):
    """The first `T x = E;` at a statement start whose next statement tests `x instanceof`, x used once in the
    rest: the declaration removed and E put in x's place. `loose`: bare declarations `U y;` may come between,
    and x may be used again after an assignment `x = ..` that follows the test."""
    for i in range(len(t)):
        if not _sl_stmt_start(t, i):
            continue
        d = _sl_decl(t, i)
        if not d or d[1] is None or (i > 0 and t[i - 1] == '('):
            continue
        name, s = d[0], d[1]
        e = _sl_expr_end(t, s)
        if e is None:
            continue
        rest = t[e + 1:]
        skip = 0
        while loose:
            b = _sl_decl(rest, skip)
            if not b or b[1] is not None:
                break
            skip = b[2] + 1
        uses = [k for k, x in enumerate(rest) if x == name and (k == 0 or rest[k - 1] != '.')]
        if not uses or rest[uses[0] + 1:uses[0] + 2] != ['instanceof']:
            continue
        if len(uses) > 1 and not (loose and rest[uses[1] + 1:uses[1] + 2] == ['='] and _sl_stmt_start(rest, uses[1])):
            continue
        if uses[0] > skip + _jp_statement_head_end(rest[skip:]):
            continue
        value = t[s:e] if _jp_simple(t[s:e]) else ['('] + t[s:e] + [')']
        return t[:i] + rest[:uses[0]] + value + rest[uses[0] + 1:]
    return None


def _jp_fold(t, loose=False):
    for _ in range(64):
        u = _jp_fold_once(t, loose)
        if u is None:
            break
        t = u
    return t


def _jp_merge_ifs(t):
    """`if (A) { if (B) { S } }` (neither with an else) as `if (A && B) { S }`."""
    for _ in range(64):
        for i in range(len(t)):
            if t[i:i + 2] != ['if', '('] or not _sl_stmt_start(t, i):
                continue
            c = _sl_match_close(t, i + 1)
            if c is None or t[c + 1:c + 4] != ['{', 'if', '(']:
                continue
            outer = _sl_match_close(t, c + 1)
            c2 = _sl_match_close(t, c + 3)
            if outer is None or c2 is None or t[c2 + 1:c2 + 2] != ['{']:
                continue
            inner = _sl_match_close(t, c2 + 1)
            if inner is None or inner + 1 != outer or t[outer + 1:outer + 2] == ['else']:
                continue
            t = t[:i + 2] + t[i + 2:c] + [')', '&&', '('] + t[c + 4:c2] + [')', '{'] + t[c2 + 2:inner] + ['}'] + t[outer + 1:]
            break
        else:
            return t
    return t


def _jp_canon(t):
    return [x for x in _sl_raw_instanceof(t) if x not in ('(', ')')]


def _jp_unbind(t):
    """`instanceof T v` as `instanceof T`."""
    out, i = [], 0
    while i < len(t):
        out.append(t[i])
        if t[i] == 'instanceof':
            e = _sl_type_end(t, i + 1)
            if e is not None and e < len(t) and _SL_IDENT.match(t[e]) and t[e] not in ('instanceof',):
                out += t[i + 1:e]
                i = e + 1
                continue
        i += 1
    return out


def _jp_rebuild(old, new, cast, fold):
    a, b = _jp_toks(old), _jp_toks(new)
    if a == b:
        return False
    nb = b
    if cast:
        nb = _jp_uncast(nb)
        if nb == b:
            return False
    if fold:
        folded = _jp_fold(nb)
        if folded == nb:
            return False
        nb = _jp_merge_ifs(folded)
    return _jp_canon(nb) == _jp_canon(a) or _jp_canon(_sl_canon(nb)) == _jp_canon(_sl_canon(a))


def instanceof_cast(old, new):
    """pattern_binding_shape, jvm_source_patterns: `x instanceof T v` became `x instanceof T` and `T v = (T)x;`
    (in the if's body, or after a negated test's exit): the declaration taken back into the test gives the old
    hunk; or the hunk holds only the test, its binding dropped."""
    if _jp_rebuild(old, new, True, False):
        return True
    a, b = _jp_toks(old), _jp_toks(new)
    return a != b and _jp_unbind(a) == b


def pattern_subject_local(old, new):
    """jvm_source_locals: a local the class file names stays the subject of a pattern, `T x = E; if (x
    instanceof U u)`, where Vineflower folded it, `if (E instanceof U u)` (two ifs then merged into one)."""
    return _jp_rebuild(old, new, False, True)


def instanceof_cast_subject_local(old, new):
    """Both of the above in one hunk: a kept local tested and cast, `T x = E; if (x instanceof U) { U u = (U)x;`."""
    return _jp_rebuild(old, new, True, True)


_JP_CAST_DECL = re.compile(r'^(?:final\s+)?[\w$.]+(?:<.*>)?(?:\[\])*\s+([\w$]+)\s*=\s*\(([\w$.]+)(?:<.*>)?(?:\[\])*\)(.+);$')


def instanceof_cast_declaration(a, b, i1, i2, j1, j2, throw_names):
    """pattern_binding_shape, jvm_source_patterns, a hunk that only inserts `T v = (T)E;`: the old text binds
    `v` in a pattern on E above, which the new text tests without the binding."""
    if i1 != i2 or not any(x.strip() for x in b[j1:j2]):
        return False
    for line in b[j1:j2]:
        if not line.strip():
            continue
        m = _JP_CAST_DECL.match(line.strip())
        if not m:
            return False
        name, cls, subject = m.group(1), m.group(2), m.group(3).strip()
        tested = re.escape(subject) + r'\s+instanceof\s+' + re.escape(cls) + r'\b'
        if not any(re.search(tested + r'(?:<[^)]*>)?(?:\[\])*\s+' + re.escape(name) + r'\b', x) for x in a[max(0, i1 - 60):i1]):
            return False
        if not any(re.search(tested + r'(?:\[\])*\s*\)', x) for x in b[max(0, j1 - 60):j1]):
            return False
    return True


RECOGNISERS['instanceof_cast'] = instanceof_cast
RECOGNISERS['pattern_subject_local'] = pattern_subject_local
RECOGNISERS['instanceof_cast_subject_local'] = instanceof_cast_subject_local
CONTEXT_RECOGNISERS['instanceof_cast'] = instanceof_cast_declaration


def instanceof_forms(a, b, i1, i2, j1, j2, throw_names):
    """The four changes above across several hunks of a method (two nested ifs, an if javac laid out negated
    around its pattern, `if (!(x instanceof T v)) { exit } ..`): the whole method, its casts taken back into
    the tests and its kept locals folded, lays out as the old one (jvm_javac_layout's recogniser)."""
    ra, rb = _sl_method_span(a, i1), _sl_method_span(b, j1)
    if not ra or not rb or a[ra[0]].strip() != b[rb[0]].strip():
        return False
    old, new = a[ra[0]:ra[1]], b[rb[0]:rb[1]]
    if not any('instanceof' in x for x in old):
        return False
    t = _jp_merge_ifs(_jp_fold(_jp_uncast(_jp_toks(new)), True))
    if _jp_canon(t) == _jp_canon(_jp_toks(old)) or _jp_renamed(_jp_toks(old), t):
        return True
    rebuilt = [' '.join(t)]
    return javac_layout(old, rebuilt) or source_locals(old, rebuilt)


def _jp_renamed(a, b):
    """Whether a and b, their declarations dropped, differ only by a one-to-one renaming of identifiers (a kept
    local reassigned after the test, where Vineflower declared a variable of its own)."""
    _, da = _sl_declarations(a)
    _, db = _sl_declarations(b)
    if da is None or db is None:
        return False
    da, db = _jp_canon(da), _jp_canon(db)
    if len(da) != len(db) or da == db:
        return False
    forward, backward = {}, {}
    for x, y in zip(da, db):
        if x == y:
            continue
        if not (_SL_IDENT.match(x) and _SL_IDENT.match(y)) or forward.setdefault(x, y) != y or backward.setdefault(y, x) != x:
            return False
    return True


CONTEXT_RECOGNISERS['instanceof_forms'] = instanceof_forms


# --- jfinally fixes: finally_exit_versions, var_merge_lost_definition, head_definition_scope ---

JF_PURE_DECL = re.compile(r'^(?:final )?' + TYPE_NAME + r' ([\w$]+);$|^var ([\w$`]+): ' + TYPE_NAME + r'$')
JF_INIT_DECL = re.compile(r'^(?:final )?' + TYPE_NAME + r' ([\w$]+) = |^(?:val|var) ([\w$`]+)(?:: ' + TYPE_NAME
                          + r')? = ')
JF_CONTROL = ('if ', 'if(', 'while ', 'for ', 'switch ', 'synchronized ', 'try ', 'catch ', 'do ', 'else', 'when ',
              'return ', 'throw ', 'label', 'run ', '}')
JF_HEAD = re.compile(r'^(?:switch|synchronized|when) \(')
JF_VAR = re.compile(r'var\d+$')


def jf_declared(line):
    """The name a line declares (`T v;`, `T v = e;`, `var v: T`, `val v: T = e`), or None."""
    s = line.strip()
    m = JF_PURE_DECL.match(s) or JF_INIT_DECL.match(s)
    return next((g for g in m.groups() if g), None) if m and not s.startswith(('return ', 'throw ')) else None


def jf_method(lines, k):
    """The lines (start, end) of the method or lambda body around line `k`: from the nearest enclosing line that
    opens a block and is no statement (a method header, `) {` ending a wrapped one, a lambda's `-> {`)."""
    if k >= len(lines):
        return 0, len(lines)
    indent = indent_of(lines[k]) if lines[k].strip() else 10 ** 6
    for i in range(k - 1, -1, -1):
        s = lines[i].strip()
        if not s or indent_of(lines[i]) >= indent:
            continue
        indent = indent_of(lines[i])
        if s.endswith('{') and not s.startswith(JF_CONTROL) and ('(' in s or s.startswith(')')) \
                and ' = ' not in s and not s.endswith('@{'):
            end = closing(lines, i, indent)
            return i, end if end is not None else len(lines)
    return 0, len(lines)


def jf_enclosing(lines, k):
    """The lines opening the blocks around line `k`, innermost first, up to its method."""
    out, indent = [], indent_of(lines[k]) if lines[k].strip() else 10 ** 6
    start, _ = jf_method(lines, k)
    for i in range(k - 1, start, -1):
        if lines[i].strip() and indent_of(lines[i]) < indent:
            indent = indent_of(lines[i])
            out.append(lines[i].strip())
    return out


def finally_exit_versions(a, b, i1, i2, j1, j2, throw_names):
    """finally_exit_versions: a value a `finally` body assigns reaches the code after the `try`: in the body,
    a declaration of a variable of its own (`T v = e;`) became an assignment of the variable the code after
    the `try` reads (`w = e;`), that variable's bare declaration is added before the `try`, or a read after
    the `try` is renamed to it."""
    old, new = a[i1:i2], b[j1:j2]
    if old and len(old) == len(new):
        pairs = list(zip(old, new))
        if all(o == n or (jf_declared(o) and not jf_declared(n) and undeclared([o])[1:] == undeclared([n])[1:]
                          and '} finally {' in jf_enclosing(b, j1 + k))
               for k, (o, n) in enumerate(pairs)) and old != new:
            return True
    if not old and new and all(JF_PURE_DECL.match(n.strip()) for n in new):
        nxt = next((x.strip() for x in b[j2:] if x.strip()), '')
        if nxt.endswith(': {'):
            nxt = next((x.strip() for x in b[b.index(b[j2]) + 1:] if x.strip() and not x.strip().endswith(': {')), '')
        names = {jf_declared(n) for n in new} - {None}
        start, end = jf_method(b, j1)
        body = b[j2:end]
        return nxt == 'try {' and not names & jf_restored(a, b, j1) \
            and all(any(x.strip().startswith(name + ' = ') for x in body) for name in names)
    if len(old) == len(new) and old != new:
        ta, tb = tokens(old), tokens(new)
        renamed = {(x, y) for x, y in zip(ta, tb) if x != y}
        if len(ta) != len(tb) or len(renamed) != 1:
            return False
        (_, w), = renamed
        start, _ = jf_method(b, j1)
        fin = [k for k in range(start, j1) if b[k].strip() == '} finally {']
        return IDENT.match(w) is not None and any(
            b[x].strip().startswith(w + ' = ') for k in fin for x in range(k, (closing(b, k, indent_of(b[k])) or k)))
    return False


def jf_restored(a, b, j1):
    """The names whose bare declaration the method around `b[j1]` gained while the method in `a` used them
    undeclared (by that name, or by the name the clash renaming then gave another variable)."""
    start, end = jf_method(b, j1)
    names = {jf_declared(x) for x in b[start:end] if JF_PURE_DECL.match(x.strip())} - {None}
    head = b[start].strip()
    k = next((i for i, x in enumerate(a) if x.strip() == head and indent_of(x) == indent_of(b[start])), None)
    if k is None:
        return set()
    a_end = closing(a, k, indent_of(a[k])) or len(a)
    old = a[k:a_end]
    used = {t for t in tokens(old) if IDENT.match(t)}
    return {n for n in names if (n in used or re.sub(r'x+$', '', n) in used)
            and not any(jf_declared(x) == n for x in old)}


def var_merge_lost_definition(a, b, i1, i2, j1, j2, throw_names):
    """var_merge_lost_definition: a variable Vineflower's `mergeVars` left undeclared gets its declaration
    back (`T v;` added where the method used `v` undeclared, renamed with `x`s where another variable of its
    name is in scope), and, in that method, the variables merged differently in its stead: `varN`
    declarations added and the same statements once declarations are reduced to assignments and `varN`
    names are exchanged."""
    old, new = a[i1:i2], b[j1:j2]
    restored = jf_restored(a, b, j1)
    if not restored or old == new:
        return False
    if not old and new and all(jf_declared(n) and JF_PURE_DECL.match(n.strip())
                               and (jf_declared(n) in restored or JF_VAR.match(jf_declared(n))) for n in new):
        return True
    if x_renamed(old, new):
        return all(y in restored for x, y in zip(tokens(old), tokens(new)) if x != y)
    ua, ub = undeclared(old), undeclared(new)
    if not ua or len(ua) != len(ub):
        return False
    return all(x == y or (JF_VAR.match(x) and JF_VAR.match(y)) for x, y in zip(ua, ub))


def head_definition_scope(a, b, i1, i2, j1, j2, throw_names):
    """head_definition_scope: a declaration renamed with `x`s, with its reads, because a variable of that name
    declared before a `switch` or `synchronized` (in the statement's head block) is still in scope there."""
    old, new = a[i1:i2], b[j1:j2]
    if not x_renamed(old, new):
        return False
    names = {re.sub(r'x+$', '', y) for x, y in zip(tokens(old), tokens(new)) if x != y}
    for name in names:
        found = False
        for k in range(j1):
            if jf_declared(b[k]) != name:
                continue
            d = indent_of(b[k])
            h = next((m for m in range(k + 1, j1) if b[m].strip() and indent_of(b[m]) <= d
                      and not jf_declared(b[m]) and not b[m].strip().endswith(';')), None)
            if h is not None and indent_of(b[h]) == d and JF_HEAD.match(b[h].strip()) \
                    and all(not x.strip() or indent_of(x) >= d for x in b[h:j1]):
                found = True
        if not found:
            return False
    return True


CONTEXT_RECOGNISERS['finally_exit_versions'] = finally_exit_versions
CONTEXT_RECOGNISERS['var_merge_lost_definition'] = var_merge_lost_definition
CONTEXT_RECOGNISERS['head_definition_scope'] = head_definition_scope


# --- jswitch: switch layout (fixes switch_case_order, switch_table_fillers, switch_synthetic_default) and the
# boolean comparisons of jvm_javac_layout. A switch's hunks are judged on the whole switch statement or
# expression around them: its cases, compared as a multiset (order aside), lose only filler labels, javac's
# synthetic default and the trailing `break;` the last case leaves implicit.
JSW_HEADER = re.compile(r'^(\s*)(?:.*\W)?switch \(.*\) \{$')
JSW_LABEL = re.compile(r'^(?:case .+|default)(?::(?: \{)?| ->.*)$')
JSW_SYNTHETIC = ('throw new MatchException(null, null);', 'throw new IncompatibleClassChangeError();')


def jsw_span(text, lo, hi):
    """(header, closing line) of the innermost switch around text[lo:hi] (an empty range is a position)."""
    for s in range(min(lo, len(text) - 1), -1, -1):
        m = JSW_HEADER.match(text[s])
        if not m:
            continue
        indent = m.group(1)
        for e in range(s + 1, len(text)):
            line = text[e]
            if line.startswith(indent + '}') and not line.startswith(indent + ' '):
                if e >= max(hi, lo + 1) - 1 and s < lo + (hi == lo):
                    return s, e
                break
    return None


def jsw_groups(body, indent):
    """The cases of a switch body: (labels, statement lines) with the labels of one case together."""
    groups, labels, stmts = [], [], []
    case_indent = indent + '   '
    for line in body:
        s = line.strip()
        if line.startswith(case_indent) and not line.startswith(case_indent + ' ') and JSW_LABEL.match(s):
            if stmts or (labels and ' -> ' in labels[-1]) or (labels and ' -> ' in s):
                groups.append((labels, stmts))
                labels, stmts = [], []
            labels.append(s)
        elif s:
            stmts.append(line)
    if labels or stmts:
        groups.append((labels, stmts))
    return groups


def jsw_canon(stmts):
    """Statement lines as tokens up to negation, every switch in them as its cases sorted."""
    out, i = [], 0
    while i < len(stmts):
        m = JSW_HEADER.match(stmts[i])
        end = next((e for e in range(i + 1, len(stmts)) if stmts[e].startswith(m.group(1) + '}')
                    and not stmts[e].startswith(m.group(1) + ' ')), None) if m else None
        if end is None:
            out += jbool_canon(tokens([stmts[i]]))
            i += 1
            continue
        out += jbool_canon(tokens([stmts[i]])) + ['<cases>'] + [repr(c) for c in
                                                                 sorted(jsw_normal(jsw_groups(stmts[i + 1:end], m.group(1))))]
        out += tokens([stmts[end]])
        i = end + 1
    return tuple(out)


def jsw_normal(groups):
    """The cases without filler labels (the keys of the default's case), javac's synthetic default, empty
    trailing cases and a trailing `break;` (or the `return;` a void method's last case leaves implicit), each
    as a hashable value."""
    out = []
    for labels, stmts in groups:
        if any(x.startswith('default') for x in labels):
            labels = [x for x in labels if x.startswith('default')]
        while stmts and stmts[-1].strip() in ('break;', 'return;'):
            stmts = stmts[:-1]
        if len(stmts) > 1 and stmts[-1].strip() == '}' and stmts[-2].strip() == 'break;' and labels[-1].endswith('{'):
            stmts = stmts[:-2] + stmts[-1:]
        body = [x.strip() for x in stmts]
        arrow = [x.split(' -> ', 1)[1] for x in labels if ' -> ' in x]
        if labels == ['default:'] and len(body) == 1 and body[0] in JSW_SYNTHETIC:
            continue
        if arrow and labels[0].startswith('default') and arrow[0] in JSW_SYNTHETIC:
            continue
        if not stmts and not arrow:
            continue
        out.append((tuple(labels), tuple(re.sub(r'^(var|label)\d+$', r'\1#', t) for t in jsw_canon(stmts))))
    return out


def jsw_cases(text, span):
    s, e = span
    return jsw_normal(jsw_groups(text[s + 1:e], JSW_HEADER.match(text[s]).group(1)))


def switch_case_order_scope(a, b, i1, i2, j1, j2, throw_names):
    """switch_case_order: the hunk lies in a switch whose cases are the same as before in another order (the
    order of their code), filler labels, javac's synthetic default and implicit last `break;` aside."""
    sa, sb = jsw_span(a, i1, i2), jsw_span(b, j1, j2)
    if not sa or not sb or a[sa[0]].strip() != b[sb[0]].strip():
        return False
    ca, cb = jsw_cases(a, sa), jsw_cases(b, sb)
    return sorted(ca) == sorted(cb) and (ca != cb or a[sa[0]:sa[1]] != b[sb[0]:sb[1]])


def switch_table_fillers_scope(a, b, i1, i2, j1, j2, throw_names):
    """switch_table_fillers: only `case` label lines removed, each followed in the old text by more labels down
    to `default:`, or forming with them the empty last case of the switch."""
    if j1 != j2 or i1 == i2 or not all(JSW_LABEL.match(x.strip()) and x.strip().startswith('case ')
                                       for x in a[i1:i2]):
        return False
    k = i2
    while k < len(a) and a[k].strip().startswith('case ') and a[k].strip().endswith(':'):
        k += 1
    if k < len(a) and a[k].strip() == 'default:':
        return True
    span = jsw_span(a, i1, i2)
    return span is not None and k == span[1]


def switch_synthetic_default(old, new):
    """switch_synthetic_default: javac's default of an exhaustive switch (`default: throw new MatchException(null,
    null);` or `IncompatibleClassChangeError`, colon or arrow) removed and nothing else."""
    s = [x.strip() for x in old if x.strip()]
    if any(x.strip() for x in new):
        return False
    return s in ([y + ' ' + x for y in ('default ->',) for x in JSW_SYNTHETIC[:1]],
                 ['default -> ' + JSW_SYNTHETIC[1]], ['default:', JSW_SYNTHETIC[0]], ['default:', JSW_SYNTHETIC[1]])


def jsw_expression_span(text, lo, hi):
    """(header, closing line) of a `return switch (..) {` expression in or around text[lo:hi]."""
    for s in list(range(lo, min(hi, len(text)))) + list(range(min(lo, len(text) - 1), -1, -1)):
        if re.match(r'^\s*return switch \(.*\) \{$', text[s]):
            indent = re.match(r'^(\s*)', text[s]).group(1)
            for e in range(s + 1, len(text)):
                if text[e] == indent + '};':
                    return (s, e) if e >= max(hi, lo + 1) - 1 else None
            return None
    return None


def jsw_as_expression(groups):
    """The arms (as token lists) the cases of a switch statement that return or throw become in `return
    switch`, or None when a case does something else."""
    out = []
    for labels, stmts in groups:
        if not labels or not stmts or not all(x.endswith(':') for x in labels):
            return None
        names = [x[:-1].split(' ', 1)[1] if x.startswith('case ') else 'default' for x in labels]
        head = tokens(['default' if names == ['default'] else 'case ' + ', '.join(names), '->'])
        toks = tokens(stmts)
        ends = [j for j in top_level(toks, 0, len(toks)) if toks[j] == ';']
        if ends == [len(toks) - 1] and toks[0] in ('return', 'throw'):
            out.append(head + (toks[1:] if toks[0] == 'return' else toks))
        elif ends and ends[-1] == len(toks) - 1 and toks[ends[-2] + 1 if len(ends) > 1 else 0] == 'return':
            last = ends[-2] + 1 if len(ends) > 1 else 0
            out.append(head + ['{'] + toks[:last] + ['yield'] + toks[last + 1:] + ['}'])
        else:
            return None
    return out


def switch_expression_return_scope(a, b, i1, i2, j1, j2, throw_names):
    """switch_synthetic_default: a switch statement over an enum with javac's synthetic default whose cases
    return or throw became `return switch (..) { .. };`, its returns yields (the arms in their code's order)."""
    sb = jsw_expression_span(b, j1, j2)
    if not sb:
        return False
    head = b[sb[0]].strip()[len('return '):]
    for s in range(min(i1, len(a) - 1), max(-1, i1 - 400), -1):
        if a[s].strip() != head:
            continue
        span = jsw_span(a, s + 1, s + 1)
        if not span or span[0] != s or not (s <= i1 and i2 <= span[1] + 1):
            return False
        groups = [(g[0], [x.strip() for x in g[1]]) for g in jsw_groups(a[s + 1:span[1]], JSW_HEADER.match(a[s]).group(1))]
        arms = jsw_as_expression([g for g in groups
                                  if not (g[0] == ['default:'] and len(g[1]) == 1 and g[1][0] in JSW_SYNTHETIC)])
        have = [tokens(labels + [x.strip() for x in stmts]) for labels, stmts in
                jsw_groups(b[sb[0] + 1:sb[1]], re.match(r'^(\s*)', b[sb[0]]).group(1))]
        return arms is not None and sorted(arms) == sorted(have)
    return False


JBOOL_LITERAL = [['false', '=', '='], ['=', '=', 'false'], ['true', '=', '='], ['=', '=', 'true'],
                 ['!', '=', 'true'], ['!', '=', 'false'], ['false', '!', '='], ['true', '!', '=']]
JBOOL_OPS = {('=', '='): 'EQ', ('!', '='): 'EQ', ('<', '='): 'GT', ('>', '='): 'LT', ('&', '&'): 'AND',
             ('|', '|'): 'AND'}


def jbool_canon(toks):
    """Tokens up to negation: comparisons with `true`/`false` and `!` dropped, `==`/`!=`, `<`/`>=`, `>`/`<=` and
    `&&`/`||` each made one operator, parentheses dropped."""
    out, i = [], 0
    toks = list(toks)
    while i < len(toks):
        if any(toks[i:i + 3] == lit for lit in JBOOL_LITERAL):
            i += 3
            continue
        pair = tuple(toks[i:i + 2])
        if pair in JBOOL_OPS:
            out.append(JBOOL_OPS[pair])
            i += 2
            continue
        t = toks[i]
        if t in ('!', '(', ')'):
            i += 1
            continue
        out.append({'<': 'LT', '>': 'GT'}.get(t, t))
        i += 1
    return out


def jbool_count(toks):
    return sum(1 for i in range(len(toks)) for lit in JBOOL_LITERAL if toks[i:i + 3] == lit)


def jbool_unpattern(toks):
    """Tokens with each `instanceof` type's arguments and pattern binding dropped."""
    out, i = [], 0
    while i < len(toks):
        out.append(toks[i])
        if toks[i] != 'instanceof':
            i += 1
            continue
        i += 1
        if i < len(toks) and IDENT.match(toks[i]):
            out.append(toks[i])
            i += 1
        while i + 1 < len(toks) and toks[i] == '.' and IDENT.match(toks[i + 1]):
            out += toks[i:i + 2]
            i += 2
        if i < len(toks) and toks[i] == '<':
            depth = 0
            while i < len(toks):
                depth += {'<': 1, '>': -1}.get(toks[i], 0)
                i += 1
                if depth == 0:
                    break
        if i < len(toks) and IDENT.match(toks[i]):
            i += 1
    return out


def javac_boolean_compare(old, new):
    """jvm_javac_layout, its boolean comparisons: the same expressions up to negation (De Morgan, a relation
    negated) with comparisons with `false`/`true` that the new text writes where the old one wrote `!x`, `x` or
    a negated condition."""
    a, b = tokens(old), tokens(new)
    return (a != b and jbool_count(b) > jbool_count(a)
            and jbool_canon(jbool_unpattern(a)) == jbool_canon(jbool_unpattern(b)))


def javac_boolean_pattern_scope(a, b, i1, i2, j1, j2, throw_names):
    """jvm_javac_layout, its boolean comparisons: `T v = (T)x;` added below `false == (x instanceof T)`, where
    the old text bound `v` in a pattern `!(x instanceof T v)` (javac's instanceof without a pattern's
    checkcast right after it: the source had no pattern)."""
    added = [x.strip() for x in b[j1:j2] if x.strip()]
    if i1 != i2 or len(added) != 1:
        return False
    m = re.fullmatch(r'([\w$.<>?, \[\]]+) ([\w$]+) = \(\1\)([\w$.]+);', added[0])
    if not m:
        return False
    var, value = m.group(2), m.group(3)
    pattern = re.compile(r'%s instanceof [\w$.<>?, \[\]]+ %s\)' % (re.escape(value), re.escape(var)))
    return (any(pattern.search(x) for x in a[max(0, i1 - 12):i1])
            and any(re.search(r'false == \(%s instanceof ' % re.escape(value), x) for x in b[max(0, j1 - 12):j1]))


def javac_layout_or_boolean_compare(old, new):
    """jvm_javac_layout: its ifs and exits (javac_layout) or its boolean comparisons (javac_boolean_compare)."""
    return javac_layout(old, new) or javac_boolean_compare(old, new)


RECOGNISERS['switch_synthetic_default'] = switch_synthetic_default
RECOGNISERS['jvm_javac_layout'] = javac_layout_or_boolean_compare
CONTEXT_RECOGNISERS['switch_table_fillers'] = switch_table_fillers_scope
CONTEXT_RECOGNISERS['switch_case_order'] = switch_case_order_scope
CONTEXT_RECOGNISERS['switch_synthetic_default'] = switch_expression_return_scope
CONTEXT_RECOGNISERS['jvm_javac_layout'] = javac_boolean_pattern_scope


JW3_NUM = re.compile(r'Float\.(?:MAX_VALUE|MIN_VALUE|MIN_NORMAL)|\d+\.\d+(?:E-?\d+)?[FfDd]?|\d+(?:E-?\d+)?[FfDdLl]?'
                     r'|"(?:[^"\\\n]|\\.)*"|\'(?:[^\'\\\n]|\\.)*\'|[\w$]+|\S')
JW3_FLOAT_NAMES = {'Float.MAX_VALUE': 3.4028234663852886e38, 'Float.MIN_VALUE': 1.401298464324817e-45,
                   'Float.MIN_NORMAL': 1.1754943508222875e-38}
JW3_PRIMITIVE_CASTS = {'int', 'long', 'float', 'double', 'char', 'short', 'byte'}
JW3_UNBOXING = {'booleanValue', 'byteValue', 'charValue', 'shortValue', 'intValue', 'longValue', 'floatValue',
                'doubleValue'}


def jw3_tokens(text_lines):
    """The tokens of some lines, a number literal (suffix included) and `Float.MAX_VALUE` each one token."""
    return JW3_NUM.findall('\n'.join(text_lines))


def jw3_float_value(tok):
    """The value of a float literal or `Float` constant name, as a double; None for other tokens."""
    if tok in JW3_FLOAT_NAMES:
        return JW3_FLOAT_NAMES[tok]
    if re.fullmatch(r'\d+(?:\.\d+)?(?:E-?\d+)?[Ff]', tok):
        import struct
        return struct.unpack('f', struct.pack('f', float(tok[:-1])))[0]
    return None


def double_constant_literal(old, new):
    """double_constant_literal: token for token, a float literal (or `Float.MAX_VALUE`-style name) Vineflower
    wrote for a double constant became the double literal of the same value."""
    a, b = jw3_tokens(old), jw3_tokens(new)
    if a == b or len(a) != len(b):
        return False
    for x, y in zip(a, b):
        if x == y:
            continue
        v = jw3_float_value(x)
        if v is None or not re.fullmatch(r'\d+\.\d+(?:E-?\d+)?', y) or float(y) != v:
            return False
    return True


def jw3_char_code(tok):
    """The code of a char literal token, as a decimal string; the token itself otherwise."""
    if len(tok) >= 3 and tok[0] == tok[-1] == "'":
        body = tok[1:-1]
        escapes = {'\\n': 10, '\\t': 9, '\\r': 13, '\\b': 8, '\\f': 12, "\\'": 39, '\\"': 34, '\\\\': 92, '\\0': 0}
        if len(body) == 1:
            return str(ord(body))
        if body in escapes:
            return str(escapes[body])
        if re.fullmatch(r'\\u[0-9a-fA-F]{4}', body):
            return str(int(body[2:], 16))
    return tok


def jw3_drop_primitive_casts(toks):
    """The tokens without primitive casts `(int)` before an operand."""
    out, i = [], 0
    while i < len(toks):
        if toks[i] == '(' and i + 3 < len(toks) and toks[i + 1] in JW3_PRIMITIVE_CASTS and toks[i + 2] == ')' \
                and operand_start(toks[i + 3]):
            i += 3
            continue
        out.append(toks[i])
        i += 1
    return out


def concat_operand_type(old, new):
    """concat_operand_type: in a string concatenation, primitive casts added before operands and integer
    constants written as the char literals of the same code."""
    a, b = tokens(old), tokens(new)
    if a == b or (not any(t.startswith('"') for t in a + b) and '+' not in b):
        return False
    return [jw3_char_code(t) for t in a] == [jw3_char_code(t) for t in jw3_drop_primitive_casts(b)]


def jw3_unspread(toks):
    """The tokens with array creations `new T[]{..}` written as their elements and `new T[0]` dropped (with
    the comma before it)."""
    out, i = [], 0
    while i < len(toks):
        if toks[i] == 'new':
            j = type_end(toks, i + 1)
            if j is not None and j < len(toks) and toks[j] == '{' and toks[j - 1] == ']':
                end = block_end(toks, j)
                if end is not None:
                    out += jw3_unspread(toks[j + 1:end])
                    i = end + 1
                    continue
            if j is not None and j + 2 < len(toks) and toks[j:j + 3] == ['[', '0', ']']:
                if out and out[-1] == ',':
                    out.pop()
                i = j + 3
                continue
        out.append(toks[i])
        i += 1
    return out


def vararg_spread_resolution(old, new):
    """vararg_spread_resolution: a varargs call's spread elements written back as the array creation
    (`f()` to `f(new T[0])`, `f(a)` to `f(new T[]{a})`), or a lone element given a cast."""
    a, b = tokens(old), tokens(new)
    if a == b:
        return False
    return strip_casts(jw3_unspread(b)) == strip_casts(a) or strip_casts(b) == strip_casts(a)


def jw3_unboxed(toks):
    """The tokens without unboxing calls `.intValue()` and with boxing calls `Integer.valueOf(x)` as `x`."""
    out, i = [], 0
    while i < len(toks):
        if toks[i] == '.' and i + 3 < len(toks) and toks[i + 1] in JW3_UNBOXING and toks[i + 2] == '(' \
                and toks[i + 3] == ')':
            i += 4
            continue
        if toks[i] in BOXES and i + 3 < len(toks) and toks[i + 1] == '.' and toks[i + 2] == 'valueOf' \
                and toks[i + 3] == '(':
            end = block_end(toks, i + 3)
            if end is not None:
                toks = toks[:end] + toks[end + 1:]
                i += 4
                continue
        out.append(toks[i])
        i += 1
    return out


def unseen_class_overload(old, new):
    """unseen_class_overload: arguments given a cast to their parameter type, or their unboxing or boxing call
    written out."""
    a, b = tokens(old), tokens(new)
    return a != b and strip_casts(jw3_unboxed(b)) == strip_casts(jw3_unboxed(a))


JW3_FIELD_READ = re.compile(r'(?:[\w$]+\s*\.\s*)+[\w$]+')
JW3_ASSIGN_EXPR = re.compile(r'(?:\(\s*|=\s*)([\w$]+)\s*=\s*((?:[\w$]+\s*\.\s*)+[\w$]+)\s*(?=[)=;<>!])')
JW3_KT_LOCAL = re.compile(r'^\s*(?:val|var) ([\w$`]+)(?:: [^=]+)? = ((?:[\w$`]+\s*\.\s*)+[\w$`]+)\s*$')
JW3_FLOW_WORDS = {'while', 'true', 'if', 'break', 'return', 'continue', 'else', 'for', 'do', 'int', 'long', 'boolean',
                  'val', 'var'}


JW3_TYPE_HEADER = re.compile(r'(?:^|\s)(?:class|interface|enum|record|object)(?:\s|$)|^\s*companion object')


def jw3_method(lines, k):
    """The [start, end) of the member (method, constructor, initializer) of `lines` around line k: the
    outermost brace block around it that does not open a class, or None."""
    stack, best = [], None
    for n, line in enumerate(lines):
        for t in _tokens((line,)):
            if t == '{':
                stack.append(n)
            elif t == '}' and stack:
                opener = stack.pop()
                if opener <= k <= n and not JW3_TYPE_HEADER.search(lines[opener]):
                    if best is None or opener < best[0]:
                        best = (opener, n + 1)
    return best


def jw3_count(text, expr):
    return len(re.findall(r'(?<![\w$.])' + re.escape(expr).replace(r'\.', r'\s*\.\s*') + r'(?![\w$(])', text))


def field_read_reuse_scope(a, b, i1, i2, j1, j2, throw_names):
    """field_read_reuse: in the method around the hunk, a field read Vineflower repeated at a use of the value
    the bytecode reads once is read once: `x = E; if (E ..)` became `if ((x = E) ..)` (or `a = b = E`), and
    Kotlin's unused `val x = E` copy is gone; no other name changes."""
    ma, mb = jw3_method(a, i1), jw3_method(b, j1)
    if ma is None or mb is None:
        return False
    old, new = '\n'.join(a[ma[0]:ma[1]]), '\n'.join(b[mb[0]:mb[1]])
    exprs = [m.group(2) for m in JW3_ASSIGN_EXPR.finditer(new)]
    dropped = [line for line in a[ma[0]:ma[1]] if JW3_KT_LOCAL.match(line) and line not in b[mb[0]:mb[1]]]
    gone = [JW3_KT_LOCAL.match(line).group(2) for line in dropped]
    if not exprs and not gone:
        return False
    for e in set(exprs + gone):
        if jw3_count(old, e) != jw3_count(new, e) + exprs.count(e) + gone.count(e):
            return False
    declared = {w for line in dropped for w in WORD.findall(line)}
    words = lambda t: {w for w in WORD.findall(t) if w not in JW3_FLOW_WORDS} - declared
    return words(old) == words(new)

RECOGNISERS['double_constant_literal'] = double_constant_literal
RECOGNISERS['concat_operand_type'] = concat_operand_type
RECOGNISERS['vararg_spread_resolution'] = vararg_spread_resolution
RECOGNISERS['unseen_class_overload'] = unseen_class_overload
CONTEXT_RECOGNISERS['field_read_reuse'] = field_read_reuse_scope


# --- jvm_javac_layout in loop bodies (jloops): a loop whose ifs and exits are laid out anew changes lines in
# several hunks (a `do {` and its `} while (c);`, a `while (c)` that became `while (true)` and an `if (!c)
# break;` further down), so the hunks are judged by their whole method, as javac_layout judges one hunk.

def javac_layout_method(a, b, i1, i2, j1, j2, throw_names):
    """jvm_javac_layout across hunks: the methods around the hunk are javac_layout's same code."""
    try:
        ra, rb = _sl_method_span(a, i1), _sl_method_span(b, j1)
        if not ra or not rb or a[ra[0]].strip() != b[rb[0]].strip():
            return False
        return javac_layout(a[ra[0]:ra[1]], b[rb[0]:rb[1]])
    except (IndexError, TypeError):
        return False


CONTEXT_RECOGNISERS['jvm_javac_layout'] = javac_layout_method



# --- jkmode: Kotlin classes decompiled as Java (split_constructor_call, constructor_signature_alignment, and
# nested_param_rename's declaration of a parameter whose table name is not an identifier). A constructor
# Signature that omits only the outer instance un-fails the methods that create the anonymous objects (failed
# methods, no recogniser). A joined allocation rebuilds its method's statements (an `if` gains an `else`, a
# labelled block that only computed an argument goes, a return is duplicated into each branch), so its hunks are
# judged by their whole method.

SCC_CALL = re.compile(r'([\w$]+)\./\* \$VF: Unable to resugar constructor \*/<init>\(')
SCC_MARKER_WORDS = {'$VF', 'Unable', 'to', 'resugar', 'constructor', 'init'}
SCC_GONE = {'break', 'else', 'return', 'new', 'val', 'var', 'final'}
SCC_ADDED = {'else', 'return', 'as', 'new', 'val', 'var', 'final'}
SCC_SYNTH = re.compile(r'(?:var\d+x*|label\d+)$')


def scc_allocated_type(text, v):
    """The class `v = new T;` (Kotlin `val v: T = T`) allocates without a constructor call in `text`."""
    m = re.search(r'\b' + re.escape(v) + r'\s*=\s*new\s+([\w$.]+);\s*$', text, re.M) \
        or re.search(r'\b' + re.escape(v) + r'(?::\s*[\w$.]+)?\s*=\s*([\w$.]+)\s*$', text, re.M)
    return m.group(1) if m else None


def scc_creations(text, t):
    return len(re.findall(r'(?:\bnew\s+|(?<![\w$.]))' + re.escape(t) + r'(?:<[^()]*>)?\(', text))


def split_constructor_call(old, new):
    """split_constructor_call: the method's `v./* $VF: Unable to resugar constructor */<init>(..)` calls are
    gone, every class `T` their `v = new T;` (Kotlin `val v: T = T`) allocated is created with arguments
    (`new T(..)`, Kotlin `T(..)`) once more than before for each (or more, a return copied into branches), and
    the words that go or come are only the allocated variables, labels and synthetic names, the marker's, and
    `break`/`else`/`return`/`as`."""
    o, n = '\n'.join(old), '\n'.join(new)
    calls = SCC_CALL.findall(o)
    if not calls or SCC_CALL.search(n):
        return False
    types = [scc_allocated_type(o, v) for v in calls]
    if None in types:
        return False
    for t in set(types):
        if scc_creations(n, t) < scc_creations(o, t) + types.count(t):
            return False
    wo, wn = set(WORD.findall(o)), set(WORD.findall(n))
    gone = {w for w in wo - wn if w not in set(calls) | SCC_MARKER_WORDS | SCC_GONE and not SCC_SYNTH.match(w)}
    added = {w for w in wn - wo if w not in SCC_ADDED and not SCC_SYNTH.match(w)}
    return not gone and not added


def split_constructor_call_scope(a, b, i1, i2, j1, j2, throw_names):
    """split_constructor_call across hunks: the methods around the hunk are split_constructor_call's change."""
    ra, rb = _sl_method_span(a, i1), _sl_method_span(b, j1)
    if not ra or not rb or a[ra[0]].strip() != b[rb[0]].strip():
        return False
    return split_constructor_call(a[ra[0]:ra[1]], b[rb[0]:rb[1]])


NPR_NAME = re.compile(r' ([^\s,()]+)(?=[,)])')
NPR_INVALID = re.compile(r'<[^<>\s,()]*>|[\w$]*-[\w$-]*')


def npr_renamed(o, n):
    """Whether text `n` is `o` with every parameter name of `o` that is no identifier replaced by a `varN`."""
    pattern, at, replaced = [], 0, 0
    for m in NPR_NAME.finditer(o):
        if not NPR_INVALID.fullmatch(m.group(1)):
            continue
        pattern += [re.escape(o[at:m.start(1)]), r'var\d+']
        at, replaced = m.end(1), replaced + 1
    return replaced > 0 and re.fullmatch(''.join(pattern) + re.escape(o[at:]), n) is not None


def npr_flat(lines):
    """A header's lines as one line: a wrapped parameter list joined, its line breaks dropped."""
    return re.sub(r'\(\s+', '(', re.sub(r'\s+\)', ')', ' '.join(x.strip() for x in lines)))


def nested_param_invalid_name(old, new):
    """nested_param_rename: parameters whose table names are not identifiers (`String <set-?>`, Kotlin's
    `int[] $v$c$kotlin-UIntArray$-$this$f$0`) declared under the names their uses are written by (`var1`),
    nothing else; a header the shorter names let fit on one line is joined."""
    if old == new or not old or not new:
        return False
    if len(old) == len(new):
        return all(o == n or npr_renamed(o, n) for o, n in zip(old, new))
    return npr_renamed(npr_flat(old), npr_flat(new))


def nested_param_rename_names(old, new):
    """nested_param_rename: its renamed declarations, or a parameter `<set-?>` declared as the `var1` its uses read."""
    return nested_param_rename(old, new) or nested_param_invalid_name(old, new)


RECOGNISERS['split_constructor_call'] = split_constructor_call
CONTEXT_RECOGNISERS['split_constructor_call'] = split_constructor_call_scope
RECOGNISERS['nested_param_rename'] = nested_param_rename_names


# ktvalid2 (local_final_writes, kt_not_null_assertion, kt_expression_value_check, kt_static_initializer,
# kt_object_instance_local, kt_property_initializer_key, kt_synchronized_accessor): both sides of a hunk written
# alike (kv2_canon: no `@get:Synchronized` lines, no kotlinc null checks and no `!!`, `var` declarations as
# `val`, `@JvmStatic fun {` as `init {`, a stack variable read once replaced by its value, an object's
# instance local by its class, a static initializer's leading property stores as those properties'
# initializers, `init` blocks compared apart, an initializer the new side adds to a bare property dropped,
# `varN` names numbered by first appearance), then compared token for token (parentheses aside).
KV2_SYNC = re.compile(r'^\s*@(?:get|set):Synchronized$')
KV2_NPE_IF = re.compile(r'^(\s*)if \((.+) == null\) \{$')
KV2_NPE_CALL = re.compile(r'^\s*Intrinsics\.throwNpe\(\)$')
KV2_CHECK = re.compile(r'^\s*Intrinsics\.(?:checkNotNull|checkExpressionValueIsNotNull|checkNotNullExpressionValue)\(.*\)$')
KV2_VAR_DECL = re.compile(r'^(\s*(?:[\w@]+ )*)var (`?[\w$]+`?: )')
KV2_STACK_DECL = re.compile(r'^(\s*)va[lr] (var\d+): [^=]+? = (.+)$')
KV2_INIT_OPEN = re.compile(r'^(\s*)init \{$')
KV2_STATIC_OPEN = re.compile(r'^(\s*)fun \{$')
KV2_INSTANCE_NEW = re.compile(r'^\s*val (var\d+): ([\w.]+) = \2\(\)$')
KV2_ASSIGN = re.compile(r'^\s*(`?[\w$]+`?) = (.+)$')
KV2_MODS = r'(?:(?:public|private|protected|internal|final|open|override|lateinit|const|abstract) )+'
KV2_PROP_BARE = re.compile(r'^(\s*' + KV2_MODS + r'va[lr] (`?[\w$]+`?): [^=]+?)$')
KV2_PROP_INIT = re.compile(r'^(\s*' + KV2_MODS + r'va[lr] (`?[\w$]+`?): .+?)(?: = | by )(.*)$')
KV2_STRING = re.compile(r'"(?:[^"\\]|\\.)*"')


def kv2_depth(text):
    text = KV2_STRING.sub('""', text)
    return sum(text.count(c) for c in '({[') - sum(text.count(c) for c in ')}]')


def kv2_no_assert(line):
    out, last = [], 0
    for m in KV2_STRING.finditer(line):
        out.append(line[last:m.start()].replace('!!', ''))
        out.append(m.group(0))
        last = m.end()
    out.append(line[last:].replace('!!', ''))
    return ''.join(out)


def kv2_simple(expr):
    return re.fullmatch(r'[\w$.`]+(?:\([^()]*\))?(?:\.[\w$`]+(?:\([^()]*\))?)*', expr) is not None


def kv2_lines(text):
    """Pass 1: line-local forms (annotations, null checks, `!!`, var/val) and `@JvmStatic fun {`."""
    out = []
    k = 0
    while k < len(text):
        line = text[k]
        if KV2_SYNC.match(line) or KV2_CHECK.match(line):
            k += 1
            continue
        m = KV2_NPE_IF.match(line)
        if m and k + 2 < len(text) and KV2_NPE_CALL.match(text[k + 1]) and text[k + 2].strip() == '}':
            k += 3
            if k < len(text) and text[k].strip() == '':
                k += 1
            continue
        if line.strip() == '@JvmStatic' and k + 1 < len(text) and KV2_STATIC_OPEN.match(text[k + 1]):
            out.append(text[k + 1].replace('fun {', 'init {'))
            k += 2
            continue
        if line == 'fun {':
            out.append('init {')
            k += 1
            continue
        if line.strip() == '':
            k += 1
            continue
        line = kv2_no_assert(line)
        line = KV2_VAR_DECL.sub(r'\1val \2', line)
        out.append(line)
        k += 1
    return out


def kv2_inline(text):
    """Pass 2: a stack variable declared with a one-line value and read once before it is declared again
    or its block ends is replaced by its value."""
    text = list(text)
    k = 0
    while k < len(text):
        m = KV2_STACK_DECL.match(text[k])
        if not m or kv2_depth(m.group(3)) != 0:
            k += 1
            continue
        indent, name, value = len(m.group(1)), m.group(2), m.group(3)
        pat = re.compile(r'(?<![\w$])' + name + r'(?![\w$])')
        uses, j = [], k + 1
        while j < len(text):
            line = text[j]
            if line.strip() and len(line) - len(line.lstrip()) < indent:
                break
            if KV2_STACK_DECL.match(line) and KV2_STACK_DECL.match(line).group(2) == name:
                break
            if re.match(r'^\s*' + name + r' = ', line):
                uses.append(None)
                break
            uses += [(j, mm) for mm in pat.finditer(line)]
            j += 1
        if len(uses) != 1 or uses[0] is None:
            k += 1
            continue
        j, mm = uses[0]
        repl = value if kv2_simple(value) else '(' + value + ')'
        text[j] = text[j][:mm.start()] + repl + text[j][mm.end():]
        del text[k]
    return text


def kv2_blocks(text, opener):
    """(start, end) line spans of the blocks `opener` opens, end the closing brace's line."""
    spans = []
    for k, line in enumerate(text):
        m = opener.match(line)
        if not m:
            continue
        close = m.group(1) + '}'
        for j in range(k + 1, len(text)):
            if text[j] == close:
                spans.append((k, j))
                break
    return spans


def kv2_objects(text):
    """Pass 3: in a static initializer, `val varN: X = X()` + `INSTANCE = varN` dropped, `varN` read as `X`."""
    text = list(text)
    for start, end in reversed(kv2_blocks(text, KV2_INIT_OPEN)):
        if end - start < 3:
            continue
        m = KV2_INSTANCE_NEW.match(text[start + 1])
        if not m or text[start + 2].strip() != 'INSTANCE = ' + m.group(1):
            continue
        name, cls = m.group(1), m.group(2)
        pat = re.compile(r'(?<![\w$])' + name + r'(?![\w$])')
        body = [pat.sub(cls, x) for x in text[start + 3:end]]
        text[start + 1:end] = body
    return text


def kv2_move(text):
    """Pass 4: a static initializer's top-level one-line `NAME = EXPR` becomes the initializer of a bare
    property declaration `val NAME: T`, from the first such statement on while the block starts with them."""
    text = list(text)
    bare = {}
    for k, line in enumerate(text):
        m = KV2_PROP_BARE.match(line)
        if m and not line.lstrip().startswith(('return', 'if', 'throw')):
            bare.setdefault(m.group(2).strip('`'), []).append(k)
    for start, end in reversed(kv2_blocks(text, KV2_INIT_OPEN)):
        inner = len(text[start]) - len(text[start].lstrip()) + 3
        k = start + 1
        while k < end:
            line = text[k]
            m = KV2_ASSIGN.match(line)
            if not (m and len(line) - len(line.lstrip()) == inner and kv2_depth(m.group(2)) == 0):
                break
            decls = bare.get(m.group(1).strip('`'), [])
            if len(decls) != 1:
                break
            d = decls[0]
            text[d] = text[d] + ' = ' + m.group(2)
            del text[k]
            end -= 1
            bare[m.group(1).strip('`')] = []
        if end == start + 1:
            del text[start:end + 1]
    return text


def kv2_extract(text):
    """Pass 5: every `init` block taken out of the text, kept dedented in order."""
    text = list(text)
    blocks = []
    for start, end in reversed(kv2_blocks(text, KV2_INIT_OPEN)):
        pad = len(text[start]) - len(text[start].lstrip())
        blocks.append([x[pad:] if len(x) >= pad else x.lstrip() for x in text[start:end + 1]])
        del text[start:end + 1]
    return text, sorted(blocks)


def kv2_strip(old, new):
    """Pass 6, on the new text: an initializer of a property the old text declares bare is taken out, with
    its continuation lines."""
    bare = {x.strip() for x in old if KV2_PROP_BARE.match(x)}
    out, k = [], 0
    while k < len(new):
        line = new[k]
        m = KV2_PROP_INIT.match(line)
        if m and m.group(1).strip() in bare:
            out.append(m.group(1))
            depth = kv2_depth(m.group(3))
            k += 1
            indent = len(line) - len(line.lstrip())
            while k < len(new) and (depth > 0 or (m.group(3) == '' and len(new[k]) - len(new[k].lstrip()) > indent)):
                depth += kv2_depth(new[k])
                k += 1
                if m.group(3) == '' and depth <= 0:
                    break
            continue
        out.append(line)
        k += 1
    return out


KV2_IF_ASSIGN = re.compile(r'^(\s*)if \((.+)\) \{$')


def kv2_ternaries(text):
    """`val X: T` + `if (c) { X = a } else { X = b }` (one statement each) -> `val X: T = if (c) a else b`."""
    text = list(text)
    k = 0
    while k + 6 < len(text):
        m = re.match(r'^(\s*)va[lr] (`?[\w$]+`?): ([^=]+)$', text[k])
        c = KV2_IF_ASSIGN.match(text[k + 1])
        if m and c and c.group(1) == m.group(1):
            pad, name = m.group(1), m.group(2)
            a = re.match(r'^\s*' + re.escape(name) + r' = (.+)$', text[k + 2])
            b = re.match(r'^\s*' + re.escape(name) + r' = (.+)$', text[k + 4])
            if a and b and text[k + 3] == pad + '} else {' and text[k + 5] == pad + '}':
                text[k:k + 6] = [f'{pad}val {name}: {m.group(3)} = if ({c.group(2)}) {a.group(1)} else {b.group(1)}']
                continue
        k += 1
    return text


def kv2_rename(text):
    """`varN` names numbered by first appearance."""
    names = {}
    def sub(m):
        return names.setdefault(m.group(0), 'v%d' % len(names))
    return [re.sub(r'(?<![\w$])var\d+(?![\w$])', sub, x) for x in text]


def kv2_canon(old, new):
    a = kv2_move(kv2_objects(kv2_inline(kv2_ternaries(kv2_lines(old)))))
    b = kv2_move(kv2_objects(kv2_inline(kv2_ternaries(kv2_lines(new)))))
    a, ia = kv2_extract(a)
    b, ib = kv2_extract(b)
    b = kv2_strip(a, b)
    a = kv2_rename([x for x in a if not x.startswith('import ')])
    b = kv2_rename([x for x in b if not x.startswith('import ')])
    return a, [kv2_rename(x) for x in ia], b, [kv2_rename(x) for x in ib]




KV2_MARK = re.compile(r'!!|Intrinsics\.(?:throwNpe|check)|@(?:get|set):Synchronized|^\s*(?:init|fun) \{$|@JvmStatic')
KV2_DEFAULT = re.compile(r"^(?:0|0L|0\.0|0\.0F|false|null|'\\u0000')$")


def kv2_flipped(old, new):
    """local_final_writes alone: the sides differ only in `val`/`var` declarations."""
    flip = lambda lines: [KV2_VAR_DECL.sub(r'\1val \2', x) for x in lines]
    return old != new and tokens(flip(old)) == tokens(flip(new))


def kv2_initialized(old, new):
    """kt_property_initializer_key alone: the new side gives bare property declarations of the old one an
    initializer that is no default value (those are kt_default_property_initializer's)."""
    bare = {x.strip() for x in old if KV2_PROP_BARE.match(x)}
    added = [m for m in map(KV2_PROP_INIT.match, new) if m and m.group(1).strip() in bare]
    if not added or any(KV2_DEFAULT.match(m.group(3).strip()) for m in added):
        return False
    return tokens(old) == tokens(kv2_strip(old, new))


def kv2_hunk(old, new):
    """ktvalid2: a hunk holding one of these fixes' forms (KV2_MARK) whose two sides are alike once both are
    written alike (kv2_canon), or one that only declares locals `var` or only adds initializers."""
    if all(is_import(x) for x in old + new):
        return False
    if kv2_flipped(old, new) or kv2_initialized(old, new):
        return True
    if not any(KV2_MARK.search(x) for x in old + new):
        return False
    a, ia, b, ib = kv2_canon(old, new)
    if tokens(a) == tokens(b) and ia == ib:
        return True
    plain = lambda toks: [t for t in toks if t not in '()']
    flat = lambda blocks: [x for blk in blocks for x in blk]
    return plain(tokens(a)) == plain(tokens(b)) and plain(tokens(flat(ia))) == plain(tokens(flat(ib)))


RECOGNISERS['ktvalid2'] = kv2_hunk


# --- ktunrep: synchronized blocks and anonymous types (synchronized_break_exit, inline_marker_finally_copy,
# kt_unrepresentable_declared_type). A return inside kotlinc's `synchronized` whose release block was left behind
# moves into the block, and its method's labels and synthetic variables go: judged by the whole method. The
# rebuilt `synchronized` of an inline function's body replaces a semaphore `try`/`finally` and the marker calls of
# the removed `finally` copies: judged by the whole method. A declaration of an anonymous type loses its type.

KU_MONITOR_COMMENTS = re.compile(r'^\s*// \$VF: (?:Could not create synchronized statement|Could not verify finally blocks'
                                 r'|monitorexit)|^\s*// Please report this to the (?:kuna|Vineflower) issue tracker')
KU_RETURN = re.compile(r'^\s*return ([\w$`]+);?$')
KU_STORE = re.compile(r'^\s*([\w$`]+) = (.+?);?$')
KU_DECL_ONLY = re.compile(r'^\s*(?:va[lr] ([\w$`]+): [\w$.<>?, ]+|[\w$.<>\[\]]+ ([\w$]+);)$')
KU_SYNTH = re.compile(r'`?(?:var\d+x*|label\d+)`?$')
KU_COMMENT_WORDS = {'VF', 'monitorexit', 'monitorenter', 'Could', 'not', 'create', 'synchronized', 'statement', 'marking',
                    'monitor', 'enters', 'and', 'exits', 'Please', 'report', 'this', 'to', 'the', 'kuna', 'issue',
                    'tracker', 'at', 'https', 'github', 'com', 'Noelo', 'Lab', 'issues', 'with', 'a', 'copy', 'of',
                    'class', 'file', 'if', 'you', 'have', 'rights', 'distribute', 'it', 'verify', 'finally', 'blocks',
                    'A', 'semaphore', 'variable', 'has', 'been', 'added', 'preserve', 'control', 'flow', 'Semaphore'}


KU_BLOCK_COMMENT = re.compile(r'/\*.*?\*/')
KU_LOCK_COPY = re.compile(r'^\s*(?:val |var |final )?(?:[\w$.<>\[\]]+ )?(`?var\d+x*`?)(?:: [\w$.<>?]+)? = ([\w$.@`]+);?$')


def ku_words(lines, skip_comments=True):
    """The words of `lines` outside `/* .. */` comments (and the monitor notes)."""
    words = collections.Counter()
    for line in lines:
        if skip_comments and KU_MONITOR_COMMENTS.match(line):
            continue
        words.update(WORD.findall(KU_BLOCK_COMMENT.sub('', line)))
    return words


def ku_gone_lock_copies(old, wn):
    """The words of `varN = x` copies (the lock a `synchronized` head now reads directly) whose `varN` is gone."""
    out = set()
    for line in old:
        m = KU_LOCK_COPY.match(line)
        if m and not wn[m.group(1).strip('`')]:
            out.update(WORD.findall(line))
    return out


def ku_decl_only(lines):
    """Names that `lines` mention only in declarations without a value (`var var5: Byte`, `Object x;`)."""
    declared = set()
    for line in lines:
        m = KU_DECL_ONLY.match(line)
        if m:
            declared.add(m.group(1) or m.group(2))
    used = collections.Counter(w for line in lines if not KU_DECL_ONLY.match(line) for w in WORD.findall(line))
    return {d.strip('`') for d in declared if not used[d.strip('`')]}


def ku_released_returns(lines):
    """The synthetic variables `lines` returns right after a `// $VF: monitorexit`."""
    out, prev = set(), ''
    for line in lines:
        if not line.strip():
            continue
        m = KU_RETURN.match(line)
        if m and KU_SYNTH.match(m.group(1)) and prev.strip() == '// $VF: monitorexit':
            out.add(m.group(1))
        prev = line
    return out


KU_DECL = re.compile(r'^\s*(?:va[lr] ([\w$`]+): ([\w$.<>?, ]+)|([\w$.<>\[\]]+) ([\w$]+);)$')


def ku_gone_declaration_types(old, wn):
    """The words of the declared types of the variables of `old` that the new text no longer names."""
    out = set()
    for line in old:
        m = KU_DECL.match(line)
        if m:
            name, type_ = (m.group(1), m.group(2)) if m.group(1) else (m.group(4), m.group(3))
            if not wn[name.strip('`')]:
                out.update(WORD.findall(type_))
    return out


def synchronized_break_exit(old, new):
    """synchronized_break_exit: the method's `// $VF: monitorexit` releases (and its "Could not create synchronized
    statement" note) are gone; every synthetic `varN` read by a `return varN` is gone and each value stored to it is
    now returned (`return E` once more per store); the words that go are only those variables, labels, the values'
    declarations and `break`/`run`/`var`/`val`, and the words that come are only `return`/`else`/`if`/`run` and labels."""
    if not any('// $VF: monitorexit' in x for x in old) or any(KU_MONITOR_COMMENTS.match(x) for x in new):
        return False
    returned = ku_released_returns(old)
    if not returned:
        return False
    stores = [(m.group(1), m.group(2)) for m in map(KU_STORE.match, old) if m and m.group(1) in returned]
    if not stores:
        return False
    wn = ku_words(new)
    if any(wn[v.strip('`')] for v in returned):
        return False
    o, n = '\n'.join(old), '\n'.join(new)
    for value in {e for _, e in stores}:
        pattern = r'\breturn ' + re.escape(value) + r';?$'
        want = len(re.findall(pattern, o, re.M)) + sum(1 for _, e in stores if e == value)
        if len(re.findall(pattern, n, re.M)) < want:
            return False
    wo = ku_words(old)
    allowed_gone = {v.strip('`') for v in returned} | ku_decl_only(old) | ku_gone_declaration_types(old, wn) \
        | ku_gone_lock_copies(old, wn) | {'break', 'run', 'var', 'val'}
    gone = {w for w in wo if not wn[w] and w not in allowed_gone and not KU_SYNTH.match(w)}
    added = {w for w in wn if not wo[w] and w not in {'return', 'else', 'if', 'run'} and not KU_SYNTH.match(w)}
    return not gone and not added


KU_FUN_HEADER = re.compile(r'^\s*(?:[\w@]+ )*fun .*\{$')


def ku_method_span(lines, i):
    """_sl_method_span, also for a Kotlin `fun` header that names a function type (`(T) -> Boolean`)."""
    for k in range(min(i, len(lines) - 1), -1, -1):
        line = lines[k]
        if (_SL_HEADER.match(line) and not _SL_NOT_HEADER.match(line)) or KU_FUN_HEADER.match(line):
            ind = len(line) - len(line.lstrip())
            for m in range(k + 1, len(lines)):
                x = lines[m]
                if x.strip() and len(x) - len(x.lstrip()) <= ind:
                    if x.strip() == '}' and len(x) - len(x.lstrip()) == ind and m >= i:
                        return k, m + 1
                    break
    return None


def synchronized_break_exit_scope(a, b, i1, i2, j1, j2, throw_names):
    """synchronized_break_exit across hunks: the methods around the hunk are its change, or the hunk is the
    method note "Could not create synchronized statement" (Java) going with the release it was about."""
    if i2 > i1 and j1 == j2 and all(KU_MONITOR_COMMENTS.match(x) for x in a[i1:i2]) \
            and any('Could not create synchronized' in x for x in a[i1:i2]) \
            and not any('Could not verify finally' in x for x in a[i1:i2]):
        return True
    ra, rb = ku_method_span(a, i1), ku_method_span(b, j1)
    if not ra or not rb or a[ra[0]].strip() != b[rb[0]].strip():
        return False
    return synchronized_break_exit(a[ra[0]:ra[1]], b[rb[0]:rb[1]])


KU_CALL = re.compile(r'([\w$]+)\(')
KU_MARKER_CALL = re.compile(r'^\s*InlineMarker\.finally(?:Start|End)\(\d+\);?$')
KU_SEMAPHORE = re.compile(r'/\* VF: Semaphore variable \*/')


def inline_marker_finally_copy(old, new):
    """inline_marker_finally_copy: a method whose `finally` had a semaphore (`/* VF: Semaphore variable */`) has
    none, nor a `// $VF: monitorenter`/`monitorexit`; every call of the old method but the `InlineMarker` ones is
    still made, the words that go are only the semaphore's and the synthetic variables, labels, markers and
    `try`/`finally`/`if`/`break`/`run` and Boolean literals, and the words that come are only `return`, `else`,
    `null`, `synchronized`, `try`, `finally`."""
    if not any(KU_SEMAPHORE.search(x) for x in old) or any(KU_SEMAPHORE.search(x) for x in new):
        return False
    if any('$VF: monitor' in x for x in new):
        return False
    calls_old = {c for x in old if not KU_MARKER_CALL.match(x) for c in KU_CALL.findall(x)}
    calls_new = {c for x in new for c in KU_CALL.findall(x)}
    if not calls_old - {'synchronized'} <= calls_new | {'synchronized'}:
        return False
    wo, wn = ku_words(old), ku_words(new)
    ok_gone = {'InlineMarker', 'finallyStart', 'finallyEnd', 'try', 'finally', 'if', 'break', 'run', 'var', 'val',
               'Boolean', 'boolean', 'true', 'false', 'VF', '$VF', 'Semaphore', 'variable', 'monitorenter', 'monitorexit'}
    ok_gone |= {m.group(1) for x in old for m in [re.search(r'([\w$]+)(?:: Boolean)? = false /\* VF: Semaphore', x)] if m}
    ok_gone |= ku_decl_only(old) | ku_gone_declaration_types(old, wn) | ku_gone_lock_copies(old, wn)
    gone = {w for w in wo if not wn[w] and w not in ok_gone and not KU_SYNTH.match(w)}
    added = {w for w in wn if not wo[w] and w not in {'return', 'else', 'null', 'synchronized', 'try', 'finally'}
             and not KU_SYNTH.match(w)}
    return not gone and not added


def inline_marker_finally_copy_scope(a, b, i1, i2, j1, j2, throw_names):
    """inline_marker_finally_copy across hunks: the methods around the hunk are its change, or the hunk is the
    method notes (Java) about the semaphore and the unbuilt `synchronized` going with them."""
    if i2 > i1 and j1 == j2 and all(KU_MONITOR_COMMENTS.match(x) for x in a[i1:i2]) \
            and any('Could not verify finally' in x for x in a[i1:i2]):
        return True
    ra, rb = ku_method_span(a, i1), ku_method_span(b, j1)
    if not ra or not rb or a[ra[0]].strip() != b[rb[0]].strip():
        return False
    return inline_marker_finally_copy(a[ra[0]:ra[1]], b[rb[0]:rb[1]])


def kt_unrepresentable_declared_type(old, new):
    """kt_unrepresentable_declared_type: line for line, `: <unrepresentable> =` of a declaration became ` =`."""
    if len(old) != len(new) or old == new:
        return False
    return all(o == n or (': <unrepresentable> = ' in o and o.replace(': <unrepresentable> = ', ' = ', 1) == n)
               for o, n in zip(old, new))


RECOGNISERS['synchronized_break_exit'] = synchronized_break_exit
CONTEXT_RECOGNISERS['synchronized_break_exit'] = synchronized_break_exit_scope
RECOGNISERS['inline_marker_finally_copy'] = inline_marker_finally_copy
CONTEXT_RECOGNISERS['inline_marker_finally_copy'] = inline_marker_finally_copy_scope
RECOGNISERS['kt_unrepresentable_declared_type'] = kt_unrepresentable_declared_type



# ktlockexpr (fixes kt_inline_lock_local, synchronized_lock_assignment): the value a `synchronized` locks stays in the
# local the bytecode locks. Both are judged on the whole method around the hunk.

LK_HEAD = re.compile(r'^\s*synchronized \((.+)\) \{$')
LK_KT_DECL = re.compile(r'^\s*(?:va[lr] )?(`?[\w$]+`?)(?:: [^=]+)? = (.+?)$')
LK_JAVA_DECL = re.compile(r'^\s*(?:[\w$.<>\[\]?, ]+ )?([\w$]+) = (.+?);$')


def lk_methods(a, b, i1, j1):
    """The methods around a hunk in a and b, when both have the same header line."""
    ra, rb = ku_method_span(a, i1), ku_method_span(b, j1)
    if not ra or not rb or a[ra[0]].strip() != b[rb[0]].strip():
        return None
    return a[ra[0]:ra[1]], b[rb[0]:rb[1]]


def lk_lock_decls(lines):
    """name -> value of the locals a `synchronized` of `lines` locks, by their declarations or assignments."""
    locks = {m.group(1) for x in lines for m in [LK_HEAD.match(x)] if m}
    out = {}
    for x in lines:
        m = LK_KT_DECL.match(x) or LK_JAVA_DECL.match(x)
        if m and m.group(1) in locks and not LK_HEAD.match(x):
            out.setdefault(m.group(1), m.group(2))
    return out


def kt_inline_lock_local(old, new):
    """kt_inline_lock_local, on a method: line for line, the new line is the old one with the value of a local that
    a `synchronized` of the new method locks (its declaration, unchanged, says which) written as the local; and the
    head of at least one `synchronized` changed so."""
    if len(old) != len(new) or old == new:
        return False
    decls = lk_lock_decls(new)
    head_changed = False
    for o, n in zip(old, new):
        if o == n:
            continue
        if not any(e != name and e in o and o.replace(e, name) == n for name, e in decls.items()):
            return False
        head_changed |= bool(LK_HEAD.match(n))
    return head_changed


def kt_inline_lock_local_scope(a, b, i1, i2, j1, j2, throw_names):
    """kt_inline_lock_local across hunks: the methods around the hunk are its change."""
    methods = lk_methods(a, b, i1, j1)
    return bool(methods) and kt_inline_lock_local(*methods)


def synchronized_lock_assignment(old, new):
    """synchronized_lock_assignment, on a method: the new method is the old one with declarations `T n = e;` added,
    each before a `synchronized` whose head `e` became `n`, or before one that repeats `e` (a call kept)."""
    added = []
    sm = difflib.SequenceMatcher(None, old, new, autojunk=False)
    for tag, i1, i2, j1, j2 in sm.get_opcodes():
        if tag == 'equal':
            continue
        if tag == 'insert' and all(LK_JAVA_DECL.match(x) or LK_KT_DECL.match(x) for x in new[j1:j2]):
            added += new[j1:j2]
            continue
        if tag == 'replace' and i2 - i1 == j2 - j1:
            for o, n in zip(old[i1:i2], new[j1:j2]):
                mo, mn = LK_HEAD.match(o), LK_HEAD.match(n)
                if not mo or not mn or o.replace(mo.group(1), mn.group(1), 1) != n:
                    return False
            continue
        return False
    if not added:
        return False
    decls = {m.group(1): m.group(2) for x in added for m in [LK_JAVA_DECL.match(x) or LK_KT_DECL.match(x)]}
    heads_old = [m.group(1) for x in old for m in [LK_HEAD.match(x)] if m]
    heads_new = [m.group(1) for x in new for m in [LK_HEAD.match(x)] if m]
    if len(heads_old) != len(heads_new):
        return False
    for ho, hn in zip(heads_old, heads_new):
        if ho != hn and decls.get(hn) != ho:
            return False
    return all(e in heads_old for e in decls.values())


def synchronized_lock_assignment_scope(a, b, i1, i2, j1, j2, throw_names):
    """synchronized_lock_assignment across hunks: the methods around the hunk are its change."""
    methods = lk_methods(a, b, i1, j1)
    return bool(methods) and synchronized_lock_assignment(*methods)


CONTEXT_RECOGNISERS['kt_inline_lock_local'] = kt_inline_lock_local_scope
CONTEXT_RECOGNISERS['synchronized_lock_assignment'] = synchronized_lock_assignment_scope


# --- ktfacade fixes: kt_facade_call, kt_extension_call, kt_jvm_name_call, kt_synthetic_accessor_call ---
# Each changes how a call is written, never what it calls: a facade class qualifier dropped or a package written
# for it (F), the receiver moved from the first argument to before the call or written where Vineflower dropped it,
# the dispatch receiver `this`/an object dropped (E), the Kotlin name for the JVM one or a property for its
# accessor's call (J), an accessor's call for the access it makes (A), a cast written on a moved receiver (C). The
# recognisers align the old hunk's tokens with the new one's under those moves only; a hunk passes when every
# token is matched, with the fix's own move among the moves used.

KTF_MODULE_NAME = re.compile(r'^([A-Za-z_]\w*)\$[a-z_][a-z0-9_]*$')
KTF_ACCESSOR = re.compile(r'^access\$(?:(get|set)(\w+?)\$c?p|([A-Za-z_]\w*?)(?:\$\w+)?)$')
KTF_UPPER = re.compile(r'^_*[A-Z][\w$]*$')
KTF_LOWER = re.compile(r'^[a-z_][\w]*$')
KTF_IDENT = re.compile(r'^[A-Za-z_$][\w$]*$')
KTF_KEYWORDS = {'this', 'super', 'return', 'if', 'else', 'when', 'is', 'in', 'as', 'val', 'var', 'fun', 'null', 'true',
                'false', 'throw', 'while', 'for', 'do', 'object', 'class', 'try', 'catch', 'finally', 'break',
                'continue'}
KTF_OPEN = ('(', '[', '{', '${')
KTF_RECEIVER_STOP = {',', ';', '=', '+', '-', '*', '/', '%', '&&', '||', '?', ':', '<', '>', '==', '!=', '<=', '>=',
                     '{', '}', 'return', 'else', 'if', 'in', 'is', 'throw'}


def ktf_tokens(lines):
    """The tokens of some lines for the call-shape alignment: imports and blank lines out, backticks off names,
    a string literal's text one token, the expressions of its `${..}` templates tokens of their own."""
    text = '\n'.join(x for x in lines if not is_import(x))
    text = re.sub(r'/\*.*?\*/', ' ', text)
    text = re.sub(r'`([^`\n]+)`', lambda m: m.group(0) if m.group(1) in KTF_KEYWORDS else m.group(1), text)
    return tuple(ktf_lex(text))


KTF_WORD = re.compile(r"'(?:[^'\\\n]|\\.)*'|`[^`\n]+`|[\w$]+|::|\?\.|!!|&&|\|\||==|!=|<=|>=|\S")


def ktf_lex(text):
    """The tokens of `text` (ktf_tokens)."""
    out, i = [], 0
    while i < len(text):
        c = text[i]
        if c.isspace():
            i += 1
            continue
        if c != '"':
            m = KTF_WORD.match(text, i)
            out.append(m.group(0))
            i = m.end()
            continue
        j, part = i + 1, '"'
        while j < len(text) and text[j] != '"' and text[j] != '\n':
            if text[j] == '\\':
                part += text[j:j + 2]
                j += 2
            elif text.startswith('${', j):
                out.append(part + '"')
                part, depth, k = '"', 1, j + 2
                while k < len(text) and depth:
                    depth += {'{': 1, '}': -1}.get(text[k], 0)
                    k += 1
                out.append('${')
                out.extend(ktf_lex(text[j + 2:k - 1]))
                out.append('}')
                j = k
            else:
                part += text[j]
                j += 1
        out.append(part + '"')
        i = j + 1
    return out


def ktf_property(getter):
    """The property names a JVM getter or setter name may stand for (`getClosed`, `setClosed`, `isEmpty`, an internal
    one's `getHashCode$okio`; `getNCollectors` for `nCollectors` or `NCollectors`), else ()."""
    module = KTF_MODULE_NAME.match(getter)
    if module:
        getter = module.group(1)
    m = re.match(r'^(?:get|set)([A-Z_]\w*)$', getter)
    if m:
        name = m.group(1)
        return tuple(dict.fromkeys((name[0].lower() + name[1:], name)))
    return (getter,) if re.match(r'^is[A-Z]', getter) else ()


def ktf_close(t, k):
    """The index of the token closing the bracket at `t[k]`, else None."""
    pairs = {'(': ')', '[': ']', '{': '}', '${': '}'}
    depth, opener = 0, t[k]
    for x in range(k, len(t)):
        if t[x] == opener or opener == '}' and t[x] == '${':
            depth += 1
        elif t[x] == pairs[opener]:
            depth -= 1
            if depth == 0:
                return x
    return None


def ktf_arg_end(t, k):
    """The index of the `,` or `)` ending the argument that starts at `t[k]` (depth 0); the end of `t` when the
    hunk ends inside the argument list (line matching cut the call); else None."""
    depth = 0
    for x in range(k, len(t)):
        if t[x] in KTF_OPEN:
            depth += 1
        elif t[x] in (')', ']', '}'):
            if depth == 0:
                return x
            depth -= 1
        elif t[x] == ',' and depth == 0:
            return x
    return len(t) if depth == 0 else None


def ktf_receiver_ok(r):
    """Whether tokens `r` can be a receiver written before `.name`: balanced, nothing at depth 0 that ends an
    expression or binds looser than `.`."""
    if not r:
        return False
    depth = 0
    for k, x in enumerate(r):
        if x in KTF_OPEN:
            depth += 1
        elif x in (')', ']', '}'):
            depth -= 1
            if depth < 0:
                return False
        elif depth == 0 and x in KTF_RECEIVER_STOP and not (x == ':' and k > 0 and r[k - 1] == 'object'):
            return False
    return depth == 0


def ktf_uncast(t):
    """`t` without a cast written around or after it: `( E as T )`, `E as T`."""
    t = tuple(t)
    if len(t) > 2 and t[0] == '(' and ktf_close(t, 0) == len(t) - 1:
        inner = t[1:-1]
        if 'as' in inner:
            return ktf_uncast(inner[:inner.index('as')])
    if 'as' in t:
        depth = 0
        for x, tok in enumerate(t):
            if tok in KTF_OPEN:
                depth += 1
            elif tok in (')', ']', '}'):
                depth -= 1
            elif tok == 'as' and depth == 0:
                return t[:x]
    return t


def ktf_rename_ok(old, new):
    """Whether `new` may be the Kotlin name of the JVM name `old` (internal suffix, `@JvmName` shapes)."""
    m = KTF_MODULE_NAME.match(old)
    if m and m.group(1) == new:
        return True
    lo, ln = old.lower(), new.lower()
    return lo.startswith(ln) or ln.startswith(lo) or ln.endswith(lo)


class KtfAligner:
    """Aligns old tokens with new ones under the call-shape moves of the kuna_fix_kt_*_call fixes; `align`
    returns the set of move kinds a successful alignment used, or None."""

    def __init__(self, old, new, kinds, depth=0):
        self.o, self.n, self.kinds, self.depth = tuple(old), tuple(new), kinds, depth
        self.memo = {}

    def sub(self, old, new):
        if self.depth > 6:
            return None if tuple(old) != tuple(new) else frozenset()
        return KtfAligner(old, new, self.kinds, self.depth + 1).align()

    def receiver(self, old_arg, new_receiver):
        """The kinds aligning an old first argument with a new receiver (casts on either side aside)."""
        for a in dict.fromkeys((tuple(old_arg), ktf_uncast(old_arg))):
            for r in dict.fromkeys((tuple(new_receiver), ktf_uncast(new_receiver))):
                candidates = [r]
                if r and r[0] == '(' and ktf_close(r, 0) == len(r) - 1:
                    candidates.append(r[1:-1])
                for c in candidates:
                    got = self.sub(a, c)
                    if got is not None:
                        return got
        return None

    def align(self):
        return self.go(0, 0)

    def go(self, i, j):
        key = (i, j)
        if key in self.memo:
            return self.memo[key]
        self.memo[key] = None
        res = self.step(i, j)
        self.memo[key] = res
        return res

    def step(self, i, j):
        o, n = self.o, self.n
        if i == len(o) and j == len(n):
            return frozenset()
        if i < len(o) and j < len(n) and o[i] == n[j]:
            r = self.go(i + 1, j + 1)
            if r is not None:
                return r
        for kind, attempt in self.moves(i, j):
            if kind not in self.kinds:
                continue
            r = attempt()
            if r is not None:
                return r | {kind}
        return None

    def rest(self, o_rest, n_rest):
        return KtfAligner(o_rest, n_rest, self.kinds, self.depth).align()

    def moves(self, i, j):
        o, n = self.o, self.n
        chain_start_o = i == 0 or o[i - 1] not in ('.', '?.')
        chain_start_n = j == 0 or n[j - 1] not in ('.', '?.')
        oi = o[i] if i < len(o) else None
        nj = n[j] if j < len(n) else None
        # kt_facade_call: a class qualifier of a top-level callable dropped; a package written for it
        dash = oi == '-' and nj != '-' and i + 2 < len(o) and KTF_UPPER.match(o[i + 1]) and o[i + 2] == '.'
        if oi and chain_start_o and (KTF_UPPER.match(oi) or dash) and i + 1 < len(o) and (o[i + 1] == '.' or dash):
            k = i + 1 if dash else i
            while k + 1 < len(o) and KTF_UPPER.match(o[k]) and o[k + 1] == '.':
                k += 2
                if k < len(o) and KTF_IDENT.match(o[k]) and oi != nj:
                    yield 'F', lambda k=k: self.go(k, j)
        if nj == '(' and oi != '(':
            m = ktf_close(n, j)
            inner = n[j + 1:m] if m is not None else ()
            cast = ktf_uncast(inner)
            if m is not None and cast != inner and cast:
                yield 'C', lambda m=m, cast=cast: self.rest(o[i:], cast + n[m + 1:])
        if nj == 'as' and j > 0 and (oi is None or oi != 'as'):
            k = ktf_type_end(n, j + 1)
            if k is not None:
                yield 'C', lambda k=k: self.go(i, k)
        if oi is None or nj is None:
            return
        if oi == 'this' and chain_start_o:
            k = i + 1
            if k + 1 < len(o) and o[k] == '@' and KTF_IDENT.match(o[k + 1]):
                k += 2
            if k + 1 < len(o) and o[k] == '.' and KTF_IDENT.match(o[k + 1]):
                yield 'E', lambda k=k: self.go(k + 1, j)
        if oi == '(' and chain_start_o and i + 2 < len(o) and o[i + 1] == 'this' and o[i + 2] == 'as':
            k = ktf_close(o, i)
            if k is not None and k + 2 < len(o) and o[k + 1] == '.' and KTF_IDENT.match(o[k + 2]):
                yield 'E', lambda k=k: self.go(k + 2, j)
        accessor = KTF_ACCESSOR.match(oi)
        if accessor:
            name = (accessor.group(1) + accessor.group(2)) if accessor.group(1) else accessor.group(3)
            yield 'A', lambda name=name: self.rest((name,) + o[i + 1:], n[j:])
            if i + 3 < len(o) and o[i + 1] == '(' and (o[i + 2] == 'this' or KTF_UPPER.match(o[i + 2])) and \
                    o[i + 3] in (',', ')'):
                after = o[i + 4:] if o[i + 3] == ',' else o[i + 3:]
                yield 'A', lambda name=name, after=after: self.rest((name, '(') + after, n[j:])
        # kt_jvm_name_call: the Kotlin name of a member; a property for its accessor's call
        if KTF_IDENT.match(oi) and KTF_IDENT.match(nj) and oi != nj and ktf_rename_ok(oi, nj) and \
                i + 1 < len(o) and o[i + 1] == '(' and j + 1 < len(n) and n[j + 1] == '(':
            yield 'J', lambda: self.go(i + 1, j + 1)
        if oi == nj and i + 2 < len(o) and o[i + 1] == '(' and o[i + 2] == ')' and \
                (j + 1 >= len(n) or n[j + 1] not in ('(', '=')):
            yield 'J', lambda: self.go(i + 3, j + 1)
        prop = ktf_property(oi)
        if prop and i + 1 < len(o) and o[i + 1] == '(':
            close = ktf_close(o, i + 1)
            if close is not None:
                invoked = close + 1 < len(o) and o[close + 1] == '('
                if close == i + 2 and nj in prop and (j + 1 >= len(n) or n[j + 1] not in ('(', '=') or
                                                      invoked and n[j + 1] == '('):
                    yield 'J', lambda: self.go(close + 1, j + 1)
                if oi.startswith('set') and close > i + 2 and nj in prop and j + 1 < len(n) and n[j + 1] == '=':
                    yield 'J', lambda close=close: self.rest(o[i + 2:close] + o[close + 1:], n[j + 2:])
        # kt_extension_call: the receiver passed as the first argument written before the call
        targs = ktf_type_args(o, i + 1) if KTF_IDENT.match(oi) else None
        if targs is not None and targs < len(o) and o[targs] == '(':
            end = ktf_arg_end(o, targs + 1)
            if end is not None and end > targs + 1:
                arg = o[targs + 1:end]
                type_args = o[i + 1:targs]
                setter = bool(prop) and oi.startswith('set') and end < len(o) and o[end] == ','
                getter = bool(prop) and not oi.startswith('set') and end < len(o) and o[end] == ')'
                for e in range(j + 1, len(n) - 1):
                    if n[e] not in ('.', '?.') or not ktf_receiver_ok(n[j:e]):
                        continue
                    name = n[e + 1]
                    call = e + 2 + len(type_args)
                    if call < len(n) and n[e + 2:call] == type_args and n[call] == '(' and KTF_IDENT.match(name):
                        got = self.receiver(arg, n[j:e])
                        if got is None:
                            continue
                        if end < len(o) and o[end] == ',':
                            r = self.rest(o[end + 1:], n[call + 1:])
                        else:
                            r = self.rest(o[end:], n[call + 1:])
                        if r is not None:
                            yield 'E', lambda got=got, r=r: got | r
                    elif getter and name in prop:
                        got = self.receiver(arg, n[j:e])
                        if got is not None:
                            r = self.rest(o[end + 1:], n[e + 2:])
                            if r is not None:
                                yield 'E', lambda got=got, r=r: got | r
                    elif setter and name in prop and e + 2 < len(n) and n[e + 2] == '=':
                        close = ktf_close(o, i + 1)
                        got = self.receiver(arg, n[j:e])
                        if got is not None and close is not None:
                            r = self.rest(o[end + 1:close] + o[close + 1:], n[e + 3:])
                            if r is not None:
                                yield 'E', lambda got=got, r=r: got | r
        # kt_facade_call: an infix call Vineflower writes once it reads a multifile part's metadata
        if KTF_IDENT.match(oi) and i + 1 < len(o) and o[i + 1] == '(':
            end = ktf_arg_end(o, i + 2)
            close = ktf_close(o, i + 1)
            if end is not None and close is not None and o[end] == ',' and ktf_arg_end(o, end + 1) == close:
                for e in range(j + 1, len(n) - 1):
                    if n[e] != oi or not ktf_infix_operand(n[j:e]):
                        continue
                    left = self.receiver(ktf_unconverted(o[i + 2:end]), ktf_unconverted(n[j:e]))
                    if left is None:
                        continue
                    for f in range(e + 2, len(n) + 1):
                        if not ktf_infix_operand(n[e + 1:f]):
                            continue
                        right = self.receiver(ktf_unconverted(o[end + 1:close]), ktf_unconverted(n[e + 1:f]))
                        if right is None:
                            continue
                        r = self.rest(o[close + 1:], n[f:])
                        if r is not None:
                            yield 'F', lambda left=left, right=right, r=r: left | right | r
        # kt_extension_call: an extension property Vineflower wrote without its receiver
        if (chain_start_o or ktf_dispatch_before(o, i)) and KTF_IDENT.match(oi) and oi not in KTF_KEYWORDS and (i + 1 >= len(o) or o[i + 1] != '('):
            for e in range(j + 1, len(n) - 1):
                if n[e] in ('.', '?.') and n[e + 1] == oi and ktf_receiver_ok(n[j:e]) and \
                        (e + 2 >= len(n) or n[e + 2] != '('):
                    if i + 1 < len(o) and o[i + 1] == '=' and e + 2 < len(n) and n[e + 2] == '=':
                        stmt = ktf_statement_end(o, i + 2)
                        got = self.receiver(o[i + 2:stmt], n[j:e])
                        if got is not None:
                            vend = ktf_statement_end(n, e + 3)
                            r = self.rest(o[stmt:], n[vend:])
                            if r is not None:
                                yield 'E', lambda got=got, r=r: got | r
                    else:
                        yield 'E', lambda e=e: self.go(i + 1, e + 2)
        if nj and chain_start_n and KTF_LOWER.match(nj) and nj not in KTF_KEYWORDS and j + 1 < len(n) and n[j + 1] == '.':
            k = j
            while k + 1 < len(n) and KTF_LOWER.match(n[k]) and n[k] not in KTF_KEYWORDS and n[k + 1] == '.':
                k += 2
                if k < len(n) and KTF_IDENT.match(n[k]) and (k + 1 >= len(n) or n[k + 1] != '.'):
                    yield 'F', lambda k=k: self.go(i, k)


def ktf_type_end(t, k):
    """The index after the type written at `t[k]` (`E`, `List<T>`, `Foo?`), else None."""
    if k >= len(t) or not KTF_IDENT.match(t[k]):
        return None
    k += 1
    while k + 1 < len(t) and t[k] == '.' and KTF_IDENT.match(t[k + 1]):
        k += 2
    k = ktf_type_args(t, k)
    if k is None:
        return None
    if k < len(t) and t[k] == '?':
        k += 1
    return k


def ktf_type_args(t, k):
    """The index after the type arguments `<..>` at `t[k]` (`k` itself when there are none), else None."""
    if k >= len(t) or t[k] != '<':
        return k
    depth = 0
    for x in range(k, len(t)):
        if t[x] == '<':
            depth += 1
        elif t[x] == '>':
            depth -= 1
            if depth == 0:
                return x + 1
        elif not (KTF_IDENT.match(t[x]) or t[x] in (',', '.', '*', '?', 'out', 'in', '(', ')', '->')):
            return None
    return None


def ktf_dispatch_before(t, i):
    """Whether `t[i]` follows a dispatch receiver Vineflower writes for a member extension property it writes
    without its receiver: `this.`, `Companion.`, `(this as X).`."""
    if i < 2 or t[i - 1] != '.':
        return False
    if t[i - 2] == 'this' or KTF_UPPER.match(t[i - 2]):
        return True
    if t[i - 2] != ')':
        return False
    depth = 0
    for x in range(i - 2, -1, -1):
        depth += {')': 1, '(': -1}.get(t[x], 0)
        if depth == 0:
            return x + 2 < len(t) and t[x + 1] == 'this' and t[x + 2] == 'as'
    return False


def ktf_infix_operand(t):
    """Whether tokens `t` can be an operand of an infix call: balanced, without a depth-0 separator."""
    if not t:
        return False
    depth = 0
    for x in t:
        if x in KTF_OPEN:
            depth += 1
        elif x in (')', ']', '}'):
            depth -= 1
            if depth < 0:
                return False
        elif depth == 0 and x in (',', ';', '=', '{', '}', '&&', '||', 'return', 'else'):
            return False
    return depth == 0


def ktf_unconverted(t):
    """`t` without the parentheses around it and a trailing same-type conversion (`.toInt()`), which Vineflower's
    infix writer leaves out."""
    t = tuple(t)
    while len(t) >= 4 and t[-1] == ')' and t[-2] == '(' and re.match(r'^to(Int|Long)$', t[-3]) and t[-4] == '.':
        t = t[:-4]
    while len(t) > 2 and t[0] == '(' and ktf_close(t, 0) == len(t) - 1:
        t = t[1:-1]
    return t


KTF_INFIX_WORDS = {'as', 'is', 'in', 'and', 'or', 'xor', 'shl', 'shr', 'ushr', 'until', 'downTo', 'step', 'to'}


def ktf_statement_end(t, k):
    """The index after the expression starting at `t[k]`: up to a depth-0 token that cannot continue it (an
    identifier right after an operand, the next statement's)."""
    depth = 0
    for x in range(k, len(t)):
        tok = t[x]
        if tok in KTF_OPEN:
            depth += 1
        elif tok in (')', ']', '}'):
            if depth == 0:
                return x
            depth -= 1
        elif depth == 0 and x > k and KTF_IDENT.match(tok) and tok not in KTF_INFIX_WORDS and \
                (KTF_IDENT.match(t[x - 1]) or t[x - 1][:1].isdigit() or t[x - 1][:1] in '"\')]}') and \
                t[x - 1] not in KTF_INFIX_WORDS:
            return x
    return len(t)


KTF_FIX_KINDS = {'kt_facade_call': 'F', 'kt_extension_call': 'E', 'kt_jvm_name_call': 'J',
                 'kt_synthetic_accessor_call': 'A'}


def ktf_kinds(old, new):
    """The call-shape moves that turn the old hunk into the new one, or None."""
    a, b = ktf_tokens(old), ktf_tokens(new)
    if a == b:
        return None
    if len(a) + len(b) > 1200:
        return None
    limit = sys.getrecursionlimit()
    sys.setrecursionlimit(max(limit, 20000))
    try:
        got = KtfAligner(a, b, frozenset('FEJAC')).align()
    except RecursionError:
        got = None
    finally:
        sys.setrecursionlimit(limit)
    return got if got and got - {'C'} else None


def ktf_window_hunks(a, b, i1, i2, j1, j2, reach=160):
    """The differing line ranges of the two texts within `reach` lines around a hunk, in file coordinates:
    line matching of a window, not of the whole files, which the context passes of other fixes ask about
    with texts of their own."""
    x0, y0 = max(0, i1 - reach), max(0, j1 - reach)
    wa, wb = a[x0:i2 + reach], b[y0:j2 + reach]
    return [(x1 + x0, x2 + x0, y1 + y0, y2 + y0) for tag, x1, x2, y1, y2 in
            difflib.SequenceMatcher(None, wa, wb, autojunk=False).get_opcodes() if tag != 'equal']


def kt_call_shapes_context(a, b, i1, i2, j1, j2, throw_names):
    """A call-shape change line matching split: the hunk with up to two neighbouring hunks before or after it, no
    more than 150 equal lines apart (an anonymous object or lambda written in the receiver), is one."""
    around = ktf_window_hunks(a, b, i1, i2, j1, j2)
    before = [h for h in around if h[1] <= i1 and h[3] <= j1][-2:]
    after = [h for h in around if h[0] >= i2 and h[2] >= j2][:2]
    for back in range(0, len(before) + 1):
        for fwd in range(0, len(after) + 1):
            if back + fwd == 0:
                continue
            first = before[len(before) - back] if back else (i1, i2, j1, j2)
            last = after[fwd - 1] if fwd else (i1, i2, j1, j2)
            if last[0] - first[1] > 150 or last[2] - first[3] > 150:
                continue
            if ktf_kinds(a[first[0]:last[1]], b[first[2]:last[3]]):
                return True
    return False


def ktf_has(old, new, kind):
    """Whether the old hunk becomes the new one under the call-shape moves, `kind` among them."""
    kinds = ktf_kinds(old, new)
    return bool(kinds) and kind in kinds


def kt_facade_call(old, new):
    """kt_facade_call: calls and field reads through a facade class written by name or with their package
    (`Util.format(..)` to `format(..)`, `Util.EMPTY_RESPONSE` to `EMPTY_RESPONSE`, `pkg.lib.twice(..)`), and the
    infix calls Vineflower writes once it reads a multifile part's metadata (`RangesKt.until(0, n)` to `0 until
    n`); other call-shape moves alongside."""
    return ktf_has(old, new, 'F')


def kt_extension_call(old, new):
    """kt_extension_call: extension calls written on their receivers (`f(x, a)` to `x.f(a)`, `getP(x)` to
    `x.p`), a dropped receiver written (`p` to `x.p`, `p = x` to `x.p = v`), a member extension's dispatch
    receiver left out; other call-shape moves alongside."""
    return ktf_has(old, new, 'E')


def kt_jvm_name_call(old, new):
    """kt_jvm_name_call: a member call by its Kotlin name (`bump$main()` to `bump()`, `sizeOf()` to `size()`),
    an accessor's call written as its property (`getClosed()` to `closed`, `setClosed(v)` to `closed = v`); other
    call-shape moves alongside."""
    return ktf_has(old, new, 'J')


def kt_synthetic_accessor_call(old, new):
    """kt_synthetic_accessor_call: a synthetic accessor's call written as its access (`access$getX$p(o)` to
    `o.x`, `access$foo(o, a)` to `o.foo(a)`); other call-shape moves alongside."""
    return ktf_has(old, new, 'A')


RECOGNISERS['kt_facade_call'] = kt_facade_call
RECOGNISERS['kt_extension_call'] = kt_extension_call
RECOGNISERS['kt_jvm_name_call'] = kt_jvm_name_call
RECOGNISERS['kt_synthetic_accessor_call'] = kt_synthetic_accessor_call
CONTEXT_RECOGNISERS['ktfacade_calls'] = kt_call_shapes_context


KN_DECL = re.compile(r'^\s*(?:val|var) ([\w$`]+): ')
KN_DEF = re.compile(r'^\s*(?:val|var) ([\w$`]+)(?:: [^=]+?)? = (.+)$')
KN_KEYWORDS = {'val', 'var', 'if', 'else', 'while', 'do', 'for', 'in', 'is', 'as', 'return', 'break', 'continue',
               'throw', 'try', 'catch', 'finally', 'when', 'null', 'true', 'false', 'this', 'super', 'and', 'or',
               'xor', 'shl', 'shr', 'ushr', 'inv', 'run', 'synchronized', 'fun', 'object', 'it'}


KN_CAST_FUNC = re.compile(r' as \((\([^()]*\) -> [^()=,]+)\)\?(?=[\s,)=]|$)')
KN_CAST_Q = re.compile(r'( as [\w.$`]+(?:<[^=]*?>)?)\?(?=[\s,)=]|$)')
KN_DECL_Q = re.compile(r'^(\s*(?:val|var) [\w$`]+: )(?:\((\([^()]*\) -> [^=]+)\)|(.+?))\?( = |$)')


def kn_typed(o, n, decl):
    """`n` is `o` with `?` after types: a local declaration's type (`decl`), or `as` casts' types."""
    if o == n:
        return False
    if decl:
        return KN_DECL_Q.sub(lambda m: m.group(1) + (m.group(2) or m.group(3)) + m.group(4), n, count=1) == o
    return KN_CAST_Q.sub(r'\1', KN_CAST_FUNC.sub(r' as \1', n)) == o


def kn_strip(line):
    """`line` without the `?` kt_nullable_local and kt_nullable_cast add: a local's declared type, a cast's."""
    line = KN_DECL_Q.sub(lambda m: m.group(1) + (m.group(2) or m.group(3)) + m.group(4), line, count=1)
    return KN_CAST_Q.sub(r'\1', KN_CAST_FUNC.sub(r' as \1', line))


def kt_nullable_local(old, new):
    """kt_nullable_local: line for line, a local's declared type gained a `?` (`var s: Segment? = this.head`)."""
    return len(old) == len(new) and old != new and all(o == n or kn_typed(o, n, True) for o, n in zip(old, new))


def kt_nullable_cast(old, new):
    """kt_nullable_cast: line for line, the type of `as` casts gained a `?` (`names.get(key) as String?`)."""
    return len(old) == len(new) and old != new and all(o == n or kn_typed(o, n, False) for o, n in zip(old, new))


def kn_base(name):
    """A local's name without what Vineflower adds to tell clashing names apart (`x`, an inline `$iv`)."""
    name = name.strip('`')
    while name.endswith('$iv'):
        name = name[:-3]
    return name.rstrip('x') or name


def kn_renamed(old, new):
    """The same tokens up to the `?` of types, parentheses, and locals renamed one to one where Vineflower's
    numbering moved (`var12` for `var11`) or its clash renaming did (`vx`, `` `v$iv` `` for `v`)."""
    to, tn = (tokens([kn_strip(x) for x in t]) for t in (old, new))
    to, tn = [x for x in to if x not in ('(', ')', '`')], [x for x in tn if x not in ('(', ')', '`')]
    if len(to) != len(tn):
        return False
    pairs, back = {}, {}
    for a, b in zip(to, tn):
        if a == b:
            continue
        local = re.compile(r'`[^`]+`|[a-z_][\w$]*')
        if not (local.fullmatch(a) and local.fullmatch(b)) or a in KN_KEYWORDS or b in KN_KEYWORDS:
            return False
        if not (kn_base(a) == kn_base(b) or (re.fullmatch(r'var\d+', a) and re.fullmatch(r'var\d+', b))):
            return False
        if pairs.setdefault(a, b) != b or back.setdefault(b, a) != a:
            return False
    return True


def kn_substitute(line, names):
    """`line` with each local of `names` read as its value."""
    for _ in range(4):
        before = line
        for name, value in names.items():
            simple = re.fullmatch(r'[\w$`.@]+(\([^()]*\))?', value) is not None
            pat = r'(?<![\w$.`])' + re.escape(name) + r'(?![\w$`])'
            dotted = value if simple else '(' + value + ')'
            line = re.sub(pat + r'(?=\.)', lambda _m, v=dotted: v, line)
            line = re.sub(pat, lambda _m, v=value: v, line)
        if line == before:
            break
    return line


def kn_hunk(old, new):
    """The ktnullable fixes in one hunk: a variable the bytecode reads at each use is kept and read there
    (kt_field_read_reuse, kt_partial_inline), with the renaming of locals that follows (Vineflower numbers and
    renames the others around a kept one), and `?` after types (kt_nullable_local, kt_nullable_cast), also
    where another fix changed the same lines (the hunk without the `?` is another recogniser's). The new
    lines, each variable they newly declare (`val v: T = e`) read as its value and that declaration dropped,
    are the old lines up to the names of locals and the `?` of types."""
    if old == new:
        return False
    if kn_renamed(old, new):
        return True
    plain = [kn_strip(x) for x in new]
    if plain != new and (plain == old or kt_other_recogniser_accepts(old, plain, 'ktnullable')):
        return True
    old_set = {kn_strip(x).strip() for x in old}
    names, kept = {}, []
    for line in new:
        m = KN_DEF.match(kn_strip(line))
        if m and kn_strip(line).strip() not in old_set:
            names[m.group(1)] = m.group(2).strip()
        else:
            kept.append(line)
    return bool(names) and kn_renamed(old, [kn_substitute(kn_strip(x), names) for x in kept])


def kn_remainder(text):
    """The stripped lines of the `fun {` blocks (a file class's static initializer) of a file."""
    res, inside = [], False
    for x in text:
        if x == 'fun {':
            inside = True
        elif inside and x == '}':
            inside = False
        elif inside:
            res.append(x.strip())
    return res


def kt_file_static_initializer(a, b, i1, i2, j1, j2, throw_names):
    """kt_file_static_initializer: the `fun {` remainder of the old file is gone, and its statements are the
    new properties' initializers (`val x: T = run { ..; value }`, or the value alone): every old line of the
    hunk is the remainder's or a property declaration, every new line a property with an initializer, `run {`
    or `}`, or a remainder line (a value it stored, or a store into a local written as its definition)."""
    block = kn_remainder(a)
    if not block or kn_remainder(b):
        return False
    stored = {x.split(' = ', 1)[1] for x in block if ' = ' in x}
    for x in (y.strip() for y in a[i1:i2]):
        if x and x not in ('fun {', '}') and x not in block \
                and not re.match(r'^(?:@|(?:public |internal |private )?(?:val|var) )', x):
            return False
    for x in (y.strip() for y in b[j1:j2]):
        if not x or x in ('}', 'run {') or x in block or x in stored \
                or re.match(r'^(?:public |internal |private )?(?:val|var) [\w$`]+: .+ = ', x):
            continue
        m = re.match(r'^(?:val|var) ([\w$`]+)(?:: [^=]+?)?( = .*)?$', x)
        if m and (m.group(2) is None or m.group(1) + m.group(2) in block):
            continue
        return False
    return True


def kn_scope(a, b, i1, i2, j1, j2, throw_names):
    """ktnullable across a method: the hunk's new lines read as the old ones once each local the new method
    defines as a plain copy (`val snapshot: Snapshot = entry`, `val v: T? = this.head`) is read as what it
    copies, which is what Vineflower wrote at the uses it inlined it into."""
    rb = ku_method_span(b, j1)
    if not rb or i2 - i1 > 40 or j2 - j1 > 40:
        return False
    names = {}
    for line in b[rb[0]:rb[1]]:
        m = KN_DEF.match(kn_strip(line))
        if m and re.fullmatch(r'[\w$`.@]+', m.group(2).strip()):
            names.setdefault(m.group(1), m.group(2).strip())
    if not names:
        return False
    read = [kn_strip(x) if KN_DEF.match(kn_strip(x)) else kn_substitute(kn_strip(x), names) for x in b[j1:j2]]
    return kn_hunk(a[i1:i2], read) or kn_renamed(a[i1:i2], read)


RECOGNISERS['kt_nullable_local'] = kt_nullable_local
RECOGNISERS['kt_nullable_cast'] = kt_nullable_cast
RECOGNISERS['ktnullable'] = kn_hunk
CONTEXT_RECOGNISERS['kt_file_static_initializer'] = kt_file_static_initializer
CONTEXT_RECOGNISERS['ktnullable'] = kn_scope


# ktvolatile (fixes kt_property_field_flags, kt_jvm_field_setter, kt_inline_heap_read).

KV_FLAG_LINE = re.compile(r'^\s*@(?:delegate:)?(?:Volatile|Transient|JvmSynthetic)$')
KV_PRIVATE_SET = re.compile(r'^\s*private set$')
KV_VAR_DECL = re.compile(r'^\s*(?:(?:public|internal|protected|final|open|override|lateinit) )*var ')
KV_CALL = re.compile(r'([\w$`]+)\(')
KV_KEYWORDS = {'if', 'when', 'while', 'for', 'catch', 'return', 'throw', 'in', 'is', 'as'}
KV_FN_CAST = re.compile(r' as \([^()]*\) -> [\w.$<>?]+')
KV_COMPOUND = re.compile(r'^(\s*)((?:this\.|[\w$`]+\.)*[\w$`]+(?:\[[^\]]*\])?) ([-+*/%]|and|or|xor)= (.*)$')
KV_NORMAL = {}
KV_CAST = re.compile(r' as [\w.$]+(?:<(?:[^<>()]|<[^<>()]*>)*>)?\??')


def kv_uncast(text):
    """`text` without Kotlin casts (nested type arguments and function types included)."""
    return KT_AS_CAST.sub('', KV_CAST.sub('', KV_FN_CAST.sub('', text)))
KV_HEAP = re.compile(r'\[|[\w$`)!]\.`?[A-Za-z_$][\w$]*`?(?![\w$`(])')


def kt_property_field_flags(old, new):
    """kt_property_field_flags: `@Volatile`, `@Transient`, `@JvmSynthetic` lines (`@delegate:` on a delegated
    property) added above a property, with the blank lines Vineflower puts around a property of several lines."""
    return (any(KV_FLAG_LINE.match(x) for x in new)
            and [x for x in old if x.strip()] == [x for x in new if x.strip() and not KV_FLAG_LINE.match(x)])


def kt_jvm_field_setter_scope(a, b, i1, i2, j1, j2, throw_names):
    """kt_jvm_field_setter: the `private set` line under a `var` annotated `@JvmField` is gone (blank lines aside)."""
    old, new = a[i1:i2], b[j1:j2]
    if any(x.strip() for x in new) or not any(KV_PRIVATE_SET.match(x) for x in old):
        return False
    if not all(KV_PRIVATE_SET.match(x) or not x.strip() for x in old):
        return False
    k = i1 - 1
    if k < 0 or not KV_VAR_DECL.match(a[k]):
        return False
    k -= 1
    while k >= 0 and a[k].strip().startswith('@'):
        if a[k].strip() == '@JvmField':
            return True
        k -= 1
    return False


def kv_calls(value):
    """Whether a value calls something (a name followed by `(`, keywords and Kotlin's conversions aside)."""
    value = KT_KEPT_CONV.sub('', kv_uncast(value))
    return any(m.group(1) not in KV_KEYWORDS for m in KV_CALL.finditer(value))


def kv_heap_values(values):
    """Whether kept values call nothing and one of them reads a field or an array element."""
    values = [kv_uncast(v) for v in values]
    return bool(values) and not any(kv_calls(v) for v in values) and any(KV_HEAP.search(v) for v in values)


def kv_normal(lines):
    """A function's text with compound assignments written out and every local whose assignments in it all give
    the same call-free value replaced by that value (assignments dropped), repeated to a fixed point; as tokens
    without parentheses and casts, with the replaced values (name -> value)."""
    text = [KV_COMPOUND.sub(lambda m: f'{m.group(1)}{m.group(2)} = {m.group(2)} {m.group(3)} ({m.group(4)})', x)
            for x in lines]
    done = {}
    stepped = set(re.findall(r'(?:\+\+|--)\s*(`[^`]+`|[\w$]+)|(`[^`]+`|[\w$]+)\s*(?:\+\+|--)', '\n'.join(text)))
    stepped = {x for pair in stepped for x in pair if x}
    for _ in range(6):
        heads = {m.group(2) for x in text for m in [KT_KEPT_HEAD.match(x)] if m} - stepped
        groups = collections.defaultdict(list)
        for start, end, n, v in kt_kept_assignments(text, heads):
            groups[n].append((start, end, v))
        subst = {}
        for n, assigns in groups.items():
            vals = {' '.join(t for t in kt_tokens([kv_uncast(v)]) if t not in ('(', ')')) for _, _, v in assigns}
            v = assigns[0][2]
            if len(vals) == 1 and not kv_calls(v):
                subst[n] = v
        if not subst:
            break
        for _ in range(4):
            subst = {n: kt_kept_sub_text(v, {m: w for m, w in subst.items() if m != n}) for n, v in subst.items()}
        drop = {k for n in subst for start, end, _ in groups[n] for k in range(start, end)}
        kept = [x for k, x in enumerate(text) if k not in drop]
        text = kt_kept_sub_text('\n'.join(kept), subst).split('\n')
        done.update(subst)
    toks = [t for t in kt_tokens([KT_KEPT_CONV.sub('', kv_uncast(x)) for x in text])
            if t not in ('(', ')')]
    return toks, done


KV_JAVA_HEAD = re.compile(r'^(\s*)(?:final )?(?:[\w$.]+(?:<[^=]*>)?(?:\[\])* )?([A-Za-z_$][\w$]*) = (.*?);?$')
KV_JAVA_DECL = re.compile(r'^\s*(?:final )?[\w$.]+(?:<[^=]*>)?(?:\[\])* [A-Za-z_$][\w$]* = ')
KV_JAVA_BARE = re.compile(r'^\s*(?!(?:return|throw|break|continue|yield|case|else) )(?:final )?[\w$.]+(?:<[^=]*>)?(?:\[\])* '
                          r'[A-Za-z_$][\w$]*;$')


KV_JAVA_FOR = re.compile(r'^(\s*)for \((.*)\) \{$')


def kv_java_for_to_while(lines):
    """Java lines with each `for (a; b; c) {` written as `a;` and `while (b) {` with `c;` before its closing
    brace (a kept variable updated in the loop body stops Vineflower from building the `for`)."""
    out = list(lines)
    i = 0
    while i < len(out):
        m = KV_JAVA_FOR.match(out[i])
        parts = []
        if m:
            depth, cur = 0, ''
            for ch in m.group(2):
                if ch in '([{':
                    depth += 1
                elif ch in ')]}':
                    depth -= 1
                if ch == ';' and depth == 0:
                    parts.append(cur.strip())
                    cur = ''
                else:
                    cur += ch
            parts.append(cur.strip())
        if len(parts) != 3:
            i += 1
            continue
        ind = m.group(1)
        end = next((k for k in range(i + 1, len(out)) if out[k].strip() and indent_of(out[k]) <= len(ind)), None)
        if end is None or out[end].strip() != '}':
            i += 1
            continue
        init, cond, update = parts
        new = ([f'{ind}{init};'] if init else []) + [f'{ind}while ({cond or "true"}) {{']
        body = out[i + 1:end] + ([f'{ind}   {update};'] if update else [])
        out[i:end] = new + body
        i += 1
    return out


def kv_seq_normal(lines):
    """A function's tokens with each local, from the statement that assigns it a call-free value on, replaced by
    that value (the assignment dropped), in text order; with the replaced values that read the heap. Kotlin or
    Java (statements ending in `;`), Java casts dropped."""
    java = any(x.rstrip().endswith(';') for x in lines)
    head = KV_JAVA_HEAD if java else KT_KEPT_HEAD
    text = [KV_COMPOUND.sub(lambda m: f'{m.group(1)}{m.group(2)} = {m.group(2)} {m.group(3)} ({m.group(4)})', x)
            for x in lines]
    if java:
        text = kv_java_for_to_while([kt_uncast(x) for x in text])
    stepped = set(re.findall(r'(?:\+\+|--)\s*(`[^`]+`|[\w$]+)|(`[^`]+`|[\w$]+)\s*(?:\+\+|--)', '\n'.join(text)))
    stepped = {x for pair in stepped for x in pair if x}
    params = set(re.findall(r'(`[^`]+`|[\w$]+)(?:: |[,)])', text[0])) if text else set()
    declared = {m.group(2) for x in text for m in [head.match(x)]
                if m and (KV_JAVA_DECL.match(x) if java else x.lstrip().startswith(('val ', 'var ')))}
    env, out, heap, i = {}, [], [], 0
    while i < len(text):
        if (KV_JAVA_BARE if java else KT_KEPT_BARE).match(text[i]):
            i += 1
            continue
        m = head.match(text[i])
        if m and java and m.group(2) in ('return', 'throw', 'new', 'else'):
            m = None
        j = kt_kept_statement(text, i) if m else i + 1
        stmt = '\n'.join(text[i:j])
        sub = kt_kept_sub_text(stmt, env) if env else stmt
        if m and m.group(2) not in stepped:
            value = sub.split(' = ', 1)[1] if ' = ' in sub else ''
            value = value.rstrip()[:-1] if java and value.rstrip().endswith(';') else value
            if value and not kv_calls(value) and not kt_kept_free([value])[m.group(2)]:
                env[m.group(2)] = value
                if KV_HEAP.search(kv_uncast(value)) or any(
                        t not in declared and t not in params and t not in env and KT_NAME.fullmatch(t)
                        and not t[0].isupper() and t not in KV_KEYWORDS and t != 'this' for t in kt_tokens([value])):
                    heap.append(value)
                i = j
                continue
            env.pop(m.group(2), None)
        if m and ' = ' in sub:
            sub = m.group(2) + ' = ' + sub.split(' = ', 1)[1]
        out += [t for t in kt_tokens([KT_KEPT_CONV.sub('', kv_uncast(sub))])
                if t not in ('(', ')', ';')]
        i = j
    return out, heap


def kv_span(lines, k):
    """(k, end) of the member whose header is line k: up to the next line indented no deeper than the header
    that is not a closing brace (Vineflower can indent a function's closing brace too deep after a wrapped
    expression)."""
    ind = indent_of(lines[k])
    for m in range(k + 1, len(lines)):
        x = lines[m]
        if x.strip() and indent_of(x) <= ind and not x.strip().startswith('}'):
            end = m
            while end > k + 1 and not lines[end - 1].strip():
                end -= 1
            return k, end
    return k, len(lines)


KV_ACCESSOR = re.compile(r'^\s*(?:(?:public|private|protected|internal|open|override|final|inline|external) )*(?:get|set)\(')


def kv_member(lines, k):
    """The header line of the innermost function, constructor, init block or accessor around line k, or None."""
    starts = [kt_kept_method(lines, k), ku_method_span(lines, k)]
    starts = [r[0] for r in starts if r and r != (0, len(lines))]
    for i in range(min(k, len(lines) - 1), -1, -1):
        if KV_ACCESSOR.match(lines[i]):
            starts.append(i)
            break
    starts = [i for i in starts if kv_span(lines, i)[1] > k]
    return max(starts) if starts else None


def kv_method_scope(a, b, i1, j1):
    """kt_inline_heap_read on the functions around a hunk: both normalise (kv_normal, else kv_seq_normal, with
    braces or without: a kept declaration turns `else { if` into `else if`) to the same tokens up to numbered
    names, a replaced value reads a field or an array element, and the new function reads the heap no more
    often than the old one."""
    ha, hb = kv_member(a, i1), kv_member(b, j1)
    if ha is None or hb is None:
        return False
    if a[ha].strip() != b[hb].strip():
        heads = [k for k, x in enumerate(b) if x.strip() == a[ha].strip()]
        if not heads:
            return False
        hb = min(heads, key=lambda k: abs(k - j1))
    ra, rb = kv_span(a, ha), kv_span(b, hb)
    if rb[0] > j1 + (ra[1] - i1) or rb[1] < j1:
        return False
    key = (id(a), id(b), ra, rb)
    if key not in KV_NORMAL:
        old, new = a[ra[0]:ra[1]], b[rb[0]:rb[1]]
        (to, vo), (tn, vn) = kv_normal(old), kv_normal(new)
        heap = [n for n, v in list(vo.items()) + list(vn.items())
                if KV_HEAP.search(kv_uncast(v)) or v.strip() == n]
        reads = [len(KV_HEAP.findall('\n'.join(x))) for x in (old, new)]
        same = bool(heap) and kt_kept_same(to, tn) and not any(x.rstrip().endswith(';') for x in old)
        if not same:
            (so, ho), (sn, hn) = kv_seq_normal(old), kv_seq_normal(new)
            same = bool(ho or hn) and (kt_kept_same(so, sn) or kt_kept_same(
                [t for t in so if t not in ('{', '}')], [t for t in sn if t not in ('{', '}')]))
        KV_NORMAL[key] = same and reads[1] <= reads[0]
    return KV_NORMAL[key]


def kv_partial(old, new, values):
    """Whether the old tokens are the new ones with some occurrences of a name of `values` written as one of its
    values (token lists; parentheses are left out of all three)."""
    i = j = 0
    while j < len(new):
        if new[j] in values:
            vt = next((vt for vt in values[new[j]] if old[i:i + len(vt)] == vt), None)
            if vt is not None and (i >= len(old) or old[i] != new[j]):
                i, j = i + len(vt), j + 1
                continue
        if i >= len(old) or old[i] != new[j]:
            return False
        i, j = i + 1, j + 1
    return i == len(old)


def kv_line_tokens(line):
    """A line's tokens without parentheses, casts and conversions, a compound assignment written out."""
    line = KV_COMPOUND.sub(lambda m: f'{m.group(1)}{m.group(2)} = {m.group(2)} {m.group(3)} ({m.group(4)})', line)
    line = KT_KEPT_CONV.sub('', kv_uncast(line))
    return [t for t in kt_tokens([line]) if t not in ('(', ')')]


def kv_partial_lines(a, b, i1, i2, j1, j2):
    """kt_inline_heap_read, line for line: each changed old line is the new one with some uses of a kept local
    (one the new function reads more often, whose values call nothing and read the heap) written as its value."""
    old, new = a[i1:i2], b[j1:j2]
    if len(old) != len(new) or old == new:
        return False
    _, (mb, eb), kept = kt_kept_scope(a, b, i1, j1)
    assigned = kt_kept_assignments(b[mb:eb], kept)
    used = set(kt_tokens(new))
    values = collections.defaultdict(list)
    for _, _, n, v in assigned:
        if n in used and not kv_calls(v):
            values[n].append(v)
    if not values or not kv_heap_values([v for vs in values.values() for v in vs]):
        return False
    vtoks = {n: [kv_line_tokens(v) for v in vs] for n, vs in values.items()}
    return all(o == n or kv_partial(kv_line_tokens(o), kv_line_tokens(n), vtoks) for o, n in zip(old, new))


def kt_inline_heap_read_scope(a, b, i1, i2, j1, j2, throw_names):
    """kt_inline_heap_read: the hunk, alone or with up to three neighbouring hunks on either side, is a run whose
    new text keeps locals whose values call nothing and one of which reads a field or an array element, and the
    kept values written back give the old text (kt_kept_span), or give it line for line where Vineflower copied
    the value into some uses and kept the local for the others (kv_partial_lines)."""
    if kv_partial_lines(a, b, i1, i2, j1, j2) or kv_method_scope(a, b, i1, j1):
        return True
    if KT_KEPT_HUNKS.get('a') is not a or KT_KEPT_HUNKS.get('b') is not b:
        KT_KEPT_HUNKS.clear()
        KT_KEPT_HUNKS.update(a=a, b=b, hunks=[h[1:] for h in difflib.SequenceMatcher(None, a, b, autojunk=False)
                                               .get_opcodes() if h[0] != 'equal'])
    hunks = KT_KEPT_HUNKS['hunks']
    k = next((n for n, h in enumerate(hunks) if h == (i1, i2, j1, j2)), None)
    spans = [(k, k)] if k is None else [(k - x, k + y) for x in range(4) for y in range(4)
                                         if k - x >= 0 and k + y < len(hunks)]
    for lo, hi in spans:
        x1, _, y1, _ = hunks[lo] if lo is not None else (i1, i2, j1, j2)
        _, x2, _, y2 = hunks[hi] if hi is not None else (i1, i2, j1, j2)
        if kt_kept_span(a, b, x1, x2, y1, y2) is None:
            continue
        _, (mb, eb), kept = kt_kept_scope(a, b, x1, y1)
        if kv_heap_values([v for _, _, _, v in kt_kept_assignments(b[mb:eb], kept)]):
            return True
    return False


KV_COMPOSED = {}
KV_OWN = ('kt_property_field_flags', 'kt_jvm_field_setter', 'kt_inline_heap_read')


def kv_jvm_field_set_lines(lines):
    """The indices of the `private set` lines under a `var` annotated `@JvmField`."""
    out = set()
    for k, x in enumerate(lines):
        if KV_PRIVATE_SET.match(x) and k > 0 and KV_VAR_DECL.match(lines[k - 1]):
            j = k - 2
            while j >= 0 and lines[j].strip().startswith('@'):
                if lines[j].strip() == '@JvmField':
                    out.add(k)
                    break
                j -= 1
    return out


def kv_composed(a, b, i1, i2, j1, j2, throw_names):
    """kt_property_field_flags and kt_jvm_field_setter composed with other fixes' changes in one hunk: with the
    field-flag annotations taken out of the new text and the `@JvmField` `private set` lines out of the old one,
    every change left in the hunk's span is another fix's (or blank lines)."""
    key = (id(a), id(b))
    if key not in KV_COMPOSED:
        drop_a = kv_jvm_field_set_lines(a)
        a2 = [x for k, x in enumerate(a) if k not in drop_a]
        b2 = [x for x in b if not KV_FLAG_LINE.match(x)]
        amap = [k - sum(1 for d in drop_a if d < k) for k in range(len(a) + 1)]
        ops = [h for h in difflib.SequenceMatcher(None, a2, b2, autojunk=False).get_opcodes() if h[0] != 'equal']
        KV_COMPOSED.clear()
        KV_COMPOSED[key] = (drop_a, a2, b2, amap, ops, failure_blocks(a2))
    drop_a, a2, b2, amap, ops, marked = KV_COMPOSED[key]
    lo, hi = amap[i1], amap[i2]
    for _, x1, x2, y1, y2 in ops:
        if x2 < lo or x1 > hi or (x1 == x2 == hi and hi > lo):
            continue
        if all(not x.strip() for x in a2[x1:x2] + b2[y1:y2]):
            continue
        kind = classify(a2, b2, x1, x2, y1, y2, marked)
        if kind == 'other':
            kind = next((fix for fix, recognises in CONTEXT_RECOGNISERS.items()
                         if fix not in KV_OWN and recognises(a2, b2, x1, x2, y1, y2, throw_names)), 'other')
        if kind == 'other':
            return False
    return True


def kt_jvm_field_setter_composed(a, b, i1, i2, j1, j2, throw_names):
    """kt_jvm_field_setter: the hunk alone (kt_jvm_field_setter_scope), or a hunk that drops such a line composed
    with other fixes (kv_composed)."""
    if kt_jvm_field_setter_scope(a, b, i1, i2, j1, j2, throw_names):
        return True
    return any(i1 <= k < i2 for k in kv_jvm_field_set_lines(a)) and kv_composed(a, b, i1, i2, j1, j2, throw_names)


def kt_property_field_flags_composed(a, b, i1, i2, j1, j2, throw_names):
    """kt_property_field_flags: a hunk that adds a field-flag annotation composed with other fixes (kv_composed)."""
    return any(KV_FLAG_LINE.match(x) for x in b[j1:j2]) and kv_composed(a, b, i1, i2, j1, j2, throw_names)


RECOGNISERS['kt_property_field_flags'] = kt_property_field_flags
CONTEXT_RECOGNISERS['kt_jvm_field_setter'] = kt_jvm_field_setter_composed
CONTEXT_RECOGNISERS['kt_inline_heap_read'] = kt_inline_heap_read_scope
CONTEXT_RECOGNISERS['kt_property_field_flags'] = kt_property_field_flags_composed


def jg2_casts_added(a, b, before):
    """The token lists differ only by casts added (none dropped or changed), each right after one of the
    tokens `before`."""
    if a == b or strip_casts(a) != strip_casts(b):
        return False
    old_casts = [a[j:cast_at(a, j) + 1] for j in range(len(a)) if cast_at(a, j) is not None]
    new_casts = [i for i in range(len(b)) if cast_at(b, i) is not None]
    added = [i for i in new_casts if b[i:cast_at(b, i) + 1] not in old_casts]
    return bool(added) and len(new_casts) > len(old_casts) and \
        all(i > 0 and b[i - 1] in before for i in added)


def jg2_returned_cast(old, new):
    """external_supertype_arguments / bounded_inference_cast: casts added to returned or assigned
    values (`return (Function<E, E>) X.INSTANCE`, `o = (Ordering<E>) Ordering.natural()`), nothing else
    changed, the lines possibly reflowed."""
    return jg2_casts_added(tokens(old), tokens(new), ('return', '='))


def jg2_single_casts(toks):
    """The tokens with each double cast `(T)(R)` written before an operand reduced to `(T)`."""
    out, i = [], 0
    while i < len(toks):
        if toks[i] == '(' and (not out or not IDENT.match(out[-1])):
            j = cast_type_end(toks, i + 1)
            if j is not None and j + 2 < len(toks) and toks[j] == ')' and toks[j + 1] == '(':
                k = cast_type_end(toks, j + 2)
                if k is not None and k + 1 < len(toks) and toks[k] == ')' and \
                        (IDENT.match(toks[k + 1]) or toks[k + 1] == '('):
                    out.extend(toks[i:j + 1])
                    i = k + 1
                    continue
        out.append(toks[i])
        i += 1
    return out


def argument_generic_cast(old, new):
    """argument_generic_cast: casts, or double casts through a raw class `(Map<String, Object>)(Map) x`,
    added to arguments (right after `(` or `,`), nothing else changed, the lines possibly reflowed."""
    return jg2_casts_added(tokens(old), jg2_single_casts(tokens(new)), ('(', ','))


def jg2_undiamond(toks):
    """The tokens with the type arguments of every creation `new C<...>(` or diamond `new C<>(` dropped,
    as if it were raw."""
    out, i = [], 0
    while i < len(toks):
        if toks[i] == '<' and out and IDENT.match(out[-1]):
            k = len(out) - 1
            while k >= 2 and out[k - 1] == '.' and IDENT.match(out[k - 2]):
                k -= 2
            if k >= 1 and out[k - 1] == 'new':
                depth, j = 0, i
                while j < len(toks):
                    depth += {'<': 1, '>': -1}.get(toks[j], 0)
                    j += 1
                    if depth == 0:
                        break
                if j < len(toks) and toks[j] == '(':
                    i = j
                    continue
        out.append(toks[i])
        i += 1
    return out


def receiver_type_arguments(old, new):
    """receiver_type_arguments: the call or creation a chain starts with got explicit type arguments,
    `List.<Setting<?>>of(x)` for `List.of(x)`, `new Builder<E>(c)` for `new Builder<>(c)`, nothing else
    changed, the lines possibly reflowed."""
    a, b = tokens(old), tokens(new)
    if a == b:
        return False
    stripped = without_type_arguments(b, False) or b
    return jg2_undiamond(a) == jg2_undiamond(stripped)


RECOGNISERS['external_supertype_arguments'] = jg2_returned_cast
RECOGNISERS['bounded_inference_cast'] = jg2_returned_cast
RECOGNISERS['receiver_type_arguments'] = receiver_type_arguments
RECOGNISERS['argument_generic_cast'] = argument_generic_cast


JW4_WIDENING = {'long', 'float', 'double'}
JW4_UNBOXING = {'booleanValue', 'byteValue', 'shortValue', 'intValue', 'longValue', 'floatValue', 'doubleValue',
                'charValue'}
JW4_LABEL = re.compile(r'label\d+')


def jw4_inserted(a, b):
    """The (index in b, token) pairs b adds to the tokens a, if b is a with tokens inserted only, else None."""
    out = []
    for tag, i1, i2, j1, j2 in difflib.SequenceMatcher(None, a, b, autojunk=False).get_opcodes():
        if tag == 'equal':
            continue
        if tag != 'insert':
            return None
        out += [(j, b[j]) for j in range(j1, j2)]
    return out


def implicit_widening_arithmetic(old, new):
    """implicit_widening_arithmetic: widening casts `(long)`, `(float)`, `(double)` written (with the
    parentheses Vineflower puts around a cast that is the operand of `-`), nothing else."""
    a, b = tokens(old), tokens(new)
    ins = jw4_inserted(a, b)
    if not ins:
        return False
    toks = [t for _, t in ins]
    casts = [j for j, t in ins if t in JW4_WIDENING]
    return (bool(casts) and set(toks) <= {'(', ')'} | JW4_WIDENING and toks.count('(') == toks.count(')')
            and all(0 < j < len(b) - 1 and b[j - 1] == '(' and b[j + 1] == ')' for j in casts))


def mixed_type_reassociation(old, new):
    """mixed_type_reassociation: parentheses written around the right operand of a `*` or `+`, nothing else."""
    a, b = tokens(old), tokens(new)
    ins = jw4_inserted(a, b)
    if not ins:
        return False
    opens = [j for j, t in ins if t == '(']
    return (bool(opens) and all(t in ('(', ')') for _, t in ins) and len(opens) * 2 == len(ins)
            and all(j > 0 and b[j - 1] in ('*', '+') for j in opens))


def reference_unboxing(old, new):
    """reference_unboxing: unboxing calls `.intValue()` written (the operand parenthesised where it is a
    cast), nothing else."""
    a, b = tokens(old), tokens(new)
    ins = jw4_inserted(a, b)
    if not ins:
        return False
    toks = [t for _, t in ins]
    rest = [t for t in toks if t not in ('(', ')')]
    return (bool(rest) and len(rest) % 2 == 0 and toks.count('(') == toks.count(')')
            and all(rest[k] == '.' and rest[k + 1] in JW4_UNBOXING for k in range(0, len(rest), 2)))


def jw4_method_lines(lines, k):
    """The stripped, non-blank lines of the member around line k, label numbers dropped, or None."""
    span = jw3_method(lines, k)
    if span is None:
        return None
    return [JW4_LABEL.sub('label', x.strip()) for x in lines[span[0]:span[1]] if x.strip()]


def jw4_pairs(lines, first, second):
    return sum(1 for x, y in zip(lines, lines[1:]) if x == first and y == second)


def protected_back_edge(a, b, i1, i2, j1, j2, throw_names):
    """protected_back_edge: in the method around the hunk, `while (true) { try {` became `try { while (true) {`
    (the `finally` moved out with the `try`); the method's lines are the same otherwise, in another order."""
    ma, mb = jw4_method_lines(a, i1), jw4_method_lines(b, j1)
    if ma is None or mb is None or ma == mb:
        return False
    moved = jw4_pairs(ma, 'while (true) {', 'try {') - jw4_pairs(mb, 'while (true) {', 'try {')
    return (moved > 0 and jw4_pairs(mb, 'try {', 'while (true) {') - jw4_pairs(ma, 'try {', 'while (true) {') == moved
            and collections.Counter(ma) == collections.Counter(mb))


JW4_LOOP_HEAD = re.compile(r'^(?:\}\s*)?(?:while \((.*)\)|for \(([^;]*); ([^;]*); ([^;]*)\)) \{$')
JW4_JUMPS = re.compile(r'^(?:break|continue)(?: label)?;$|^label:$|^\{$|^\}$')


def loop_first_if_exit(a, b, i1, i2, j1, j2, throw_names):
    """loop_first_if_exit: in the method around the hunk, a loop `while (C) {` (or `for (I; C; U) {`) that
    Vineflower made of an infinite loop's first if is `while (true) {` with `if (C) {` as its first statement
    again (`I;` and `U;` written as statements); the method's other statements are the same up to the
    `break` and `continue` jumps between them."""
    ma, mb = jw4_method_lines(a, i1), jw4_method_lines(b, j1)
    if ma is None or mb is None or ma == mb:
        return False
    old_rest, swapped = [], 0
    for x in ma:
        m = JW4_LOOP_HEAD.match(x)
        cond = m and (m.group(1) or m.group(3))
        if cond and cond != 'true' and f'if ({cond}) {{' in mb:
            swapped += 1
            old_rest += ['while (true) {', f'if ({cond}) {{']
            if m.group(2):
                old_rest += [m.group(2) + ';', m.group(4) + ';']
            continue
        old_rest.append(x)
    if not swapped:
        return False
    keep = lambda xs: collections.Counter(x for x in xs if not JW4_JUMPS.match(x)
                                          and x not in ('while (true) {', '} else {'))
    return keep(old_rest) == keep(mb)


RECOGNISERS['implicit_widening_arithmetic'] = implicit_widening_arithmetic
RECOGNISERS['mixed_type_reassociation'] = mixed_type_reassociation
RECOGNISERS['reference_unboxing'] = reference_unboxing
CONTEXT_RECOGNISERS['protected_back_edge'] = protected_back_edge
CONTEXT_RECOGNISERS['loop_first_if_exit'] = loop_first_if_exit


ENUM_CONSTANT = re.compile(r'^(\s*)([A-Za-z_$][\w$]*)(\(.*\))?([,;]?)$')


def enum_constant_in_values_array(old, new):
    """enum_constant_in_values_array: enum constants printed bare (`NAME,`) now carry their constructor
    arguments (`NAME(7, "one"),`): the same names in the same order, with the same indent and separator."""
    if len(old) != len(new) or not old:
        return False
    for o, n in zip(old, new):
        mo, mn = ENUM_CONSTANT.match(o), ENUM_CONSTANT.match(n)
        if not mo or not mn or mo.group(3) or not mn.group(3) or mo.group(1, 2, 4) != mn.group(1, 2, 4):
            return False
    return True


RECOGNISERS['enum_constant_in_values_array'] = enum_constant_in_values_array

KT_EXT_PROPERTY_HEADER = re.compile(r'^(\s*(?:@\S+ )*(?:[a-z]+ )*va[lr] )(?:<[^=]*?> )?\S.*?\.([\w`]+)(: .*)$')


def kt_extension_property(old, new):
    """kt_extension_property: a property header gained its receiver and type parameters (`val firstChar: Char` ->
    `val String.firstChar: Char`), and the accessor bodies read the receiver `this` where they read
    `` `$this$firstChar` `` (or the receiver parameter `var0`) and the class's own instance `this@C` where they read
    `this`; lines possibly reflowed, imports the receiver type needs added."""
    def plain(line):
        m = KT_EXT_PROPERTY_HEADER.match(line)
        return m.group(1) + m.group(2) + m.group(3) if m else line
    old_k = [x for x in old if not is_import(x)]
    new_k = [x for x in new if not is_import(x)]
    a, b = tokens(old_k), tokens([plain(x) for x in new_k])
    ra, i, headers = [], 0, False
    while i < len(a):
        if a[i] == '`' and i + 2 < len(a) and a[i + 1].startswith('$this$') and a[i + 2] == '`':
            ra.append('\x00RECV'); i += 3
        elif a[i] == 'var0':
            ra.append('\x00RECV'); i += 1
        elif a[i].startswith('"') and '`$this$' in a[i]:
            ra.append(re.sub(r'`\$this\$[\w$]+`', 'this', a[i])); i += 1
            headers = True
        else:
            ra.append(a[i]); i += 1
    rb, i = [], 0
    while i < len(b):
        if b[i] == 'this' and i + 1 < len(b) and b[i + 1] == '@':
            rb.append('this'); i += 2 + (b[i + 2] == '`' if i + 2 < len(b) else 0)
            while i < len(b) and b[i] != '.' and b[i] != ')' and b[i] != ',':
                i += 1
        elif b[i] == 'this':
            rb.append('\x00RECV'); i += 1
        else:
            rb.append(b[i]); i += 1
    headers = headers or any(plain(x) != x for x in new_k)
    return (headers or '\x00RECV' in ra) and old != new and ra == rb


RECOGNISERS['kt_extension_property'] = kt_extension_property

KT_INTERFACE_COMPANION_FIELD = re.compile(r'^\s*[\w.$`]+ ([\w$`]+) = [\w.$`]+\.`?\$\$INSTANCE`?;$')


def kt_interface_companion_field(old, new):
    """kt_interface_companion_field: an interface's Java-form companion field (`@JvmStatic` and `I.Key Key =
    I.Key.$$INSTANCE;`) is gone; the `companion object` declares it."""
    kept = [x for x in old if x.strip() and x.strip() != '@JvmStatic']
    return (not any(x.strip() for x in new) and len(kept) == 1
            and KT_INTERFACE_COMPANION_FIELD.match(kept[0]) is not None)


RECOGNISERS['kt_interface_companion_field'] = kt_interface_companion_field

KT_OVERLOAD_HEADER = re.compile(r'^\s*(?:[a-z]+ )*fun (?:<[^>]*> )?(?:[\w.<>?, *]+\.)?([\w$`]+)\(.*\)(?:: .+)? \{$')
KT_OVERLOAD_CALL = re.compile(r'^\s*(?:return )?(?:[\w.]+\.)?`?([\w$]+)\$default`?\(.*\)(?: as [\w.<>?, *]+)?$')


def kt_annotation_line(line):
    """An annotation line, or the one-line doc comment Vineflower writes before `@Deprecated`."""
    s = line.strip()
    return s.startswith('@') or (s.startswith('/**') and s.endswith('*/'))


def kt_generated_overload_hidden(old, new):
    """kt_generated_overload_hidden: the overloads kotlinc generated for an `@JvmOverloads` function are gone: each
    removed block is annotations including `@JvmOverloads`, `fun f(..) {`, one call of `f$default(..)` and `}`."""
    if any(x.strip() for x in new):
        return False
    lines = [x.strip() for x in old if x.strip()]
    i, blocks = 0, 0
    while i < len(lines):
        annotations = []
        while i < len(lines) and kt_annotation_line(lines[i]):
            annotations.append(lines[i]); i += 1
        if i + 2 >= len(lines) or '@JvmOverloads' not in annotations:
            return False
        header, call, close = KT_OVERLOAD_HEADER.match(lines[i]), KT_OVERLOAD_CALL.match(lines[i + 1]), lines[i + 2]
        if not header or not call or close != '}' or header.group(1).strip('`') != call.group(1):
            return False
        i += 3
        blocks += 1
    return blocks > 0


RECOGNISERS['kt_generated_overload_hidden'] = kt_generated_overload_hidden

def kt_overload_block_lines(a):
    """The indices of the lines of `a` that make up `@JvmOverloads` overload blocks (annotations, `fun f(..) {`, one
    `f$default(..)` call, `}`)."""
    marked, k = set(), 0
    while k < len(a):
        if not kt_annotation_line(a[k]):
            k += 1
            continue
        s = k
        while k < len(a) and kt_annotation_line(a[k]):
            k += 1
        if k + 2 < len(a) and any(x.strip() == '@JvmOverloads' for x in a[s:k]):
            header, call = KT_OVERLOAD_HEADER.match(a[k]), KT_OVERLOAD_CALL.match(a[k + 1])
            if header and call and a[k + 2].strip() == '}' and header.group(1).strip('`') == call.group(1):
                marked.update(range(s, k + 3))
                k += 3
    return marked


def kt_generated_overload_hidden_scope(a, b, i1, i2, j1, j2, throw_names):
    """kt_generated_overload_hidden when the diff cuts the removed overload blocks elsewhere than at their
    annotations: the hunk adds nothing, and its removal, or the same removal shifted over equal lines (the diff's
    choice among them), takes only blank lines and lines of such blocks."""
    if i1 == i2:
        return False
    marked = kt_overload_block_lines(a)
    if any(x.strip() for x in b[j1:j2]):
        return kt_overload_composed(a, b, i1, i2, j1, j2, marked)
    n = i2 - i1
    for d in range(-n, n + 1):
        s, e = i1 + d, i2 + d
        if s < 0 or e > len(a):
            continue
        if d < 0 and a[s:i1] != a[e:i2] or d > 0 and a[i1:s] != a[i2:e]:
            continue
        if any(k in marked for k in range(s, e)) and all(k in marked or not a[k].strip() for k in range(s, e)):
            return True
    return kt_overload_composed(a, b, i1, i2, j1, j2, marked)


def kt_overload_composed(a, b, i1, i2, j1, j2, marked):
    """A hunk that removes overload blocks next to another fix's change: around the hunk, the old lines without
    the blocks (and the blank line after each) differ from the new ones only by hunks another recogniser takes."""
    lo, hi, blo, bhi = max(0, i1 - 8), min(len(a), i2 + 8), max(0, j1 - 8), min(len(b), j2 + 8)
    if not any(k in marked for k in range(i1, i2)):
        return False
    gone = set(marked)
    for k in sorted(marked):
        if k + 1 in marked:
            continue
        start = k
        while start - 1 in marked:
            start -= 1
        if k + 1 < len(a) and not a[k + 1].strip():
            gone.add(k + 1)
        elif start > 0 and not a[start - 1].strip():
            gone.add(start - 1)
    ra = [a[k] for k in range(lo, hi) if k not in gone]
    rb = b[blo:bhi]
    for tag, x1, x2, y1, y2 in difflib.SequenceMatcher(None, ra, rb, autojunk=False).get_opcodes():
        if tag == 'equal':
            continue
        o, n = ra[x1:x2], rb[y1:y2]
        if all(is_import(x) for x in o + n) or same_tokens(o, n):
            continue
        if not kt_other_recogniser_accepts(o, n, 'kt_generated_overload_hidden'):
            return False
    return True


CONTEXT_RECOGNISERS['kt_generated_overload_hidden'] = kt_generated_overload_hidden_scope

def kt_extension_call_type_argument(old, new):
    """kt_extension_call (a primitive receiver boxed into a generic callee's receiver type parameter): the call on
    the receiver gained the explicit type argument Kotlin needs to pick the callee, `this.coerceIn<Int>(range)`."""
    stripped = [re.sub(r'(\.[\w$`]+)<[\w.]+>\(', r'\1(', x) for x in new]
    return stripped != new and tokens(stripped) == tokens(old)


RECOGNISERS['kt_extension_call_type_argument'] = kt_extension_call_type_argument

def kt_interface_companion_field_scope(a, b, i1, i2, j1, j2, throw_names):
    """kt_interface_companion_field next to another fix's change: around the hunk, the old lines without the
    Java-form companion field (its `@JvmStatic` and the blank line after it) differ from the new ones only by
    hunks another recogniser takes."""
    fields = {k for k in range(max(0, i1 - 2), min(len(a), i2 + 1)) if KT_INTERFACE_COMPANION_FIELD.match(a[k])}
    if not fields:
        return False
    gone = set(fields)
    for k in fields:
        if k > 0 and a[k - 1].strip() == '@JvmStatic':
            gone.add(k - 1)
        if k + 1 < len(a) and not a[k + 1].strip():
            gone.add(k + 1)
    lo, hi, blo, bhi = max(0, i1 - 8), min(len(a), i2 + 8), max(0, j1 - 8), min(len(b), j2 + 8)
    ra = [a[k] for k in range(lo, hi) if k not in gone]
    for tag, x1, x2, y1, y2 in difflib.SequenceMatcher(None, ra, b[blo:bhi], autojunk=False).get_opcodes():
        if tag == 'equal':
            continue
        o, n = ra[x1:x2], b[blo + y1:blo + y2]
        if all(is_import(x) for x in o + n) or same_tokens(o, n):
            continue
        if not kt_other_recogniser_accepts(o, n, 'kt_interface_companion_field'):
            return False
    return True


CONTEXT_RECOGNISERS['kt_interface_companion_field'] = kt_interface_companion_field_scope


SS_DECL = re.compile(r'^(\s*)(?:final\s+)?(?:@[\w$.]+\s+)*[\w$.]+(?:<.*>)?(?:\[\])*\s+([\w$]+)(\s*=\s*.*)?;$')
SS_FOR_DECL = re.compile(r'^(\s*for \()(?:final\s+)?[\w$.]+(?:<.*?>)?(?:\[\])*\s+([\w$]+\s*=)')
SS_KEYWORDS = ('return', 'throw', 'new', 'else', 'case', 'yield', 'break', 'continue', 'assert')


def ss_undeclared(lines):
    """The lines with every declaration made an assignment (`T v = e;` to `v = e;`, `for (T v = e;` to
    `for (v = e;`) and every declaration without an initializer dropped."""
    out = []
    for line in lines:
        m = SS_DECL.match(line)
        if m and line.strip().split()[0] not in SS_KEYWORDS:
            if m.group(3):
                out.append(m.group(1) + m.group(2) + m.group(3) + ';')
            continue
        out.append(SS_FOR_DECL.sub(r'\1\2', line))
    return out


def ss_renamed(a, b):
    """`a`'s tokens with each identifier renamed where `b` has another one in its place, when each name is
    always renamed alike (several split-off variables may become one local) and no renamed name is renamed in
    turn (a swap): None when the token sequences do not align."""
    if len(a) != len(b):
        return None
    names = {}
    for x, y in zip(a, b):
        if x == y:
            continue
        if not (WORD.fullmatch(x) and WORD.fullmatch(y)) or names.get(x, y) != y:
            return None
        names[x] = y
    if any(y in names for y in names.values()):
        return None
    return [names.get(t, t) for t in a]


SS_COMPOUND = ('+', '-', '*', '/', '%', '&', '|', '^')


SS_SHIFTS = (['<', '<'], ['>', '>', '>'], ['>', '>'])


def ss_same_operator(toks, i, op):
    """Whether the expression from `toks[i]` to the statement's end needs no parentheses in `x = x op e`: it has
    no operator at its top level, or only `op`, an associative one."""
    depth, ops = 0, set()
    for t in toks[i:]:
        if t in '([{':
            depth += 1
        elif t in ')]}':
            depth -= 1
        elif depth == 0 and t in (';', ','):
            break
        elif depth == 0 and t in SS_COMPOUND + ('<', '>', '?', ':', '=', '!'):
            ops.add(t)
    return not ops or (len(op) == 1 and op[0] in ('+', '*', '&', '|', '^') and ops == {op[0]})


def ss_expanded(toks, grouped=False):
    """The tokens with each statement (or `for` initializer) `x op= e` written `x = x op e`, or with `grouped`
    `x = x op (e)`."""
    out, i, closing = [], 0, []
    while i < len(toks):
        if WORD.fullmatch(toks[i]) and (not out or out[-1] in (';', '{', '}', '(')):
            op = next(([o] for o in SS_COMPOUND if toks[i + 1:i + 3] == [o, '=']), None) or next(
                (o for o in SS_SHIFTS if toks[i + 1:i + 2 + len(o)] == o + ['=']), None)
            if op:
                group = grouped or not ss_same_operator(toks, i + 2 + len(op), op)
                out += [toks[i], '=', toks[i]] + op + (['('] if group else [])
                closing.append(group)
                i += 2 + len(op)
                continue
        if toks[i] == ';' and closing:
            if closing.pop():
                out.append(')')
        out.append(toks[i])
        i += 1
    return out


def source_scopes(old, new):
    """jvm_source_locals, declaration scopes: with declarations made assignments (or dropped when they have no
    initializer) on both sides, the hunks are the same, up to one-to-one renamings of locals (a variable
    Vineflower split off, `var3`, written as the local it is, or two locals no longer merged; the local
    may then be written `x op= e` where the split-off variable had `x = var3 op e`)."""
    a, b = tokens(ss_undeclared(old)), tokens(ss_undeclared(new))
    if a == b:
        return tokens(old) != tokens(new)
    for grouped in (False, True):
        x, y = ss_expanded(a, grouped), ss_expanded(b, grouped)
        if x == y or ss_renamed(x, y) == y:
            return True
    return False


def source_scopes_context(a, b, i1, i2, j1, j2, throw_names):
    """jvm_source_locals, declaration scopes across hunks: the whole method around the hunk."""
    ra, rb = _sl_method_span(a, i1), _sl_method_span(b, j1)
    if not ra or not rb or a[ra[0]].strip() != b[rb[0]].strip():
        return False
    return source_scopes(a[ra[0]:ra[1]], b[rb[0]:rb[1]]) or source_increments(a[ra[0]:ra[1]], b[rb[0]:rb[1]])


def si_loops(toks):
    """The tokens with each `for (a; c; u) { body }` written `a; while (c) { body u; }`."""
    out, i, pending = [], 0, []
    while i < len(toks):
        if toks[i] == 'for' and i + 1 < len(toks) and toks[i + 1] == '(':
            end = block_end(toks, i + 1)
            parts, depth, cur = [], 0, []
            for t in toks[i + 2:end] if end is not None else []:
                if t in '([{':
                    depth += 1
                elif t in ')]}':
                    depth -= 1
                if t == ';' and depth == 0:
                    parts.append(cur)
                    cur = []
                else:
                    cur.append(t)
            parts.append(cur)
            if end is not None and len(parts) == 3 and end + 1 < len(toks) and toks[end + 1] == '{':
                if parts[0]:
                    out += parts[0] + [';']
                out += ['while', '('] + (parts[1] or ['true']) + [')', '{']
                pending.append((len([t for t in out if t == '{']) - len([t for t in out if t == '}']), parts[2]))
                i = end + 2
                continue
        if toks[i] == '}' and pending and pending[-1][0] == len([t for t in out if t == '{']) - len([t for t in out if t == '}']):
            if pending[-1][1]:
                out += pending[-1][1] + [';']
            pending.pop()
        out.append(toks[i])
        i += 1
    return out


def si_canon(lines, anonymous=False):
    """Tokens with increments dropped: `for` loops written as `while` loops with the update last in the body,
    `++`/`--` operators dropped, then statements left as a bare variable or element (`i;`) dropped; with
    `anonymous`, every name assigned at the start of a statement written `_`."""
    res, toks, i = [], si_loops(tokens(ss_undeclared(lines))), 0
    while i < len(toks):
        if toks[i] in ('+', '-') and i + 1 < len(toks) and toks[i + 1] == toks[i]:
            i += 2
            continue
        res.append(toks[i])
        i += 1
    clean, stmt = [], []
    for t in res:
        stmt.append(t)
        if t in ('{', '}'):
            clean += stmt
            stmt = []
        elif t == ';':
            body = stmt[:-1]
            if not (body and WORD.fullmatch(body[0]) and body[0] not in SS_KEYWORDS
                    and all(WORD.fullmatch(x) or x in ('.', '[', ']') for x in body)):
                clean += stmt
            stmt = []
    clean += stmt
    if anonymous:
        names = {clean[k] for k in range(len(clean) - 1) if WORD.fullmatch(clean[k]) and clean[k + 1] == '='
                 and (k == 0 or clean[k - 1] in (';', '{', '}', '('))}
        clean = ['_' if t in names else t for t in clean]
    return clean


def source_increments(old, new):
    """jvm_source_locals, increments: the texts differ in where `++`/`--` stand (`a[i] = i++;` against
    `a[i] = i; i++;`, `f(--m)` against `m--; f(m)`, a `while` whose last statement folded the update
    against the `for` it is): with every increment dropped and `for` loops written as `while` loops
    they are the same, or the same once kept locals are folded (jvm_source_locals)."""
    if not any('++' in x or '--' in x for x in old + new):
        return False
    a, b = si_canon(old), si_canon(new)
    return a == b or source_locals([' '.join(a)], [' '.join(b)])


def source_increments_method(a, b, i1, i2, j1, j2, throw_names):
    """jvm_source_locals, increments across hunks: the whole method around the hunk, the same once increments
    are dropped (`si_canon`) and the names of locals set aside (a `for` loop's own counter where Vineflower
    reused another local's)."""
    ra, rb = _sl_method_span(a, i1), _sl_method_span(b, j1)
    if not ra or not rb or a[ra[0]].strip() != b[rb[0]].strip():
        return False
    ma, mb = a[ra[0]:ra[1]], b[rb[0]:rb[1]]
    if not any('++' in x or '--' in x for x in ma + mb):
        return False
    return si_canon(ma, True) == si_canon(mb, True)


SI_INC = re.compile(r'^(\s*)(?:(\+\+|--)([\w$.\[\]() +]+)|([\w$.\[\]() +]+)(\+\+|--));$')
SI_ASSIGN = re.compile(r'^(\s*)([\w$.\[\]() +]+) = \2 ([+-]) 1L?;$')


def compound_increment(old, new):
    """compound_assignment_shape, increments: `x.f++;` (or `--`, prefix or postfix) of a field or array element
    became `x.f = x.f + 1;` (or `- 1`) where the bytecode loads the receiver twice; every other line the same."""
    if len(old) != len(new) or old == new:
        return False
    for o, n in zip(old, new):
        if o == n:
            continue
        mo, mn = SI_INC.match(o), SI_ASSIGN.match(n)
        if not mo or not mn:
            return False
        target = (mo.group(3) or mo.group(4)).strip()
        op = mo.group(2) or mo.group(5)
        if target != mn.group(2) or op[0] != mn.group(3) or ('.' not in target and '[' not in target):
            return False
    return True


def switch_filler_labels(old, new):
    """switch_table_fillers, a wide table: `case N:` labels of the default's group dropped, nothing else."""
    return bool(old) and not any(x.strip() for x in new) and all(re.fullmatch(r'\s*case -?\d+:', x) for x in old)


RECOGNISERS['source_scopes'] = source_scopes
RECOGNISERS['switch_filler_labels'] = switch_filler_labels
RECOGNISERS['source_increments'] = source_increments
RECOGNISERS['compound_increment'] = compound_increment
CONTEXT_RECOGNISERS['source_scopes'] = source_scopes_context
CONTEXT_RECOGNISERS['source_increments'] = source_increments_method


KTD_OPEN, KTD_CLOSE = '([{', ')]}'
KTD_MODIFIERS = {'vararg', 'noinline', 'crossinline', 'public', 'private', 'protected', 'internal', 'override',
                 'open', 'final', 'val', 'var'}
KTD_TOKEN = re.compile(r'"(?:[^"\\\n]|\\.)*"|\'(?:[^\'\\\n]|\\.)*\'|[\w$]+|->|==|!=|<=|>=|&&|\|\||\S')
KTD_COMPOSING = []


def ktd_parameters(text_lines):
    """The parameter declarations with defaults in some lines of Kotlin headers: (skeleton, defaults), the
    tokens without each `= value`, and {(parameter name, occurrence): (value tokens, (start, end) of the value
    in the lines joined by newlines)}. A `=` is a default's when the tokens since the last `,` or opening
    bracket of its list hold a `:` (else it is an assignment or a named argument, part of the skeleton) and,
    outside brackets (a parameter line of a wrapped header), start with a name, not `val`/`var` (then None).
    None too when a default does not end before the text does."""
    matches = list(KTD_TOKEN.finditer('\n'.join(text_lines)))
    toks = [m.group(0) for m in matches]
    skeleton, defaults, starts, start, seen, i = [], {}, [], 0, collections.Counter(), 0
    while i < len(toks):
        t = toks[i]
        if t in KTD_OPEN:
            skeleton.append(t)
            starts.append(start)
            start = len(skeleton)
        elif t in KTD_CLOSE:
            skeleton.append(t)
            start = starts.pop() if starts else len(skeleton)
        elif t == ',':
            skeleton.append(t)
            start = len(skeleton)
        elif t == '=' and ':' not in skeleton[start:]:
            skeleton.append(t)
        elif t == '=':
            decl = skeleton[start:]
            plain = [x for x in decl if x not in KTD_MODIFIERS and x != '`']
            if (not starts and (not decl or decl[0] in ('val', 'var'))) or not plain \
                    or not re.match(r'[\w$]+$', plain[0]):
                return None
            j, inner = i + 1, 0
            while j < len(toks) and not (inner == 0 and toks[j] in (',', ')')):
                inner += (toks[j] in KTD_OPEN) - (toks[j] in KTD_CLOSE)
                if inner < 0:
                    return None
                j += 1
            if j == i + 1:
                return None
            key = (plain[0], seen[plain[0]])
            seen[plain[0]] += 1
            defaults[key] = (tuple(toks[i + 1:j]), (matches[i + 1].start(), matches[j - 1].end()))
            i = j
            continue
        else:
            skeleton.append(t)
        i += 1
    return tuple(skeleton), defaults


def ktd_names(old, new):
    """Whether the default `new` is `old` with its variables named as kt_default_value_names names them: a
    receiver's debug name (`$this$f`, `` `$this` ``) as `this`, `this` as `this@C`, a numbered `varN` as a name."""
    i = j = 0
    while i < len(old) and j < len(new):
        if old[i] == 'this' and new[j:j + 2] == ('this', '@') and j + 2 < len(new) \
                and old[i + 1:i + 2] != ('@',):
            i += 1
            j += 3
        elif old[i] == new[j]:
            i += 1
            j += 1
        elif i + 2 < len(old) and old[i] == old[i + 2] == '`' and re.match(r'\$this(\$|$)', old[i + 1]) \
                and new[j] == 'this':
            i += 3
            j += 1
        elif re.match(r'\$this\$', old[i]) and new[j] == 'this':
            i += 1
            j += 1
        elif re.match(r'var\d+$', old[i]) and re.match(r'\w+$', new[j]):
            i += 1
            j += 1
        else:
            return False
    return i == len(old) and j == len(new)


def ktd_uncast(old, new):
    """Whether the default `new` is `old` without its trailing cast (`x as T`, `(x) as T`), names aside."""
    if 'as' not in old:
        return False
    k = len(old) - 1 - old[::-1].index('as')
    operand = old[:k]
    if len(operand) > 2 and operand[0] == '(' and operand[-1] == ')' and ktd_names(operand[1:-1], new):
        return True
    return ktd_names(operand, new)


def ktd_kind(o, n):
    """Which of the default fixes explains the default `o` becoming `n` ('same', a fix id, or None)."""
    if o == n:
        return 'same'
    if o == ('.', '.', '.'):
        return 'kt_delegated_member_default' if n is None else 'kt_default_args_bridge_lookup'
    if o is None or n is None:
        return None
    if ktd_names(o, n):
        return 'kt_default_value_names'
    if ktd_uncast(o, n):
        return 'kt_default_value_coercion_cast'
    return None


def ktd_reverted(old, new, a, b):
    """The lines `new` with each default the default fixes changed put back as `old` (parsed `a`) has it."""
    old_text, new_text = '\n'.join(old), '\n'.join(new)
    for key in sorted(b[1], key=lambda k: b[1][k][1][0], reverse=True):
        (o, (os_, oe)), (n, (ns, ne)) = a[1][key], b[1][key]
        if o != n:
            new_text = new_text[:ns] + old_text[os_:oe] + new_text[ne:]
    return new_text.split('\n')


def ktd_explained(old, new):
    """(a, b) parsed when the hunk's defaults pair up and the default fixes explain each change, at least one."""
    a, b = ktd_parameters(old), ktd_parameters(new)
    if a is None or b is None or set(a[1]) != set(b[1]):
        return None
    kinds = [ktd_kind(a[1][k][0], b[1][k][0]) for k in a[1]]
    if None in kinds or all(k == 'same' for k in kinds):
        return None
    return a, b


def ktd_composed(old, new, a, b):
    """Whether the hunk `old` -> `new`, whose defaults the default fixes explain (parsed `a`, `b`), is the
    other fixes' once each changed default of `new` is put back as `old` has it: every hunk between `old`
    and that text is one the tool classifies as another fix's (a header whose types another fix changes, a
    receiver renamed in the body line that follows)."""
    if KTD_COMPOSING or set(a[1]) != set(b[1]):
        return False
    back = ktd_reverted(old, new, a, b)
    KTD_COMPOSING.append(True)
    try:
        for tag, i1, i2, j1, j2 in difflib.SequenceMatcher(None, old, back, autojunk=False).get_opcodes():
            if tag != 'equal' and classify(old, back, i1, i2, j1, j2, set()) in ('other', 'fixed'):
                return False
    finally:
        KTD_COMPOSING.pop()
    return True


def ktdefaults_composed(a, b, i1, i2, j1, j2, throw_names):
    """The default fixes composed with others where the line matching joined their hunks (a changed header
    no longer anchors the alignment, so a block another fix removed before it shares its hunk): with the
    hunk's changed defaults put back, the whole file is diffed again and every hunk overlapping this one
    must be another fix's (recognised, or by a context recogniser)."""
    if KTD_COMPOSING:
        return False
    old, new = a[i1:i2], b[j1:j2]
    parsed = ktd_explained(old, new)
    if parsed is None:
        return False
    back = ktd_reverted(old, new, *parsed)
    b2 = b[:j1] + back + b[j2:]
    KTD_COMPOSING.append(True)
    try:
        marked = failure_blocks(a)
        hunks = [(classify(a, b2, x1, x2, y1, y2, marked), x1, x2, y1, y2)
                 for tag, x1, x2, y1, y2 in difflib.SequenceMatcher(None, a, b2, autojunk=False).get_opcodes()
                 if tag != 'equal']
        for kind, x1, x2, y1, y2 in reflowed(a, b2, hunks):
            if x2 < i1 or x1 > i2:
                continue
            if kind == 'other' and not any(recognises(a, b2, x1, x2, y1, y2, throw_names)
                                           for fix, recognises in CONTEXT_RECOGNISERS.items()):
                return False
    finally:
        KTD_COMPOSING.pop()
    return True


def ktd_recogniser(fix):
    def recognise(old, new):
        a, b = ktd_parameters(old), ktd_parameters(new)
        if a is None or b is None or not (a[1] or b[1]):
            return False
        keys = set(a[1]) | set(b[1])
        kinds = [ktd_kind(a[1][k][0] if k in a[1] else None, b[1][k][0] if k in b[1] else None) for k in keys]
        if None in kinds or fix not in kinds:
            return False
        return a[0] == b[0] or ktd_composed(old, new, a, b)
    return recognise


def kt_default_args_bridge_lookup(old, new):
    """kt_default_args_bridge_lookup: in function headers, unknown defaults (`= ...`) now have values; the other
    defaults of the hunk are unchanged or changed by the other default fixes."""
    return ktd_recogniser('kt_default_args_bridge_lookup')(old, new)


def kt_default_value_names(old, new):
    """kt_default_value_names: in function headers, defaults name receivers `this`/`this@C` and parameters."""
    return ktd_recogniser('kt_default_value_names')(old, new)


def kt_default_value_coercion_cast(old, new):
    """kt_default_value_coercion_cast: in function headers, defaults lose their trailing cast."""
    return ktd_recogniser('kt_default_value_coercion_cast')(old, new)


def kt_delegated_member_default(old, new):
    """kt_delegated_member_default: in function headers, unknown defaults (`= ...`) are gone."""
    return ktd_recogniser('kt_delegated_member_default')(old, new)


RECOGNISERS['kt_default_args_bridge_lookup'] = kt_default_args_bridge_lookup
RECOGNISERS['kt_default_value_names'] = kt_default_value_names
RECOGNISERS['kt_default_value_coercion_cast'] = kt_default_value_coercion_cast
RECOGNISERS['kt_delegated_member_default'] = kt_delegated_member_default
CONTEXT_RECOGNISERS['ktdefaults_composed'] = ktdefaults_composed


if __name__ == '__main__':
    main()
