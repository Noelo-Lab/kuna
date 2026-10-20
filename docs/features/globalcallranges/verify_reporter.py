"""Check every initializer store and constructor argument against raw p-code."""
import ast, json, pathlib, re, struct
import sys
p = pathlib.Path(sys.argv[1] if len(sys.argv) > 1 else '.').resolve()
raw = (p / 'crashlogger-raw.txt').read_text().splitlines()
c = (p / 'final-on.c').read_text().splitlines()
blob = (p / 'SKSE/Plugins/CrashLogger.dll').read_bytes()
pe = struct.unpack_from('<I', blob, 60)[0]
n = struct.unpack_from('<H', blob, pe + 6)[0]
opt = struct.unpack_from('<H', blob, pe + 20)[0]
base = struct.unpack_from('<Q', blob, pe + 24 + 24)[0]
sections = []
for i in range(n):
    at = pe + 24 + opt + 40 * i
    (size, va, raw_size, ptr) = struct.unpack_from('<IIII', blob, at + 8)
    sections.append((va, raw_size, ptr))

def string_at(address):
    for (va, sz, ptr) in sections:
        if va <= address - base < va + sz:
            at = ptr + address - base - va
            return blob[at:blob.index(b'\x00', at)]
    return None
state = {}
mask = (1 << 64) - 1

def key(x):
    return x.split('(', 1)[0].split(':', 1)[0]

def value(x):
    x = x.strip()
    if x.startswith('#0x'):
        return int(x[3:].split(':', 1)[0], 16)
    m = re.fullmatch('ZEXT\\d+\\((.*)\\)', x)
    if m:
        return value(m[1])
    m = re.fullmatch('(.*?) ([+&|^-]) (.*)', x)
    if m:
        (a, b) = (value(m[1]), value(m[3]))
        if m[2] == '^' and key(m[1]) == key(m[3]):
            return 0
        if a is None or b is None:
            return None
        return {'+': lambda : a + b, '-': lambda : a - b, '&': lambda : a & b, '|': lambda : a | b, '^': lambda : a ^ b}[m[2]]() & mask
    return state.get(key(x))
events = []
for line in raw:
    if '\t' not in line:
        continue
    body = line.split('\t', 1)[1]
    m = re.fullmatch('call fsub_([0-9a-f]+)\\(free\\)', body)
    if m:
        target = int(m[1], 16)
        args = [state.get('RCX')]
        if target == 6443051808:
            args += [state.get('RDX'), state.get('R8')]
        events.append(('call', target, args))
        state['RAX'] = state.get('RCX')
        continue
    if ' = ' not in body:
        continue
    (out, expr) = body.split(' = ', 1)
    v = value(expr)
    m = re.match('r0x([0-9a-f]+)\\(', out)
    if m:
        events.append(('store', int(m[1], 16), v))
    state[key(out)] = v
actual = []
for line in c:
    m = re.fullmatch('  dat_([0-9a-f]+) = (.*);', line)
    if m:
        expr = m[2]
        if expr.startswith('"'):
            v = ast.literal_eval(expr).encode('utf-8')
        else:
            v = int(expr, 0)
        actual.append(('store', int(m[1], 16), v))
    m = re.fullmatch('  sub_([0-9a-f]+)\\((.*)\\);', line)
    if m:
        actual.append(('call', int(m[1], 16), [int(v, 0) for v in m[2].split(',')]))
assert len(events) == len(actual)
unknown = []
wrong = []
for (i, (want, got)) in enumerate(zip(events, actual)):
    if want[:2] != got[:2]:
        wrong.append((i, want, got))
        continue
    if want[2] is None or (isinstance(want[2], list) and None in want[2]):
        unknown.append((i, want, got))
        continue
    if isinstance(got[2], bytes):
        ok = string_at(want[2]) == got[2]
    else:
        ok = want[2] == got[2]
    if not ok:
        wrong.append((i, want, got))
report = dict(events=len(events), unknown_values=len(unknown), incorrect_values=len(wrong), same_order_and_targets=all((a[:2] == b[:2] for (a, b) in zip(events, actual))))
(p / 'reporter-values.json').write_text(json.dumps(report, indent=2) + '\n')
print(json.dumps(report))
print('unknown examples', unknown[:2])
print('wrong examples', wrong[:2])
assert not unknown and not wrong, report
