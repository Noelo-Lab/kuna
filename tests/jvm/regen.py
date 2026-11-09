#!/usr/bin/env python3
"""Rebuild tests/jvm/ (the vendored kuna-jvm regression corpus) from the Vineflower oracle corpus.

  python3 -I tests/jvm/regen.py [--corpus DIR] [--out tests/jvm] [--full-manifest FILE]

--corpus defaults to $KUNA_JVM_ORACLE/corpus, the Java oracle corpus that
decompiler/nonnative/kuna-jvm/tools/oracle builds (its README.md). Reads <corpus>/manifest.json and writes, under --out (default: this script's directory):
  cases.tsv          the kuna-jvm corpus manifest (format: README.md) for every vendored case
  classes/...        the class files those cases feed the decompiler (same layout as <corpus>/classes)
  expected/...       the Vineflower 1.12.0 output each case is compared with (<corpus>/expected)
  README.md          provenance, counts and the excluded cases

The cases of plugins.json (Vineflower's bundled plugins, with kuna's jvm_plugins switch on) and of
kotlin.json (the Kotlin oracle corpus) are appended to cases.tsv; their class files and expected text
live in this directory (written by plugin_oracle.py and kotlin_corpus.py) and are copied when --out is
elsewhere. So are the kuna-fixes cases (kuna-fixes/cases.json, added by kuna_fix_case.py), each as two
runs: Vineflower-compatible (jvm_vineflower_compat on, expected kuna-fixes/expected-compat/) and
default (expected kuna-fixes/expected/); kuna-fixes/README.md is written from the same table.

Existing files are overwritten, nothing is deleted; regenerate into a fresh directory to drop stale
files. --full-manifest additionally writes a manifest of all runnable cases (JDK-dependent ones
included) whose paths point into <corpus> itself.
"""
import argparse
import difflib
import hashlib
import json
import os
import shutil
import sys

NO_ORACLE = ('KUNA_JVM_ORACLE is not set: set it to the JVM oracle workspace '
             '(decompiler/nonnative/kuna-jvm/tools/oracle/README.md), or pass --corpus')
FORMAT = 'kuna-jvm-corpus\t1'


def esc(s):
    return s.replace('\\', '\\\\').replace('\t', '\\t').replace('\n', '\\n').replace('\r', '\\r')


def line(*fields):
    return '\t'.join(esc(str(f)) for f in fields) + '\n'


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


def plugin_records(pm, base):
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
        case_lines.append(line('case', t['id'], t['set']))
        for f in t['class_files']:
            case_lines.append(line('class', 'classes/' + f))
        case_lines.append(line('expected', t['oracle']))
        case_lines.append(line('compare', t['decompiled_file'], *filter(None, [t.get('decompiled_file_fallback')])))
    return set_lines, case_lines


FIXES_DIR = 'kuna-fixes'
FIX_SETS = (('Kuna Fixes, Vineflower compat', 'kuna-fixes-compat', '1', 'expected-compat'),
            ('Kuna Fixes', 'kuna-fixes', '0', 'expected'))


def fix_ext(c):
    """The extension of a kuna-fixes case's text: `.kt` for a Kotlin case (`"language": "kotlin"`), else `.java`."""
    return '.kt' if c.get('language') == 'kotlin' else '.java'


def fix_records(fm, base):
    """The set and case records of the kuna-fixes table: every case twice, compat and default."""
    set_lines, case_lines = [], []
    for name, slug, compat, expected in FIX_SETS:
        opts = dict(base)
        for k, v in fm['options']:
            opts[k] = v
        opts['jvm_vineflower_compat'] = compat
        set_lines.append(line('set', name, slug))
        for k, v in opts.items():
            set_lines.append(line('opt', k, v))
    for name, slug, _, expected in FIX_SETS:
        for c in sorted(fm['cases'], key=lambda c: c['id']):
            case_lines.append(line('case', f'{slug}/{c["id"]}', name))
            if c.get('language') == 'kotlin':
                case_lines.append(line('opt', 'jvm_plugins', '1'))
            for k, v in c.get('options', []):
                case_lines.append(line('opt', k, v))
            for f in c['class_files']:
                case_lines.append(line('class', f'{FIXES_DIR}/classes/{f}'))
            case_lines.append(line('expected', f'{FIXES_DIR}/{expected}/{c["id"]}{fix_ext(c)}'))
            stem = os.path.basename(c['class_files'][0])[:-len('.class')]
            case_lines.append(line('compare', *dict.fromkeys([stem + fix_ext(c), stem + '.java'])))
    return set_lines, case_lines


def fix_files(fm):
    """Every file of the kuna-fixes cases, relative to this directory."""
    files = set()
    for c in fm['cases']:
        files.update(f'{FIXES_DIR}/classes/{f}' for f in c['class_files'])
        files.update(f'{FIXES_DIR}/{e}/{c["id"]}{fix_ext(c)}' for _, _, _, e in FIX_SETS)
        if c.get('source'):
            files.add(f'{FIXES_DIR}/{c["source"]}')
    return sorted(files)


FIXES_README = """# kuna-fixes: the two-pass cases of kuna's own fixes

Generated by `tests/jvm/regen.py` from `cases.json`; add a case with `tests/jvm/kuna_fix_case.py`
(`docs/jvm.md`, "Fixing a Vineflower bug"); do not edit by hand.

Each case is a small class that Vineflower 1.12.0 fails on, and runs twice in the corpus
(`tests/jvm/cases.tsv`, gated by `tests/jvm/baseline.txt`):

- `kuna-fixes-compat/<id>`: `jvm_vineflower_compat` on. The expected text, `expected-compat/<id>.java`, is
  what the released `vineflower-1.12.0.jar` prints (the oracle's `vf-decompile --fixture`), failure comment
  included: kuna in compat mode still fails the method exactly as Vineflower does.
- `kuna-fixes/<id>`: kuna's defaults, every fix on. The expected text, `expected/<id>.java`, is kuna's
  output, reviewed by the diff below.

Both run with the fixture's base option map plus {options}: Vineflower's stack trace in a failure
comment is not kuna's, so no exception text is printed. A Kotlin case (`"language": "kotlin"` in
`cases.json`) also runs with `jvm_plugins=1`, its oracle with Vineflower's bundled plugins loaded
(`vf-decompile --plugins`), and its expected texts are `<id>.kt`. `classes/` holds the class files,
`src/` the source they were compiled from.

| Case | Fix | Compiled with | With the fix |
|---|---|---|---|
{rows}

## Diffs (compat -> default)
{diffs}"""


def fix_readme(fm, root):
    rows, diffs = [], []
    for c in sorted(fm['cases'], key=lambda c: c['id']):
        rows.append(f'| `{c["id"]}` | `{c["fix"]}` | {c["compiler"]} | {c["summary"]} |')
        texts = []
        for _, _, _, e in FIX_SETS:
            with open(os.path.join(root, FIXES_DIR, e, c['id'] + fix_ext(c)), encoding='utf-8') as f:
                texts.append(f.read().splitlines())
        diff = difflib.unified_diff(texts[0], texts[1], f'expected-compat/{c["id"]}{fix_ext(c)}',
                                    f'expected/{c["id"]}{fix_ext(c)}', lineterm='')
        diffs.append(f'\n### `{c["id"]}`\n\n```diff\n' + '\n'.join(diff) + '\n```\n')
    options = ', '.join(f'`{k}={v}`' for k, v in fm['options'])
    return FIXES_README.format(options=options, rows='\n'.join(rows), diffs=''.join(diffs))


def write_manifest(path, tests, sets, base, root, java_homes, rel_class, rel_expected, extra=([], [])):
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
        out.append(line('case', t['id'], t['set']))
        if t['runtime_injected'] is not None:
            out.append(line('opt', 'include-runtime', '${java.%s.home}' % t['runtime_injected']))
        for f in t['class_files']:
            out.append(line('class', rel_class(f)))
        out.append(line('expected', rel_expected(t['oracle'])))
        names = [t['decompiled_file']] + ([t['decompiled_file_fallback']] if t.get('decompiled_file_fallback') else [])
        out.append(line('compare', *names))
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


README = """# kuna-jvm regression corpus

The differential corpus for `kuna-jvm`, the Rust port of Vineflower 1.12.0 and its bundled plugins:
for each case, the class files one of Vineflower's own tests feeds the decompiler (core's
`SingleClassesTest`, or the tests of a bundled plugin), the option map it runs with, and the exact
Java or Kotlin text the released `vineflower-1.12.0.jar` prints for it. kuna-jvm passes a case when
it prints the same text. These Vineflower-oracle cases run with `jvm_vineflower_compat` on, so kuna's
own fixes of Vineflower's failures stay off; those fixes have cases of their own (`kuna-fixes/`, below).

Generated by `tests/jvm/regen.py` from the Java oracle corpus (`$KUNA_JVM_ORACLE/corpus`, built by
`decompiler/nonnative/kuna-jvm/tools/oracle`, whose `README.md` documents how it was built and
validated); do not edit by hand.

| Path | What |
|---|---|
| `cases.tsv` | the manifest: one record block per case (format below) |
| `classes/<source set>/...` | the class files, laid out as in the oracle corpus |
| `expected/<set slug>/<test class>.java` | Vineflower's output for each case, raw bytes (`.kt` for a case printed as Kotlin) |
| `baseline.txt` | the regression gate: ids of the cases kuna-jvm matches today; refresh it with `corpus --save-baseline ../tests/jvm/baseline.txt` when a change makes more cases match |
| `regen.py` | rebuilds everything here except `baseline.txt` |
| `plugins.json` | the plugin cases: Vineflower's bundled plugins' own tests, run with the plugins on |
| `plugin_oracle.py` | rebuilds the plugin cases' class files (`classes/plugins/`) and expected text with the release jar |
| `kotlin.json` | the Kotlin cases: the Kotlin plugin's own tests and core's Kotlin tests, from the Kotlin oracle corpus |
| `kotlin_corpus.py` | vendors the Kotlin cases (`classes/kotlin/`, `expected/kotlin-*/`, `kotlin.json`) from the Kotlin oracle corpus (`$KUNA_JVM_ORACLE/corpus-kt`), and writes the manifest of all its forms |
| `kuna-fixes/` | kuna's own cases: classes Vineflower fails on that a kuna fix decompiles, each run in compat and in default mode (`kuna-fixes/README.md`) |
| `kuna_fix_case.py` | adds a kuna-fixes case from a class file: Vineflower's text through the oracle, kuna's text for review, the table, the baseline |

## Provenance

- Test sources: Vineflower 1.12.0 (`b8273988`) `testData/src`, Apache-2.0 (see `LICENSE`/`NOTICE`
  at the repo root); registrations replayed from `SingleClassesTest`.
- Class files: compiled per source set with the JDK of that language version, the way Vineflower's
  Gradle build does (`-g`, `-source N -target N`, set-specific flags such as `-g:none` and
  `--enable-preview`); `jasm` classes are assembled with asmtools-core 7.0.b10-ea, `custom` classes
  are Vineflower's checked-in binaries:
{compilers}
- Expected text: `vineflower-1.12.0.jar` (sha256 `1dfcfe97...b9d`) on OpenJDK 17.0.20.1 through
  `ConsoleDecompiler` with the test fixture's options and saver. {nmatch} of the {included} vendored
  outputs pass the fixture's comparison with upstream's `testData/results/*.dec` (or its `_1.dec`).

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
class into one directory), and compares it with the expected text after the fixture's normalization
(`\\r\\n` -> `\\n`, then Java `String.trim()`).

## Format of `cases.tsv`

Line oriented, tab separated; fields escape `\\\\`, `\\t`, `\\n`, `\\r`; `#` starts a comment line.

| Record | Meaning |
|---|---|
| `kuna-jvm-corpus` `1` | format header (first record) |
| `root` `DIR` | base of relative paths (default: the manifest's directory) |
| `java_home` `N` `DIR` | resolves `${{java.N.home}}` in option values |
| `set` `NAME` `SLUG` | starts a test set; the `opt`/`javadoc`/`current_jdk` lines after it are the set's |
| `case` `ID` `SET` | starts a case of set `SET`; following `opt`/`javadoc`/`current_jdk` lines overlay the set's |
| `opt` `KEY` `VALUE` | an option pair (a repeated key replaces the earlier value in place) |
| `javadoc` `plain\\|markdown` | the set passes an `IFabricJavadocProvider` object as `fabric:javadoc` |
| `current_jdk` `N` | `include-classpath=1`: the "current" JDK must be JDK `N` |
| `class` `PATH` | a class file, in `addSource` order |
| `lib` `PATH` | a library (`addLibrary`), added before the sources |
| `expected` `PATH` | the expected Java (or Kotlin) text |
| `compare` `NAME`... | the output file compared; later names are fallbacks |

## Counts

- Vendored: **{ncases} cases** in all, the baseline: {noracle} Vineflower-oracle cases (the Java cases here,
  the plugin and Kotlin cases below) and {nfixruns} runs of the {nfix} kuna-fixes cases (below).
- Java cases: **{included}** ({sets}), {nclasses} class files ({cls_kb} KiB) and {included}
  expected files ({exp_kb} KiB); {total_kb} KiB in all with `cases.tsv`.
- Of the oracle corpus's {runnable} runnable tests, **{excluded} are not vendored** because their
  output depends on a particular JDK's class library; run them from the external corpus (below).
  {skipped} registered tests are not runnable from it (Groovy and Scala sources it never compiles, and the
  Kotlin ones, which the Kotlin cases below cover).
- {unsupported_note}
- Plugin cases (`plugins.json`): **{nplugin}** ({plugin_sets}). The fixture sets replay the tests of
  Vineflower's bundled plugins (`plugins/idea-not-null`, `plugins/variable-renaming`: the plugin's
  `testData/classes/custom` binaries and its `testData/src/java8` compiled with javac 1.8 like the corpus's
  `java8` set) on the fixture's base map; *Plugin Defaults* is `java -jar` with no options. Each set turns
  `jvm_plugins` on; the expected text is the release jar with its bundled plugins loaded
  (`vf-decompile --plugins`), which `plugin_oracle.py` accepts only when it matches upstream's `.dec` (for
  *Plugin Defaults*, the oracle corpus's `expected-defaults`) under the fixture's comparison. Rebuild them with
  `python3 -I tests/jvm/plugin_oracle.py --work NEW_DIR` (the corpus and the Vineflower checkout default to
  `$KUNA_JVM_ORACLE/corpus` and `$KUNA_JVM_ORACLE/vineflower`).

## Kotlin cases

`kotlin.json` holds **{nkotlin}** cases ({kotlin_sets}): {kt_nclasses} class files ({kt_cls_kb} KiB) and
{nkotlin} expected files ({kt_exp_kb} KiB). They come from the Kotlin oracle corpus `$KUNA_JVM_ORACLE/corpus-kt`
(the oracle's `README.md` documents the build: kotlinc 2.0.0, JVM target 17, the plugin's own Gradle setup; all
73 of its fixture outputs pass the fixture comparison with upstream's `.dec`). Each case replays one test of
Vineflower's Kotlin plugin (`KotlinTests`: sets *Kotlin Entire Classpath* and *Kotlin Multifile Facade*,
only the Kotlin plugin loaded, so here `jvm_plugins` is on; the release jar's other two bundled plugins do
not change these outputs) or one `KOTLIN` test of core `SingleClassesTest` (*Kotlin Core Default*, no
plugin: the Kotlin classes come out as Java). A set's pairs are the complete effective option map; the
compared file is `<class file name>.kt`, falling back to `.java` as the fixture does, and the expected
text is stored under the test class name like upstream's `.dec`.

Not vendored, because they need the Kotlin standard library or a JDK: the core *Entire Classpath* tests
({kt_excluded}: `include-classpath=1` with kotlin-stdlib on the oracle JVM's class path) and the corpus's
two plain-CLI forms (`java -jar` defaults, with and without `-e=kotlin-stdlib`). Run all 73 cases in all
three forms from the corpus itself:

```sh
python3 -I ../tests/jvm/kotlin_corpus.py --skip-vendor --full-manifest kt-all.tsv
cargo run -q --release -p kuna-jvm --example corpus -- --manifest kt-all.tsv --jobs 16
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
a small class Vineflower 1.12.0 fails on, the fix (`jvm_fix_*`) that decompiles it, and the source it was
compiled from. Each runs twice, on the fixture's base map plus `dump-exception-on-error=0`: as
`kuna-fixes-compat/<id>` with `jvm_vineflower_compat` on, against what the release jar prints
(`kuna-fixes/expected-compat/`, failure comment included), and as `kuna-fixes/<id>` with kuna's defaults,
against kuna's reviewed output (`kuna-fixes/expected/`). `kuna-fixes/README.md` lists them with the diff
between the two. Add one with
`python3 -I tests/jvm/kuna_fix_case.py --fix jvm_fix_<slug> --class Foo.class --source Foo.java --summary TEXT`.

## Running

```sh
cd decompiler
# the whole vendored corpus, one process per case (survives stack overflows, aborts and hangs)
cargo run -q --release -p kuna-jvm --example corpus -- --jobs 16 --out NEW_DIR
# a quick in-process run of a few cases
cargo run -q -p kuna-jvm --example corpus -- --filter TestEnhancedForLoops --out NEW_DIR
# the regression gate: fail on any case that matched in the baseline and no longer does
cargo run -q --release -p kuna-jvm --example corpus -- --jobs 16 --baseline ../tests/jvm/baseline.txt
# all {runnable} runnable cases, JDK-dependent ones included, from the external corpus
python3 -I ../tests/jvm/regen.py --skip-vendor --full-manifest all-cases.tsv
cargo run -q --release -p kuna-jvm --example corpus -- --manifest all-cases.tsv --jobs 16
```

`corpus --help` lists every flag. Regenerate this directory with `python3 -I tests/jvm/regen.py`
(`--corpus DIR` overrides `$KUNA_JVM_ORACLE/corpus`).
"""


def main():
    here = os.path.dirname(os.path.abspath(__file__))
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument('--corpus', help='the oracle corpus (holds manifest.json; default: $KUNA_JVM_ORACLE/corpus)')
    ap.add_argument('--out', default=here, help='directory to (re)build (default: %(default)s)')
    ap.add_argument('--full-manifest', help='also write a manifest of every runnable case, rooted at --corpus')
    ap.add_argument('--skip-vendor', action='store_true', help='only write --full-manifest; leave --out alone')
    a = ap.parse_args()
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
    for t in included:
        copy(os.path.join(corpus, t['oracle']), os.path.join(out, t['oracle']))
    plugin_files = sorted({'classes/' + f for t in pm['tests'] + km['tests'] for f in t['class_files']}
                          | {t['oracle'] for t in pm['tests'] + km['tests']})
    if out != here:
        for f in plugin_files + fix_files(fm) + [f'{FIXES_DIR}/cases.json']:
            copy(os.path.join(here, f), os.path.join(out, f))
    plugin_sets, plugin_cases = plugin_records(pm, base)
    kotlin_sets, kotlin_cases = plugin_records(km, base)
    fix_sets, fix_cases = fix_records(fm, base)
    write_manifest(os.path.join(out, 'cases.tsv'), included, sets, base, None, {},
                   lambda f: 'classes/' + f, lambda p: p,
                   (plugin_sets + kotlin_sets + fix_sets, plugin_cases + kotlin_cases + fix_cases))
    os.makedirs(os.path.join(out, FIXES_DIR), exist_ok=True)
    with open(os.path.join(out, FIXES_DIR, 'README.md'), 'w', encoding='utf-8', newline='\n') as f:
        f.write(fix_readme(fm, out))

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
    kt_exp = [os.path.join(out, t['oracle']) for t in km['tests']]
    cls_paths = [os.path.join(out, 'classes', f) for f in class_files]
    exp_paths = [os.path.join(out, t['oracle']) for t in included]
    cls_b, exp_b = tree_size(cls_paths), tree_size(exp_paths)
    man_b = os.path.getsize(os.path.join(out, 'cases.tsv'))
    readme = README.format(
        compilers=compilers, nmatch=len(passed & {t['id'] for t in included}),
        included=len(included), sets=', '.join(f'{n} {c}' for n, c in set_counts.items()),
        nclasses=len(class_files), cls_kb=cls_b // 1024, exp_kb=exp_b // 1024,
        total_kb=(cls_b + exp_b + man_b) // 1024, runnable=len(runnable), excluded=len(excluded),
        skipped=len(skipped), unsupported_note=unsupported_note, excluded_list=excluded_list,
        nplugin=len(pm['tests']),
        noracle=len(included) + len(pm['tests']) + len(km['tests']), nfix=len(fm['cases']),
        nfixruns=len(FIX_SETS) * len(fm['cases']),
        ncases=len(included) + len(pm['tests']) + len(km['tests']) + len(FIX_SETS) * len(fm['cases']),
        plugin_sets=', '.join(f'{n} {c}' for n, c in plugin_counts.items()),
        nkotlin=len(km['tests']), kotlin_sets=', '.join(f'{n} {c}' for n, c in kotlin_counts.items()),
        kt_nclasses=len(kt_class), kt_cls_kb=tree_size(kt_class) // 1024, kt_exp_kb=tree_size(kt_exp) // 1024,
        kt_excluded=', '.join(f'`{e["id"]}`' for e in km['excluded']))
    with open(os.path.join(out, 'README.md'), 'w', encoding='utf-8', newline='\n') as f:
        f.write(readme)

    digest = hashlib.sha256()
    for p in sorted(cls_paths + exp_paths + [os.path.join(out, 'cases.tsv')]):
        with open(p, 'rb') as f:
            digest.update(os.path.relpath(p, out).encode() + b'\0' + f.read())
    print(f'vendored {len(included)} cases ({len(class_files)} class files, {cls_b} B; expected {exp_b} B; '
          f'cases.tsv {man_b} B); excluded {len(excluded)}; skipped {len(skipped)}; sha256 {digest.hexdigest()}')


if __name__ == '__main__':
    main()
