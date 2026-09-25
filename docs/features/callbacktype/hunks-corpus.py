"""Whole-corpus before/after `decompile-all` over N binaries, every hunk classified.

A hunk is IN SCOPE when it lies inside a function the option parked a declared
prototype on, or inside a function that CALLS one (its call site is rebuilt from
the declaration) without changing the number of arguments any call to a parked
function passes or growing the number of lines the caller prints -- a caller
that grows an argument or a statement is what a closed list longer than the body
invents, and a caller that grows a local is what a declared return narrower than
the caller's read of it splits the register into. A caller that prints fewer
lines is in scope and is read by hand. A changed function of either kind
that gains a `CONCAT` has had half a register invented for it. Anything else is
a bug, and so is a function that is not itself parked whose own parameter types
change: the declaration rebuilds a call, never what the caller was handed. Each
changed function also carries its parameter types and its cast count in both
arms, counted by the castbench counter over the whole file's vocabulary.
"""
import json, os, re, subprocess, sys, collections, difflib
sys.path.insert(0, os.environ.get('CASTBENCH', '/home/mahaloz/kwt/castbench'))
import castcount as CC
from concurrent.futures import ProcessPoolExecutor, as_completed

K = os.environ.get('KUNA_BIN', '/home/mahaloz/kwt/callbacktype/decompiler/target/release/kuna')
R = '/home/mahaloz/github/decbench/results/full_run_address_2026-09-11'
BINS = [
    f'{R}/O0/coreutils/stripped/cut',
    f'{R}/O2/coreutils/stripped/sort',
    f'{R}/O2-noinline/coreutils/stripped/ptx',
    f'{R}/O0/bzip2/stripped/bzip2',
    f'{R}/O2/shadow/stripped/su',
    f'{R}/O0/tar/stripped/tar',
    f'{R}/O2/diffutils/stripped/diff',
    f'{R}/O0/findutils/stripped/find',
    f'{R}/O2/coreutils/stripped/fmt',
    f'{R}/O2/coreutils/stripped/ls',
    f'{R}/O0/grep/stripped/grep',
    f'{R}/O2/gzip/stripped/gzip',
    # the review's disjoint set
    f'{R}/O2/coreutils/stripped/ptx',
    f'{R}/O2/tar/stripped/tar',
    f'{R}/O2/shadow/stripped/login',
    f'{R}/O0/shadow/stripped/su',
    f'{R}/O2/bzip2/stripped/bzip2',
    f'{R}/O0/coreutils/stripped/sort',
    f'{R}/O2-noinline/coreutils/stripped/cut',
    f'{R}/O0/cronie/stripped/crond',
    f'{R}/O2/dash/stripped/dash',
    # the round-3 review's disjoint set
    f'{R}/O2/libselinux/stripped/libselinux.so.1',
    f'{R}/O0/dash/stripped/dash',
    f'{R}/O2/dpkg/stripped/dpkg-statoverride',
    f'{R}/O0/kmod/stripped/kmod',
    f'{R}/O2/sysvinit/stripped/init',
    f'{R}/O2/cronie/stripped/crontab',
    f'{R}/O2-noinline/shadow/stripped/useradd',
    f'{R}/O2/coreutils/stripped/timeout',
    f'{R}/O2/gnutls/stripped/gnutls-serv',
    f'{R}/O2/e2fsprogs/stripped/e2fsck',
    f'{R}/O0/rsyslog/stripped/rsyslogd',
    f'{R}/O2-noinline/dpkg/stripped/dpkg-query',
    # the round-4 review's disjoint set
    f'{R}/O2-noinline/coreutils/stripped/sort',
    f'{R}/O2-noinline/dash/stripped/dash',
    f'{R}/O2-noinline/libselinux/stripped/libselinux.so.1',
    f'{R}/O2-noinline/kmod/stripped/kmod',
    f'{R}/O2/shadow/stripped/useradd',
    f'{R}/O2-noinline/findutils/stripped/find',
    f'{R}/O2/dpkg/stripped/dpkg-divert',
    f'{R}/O2/gnutls/stripped/certtool',
    f'{R}/O2/sysvinit/stripped/shutdown',
    f'{R}/O2/diffutils/stripped/sdiff',
    f'{R}/O2/grep/stripped/grep',
    f'{R}/O0/libselinux/stripped/libselinux.so.1',
    f'{R}/O2-noinline/gnutls/stripped/systemkey',
    f'{R}/O2/coreutils/stripped/ginstall',
    # the round-7 review's disjoint set
    f'{R}/O2/rsyslog/stripped/rsyslogd',
    f'{R}/O2-noinline/tar/stripped/tar',
    f'{R}/O0/e2fsprogs/stripped/e2fsck',
    f'{R}/O2-noinline/e2fsprogs/stripped/e2fsck',
    f'{R}/O0/gnutls/stripped/certtool',
    f'{R}/O0/coreutils/stripped/ptx',
    f'{R}/O0/iproute2/stripped/ip',
    f'{R}/O2/libedit/stripped/libedit.so.0.0.70',
    f'{R}/O0/openssh-portable/stripped/sftp',
    f'{R}/O2/openssh-portable/stripped/scp',
    f'{R}/O0/coreutils/stripped/ls',
    f'{R}/O2/coreutils/stripped/du',
]

def run(b, value, trace=False):
    env = dict(os.environ)
    if trace:
        env['KUNA_CALLBACKTYPE_TRACE'] = '1'
    p = subprocess.run([K, 'decompile-all', b, '--option', 'callbacktype', value,
                        '--max-fn-seconds', '120'],
                       capture_output=True, text=True, env=env, timeout=3600)
    return p.stdout, p.stderr

FN = re.compile(r'^// Function: (\S+) @ (0x[0-9a-f]+)')

def by_function(text):
    out, cur, buf = {}, None, []
    for line in text.splitlines():
        m = FN.match(line)
        if m:
            if cur:
                out[cur] = buf
            cur, buf = (m.group(1), m.group(2)), []
        buf.append(line)
    if cur:
        out[cur] = buf
    return out

def call_args(lines, callee):
    """Argument counts of every call to `callee` in `lines`, sorted."""
    pat = re.compile(r'(?<![\w.>])' + re.escape(callee) + r'\(')
    out = []
    for line in lines:
        if re.match(r'^\S.*\b' + re.escape(callee) + r'\(', line):
            continue
        for m in pat.finditer(line):
            i, depth, n, empty = m.end(), 1, 1, True
            while i < len(line) and depth:
                ch = line[i]
                depth += (ch == '(') - (ch == ')')
                if ch == ',' and depth == 1:
                    n += 1
                if depth and not ch.isspace():
                    empty = False
                i += 1
            out.append(0 if empty else n)
    return sorted(out)

SIG = re.compile(r'^(?!//)(?![ \t])[^(;]*?\b(\w+)\((.*)\)\s*(//.*)?$')

def param_types(lines):
    """The parameter types of the signature line `lines` prints, or None."""
    for line in lines[1:6]:
        m = SIG.match(line)
        if not m:
            continue
        params = m.group(2).strip()
        if params in ('', 'void'):
            return []
        return [re.sub(r'\s*\ba\d+$', '', q.strip()) for q in params.split(',')]
    return None

def work(b):
    off, _ = run(b, 'off')
    on, err = run(b, 'on', trace=True)
    parked = set()
    for l in err.splitlines():
        m = re.match(r'\[callbacktype\] park (\S+) @0x([0-9a-f]+)', l)
        if m:
            parked.add(int(m.group(2), 16))
    a, c = by_function(off), by_function(on)
    va = CC.harvest_types(CC.tokenize(off), off)
    vc = CC.harvest_types(CC.tokenize(on), on)
    casts = lambda lines, v: len(CC.find_casts(CC.tokenize('\n'.join(lines)), v))
    changed = []
    for k in sorted(set(a) | set(c)):
        if a.get(k) != c.get(k):
            changed.append((k, a.get(k, []), c.get(k, [])))
    # which changed functions call a parked one?
    names = {int(addr, 0): name for (name, addr) in c}
    callers = set()
    for (name, addr), old, new in changed:
        body = '\n'.join(new)
        for p in parked:
            callee = re.escape(names.get(p, 'sub_%x' % p))
            if re.search(r'\b(%s|sub_%x|FUN_%x)\b' % (callee, p, p), body) or ('0x%x' % p) in body:
                callers.add((name, addr))
    rows = []
    for (name, addr), old, new in changed:
        a_ = int(addr, 0)
        kind = ('parked' if a_ in parked else
                'caller-of-parked' if (name, addr) in callers else 'UNEXPLAINED')
        if kind == 'caller-of-parked':
            grew = len(new) > len(old) or any(
                call_args(old, names[p]) != call_args(new, names[p])
                for p in parked if p in names)
            if grew:
                kind = 'UNEXPLAINED-caller-grew'
        if sum('CONCAT' in l for l in new) > sum('CONCAT' in l for l in old):
            kind = 'UNEXPLAINED-concat'
        # A function that is not itself parked keeps its own parameter types:
        # the declaration changes a call, never what the caller is handed.
        pt_off, pt_on = param_types(old), param_types(new)
        params_moved = a_ not in parked and pt_off != pt_on
        if params_moved:
            kind = 'UNEXPLAINED-own-params'
        diff = [l for l in difflib.unified_diff(old, new, lineterm='', n=0)
                if l.startswith(('+', '-')) and not l.startswith(('+++', '---'))]
        rows.append({'bin': b, 'fn': name, 'addr': addr, 'kind': kind,
                     'casts_off': casts(old, va), 'casts_on': casts(new, vc),
                     'params_off': pt_off, 'params_on': pt_on,
                     'hunk_lines': len(diff), 'diff': diff[:40]})
    return b, len(parked), len(a), rows

if __name__ == '__main__':
    out = []
    tot = collections.Counter()
    with ProcessPoolExecutor(max_workers=4) as ex:
        for f in as_completed([ex.submit(work, b) for b in BINS]):
            b, np, nfn, rows = f.result()
            tot['functions'] += nfn
            tot['parks'] += np
            for r in rows:
                tot[r['kind']] += 1
            out.extend(rows)
            print(f'{b.split("/full_run_address_2026-09-11/")[1]}: {nfn} functions, '
                  f'{np} parked, {len(rows)} changed')
    json.dump(out, open(sys.argv[1] if len(sys.argv) > 1 else '.scratch/hunks.json', 'w'), indent=1)
    print('TOTAL', dict(tot))
    print('NON-PARKED functions whose own parameter types change: %d' % sum(
        r['kind'] == 'UNEXPLAINED-own-params' for r in out))
    co, cn = sum(r['casts_off'] for r in out), sum(r['casts_on'] for r in out)
    print('CASTS changed functions: off %d -> on %d; more %d, fewer %d, same %d' % (
        co, cn, sum(r['casts_on'] > r['casts_off'] for r in out),
        sum(r['casts_on'] < r['casts_off'] for r in out),
        sum(r['casts_on'] == r['casts_off'] for r in out)))
    for r in sorted(out, key=lambda r: r['casts_off'] - r['casts_on'])[:25]:
        if r['casts_on'] != r['casts_off']:
            print('  casts %+d %s %s %s (%d -> %d)' % (r['casts_on'] - r['casts_off'],
                  r['bin'].split('/full_run_address_2026-09-11/')[1], r['fn'], r['addr'],
                  r['casts_off'], r['casts_on']))
    for r in out:
        if r['kind'].startswith('UNEXPLAINED'):
            print('UNEXPLAINED', r['bin'], r['fn'], r['addr'])
            for l in r['diff'][:12]:
                print('   ', l)
