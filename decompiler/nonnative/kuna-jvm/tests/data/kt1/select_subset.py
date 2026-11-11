"""Picks the committed subset of a `gen.sh sweep` dump directory: a greedy
cover of every structural feature seen (message shapes, flag values, record
kinds, extensions), cheapest records first, plus a few records per input.

usage: python3 select_subset.py SWEEP_DIR BUDGET_BYTES > subset.txt
"""
import os
import re
import sys

ALWAYS = ('fxutf8!',)


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
        depth = len(line) - len(line.lstrip(' '))
        if s.startswith('raw '):
            name, value = s[4:].split(' ', 1)
            out.add('raw ' + name + ' ' + value[:3])
            continue
        if s.startswith('===') or re.match(r'^-?\d+ ', s):
            continue
        m = re.match(r'^(\S+) ([+-]) (-?\d+)(?: \{(.*)\})?', s)
        if m:
            out.add(m.group(1) + m.group(2))
            if m.group(4):
                for kv in m.group(4).split():
                    out.add(m.group(1) + ':' + kv)
            continue
        m = re.match(r'^(\S+) ([+-]) (\S+)', s)
        if m:
            out.add(m.group(1) + m.group(2) + m.group(3)[:12])
            continue
        m = re.match(r'^(\S+) (\d+)', s)
        if m:
            out.add(m.group(1) + (' 0' if m.group(2) == '0' else ' n'))
            continue
        out.add(s[:40])
        out.add('depth%d' % min(depth, 40))
    return out


def main():
    sweep, budget = sys.argv[1], int(sys.argv[2])
    cands = []
    for f in sorted(os.listdir(sweep)):
        for rec in records(os.path.join(sweep, f)):
            label = rec[0][4:]
            size = sum(len(l) + 1 for l in rec)
            cands.append((label, size, features(rec)))
    chosen, covered, used = [], set(), 0
    for label, size, feats in cands:
        if label.startswith(ALWAYS):
            chosen.append(label)
            covered |= feats
            used += size
    per_input = {}
    while True:
        best, score = None, 0.0
        for c in cands:
            label, size, feats = c
            if label in chosen or used + size > budget:
                continue
            new = len(feats - covered)
            if new and new / size > score:
                best, score = c, new / size
        if best is None:
            break
        chosen.append(best[0])
        covered |= best[2]
        used += best[1]
        per_input[best[0].split('!')[0]] = per_input.get(best[0].split('!')[0], 0) + 1
    for label, size, feats in sorted(cands, key=lambda c: c[1]):
        key = label.split('!')[0]
        if label not in chosen and per_input.get(key, 0) < (8 if key == 'fx8to7' else 6) and used + size <= budget:
            chosen.append(label)
            used += size
            per_input[key] = per_input.get(key, 0) + 1
    for label in sorted(chosen):
        print(label)
    print('selected %d records, %d bytes, %d features' % (len(chosen), used, len(covered)), file=sys.stderr)


main()
