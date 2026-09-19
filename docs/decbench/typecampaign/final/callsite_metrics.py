#!/usr/bin/env python3
"""Caller-side metrics for `protoorder`: what the option does to CALL SITES.

`typesweep`/`type_match` scores a function's own `variables[]`, so it cannot see
an argument that a call site gained or lost.  This is the metric that can.

Reads `<dir>/<binary>.{off,on}.json` (`kuna decompile-all --json`) and reports,
over every function whose emitted C differs between the arms:

  deleted     arguments the OFF arm renders and the ON arm does not (must be 0)
  added       arguments the ON arm renders and the OFF arm does not
  phantom     arguments that are a local the caller never assigns (either arm) --
              the reader-visible shape of an argument the disassembly does not set
  strings     arguments CONTAINING a string literal, per arm (the RE artifact a
              deletion destroys).  Matched with a literal regex, not
              `startswith('"')`: the option adds `(char *)` casts in front of
              literals, and a prefix test counts the cast as a lost string.
  over-arity  groups where an arm renders more arguments at a site than the
              callee's own emitted declaration takes
  argrows     `variables[]` rows with `kind == "arg"` -- the FABRICATED-PARAMETER
              counter.  A call-site argument is only half the cost: parking a
              prototype can also give a caller parameters of its own, and
              `type_match` cannot see those (it scores tp/fp/fn over the DWARF
              variable list only, so a decompiled variable with no ground-truth
              counterpart is never a false positive).  Reported as functions
              gaining/losing arg rows and the row totals.
"""
import collections
import json
import os
import re
import sys

CALL = re.compile(r'\b([A-Za-z_][A-Za-z0-9_]*)\s*\(')
# (round 5) C type keywords join the statement keywords: `(char)(v1 + 2)` is a
# CAST followed by a parenthesised expression, and the naive "identifier then
# '('" rule reads it as a call to `char`.  This option adds and removes casts, so
# leaving them in made the whole-body argument total drift by hundreds on tar and
# e2fsck while the per-callee pairing -- which is what `added`/`deleted` use --
# reported nothing.
KW = {'if', 'while', 'for', 'switch', 'return', 'sizeof', 'do', 'else',
      'char', 'short', 'int', 'long', 'float', 'double', 'void', 'signed',
      'unsigned', 'bool', '_Bool', 'int1', 'int2', 'int4', 'int8', 'int16',
      'uint1', 'uint2', 'uint4', 'uint8', 'uint16', 'undefined', 'undefined1',
      'undefined2', 'undefined4', 'undefined8', 'code', 'struct', 'union'}
STRLIT = re.compile(r'"(?:[^"\\]|\\.)*"')
# (round 5) Printer intrinsics, which are spellings and not calls: `SUB84(x,0)`,
# `CONCAT31(...)`, `ZEXT48(...)` and the `builtin_strncpy` constant-sequence
# form all appear and vanish when a value's TYPE changes.  Counting them as
# calls is how a pure type change reports a call-count delta.
INTRINSIC = re.compile(r'^(?:SUB|SEXT|ZEXT|CONCAT|PIECE)\d\d?$|^builtin_')


def split_args(code, start):
    """The argument texts of the call whose '(' is at `start`, and the index past ')'."""
    i, depth, cur, args = start + 1, 1, '', []
    while i < len(code) and depth > 0:
        c = code[i]
        # (round 5) A string or character literal is one token: `foo(x, ", ")`
        # has two arguments, not three.  Typing an argument `char *` is what
        # makes the printer render the literal instead of its address, so
        # splitting inside one reported a GAINED type as a gained argument.
        if c in '"\'':
            quote, cur, i = c, cur + c, i + 1
            while i < len(code):
                cur += code[i]
                if code[i] == '\\':
                    i += 2
                    if i - 1 < len(code):
                        cur += code[i - 1]
                    continue
                if code[i] == quote:
                    i += 1
                    break
                i += 1
            continue
        if c in '([':
            depth += 1
        elif c in ')]':
            depth -= 1
            if depth == 0:
                break
        elif c == ',' and depth == 1:
            args.append(cur)
            cur = ''
            i += 1
            continue
        cur += c
        i += 1
    args.append(cur)
    if len(args) == 1 and args[0].strip() == '':
        args = []
    return [a.strip() for a in args], i


def sanitize(code):
    """Replace every string/char literal with a fixed token and drop `//` comments.

    (round 5) A literal and a comment both carry parentheses, commas and
    apostrophes, and this option moves text between the two forms -- an argument
    typed `char *` renders as `"...%s..."` where it rendered as `0x807fde4`.
    Scanning the raw text made the parenthesis matcher run away inside a comment
    and report six deleted arguments on betaflight that are not there.  Counting
    on the sanitized text keeps `STRLIT` working (the token IS a string literal)
    and makes every bracket the counter sees real code.
    """
    out, i, n = [], 0, len(code)
    while i < n:
        c = code[i]
        if c in '"\'':
            quote, i = c, i + 1
            while i < n:
                if code[i] == '\\':
                    i += 2
                    continue
                if code[i] == quote:
                    i += 1
                    break
                i += 1
            out.append('"S"' if quote == '"' else "'c'")
            continue
        if c == '/' and i + 1 < n and code[i + 1] == '/':
            while i < n and code[i] != '\n':
                i += 1
            continue
        out.append(c)
        i += 1
    return ''.join(out)


def sites(code):
    """Every call in the BODY, in source order, keyed by callee name.

    The header is dropped first: the function's own declaration matches the call
    regex, and counting it as a self-call is how a differ invents deletions.
    """
    body = sanitize(code).split('{', 1)[-1]
    out = collections.defaultdict(list)
    for m in CALL.finditer(body):
        nm = m.group(1)
        if nm in KW or INTRINSIC.match(nm):
            continue
        args, _ = split_args(body, m.end() - 1)
        out[nm].append(args)
    return out


def declared(code):
    first = code.split('\n', 1)[0]
    m = re.match(r'^[^(]*\b([A-Za-z_]\w*)\s*\((.*?)\)\s*(//.*)?$', first)
    if not m:
        return None, None
    inner = m.group(2).strip()
    if inner in ('', 'void'):
        return m.group(1), 0
    return m.group(1), inner.count(',') + 1


ASSIGN = re.compile(r'(?<![=!<>+\-*/%&|^])=(?!=)')


def unassigned_locals(code):
    """Locals the function declares and never writes.

    A write is `x =`, `x++`/`x--`, `&x`, or any member/element reach (`x[`,
    `x.`, `x->`) -- the last three because the value is then reached through a
    pointer this differ does not track, and calling that a phantom would
    over-count.  What is left is the shape of an argument the caller never set:
    a declared local that only ever appears as an rvalue at a call.
    """
    body = sanitize(code).split('{', 1)[-1]
    names = set(re.findall(r'\b(v\d+)\b', body))
    written = set()
    for line in body.split('\n'):
        for m in re.finditer(r'\b(v\d+)\b', line):
            n = m.group(1)
            rest = line[m.end():].lstrip()
            if ASSIGN.match(rest) or rest[:2] in ('++', '--'):
                written.add(n)
            if rest[:1] in ('[', '.') or rest[:2] == '->':
                written.add(n)
            if line[max(0, m.start() - 1):m.start()] == '&':
                written.add(n)
    return names - written


def main(directory, out_path):
    tot = collections.Counter()
    per_bin = {}
    detail = []
    names = sorted({f.rsplit('.', 2)[0] for f in os.listdir(directory) if f.endswith('.json')})
    for n in names:
        off = {f['address_hex']: f for f in json.load(open(f'{directory}/{n}.off.json'))['functions']}
        on = {f['address_hex']: f for f in json.load(open(f'{directory}/{n}.on.json'))['functions']}
        decl_off, decl_on = {}, {}
        for src, dst in ((off, decl_off), (on, decl_on)):
            for f in src.values():
                nm, k = declared(f.get('code') or '')
                if nm:
                    dst[nm] = k
        c = collections.Counter()
        # Fabricated parameters: `variables[]` rows with kind == "arg", the
        # surface decbench scores and the one `type_match` cannot penalise.
        for a in sorted(set(off) & set(on)):
            ro = sum(1 for v in (off[a].get('variables') or []) if v.get('kind') == 'arg')
            rn = sum(1 for v in (on[a].get('variables') or []) if v.get('kind') == 'arg')
            c['argrows_off'] += ro
            c['argrows_on'] += rn
            if rn > ro:
                c['funcs_gaining_argrows'] += 1
                c['argrows_added'] += rn - ro
            elif ro > rn:
                c['funcs_losing_argrows'] += 1
                c['argrows_removed'] += ro - rn
        c['functions'] = len(set(off) & set(on))
        # Whole-binary totals, both arms, every function: immune to how call
        # sites are paired, so they are the number to quote.
        for src, tag in ((off, 'off'), (on, 'on')):
            for f in src.values():
                code = f.get('code') or ''
                u = unassigned_locals(code)
                for callee, lst in sites(code).items():
                    for site in lst:
                        c['allargs_' + tag] += len(site)
                        for arg in site:
                            if arg in u:
                                c['allphantom_' + tag] += 1
                            if STRLIT.search(arg):
                                c['allstring_' + tag] += 1
        for a in sorted(set(off) & set(on)):
            co, cn = off[a].get('code') or '', on[a].get('code') or ''
            if co == cn:
                continue
            so, sn = sites(co), sites(cn)
            uo, un = unassigned_locals(co), unassigned_locals(cn)
            for callee in sorted(set(so) | set(sn)):
                # Source order, paired positionally: `protoorder` does not move
                # statements, so the Nth call to a callee in the OFF body is the
                # Nth in the ON body.  (Sorting the two lists by length instead
                # pairs a 4-argument site with an unrelated 3-argument one and
                # reports the difference as a deletion.)
                lo = so.get(callee, [])
                ln = sn.get(callee, [])
                ko = [len(x) for x in lo]
                kn = [len(x) for x in ln]
                if len(ko) != len(kn):
                    c['site_count_changed'] += 1
                if ko == kn and [x for y in lo for x in y] == [x for y in ln for x in y]:
                    continue
                for i in range(min(len(ko), len(kn))):
                    if ko[i] > kn[i]:
                        c['deleted'] += ko[i] - kn[i]
                        c['deleted_sites'] += 1
                        detail.append(dict(kind='deleted', binary=n, caller=off[a]['name'] + '@' + a,
                                           callee=callee, off=lo[i], on=ln[i]))
                    elif kn[i] > ko[i]:
                        c['added'] += kn[i] - ko[i]
                        c['added_sites'] += 1
                for args, u, tag in ((lo, uo, 'off'), (ln, un, 'on')):
                    for site in args:
                        for arg in site:
                            if arg in u:
                                c['phantom_' + tag] += 1
                            if STRLIT.search(arg):
                                c['string_' + tag] += 1
                do_, dn_ = decl_off.get(callee), decl_on.get(callee)
                over_off = do_ is not None and any(x > do_ for x in ko)
                over_on = dn_ is not None and any(x > dn_ for x in kn)
                if over_on and not over_off:
                    c['new_over_arity_groups'] += 1
                if over_off and not over_on:
                    c['fixed_over_arity_groups'] += 1
        per_bin[n] = dict(c)
        tot.update(c)
    json.dump(dict(total=dict(tot), per_binary=per_bin, detail=detail[:200]),
              open(out_path, 'w'), indent=1)
    print('TOTAL', json.dumps(dict(tot), sort_keys=True))
    for n in names:
        print(' ', n, json.dumps(per_bin[n], sort_keys=True))


if __name__ == '__main__':
    main(sys.argv[1], sys.argv[2])
