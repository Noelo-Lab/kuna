#!/usr/bin/env python3
"""Recover every Kotlin test Vineflower 1.12.0 registers and what DecompilerTestFixture would feed it.

  KotlinTests        plugins/kotlin/src/test/java/org/vineflower/kotlin/KotlinTests.java (66 registrations;
                     getClassFile override: <name>.class if it exists, else <name>Kt.class; testData dir =
                     plugins/kotlin/testData because Gradle runs a subproject's tests in its project dir;
                     plugins: only the Kotlin plugin, found by ServicePluginSource)
  SingleClassesTest  test/org/jetbrains/java/decompiler/SingleClassesTest.java, only the KOTLIN/KOTLIN_OLD
                     registrations (7; base getClassFile; testData dir = testData; no plugins)

Replicates testFixtures/org/jetbrains/java/decompiler/{SingleClassesTestBase,DecompilerTestFixture}.java:
register/getFullClassName/collectClasses, TestSet.getTests (include-runtime injection: never applies, KOTLIN
has no runtime version), TestDefinition.run (decompiled file = <classfile name>.<version.extension>, else .java),
DecompilerTestFixture.setUp base options.

Usage: build_manifest.py --vf-src VF --classes CORPUS/classes --logs CORPUS/logs --jar VF_JAR --corpus CORPUS
                         -o manifest.json
"""
import argparse
import hashlib
import json
import os
import re
import sys

TOKEN_RE = re.compile(r'''
    (?P<ws>\s+)
  | (?P<lcomment>//[^\n]*)
  | (?P<bcomment>/\*.*?\*/)
  | (?P<string>"(?:[^"\\\n]|\\.)*")
  | (?P<char>'(?:[^'\\\n]|\\.)')
  | (?P<number>\d+)
  | (?P<ident>[A-Za-z_$][A-Za-z0-9_$]*)
  | (?P<op>::|->|[(){}\[\],;.=<>@!?:+\-*/&|^%~])
''', re.S | re.X)


def tokenize(text):
    toks, pos = [], 0
    while pos < len(text):
        m = TOKEN_RE.match(text, pos)
        if not m:
            raise SystemExit(f"tokenizer stuck at {text[pos:pos + 40]!r}")
        kind, pos = m.lastgroup, m.end()
        if kind in ('ws', 'lcomment', 'bcomment'):
            continue
        val = m.group()
        if kind == 'string':
            val = json.loads(val)
        toks.append((kind, val))
    return toks


def match_close(toks, i):
    pairs = {'(': ')', '{': '}', '[': ']'}
    opener = toks[i][1]
    closer, depth = pairs[opener], 0
    for j in range(i, len(toks)):
        if toks[j] == ('op', opener):
            depth += 1
        elif toks[j] == ('op', closer):
            depth -= 1
            if depth == 0:
                return j
    raise SystemExit("unbalanced brackets")


def split_args(toks, lo, hi):
    args, cur, depth = [], [], 0
    for t in toks[lo:hi]:
        if t[0] == 'op' and t[1] in '({[':
            depth += 1
        elif t[0] == 'op' and t[1] in ')}]':
            depth -= 1
        if depth == 0 and t == ('op', ','):
            args.append(cur)
            cur = []
        else:
            cur.append(t)
    if cur:
        args.append(cur)
    return args


def find_method_body(toks, name):
    for i in range(len(toks) - 4):
        if toks[i] == ('ident', 'void') and toks[i + 1] == ('ident', name) and toks[i + 2] == ('op', '('):
            j = match_close(toks, i + 2)
            assert toks[j + 1] == ('op', '{'), name
            return j + 2, match_close(toks, j + 1)
    raise SystemExit(f"method {name} not found")


def java_string_constants(path):
    with open(path, encoding='utf-8') as f:
        return {m.group(1): json.loads(m.group(2))
                for m in re.finditer(r'\bString\s+([A-Z_0-9]+)\s*=\s*("(?:[^"\\]|\\.)*")\s*;', f.read())}


def parse_versions(base_src):
    toks = tokenize(base_src)
    start = next(i + 3 for i in range(len(toks) - 2)
                 if toks[i] == ('ident', 'enum') and toks[i + 1] == ('ident', 'Version'))
    versions, i = {}, start
    while toks[i][0] == 'ident':
        name = toks[i][1]
        j = match_close(toks, i + 1)
        args = [a[0] for a in split_args(toks, i + 2, j)]
        kinds = [a[0] for a in args]
        vals = [int(a[1]) if a[0] == 'number' else a[1] for a in args]
        if kinds == ['number']:
            v = dict(runtime=vals[0], directory=f"java{vals[0]}", display=f"Java {vals[0]}", extension='java')
        elif kinds == ['number', 'string', 'string']:
            v = dict(runtime=vals[0], directory=f"java{vals[0]}{vals[1]}",
                     display=f"Java {vals[0]}" + (f" {vals[2]}" if vals[2] else ''), extension='java')
        elif kinds == ['string', 'string']:
            v = dict(runtime=-1, directory=vals[0], display=vals[1], extension='java')
        elif kinds == ['string', 'string', 'string']:
            v = dict(runtime=-1, directory=vals[0], display=vals[1], extension=vals[2])
        else:
            raise SystemExit(f"unexpected Version ctor {name}{vals}")
        versions[name] = v
        i = j + 1
        if toks[i] == ('op', ','):
            i += 1
        if toks[i] == ('op', ';'):
            break
    return versions


def parse_registrations(toks, lo, hi):
    regs, i = [], lo
    while i < hi:
        kind = toks[i]
        assert kind[0] == 'ident' and kind[1] in ('register', 'registerRaw', 'registerFailable'), toks[i:i + 6]
        j = match_close(toks, i + 1)
        args = split_args(toks, i + 2, j)
        ver = args[0]
        assert len(ver) == 1 and ver[0][0] == 'ident', ver
        names = []
        for a in args[1:]:
            assert len(a) == 1 and a[0][0] == 'string', a
            names.append(a[0][1])
        expected_name, k = None, j + 1
        if k < hi and toks[k] == ('op', '.'):
            assert toks[k + 1] == ('ident', 'setExpectedFileName')
            e = match_close(toks, k + 2)
            expected_name = toks[k + 3][1]
            k = e + 1
        if k < hi:
            assert toks[k] == ('op', ';'), toks[k:k + 4]
            k += 1
        regs.append(dict(kind=kind[1], version=ver[0][1], test_class_arg=names[0], others_arg=names[1:],
                         expected_file_name=expected_name))
        i = k
    return regs


def parse_register_all(src, consts):
    """registerAll(): registerSet(name, this::method | lambda, KEY, "value", ...)."""
    toks = tokenize(src)
    lo, hi = find_method_body(toks, 'registerAll')
    sets, i = [], lo
    while i < hi:
        assert toks[i] == ('ident', 'registerSet'), toks[i:i + 4]
        j = match_close(toks, i + 1)
        args = split_args(toks, i + 2, j)
        name, init = args[0], args[1]
        if len(init) == 3 and init[:2] == [('ident', 'this'), ('op', '::')]:
            blo, bhi = find_method_body(toks, init[2][1])
            init_desc, regs = f"this::{init[2][1]}", parse_registrations(toks, blo, bhi)
        else:
            body = init[3:]
            init_desc = 'lambda'
            if body[0] == ('op', '{'):
                body = body[1:-1]
            regs = parse_registrations(body, 0, len(body))
        opts, pairs = args[2:], []
        for k in range(0, len(opts), 2):
            key = opts[k]
            assert len(key) == 3 and key[1] == ('op', '.'), key
            cls, cname = key[0][1], key[2][1]
            val = opts[k + 1]
            if not (len(val) == 1 and val[0][0] == 'string'):
                pairs.append(dict(constant=f"{cls}.{cname}", key=consts[cls][cname], value={'$unsupported': True}))
                continue
            pairs.append(dict(constant=f"{cls}.{cname}", key=consts[cls][cname], value=val[0][1]))
        sets.append(dict(name=name[0][1], initializer=init_desc, options=pairs, registrations=regs))
        i = j + 2
    return sets


def full_class_name(name):  # SingleClassesTestBase.getFullClassName
    return name if '/' in name else 'pkg/' + name


def collect_classes(class_file):
    """SingleClassesTestBase.collectClasses; Files.list order replaced by C-locale name order."""
    parent = os.path.dirname(class_file)
    base = os.path.basename(class_file).replace('.class', '')
    pat = re.compile(base + r'\$.*\.class')
    sibs = sorted(n for n in os.listdir(parent) if pat.fullmatch(n))
    return [class_file] + [os.path.join(parent, n) for n in sibs]


def slug(name):
    return re.sub(r'[^a-z0-9]+', '-', name.lower()).strip('-')


FIXTURE_BASE = [("LOG_LEVEL", "warn"), ("DECOMPILE_GENERIC_SIGNATURES", "1"), ("REMOVE_SYNTHETIC", "1"),
                ("REMOVE_BRIDGE", "1"), ("LITERALS_AS_IS", "1"), ("UNIT_TEST_MODE", "1"),
                ("NEW_LINE_SEPARATOR", "1"), ("ERROR_MESSAGE", ""), ("INCLUDE_JAVA_RUNTIME", "")]

# What include-classpath=1 scans in Gradle's root-project test JVM that can matter for Kotlin output: the
# resolved testRuntimeClasspath jars kotlin-stdlib-jdk8/-jdk7/kotlin-stdlib 2.0.0 (testRuntimeOnly
# kotlin-stdlib-jdk8, version aligned by KGP 2.0.0) and annotations 24.0.0 (main implementation dep, wins over
# stdlib's 13.0). JUnit/hamcrest/opentest4j/test-fixtures jars are also there but no test class references them.
INCLUDE_CLASSPATH_JARS = ['tools/m2/kotlin-stdlib-jdk8-2.0.0.jar', 'tools/m2/kotlin-stdlib-2.0.0.jar',
                          'tools/m2/kotlin-stdlib-jdk7-2.0.0.jar', 'tools/m2/annotations-24.0.0.jar']

SUITES = [
    dict(suite='KotlinTests', test_file='plugins/kotlin/src/test/java/org/vineflower/kotlin/KotlinTests.java',
         testdata='plugins/kotlin/testData', slug_prefix='', plugins='kotlin-only',
         dirs={'kt': 'kt', 'ktold': 'ktold'}, kt_fallback=True, versions=None),
    dict(suite='SingleClassesTest', test_file='test/org/jetbrains/java/decompiler/SingleClassesTest.java',
         testdata='testData', slug_prefix='core-', plugins='none',
         dirs={'kt': 'core-kt', 'ktold': 'core-ktold'}, kt_fallback=False, versions={'KOTLIN', 'KOTLIN_OLD'}),
]


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('--vf-src', required=True)
    ap.add_argument('--classes', required=True)
    ap.add_argument('--logs', required=True, help='dir with compile-info-<set>.json')
    ap.add_argument('--jar', required=True)
    ap.add_argument('--corpus', required=True, help='the corpus directory (holds tools/plugin)')
    ap.add_argument('-o', '--out', required=True)
    a = ap.parse_args()
    vf = a.vf_src
    classes_root = os.path.abspath(a.classes)
    consts = dict(
        IFernflowerPreferences=java_string_constants(
            os.path.join(vf, 'src/org/jetbrains/java/decompiler/main/extern/IFernflowerPreferences.java')),
        KotlinOptions=java_string_constants(
            os.path.join(vf, 'plugins/kotlin/src/main/java/org/vineflower/kotlin/KotlinOptions.java')),
        IFabricJavadocProvider=java_string_constants(
            os.path.join(vf, 'src/net/fabricmc/fernflower/api/IFabricJavadocProvider.java')))
    versions = parse_versions(open(os.path.join(
        vf, 'testFixtures/org/jetbrains/java/decompiler/SingleClassesTestBase.java'), encoding='utf-8').read())
    base_opts = {consts['IFernflowerPreferences'][k]: v for k, v in FIXTURE_BASE}

    tests, set_rows = [], []
    for su in SUITES:
        sets = parse_register_all(open(os.path.join(vf, su['test_file']), encoding='utf-8').read(), consts)
        seen = set()  # SingleClassesTestBase.classNames (per test-class instance)
        for s in sets:
            regs = [r for r in s['registrations'] if su['versions'] is None or r['version'] in su['versions']]
            if not regs:
                continue
            sslug = su['slug_prefix'] + slug(s['name'])
            set_rows.append(dict(suite=su['suite'], name=s['name'], slug=sslug, initializer=s['initializer'],
                                 option_pairs=s['options'], registered=len(s['registrations']),
                                 kotlin_tests=len(regs)))
            for r in regs:
                ver = versions[r['version']]
                assert r['kind'] != 'registerRaw'
                if r['test_class_arg'] in seen:
                    raise SystemExit("Registered same class twice! " + r['test_class_arg'])
                seen.add(r['test_class_arg'])
                test_class = full_class_name(r['test_class_arg'])
                others = [full_class_name(o) for o in r['others_arg']]
                eff = dict(base_opts)
                for p in s['options']:
                    eff[p['key']] = p['value']
                assert ver['runtime'] == -1  # no include-runtime injection for Kotlin versions
                cdir = su['dirs'][ver['directory']]

                def class_file(name):
                    reg = os.path.join(classes_root, cdir, name + '.class')
                    kt = os.path.join(classes_root, cdir, name + 'Kt.class')
                    if su['kt_fallback']:  # KotlinTests.getClassFile
                        return reg if os.path.exists(reg) else kt
                    return reg

                primary = class_file(test_class)
                files, missing = [], []
                for f in [primary] + [class_file(o) for o in others]:
                    if not os.path.isfile(f):
                        missing.append(os.path.relpath(f, classes_root))
                        continue
                    files += [os.path.relpath(p, classes_root) for p in collect_classes(f)]
                test_name = os.path.basename(primary)[:-len('.class')]
                ext = 'kt' if su['plugins'] == 'kotlin-only' else 'java'
                ref = os.path.join(vf, su['testdata'], 'results', test_class + '.dec')
                alt = os.path.join(vf, su['testdata'], 'results', test_class + '_1.dec')
                incl_cp = eff.get(consts['IFernflowerPreferences']['INCLUDE_ENTIRE_CLASSPATH']) == '1'
                tests.append(dict(
                    id=f"{sslug}/{test_class}", suite=su['suite'],
                    junit_display_path=[su['suite'], s['name'], ver['display'], os.path.basename(test_class)],
                    set=s['name'], set_slug=sslug, set_initializer=s['initializer'],
                    set_option_pairs=[dict(p) for p in s['options']],
                    register_kind=r['kind'], failable=(r['kind'] == 'registerFailable'),
                    version=r['version'], source_set=cdir, upstream_classes_dir=f"{su['testdata']}/classes/{ver['directory']}",
                    test_class=test_class, others=others, expected_file_name=r['expected_file_name'],
                    class_file=os.path.relpath(primary, classes_root),
                    class_file_choice=('<name>Kt.class' if primary.endswith('Kt.class') and not test_class.endswith('Kt')
                                       else '<name>.class'),
                    options=eff, plugins=su['plugins'],
                    driver_classpath_extra=INCLUDE_CLASSPATH_JARS if incl_cp else [],
                    class_files=files,
                    decompiled_file=r['expected_file_name'] or f"{test_name}.{ver['extension']}",
                    decompiled_file_fallback=None if r['expected_file_name'] else f"{test_name}.java",
                    defaults_file=f"{test_name}.kt",
                    reference=os.path.relpath(ref, vf) if os.path.exists(ref) else None,
                    reference_alt=os.path.relpath(alt, vf) if os.path.exists(alt) else None,
                    oracle=f"expected/{sslug}/{test_class}.{ext}",
                    defaults_oracle=f"expected-defaults/{cdir}/{test_class}.kt",
                    defaults_stdlib_oracle=f"expected-defaults-stdlib/{cdir}/{test_class}.kt",
                    status='missing-class' if missing else 'ok',
                    skip_reason=('class file(s) not produced: ' + ', '.join(missing)) if missing else None))
    ids = [t['id'] for t in tests]
    assert len(ids) == len(set(ids)), "duplicate ids"
    source_sets = {}
    for n in sorted(os.listdir(a.logs)):
        if n.startswith('compile-info-') and n.endswith('.json'):
            info = json.load(open(os.path.join(a.logs, n)))
            info.pop('seconds', None)  # timing stays in logs/; keeps manifest.json reproducible
            source_sets[info['set']] = info
    manifest = dict(
        description="Vineflower 1.12.0 Kotlin test registrations (KotlinTests + SingleClassesTest KOTLIN) "
                    "replayed against corpus-kt/classes",
        vineflower_src=os.path.realpath(vf),
        vineflower_jar=dict(path=os.path.realpath(a.jar), sha256=hashlib.sha256(open(a.jar, 'rb').read()).hexdigest()),
        kotlin_plugin_jar=dict(path='tools/plugin/vineflower-kotlin-0.1.0.jar', origin='META-INF/plugins/ of the release jar',
                               sha256=hashlib.sha256(open(os.path.join(a.corpus,
                                    'tools/plugin/vineflower-kotlin-0.1.0.jar'), 'rb').read()).hexdigest()),
        source_sets=source_sets,
        notes=[
            "class_files is the exact addSource() order (paths relative to classes/): the primary class file (KotlinTests:"
            " <testClass>.class if it exists else <testClass>Kt.class; SingleClassesTest: <testClass>.class), its"
            " Name$*.class siblings (Files.list order replaced by C-locale name order), then each companion the same way.",
            "options is the effective map handed to ConsoleDecompiler: fixture_base_options overlaid with the set's pairs.",
            "plugins: 'kotlin-only' = only org.vineflower.kotlin.KotlinPlugin (bin/vf-decompile-kt --kotlin-only), as in the"
            " plugin's Gradle test run; 'none' = no plugin at all, as in the root project's SingleClassesTest run.",
            "driver_classpath_extra: jars appended to the driver JVM's java.class.path (env VF_CP_EXTRA), which"
            " include-classpath=1 scans (plus the running JVM's system modules).",
            "decompiled_file is the file the fixture reads (<classfile name>.<version.extension>, falling back to .java);"
            " oracle/defaults_oracle store it under the registered test class name, mirroring results/<testClass>.dec.",
            "defaults_oracle: java -jar vineflower-1.12.0.jar <class_files> <dir> (no options: all 3 bundled plugins,"
            " include-runtime=current); defaults_stdlib_oracle: the same plus -e=tools/m2/kotlin-stdlib-2.0.0.jar.",
        ],
        fixture_base_options=base_opts,
        versions={k: v for k, v in versions.items() if k in ('KOTLIN', 'KOTLIN_OLD')},
        sets=set_rows,
        tests=tests,
    )
    with open(a.out + '.tmp', 'w', encoding='utf-8') as f:
        json.dump(manifest, f, indent=1)
        f.write('\n')
    os.replace(a.out + '.tmp', a.out)
    by = {}
    for t in tests:
        by[t['status']] = by.get(t['status'], 0) + 1
    print(f"{len(set_rows)} sets, {len(tests)} tests: {by}", file=sys.stderr)
    for t in tests:
        if t['status'] != 'ok':
            print("  ", t['id'], t['skip_reason'], file=sys.stderr)


if __name__ == '__main__':
    main()
