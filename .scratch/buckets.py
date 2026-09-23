#!/usr/bin/env python3
"""Aggregate the instrumented CASTCENSUS rows into the four buckets."""
import sys, collections, re

def load(files):
    c = collections.Counter()
    for f in files:
        for line in open(f, errors='replace'):
            if not line.startswith('CASTCENSUS\t'):
                continue
            parts = line.rstrip('\n').split('\t')
            n = int(parts[1]); key = tuple(parts[2:])
            c[key] += n
    return c

def bucket(k):
    kind = k[0]
    if kind == 'PRINT':
        if k[1] == 'typecast':
            oc = k[2]
            if oc == 'CPUI_CAST':
                return None            # attributed by the IN/OUT rows instead
            if oc in ('CPUI_INT_SEXT', 'CPUI_INT_ZEXT'):
                return 'EXT'
            if oc == 'CPUI_SUBPIECE':
                return 'TRUNC'
            if oc == 'CPUI_FLOAT_TRUNC':
                return 'FLOAT'
        if k[1] == 'ptr-constant':
            return 'PTRCONST'
        if k[1] == 'int2float':
            return 'FLOAT'
        if k[1] == 'partialsymbol-finalcast':
            return 'PARTIAL'
        if k[1] == 'boolbyte(x2)':
            return 'BOOLBYTE'
        if k[1] == 'switchlabel':
            return 'SWITCH'
        return 'PRINT-other'
    if kind == 'EXT' or kind == 'DEMOTE':
        return None                     # diagnostic rows, not printed casts
    opc = k[1]
    if opc in ('CPUI_INT_ADD', 'CPUI_INT_SUB', 'CPUI_PTRADD', 'CPUI_PTRSUB'):
        return 'PTRARITH'
    if opc == 'CPUI_LOAD' or opc == 'CPUI_STORE':
        return 'MEMACCESS'
    if opc in ('CPUI_CALL', 'CPUI_CALLIND'):
        return 'CALL'
    if opc in ('CPUI_INT_LESS', 'CPUI_INT_LESSEQUAL', 'CPUI_INT_SLESS',
               'CPUI_INT_SLESSEQUAL', 'CPUI_INT_EQUAL', 'CPUI_INT_NOTEQUAL'):
        return 'COMPARE'
    if opc in ('CPUI_INT_ZEXT', 'CPUI_INT_SEXT'):
        return 'EXT-in'
    if opc.startswith('CPUI_FLOAT'):
        return 'FLOAT'
    if opc == 'CPUI_COPY':
        return 'COPY'
    return 'OTHER'

if __name__ == '__main__':
    c = load(sys.argv[1:])
    tot_printed = sum(n for k, n in c.items() if k[0] == 'PRINT')
    b = collections.Counter()
    for k, n in c.items():
        t = bucket(k)
        if t:
            b[t] += n
    print('total PRINTed cast tokens: %d' % tot_printed)
    s = sum(b.values())
    print('attributed: %d' % s)
    for t, n in b.most_common():
        print('  %-12s %6d  %5.1f%%' % (t, n, 100.0 * n / s))
