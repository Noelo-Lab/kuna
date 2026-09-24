"""For every variable castsign flips (off -> on), list the lines of its function that
mention it beside a decimal literal of 2^(w-1) or more (w = the declaration's width),
or any integer literal whose top bit at width w is set.
  python3 widescan.py <off-dir> <on-dir>"""
import pathlib, re, sys
FN = re.compile(r'^// Function: .* @ (0x[0-9a-f]+)$')
DECL = re.compile(r'^\s*(unsigned )?(int|long|u?int[48])( v\d+);')
W = {'int': 32, 'long': 64, 'int4': 32, 'int8': 64, 'uint4': 32, 'uint8': 64}
LIT = re.compile(r'(?<![\w.])(0x[0-9a-fA-F]+|\d+)(?![\w.])')
def funcs(p):
    out, cur, a = {}, [], None
    for l in p.read_text(errors='replace').split('\n'):
        m = FN.match(l)
        if m:
            if a: out[a] = cur
            a, cur = m.group(1), []
        cur.append(l)
    if a: out[a] = cur
    return out
off, on = map(pathlib.Path, sys.argv[1:3])
nflip = dec = hexn = 0
for f in sorted(on.rglob('*.c')):
    A, B = funcs(off / f.relative_to(on)), funcs(f)
    for a, lines in B.items():
        if A.get(a) == lines: continue
        od = {m.group(3).strip(): (m.group(1), m.group(2)) for l in A.get(a, []) for m in [DECL.match(l)] if m}
        nd = {m.group(3).strip(): (m.group(1), m.group(2)) for l in lines for m in [DECL.match(l)] if m}
        for v, (u, t) in nd.items():
            if v not in od or od[v] == (u, t): continue
            nflip += 1
            w = W[t]
            for l in lines:
                if not re.search(r'\b' + v + r'\b', l) or DECL.match(l): continue
                for m in LIT.finditer(l):
                    s = m.group(1); x = int(s, 16) if s.startswith('0x') else int(s)
                    if x >= 2 ** (w - 1):
                        if s.startswith('0x'): hexn += 1; print('HEX', f.name, a, v, t, l.strip())
                        else: dec += 1; print('DECIMAL', f.name, a, v, t, l.strip())
print(f'flipped variables {nflip}  decimal top-bit literals on their lines {dec}  hex top-bit literals {hexn}')
