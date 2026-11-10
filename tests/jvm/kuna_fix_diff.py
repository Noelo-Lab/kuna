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
    'ktprint6_file': ('kt_legacy_constructor_parameter_checks', 'kt_hidden_empty_super_call'),
    'kt_unboxed_cast': ('kt_unboxing_cast_type', 'kt_operator_precedence'),
    'twr_resugar': ('twr_unresolved_resource', 'twr_class_version', 'twr_desugaring_shape'),
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


# --- ktops2 fixes: kt_negated_equality, kt_unboxing_cast_type (with kt_operator_precedence's unwritten unboxing
# calls), kt_binary_numeric_promotion, kt_copied_equality, kt_operator_line_break ---

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


if __name__ == '__main__':
    main()
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

