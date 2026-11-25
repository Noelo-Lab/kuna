#!/usr/bin/env python3
"""Add (or refresh) a kuna-fixes case: a class Vineflower 1.12.0 fails on that a kuna fix decompiles.

  python3 -I tests/jvm/kuna_fix_case.py --fix <id> --class path/to/pkg/TestFoo.class \\
      [--source path/to/pkg/TestFoo.java] [--compiler "javac 17.0.20.1"] [--summary TEXT] [--opt k=v]...
      [--language java|kotlin]
      [--wrong-output] [--extra-class path/to/other/Lib.class]...

<id> is the fix's id, its `id` in decompiler/nonnative/kuna-jvm/fixes.toml (e.g. lambda_wildcard_return).
The class's `Name$*.class` siblings in the same directory come with it, in name order (the corpus
fixture's `addSource` order). --language defaults to kotlin when the class carries `kotlin.Metadata`, else
java. A Kotlin case (`"language": "kotlin"` in cases.json) is decompiled with Vineflower's bundled plugins
(`vf-decompile --plugins`; kuna: `jvm_plugins=1`) and its texts are `.kt`; below, EXT is `.java` or `.kt`.
Steps, all under tests/jvm/kuna-fixes/ (README.md there has the layout):

  1. copies the class files to classes/<package>/ and the source to src/<package>/;
  2. runs the released Vineflower jar through the oracle (`vf-decompile --fixture`, plus the options of
     cases.json, dump-exception-on-error=0, and every --opt); the text must contain Vineflower's failure
     comment, unless --wrong-output says the fix corrects code Vineflower prints wrongly;
  3. records the case in cases.json with that text's digest (`vineflower`, the compat run's expectation)
     and rebuilds cases.tsv and the READMEs with regen.py;
  4. runs `kuna test --jvm` on both passes: the compat pass must match Vineflower's text, and kuna's
     default output becomes expected/<id>EXT (printed as a diff against Vineflower's text, for review);
  5. runs both passes again: both must match.
The new case must also pass tests/fix_isolation.rs, and a fix that changes kuna's default text on
Vineflower's own inputs moves digests that make test-jvm reports (kuna/<id>): read those texts and
re-record them with `kuna test --jvm --save-digests` (docs/jvm.md, "Fixing a Vineflower bug").

Needs KUNA_JVM_ORACLE (the oracle workspace, decompiler/nonnative/kuna-jvm/tools/oracle/README.md) and
cargo; kuna is built in release mode (CARGO_TARGET_DIR is honoured).
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
sys.path.append(HERE)
from regen import VF_DECOMPILE, fix_ids, text_digest  # noqa: E402

REPO = os.path.dirname(os.path.dirname(HERE))
FIXES = os.path.join(HERE, 'kuna-fixes')
MARKER = "$VF: Couldn't be decompiled"
# The fixture's leak check (unit-test mode) fails after the output is saved when a method fails
# while it is written (once per such method); the saved text is still Vineflower's.
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


def run(cmd, cwd=None, env=None):
    r = subprocess.run(cmd, cwd=cwd, env=env, capture_output=True, text=True)
    return r.returncode, r.stdout, r.stderr


def corpus(case_ids, out):
    """Runs `kuna test --jvm` on the given case ids, writing their text under `out`: the ids that match and
    the runner's output."""
    cmd = ['cargo', 'run', '-q', '--release', '-p', 'kuna-cli', '--', 'test', '--jvm', '--json',
           '--manifest', os.path.join(HERE, 'cases.tsv'), '--out', out]
    for case_id in case_ids:
        cmd += ['--name', case_id]
    _, stdout, stderr = run(cmd, cwd=os.path.join(REPO, 'decompiler'))
    try:
        passing = {p.removeprefix('jvm:') for p in json.loads(stdout)['passing']}
    except (ValueError, KeyError):
        passing = set()
    return passing, stdout + stderr


def text_of(out, run_id):
    """The text `kuna test --jvm --out` wrote for a run (`<id>.kt` or `<id>.java`), or ''."""
    for ext in ('.kt', '.java'):
        if os.path.isfile(os.path.join(out, run_id + ext)):
            with open(os.path.join(out, run_id + ext), encoding='utf-8') as f:
                return f.read()
    return 


def regen():
    code, out, err = run([sys.executable, '-I', os.path.join(HERE, 'regen.py')])
    if code != 0:
        sys.exit(f'regen.py failed:\n{out}{err}')


def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument('--fix', required=True, help='the id of the fix the case demonstrates (its fixes.toml id)')
    ap.add_argument('--class', dest='cls', required=True, help='the class file (its Name$*.class siblings come along)')
    ap.add_argument('--source', help='the Java source the class was compiled from (vendored under src/)')
    ap.add_argument('--compiler', default='', help='the compiler that produced the class, e.g. "javac 17.0.20.1"')
    ap.add_argument('--summary', default='', help='one line: what the default output does instead of failing')
    ap.add_argument('--opt', action='append', default=[], help='an extra option pair k=v for both passes')
    ap.add_argument('--language', choices=('java', 'kotlin'),
                    help='the case\'s output language (default: kotlin if the class has kotlin.Metadata, else java)')
    ap.add_argument('--extra-class', action='append', default=[],
                    help='another class of the input (e.g. a superclass in another package), vendored after the siblings')
    ap.add_argument('--wrong-output', action='store_true',
                    help='the fix corrects output Vineflower prints wrongly: no failure comment is required')
    a = ap.parse_args()
    if not os.environ.get('KUNA_JVM_ORACLE'):
        sys.exit('KUNA_JVM_ORACLE is not set: set it to the JVM oracle workspace '
                 '(decompiler/nonnative/kuna-jvm/tools/oracle/README.md)')
    ids = fix_ids(REPO)
    if a.fix not in ids:
        sys.exit(f'{a.fix}: no such fix id (add its row to fixes.toml first); the ids are: '
                 + ', '.join(ids))
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
    for path in [main_class] + siblings + [os.path.abspath(x) for x in a.extra_class]:
        rel = '/'.join(filter(None, [os.path.dirname(class_name(path)), os.path.basename(path)]))
        os.makedirs(os.path.join(FIXES, 'classes', os.path.dirname(rel)), exist_ok=True)
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
    cmd = [os.path.join(REPO, VF_DECOMPILE), '--fixture'] + (['--plugins'] if language == 'kotlin' else [])
    for k, v in table['options'] + extra:
        cmd += ['--opt', f'{k}={v}']
    cmd += [os.path.join(FIXES, 'classes', f) for f in class_files] + ['-o', vf_out]
    code, out, err = run(cmd)
    produced = os.path.join(vf_out, stem + ext)
    leak_only = LEAK in err and err.count('Exception in thread') == err.count(LEAK)
    if not os.path.isfile(produced) or (code != 0 and not leak_only):
        sys.exit(f'vf-decompile failed ({code}):\n{out}{err}')
    with open(produced, encoding='utf-8') as f:
        compat_text = f.read()
    if MARKER not in compat_text and not a.wrong_output:
        sys.exit(f'Vineflower decompiles {name} without a failure comment: there is nothing for a fix to change')
    default_path = os.path.join(FIXES, 'expected', case_id + ext)
    os.makedirs(os.path.dirname(default_path), exist_ok=True)
    if not os.path.exists(default_path):
        with open(default_path, 'w', encoding='utf-8', newline='\n') as f:
            f.write(compat_text)

    entry = {'id': case_id, 'fix': a.fix, 'vineflower': text_digest(compat_text.encode('utf-8')),
             'class_files': class_files, 'source': source, 'compiler': a.compiler, 'summary': a.summary}
    if language == 'kotlin':
        entry['language'] = 'kotlin'
    if extra:
        entry['options'] = extra
    ids = [c['id'] for c in table['cases']]
    if case_id in ids:
        table['cases'][ids.index(case_id)] = entry
    else:
        table['cases'].append(entry)
    with open(table_path, 'w', encoding='utf-8', newline='\n') as f:
        head = [f'  {json.dumps(k)}: {json.dumps(v)}' for k, v in table.items() if k != 'cases']
        f.write('{\n' + ',\n'.join(head) + ',\n  "cases": [\n')
        f.write(',\n'.join('    ' + json.dumps(c) for c in table['cases']) + '\n  ]\n}\n')
    regen()

    compat_id, default_id = f'kuna-fixes-compat/{case_id}', f'kuna-fixes/{case_id}'
    run_out = os.path.join(work, 'run1')
    passing, out = corpus([compat_id, default_id], run_out)
    default_text = text_of(run_out, default_id)
    if not default_text:
        sys.exit(f'kuna test --jvm produced no output for {default_id}:\n{out}')
    if compat_id not in passing:
        diff = difflib.unified_diff(compat_text.splitlines(), text_of(run_out, compat_id).splitlines(),
                                    f'vineflower/{case_id}{ext}', f'kuna/{compat_id}{ext}', lineterm='')
        sys.exit(f'{compat_id}: kuna in compat mode does not print what Vineflower prints; fix the port '
                 f'first:\n' + '\n'.join(diff))
    with open(default_path, 'w', encoding='utf-8', newline='\n') as f:
        f.write(default_text)
    regen()

    diff = difflib.unified_diff(compat_text.splitlines(), default_text.splitlines(),
                                f'vineflower/{case_id}{ext}', f'expected/{case_id}{ext}', lineterm='')
    print('\n'.join(diff))
    before, after = compat_text.count(MARKER), default_text.count(MARKER)
    print(f'\nfailed methods: {before} with jvm_vineflower_compat on, {after} with the defaults')
    if a.wrong_output:
        if default_text == compat_text:
            print('warning: the fix does not change the output of this class')
    elif after >= before:
        print('warning: the fix does not decompile any method Vineflower fails on in this class')

    passing, out = corpus([compat_id, default_id], os.path.join(work, 'run2'))
    if compat_id not in passing or default_id not in passing:
        sys.exit(f'the two passes do not both match after recording:\n{out}')
    print(f'{compat_id} and {default_id} match; review the diff above, '
          f'then fill in the case\'s summary in kuna-fixes/cases.json (and rerun regen.py) if it is empty')


if __name__ == '__main__':
    main()
