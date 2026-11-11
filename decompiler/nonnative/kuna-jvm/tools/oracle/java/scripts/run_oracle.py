#!/usr/bin/env python3
"""Run the Vineflower 1.12.0 jar over every runnable manifest entry.

  --mode fixture   bin/vf-decompile --fixture + the entry's effective options (DecompilerTestFixture path:
                   ConsoleDecompiler API, LEGACY_CONSOLEDECOMPILER saver, no plugins) -> <out-root>/<oracle>
  --mode defaults  java -jar vineflower-1.12.0.jar <class files...> <outdir>  (pure CLI defaults, FOLDER saver,
                   bundled plugins active, include-runtime=current) -> <out-root>/<defaults_oracle>
  --mode defaults-driver  the same defaults through bin/vf-decompile --save folder (cross-check; --plugins
                   keeps the bundled plugins, otherwise none are loaded)

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


def resolve(value, java_homes):
    if isinstance(value, str) and value.startswith('${java.') and value.endswith('.home}'):
        v = value[len('${java.'):-len('.home}')]
        home = java_homes.get(v)
        if not home:
            raise RuntimeError(f"no JDK home for java.{v}.home")
        return home
    return value


def build_cmd(t, a, inputs, outdir, java_homes):
    if a.mode == 'fixture':
        cmd = [os.path.join(TOOLS, 'bin', 'vf-decompile'), '--fixture']
        for k, v in t['options'].items():
            if isinstance(v, dict) and '$javadoc_provider' in v:
                cmd += ['--javadoc', v['$javadoc_provider']]
            else:
                cmd += ['--opt', f"{k}={resolve(v, java_homes)}"]
        cmd += inputs + ['-o', outdir]
    elif a.mode == 'defaults-driver':
        # same as the CLI's defaults (CONSOLE_DEFAULT_OPTIONS = {include-runtime: current}, FOLDER saver),
        # but through the driver, optionally without the bundled plugins
        cmd = [os.path.join(TOOLS, 'bin', 'vf-decompile'), '--save', 'folder', '--opt', 'include-runtime=current']
        if a.plugins:
            cmd.append('--plugins')
        for kv in a.extra_opt or []:
            cmd += ['--opt', kv]
        cmd += inputs + ['-o', outdir]
    else:
        cmd = [a.java, f'-Xmx{a.xmx}', '-Dfile.encoding=UTF-8', '-jar', a.jar] + inputs + [outdir]
    return cmd


def run_one(t, a, java_homes):
    tid = t['id']
    work = os.path.join(a.work, a.mode + a.suffix, tid)
    outdir = os.path.join(work, 'out')
    os.makedirs(work, exist_ok=False)
    files = [os.path.join(a.classes, f) for f in t['class_files']]
    if a.order == 'reverse':
        files = files[::-1]
    elif a.order == 'sorted':
        files = sorted(files)
    cmd = build_cmd(t, a, files, outdir, java_homes)
    env = dict(os.environ, LANG='C.UTF-8', LC_ALL='C.UTF-8', VF_JAVA=a.java, VF_JAR=a.jar, VF_XMX=a.xmx,
               CORPUS=a.corpus)
    with open(os.path.join(work, 'cmd.json'), 'w') as f:
        json.dump(cmd, f, indent=1)
    t0 = time.time()
    timed_out = False
    with open(os.path.join(work, 'log.txt'), 'wb') as log:
        try:
            rc = subprocess.run(cmd, stdout=log, stderr=subprocess.STDOUT, env=env, cwd=work,
                                timeout=a.timeout).returncode
        except subprocess.TimeoutExpired:
            rc, timed_out = None, True
    secs = round(time.time() - t0, 2)
    produced = None
    for name in [t['decompiled_file'], t.get('decompiled_file_fallback')]:
        if name and os.path.isfile(os.path.join(outdir, name)):
            produced = os.path.join(outdir, name)
            break
    dest_rel = t['oracle'] if a.mode == 'fixture' else t['defaults_oracle']
    if produced:
        dest = os.path.join(a.out_root, dest_rel)
        os.makedirs(os.path.dirname(dest), exist_ok=True)
        tmp = dest + '.tmp-run'
        shutil.copyfile(produced, tmp)
        os.replace(tmp, dest)
    outputs = sorted(os.listdir(outdir)) if os.path.isdir(outdir) else []
    return dict(id=tid, rc=rc, timed_out=timed_out, seconds=secs, produced=bool(produced),
                dest=dest_rel if produced else None, outputs=outputs, work=work)


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('--manifest', required=True)
    ap.add_argument('--classes', required=True)
    ap.add_argument('--corpus', required=True, help='the corpus directory (holds driver/classes)')
    ap.add_argument('--mode', choices=['fixture', 'defaults', 'defaults-driver'], required=True)
    ap.add_argument('--plugins', action='store_true', help='defaults-driver: keep the bundled plugins')
    ap.add_argument('--extra-opt', action='append', help='defaults-driver: extra k=v options (applied last)')
    ap.add_argument('--out-root', required=True)
    ap.add_argument('--work', required=True, help='a new work directory for the per-run output dirs')
    ap.add_argument('--jar', help='default: $KUNA_JVM_ORACLE/oracle/vineflower-1.12.0.jar')
    ap.add_argument('--java', help='default: $KUNA_JVM_ORACLE/jdks/jdk17/bin/java')
    ap.add_argument('--xmx', default='1g')
    ap.add_argument('--jobs', type=int, default=16)
    ap.add_argument('--timeout', type=int, default=300)
    ap.add_argument('--order', choices=['fixture', 'reverse', 'sorted'], default='fixture')
    ap.add_argument('--suffix', default='', help='suffix for work subdir / report name')
    ap.add_argument('--only', action='append', help='run only ids containing this substring')
    a = ap.parse_args()
    a.jar = a.jar or oracle('oracle', 'vineflower-1.12.0.jar')
    a.java = a.java or oracle('jdks', 'jdk17', 'bin', 'java')
    a.classes = os.path.abspath(a.classes)
    a.corpus = os.path.abspath(a.corpus)
    a.out_root = os.path.abspath(a.out_root)
    a.work = os.path.abspath(a.work)

    m = json.load(open(a.manifest, encoding='utf-8'))
    tests = [t for t in m['tests'] if t['status'] == 'ok']
    if a.only:
        tests = [t for t in tests if any(s in t['id'] for s in a.only)]
    # longest groups first for better packing
    tests.sort(key=lambda t: -len(t['class_files']))
    results = []
    with cf.ThreadPoolExecutor(max_workers=a.jobs) as ex:
        futs = {ex.submit(run_one, t, a, m['java_homes']): t for t in tests}
        for f in cf.as_completed(futs):
            r = f.result()
            results.append(r)
            if not r['produced'] or r['rc'] != 0:
                print(f"  {r['id']}: rc={r['rc']} timeout={r['timed_out']} produced={r['produced']}", file=sys.stderr)
    results.sort(key=lambda r: r['id'])
    rep_dir = os.path.join(a.out_root, 'reports')
    os.makedirs(rep_dir, exist_ok=True)
    rep = os.path.join(rep_dir, f'run-{a.mode}{a.suffix}.json')
    summary = dict(mode=a.mode, order=a.order, java=a.java, jar=a.jar, total=len(results),
                   produced=sum(r['produced'] for r in results),
                   nonzero_rc=sum(1 for r in results if r['rc'] != 0),
                   timeouts=sum(r['timed_out'] for r in results),
                   max_seconds=max((r['seconds'] for r in results), default=0))
    with open(rep + '.tmp', 'w') as f:
        json.dump(dict(summary=summary, results=results), f, indent=1)
    os.replace(rep + '.tmp', rep)
    print(json.dumps(summary), file=sys.stderr)


if __name__ == '__main__':
    main()
