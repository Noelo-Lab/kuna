"""Arity / phantom counters for `callbacktype`, checked against DWARF, plus the
direct call sites of every function whose parameter list moves.

Over every slice the census found a callback argument in, diff the per-function
argument list between the two arms, ask the unstripped twin whether each gained
parameter is one the program declares, and count the arguments at every direct
call site of a moved function in both arms' C text (an argument a call site
gains is one the closed list materialized).
"""
import json, os, re, subprocess, sys, collections
from concurrent.futures import ProcessPoolExecutor, as_completed

K = os.environ.get('KUNA_BIN', '/home/mahaloz/kwt/callbacktype/decompiler/target/release/kuna')


def run(b, value):
    p = subprocess.run([K, 'decompile-all', b, '--json', '--max-fn-seconds', '120',
                        '--option', 'callbacktype', value],
                       capture_output=True, text=True, timeout=3600)
    try:
        doc = json.loads(p.stdout)
    except Exception:  # noqa: BLE001
        return {}
    out = {}
    for f in doc.get('functions', []):
        try:
            a = int(str(f.get('address')), 0)
        except Exception:  # noqa: BLE001
            continue
        v = f.get('variables') or []
        out[a] = {'name': f.get('name'), 'code': f.get('code') or '',
                  'args': [x.get('type') for x in v if x.get('kind') == 'arg'],
                  'unread': [x.get('name') for x in v if x.get('kind') == 'arg'
                             and not x.get('line_numbers') and not x.get('addresses')]}
    return out


def dwarf(unstripped):
    from elftools.elf.elffile import ELFFile
    out = {}
    try:
        fh = open(unstripped, 'rb')
    except OSError:
        return out
    with fh:
        elf = ELFFile(fh)
        if not elf.has_dwarf_info():
            return out
        for cu in elf.get_dwarf_info().iter_CUs():
            for die in cu.get_top_DIE().iter_children():
                if die.tag != 'DW_TAG_subprogram':
                    continue
                lo = die.attributes.get('DW_AT_low_pc')
                if lo is None:
                    continue
                out[lo.value] = sum(1 for c in die.iter_children()
                                    if c.tag == 'DW_TAG_formal_parameter')
    return out


def call_arg_counts(fns, name):
    """Argument count of every call to `name` in every function but its own
    definition line, as a sorted list."""
    pat = re.compile(r'(?<![\w.>])' + re.escape(name) + r'\(')
    counts = []
    for f in fns.values():
        for line in f['code'].splitlines():
            if re.match(r'^\S.*\b' + re.escape(name) + r'\(', line):
                continue  # the definition's own header
            for m in pat.finditer(line):
                i, depth, n, empty = m.end(), 1, 1, True
                while i < len(line) and depth:
                    ch = line[i]
                    if ch == '(':
                        depth += 1
                    elif ch == ')':
                        depth -= 1
                    elif ch == ',' and depth == 1:
                        n += 1
                    if depth and not ch.isspace() and ch != ')':
                        empty = False
                    i += 1
                counts.append(0 if empty else n)
    return sorted(counts)


def work(b):
    off, on = run(b, 'off'), run(b, 'on')
    dw = dwarf(b.replace('/stripped/', '/compiled/'))
    rows = []
    for a in sorted(set(off) & set(on)):
        if len(off[a]['args']) == len(on[a]['args']):
            continue
        name = on[a]['name']
        rows.append({'bin': b, 'addr': a, 'name': name, 'off': len(off[a]['args']),
                     'on': len(on[a]['args']), 'dwarf': dw.get(a),
                     'unread_on': on[a]['unread'],
                     'calls_off': call_arg_counts(off, name),
                     'calls_on': call_arg_counts(on, name)})
    return b, rows


if __name__ == '__main__':
    scan = json.load(open('.scratch/cbscan.json'))
    bins = sorted({r['bin'] for r in scan if r.get('hits')})
    print(f'binaries with a callback argument: {len(bins)}', file=sys.stderr)
    rows = []
    with ProcessPoolExecutor(max_workers=int(os.environ.get('ARITY_WORKERS', '5'))) as ex:
        for f in as_completed([ex.submit(work, b) for b in bins]):
            _, r = f.result()
            rows.extend(r)
    out = sys.argv[1] if len(sys.argv) > 1 else '.scratch/arity.json'
    json.dump(rows, open(out, 'w'), indent=1)
    c = collections.Counter()
    for r in rows:
        d = 'gained' if r['on'] > r['off'] else 'lost'
        c[d] += abs(r['on'] - r['off'])
        if r['dwarf'] is None:
            c[d + ':no-dwarf'] += 1
        elif r['on'] <= r['dwarf']:
            c[d + ':dwarf-confirms'] += 1
        else:
            c[d + ':PHANTOM'] += 1
        if r['calls_on']:
            c['moved-with-a-direct-call-site'] += 1
        if r['calls_on'] != r['calls_off']:
            c['call-site-arg-count-moved'] += 1
    print('functions whose argument count moves:', len(rows))
    print(dict(c))
    for r in rows:
        if r['dwarf'] is not None and r['on'] > r['dwarf']:
            print('PHANTOM', r['bin'], hex(r['addr']), r['name'])
        if r['calls_on'] != r['calls_off']:
            print('CALLSITE', r['bin'], hex(r['addr']), r['name'], r['calls_off'], r['calls_on'])
