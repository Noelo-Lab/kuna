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
                       'assert_dead_throw': assert_dead_throw_scope}


RECOGNISERS = {
    'external_class_diamond': external_class_diamond,
    'statement_as_expression': statement_as_expression,
    'anonymous_static_qualifier': anonymous_static_qualifier,
    'assert_dead_throw': assert_dead_throw,
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
    counts.update((fix, 0) for fix in list(RECOGNISERS) + ['layout'])
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
        for kind, i1, i2, j1, j2 in reflowed(a, b, hunks):
            counts['fixed hunks' if kind == 'fixed' else kind] += 1
            if kind == 'other':
                bad.append(f'{rel}:{i1 + 1}: ' + ' | '.join((a[i1:i2] + ['->'] + b[j1:j2])[:6]))
    print(f"files {counts['files']}, differing {counts['differing']}; failed methods: {failed_compat} in compat, "
          f"{failed_default} by default ({failed_compat - failed_default} fixed); hunks: {counts['fixed hunks']} in "
          f"failed methods, {counts['note']} tracker notes, {counts['import']} imports, {counts['other']} other"
          + ''.join(f", {counts[fix]} {fix}" for fix in list(RECOGNISERS) + ['layout']))
    for line in bad:
        print('COLLATERAL', line)
    sys.exit(1 if bad else 0)


if __name__ == '__main__':
    main()
