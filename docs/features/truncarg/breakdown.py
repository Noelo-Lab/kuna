"""Classify each new narrowed-argument cast by the callee's own printed parameter width."""
import json, re, collections, sys
O = sys.argv[1] if len(sys.argv) > 1 else '.'
d = json.load(open(O + '/classify.json'))
CASTRE = re.compile(r'\((unsigned short|unsigned char|uint2|uint1)\)')
NARROW = {'char', 'unsigned char', 'uint1', 'int1', 'bool', 'short', 'unsigned short', 'uint2', 'int2', 'byte',
          'undefined1', 'undefined2'}
cache = {}
stats = collections.Counter()


def enclosing_call(line, pos):
    depth = 0
    argi = 0
    i = pos - 1
    while i >= 0:
        ch = line[i]
        if ch == ')':
            depth += 1
        elif ch == '(':
            if depth == 0:
                m = re.search(r'([A-Za-z_]\w*)\s*$', line[:i])
                return (m.group(1) if m else None), argi
            depth -= 1
        elif ch == ',' and depth == 0:
            argi += 1
        i -= 1
    return None, 0


for name, fn, o, n in d['examples']:
    if name not in cache:
        f = json.load(open('%s/corpus/%s.fix.json' % (O, name)))
        cache[name] = {x['name']: x for x in f['functions']}
    m = cache[name]
    arch = 'arm32' if ('betaflight' in name or 'crazyflie' in name) else 'x86-64'
    newc = len(CASTRE.findall(n)) - len(CASTRE.findall(o))
    seen = 0
    for cm in CASTRE.finditer(n):
        if seen >= newc and CASTRE.findall(o):
            break
        callee, argi = enclosing_call(n, cm.start())
        if callee is None:
            k = 'indirect call'
        elif callee not in m:
            k = 'callee without a body in the corpus'
        else:
            sig = m[callee]['code'].splitlines()[0]
            ps = sig[sig.find('(') + 1:sig.rfind(')')]
            params = [p.strip() for p in ps.split(',')] if ps.strip() not in ('void', '') else []
            if argi >= len(params):
                k = 'argument beyond the callee definition\'s parameters'
            else:
                ptype = params[argi].rsplit(' ', 1)[0].replace('*', '').strip()
                k = 'callee declares the narrow parameter (redundant)' if ptype in NARROW else \
                    'callee reads a wider parameter (value fixed)'
        stats[(arch, k)] += 1
        seen += 1
tot = collections.Counter()
for (a, k), v in sorted(stats.items()):
    print('%-7s %-52s %d' % (a, k, v))
    tot[k] += v
print('total', sum(tot.values()), dict(tot))
