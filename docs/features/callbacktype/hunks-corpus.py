"""Whole-corpus before/after `decompile-all` over N binaries, every hunk classified.

A hunk is IN SCOPE only when it lies inside a function the option parked a
declared prototype on. The park round runs after every other decompile of the
run and decompiles only the parked functions again, so any other function that
prints differently -- a caller of a parked callback included -- is a bug
(UNEXPLAINED), and so is a parked function that gains a `CONCAT` (half a
register invented for it). Each changed function carries its parameter and
return types and its cast count in both arms, counted by the castbench counter
over the whole file's vocabulary.

Extra binaries (for instance a reviewer's private counterexamples) can be added
with HUNKS_EXTRA=path:path:...
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
    # the round-8 review's disjoint set, and the forwarding fixture
    f'{R}/O0/dpkg/stripped/dpkg-query',
    f'{R}/O0/dpkg/stripped/dpkg-trigger',
    f'{R}/O2/coreutils/stripped/numfmt',
    f'{R}/O2/dpkg/stripped/dpkg',
    f'{R}/O2/libexpat/stripped/xmlwf',
    f'{R}/O2-noinline/rsyslog/stripped/rsyslogd',
    f'{R}/O2-noinline/shadow/stripped/sulogin',
    f'{R}/O2-noinline/sysvinit/stripped/sulogin',
    f'{R}/O0/shadow/stripped/userdel',
    f'{R}/O2/shadow/stripped/userdel',
    f'{R}/O0/shadow/stripped/chfn',
] + [os.path.normpath(os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', '..',
                      'decompiler/crates/kuna-analysis/tests/fixtures', f))
     for f in ('callbacktype_x86_64', 'callbacktype_refused_x86_64', 'callbacktype_width_x86_64',
               'callbacktype_narrow_x86_64', 'callbacktype_forward_x86_64')
] + [b for b in os.environ.get('HUNKS_EXTRA', '').split(':') if b]

def label(b):
    """`<opt>/<project>/stripped/<bin>` for a decbench binary, else the file name."""
    parts = b.split('/full_run_address_2026-09-11/')
    return parts[1] if len(parts) > 1 else os.path.basename(b)

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

SIG = re.compile(r'^(?!//)(?![ \t])([^(;]*?)\b(\w+)\((.*)\)\s*(//.*)?$')

def signature(lines):
    """`(return type, parameter types)` of the signature line `lines` prints,
    or `(None, None)`."""
    for line in lines[1:6]:
        m = SIG.match(line)
        if not m:
            continue
        ret, params = m.group(1).strip(), m.group(3).strip()
        if params in ('', 'void'):
            return ret, []
        return ret, [re.sub(r'\s*\ba\d+$', '', q.strip()) for q in params.split(',')]
    return None, None

def param_types(lines):
    """The parameter types of the signature line `lines` prints, or None."""
    return signature(lines)[1]

def work(b):
    off, _ = run(b, 'off')
    on, err = run(b, 'on', trace=True)
    parked = set()
    for l in err.splitlines():
        m = re.match(r'\[callbacktype\] park (\S+) @0x([0-9a-f]+) ', l)
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
    names = {int(addr, 0): name for (name, addr) in c}
    rows = []
    for (name, addr), old, new in changed:
        a_ = int(addr, 0)
        body = '\n'.join(old + new)
        calls_parked = a_ not in parked and any(
            re.search(r'\b%s\(' % re.escape(names.get(p, 'sub_%x' % p)), body) for p in parked)
        (ret_off, pt_off), (ret_on, pt_on) = signature(old), signature(new)
        kind = 'parked' if a_ in parked else 'UNEXPLAINED-not-parked'
        if sum('CONCAT' in l for l in new) > sum('CONCAT' in l for l in old):
            kind = 'UNEXPLAINED-concat'
        diff = [l for l in difflib.unified_diff(old, new, lineterm='', n=0)
                if l.startswith(('+', '-')) and not l.startswith(('+++', '---'))]
        rows.append({'bin': b, 'fn': name, 'addr': addr, 'kind': kind,
                     'casts_off': casts(old, va), 'casts_on': casts(new, vc),
                     'params_off': pt_off, 'params_on': pt_on,
                     'return_off': ret_off, 'return_on': ret_on,
                     'own_params_moved': a_ not in parked and pt_off != pt_on,
                     'calls_a_parked_function': calls_parked,
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
            print(f'{label(b)}: {nfn} functions, {np} parked, {len(rows)} changed')
    json.dump(out, open(sys.argv[1] if len(sys.argv) > 1 else '.scratch/hunks.json', 'w'), indent=1)
    print('TOTAL', dict(tot))
    print('NON-PARKED functions that change at all: %d' % sum(
        r['kind'] == 'UNEXPLAINED-not-parked' for r in out))
    print('  of them, own parameter types moved (UNEXPLAINED-own-params): %d' % sum(
        r['own_params_moved'] for r in out))
    print('  of them, callers of a parked function (caller hunks): %d' % sum(
        r['calls_a_parked_function'] for r in out))
    co, cn = sum(r['casts_off'] for r in out), sum(r['casts_on'] for r in out)
    print('CASTS changed functions: off %d -> on %d; more %d, fewer %d, same %d' % (
        co, cn, sum(r['casts_on'] > r['casts_off'] for r in out),
        sum(r['casts_on'] < r['casts_off'] for r in out),
        sum(r['casts_on'] == r['casts_off'] for r in out)))
    for r in sorted(out, key=lambda r: r['casts_off'] - r['casts_on'])[:25]:
        if r['casts_on'] != r['casts_off']:
            print('  casts %+d %s %s %s (%d -> %d)' % (r['casts_on'] - r['casts_off'],
                  label(r['bin']), r['fn'], r['addr'],
                  r['casts_off'], r['casts_on']))
    for r in out:
        if r['kind'].startswith('UNEXPLAINED'):
            print('UNEXPLAINED', r['bin'], r['fn'], r['addr'])
            for l in r['diff'][:12]:
                print('   ', l)
