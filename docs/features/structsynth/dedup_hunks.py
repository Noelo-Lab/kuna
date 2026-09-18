#!/usr/bin/env python3
"""Classify every changed line of a structsynth `decompile-all` before/after pair.

    kuna decompile-all <bin> --option structsynth param > before.c   # main build
    kuna decompile-all <bin> --option structsynth param > after.c    # branch build
    python3 docs/features/structsynth/dedup_hunks.py before.c after.c

Buckets:
  rename       - the line differs only in which struct_N it names
  field-added  - an address that now lands on a member of the reused, larger
                 record instead of past the end of the smaller one
  other        - anything else; printed in full, to be read by hand
"""
import re, sys, difflib, collections

NORM = re.compile(r'\bstruct_\d+\b')
FIELD = re.compile(r'->field_0x[0-9a-f]+|\.field_0x[0-9a-f]+')

def classify(a, b):
    rows = collections.Counter()
    other = []
    sm = difflib.SequenceMatcher(None, a, b, autojunk=False)
    for tag, i1, i2, j1, j2 in sm.get_opcodes():
        if tag == 'equal':
            continue
        old = a[i1:i2]; new = b[j1:j2]
        if tag == 'replace' and len(old) == len(new):
            for o, n in zip(old, new):
                if NORM.sub('S', o) == NORM.sub('S', n) and NORM.search(o):
                    rows['rename'] += 1
                elif len(FIELD.findall(n)) > len(FIELD.findall(o)):
                    rows['field-added'] += 1
                else:
                    rows['other'] += 1
                    other.append((o, n))
            continue
        for o in old:
            rows['other-del'] += 1; other.append((o, ''))
        for n in new:
            rows['other-add'] += 1; other.append(('', n))
    return rows, other

if __name__ == '__main__':
    a = open(sys.argv[1]).read().splitlines()
    b = open(sys.argv[2]).read().splitlines()
    rows, other = classify(a, b)
    print(sys.argv[1].split('/')[-1], dict(rows))
    for o, n in other[:40]:
        print('   OLD', o)
        print('   NEW', n)
