"""Function-level invariants over the protoorder before/after corpus dump.

A line-level taxonomy over-reports for this option: a stated type retypes a
parameter, and a retyped pointer rewrites
`*(int8 *)(a0 + 0xb8)` into `a0[0x17]` -- the same access with a scaled index, on
a line whose token stream no longer matches. What actually has to hold is
coarser and checkable exactly, per function:

  calls_lost       a callee named in the OFF body is called FEWER times in the ON
                   body                                       <-- the bug class
  calls_gained     ... more times (a de-indirected call, or a call the OFF body
                   folded into an expression)
  gotos_delta      `goto` count (structure regressions show up here)
  returns_delta    `return` count
  stmt_delta       `;` count

Run it as `python3 invariants.py <arm> [dir]`; the directory defaults to this one
and holds `<name>.off.c` / `<name>.<arm>.c` or `<name>.off.json` /
`<name>.<arm>.json` (`decompile-all --json`, read from each function's `code`).
"""
import re, sys, json, os, collections

OUT = os.path.dirname(os.path.abspath(__file__))
SWEEP = OUT
INTRINSIC = {'CONCAT44', 'CONCAT71', 'CONCAT31', 'CONCAT62', 'CONCAT53', 'CONCAT17',
             'SUB168', 'SUB84', 'SUBch', 'SEXT48', 'SEXT18', 'SEXT28', 'SEXT14',
             'SEXT24', 'SEXT12', 'ZEXT48', 'ZEXT18', 'ZEXT28', 'ZEXT12', 'ZEXT14',
             'ZEXT816', 'NAN', 'ABS', 'SQRT', 'POPCOUNT', 'if', 'while', 'for',
             'switch', 'return', 'sizeof', 'do'}
CALL_RE = re.compile(r'\b([A-Za-z_]\w*)\s*\(')
# (round 5) Two more families that are SPELLINGS, not calls, and that a pure type
# change moves: every widening/narrowing intrinsic (`SUB41`, `CONCAT31`, ...) and
# the `builtin_*` printer forms, plus a C type keyword, which `(char)(v1 + 2)`
# puts in front of a '('.  Without them a retyped value reports as a lost call.
NOT_A_CALL = re.compile(r'^(?:SUB|SEXT|ZEXT|CONCAT|PIECE)\d\d?$|^builtin_')
TYPEWORD = {'char', 'short', 'int', 'long', 'float', 'double', 'void', 'signed',
            'unsigned', 'bool', '_Bool', 'code', 'struct', 'union', 'else',
            'int1', 'int2', 'int4', 'int8', 'int16', 'uint1', 'uint2', 'uint4',
            'uint8', 'uint16', 'undefined', 'undefined1', 'undefined2',
            'undefined4', 'undefined8'}

def funcs(path):
    if path.endswith('.json'):
        return {f"{f['name']}@{f['address_hex']}": f.get('code') or ''
                for f in json.load(open(path))['functions']}
    out, cur, buf = {}, None, []
    for ln in open(path, errors='replace').read().splitlines():
        m = re.match(r'// Function: (\S+) @ (0x[0-9a-f]+)', ln)
        if m:
            if cur: out[cur] = '\n'.join(buf)
            cur, buf = f"{m.group(1)}@{m.group(2)}", [ln]
        else:
            buf.append(ln)
    if cur: out[cur] = '\n'.join(buf)
    return out

def calls(body):
    c = collections.Counter()
    for m in CALL_RE.finditer(body):
        nm = m.group(1)
        if nm in INTRINSIC or nm in TYPEWORD or NOT_A_CALL.match(nm):
            continue
        c[nm] += 1
    return c

def main(arm, sweep=None):
    global SWEEP
    if sweep:
        SWEEP = sweep
    tot = collections.Counter()
    lost_rows, gained_rows, goto_rows = [], [], []
    per_bin = {}
    for f in sorted(os.listdir(SWEEP)):
        ext = '.c' if f.endswith('.off.c') else '.json' if f.endswith('.off.json') else None
        if ext is None: continue
        base = f[:-len('.off' + ext)]
        B = os.path.join(SWEEP, f'{base}.{arm}{ext}')
        if not os.path.exists(B): continue
        fa, fb = funcs(os.path.join(SWEEP, f)), funcs(B)
        n = collections.Counter()
        for k in fa:
            if k not in fb: 
                n['function_missing'] += 1
                continue
            a, b = fa[k], fb[k]
            if a == b: continue
            n['changed'] += 1
            ca, cb = calls(a), calls(b)
            lost = {x: ca[x] - cb[x] for x in ca if cb[x] < ca[x]}
            gained = {x: cb[x] - ca[x] for x in cb if cb[x] > ca[x]}
            if lost:
                n['calls_lost'] += 1
                lost_rows.append({'binary': base, 'function': k, 'lost': lost})
            if gained:
                n['calls_gained'] += 1
                gained_rows.append({'binary': base, 'function': k, 'gained': gained})
            ga, gb = a.count('goto '), b.count('goto ')
            if ga != gb:
                n['gotos_moved'] += 1
                goto_rows.append({'binary': base, 'function': k, 'gotos_off': ga, 'gotos_on': gb})
            n['goto_delta'] += gb - ga
            n['return_delta'] += b.count('return') - a.count('return')
            n['stmt_delta'] += b.count(';') - a.count(';')
        per_bin[base] = dict(n)
        tot.update(n)
    print(json.dumps({'arm': arm, 'totals': dict(tot), 'per_binary': per_bin}, indent=2))
    with open(os.path.join(OUT, f'calls-lost.{arm}.txt'), 'w') as fh:
        for r in lost_rows: fh.write(json.dumps(r) + '\n')
    with open(os.path.join(OUT, f'calls-gained.{arm}.txt'), 'w') as fh:
        for r in gained_rows: fh.write(json.dumps(r) + '\n')
    with open(os.path.join(OUT, f'gotos.{arm}.txt'), 'w') as fh:
        for r in goto_rows: fh.write(json.dumps(r) + '\n')

main(sys.argv[1], sys.argv[2] if len(sys.argv) > 2 else None)
