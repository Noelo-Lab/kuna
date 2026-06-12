#!/usr/bin/env python3
"""Driver for the B2-grade STRUCTURAL lift differential (item `w5-infra-lift-diff`).

The differential itself lives in Rust:
  - `rust/crates/kuna-harness/src/liftdiff.rs` parses the C++ B2 `print raw`
    snapshot format and compares it, op-for-op and block-for-block, against the
    structural model the Rust `FlowInfo` follow-flow + basic-block builder produces.
    Blocks are keyed by INDEX (print order), never by cover-start (which is not
    unique) — see the F1 repair note in that module;
  - `rust/crates/kuna-harness/src/corpus.rs` holds the pinned `CASES` table, the
    bootstrap, the extractor, and `run_self_test()` (the shared driver);
  - `rust/crates/kuna-harness/tests/lift_diff.rs` is the integration gate and
    `rust/crates/kuna-lift-diff` is the `--self-test` binary (the declared item
    gate `cargo run -p kuna-lift-diff -- --self-test`).  Both call the same
    `run_self_test()` (>= 8 of the pinned corpus cases structurally equal end-to-end;
    every divergence/exclusion attributed to a named unported pre-heritage action or
    a W4 flow seam).

This script is a thin convenience wrapper with three subcommands:

    python tools/rust-port/liftdiff.py snapshot   # (re)generate the committed B2
                                                  # fixtures from the C++ oracle
    python tools/rust-port/liftdiff.py test       # run the Rust gate (cargo test)
    python tools/rust-port/liftdiff.py selftest   # run the declared bin gate

`snapshot` must run from the MAIN checkout (it needs `decompiler/cpp/decomp_dbg`
built and `make specs` done); it drives `kuna.goldens` to snapshot B2 for each
pinned corpus test and copies the first decompile's dump (`000-B2.txt`) into the
committed fixtures.  See `rust/crates/kuna-harness/tests/fixtures/README.md`.
"""
import argparse
import os
import shutil
import subprocess
import sys

# The pinned corpus stems (kept in lock-step with the `CASES` table in
# `rust/crates/kuna-harness/src/corpus.rs`).
CASES = [
    "lzcount", "boolless", "skipnext2", "floatprint", "promotecompare",
    "readvolatile", "condconst", "gh6990-returnpair",        # PASS cases
    "ccmp", "convert", "nan", "gh1243-8051-addc",            # DIVERGENT (call/volatile)
    "condexesub", "gp", "sbyte",                             # DIVERGENT/EXCLUDED
]

REPO = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", ".."))
FIXTURES = os.path.join(REPO, "rust", "crates", "kuna-harness", "tests", "fixtures")
SNAPDIR = os.path.join(REPO, "tests", "golden", "snapshots", "cpp")


def do_snapshot(args):
    """Regenerate the committed B2 fixtures from the C++ oracle via kuna.goldens."""
    cmd = [sys.executable, "-m", "kuna.goldens", "snapshot",
           "--boundary", "B2", "--engine", "cpp"]
    for stem in CASES:
        cmd += ["--xml", stem]
    print("running:", " ".join(cmd))
    r = subprocess.run(cmd, cwd=REPO)
    if r.returncode != 0:
        print("kuna.goldens snapshot failed", file=sys.stderr)
        return r.returncode

    n = 0
    for stem in CASES:
        src = os.path.join(SNAPDIR, stem, "000-B2.txt")
        dst = os.path.join(FIXTURES, "%s.b2.txt" % stem)
        if not os.path.isfile(src):
            print("MISSING snapshot for %s (%s)" % (stem, src), file=sys.stderr)
            return 1
        shutil.copyfile(src, dst)
        n += 1
        print("  copied %-20s -> %s" % (stem, os.path.relpath(dst, REPO)))
    print("%d fixtures refreshed" % n)
    return 0


def do_test(args):
    """Run the Rust lift-diff gate (and clippy), printing the per-test table."""
    rust = os.path.join(REPO, "rust")
    rc = subprocess.run(
        ["cargo", "test", "-p", "kuna-harness", "--test", "lift_diff",
         "--", "--nocapture"],
        cwd=rust,
    ).returncode
    if rc != 0:
        return rc
    return subprocess.run(
        ["cargo", "clippy", "-p", "kuna-harness", "--all-targets", "--", "-D", "warnings"],
        cwd=rust,
    ).returncode


def do_selftest(args):
    """Run the declared item gate: `cargo run -p kuna-lift-diff -- --self-test`."""
    rust = os.path.join(REPO, "rust")
    return subprocess.run(
        ["cargo", "run", "-p", "kuna-lift-diff", "--", "--self-test"],
        cwd=rust,
    ).returncode


def main():
    p = argparse.ArgumentParser(description=__doc__,
                                formatter_class=argparse.RawDescriptionHelpFormatter)
    sub = p.add_subparsers(dest="cmd", required=True)
    sub.add_parser("snapshot", help="regenerate the committed B2 fixtures").set_defaults(
        func=do_snapshot)
    sub.add_parser("test", help="run the Rust lift-diff gate + clippy").set_defaults(
        func=do_test)
    sub.add_parser("selftest", help="run the declared bin gate (--self-test)").set_defaults(
        func=do_selftest)
    args = p.parse_args()
    sys.exit(args.func(args))


if __name__ == "__main__":
    main()
