#!/usr/bin/env python3
"""Recover every test SingleClassesTest registers and what DecompilerTestFixture would feed Vineflower.

Usage: build_manifest.py --vf-src VINEFLOWER_CHECKOUT --classes CORPUS_CLASSES_DIR --jdks JDKS_DIR -o manifest.json

Replicates (testFixtures/org/jetbrains/java/decompiler/):
  SingleClassesTestBase.register/registerRaw/registerFailable/getFullClassName/collectClasses,
  TestSet.getTests (include-runtime "1"/"current" -> java.<runtimeVersion>.home), TestDefinition.run,
  DecompilerTestFixture.setUp base options.
"""
import argparse
import json
import os
import re
import sys

# ---------------------------------------------------------------- tokenizer

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
    toks = []
    pos = 0
    while pos < len(text):
        m = TOKEN_RE.match(text, pos)
        if not m:
            raise SystemExit(f"tokenizer stuck at {text[pos:pos + 40]!r}")
        kind = m.lastgroup
        pos = m.end()
        if kind in ('ws', 'lcomment', 'bcomment'):
            continue
        val = m.group()
        if kind == 'string':
            val = json.loads(val)  # Java escapes used in the test file are JSON-compatible
        toks.append((kind, val))
    return toks


def match_close(toks, i):
    """toks[i] is an opener; return index of its matching closer."""
    pairs = {'(': ')', '{': '}', '[': ']'}
    opener = toks[i][1]
    closer = pairs[opener]
    depth = 0
    for j in range(i, len(toks)):
        if toks[j] == ('op', opener):
            depth += 1
        elif toks[j] == ('op', closer):
            depth -= 1
            if depth == 0:
                return j
    raise SystemExit("unbalanced brackets")


def split_args(toks, lo, hi):
    """Split toks[lo:hi] at top-level commas."""
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
            k = match_close(toks, j + 1)
            return j + 2, k
    raise SystemExit(f"method {name} not found")

# ---------------------------------------------------------------- constants


def java_string_constants(path):
    consts = {}
    with open(path, encoding='utf-8') as f:
        for m in re.finditer(r'\bString\s+([A-Z_0-9]+)\s*=\s*("(?:[^"\\]|\\.)*")\s*;', f.read()):
            consts[m.group(1)] = json.loads(m.group(2))
    return consts


def parse_versions(base_src):
    """Evaluate SingleClassesTestBase.TestDefinition.Version's constructors."""
    toks = tokenize(base_src)
    start = None
    for i in range(len(toks) - 2):
        if toks[i] == ('ident', 'enum') and toks[i + 1] == ('ident', 'Version'):
            start = i + 3
            break
    assert start is not None
    versions = {}
    i = start
    while toks[i][0] == 'ident':
        name = toks[i][1]
        assert toks[i + 1] == ('op', '('), name
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

# ---------------------------------------------------------------- SingleClassesTest

JAVADOC_EXPECT = ["Class javadoc for '", "'", "Field javadoc for '", "'", "Method javadoc for '", "'"]


def parse_option_value(arg, consts_prefs, consts_javadoc):
    if len(arg) == 1 and arg[0][0] == 'string':
        return arg[0][1]
    if arg[:3] == [('ident', 'new'), ('ident', 'IFabricJavadocProvider'), ('op', '(')]:
        strings = [t[1] for t in arg if t[0] == 'string']
        assert strings == JAVADOC_EXPECT, strings
        markdown = ('ident', 'MARKDOWN') in arg
        idents = {t[1] for t in arg if t[0] == 'ident'}
        assert {'getClassDoc', 'getFieldDoc', 'getMethodDoc'} <= idents
        if markdown:
            assert 'getClassJavadocStyle' in idents
        return {"$javadoc_provider": "markdown" if markdown else "plain"}
    raise SystemExit(f"unsupported option value {arg}")


def parse_option_key(arg, consts_prefs, consts_javadoc):
    assert len(arg) == 3 and arg[1] == ('op', '.'), arg
    cls, name = arg[0][1], arg[2][1]
    if cls == 'IFernflowerPreferences':
        return name, consts_prefs[name]
    if cls == 'IFabricJavadocProvider':
        return f"{cls}.{name}", consts_javadoc[name]
    raise SystemExit(f"unknown option class {cls}")


def parse_registrations(toks, lo, hi):
    """Parse the statements of a register* body (toks[lo:hi])."""
    regs = []
    i = lo
    while i < hi:
        kind = toks[i]
        assert kind[0] == 'ident' and kind[1] in ('register', 'registerRaw', 'registerFailable'), toks[i:i + 6]
        assert toks[i + 1] == ('op', '(')
        j = match_close(toks, i + 1)
        args = split_args(toks, i + 2, j)
        ver = args[0]
        assert len(ver) == 1 and ver[0][0] == 'ident', ver
        names = []
        for a in args[1:]:
            assert len(a) == 1 and a[0][0] == 'string', a
            names.append(a[0][1])
        expected_name = None
        k = j + 1
        if k < hi and toks[k] == ('op', '.'):
            assert toks[k + 1] == ('ident', 'setExpectedFileName') and toks[k + 2] == ('op', '(')
            e = match_close(toks, k + 2)
            assert e == k + 4 and toks[k + 3][0] == 'string'
            expected_name = toks[k + 3][1]
            k = e + 1
        if k < hi:
            assert toks[k] == ('op', ';'), toks[k:k + 4]
            k += 1
        regs.append(dict(kind=kind[1], version=ver[0][1], test_class_arg=names[0], others_arg=names[1:],
                         expected_file_name=expected_name))
        i = k
    return regs


def parse_single_classes_test(src, consts_prefs, consts_javadoc):
    toks = tokenize(src)
    lo, hi = find_method_body(toks, 'registerAll')
    sets = []
    i = lo
    while i < hi:
        assert toks[i] == ('ident', 'registerSet') and toks[i + 1] == ('op', '('), toks[i:i + 4]
        j = match_close(toks, i + 1)
        args = split_args(toks, i + 2, j)
        assert toks[j + 1] == ('op', ';')
        name = args[0]
        assert len(name) == 1 and name[0][0] == 'string'
        init = args[1]
        if init[:3] == [('ident', 'this'), ('op', '::'), init[2]] and len(init) == 3:
            blo, bhi = find_method_body(toks, init[2][1])
            init_desc = f"this::{init[2][1]}"
            regs = parse_registrations(toks, blo, bhi)
        else:
            assert init[:4] == [('op', '('), ('op', ')'), ('op', '->')] + init[3:4], init[:5]
            body = init[3:]
            init_desc = 'lambda'
            if body[0] == ('op', '{'):
                assert body[-1] == ('op', '}')
                body = body[1:-1]
            regs = parse_registrations(body, 0, len(body))
        opts = args[2:]
        assert len(opts) % 2 == 0
        pairs = []
        for k in range(0, len(opts), 2):
            const_name, key = parse_option_key(opts[k], consts_prefs, consts_javadoc)
            pairs.append(dict(constant=const_name, key=key,
                              value=parse_option_value(opts[k + 1], consts_prefs, consts_javadoc)))
        sets.append(dict(name=name[0][1], initializer=init_desc, options=pairs, registrations=regs))
        i = j + 2
    return sets

# ---------------------------------------------------------------- fixture replication


def full_class_name(name):  # SingleClassesTestBase.getFullClassName
    return name if '/' in name else 'pkg/' + name


def collect_classes(class_file):
    """SingleClassesTestBase.collectClasses: the file, then Files.list(parent) entries matching
    <name-without-.class>\\$.*\\.class. Files.list order is the filesystem's directory order; we emit
    those siblings sorted by name (C locale) so the manifest is reproducible -- order-insensitivity of
    the output is checked separately (see README)."""
    files = [class_file]
    parent = os.path.dirname(class_file)
    base = os.path.basename(class_file).replace('.class', '')
    pat = re.compile(base + r'\$.*\.class')  # same regex text as the fixture (Java String.matches semantics)
    if os.path.isdir(parent):
        sibs = [n for n in os.listdir(parent) if pat.fullmatch(n)]
        files += [os.path.join(parent, n) for n in sorted(sibs)]
    return files


def slug(name):
    return re.sub(r'[^a-z0-9]+', '-', name.lower()).strip('-')


FIXTURE_BASE = [  # DecompilerTestFixture.setUp, in insertion order
    ("LOG_LEVEL", "warn"), ("DECOMPILE_GENERIC_SIGNATURES", "1"), ("REMOVE_SYNTHETIC", "1"),
    ("REMOVE_BRIDGE", "1"), ("LITERALS_AS_IS", "1"), ("UNIT_TEST_MODE", "1"), ("NEW_LINE_SEPARATOR", "1"),
    ("ERROR_MESSAGE", ""), ("INCLUDE_JAVA_RUNTIME", ""),
]

SKIP_DIRS = {'kt': 'Kotlin source set (not compiled; Kotlin comes later)',
             'ktold': 'Kotlin source set (not compiled; Kotlin comes later)',
             'groovy': 'Groovy source set (not compiled)',
             'scala': 'Scala source set (not compiled)'}


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('--vf-src', required=True)
    ap.add_argument('--classes', required=True, help='corpus classes dir (contains java8/, jasm/, custom/ ...)')
    ap.add_argument('--jdks', required=True)
    ap.add_argument('--compile-info-dir', help='dir with compile-info-<set>.json from compile_sets.sh')
    ap.add_argument('--jar', help='the Vineflower jar the oracle runs (recorded with its sha256)')
    ap.add_argument('-o', '--out', required=True)
    a = ap.parse_args()

    vf = a.vf_src
    test_src = open(os.path.join(vf, 'test/org/jetbrains/java/decompiler/SingleClassesTest.java'), encoding='utf-8').read()
    base_src = open(os.path.join(vf, 'testFixtures/org/jetbrains/java/decompiler/SingleClassesTestBase.java'), encoding='utf-8').read()
    prefs = java_string_constants(os.path.join(vf, 'src/org/jetbrains/java/decompiler/main/extern/IFernflowerPreferences.java'))
    jdoc = java_string_constants(os.path.join(vf, 'src/net/fabricmc/fernflower/api/IFabricJavadocProvider.java'))
    versions = parse_versions(base_src)
    sets = parse_single_classes_test(test_src, prefs, jdoc)

    classes_root = os.path.abspath(a.classes)
    results_root = os.path.join(vf, 'testData', 'results')
    base_opts = {prefs[k]: v for k, v in FIXTURE_BASE}

    java_homes = {}
    for v in sorted({vv['runtime'] for vv in versions.values() if vv['runtime'] > 0}):
        p = os.path.join(a.jdks, f'jdk{v}')
        java_homes[str(v)] = os.path.realpath(p) if os.path.isdir(p) else None

    tests = []
    seen_classnames = set()
    for s in sets:
        sslug = slug(s['name'])
        for r in s['registrations']:
            ver = versions[r['version']]
            if r['kind'] == 'registerRaw':
                test_class = r['test_class_arg']
                others = list(r['others_arg'])
            else:
                if r['test_class_arg'] in seen_classnames:
                    raise SystemExit("Registered same class twice! " + r['test_class_arg'])
                seen_classnames.add(r['test_class_arg'])
                test_class = full_class_name(r['test_class_arg'])
                others = [full_class_name(o) for o in r['others_arg']]

            # TestSet.getTests runtime injection
            set_opts = [(p['key'], p['value']) for p in s['options']]
            runtime_injected = None
            if ver['runtime'] != -1:
                for idx, (k, v) in enumerate(set_opts):
                    if k == prefs['INCLUDE_JAVA_RUNTIME'] and v in ('1', 'current'):
                        runtime_injected = ver['runtime']
                        set_opts[idx] = (k, '${java.%d.home}' % ver['runtime'])
            eff = dict(base_opts)
            for k, v in set_opts:
                eff[k] = v

            vdir = ver['directory']
            rel_primary = f"{vdir}/{test_class}.class"
            entry = dict(
                id=f"{sslug}/{test_class}",
                junit_display_path=[s['name'], ver['display'], os.path.basename(test_class)],
                set=s['name'], set_slug=sslug, set_initializer=s['initializer'],
                set_option_pairs=[dict(p) for p in s['options']],
                register_kind=r['kind'], failable=(r['kind'] == 'registerFailable'),
                version=r['version'], source_set=vdir, runtime_version=ver['runtime'],
                test_class=test_class, others=others,
                expected_file_name=r['expected_file_name'],
                options=eff, runtime_injected=runtime_injected,
            )
            test_name = os.path.basename(test_class)  # classFile name minus ".class"
            entry['decompiled_file'] = r['expected_file_name'] or f"{test_name}.{ver['extension']}"
            entry['decompiled_file_fallback'] = None if r['expected_file_name'] or ver['extension'] == 'java' else f"{test_name}.java"
            ref = os.path.join(results_root, test_class + '.dec')
            alt = os.path.join(results_root, test_class + '_1.dec')
            entry['reference'] = os.path.relpath(ref, vf) if os.path.exists(ref) else None
            entry['reference_alt'] = os.path.relpath(alt, vf) if os.path.exists(alt) else None
            entry['oracle'] = f"expected/{sslug}/{test_class}.java"
            entry['defaults_oracle'] = f"expected-defaults/{vdir}/{test_class}.java"

            if vdir in SKIP_DIRS:
                entry.update(status='skipped', skip_reason=SKIP_DIRS[vdir], class_files=None)
            else:
                files = []
                missing = []
                for cls in [test_class] + others:
                    cf = os.path.join(classes_root, vdir, cls + '.class')
                    if not os.path.isfile(cf):
                        missing.append(os.path.relpath(cf, classes_root))
                        continue
                    files += [os.path.relpath(p, classes_root) for p in collect_classes(cf)]
                entry['class_files'] = files
                if missing:
                    entry.update(status='missing-class', skip_reason='class file(s) not produced: ' + ', '.join(missing))
                else:
                    entry['status'] = 'ok'
            tests.append(entry)

    ids = [t['id'] for t in tests]
    assert len(ids) == len(set(ids)), "duplicate ids"
    source_sets = {}
    if a.compile_info_dir:
        for n in sorted(os.listdir(a.compile_info_dir)):
            if n.startswith('compile-info-') and n.endswith('.json'):
                info = json.load(open(os.path.join(a.compile_info_dir, n)))
                info['args'] = [x.replace(classes_root, '${classes}').replace(os.path.realpath(vf), '${vineflower}')
                                for x in info['args']]
                source_sets[info['set']] = info
        source_sets['custom'] = dict(set='custom', compiler='checked-in class files copied from testData/classes/custom')
        for d, why in SKIP_DIRS.items():
            if any(t['source_set'] == d for t in tests):
                source_sets[d] = dict(set=d, skipped=why)
    jar = None
    if a.jar:
        import hashlib
        jar = dict(path=os.path.realpath(a.jar), sha256=hashlib.sha256(open(a.jar, 'rb').read()).hexdigest())
    manifest = dict(
        description="Vineflower 1.12.0 SingleClassesTest registrations replayed against corpus/classes",
        vineflower_src=os.path.realpath(vf),
        vineflower_jar=jar,
        source_sets=source_sets,
        notes=[
            "class_files is the exact addSource() order: primary class + its Name$*.class siblings, then each "
            "companion the same way (duplicates kept). The fixture lists siblings with Files.list (filesystem "
            "directory order, i.e. unspecified); here siblings are sorted by name. Only one test is sensitive to "
            "that order: renaming/pkg/TestRenameEntities (rename-members numbering follows input order; upstream "
            "marks it registerFailable for this reason). The sorted order reproduces upstream's .dec.",
            "options is the effective map handed to ConsoleDecompiler: fixture_base_options overlaid with the set's "
            "pairs; '${java.N.home}' marks the fixture's include-runtime substitution (resolve via java_homes); "
            "{'$javadoc_provider': ...} is the anonymous IFabricJavadocProvider of the Javadoc sets.",
        ],
        fixture_base_options=base_opts,
        java_homes=java_homes,
        versions=versions,
        sets=[dict(name=s['name'], slug=slug(s['name']), initializer=s['initializer'], option_pairs=s['options'],
                   test_count=len(s['registrations'])) for s in sets],
        tests=tests,
    )
    with open(a.out + '.tmp', 'w', encoding='utf-8') as f:
        json.dump(manifest, f, indent=1, sort_keys=False)
        f.write('\n')
    os.replace(a.out + '.tmp', a.out)
    by = {}
    for t in tests:
        by[t['status']] = by.get(t['status'], 0) + 1
    print(f"{len(sets)} sets, {len(tests)} tests: {by}", file=sys.stderr)
    for t in tests:
        if t['status'] == 'missing-class':
            print("  missing:", t['id'], t['skip_reason'], file=sys.stderr)


if __name__ == '__main__':
    main()
