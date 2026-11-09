#!/usr/bin/env python3
"""Run the Vineflower 1.12.0 jar over every runnable manifest entry.

  --mode fixture   bin/vf-decompile-kt --fixture [--kotlin-only] + the entry's effective options (the
                   DecompilerTestFixture path: ConsoleDecompiler API, LEGACY_CONSOLEDECOMPILER saver; plugins as
                   the test's Gradle run sees them; driver_classpath_extra appended to java.class.path)
                   -> <out-root>/<oracle>
  --mode defaults  java -jar vineflower-1.12.0.jar [-e=LIB]... <class files...> <outdir>  (pure CLI defaults,
                   FOLDER saver, all 3 bundled plugins, include-runtime=current) -> <out-root>/<defaults_oracle>,
                   or with --lib ... and --dest-key defaults_stdlib_oracle -> <out-root>/<defaults_stdlib_oracle>
  --mode fixture-variant  like fixture, but with --plugin-mode / --cp-extra overrides (cross-checks; output
                   under <out-root>/<oracle> of a separate --out-root)
  --mode defaults-driver  the CLI's defaults through bin/vf-decompile-kt --save folder --opt include-runtime=current
                   (+ --plugin-mode all|kotlin-only|none, --lib, --extra-opt) -> <out-root>/<defaults_oracle>

Each run gets a fresh output directory under --work; the produced file is copied into --out-root with an
atomic rename. A per-run JSON report goes to <out-root>/reports/run-<mode><suffix>.json.
"""
import argparse
import concurrent.futures as cf
import json
import os
import shutil
import subprocess
import sys
import time

TOOLS = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
NO_ORACLE = ('KUNA_JVM_ORACLE is not set: set it to the JVM oracle workspace '
             '(decompiler/nonnative/kuna-jvm/tools/oracle/README.md)')


def oracle(*parts):
    root = os.environ.get('KUNA_JVM_ORACLE')
    if not root:
        sys.exit(NO_ORACLE)
    return os.path.join(root, *parts)


def build_cmd(t, a, inputs, outdir):
    if a.mode in ('fixture', 'fixture-variant'):
        cmd = [os.path.join(TOOLS, 'bin', 'vf-decompile-kt'), '--fixture']
        plugins = t['plugins'] if a.plugin_mode == 'manifest' else a.plugin_mode
        if plugins == 'kotlin-only':
            cmd.append('--kotlin-only')
        elif plugins == 'all':
            cmd.append('--plugins')
        for k, v in t['options'].items():
            cmd += ['--opt', f"{k}={v}"]
        for kv in a.extra_opt or []:
            cmd += ['--opt', kv]
        cmd += inputs + ['-o', outdir]
    elif a.mode == 'defaults-driver':
        cmd = [os.path.join(TOOLS, 'bin', 'vf-decompile-kt'), '--save', 'folder', '--opt', 'include-runtime=current']
        plugins = 'all' if a.plugin_mode == 'manifest' else a.plugin_mode
        if plugins == 'kotlin-only':
            cmd.append('--kotlin-only')
        elif plugins == 'all':
            cmd.append('--plugins')
        for kv in a.extra_opt or []:
            cmd += ['--opt', kv]
        for l in a.lib or []:
            cmd += ['--lib', os.path.join(a.corpus, l)]
        cmd += inputs + ['-o', outdir]
    else:
        cmd = [a.java, f'-Xmx{a.xmx}', '-Dfile.encoding=UTF-8', '-jar', a.jar]
        cmd += [f'-e={os.path.join(a.corpus, l)}' for l in a.lib or []]
        cmd += inputs + [outdir]
    return cmd


def run_one(t, a):
    tid = t['id']
    work = os.path.join(a.work, a.mode + a.suffix, tid)
    outdir = os.path.join(work, 'out')
    os.makedirs(work, exist_ok=False)
    files = [os.path.join(a.classes, f) for f in t['class_files']]
    if a.order == 'reverse':
        files = files[::-1]
    cmd = build_cmd(t, a, files, outdir)
    env = dict(os.environ, LANG='C.UTF-8', LC_ALL='C.UTF-8', VF_JAVA=a.java, VF_JAR=a.jar, VF_XMX=a.xmx,
               CORPUS=a.corpus)
    extra = (t['driver_classpath_extra'] if a.mode != 'defaults-driver' else []) if a.cp_extra == 'manifest' else ([] if a.cp_extra == 'none' else a.cp_extra.split(':'))
    env['VF_CP_EXTRA'] = ':'.join(os.path.join(a.corpus, p) for p in extra)
    with open(os.path.join(work, 'cmd.json'), 'w') as f:
        json.dump(dict(cmd=cmd, VF_CP_EXTRA=env['VF_CP_EXTRA']), f, indent=1)
    t0 = time.time()
    timed_out = False
    with open(os.path.join(work, 'log.txt'), 'wb') as log:
        try:
            rc = subprocess.run(cmd, stdout=log, stderr=subprocess.STDOUT, env=env, cwd=work,
                                timeout=a.timeout).returncode
        except subprocess.TimeoutExpired:
            rc, timed_out = None, True
    secs = round(time.time() - t0, 2)
    if a.mode in ('fixture', 'fixture-variant'):
        names = [t['decompiled_file'], t.get('decompiled_file_fallback')]
        dest_rel = t['oracle']
    else:
        names = [t['defaults_file'], t['defaults_file'][:-3] + '.java']
        dest_rel = t[a.dest_key]
    produced = None
    for name in names:
        if name and os.path.isfile(os.path.join(outdir, name)):
            produced = name
            break
    if produced:
        dest = os.path.join(a.out_root, dest_rel)
        os.makedirs(os.path.dirname(dest), exist_ok=True)
        tmp = dest + '.tmp-run'
        shutil.copyfile(os.path.join(outdir, produced), tmp)
        os.replace(tmp, dest)
    outputs = sorted(os.path.relpath(os.path.join(dp, f), outdir) for dp, _, fs in os.walk(outdir) for f in fs) \
        if os.path.isdir(outdir) else []
    ext_ok = bool(produced) and os.path.splitext(produced)[1] == os.path.splitext(dest_rel)[1]
    with open(os.path.join(work, 'log.txt'), 'rb') as f:
        log_bytes = f.read()
    return dict(id=tid, rc=rc, timed_out=timed_out, seconds=secs, produced=produced, extension_ok=ext_ok,
                dest=dest_rel if produced else None, outputs=outputs, log_bytes=len(log_bytes), work=work)


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('--manifest', required=True)
    ap.add_argument('--classes', required=True)
    ap.add_argument('--corpus', required=True, help='the corpus directory (driver/classes, tools/m2, tools/plugin)')
    ap.add_argument('--mode', choices=['fixture', 'fixture-variant', 'defaults', 'defaults-driver'], required=True)
    ap.add_argument('--plugin-mode', default='manifest', choices=['manifest', 'kotlin-only', 'all', 'none'])
    ap.add_argument('--cp-extra', default='manifest', help="'manifest', 'none', or ':'-separated corpus paths")
    ap.add_argument('--extra-opt', action='append', help='fixture modes: extra k=v options (applied last)')
    ap.add_argument('--lib', action='append', help='defaults: -e= libraries')
    ap.add_argument('--dest-key', default='defaults_oracle', help='defaults: manifest key of the destination path')
    ap.add_argument('--out-root', required=True)
    ap.add_argument('--work', required=True, help='a new work directory for the per-run output dirs')
    ap.add_argument('--jar', help='default: $KUNA_JVM_ORACLE/oracle/vineflower-1.12.0.jar')
    ap.add_argument('--java', help='default: $KUNA_JVM_ORACLE/jdks/jdk17/bin/java')
    ap.add_argument('--xmx', default='1g')
    ap.add_argument('--jobs', type=int, default=16)
    ap.add_argument('--timeout', type=int, default=600)
    ap.add_argument('--order', choices=['fixture', 'reverse'], default='fixture')
    ap.add_argument('--suffix', default='')
    ap.add_argument('--only', action='append', help='run only ids containing this substring')
    a = ap.parse_args()
    a.jar = a.jar or oracle('oracle', 'vineflower-1.12.0.jar')
    a.java = a.java or oracle('jdks', 'jdk17', 'bin', 'java')
    for k in ('classes', 'corpus', 'out_root', 'work'):
        setattr(a, k, os.path.abspath(getattr(a, k)))
    m = json.load(open(a.manifest, encoding='utf-8'))
    tests = [t for t in m['tests'] if t['status'] == 'ok']
    if a.only:
        tests = [t for t in tests if any(s in t['id'] for s in a.only)]
    tests.sort(key=lambda t: -len(t['class_files']))
    results = []
    with cf.ThreadPoolExecutor(max_workers=a.jobs) as ex:
        futs = [ex.submit(run_one, t, a) for t in tests]
        for f in cf.as_completed(futs):
            r = f.result()
            results.append(r)
            if not r['produced'] or r['rc'] != 0 or not r['extension_ok']:
                print(f"  {r['id']}: rc={r['rc']} timeout={r['timed_out']} produced={r['produced']}", file=sys.stderr)
    results.sort(key=lambda r: r['id'])
    rep_dir = os.path.join(a.out_root, 'reports')
    os.makedirs(rep_dir, exist_ok=True)
    rep = os.path.join(rep_dir, f'run-{a.mode}{a.suffix}.json')
    summary = dict(mode=a.mode, plugin_mode=a.plugin_mode, cp_extra=a.cp_extra, extra_opt=a.extra_opt, lib=a.lib,
                   order=a.order, java=a.java, jar=a.jar, total=len(results),
                   produced=sum(bool(r['produced']) for r in results),
                   wrong_extension=sum(1 for r in results if r['produced'] and not r['extension_ok']),
                   nonzero_rc=sum(1 for r in results if r['rc'] != 0),
                   timeouts=sum(r['timed_out'] for r in results),
                   total_seconds=round(sum(r['seconds'] for r in results), 1),
                   max_seconds=max((r['seconds'] for r in results), default=0))
    with open(rep + '.tmp', 'w') as f:
        json.dump(dict(summary=summary, results=results), f, indent=1)
    os.replace(rep + '.tmp', rep)
    print(json.dumps(summary), file=sys.stderr)


if __name__ == '__main__':
    main()
