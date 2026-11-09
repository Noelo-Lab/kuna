#!/usr/bin/env python3
"""Check that kuna's fixes change only what they may: compare two `kuna decompile-project` trees.

  kuna decompile-project IN.jar -o COMPAT --option jvm_vineflower_compat on
  kuna decompile-project IN.jar -o DEFAULT
  python3 -I tests/jvm/kuna_fix_diff.py COMPAT DEFAULT

Every difference between the trees must be one of: a method that failed in compat mode (its
`// $VF: Couldn't be decompiled` comment block replaced by code), the tracker note of a failure
comment (`jvm_fix_report_to_kuna`), or import lines. Anything else is printed as collateral and the
exit status is 1. The counts it prints are the evidence a fix's commit quotes (docs/jvm.md, "Fixing a
Vineflower bug").
"""
import difflib
import os
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


def classify(a, b, i1, i2, j1, j2, marked):
    old, new = a[i1:i2], b[j1:j2]
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
        for tag, i1, i2, j1, j2 in difflib.SequenceMatcher(None, a, b, autojunk=False).get_opcodes():
            if tag == 'equal':
                continue
            kind = classify(a, b, i1, i2, j1, j2, marked)
            counts['fixed hunks' if kind == 'fixed' else kind] += 1
            if kind == 'other':
                bad.append(f'{rel}:{i1 + 1}: ' + ' | '.join((a[i1:i2] + ['->'] + b[j1:j2])[:6]))
    print(f"files {counts['files']}, differing {counts['differing']}; failed methods: {failed_compat} in compat, "
          f"{failed_default} by default ({failed_compat - failed_default} fixed); hunks: {counts['fixed hunks']} in "
          f"failed methods, {counts['note']} tracker notes, {counts['import']} imports, {counts['other']} other")
    for line in bad:
        print('COLLATERAL', line)
    sys.exit(1 if bad else 0)


if __name__ == '__main__':
    main()
