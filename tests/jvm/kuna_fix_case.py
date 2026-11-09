#!/usr/bin/env python3
"""Add (or refresh) a kuna-fixes case: a class Vineflower 1.12.0 fails on that a kuna fix decompiles.

  python3 -I tests/jvm/kuna_fix_case.py --fix jvm_fix_<slug> --class path/to/pkg/TestFoo.class \\
      [--source path/to/pkg/TestFoo.java] [--compiler "javac 17.0.20.1"] [--summary TEXT] [--opt k=v]...
      [--language java|kotlin]

The class's `Name$*.class` siblings in the same directory come with it, in name order (the corpus
fixture's `addSource` order). --language defaults to kotlin when the class carries `kotlin.Metadata`, else
java. A Kotlin case (`"language": "kotlin"` in cases.json) is decompiled with Vineflower's bundled plugins
(`vf-decompile --plugins`; kuna: `jvm_plugins=1`) and its texts are `.kt`; below, EXT is `.java` or `.kt`.
Steps, all under tests/jvm/kuna-fixes/ (README.md there has the layout):

  1. copies the class files to classes/<package>/ and the source to src/<package>/;
  2. runs the released Vineflower jar through the oracle (`vf-decompile --fixture`, plus the options of
     cases.json, dump-exception-on-error=0, and every --opt) and stores its text as
     expected-compat/<id>EXT; the text must contain Vineflower's failure comment;
  3. records the case in cases.json and rebuilds cases.tsv and the READMEs with regen.py;
  4. runs the corpus runner on both passes: the compat pass must match Vineflower's text, and kuna's
     default output becomes expected/<id>EXT (printed as a diff against the compat text, for review);
  5. runs both passes again and records them in tests/jvm/baseline.txt.

Needs KUNA_JVM_ORACLE (the oracle workspace, decompiler/nonnative/kuna-jvm/tools/oracle/README.md) and
cargo; the corpus runner is built in release mode (CARGO_TARGET_DIR is honoured).
"""
import argparse
import difflib
import glob
import json
import os
import shutil
import struct
import subprocess
import sys
import tempfile

HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.dirname(os.path.dirname(HERE))
FIXES = os.path.join(HERE, 'kuna-fixes')
VF_DECOMPILE = os.path.join(REPO, 'decompiler/nonnative/kuna-jvm/tools/oracle/java/bin/vf-decompile')
OPTIONS_TOML = os.path.join(REPO, 'decompiler/nonnative/kuna-jvm/options.toml')
MARKER = "$VF: Couldn't be decompiled"
# The fixture's leak check (unit-test mode) fails after the output is saved when a method fails
# while it is written; the saved text is still Vineflower's.
LEAK = 'TextBuffer was garbage collected without being added to another TextBuffer'


def is_kotlin(path):
    """Whether a class file carries the `kotlin.Metadata` annotation (its descriptor is in the constant pool)."""
    with open(path, 'rb') as f:
        return b'Lkotlin/Metadata;' in f.read()


def class_name(path):
    """The internal name of a class file (this_class), read from its constant pool."""
    with open(path, 'rb') as f:
        data = f.read()
    if data[:4] != b'\xca\xfe\xba\xbe':
        sys.exit(f'{path}: not a class file')
    count = struct.unpack('>H', data[8:10])[0]
    pos, index, utf8, classes = 10, 1, {}, {}
    while index < count:
        tag = data[pos]
        if tag == 1:
            n = struct.unpack('>H', data[pos + 1:pos + 3])[0]
            utf8[index] = data[pos + 3:pos + 3 + n].decode('utf-8', 'replace')
            pos += 3 + n
        elif tag == 7:
            classes[index] = struct.unpack('>H', data[pos + 1:pos + 3])[0]
            pos += 3
        elif tag in (8, 16, 19, 20):
            pos += 3
        elif tag == 15:
            pos += 4
        elif tag in (3, 4, 9, 10, 11, 12, 17, 18):
            pos += 5
        elif tag in (5, 6):
            pos += 9
            index += 1
        else:
            sys.exit(f'{path}: unknown constant pool tag {tag}')
        index += 1
    this_class = struct.unpack('>H', data[pos + 2:pos + 4])[0]
    return utf8[classes[this_class]]


def fix_options():
    """The jvm_fix_* rows of options.toml."""
    with open(OPTIONS_TOML, encoding='utf-8') as f:
        return {line.split('"')[1] for line in f if line.startswith('option = "jvm_fix_')}


def run(cmd, cwd=None, env=None):
    r = subprocess.run(cmd, cwd=cwd, env=env, capture_output=True, text=True)
    return r.returncode, r.stdout, r.stderr


def corpus(case_ids, out, extra=()):
    """Runs the corpus runner on the given case ids, in process: their statuses and the runner's output."""
    cmd = ['cargo', 'run', '-q', '--release', '-p', 'kuna-jvm', '--example', 'corpus', '--',
           '--manifest', os.path.join(HERE, 'cases.tsv'), '--in-process', '--out', out]
    for case_id in case_ids:
        cmd += ['--filter', case_id]
    _, stdout, stderr = run(cmd + list(extra), cwd=os.path.join(REPO, 'decompiler'))
    statuses = {}
    summary = os.path.join(out, 'summary.tsv')
    if os.path.isfile(summary):
        with open(summary, encoding='utf-8') as f:
            for row in list(f)[1:]:
                fields = row.rstrip('\n').split('\t')
                statuses[fields[0]] = fields[2]
    return statuses, stdout + stderr


def regen():
    code, out, err = run([sys.executable, '-I', os.path.join(HERE, 'regen.py')])
    if code != 0:
        sys.exit(f'regen.py failed:\n{out}{err}')


def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument('--fix', required=True, help='the jvm_fix_* option the case demonstrates')
    ap.add_argument('--class', dest='cls', required=True, help='the class file (its Name$*.class siblings come along)')
    ap.add_argument('--source', help='the Java source the class was compiled from (vendored under src/)')
    ap.add_argument('--compiler', default='', help='the compiler that produced the class, e.g. "javac 17.0.20.1"')
    ap.add_argument('--summary', default='', help='one line: what the default output does instead of failing')
    ap.add_argument('--opt', action='append', default=[], help='an extra option pair k=v for both passes')
    ap.add_argument('--language', choices=('java', 'kotlin'),
                    help='the case\'s output language (default: kotlin if the class has kotlin.Metadata, else java)')
    a = ap.parse_args()
    if not os.environ.get('KUNA_JVM_ORACLE'):
        sys.exit('KUNA_JVM_ORACLE is not set: set it to the JVM oracle workspace '
                 '(decompiler/nonnative/kuna-jvm/tools/oracle/README.md)')
    if a.fix not in fix_options():
        sys.exit(f'{a.fix}: no such jvm_fix_* row in options.toml (add the row first)')
    extra = []
    for kv in a.opt:
        if '=' not in kv:
            sys.exit(f'--opt expects k=v, got {kv}')
        extra.append(kv.split('=', 1))

    with tempfile.TemporaryDirectory(prefix='kuna-fix-case-') as work:
        add_case(a, extra, work)


def add_case(a, extra, work):
    main_class = os.path.abspath(a.cls)
    name = class_name(main_class)
    language = a.language or ('kotlin' if is_kotlin(main_class) else 'java')
    ext = '.kt' if language == 'kotlin' else '.java'
    print(f'{name}: a {language} case')
    stem = os.path.basename(main_class)[:-len('.class')]
    siblings = sorted(glob.glob(os.path.join(glob.escape(os.path.dirname(main_class)), glob.escape(stem) + '$*.class')))
    package = os.path.dirname(name)
    case_id = name
    class_files = []
    for path in [main_class] + siblings:
        rel = '/'.join(filter(None, [package, os.path.basename(path)]))
        os.makedirs(os.path.join(FIXES, 'classes', package), exist_ok=True)
        shutil.copyfile(path, os.path.join(FIXES, 'classes', rel))
        class_files.append(rel)
    source = None
    if a.source:
        source = 'src/' + '/'.join(filter(None, [package, os.path.basename(a.source)]))
        os.makedirs(os.path.dirname(os.path.join(FIXES, source)), exist_ok=True)
        shutil.copyfile(a.source, os.path.join(FIXES, source))

    table_path = os.path.join(FIXES, 'cases.json')
    with open(table_path, encoding='utf-8') as f:
        table = json.load(f)
    vf_out = os.path.join(work, 'vf')
    cmd = [VF_DECOMPILE, '--fixture'] + (['--plugins'] if language == 'kotlin' else [])
    for k, v in table['options'] + extra:
        cmd += ['--opt', f'{k}={v}']
    cmd += [os.path.join(FIXES, 'classes', f) for f in class_files] + ['-o', vf_out]
    code, out, err = run(cmd)
    produced = os.path.join(vf_out, stem + ext)
    leak_only = err.count('Exception in thread') == 1 and LEAK in err
    if not os.path.isfile(produced) or (code != 0 and not leak_only):
        sys.exit(f'vf-decompile failed ({code}):\n{out}{err}')
    with open(produced, encoding='utf-8') as f:
        compat_text = f.read()
    if MARKER not in compat_text:
        sys.exit(f'Vineflower decompiles {name} without a failure comment: there is nothing for a fix to change')
    for sub in ('expected-compat', 'expected'):
        os.makedirs(os.path.dirname(os.path.join(FIXES, sub, case_id + ext)), exist_ok=True)
    with open(os.path.join(FIXES, 'expected-compat', case_id + ext), 'w', encoding='utf-8', newline='\n') as f:
        f.write(compat_text)
    default_path = os.path.join(FIXES, 'expected', case_id + ext)
    if not os.path.exists(default_path):
        shutil.copyfile(os.path.join(FIXES, 'expected-compat', case_id + ext), default_path)

    entry = {'id': case_id, 'fix': a.fix, 'class_files': class_files, 'source': source,
             'compiler': a.compiler, 'summary': a.summary}
    if language == 'kotlin':
        entry['language'] = 'kotlin'
    if extra:
        entry['options'] = extra
    table['cases'] = sorted([c for c in table['cases'] if c['id'] != case_id] + [entry], key=lambda c: c['id'])
    with open(table_path, 'w', encoding='utf-8', newline='\n') as f:
        json.dump(table, f, indent=2)
        f.write('\n')
    regen()

    compat_id, default_id = f'kuna-fixes-compat/{case_id}', f'kuna-fixes/{case_id}'
    run_out = os.path.join(work, 'run1')
    statuses, out = corpus([compat_id, default_id], run_out)
    actual = os.path.join(run_out, default_id + '.actual.java')
    if not os.path.isfile(actual):
        sys.exit(f'the corpus runner produced no output for {default_id}:\n{out}')
    if statuses.get(compat_id) != 'Match':
        with open(os.path.join(run_out, compat_id + '.diff'), encoding='utf-8', errors='replace') as f:
            sys.exit(f'{compat_id}: kuna in compat mode does not print what Vineflower prints; fix the port '
                     f'first:\n{f.read()}')
    with open(actual, encoding='utf-8') as f:
        default_text = f.read()
    with open(default_path, 'w', encoding='utf-8', newline='\n') as f:
        f.write(default_text)
    regen()

    diff = difflib.unified_diff(compat_text.splitlines(), default_text.splitlines(),
                                f'expected-compat/{case_id}{ext}', f'expected/{case_id}{ext}', lineterm='')
    print('\n'.join(diff))
    before, after = compat_text.count(MARKER), default_text.count(MARKER)
    print(f'\nfailed methods: {before} with jvm_vineflower_compat on, {after} with the defaults')
    if after >= before:
        print('warning: the fix does not decompile any method Vineflower fails on in this class')

    statuses, out = corpus([compat_id, default_id], os.path.join(work, 'run2'),
                           ['--save-baseline', os.path.join(HERE, 'baseline.txt')])
    if statuses.get(compat_id) != 'Match' or statuses.get(default_id) != 'Match':
        sys.exit(f'the two passes do not both match after recording:\n{out}')
    print(f'{compat_id} and {default_id} match and are in tests/jvm/baseline.txt; review the diff above, '
          f'then fill in the case\'s summary in kuna-fixes/cases.json (and rerun regen.py) if it is empty')


if __name__ == '__main__':
    main()
