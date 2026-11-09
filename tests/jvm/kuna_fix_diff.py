#!/usr/bin/env python3
"""Check that kuna's fixes change only what they may: compare two `kuna decompile-project` trees.

  kuna decompile-project IN.jar -o COMPAT --option jvm_vineflower_compat on
  kuna decompile-project IN.jar -o DEFAULT
  python3 -I tests/jvm/kuna_fix_diff.py COMPAT DEFAULT

Every difference between the trees must be one of: a method that failed in compat mode (its
`// $VF: Couldn't be decompiled` comment block replaced by code), the tracker note of a failure
comment (`jvm_fix_report_to_kuna`), or import lines, or a hunk a fix of wrong output recognises
(RECOGNISERS: the before/after shape of that fix, counted under the fix's name). Anything else is printed
as collateral and the exit status is 1. The counts it prints are the evidence a fix's commit quotes
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
    """The indices of the lines of every failure comment block."""
    marked, inside = set(), False
    for i, line in enumerate(text):
        stripped = line.strip()
        if stripped == MARKER:
            inside = True
        elif not stripped.startswith('//'):
            inside = False
        if inside:
            marked.add(i)
    return marked


def is_import(line):
    s = line.strip()
    return s == '' or s.startswith('import ')


UNREPRESENTABLE = '<unrepresentable>.'
ASSERT_THROW = re.compile(r'\s*throw new AssertionError\(')
WORD = re.compile(r'[A-Za-z_$][\w$]*')
ENUM_HEADER = re.compile(r'^\s*(?:[\w$]+ )*enum [\w$]+(?: implements [^{]*)? \{$')


def anonymous_static_qualifier(old, new):
    """jvm_fix_anonymous_static_qualifier: line for line, `<unrepresentable>.` dropped and nothing else changed."""
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
    """jvm_fix_assert_dead_throw: dead `throw new AssertionError(...)` statements (with the `return;` before
    them) removed and nothing added."""
    return not any(x.strip() for x in new) and assert_throws(old) is not None


RECOGNISERS = {'jvm_fix_anonymous_static_qualifier': anonymous_static_qualifier,
               'jvm_fix_assert_dead_throw': assert_dead_throw}


def enum_body_separator(a, b, i1, i2, j1, j2, throw_names):
    """jvm_fix_enum_body_separator: a `;` line (and a blank line) inserted right after an enum's header."""
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
    """jvm_fix_assert_dead_throw, in a file where it removed throws: a variable the removed throws read is
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


CONTEXT_RECOGNISERS = {'jvm_fix_enum_body_separator': enum_body_separator,
                       'jvm_fix_assert_dead_throw': assert_dead_throw_scope}


def classify(a, b, i1, i2, j1, j2, marked):
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


def main():
    if len(sys.argv) != 3:
        sys.exit(__doc__)
    compat, default = tree(sys.argv[1]), tree(sys.argv[2])
    counts = {'files': 0, 'differing': 0, 'import': 0, 'note': 0, 'fixed hunks': 0, 'other': 0}
    counts.update((fix, 0) for fix in list(RECOGNISERS) + list(CONTEXT_RECOGNISERS))
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
        hunks = [(op, classify(a, b, *op[1:], marked))
                 for op in difflib.SequenceMatcher(None, a, b, autojunk=False).get_opcodes() if op[0] != 'equal']
        throw_names = {w for (_, i1, i2, _, _), kind in hunks if kind == 'jvm_fix_assert_dead_throw'
                       for t in assert_throws(a[i1:i2]) for w in WORD.findall(t.split('(', 1)[1])}
        for (_, i1, i2, j1, j2), kind in hunks:
            if kind == 'other':
                kind = next((fix for fix, recognises in CONTEXT_RECOGNISERS.items()
                             if recognises(a, b, i1, i2, j1, j2, throw_names)), 'other')
            counts['fixed hunks' if kind == 'fixed' else kind] += 1
            if kind == 'other':
                bad.append(f'{rel}:{i1 + 1}: ' + ' | '.join((a[i1:i2] + ['->'] + b[j1:j2])[:6]))
    print(f"files {counts['files']}, differing {counts['differing']}; failed methods: {failed_compat} in compat, "
          f"{failed_default} by default ({failed_compat - failed_default} fixed); hunks: {counts['fixed hunks']} in "
          f"failed methods, {counts['note']} tracker notes, {counts['import']} imports, {counts['other']} other"
          + ''.join(f", {counts[fix]} {fix}" for fix in dict.fromkeys(list(RECOGNISERS) + list(CONTEXT_RECOGNISERS))))
    for line in bad:
        print('COLLATERAL', line)
    sys.exit(1 if bad else 0)


if __name__ == '__main__':
    main()
