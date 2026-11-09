#!/usr/bin/env python3
"""Compile the three Kotlin test-data source sets the way Gradle (Kotlin Gradle plugin 2.0.0) does.

  kt       plugins/kotlin/testData/src/kt     -> <classes>/kt       (plugin  :kotlin  sourceSet testDataKotlin)
  ktold    plugins/kotlin/testData/src/ktold  -> <classes>/ktold    (plugin  :kotlin  sourceSet testDataOldKotlin)
  core-kt  testData/src/kt                    -> <classes>/core-kt  (root    project  sourceSet testDataKotlin)

Compiler: standalone kotlinc 2.0.0 (kotlin-compiler-2.0.0.zip) run on JDK 17 (the java.toolchain), with the
arguments KGP 2.0.0 derives for a JVM compilation: -no-stdlib -no-reflect (KGP never lets kotlinc add its own),
-Xallow-no-source-files, -jvm-target 17 (from the toolchain), -jdk-home <toolchain JDK>, -module-name
<archivesName>_<sourceSetName>, -classpath = resolved compileClasspath (kotlin-bom/kotlin-stdlib-jdk8 2.0.0 ->
kotlin-stdlib-jdk8, kotlin-stdlib, kotlin-stdlib-jdk7, annotations 13.0), -d <dir>, set freeCompilerArgs, then
all sources (sorted). No source set contains Java files, so there is no javac half of a joint compilation.

Each set is compiled into a fresh directory under --work and then copied over <classes>/<set> (never deletes).
Writes <logs>/compile-<set>.log, <logs>/sources-<set>.txt, <logs>/compile-info-<set>.json.
"""
import argparse
import json
import os
import shutil
import subprocess
import sys
import time

SETS = [
    dict(set='kt', src='plugins/kotlin/testData/src/kt', module='vineflower-kotlin_testDataKotlin',
         gradle='project :kotlin (base.archivesName = vineflower-kotlin), sourceSet testDataKotlin, '
                'task compileTestDataKotlinKotlin', extra=[]),
    dict(set='ktold', src='plugins/kotlin/testData/src/ktold', module='vineflower-kotlin_testDataOldKotlin',
         gradle='project :kotlin (base.archivesName = vineflower-kotlin), sourceSet testDataOldKotlin, '
                'task compileTestDataOldKotlinKotlin (freeCompilerArgs from plugins/kotlin/build.gradle:45-52)',
         extra=['-Xlambdas=class', '-Xsam-conversions=class', '-Xstring-concat=inline']),
    dict(set='core-kt', src='testData/src/kt', module='vineflower_testDataKotlin',
         gradle='root project (base.archivesName = vineflower), sourceSet testDataKotlin, '
                'task compileTestDataKotlinKotlin', extra=[]),
]
CLASSPATH = ['kotlin-stdlib-jdk8-2.0.0.jar', 'kotlin-stdlib-2.0.0.jar', 'kotlin-stdlib-jdk7-2.0.0.jar',
             'annotations-13.0.jar']


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('--vf-src', required=True)
    ap.add_argument('--kotlinc-home', required=True, help='extracted kotlin-compiler-2.0.0.zip/kotlinc')
    ap.add_argument('--jdk', required=True, help='JDK 17 home (runs kotlinc, -jdk-home)')
    ap.add_argument('--m2', required=True, help='dir with the Maven Central class path jars')
    ap.add_argument('--classes', required=True)
    ap.add_argument('--logs', required=True)
    ap.add_argument('--work', required=True)
    ap.add_argument('--only', action='append')
    a = ap.parse_args()
    os.makedirs(a.logs, exist_ok=True)
    kotlinc = os.path.join(a.kotlinc_home, 'bin', 'kotlinc')
    cp = [os.path.join(a.m2, j) for j in CLASSPATH]
    for p in cp + [kotlinc]:
        if not os.path.exists(p):
            sys.exit(f'missing {p}')
    env = dict(os.environ, JAVA_HOME=a.jdk, JAVACMD=os.path.join(a.jdk, 'bin', 'java'),
               JAVA_OPTS='-Xmx3g -Xms256m', LANG='C.UTF-8', LC_ALL='C.UTF-8')
    ver = subprocess.run([kotlinc, '-version'], env=env, capture_output=True, text=True)
    version = (ver.stdout + ver.stderr).strip().splitlines()[-1]
    jver = subprocess.run([os.path.join(a.jdk, 'bin', 'java'), '-version'], capture_output=True, text=True).stderr
    build = open(os.path.join(a.kotlinc_home, 'build.txt')).read().strip()
    for s in SETS:
        if a.only and s['set'] not in a.only:
            continue
        src_root = os.path.join(a.vf_src, s['src'])
        sources = sorted(os.path.join(dp, f) for dp, _, fs in os.walk(src_root) for f in fs
                         if f.endswith(('.kt', '.java')))
        if any(f.endswith('.java') for f in sources):
            sys.exit(f"{s['set']}: Java sources present; joint compilation not implemented")
        out = os.path.join(a.work, 'classes', s['set'])
        os.makedirs(out, exist_ok=False)
        args = ['-no-stdlib', '-no-reflect', '-Xallow-no-source-files', '-jvm-target', '17', '-jdk-home', a.jdk,
                '-module-name', s['module'], '-classpath', ':'.join(cp), '-d', out] + s['extra'] + sources
        t0 = time.time()
        with open(os.path.join(a.logs, f"compile-{s['set']}.log.tmp"), 'w') as log:
            rc = subprocess.run([kotlinc] + args, env=env, stdout=log, stderr=subprocess.STDOUT,
                                timeout=3600).returncode
        os.replace(os.path.join(a.logs, f"compile-{s['set']}.log.tmp"), os.path.join(a.logs, f"compile-{s['set']}.log"))
        if rc != 0:
            sys.exit(f"{s['set']}: kotlinc rc={rc}, see {a.logs}/compile-{s['set']}.log")
        dest = os.path.join(a.classes, s['set'])
        os.makedirs(dest, exist_ok=True)
        shutil.copytree(out, dest, dirs_exist_ok=True)
        nclasses = sum(1 for dp, _, fs in os.walk(out) for f in fs if f.endswith('.class'))
        rel = lambda p: os.path.relpath(p, a.vf_src)
        with open(os.path.join(a.logs, f"sources-{s['set']}.txt"), 'w') as f:
            f.write(''.join(rel(p) + '\n' for p in sources))
        info = dict(set=s['set'], gradle=s['gradle'], compiler=version, compiler_build=build,
                    kotlinc_home=a.kotlinc_home, jdk_home=os.path.realpath(a.jdk), jdk=jver.strip().splitlines(),
                    java_opts=env['JAVA_OPTS'], module_name=s['module'], free_compiler_args=s['extra'],
                    classpath=CLASSPATH, sources=len(sources), classes=nclasses,
                    seconds=round(time.time() - t0, 1),
                    args=[('${jdk}' if x == a.jdk else '${classes}/' + s['set'] if x == out else
                           ':'.join('${m2}/' + j for j in CLASSPATH) if x == ':'.join(cp) else
                           '${vf}/' + rel(x) if x in sources else x) for x in args])
        with open(os.path.join(a.logs, f"compile-info-{s['set']}.json.tmp"), 'w') as f:
            json.dump(info, f, indent=1)
        os.replace(os.path.join(a.logs, f"compile-info-{s['set']}.json.tmp"),
                   os.path.join(a.logs, f"compile-info-{s['set']}.json"))
        print(f"{s['set']}: {len(sources)} sources -> {nclasses} classes in {info['seconds']}s", file=sys.stderr)


if __name__ == '__main__':
    main()
