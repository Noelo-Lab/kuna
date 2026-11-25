#!/usr/bin/env python3
"""Rebuild tests/jvm/ (the vendored kuna-jvm regression corpus) from the Vineflower oracle corpus.

  python3 -I tests/jvm/regen.py [--corpus DIR] [--out tests/jvm] [--full-manifest FILE]
  python3 -I tests/jvm/regen.py [--corpus DIR] --expected-text NEW_DIR CASE...

--corpus defaults to $KUNA_JVM_ORACLE/corpus, the Java oracle corpus that
decompiler/nonnative/kuna-jvm/tools/oracle builds (its README.md). Reads <corpus>/manifest.json and writes, under --out (default: this script's directory):
  cases.tsv          the kuna-jvm corpus manifest (format: README.md) for every vendored case, each with the
                     digest of the Vineflower 1.12.0 output it is compared with (<corpus>/expected)
  classes/...        the class files those cases feed the decompiler (same layout as <corpus>/classes)
  README.md          provenance, counts and the excluded cases

The cases of plugins.json (Vineflower's bundled plugins, with kuna's jvm_plugins switch on) and of
kotlin.json (the Kotlin oracle corpus) are appended to cases.tsv with the digests those files record;
their class files live in this directory (written by plugin_oracle.py and kotlin_corpus.py) and are
copied when --out is elsewhere. So are the kuna-fixes cases (kuna-fixes/cases.json, added by
kuna_fix_case.py), each as two runs: Vineflower-compatible (jvm_vineflower_compat on, pinned by the
case's `vineflower` digest) and default (expected kuna-fixes/expected/); kuna-fixes/README.md is
written from the same table. Each case keeps the `kuna` digest of its default-mode run from the
current cases.tsv (`-` for a new case).

Existing files are overwritten, nothing is deleted; regenerate into a fresh directory to drop stale
files. --full-manifest additionally writes a manifest of all runnable cases (JDK-dependent ones
included) whose paths point into <corpus> itself.

--expected-text writes the Vineflower text of each named compat run (a case id of cases.tsv; not a
kuna/<id> run) to NEW_DIR/<id>.java or .kt, checks it against the digest cases.tsv pins, and exits:
a Java case's from <corpus>/expected, a Kotlin case's from <corpus>/../corpus-kt/expected, a plugin
case's from upstream's .dec in the Vineflower checkout (--vf-src, default <corpus>/../vineflower) or
<corpus>/expected-defaults, which matches it under the fixture's comparison, and a kuna-fixes case's
from a run of the oracle tooling's vf-decompile.
"""
import argparse
import hashlib
import json
import os
import re
import shutil
import subprocess
import sys
import tempfile

NO_ORACLE = ('KUNA_JVM_ORACLE is not set: set it to the JVM oracle workspace '
             '(decompiler/nonnative/kuna-jvm/tools/oracle/README.md), or pass --corpus')
FORMAT = 'kuna-jvm-corpus\t1'


def esc(s):
    return s.replace('\\', '\\\\').replace('\t', '\\t').replace('\n', '\\n').replace('\r', '\\r')


def line(*fields):
    return '\t'.join(esc(str(f)) for f in fields) + '\n'


def text_digest(data):
    """kuna_corpus::text_digest of a file's bytes: the first 16 hex digits of the SHA-256 of the text after
    the fixture's normalization (CRLF -> LF, then Java's String.trim())."""
    text = data.decode('utf-8', errors='replace').replace('\r\n', '\n')
    i, j = 0, len(text)
    while i < j and text[i] <= ' ':
        i += 1
    while j > i and text[j - 1] <= ' ':
        j -= 1
    return hashlib.sha256(text[i:j].encode('utf-8')).hexdigest()[:16]


def file_digest(path):
    with open(path, 'rb') as f:
        return text_digest(f.read())


def set_records(s, base):
    """The set's effective option list: the fixture base map overlaid by the set's pairs."""
    opts = dict(base)
    javadoc = None
    for p in s['option_pairs']:
        v = p['value']
        if isinstance(v, dict) and '$javadoc_provider' in v:
            javadoc = v['$javadoc_provider']
        else:
            opts[p['key']] = v
    return opts, javadoc


REMOVED_OPTIONS = {'dump-text-tokens': '0'}


def removed_options(t):
    """The pairs of `t` that set an option kuna-jvm does not have to anything but the value it is fixed at
    (REMOVED_OPTIONS): such a test checks behaviour kuna deleted, so it is not run."""
    return [f'{k}={v}' for k, v in t['options'].items() if REMOVED_OPTIONS.get(k, v) != v]


def exclusion(t):
    rt = t['options'].get('include-runtime')
    if isinstance(rt, str) and rt.startswith('${java.'):
        return 'jdk', rt[len('${java.'):-len('.home}')]
    if t['options'].get('include-classpath') == '1':
        return 'classpath', '17'
    return None


def check_effective(t, sets, base):
    """The case's options must be exactly base + set pairs (+ the runtime injection)."""
    opts, javadoc = set_records(sets[t['set']], base)
    if t['runtime_injected'] is not None:
        opts['include-runtime'] = '${java.%s.home}' % t['runtime_injected']
    got = {k: v for k, v in t['options'].items() if not isinstance(v, dict)}
    got_javadoc = [v['$javadoc_provider'] for v in t['options'].values() if isinstance(v, dict)]
    if list(got.items()) != list(opts.items()) or got_javadoc != ([javadoc] if javadoc else []):
        sys.exit(f'{t["id"]}: manifest options differ from base + set pairs')


def class_stems(root, classes):
    """`classes` (paths under `root`) as the stems of a `case` record, each standing for `STEM.class`
    and then its `STEM$*.class` siblings in name order; None when the list does not have that shape."""
    stems, i = [], 0
    while i < len(classes):
        stem = classes[i][:-len('.class')]
        folder, name = os.path.split(stem)
        group = [classes[i]] + [os.path.join(folder, n) for n in sorted(os.listdir(os.path.join(root, folder)))
                                if n.startswith(name + '$') and n.endswith('.class')]
        if classes[i:i + len(group)] != group:
            return None
        stems.append(stem)
        i += len(group)
    return stems


def case_records(root, case_id, set_name, classes, expected, compare, opts=(), expected_dir=None, vineflower=None,
                 kuna=None):
    """The records of one case. With `root` (the vendored manifest) the short form: class stems on the
    `case` line, and `expected`/`compare` only where they differ from what the runner derives (README.md).
    A case pinned by its `vineflower` digest has no expected text."""
    stems = class_stems(root, classes) if root else None
    out = [line('case', case_id, set_name, *(stems or []))]
    out += [line(record, d) for record, d in (('vineflower', vineflower), ('kuna', kuna)) if d]
    out += [line('opt', k, v) for k, v in opts]
    if not stems:
        out += [line('class', c) for c in classes]
    if not vineflower:
        found = None
        if root:
            found = os.path.join(expected_dir or 'expected/' + case_id.split('/')[0], case_id.split('/', 1)[1])
            found = next((found + e for e in ('.java', '.kt') if os.path.isfile(os.path.join(root, found + e))), None)
        if expected != found:
            out.append(line('expected', expected))
    stem, ext = os.path.basename(classes[0])[:-len('.class')], '.java' if vineflower else os.path.splitext(expected)[1]
    if not root or compare != [stem + ext] + ([stem + '.java'] if ext != '.java' else []):
        out.append(line('compare', *compare))
    return out


def pinned(path, record):
    """The `record` digest of each case of the manifest at `path`, if it exists."""
    digests, case = {}, None
    if os.path.isfile(path):
        with open(path, encoding='utf-8') as f:
            for fields in (row.rstrip('\n').split('\t') for row in f):
                if fields[0] == 'case':
                    case = fields[1]
                elif fields[0] == record:
                    digests[case] = fields[1]
    return digests


def plugin_records(pm, base, root, digests):
    """The set and case records of the plugin manifest: a fixture set starts from the fixture's base map."""
    set_lines, case_lines = [], []
    for s in pm['sets']:
        opts = dict(base) if s['fixture'] else {}
        for k, v in s['option_pairs']:
            opts[k] = v
        set_lines.append(line('set', s['name'], s['slug']))
        for k, v in opts.items():
            set_lines.append(line('opt', k, v))
    for t in sorted(pm['tests'], key=lambda t: t['id']):
        case_lines += case_records(root, t['id'], t['set'], ['classes/' + f for f in t['class_files']], None,
                                   [t['decompiled_file'], *filter(None, [t.get('decompiled_file_fallback')])],
                                   vineflower=t['vineflower'], kuna=digests.get(t['id'], '-'))
    return set_lines, case_lines


FIXES_DIR = 'kuna-fixes'
FIX_EXPECTED = f'{FIXES_DIR}/expected'
FIX_SETS = (('Kuna Fixes, Vineflower compat', 'kuna-fixes-compat', '1'), ('Kuna Fixes', 'kuna-fixes', '0'))


FIXES_TOML = 'decompiler/nonnative/kuna-jvm/fixes.toml'


def fix_ids(repo):
    """The ids of the kuna fixes (the `id`s of fixes.toml) and the names of the kuna-form options
    (options.toml rows with `change_kind = "kuna-form"`)."""
    with open(os.path.join(repo, FIXES_TOML), encoding='utf-8') as f:
        ids = re.findall(r'^\s*\{ id = "([a-z0-9_]+)"', f.read(), re.M)
    return ids + form_options(repo)


OPTIONS_TOML = 'decompiler/nonnative/kuna-jvm/options.toml'


def form_options(repo):
    """The names of the kuna-form options: a case of one prints Vineflower's form in compat mode (which
    turns the option off) and the option's form by default."""
    with open(os.path.join(repo, OPTIONS_TOML), encoding='utf-8') as f:
        rows = f.read().split('[[option]]')
    return [m.group(1) for row in rows if 'change_kind = "kuna-form"' in row
            for m in [re.search(r'^option = "(\w+)"', row, re.M)] if m]


def check_fixes(fm, repo):
    """Every kuna-fixes case names an existing fix by its id."""
    ids = set(fix_ids(repo))
    bad = [f'{c["id"]}: {c["fix"]}' for c in fm['cases'] if c['fix'] not in ids]
    if bad:
        sys.exit('kuna-fixes/cases.json: no such fix id (an id of ' + FIXES_TOML + '): '
                 + ', '.join(bad))


def fix_ext(c):
    """The extension of a kuna-fixes case's text: `.kt` for a Kotlin case (`"language": "kotlin"`), else `.java`."""
    return '.kt' if c.get('language') == 'kotlin' else '.java'


def fix_compare(c):
    """The units a kuna-fixes case compares: its class's, then its `compare_fallback` classes'."""
    stem = os.path.basename(c['class_files'][0])[:-len('.class')]
    names = [n for unit in [stem] + c.get('compare_fallback', []) for n in (unit + fix_ext(c), unit + '.java')]
    return list(dict.fromkeys(names))


def fix_records(fm, base, root):
    """The set and case records of the kuna-fixes table: every case twice, compat (pinned by the case's
    `vineflower` digest) and default (kuna's reviewed text under kuna-fixes/expected/)."""
    set_lines, case_lines = [], []
    for name, slug, compat in FIX_SETS:
        opts = dict(base)
        for k, v in fm['options']:
            opts[k] = v
        opts['jvm_vineflower_compat'] = compat
        set_lines.append(line('set', name, slug, *([] if compat == '1' else [FIX_EXPECTED])))
        for k, v in opts.items():
            set_lines.append(line('opt', k, v))
    for name, slug, compat in FIX_SETS:
        for c in sorted(fm['cases'], key=lambda c: c['id']):
            opts = [('jvm_plugins', '1')] if c.get('language') == 'kotlin' else []
            case_lines += case_records(root, f'{slug}/{c["id"]}', name,
                                       [f'{FIXES_DIR}/classes/{f}' for f in c['class_files']],
                                       f'{FIX_EXPECTED}/{c["id"]}{fix_ext(c)}', fix_compare(c),
                                       opts + [tuple(o) for o in c.get('options', [])], FIX_EXPECTED,
                                       vineflower=c.get('vineflower', '-') if compat == '1' else None)
    return set_lines, case_lines


def fix_files(fm):
    """Every file of the kuna-fixes cases, relative to this directory."""
    files = set()
    for c in fm['cases']:
        files.update(f'{FIXES_DIR}/classes/{f}' for f in c['class_files'])
        files.add(f'{FIX_EXPECTED}/{c["id"]}{fix_ext(c)}')
        if c.get('source'):
            files.add(f'{FIXES_DIR}/{c["source"]}')
    return sorted(files)


FIXES_README = """# kuna-fixes: the two-pass cases of kuna's own fixes

Generated by `tests/jvm/regen.py` from `cases.json`; add a case with `tests/jvm/kuna_fix_case.py`
(`docs/jvm.md`, "Fixing a Vineflower bug"); do not edit by hand.

Each case is a small class that Vineflower 1.12.0 fails on (or prints wrong), run twice in the corpus
(`tests/jvm/cases.tsv`) on the fixture's base option map plus
{options}, so no exception text is printed:

- `kuna-fixes-compat/<id>`, `jvm_vineflower_compat` on: its text must be what the released
  `vineflower-1.12.0.jar` prints (`vf-decompile --fixture`), failure comment included, pinned by the
  case's `vineflower` digest in `cases.json`.
- `kuna-fixes/<id>`, kuna's defaults: `expected/<id>.java` is kuna's reviewed output.

A Kotlin case (`"language": "kotlin"` in `cases.json`) also runs with `jvm_plugins=1`, its oracle with
Vineflower's bundled plugins (`vf-decompile --plugins`), and its texts are `<id>.kt`. A case with
`compare_fallback` compares the unit of the class kuna writes it inside. `classes/` holds the class files,
`src/` their source. What a fix changes on its case: `python3 -I tests/jvm/regen.py --expected-text DIR
kuna-fixes-compat/<id>` writes Vineflower's text, then `diff -u DIR/kuna-fixes-compat/<id>.java expected/<id>.java`.

| Case | Fix id | Compiled with | With the fix |
|---|---|---|---|
{rows}
"""


def fix_readme(fm):
    rows = [f'| `{c["id"]}` | `{c["fix"]}` | {c["compiler"]} | {c["summary"]} |'
            for c in sorted(fm['cases'], key=lambda c: c['id'])]
    options = ', '.join(f'`{k}={v}`' for k, v in fm['options'])
    return FIXES_README.format(options=options, rows='\n'.join(rows))


def write_manifest(path, tests, sets, base, root, java_homes, rel_class, rel_expected, extra=([], []), vendored=None,
                   digests=None, vineflower=None):
    used_sets = []
    for t in tests:
        if t['set'] not in used_sets:
            used_sets.append(t['set'])
    out = [line('#', 'kuna-jvm corpus manifest; format: tests/jvm/README.md; generated by tests/jvm/regen.py'), FORMAT + '\n']
    if root is not None:
        out.append(line('root', root))
    for v, home in sorted(java_homes.items(), key=lambda kv: int(kv[0])):
        if home:
            out.append(line('java_home', v, home))
    for name in used_sets:
        s = sets[name]
        opts, javadoc = set_records(s, base)
        out.append(line('set', name, s['slug']))
        for k, v in opts.items():
            out.append(line('opt', k, v))
        if javadoc:
            out.append(line('javadoc', javadoc))
        if opts.get('include-classpath') == '1':
            out.append(line('current_jdk', '17'))
    out.extend(extra[0])
    for t in tests:
        opts = [] if t['runtime_injected'] is None else [('include-runtime', '${java.%s.home}' % t['runtime_injected'])]
        names = [t['decompiled_file']] + ([t['decompiled_file_fallback']] if t.get('decompiled_file_fallback') else [])
        out += case_records(vendored, t['id'], t['set'], [rel_class(f) for f in t['class_files']],
                            rel_expected(t['oracle']), names, opts, vineflower=vineflower and vineflower[t['id']],
                            kuna=None if digests is None else digests.get(t['id'], '-'))
    out.extend(extra[1])
    tmp = path + '.tmp'
    with open(tmp, 'w', encoding='utf-8', newline='\n') as f:
        f.writelines(out)
    os.replace(tmp, path)


def copy(src, dst):
    os.makedirs(os.path.dirname(dst), exist_ok=True)
    shutil.copyfile(src, dst)


def tree_size(paths):
    return sum(os.path.getsize(p) for p in paths)


VF_DECOMPILE = 'decompiler/nonnative/kuna-jvm/tools/oracle/java/bin/vf-decompile'


def vf_fixture(c, fm, here, work):
    """What the release jar prints for a kuna-fixes case (`vf-decompile --fixture`, plus `--plugins` for a
    Kotlin case, with the table's and the case's options): the path of the first unit the case compares.
    The exit status is not read: the fixture's leak check fails a run whose text is complete."""
    out = os.path.join(work, c['id'])
    os.makedirs(out)
    cmd = [os.path.join(os.path.dirname(os.path.dirname(here)), VF_DECOMPILE), '--fixture']
    cmd += ['--plugins'] if c.get('language') == 'kotlin' else []
    for k, v in fm['options'] + c.get('options', []):
        cmd += ['--opt', f'{k}={v}']
    cmd += [os.path.join(here, FIXES_DIR, 'classes', f) for f in c['class_files']] + ['-o', out]
    subprocess.run(cmd, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL, check=False)
    return next((os.path.join(out, n) for n in fix_compare(c) if os.path.isfile(os.path.join(out, n))), None)


def expected_texts(dest, ids, corpus, vf_src, m, pm, km, fm, here):
    """Writes Vineflower's text of each compat run of `ids` to dest/<id>.java or .kt from the oracle workspace
    (the module docstring has the sources) and checks it against the digest cases.tsv pins; the exit status."""
    workspace = os.path.dirname(corpus)
    pins = pinned(os.path.join(here, 'cases.tsv'), 'vineflower')
    java, plugins = {t['id']: t for t in m['tests']}, {t['id']: t for t in pm['tests']}
    kotlin, fixes = {t['id']: t for t in km['tests']}, {'kuna-fixes-compat/' + c['id']: c for c in fm['cases']}
    failed = 0
    with tempfile.TemporaryDirectory(prefix='kuna-jvm-vf-') as work:
        for case_id in ids:
            if case_id not in pins:
                print(f'{case_id}: not a case cases.tsv pins by a vineflower digest (a kuna/<id> run has none)')
                failed += 1
                continue
            if case_id in java:
                src, ext = os.path.join(corpus, java[case_id]['oracle']), os.path.splitext(java[case_id]['oracle'])[1]
            elif case_id in kotlin:
                t = kotlin[case_id]
                src, ext = os.path.join(workspace, 'corpus-kt', t['oracle']), os.path.splitext(t['oracle'])[1]
            elif case_id in plugins:
                t = plugins[case_id]
                src = (os.path.join(vf_src, t['reference']) if t['reference'] else
                       os.path.join(corpus, 'expected-defaults', 'custom', t['decompiled_file']))
                ext = os.path.splitext(t['decompiled_file'])[1]
            else:
                src, ext = vf_fixture(fixes[case_id], fm, here, work), fix_ext(fixes[case_id])
            if not src or not os.path.isfile(src):
                print(f'{case_id}: no text{f" ({src} is missing)" if src else ""}')
                failed += 1
                continue
            with open(src, 'rb') as f:
                data = f.read()
            dst = os.path.join(dest, case_id + ext)
            os.makedirs(os.path.dirname(dst), exist_ok=True)
            with open(dst, 'wb') as f:
                f.write(data)
            got = text_digest(data)
            print(f'{dst}: digest {got}, pinned {pins[case_id]}' + ('' if got == pins[case_id] else ' (DIFFERENT)'))
            failed += got != pins[case_id]
    return 1 if failed else 0


README = """# kuna-jvm regression corpus

The differential corpus for `kuna-jvm`, the Rust port of Vineflower 1.12.0 and its bundled plugins:
for each case, the class files one of Vineflower's own tests feeds the decompiler (core's
`SingleClassesTest`, or the tests of a bundled plugin), the option map it runs with, and the digest of
the exact Java or Kotlin text the released `vineflower-1.12.0.jar` prints for it. kuna-jvm passes a
case when it prints the same text. These Vineflower-oracle cases run with `jvm_vineflower_compat` on,
so kuna's own fixes of Vineflower's failures stay off; those fixes have cases of their own
(`kuna-fixes/`, below).

Generated by `tests/jvm/regen.py` from the Java oracle corpus (`$KUNA_JVM_ORACLE/corpus`, built by
`decompiler/nonnative/kuna-jvm/tools/oracle`, whose `README.md` documents how it was built and
validated); do not edit by hand.

| Path | What |
|---|---|
| `cases.tsv` | the manifest: about one line per case (format below) |
| `classes/<source set>/...` | the class files, laid out as in the oracle corpus |
| `regen.py` | rebuilds everything here; `--expected-text` writes Vineflower's text of named cases (below) |
| `plugins.json` | the plugin cases: Vineflower's bundled plugins' own tests, run with the plugins on |
| `plugin_oracle.py` | rebuilds the plugin cases' class files (`classes/plugins/`) and digests with the release jar |
| `kotlin.json` | the Kotlin cases: the Kotlin plugin's own tests and core's Kotlin tests, from the Kotlin oracle corpus |
| `kotlin_corpus.py` | vendors the Kotlin cases (`classes/kotlin/`, `kotlin.json`) from the Kotlin oracle corpus (`$KUNA_JVM_ORACLE/corpus-kt`), and writes the manifest of all its forms |
| `kuna-fixes/` | kuna's own cases: classes Vineflower fails on that a kuna fix decompiles, each run in compat and in default mode (`kuna-fixes/README.md`) |
| `kuna_fix_case.py` | adds a kuna-fixes case from a class file: Vineflower's digest through the oracle, kuna's text for review, the table |

## Provenance

- Test sources: Vineflower 1.12.0 (`b8273988`) `testData/src`, Apache-2.0 (see `LICENSE`/`NOTICE`
  at the repo root); registrations replayed from `SingleClassesTest`.
- Class files: compiled per source set with the JDK of that language version, the way Vineflower's
  Gradle build does (`-g`, `-source N -target N`, set-specific flags such as `-g:none` and
  `--enable-preview`); `jasm` classes are assembled with asmtools-core 7.0.b10-ea, `custom` classes
  are Vineflower's checked-in binaries:
{compilers}
- Pinned text: `vineflower-1.12.0.jar` (sha256 `1dfcfe97...b9d`) on OpenJDK 17.0.20.1 through
  `ConsoleDecompiler` with the test fixture's options and saver (the oracle corpus's `expected/`).
  {nmatch} of the {included} vendored outputs pass the fixture's comparison with upstream's
  `testData/results/*.dec` (or its `_1.dec`).

## What a case is

One registered test: an id `<set-slug>/<test class>`, its test set, the ordered option pairs (the
fixture's base map `log-level=warn decompile-generics=1 remove-synthetic=1 remove-bridge=1
keep-literals=1 __unit_test_mode__=1 new-line-separator=1 error-message="" include-runtime=""`
overlaid by the set's pairs), the class files in the exact `addSource` order (the test class, its
`Name$*.class` siblings in name order, then each companion the same way, duplicates kept), and the
output file the test compares. The runner (`kuna_jvm::kuna_corpus`) hands the pairs to `Fernflower`
(which overlays them on Vineflower's defaults; no plugin is loaded unless the pairs turn kuna's
`jvm_plugins` on, as the plugin sets do; `jvm_vineflower_compat` is on unless the pairs turn it off, as
the default pass of a kuna-fixes case does), adds each class file as a source, decompiles, takes
the last delivered unit whose file name is the compared one (the fixture's saver flattens every
class into one directory), normalizes its text as the fixture does (`\\r\\n` -> `\\n`, then Java
`String.trim()`), and compares the digest of that text with the pinned one (or, for a case with an
expected file, the text itself with that file's, normalized the same way).

## Format of `cases.tsv`

Line oriented, tab separated; fields escape `\\\\`, `\\t`, `\\n`, `\\r`; `#` starts a comment line.

| Record | Meaning |
|---|---|
| `kuna-jvm-corpus` `1` | format header (first record) |
| `root` `DIR` | base of relative paths (default: the manifest's directory) |
| `java_home` `N` `DIR` | resolves `${{java.N.home}}` in option values |
| `set` `NAME` `SLUG` [`DIR`] | starts a test set; the `opt`/`javadoc`/`current_jdk` lines after it are the set's; its cases' expected texts, if any, are under `DIR` (default `expected/SLUG`) |
| `case` `ID` `SET` [`STEM`...] | starts a case of set `SET`; following `opt`/`javadoc`/`current_jdk` lines overlay the set's; each `STEM` (a path without `.class`) adds `STEM.class`, then its `STEM$*.class` siblings in name order |
| `vineflower` `HEX` | the case's text, Vineflower 1.12.0's, is pinned by this digest instead of an expected file: the first 16 hex digits of the SHA-256 of the text after the fixture's normalization |
| `kuna` `HEX` | the case also runs with kuna's defaults (`jvm_vineflower_compat` off) as `kuna/ID`, whose text must have this digest |
| `opt` `KEY` `VALUE` | an option pair (a repeated key replaces the earlier value in place) |
| `javadoc` `plain\\|markdown` | the set passes an `IFabricJavadocProvider` object as `fabric:javadoc` |
| `current_jdk` `N` | `include-classpath=1`: the "current" JDK must be JDK `N` |
| `class` `PATH` | a class file, in `addSource` order, after the stems' |
| `lib` `PATH` | a library (`addLibrary`), added before the sources |
| `expected` `PATH` | the expected Java (or Kotlin) text of a case without `vineflower` (default: `DIR/<ID without SLUG/>.java`, or `.kt` when only that exists) |
| `compare` `NAME`... | the output file compared; later names are fallbacks (default: the first class file's name with the expected text's extension, `.java` for a case with `vineflower`, then `.java`) |

## Counts

- Vendored: **{ncases} runs** in all, each of which must match: the {noracle} Vineflower-oracle cases (the
  Java cases here, the plugin and Kotlin cases below) twice, in compat mode against Vineflower's text and
  with kuna's defaults against kuna's (their `vineflower` and `kuna` digests), and {nfixruns} runs of the
  {nfix} kuna-fixes cases (below).
- Java cases: **{included}** ({sets}), {nclasses} class files ({cls_kb} KiB); {total_kb} KiB in all
  with `cases.tsv`.
- Of the oracle corpus's {runnable} runnable tests, **{excluded} are not vendored** because their
  output depends on a particular JDK's class library; run them from the external corpus (below).
  {skipped} registered tests are not runnable from it (Groovy and Scala sources it never compiles, and the
  Kotlin ones, which the Kotlin cases below cover), and {removed_note}.
- {unsupported_note}
- Plugin cases (`plugins.json`): **{nplugin}** ({plugin_sets}). The fixture sets replay the tests of
  Vineflower's bundled plugins (`plugins/idea-not-null`, `plugins/variable-renaming`: the plugin's
  `testData/classes/custom` binaries and its `testData/src/java8` compiled with javac 1.8 like the corpus's
  `java8` set) on the fixture's base map; *Plugin Defaults* is `java -jar` with no options. Each set turns
  `jvm_plugins` on; the pinned text is the release jar's with its bundled plugins loaded
  (`vf-decompile --plugins`), which `plugin_oracle.py` accepts only when it matches upstream's `.dec` (for
  *Plugin Defaults*, the oracle corpus's `expected-defaults`) under the fixture's comparison. Rebuild them with
  `python3 -I tests/jvm/plugin_oracle.py --work NEW_DIR` (the corpus and the Vineflower checkout default to
  `$KUNA_JVM_ORACLE/corpus` and `$KUNA_JVM_ORACLE/vineflower`).

## Kotlin cases

`kotlin.json` holds **{nkotlin}** cases ({kotlin_sets}): {kt_nclasses} class files ({kt_cls_kb} KiB).
They come from the Kotlin oracle corpus `$KUNA_JVM_ORACLE/corpus-kt` (the oracle's `README.md`
documents the build: kotlinc 2.0.0, JVM target 17, the plugin's own Gradle setup; all 73 of its fixture
outputs pass the fixture comparison with upstream's `.dec`). Each case replays one test of
Vineflower's Kotlin plugin (`KotlinTests`: sets *Kotlin Entire Classpath* and *Kotlin Multifile Facade*,
only the Kotlin plugin loaded, so here `jvm_plugins` is on; the release jar's other two bundled plugins do
not change these outputs) or one `KOTLIN` test of core `SingleClassesTest` (*Kotlin Core Default*, no
plugin: the Kotlin classes come out as Java). A set's pairs are the complete effective option map; the
compared file is `<class file name>.kt`, falling back to `.java` as the fixture does.

Not vendored, because they need the Kotlin standard library or a JDK: the core *Entire Classpath* tests
({kt_excluded}: `include-classpath=1` with kotlin-stdlib on the oracle JVM's class path) and the corpus's
two plain-CLI forms (`java -jar` defaults, with and without `-e=kotlin-stdlib`). Run all 73 cases in all
three forms from the corpus itself:

```sh
python3 -I tests/jvm/kotlin_corpus.py --skip-vendor --full-manifest kt-all.tsv
decompiler/target/release/kuna test --jvm --manifest kt-all.tsv
```

The *Entire Classpath* cases need the current JDK to be 17 (`JAVA_HOME`); the plain-CLI forms use the
current JDK as their runtime (JDK 17 and 21 give the same text). `kotlin_corpus.py --overlays NEW_DIR`
also replays the 66 plugin cases under option overlays (each Kotlin plugin option, `kt-enable=0`, a few
core options) with the oracle and writes `NEW_DIR/cases.tsv`. Refresh the vendored cases with
`python3 -I tests/jvm/kotlin_corpus.py` (`--corpus-kt DIR` overrides `$KUNA_JVM_ORACLE/corpus-kt`), then `regen.py`.

### Not vendored

{excluded_list}

## kuna-fixes cases

`kuna-fixes/cases.json` holds **{nfix}** cases of kuna's own fixes (`docs/jvm.md`, "Fixing a Vineflower bug"):
a small class Vineflower 1.12.0 fails on, the fix (its `KunaFix` id) that decompiles it, and the source it was
compiled from. Each runs twice, on the fixture's base map plus `dump-exception-on-error=0`: as
`kuna-fixes-compat/<id>` with `jvm_vineflower_compat` on, against what the release jar prints (failure
comment included; the case's `vineflower` digest), and as `kuna-fixes/<id>` with kuna's defaults, against
kuna's reviewed output (`kuna-fixes/expected/`). `kuna-fixes/README.md` lists them. Add one with
`python3 -I tests/jvm/kuna_fix_case.py --fix <id> --class Foo.class --source Foo.java --summary TEXT`.

## Digests

Vineflower's text is pinned by its digest, not stored. Only the generators write `vineflower` digests,
from the oracle: `regen.py` from the oracle corpus's `expected/`, `plugin_oracle.py` and
`kotlin_corpus.py` into `plugins.json` and `kotlin.json`, `kuna_fix_case.py` into
`kuna-fixes/cases.json`. Each Vineflower-oracle case also runs with kuna's defaults, as `kuna/<id>`,
against its `kuna` record, so the gate also pins what kuna's fixes and options print on Vineflower's own
inputs. Once a change of that text is reviewed, `kuna test --jvm --save-digests` re-records the `kuna`
records of the runs it made (never a `vineflower` one); `regen.py` keeps them, and writes `-` (no digest
yet) for a new case.

## When a case does not match

A digest mismatch prints both digests. `--out DIR` writes the text kuna printed, as `DIR/<id>.java` (or
`.kt`), and `regen.py --expected-text DIR CASE...` writes Vineflower's from the oracle workspace, checked
against the pinned digest:

```sh
decompiler/target/release/kuna test --jvm --name default/pkg/TestEnhancedForLoops --out ACTUAL
python3 -I tests/jvm/regen.py --expected-text EXPECTED default/pkg/TestEnhancedForLoops
diff -u EXPECTED/default/pkg/TestEnhancedForLoops.java ACTUAL/default/pkg/TestEnhancedForLoops.java
```

Most Vineflower-oracle cases' text is also upstream's `testData/results/<test class>.dec` at tag
`1.12.0`, equal under the fixture's comparison. A `kuna/<id>` run has no Vineflower text: read kuna's
(`--out`) against the change under review. A `kuna-fixes/<id>` run's expected text is
`kuna-fixes/expected/<id>.java`.

## Running

```sh
# the gate (make test-jvm)
decompiler/target/release/kuna test --jvm
# a few cases, writing their text
decompiler/target/release/kuna test --jvm --name TestEnhancedForLoops --out NEW_DIR
# all {runnable} runnable cases, JDK-dependent ones included, from the external corpus
python3 -I tests/jvm/regen.py --skip-vendor --full-manifest all-cases.tsv
decompiler/target/release/kuna test --jvm --manifest all-cases.tsv
```

`kuna test --help` lists the flags. An external manifest need not match in full on every machine (its
cases need the JDKs it names and, for some, a particular current JDK): compare its `FAILURES` list, or
the `passing` list of `--json`, with a run of the base. Regenerate this directory with
`python3 -I tests/jvm/regen.py` (`--corpus DIR` overrides `$KUNA_JVM_ORACLE/corpus`).
"""


def removed_note(removed):
    """The README's account of the tests not run because they set an option kuna-jvm does not have."""
    by_pairs = {}
    for t in removed:
        by_pairs.setdefault((', '.join(f'`{p}`' for p in removed_options(t)), t['set']), []).append(t['id'])
    if not by_pairs:
        return 'none sets an option kuna-jvm does not have'
    return '; '.join(f'{len(ids)} set {pairs} (set *{s}*), an option kuna-jvm does not have, and are not run: '
                     + ', '.join(f'`{i}`' for i in ids) for (pairs, s), ids in by_pairs.items())


def main():
    here = os.path.dirname(os.path.abspath(__file__))
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument('--corpus', help='the oracle corpus (holds manifest.json; default: $KUNA_JVM_ORACLE/corpus)')
    ap.add_argument('--out', default=here, help='directory to (re)build (default: %(default)s)')
    ap.add_argument('--full-manifest', help='also write a manifest of every runnable case, rooted at --corpus')
    ap.add_argument('--skip-vendor', action='store_true', help='only write --full-manifest; leave --out alone')
    ap.add_argument('--expected-text', metavar='NEW_DIR',
                    help="write Vineflower's text of each CASE (an id of cases.tsv) to NEW_DIR and exit")
    ap.add_argument('cases', nargs='*', metavar='CASE', help='the case ids of --expected-text')
    ap.add_argument('--vf-src', help='the Vineflower 1.12.0 checkout of --expected-text (default: '
                    '$KUNA_JVM_ORACLE/vineflower)')
    a = ap.parse_args()
    if bool(a.cases) != bool(a.expected_text):
        ap.error('--expected-text takes one or more CASE ids')
    if not a.corpus and not os.environ.get('KUNA_JVM_ORACLE'):
        sys.exit(NO_ORACLE)
    corpus = os.path.abspath(a.corpus or os.path.join(os.environ['KUNA_JVM_ORACLE'], 'corpus'))
    out = os.path.abspath(a.out)

    with open(os.path.join(corpus, 'manifest.json'), encoding='utf-8') as f:
        m = json.load(f)
    base = m['fixture_base_options']
    sets = {s['name']: s for s in m['sets']}
    runnable = sorted((t for t in m['tests'] if t['status'] == 'ok'), key=lambda t: t['id'])
    skipped = [t for t in m['tests'] if t['status'] != 'ok']
    removed = [t for t in runnable if removed_options(t)]
    runnable = [t for t in runnable if not removed_options(t)]
    for t in runnable:
        check_effective(t, sets, base)

    included = [t for t in runnable if exclusion(t) is None]
    excluded = [t for t in runnable if exclusion(t) is not None]

    with open(os.path.join(here, 'plugins.json'), encoding='utf-8') as f:
        pm = json.load(f)
    with open(os.path.join(here, 'kotlin.json'), encoding='utf-8') as f:
        km = json.load(f)
    with open(os.path.join(here, FIXES_DIR, 'cases.json'), encoding='utf-8') as f:
        fm = json.load(f)
    check_fixes(fm, os.path.dirname(os.path.dirname(here)))
    if a.expected_text:
        vf_src = os.path.abspath(a.vf_src or os.path.join(os.path.dirname(corpus), 'vineflower'))
        sys.exit(expected_texts(os.path.abspath(a.expected_text), a.cases, corpus, vf_src, m, pm, km, fm, here))

    if a.full_manifest:
        write_manifest(os.path.abspath(a.full_manifest), runnable, sets, base, corpus, m['java_homes'],
                       lambda f: 'classes/' + f, lambda p: p)
        print(f'wrote {a.full_manifest}: {len(runnable)} cases')
    if a.skip_vendor:
        return

    os.makedirs(out, exist_ok=True)
    class_files = sorted({f for t in included for f in t['class_files']})
    for f in class_files:
        copy(os.path.join(corpus, 'classes', f), os.path.join(out, 'classes', f))
    plugin_files = sorted({'classes/' + f for t in pm['tests'] + km['tests'] for f in t['class_files']})
    if out != here:
        for f in plugin_files + fix_files(fm) + [f'{FIXES_DIR}/cases.json']:
            copy(os.path.join(here, f), os.path.join(out, f))
    digests = pinned(os.path.join(here, 'cases.tsv'), 'kuna')
    vineflower = {t['id']: file_digest(os.path.join(corpus, t['oracle'])) for t in included}
    plugin_sets, plugin_cases = plugin_records(pm, base, out, digests)
    kotlin_sets, kotlin_cases = plugin_records(km, base, out, digests)
    fix_sets, fix_cases = fix_records(fm, base, out)
    write_manifest(os.path.join(out, 'cases.tsv'), included, sets, base, None, {},
                   lambda f: 'classes/' + f, lambda p: p,
                   (plugin_sets + kotlin_sets + fix_sets, plugin_cases + kotlin_cases + fix_cases), out, digests,
                   vineflower)
    os.makedirs(os.path.join(out, FIXES_DIR), exist_ok=True)
    with open(os.path.join(out, FIXES_DIR, 'README.md'), 'w', encoding='utf-8', newline='\n') as f:
        f.write(fix_readme(fm))

    with open(os.path.join(corpus, 'reports', 'match-fixture.json'), encoding='utf-8') as f:
        match = json.load(f)
    passed = {r['id'] for r in match['rows'] if r.get('result') in ('match', 'match-alt')}

    by_reason = {}
    for t in excluded:
        kind, v = exclusion(t)
        if kind == 'jdk':
            reason = f'`include-runtime` = the JDK {v} home (set *{t["set"]}*): needs that JDK\'s class library'
        else:
            reason = (f'`include-classpath=1` (set *{t["set"]}*): needs the current JDK to be 17 '
                      '(its system modules; the oracle JVM also had the Vineflower jar on `java.class.path`, '
                      'which kuna-jvm does not add)')
        by_reason.setdefault(reason, []).append(t['id'])
    excluded_list = '\n'.join(f'- {r} ({len(ids)}): ' + ', '.join(f'`{i}`' for i in ids)
                              for r, ids in by_reason.items())
    javadoc_cases = [t['id'] for t in included if set_records(sets[t['set']], base)[1]]
    unsupported_note = (
        'The runner installs the test\'s `fabric:javadoc` provider (`FixtureJavadocProvider`) for '
        + ', '.join(f'`{i}`' for i in javadoc_cases) + '.') if javadoc_cases else 'No vendored case needs a javadoc provider.'
    srcsets = sorted({f.split('/')[0] for f in class_files})
    compilers = '\n'.join(f'  - `{s}`: {m["source_sets"][s]["compiler"]}' for s in srcsets
                           if s in m['source_sets'] and m['source_sets'][s].get('java_version'))
    set_counts = {}
    for t in included:
        set_counts[t['set']] = set_counts.get(t['set'], 0) + 1
    plugin_counts = {}
    for t in pm['tests']:
        plugin_counts[t['set']] = plugin_counts.get(t['set'], 0) + 1
    kotlin_counts = {}
    for t in km['tests']:
        kotlin_counts[t['set']] = kotlin_counts.get(t['set'], 0) + 1
    kt_class = [os.path.join(out, 'classes', f) for f in sorted({f for t in km['tests'] for f in t['class_files']})]
    cls_paths = [os.path.join(out, 'classes', f) for f in class_files]
    cls_b = tree_size(cls_paths)
    man_b = os.path.getsize(os.path.join(out, 'cases.tsv'))
    readme = README.format(
        compilers=compilers, nmatch=len(passed & {t['id'] for t in included}),
        included=len(included), sets=', '.join(f'{n} {c}' for n, c in set_counts.items()),
        nclasses=len(class_files), cls_kb=cls_b // 1024,
        total_kb=(cls_b + man_b) // 1024, runnable=len(runnable), excluded=len(excluded),
        skipped=len(skipped), unsupported_note=unsupported_note, excluded_list=excluded_list,
        removed_note=removed_note(removed),
        nplugin=len(pm['tests']),
        noracle=len(included) + len(pm['tests']) + len(km['tests']), nfix=len(fm['cases']),
        nfixruns=len(FIX_SETS) * len(fm['cases']),
        ncases=2 * (len(included) + len(pm['tests']) + len(km['tests'])) + len(FIX_SETS) * len(fm['cases']),
        plugin_sets=', '.join(f'{n} {c}' for n, c in plugin_counts.items()),
        nkotlin=len(km['tests']), kotlin_sets=', '.join(f'{n} {c}' for n, c in kotlin_counts.items()),
        kt_nclasses=len(kt_class), kt_cls_kb=tree_size(kt_class) // 1024,
        kt_excluded=', '.join(f'`{e["id"]}`' for e in km['excluded']))
    with open(os.path.join(out, 'README.md'), 'w', encoding='utf-8', newline='\n') as f:
        f.write(readme)

    digest = hashlib.sha256()
    for p in sorted(cls_paths + [os.path.join(out, 'cases.tsv')]):
        with open(p, 'rb') as f:
            digest.update(os.path.relpath(p, out).encode() + b'\0' + f.read())
    print(f'vendored {len(included)} cases ({len(class_files)} class files, {cls_b} B; cases.tsv {man_b} B); '
          f'excluded {len(excluded)}; skipped {len(skipped)}; sha256 {digest.hexdigest()}')


if __name__ == '__main__':
    main()
