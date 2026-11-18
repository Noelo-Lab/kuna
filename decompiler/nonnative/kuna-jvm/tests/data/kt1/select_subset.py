"""Picks the committed subset of a `gen.sh sweep` dump directory: the fixtures
under classes/ (which the tests also read through `StructClass`), then a greedy
cover of every structural feature seen (message shapes, field presence, flag
values, record kinds, extensions), the most new features per line first.

usage: python3 select_subset.py SWEEP_DIR > subset.txt
"""
import os
import re
import sys


def records(path):
    rec = []
    for line in open(path, encoding='utf-8'):
        line = line.rstrip('\n')
        rec.append(line)
        if line == 'end':
            yield rec
            rec = []


def features(rec):
    out = set()
    for line in rec:
        s = line.strip()
        if s.startswith('raw '):
            name, value = s[4:].split(' ', 1)
            out.add('raw ' + name + ' ' + value[:3])
        elif s.startswith('===') or re.match(r'^-?\d+ ', s):
            continue
        elif m := re.match(r'^(\S+) ([+-]) (?:-?\d+ \{(.*)\})?', s):
            out.add(m.group(1) + m.group(2))
            out.update(m.group(1) + ':' + kv for kv in (m.group(3) or '').split())
        elif m := re.match(r'^(\S+) (\d+)', s):
            out.add(m.group(1) + (' 0' if m.group(2) == '0' else ' n'))
        else:
            out.add(s.split(' ')[0])
            out.add('depth%d' % min(len(line) - len(line.lstrip(' ')), 40))
    return out


def main():
    sweep = sys.argv[1]
    cands = [(rec[0][4:], len(rec), features(rec))
             for f in sorted(os.listdir(sweep)) for rec in records(os.path.join(sweep, f))]
    fixtures = os.listdir(os.path.join(os.path.dirname(os.path.abspath(__file__)), 'classes'))
    chosen = ['fxutf8!pkg/' + name for name in fixtures]
    covered = set().union(*(c[2] for c in cands if c[0] in chosen))
    while True:
        best = max(cands, key=lambda c: len(c[2] - covered) / c[1])
        if not best[2] - covered:
            break
        chosen.append(best[0])
        covered |= best[2]
    print('\n'.join(sorted(chosen)))
    print('selected %d records, %d features' % (len(chosen), len(covered)), file=sys.stderr)


main()
