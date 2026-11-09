#!/usr/bin/env python3
"""Trace Vineflower over the oracle corpus (manifest.json), one JVM per case, in parallel.

Each runnable case is replayed exactly like the corpus fixture oracle (--fixture + the case's effective
options + its class files in addSource order) through run.sh, writing OUT/<case id>.vft. The produced
.java is compared with the case's oracle file, so a tracer that changed Vineflower's behaviour shows up.
A report goes to OUT/report.json (per case: seconds, trace bytes, methods, steps, output match).

usage: corpus_trace.py --out DIR [--hash stock|index|random:SEED] [--shift K] [--jobs 16] [--xmx 1g]
                       [--only SUBSTR]... [--limit N] [--manifest PATH] [--corpus DIR]
"""
import argparse
import concurrent.futures as cf
import json
import os
import subprocess
import sys
import time

HERE = os.path.dirname(os.path.abspath(__file__))


def resolve(value, java_homes):
    if isinstance(value, str) and value.startswith("${java.") and value.endswith(".home}"):
        return java_homes[value[len("${java."):-len(".home}")]]
    return value


def run_case(t, a, java_homes):
    trace = os.path.join(a.out, t["id"] + ".vft")
    work = os.path.join(a.out, "work", t["id"])
    os.makedirs(os.path.dirname(trace), exist_ok=True)
    os.makedirs(work, exist_ok=True)
    cmd = [os.path.join(HERE, "run.sh"), "--hash", a.hash, "--trace", trace, "--shift", str(a.shift), "--fixture"]
    for k, v in t["options"].items():
        if isinstance(v, dict) and "$javadoc_provider" in v:
            cmd += ["--javadoc", v["$javadoc_provider"]]
        else:
            cmd += ["--opt", f"{k}={resolve(v, java_homes)}"]
    cmd += [os.path.join(a.corpus, "classes", f) for f in t["class_files"]]
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
    expected = os.path.join(a.corpus, t["oracle"])
    match = None
    if os.path.isfile(produced) and os.path.isfile(expected):
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
                output_match=match)


def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--out", required=True)
    ap.add_argument("--hash", default="stock")
    ap.add_argument("--shift", type=int, default=0)
    ap.add_argument("--jobs", type=int, default=16)
    ap.add_argument("--xmx", default="1g")
    ap.add_argument("--timeout", type=int, default=600)
    ap.add_argument("--only", action="append")
    ap.add_argument("--limit", type=int, default=0)
    ap.add_argument("--manifest", default=os.path.expanduser("~/kwt/jvm/corpus/manifest.json"))
    ap.add_argument("--corpus", default=os.path.expanduser("~/kwt/jvm/corpus"))
    a = ap.parse_args()
    a.out = os.path.abspath(a.out)
    m = json.load(open(a.manifest, encoding="utf-8"))
    tests = [t for t in m["tests"] if t["status"] == "ok"]
    if a.only:
        tests = [t for t in tests if any(s in t["id"] for s in a.only)]
    tests.sort(key=lambda t: t["id"])
    if a.limit:
        tests = tests[:a.limit]
    t0 = time.time()
    results = []
    with cf.ThreadPoolExecutor(max_workers=a.jobs) as ex:
        for r in ex.map(lambda t: run_case(t, a, m["java_homes"]), tests):
            results.append(r)
            if r["rc"] != 0 or r["output_match"] is False:
                print(f"  {r['id']}: rc={r['rc']} output_match={r['output_match']}", file=sys.stderr)
    wall = round(time.time() - t0, 1)
    summary = dict(hash=a.hash, shift=a.shift, cases=len(results), wall_seconds=wall,
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
