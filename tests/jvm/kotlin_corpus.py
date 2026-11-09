#!/usr/bin/env python3
"""Vendor the Kotlin cases of tests/jvm from the Kotlin oracle corpus (~/kwt/jvm/corpus-kt).

  python3 -I tests/jvm/kotlin_corpus.py --corpus-kt ~/kwt/jvm/corpus-kt [--out tests/jvm]
  python3 -I tests/jvm/kotlin_corpus.py --corpus-kt ~/kwt/jvm/corpus-kt --skip-vendor --full-manifest FILE

The oracle corpus holds 73 cases (the 66 tests of Vineflower's Kotlin plugin, KotlinTests, plus the 7
KOTLIN tests of core SingleClassesTest), class files compiled by kotlinc 2.0.0, and what the release
vineflower-1.12.0.jar prints for them in three forms; its README.md documents how all of it was built
and validated (73/73 fixture outputs pass the fixture comparison with upstream's .dec files).

Vendoring copies the fixture form of every case that needs no library: the 66 KotlinTests (only the
Kotlin plugin loaded, kuna's jvm_plugins on) and the 5 core Default tests (no plugin, Java output).
It writes, under --out:
  kotlin.json                     the vendored sets and cases; regen.py appends them to cases.tsv
  classes/kotlin/<source set>/... their class files (<corpus-kt>/classes)
  expected/kotlin-<slug>/...      their expected text (<corpus-kt>/expected), checked against SHA256SUMS

--full-manifest writes a corpus manifest rooted at <corpus-kt> with all 73 cases in each of the three
forms (sets kotlin-*, kotlin-defaults, kotlin-defaults-stdlib); the 2 core Entire Classpath cases and
both plain-CLI forms need libraries or a JDK (kotlin-stdlib on the class path, include-classpath=1 or
include-runtime), so they are run from there, not vendored. Existing files are overwritten, nothing is
deleted.
"""
import argparse
import hashlib
import json
import os
import shutil
import sys

FORMAT = 'kuna-jvm-corpus\t1'
SOURCE = {
    'KotlinTests': 'plugins/kotlin/src/test/java/org/vineflower/kotlin/KotlinTests.java',
    'SingleClassesTest': 'test/org/jetbrains/java/decompiler/SingleClassesTest.java',
}
PLUGINS = {'kotlin-only': '1', 'none': '0'}
STDLIB = 'tools/m2/kotlin-stdlib-2.0.0.jar'


def esc(s):
    return s.replace('\\', '\\\\').replace('\t', '\\t').replace('\n', '\\n').replace('\r', '\\r')


def line(*fields):
    return '\t'.join(esc(str(f)) for f in fields) + '\n'


def kotlin_slug(slug):
    return 'kotlin-' + slug


def set_name(s):
    prefix = 'Kotlin' if s['suite'] == 'KotlinTests' else 'Kotlin Core'
    return f'{prefix} {s["name"]}'


def sha256sums(corpus):
    sums = {}
    with open(os.path.join(corpus, 'SHA256SUMS'), encoding='utf-8') as f:
        for raw in f:
            digest, path = raw.rstrip('\n').split(None, 1)
            sums[path.lstrip('*').removeprefix('./')] = digest
    return sums


def sha256(path):
    with open(path, 'rb') as f:
        return hashlib.sha256(f.read()).hexdigest()


def case_pairs(t, sets):
    """jvm_plugins, then the effective option map; checked against the set's pairs."""
    s = sets[t['set_slug']]
    pairs = [['jvm_plugins', PLUGINS[t['plugins']]]] + [[k, v] for k, v in t['options'].items()]
    for p in s['option_pairs']:
        if t['options'].get(p['key']) != p['value']:
            sys.exit(f'{t["id"]}: option {p["key"]} is not the set\'s {p["value"]!r}')
    return pairs


def vendorable(t):
    return not t['driver_classpath_extra'] and t['options'].get('include-classpath') != '1'


def main():
    here = os.path.dirname(os.path.abspath(__file__))
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument('--corpus-kt', required=True, help='the Kotlin oracle corpus (holds manifest.json)')
    ap.add_argument('--out', default=here, help='the tests/jvm directory to write (default: %(default)s)')
    ap.add_argument('--full-manifest', help='also write a manifest of all 73 cases x 3 forms rooted at --corpus-kt')
    ap.add_argument('--skip-vendor', action='store_true', help='only write --full-manifest')
    a = ap.parse_args()
    corpus = os.path.abspath(os.path.expanduser(a.corpus_kt))
    out = os.path.abspath(a.out)

    with open(os.path.join(corpus, 'manifest.json'), encoding='utf-8') as f:
        m = json.load(f)
    sums = sha256sums(corpus)
    sets = {s['slug']: s for s in m['sets']}
    tests = sorted((t for t in m['tests'] if t['status'] == 'ok'), key=lambda t: t['id'])
    if len(tests) != len(m['tests']):
        sys.exit('the corpus has cases that are not ok')

    if a.full_manifest:
        write_full_manifest(os.path.abspath(a.full_manifest), corpus, tests, sets)
    if a.skip_vendor:
        return

    vendored = [t for t in tests if vendorable(t)]
    kept_sets, records = [], []
    for t in vendored:
        s = sets[t['set_slug']]
        pairs = case_pairs(t, sets)
        entry = next((k for k in kept_sets if k['slug'] == kotlin_slug(s['slug'])), None)
        if entry is None:
            entry = {'name': set_name(s), 'slug': kotlin_slug(s['slug']), 'fixture': False,
                     'source': SOURCE[s['suite']] + ' (' + s['initializer'] + ')', 'option_pairs': pairs}
            kept_sets.append(entry)
        elif entry['option_pairs'] != pairs:
            sys.exit(f'{t["id"]}: options differ from the other cases of its set')
        oracle = 'expected/' + kotlin_slug(t['oracle'].split('/', 1)[1])
        src = os.path.join(corpus, t['oracle'])
        if sums.get(t['oracle']) != sha256(src):
            sys.exit(f'{t["oracle"]}: does not match {corpus}/SHA256SUMS')
        copy(src, os.path.join(out, oracle))
        classes = []
        for c in t['class_files']:
            if sums.get('classes/' + c) != sha256(os.path.join(corpus, 'classes', c)):
                sys.exit(f'classes/{c}: does not match {corpus}/SHA256SUMS')
            copy(os.path.join(corpus, 'classes', c), os.path.join(out, 'classes', 'kotlin', c))
            classes.append('kotlin/' + c)
        records.append({'id': kotlin_slug(t['id']), 'set': entry['name'], 'class_files': classes,
                        'oracle': oracle, 'decompiled_file': t['decompiled_file'],
                        'decompiled_file_fallback': t['decompiled_file_fallback'],
                        'reference': t['reference']})
    doc = {
        'notes': ('Kotlin cases vendored by kotlin_corpus.py from the Kotlin oracle corpus (~/kwt/jvm/corpus-kt): '
                  'the fixture form of every case that needs no library. KotlinTests sets load only the Kotlin '
                  'plugin in Vineflower (jvm_plugins on here: the other two bundled plugins do not change these '
                  'outputs, per the corpus crosscheck); core SingleClassesTest sets load none and print Java. '
                  'option_pairs are the complete effective option map. regen.py adds these cases to cases.tsv.'),
        'corpus_manifest_sha256': sha256(os.path.join(corpus, 'manifest.json')),
        'excluded': [{'id': kotlin_slug(t['id']), 'reason': 'include-classpath=1 with kotlin-stdlib on the oracle '
                      'JVM class path (' + ', '.join(t['driver_classpath_extra']) + ')'}
                     for t in tests if not vendorable(t)],
        'sets': kept_sets,
        'tests': records,
    }
    with open(os.path.join(out, 'kotlin.json'), 'w', encoding='utf-8', newline='\n') as f:
        json.dump(doc, f, indent=1)
        f.write('\n')
    print(f'vendored {len(records)} Kotlin cases in {len(kept_sets)} sets; '
          f'{len(doc["excluded"])} left in {corpus}')


def write_full_manifest(path, corpus, tests, sets):
    out = [line('#', 'kuna-jvm Kotlin corpus, all forms; generated by tests/jvm/kotlin_corpus.py'), FORMAT + '\n',
           line('root', corpus), line('java_home', '17', os.path.expanduser('~/kwt/jvm/jdks/jdk17'))]
    seen = []
    for t in tests:
        s = sets[t['set_slug']]
        if s['slug'] in seen:
            continue
        seen.append(s['slug'])
        out.append(line('set', set_name(s), kotlin_slug(s['slug'])))
        for k, v in case_pairs(t, sets):
            out.append(line('opt', k, v))
        if t['options'].get('include-classpath') == '1':
            out.append(line('current_jdk', '17'))
    out.append(line('set', 'Kotlin CLI Defaults', 'kotlin-defaults'))
    out.append(line('opt', 'jvm_plugins', '1'))
    out.append(line('set', 'Kotlin CLI Defaults Stdlib', 'kotlin-defaults-stdlib'))
    out.append(line('opt', 'jvm_plugins', '1'))
    for form in ('fixture', 'defaults', 'defaults-stdlib'):
        for t in tests:
            s = sets[t['set_slug']]
            if form == 'fixture':
                cid, sname, expected = kotlin_slug(t['id']), set_name(s), t['oracle']
                compare = [t['decompiled_file'], t['decompiled_file_fallback']]
                libs = t['driver_classpath_extra']
            else:
                key = 'defaults_oracle' if form == 'defaults' else 'defaults_stdlib_oracle'
                cid = f'kotlin-{form}/{t["source_set"]}/{t["test_class"]}'
                sname = 'Kotlin CLI Defaults' + (' Stdlib' if form == 'defaults-stdlib' else '')
                expected, compare = t[key], [t['defaults_file']]
                libs = [STDLIB] if form == 'defaults-stdlib' else []
            out.append(line('case', cid, sname))
            for lib in libs:
                out.append(line('lib', lib))
            for c in t['class_files']:
                out.append(line('class', 'classes/' + c))
            out.append(line('expected', expected))
            out.append(line('compare', *compare))
    with open(path, 'w', encoding='utf-8', newline='\n') as f:
        f.writelines(out)
    print(f'wrote {path}: {3 * len(tests)} cases')


def copy(src, dst):
    os.makedirs(os.path.dirname(dst), exist_ok=True)
    shutil.copyfile(src, dst)


if __name__ == '__main__':
    main()
