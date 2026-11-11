#!/usr/bin/env python3
"""Trace Vineflower or kuna over the oracle corpus (manifest.json) or a class directory, one process per
case, in parallel.

Each runnable case is replayed exactly like the corpus fixture oracle (--fixture + the case's effective
options + its class files in addSource order) through run.sh (Vineflower) or the jvmtrace example (kuna),
writing OUT/<case id>.vft. The produced .java is compared with the case's oracle file, so a tracer that
changed an engine's behaviour shows up. A report goes to OUT/report.json (per case: seconds, trace bytes,
methods, steps, output match, produced file).

--classes DIR replaces the manifest by DIR's class files: each top-level class together with its nested
classes (X.class, X$*.class) is one case "<--set>/<path of X>", run with the --opt pairs given (no fixture
base map unless --fixture); there is no oracle, so sweep_compare.py compares the two engines' outputs.
With the manifest, --opt pairs overlay every case's options (the oracle comparison is then meaningless).

usage: corpus_trace.py --out DIR [--engine vineflower|kuna] [--kuna BIN] [--hash stock|index|random:SEED]
                       [--shift K] [--jobs 16] [--xmx 1g] [--only SUBSTR]... [--limit N]
                       [--manifest PATH] [--corpus DIR]
                       [--classes DIR [--set NAME] [--fixture] [--opt k=v]... [--sample N --seed S]]
                       [--lib PATH]...
"""
import argparse
import concurrent.futures as cf
import json
import os
import random
import subprocess
import sys
import time

HERE = os.path.dirname(os.path.abspath(__file__))
NO_ORACLE = ('KUNA_JVM_ORACLE is not set: set it to the JVM oracle workspace '
             '(decompiler/nonnative/kuna-jvm/tools/oracle/README.md), or pass --corpus, --manifest or --classes')


def resolve(value, java_homes):
    if isinstance(value, str) and value.startswith("${java.") and value.endswith(".home}"):
        return java_homes[value[len("${java."):-len(".home}")]]
    return value


def engine_cmd(a, trace):
    if a.engine == "kuna":
        return [a.kuna, "--trace", trace]
    return [os.path.join(HERE, "run.sh"), "--hash", a.hash, "--trace", trace, "--shift", str(a.shift)]


def run_case(t, a, java_homes):
    trace = os.path.join(a.out, t["id"] + ".vft")
    work = os.path.join(a.out, "work", t["id"])
    os.makedirs(os.path.dirname(trace), exist_ok=True)
    os.makedirs(work, exist_ok=True)
    cmd = engine_cmd(a, trace)
    if t.get("fixture", True):
        cmd.append("--fixture")
    for k, v in t["options"].items():
        if isinstance(v, dict) and "$javadoc_provider" in v:
            cmd += ["--javadoc", v["$javadoc_provider"]]
        else:
            cmd += ["--opt", f"{k}={resolve(v, java_homes)}"]
    for lib in a.lib or []:
        cmd += ["--lib", lib]
    cmd += [f if os.path.isabs(f) else os.path.join(a.corpus, "classes", f) for f in t["class_files"]]
    outdir = os.path.join(work, "out")
    cmd += ["-o", outdir]
    env = dict(os.environ, LANG="C.UTF-8", LC_ALL="C.UTF-8", VF_XMX=a.xmx)
    t0 = time.time()
    with open(os.path.join(work, "log.txt"), "wb") as log:
        try:
            rc = subprocess.run(cmd, stdout=log, stderr=subprocess.STDOUT, env=env, cwd=work,
                                timeout=a.timeout).returncode
        except subprocess.TimeoutExpired:
            rc = "timeout"
    secs = round(time.time() - t0, 2)
    produced = os.path.join(outdir, t["decompiled_file"])
    expected = os.path.join(a.corpus, t["oracle"]) if t.get("oracle") else None
    match = None
    if expected and os.path.isfile(produced) and os.path.isfile(expected):
        with open(produced, "rb") as f1, open(expected, "rb") as f2:
            match = f1.read() == f2.read()
    size = os.path.getsize(trace) if os.path.isfile(trace) else 0
    keys = set()
    steps = 0
    if size:
        with open(trace, encoding="utf-8") as f:
            for line in f:
                if line.startswith("=M "):
                    keys.add(line)
                elif line.startswith("@ "):
                    steps += 1
    return dict(id=t["id"], rc=rc, seconds=secs, trace_bytes=size, methods=len(keys), steps=steps,
                output_match=match, produced=os.path.relpath(produced, a.out) if os.path.isfile(produced) else None)


def class_dir_cases(a):
    """One case per top-level class of a.classes (with its X$*.class siblings, name-sorted)."""
    groups = {}
    for dirpath, _, files in os.walk(a.classes):
        for f in files:
            if not f.endswith(".class") or f in ("module-info.class", "package-info.class"):
                continue
            outer = f[:-len(".class")].split("$", 1)[0]
            groups.setdefault((dirpath, outer), []).append(os.path.join(dirpath, f))
    options = {}
    for kv in a.opt or []:
        k, _, v = kv.partition("=")
        options[k] = v
    tests = []
    for (dirpath, outer), files in groups.items():
        primary = os.path.join(dirpath, outer + ".class")
        if primary not in files:
            continue
        rest = sorted(f for f in files if f != primary)
        rel = os.path.relpath(os.path.join(dirpath, outer), a.classes)
        tests.append(dict(id=f"{a.set}/{rel}", options=options, class_files=[primary] + rest,
                          decompiled_file=outer + ".java", oracle=None, fixture=a.fixture))
    tests.sort(key=lambda t: t["id"])
    if a.sample:
        tests = sorted(random.Random(a.seed).sample(tests, min(a.sample, len(tests))), key=lambda t: t["id"])
    return tests


def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--out", required=True)
    ap.add_argument("--engine", choices=["vineflower", "kuna"], default="vineflower")
    ap.add_argument("--kuna", default=None, help="jvmtrace binary (engine kuna)")
    ap.add_argument("--hash", default="stock")
    ap.add_argument("--shift", type=int, default=0)
    ap.add_argument("--jobs", type=int, default=16)
    ap.add_argument("--xmx", default="1g")
    ap.add_argument("--timeout", type=int, default=600)
    ap.add_argument("--only", action="append")
    ap.add_argument("--limit", type=int, default=0)
    ap.add_argument("--manifest", default=None)
    ap.add_argument("--corpus", help="the oracle corpus (default: $KUNA_JVM_ORACLE/corpus)")
    ap.add_argument("--classes", default=None)
    ap.add_argument("--set", default="classes")
    ap.add_argument("--fixture", action="store_true")
    ap.add_argument("--opt", action="append")
    ap.add_argument("--lib", action="append", help="library (addLibrary) for every case, both engines")
    ap.add_argument("--sample", type=int, default=0)
    ap.add_argument("--seed", type=int, default=1)
    a = ap.parse_args()
    a.out = os.path.abspath(a.out)
    if a.engine == "kuna" and not a.kuna:
        ap.error("--engine kuna needs --kuna BIN")
    java_homes = {}
    if a.classes:
        a.classes = os.path.abspath(a.classes)
        tests = class_dir_cases(a)
    else:
        if not a.corpus:
            if os.environ.get("KUNA_JVM_ORACLE"):
                a.corpus = os.path.join(os.environ["KUNA_JVM_ORACLE"], "corpus")
            elif a.manifest:
                a.corpus = os.path.dirname(os.path.abspath(a.manifest))
            else:
                sys.exit(NO_ORACLE)
        m = json.load(open(a.manifest or os.path.join(a.corpus, "manifest.json"), encoding="utf-8"))
        java_homes = m["java_homes"]
        tests = [t for t in m["tests"] if t["status"] == "ok"]
        overlay = dict(kv.partition("=")[::2] for kv in a.opt or [])
        if overlay:
            tests = [dict(t, options={**t["options"], **overlay}) for t in tests]
    if a.only:
        tests = [t for t in tests if any(s in t["id"] for s in a.only)]
    tests.sort(key=lambda t: t["id"])
    if a.limit:
        tests = tests[:a.limit]
    os.makedirs(a.out, exist_ok=True)
    t0 = time.time()
    results = []
    with cf.ThreadPoolExecutor(max_workers=a.jobs) as ex:
        for r in ex.map(lambda t: run_case(t, a, java_homes), tests):
            results.append(r)
            if r["rc"] != 0 or r["output_match"] is False:
                print(f"  {r['id']}: rc={r['rc']} output_match={r['output_match']}", file=sys.stderr)
    wall = round(time.time() - t0, 1)
    summary = dict(engine=a.engine, hash=a.hash, shift=a.shift, cases=len(results), wall_seconds=wall,
                   cpu_seconds=round(sum(r["seconds"] for r in results), 1),
                   trace_bytes=sum(r["trace_bytes"] for r in results),
                   methods=sum(r["methods"] for r in results), steps=sum(r["steps"] for r in results),
                   nonzero_rc=sum(1 for r in results if r["rc"] != 0),
                   output_mismatch=sum(1 for r in results if r["output_match"] is False),
                   output_missing=sum(1 for r in results if r["output_match"] is None),
                   max_trace_bytes=max((r["trace_bytes"] for r in results), default=0))
    with open(os.path.join(a.out, "report.json"), "w") as f:
        json.dump(dict(summary=summary, results=results), f, indent=1)
    print(json.dumps(summary), file=sys.stderr)


if __name__ == "__main__":
    main()
