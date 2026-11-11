"""Command line: ``python -m javabench {fetch,index,run,score,report,compare}``."""

from __future__ import annotations

import argparse
import os


def main(argv: list[str] | None = None) -> None:
    ap = argparse.ArgumentParser(prog="python3 -m scripts.javabench",
                                 description="DecBench-style Java decompiler benchmark")
    ap.add_argument("--workspace", help="the directory for the dataset, results and tools "
                    "(default: $JAVABENCH_WORKSPACE; scripts/javabench/README.md)")
    sub = ap.add_subparsers(dest="cmd", required=True)

    f = sub.add_parser("fetch", help="download + pin the dataset (reproduces <dataset>/<project>/)")
    f.add_argument("--project", action="append", help="limit to project(s); repeatable")
    f.add_argument("--force", action="store_true", help="rebuild project dirs (old ones are renamed, never deleted)")

    i = sub.add_parser("index", help="(re)build the ground-truth index")
    i.add_argument("--project", action="append")
    i.add_argument("--rebuild", action="store_true")
    i.add_argument("--jobs", type=int, default=16)

    r = sub.add_parser("run", help="decompile and score")
    r.add_argument("--decompiler", required=True, choices=["vineflower", "kuna", "source"])
    r.add_argument("--project", action="append")
    r.add_argument("--full", action="store_true", help="every scorable method instead of the default sample")
    r.add_argument("--jobs", type=int, default=16)
    r.add_argument("--run-name")
    r.add_argument("--kuna-bin", default=os.environ.get("KUNA_BIN"))
    r.add_argument("--no-score", action="store_true")
    r.add_argument("--skip-bytes", action="store_true", help="skip recompilation (GED + types only; fast loop)")
    r.add_argument("--timeout", type=int, default=7200, help="per-project decompiler timeout (s)")
    r.add_argument("--javac-jdk", type=int, choices=[17, 19, 21, 25], help="override the recompiling javac (calibration)")

    s = sub.add_parser("score", help="(re)score an existing run")
    s.add_argument("run")
    s.add_argument("--project", action="append")
    s.add_argument("--jobs", type=int, default=16)
    s.add_argument("--kuna-bin", default=os.environ.get("KUNA_BIN"))
    s.add_argument("--skip-bytes", action="store_true")
    s.add_argument("--javac-jdk", type=int, choices=[17, 19, 21, 25])
    s.add_argument("--reuse-bytes", action="store_true",
                   help="re-score GED/types only; reuse the run's previous recompilation results")

    p = sub.add_parser("report", help="print/write scoreboards for run(s)")
    p.add_argument("runs", nargs="+")

    c = sub.add_parser("compare", help="methods where A is perfect and B is not (and vice versa)")
    c.add_argument("run_a")
    c.add_argument("run_b")
    c.add_argument("--project", action="append")
    c.add_argument("--metric", action="append", choices=["ged", "types", "bytes"])
    c.add_argument("--limit", type=int, default=25, help="entries per list in the text output")
    c.add_argument("--json", help="write the full (unlimited) comparison as JSON here")

    args = ap.parse_args(argv)
    from . import projects

    projects.configure(args.workspace)
    if args.cmd == "fetch":
        from . import fetch

        fetch.main(args.project, force=args.force)
    elif args.cmd == "index":
        from . import projects as P
        from .index import load_index

        for proj in P.selected(args.project):
            idx = load_index(proj, rebuild=args.rebuild, jobs=args.jobs)
            print(proj.name, idx["summary"])
    elif args.cmd == "run":
        from .run import run

        run(args.decompiler, args.project, args.full, args.jobs, args.run_name, args.kuna_bin,
            score=not args.no_score, skip_bytes=args.skip_bytes, timeout=args.timeout, javac_jdk=args.javac_jdk)
    elif args.cmd == "score":
        from .run import score_run

        score_run(args.run, args.project, args.jobs, args.kuna_bin, skip_bytes=args.skip_bytes,
                  javac_jdk=args.javac_jdk, reuse_bytes=args.reuse_bytes)
    elif args.cmd == "report":
        for r in args.runs:
            projects.resolve_run(r)
        from .run import report

        report(args.runs)
    elif args.cmd == "compare":
        for r in (args.run_a, args.run_b):
            projects.resolve_run(r)
        from .run import compare

        compare(args.run_a, args.run_b, args.project, args.metric or ["ged", "types", "bytes"],
                args.limit, args.json)
