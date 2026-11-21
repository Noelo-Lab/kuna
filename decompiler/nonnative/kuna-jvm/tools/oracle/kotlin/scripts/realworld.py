#!/usr/bin/env python3
"""Reference Vineflower 1.12.0 output directories for real-world Kotlin code (plain CLI, default options).

  okio-2.6.0     java -jar vineflower-1.12.0.jar -e=<libs> com.squareup.okio.okio-2.6.0.jar <out>
  okhttp-4.7.2   java -jar vineflower-1.12.0.jar -e=<libs> com.squareup.okhttp3.okhttp-4.7.2.jar <out>
  spring-kotlin  java -jar vineflower-1.12.0.jar --only=<19 top-level Kotlin classes> -e=<48 deps> classes.jar <out>
                 (the whitelist selects exactly the 72 classes of spring-framework/classes.jar that carry
                 @kotlin.Metadata; every other class is still loaded as a source class, none is written)
                 Vineflower 1.12.0 quirk: with --only, ContextUnit.save drops the synthesized package directory
                 entries (they fail isWhitelisted and the jar has no non-class entries), DirectoryResultSaver
                 never creates the package directories and the run dies with "Failed to save class"
                 (NoSuchFileException) at the first class. The harness therefore pre-creates the package
                 directories of the whitelisted classes in the (otherwise empty) output directory.
  spring-full    cross-check only (kept under --work): the whole classes.jar, same libraries, no --only; its
                 Kotlin outputs are compared with spring-kotlin's.

Inputs and libraries (-e=, one flag per jar, sorted by path): okio and okhttp are the Maven Central jars that
scripts/fetch_tools.sh puts in --libs (named <group>.<artifact>-<version>.jar); okio = kotlin-stdlib 2.2.20 +
annotations 13.0 (kotlin-stdlib-common 1.3.70 of its POM holds no class files and is left out); okhttp = okio
2.6.0 + the same two; spring = every jar of <--spring>/deps (kotlin-stdlib/-reflect 2.3.21, coroutines,
reactor, ...), where --spring is the javabench dataset's spring-framework directory (scripts/javabench, default
$KUNA_JVM_ORACLE/dataset/spring-framework). No -e for the JDK: CLI default include-runtime=current = the
running JVM's image.

Each target runs into a fresh directory under --work (/usr/bin/time -v, JDK 17, -Xmx4g), is copied into
<out-root>/realworld/<name>/vf/, and <out-root>/realworld/<name>/run.json records inputs + sha256, the exact
command, JVM, wall time, max RSS and output counts. --rerun-1cpu repeats each run with -XX:ActiveProcessorCount=1
(default thread-count becomes 1) and records whether the trees are byte-identical.
"""
import argparse
import filecmp
import hashlib
import json
import os
import re
import shutil
import subprocess
import sys
import zipfile

NO_ORACLE = ('KUNA_JVM_ORACLE is not set: set it to the JVM oracle workspace '
             '(decompiler/nonnative/kuna-jvm/tools/oracle/README.md)')


def oracle(*parts):
    root = os.environ.get('KUNA_JVM_ORACLE')
    if not root:
        sys.exit(NO_ORACLE)
    return os.path.join(root, *parts)


def sha256(p):
    h = hashlib.sha256()
    with open(p, 'rb') as f:
        for b in iter(lambda: f.read(1 << 20), b''):
            h.update(b)
    return h.hexdigest()


def spring_kotlin_tops(jar):
    z = zipfile.ZipFile(jar)
    cls = [n[:-6] for n in z.namelist() if n.endswith('.class')]
    kt = {n for n in cls if b'Lkotlin/Metadata;' in z.read(n + '.class')}
    tops = sorted(n for n in kt if '$' not in n.rsplit('/', 1)[-1])
    covered = {c for c in cls if any(c.startswith(t) for t in tops)}
    assert covered == kt, (sorted(covered - kt), sorted(kt - covered))
    return tops, len(kt)


def targets(lib, spring):
    std = [f'{lib}/org.jetbrains.kotlin.kotlin-stdlib-2.2.20.jar', f'{lib}/org.jetbrains.annotations-13.0.jar']
    tops, nkt = spring_kotlin_tops(f'{spring}/classes.jar')
    return [
        dict(name='okio-2.6.0', input=f'{lib}/com.squareup.okio.okio-2.6.0.jar', libs=sorted(std), only=[],
             kotlin_classes=None),
        dict(name='okhttp-4.7.2', input=f'{lib}/com.squareup.okhttp3.okhttp-4.7.2.jar',
             libs=sorted(std + [f'{lib}/com.squareup.okio.okio-2.6.0.jar']), only=[], kotlin_classes=None),
        dict(name='spring-kotlin', input=f'{spring}/classes.jar',
             libs=sorted(os.path.join(spring, 'deps', f) for f in os.listdir(os.path.join(spring, 'deps'))
                         if f.endswith('.jar')),
             only=tops, kotlin_classes=nkt),
        dict(name='spring-full', input=f'{spring}/classes.jar', store=False,
             libs=sorted(os.path.join(spring, 'deps', f) for f in os.listdir(os.path.join(spring, 'deps'))
                         if f.endswith('.jar')),
             only=[], kotlin_classes=None),
    ]


def run(t, a, outdir, tag, java_opts):
    cmd = [a.java, '-Xmx4g', '-Dfile.encoding=UTF-8'] + java_opts + ['-jar', a.jar]
    if t['only']:
        cmd.append('--only=' + ','.join(t['only']))
    cmd += [f'-e={l}' for l in t['libs']] + [t['input'], outdir]
    for top in t['only']:  # see the --only quirk in the module docstring
        os.makedirs(os.path.join(outdir, os.path.dirname(top)), exist_ok=True)
    log = os.path.join(a.work, f"{t['name']}-{tag}.log")
    tv = os.path.join(a.work, f"{t['name']}-{tag}.time")
    with open(log, 'wb') as f:
        rc = subprocess.run(['/usr/bin/time', '-v', '-o', tv] + cmd, stdout=f, stderr=subprocess.STDOUT,
                            env=dict(os.environ, LANG='C.UTF-8', LC_ALL='C.UTF-8'), timeout=a.timeout).returncode
    tm = open(tv).read()
    wall = re.search(r'Elapsed \(wall clock\) time \(h:mm:ss or m:ss\): (\S+)', tm).group(1)
    secs = sum(float(x) * 60 ** i for i, x in enumerate(reversed(wall.split(':'))))
    rss = int(re.search(r'Maximum resident set size \(kbytes\): (\d+)', tm).group(1))
    cpu = float(re.search(r'User time \(seconds\): (\S+)', tm).group(1)) + \
        float(re.search(r'System time \(seconds\): (\S+)', tm).group(1))
    text = open(log, 'rb').read().decode('utf-8', 'replace')
    return dict(cmd=cmd, rc=rc, wall_seconds=round(secs, 2), cpu_seconds=round(cpu, 1), max_rss_mb=rss // 1024,
                log=log, log_lines=text.count('\n'),
                log_warn=len(re.findall(r'^WARN:', text, re.M)), log_error=len(re.findall(r'^ERROR:', text, re.M)))


def tree(d):
    return sorted(os.path.relpath(os.path.join(p, f), d) for p, _, fs in os.walk(d) for f in fs)


def do_target(t, a):
    first = os.path.join(a.work, t['name'], 'vf')
    os.makedirs(os.path.dirname(first), exist_ok=False)
    r = run(t, a, first, 'default', [])
    files = tree(first)
    rec = dict(name=t['name'], input=dict(path=t['input'], sha256=sha256(t['input'])),
               libraries=[dict(path=l, sha256=sha256(l)) for l in t['libs']],
               only=t['only'], kotlin_classes_selected=t['kotlin_classes'],
               java=a.java, java_version=a.java_version, vineflower_jar=dict(path=a.jar, sha256=sha256(a.jar)),
               run=r, outputs=dict(files=len(files), kt=sum(f.endswith('.kt') for f in files),
                                   java=sum(f.endswith('.java') for f in files),
                                   other=sum(not f.endswith(('.kt', '.java')) for f in files)))
    if a.rerun_1cpu:
        second = os.path.join(a.work, t['name'], 'vf-1cpu')
        r2 = run(t, a, second, '1cpu', ['-XX:ActiveProcessorCount=1'])
        f2 = tree(second)
        diffs = sorted(set(files) ^ set(f2)) + [f for f in set(files) & set(f2)
                                                 if not filecmp.cmp(os.path.join(first, f), os.path.join(second, f),
                                                                    shallow=False)]
        rec['rerun_1cpu'] = dict(run=r2, files=len(f2), differing_files=sorted(diffs))
    if not t.get('store', True):
        return rec
    dest = os.path.join(a.out_root, 'realworld', t['name'])
    os.makedirs(os.path.join(dest, 'vf'), exist_ok=True)
    shutil.copytree(first, os.path.join(dest, 'vf'), dirs_exist_ok=True)
    shutil.copyfile(r['log'], os.path.join(dest, 'vf.log.tmp'))
    os.replace(os.path.join(dest, 'vf.log.tmp'), os.path.join(dest, 'vf.log'))
    with open(os.path.join(dest, 'run.json.tmp'), 'w') as f:
        json.dump(rec, f, indent=1)
    os.replace(os.path.join(dest, 'run.json.tmp'), os.path.join(dest, 'run.json'))
    return rec


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('--out-root', required=True)
    ap.add_argument('--work', required=True)
    ap.add_argument('--jar', help='default: $KUNA_JVM_ORACLE/oracle/vineflower-1.12.0.jar')
    ap.add_argument('--java', help='default: $KUNA_JVM_ORACLE/jdks/jdk17/bin/java')
    ap.add_argument('--libs', help='the okio/okhttp jars and their libraries (default: '
                    '$KUNA_JVM_ORACLE/corpus-kt/tools/realworld, filled by fetch_tools.sh)')
    ap.add_argument('--spring', default=os.environ.get('SPRING_DIR'),
                    help='the javabench spring-framework dataset directory (classes.jar, deps/; default: '
                    '$SPRING_DIR, else $KUNA_JVM_ORACLE/dataset/spring-framework)')
    ap.add_argument('--timeout', type=int, default=7200)
    ap.add_argument('--rerun-1cpu', action='store_true')
    ap.add_argument('--only', action='append')
    a = ap.parse_args()
    a.jar = a.jar or oracle('oracle', 'vineflower-1.12.0.jar')
    a.java = a.java or oracle('jdks', 'jdk17', 'bin', 'java')
    a.libs = os.path.abspath(a.libs or oracle('corpus-kt', 'tools', 'realworld'))
    a.spring = os.path.abspath(a.spring or oracle('dataset', 'spring-framework'))
    a.out_root, a.work = os.path.abspath(a.out_root), os.path.abspath(a.work)
    os.makedirs(a.work, exist_ok=True)
    a.java_version = subprocess.run([a.java, '-version'], capture_output=True, text=True).stderr.strip().splitlines()
    ts = [t for t in targets(a.libs, a.spring) if not a.only or t['name'] in a.only]
    recs = [do_target(t, a) for t in ts]  # sequential: cleaner timings
    by = {r['name']: r for r in recs}
    if 'spring-kotlin' in by and 'spring-full' in by:
        k = os.path.join(a.work, 'spring-kotlin', 'vf')
        f = os.path.join(a.work, 'spring-full', 'vf')
        kt_k = [x for x in tree(k) if x.endswith('.kt')]
        kt_f = [x for x in tree(f) if x.endswith('.kt')]
        diff = sorted(set(kt_k) ^ set(kt_f)) + [x for x in set(kt_k) & set(kt_f)
                                                 if not filecmp.cmp(os.path.join(k, x), os.path.join(f, x), shallow=False)]
        rec = by['spring-kotlin']
        rec['fulljar_crosscheck'] = dict(cmd=by['spring-full']['run']['cmd'][:5] + ['-e=<same libraries>', '<classes.jar>', '<out>'],
                                         rc=by['spring-full']['run']['rc'],
                                         wall_seconds=by['spring-full']['run']['wall_seconds'],
                                         max_rss_mb=by['spring-full']['run']['max_rss_mb'],
                                         outputs=by['spring-full']['outputs'],
                                         kt_files_only_in_one=sorted(set(kt_k) ^ set(kt_f)),
                                         kt_files_differing=sorted(diff))
        dest = os.path.join(a.out_root, 'realworld', 'spring-kotlin', 'run.json')
        with open(dest + '.tmp', 'w') as fh:
            json.dump(rec, fh, indent=1)
        os.replace(dest + '.tmp', dest)
    for r in recs:
        print(json.dumps(dict(name=r['name'], rc=r['run']['rc'], wall=r['run']['wall_seconds'],
                              rss_mb=r['run']['max_rss_mb'], outputs=r['outputs'],
                              rerun_diffs=len(r['rerun_1cpu']['differing_files']) if 'rerun_1cpu' in r else None)),
              file=sys.stderr)


if __name__ == '__main__':
    main()
