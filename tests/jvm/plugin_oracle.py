#!/usr/bin/env python3
"""Rebuild the plugin cases of tests/jvm (listed in plugins.json) with the Vineflower 1.12.0 release jar.

  python3 -I tests/jvm/plugin_oracle.py --corpus ~/kwt/jvm/corpus --vf-src ~/github/vineflower --work NEW_DIR

1. Compiles plugins/variable-renaming/testData/src/java8 the way the plugin's build.gradle does (the oracle
   corpus's scripts/compile_sets.sh with VF_SRC at the plugin) and copies those classes and the plugin's
   checked-in custom classes to classes/plugins/<plugin>/<set>/.
2. Decompiles each case with <corpus>/bin/vf-decompile --plugins (the release jar's bundled plugins), adding
   --fixture for fixture sets, with the set's option pairs except kuna's own jvm_plugins; a non-fixture set runs
   like `java -jar` (--save folder --opt include-runtime=current). The compared file becomes the case's
   expected text.
3. Checks each output against the plugin's upstream .dec with the fixture's comparison (CRLF -> LF, trim), and
   the non-fixture outputs against the oracle corpus's expected-defaults when it has the class.

Writes under --out (default: this script's directory); overwrites, never deletes. Then run regen.py.
"""
import argparse
import json
import os
import shutil
import subprocess
import sys

KUNA_ONLY = {'jvm_plugins'}


def java_trim(s):
    i, j = 0, len(s)
    while i < j and s[i] <= ' ':
        i += 1
    while j > i and s[j - 1] <= ' ':
        j -= 1
    return s[i:j]


def get_content(path):
    with open(path, 'rb') as f:
        return java_trim(f.read().decode('utf-8', errors='replace').replace('\r\n', '\n'))


def copy(src, dst):
    os.makedirs(os.path.dirname(dst), exist_ok=True)
    shutil.copyfile(src, dst)


def main():
    here = os.path.dirname(os.path.abspath(__file__))
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument('--corpus', required=True, help='the oracle corpus (bin/vf-decompile, scripts/compile_sets.sh)')
    ap.add_argument('--vf-src', required=True, help='the Vineflower 1.12.0 checkout')
    ap.add_argument('--work', required=True, help='a new directory for compiler and decompiler runs')
    ap.add_argument('--out', default=here, help='the tests/jvm directory to write (default: %(default)s)')
    a = ap.parse_args()
    corpus = os.path.abspath(os.path.expanduser(a.corpus))
    vf = os.path.abspath(os.path.expanduser(a.vf_src))
    work = os.path.abspath(a.work)
    out = os.path.abspath(a.out)
    os.makedirs(work, exist_ok=False)

    with open(os.path.join(here, 'plugins.json'), encoding='utf-8') as f:
        m = json.load(f)
    sets = {s['name']: s for s in m['sets']}

    plugin = os.path.join(vf, 'plugins', 'variable-renaming')
    env = dict(os.environ, VF_SRC=plugin, CORPUS=corpus)
    subprocess.run(['bash', os.path.join(corpus, 'scripts', 'compile_sets.sh'), os.path.join(work, 'classes'),
                    os.path.join(work, 'logs'), 'java8'], env=env, check=True)
    for root, _, files in os.walk(os.path.join(work, 'classes', 'java8')):
        for name in files:
            rel = os.path.relpath(os.path.join(root, name), os.path.join(work, 'classes', 'java8'))
            copy(os.path.join(root, name), os.path.join(out, 'classes', 'plugins', 'variable-renaming', 'java8', rel))
    for p in ('variable-renaming', 'idea-not-null'):
        custom = os.path.join(vf, 'plugins', p, 'testData', 'classes', 'custom')
        for root, _, files in os.walk(custom):
            for name in files:
                rel = os.path.relpath(os.path.join(root, name), custom)
                dst = os.path.join(out, 'classes', 'plugins', p, 'custom', rel)
                if p == 'idea-not-null':
                    core = os.path.join(out, 'classes', 'custom', rel)
                    with open(os.path.join(root, name), 'rb') as x, open(core, 'rb') as y:
                        if x.read() != y.read():
                            sys.exit(f'{rel}: the plugin copy differs from classes/custom/{rel}')
                    continue
                copy(os.path.join(root, name), dst)

    failures = 0
    for t in m['tests']:
        s = sets[t['set']]
        outdir = os.path.join(work, 'run', t['id'])
        cmd = [os.path.join(corpus, 'bin', 'vf-decompile'), '--plugins']
        if s['fixture']:
            cmd.append('--fixture')
        else:
            cmd += ['--save', 'folder', '--opt', 'include-runtime=current']
        for k, v in s['option_pairs']:
            if k not in KUNA_ONLY:
                cmd += ['--opt', f'{k}={v}']
        cmd += [os.path.join(out, 'classes', f) for f in t['class_files']] + ['-o', outdir]
        with open(os.path.join(work, t['id'].replace('/', '_') + '.log'), 'wb') as log:
            subprocess.run(cmd, stdout=log, stderr=subprocess.STDOUT, check=True,
                           env=dict(os.environ, LANG='C.UTF-8', LC_ALL='C.UTF-8'))
        produced = [os.path.join(r, n) for r, _, fs in os.walk(outdir) for n in fs if n == t['decompiled_file']]
        if len(produced) != 1:
            sys.exit(f'{t["id"]}: expected one {t["decompiled_file"]}, got {produced}')
        copy(produced[0], os.path.join(out, t['oracle']))
        if t['reference']:
            ok = get_content(produced[0]) == get_content(os.path.join(vf, t['reference']))
            print(f'{t["id"]}: {"matches" if ok else "DIFFERS FROM"} {t["reference"]}')
        else:
            ref = os.path.join(corpus, 'expected-defaults', 'custom', t['decompiled_file'])
            ok = os.path.isfile(ref) and get_content(produced[0]) == get_content(ref)
            print(f'{t["id"]}: {"matches" if ok else "DIFFERS FROM"} {os.path.relpath(ref, corpus)}')
        failures += not ok
    if failures:
        sys.exit(f'{failures} case(s) differ from their reference')


if __name__ == '__main__':
    main()
