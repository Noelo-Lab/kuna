#!/usr/bin/env python3
"""Count call sites whose rendered argument list is shorter than the callee's own
recovered parameter count, from a `kuna decompile-all --json` dump.

Self-consistent metric: kuna itself declares `callee` as taking P parameters
(callee-side recovery is known-good); we count call sites of that same callee
that render with fewer than P arguments.
"""
import json, re, sys

SIG = re.compile(r'^\s*[\w \*]+?[\* ](\w+)\(([^)]*)\)\s*(//.*)?$')
CALL = re.compile(r'(?<![\w>.])([A-Za-z_]\w*)\s*\(')
KEYWORDS = {'if', 'while', 'for', 'switch', 'return', 'sizeof', 'do', 'else'}


def split_args(s):
    """Paren/bracket/quote-aware top-level comma split."""
    out, depth, cur, i, n = [], 0, [], 0, len(s)
    instr = None
    while i < n:
        c = s[i]
        if instr:
            if c == '\\':
                cur.append(c)
                i += 1
                if i < n:
                    cur.append(s[i])
                i += 1
                continue
            if c == instr:
                instr = None
            cur.append(c)
        elif c in '"\'':
            instr = c
            cur.append(c)
        elif c in '([':
            depth += 1
            cur.append(c)
        elif c in ')]':
            depth -= 1
            cur.append(c)
        elif c == ',' and depth == 0:
            out.append(''.join(cur))
            cur = []
        else:
            cur.append(c)
        i += 1
    tail = ''.join(cur)
    if tail.strip() or out:
        out.append(tail)
    return [a for a in out]


def match_close(s, start):
    """Index of the ')' matching the '(' at s[start]."""
    depth, i, n, instr = 0, start, len(s), None
    while i < n:
        c = s[i]
        if instr:
            if c == '\\':
                i += 2
                continue
            if c == instr:
                instr = None
        elif c in '"\'':
            instr = c
        elif c in '([':
            depth += 1
        elif c in ')]':
            depth -= 1
            if depth == 0:
                return i
        i += 1
    return -1


def analyse(path):
    doc = json.load(open(path))
    fns = doc['functions']
    # name (and alias) -> declared parameter count
    nparams = {}
    for f in fns:
        code = f.get('code') or ''
        first = code.split('\n', 1)[0]
        m = SIG.match(first)
        if not m:
            continue
        params = m.group(2).strip()
        p = 0 if params in ('', 'void') else len(split_args(params))
        for nm in [f['name']] + (f.get('aliases') or []):
            nparams[nm] = p
    total_calls = 0
    short_calls = 0
    empty_short = 0
    ge7 = 0
    lost_args = 0
    per_callee = {}
    for f in fns:
        code = f.get('code') or ''
        body = code.split('\n', 1)[1] if '\n' in code else ''
        for m in CALL.finditer(body):
            name = m.group(1)
            if name in KEYWORDS:
                continue
            if name not in nparams:
                continue
            open_i = m.end() - 1
            close_i = match_close(body, open_i)
            if close_i < 0:
                continue
            inner = body[open_i + 1:close_i].strip()
            argc = 0 if inner == '' else len(split_args(inner))
            total_calls += 1
            if argc >= 7:
                ge7 += 1
            p = nparams[name]
            if argc < p:
                short_calls += 1
                lost_args += p - argc
                if argc == 0:
                    empty_short += 1
                per_callee[name] = per_callee.get(name, 0) + 1
    return dict(binary=doc.get('binary'), functions=len(fns),
                total_calls=total_calls, short_calls=short_calls,
                empty_short=empty_short, calls_ge7_args=ge7,
                lost_args=lost_args)


if __name__ == '__main__':
    for p in sys.argv[1:]:
        try:
            print(json.dumps(analyse(p)))
        except Exception as e:  # noqa
            print(json.dumps({'binary': p, 'error': str(e)}))
