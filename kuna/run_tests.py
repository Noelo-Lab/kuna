"""Run the extracted Ghidra C++ decompiler test suite, with baseline parity checking.

Wraps the upstream ``decomp_test_dbg`` harness (204 C++ unit tests + 83 XML
datatests). Unit-test results are emitted on **stderr**, datatest results on
**stdout**; this runner captures the two streams separately and parses each.

CLI:
    python -m kuna.run_tests [--all | --unittests | --datatests] [--name N ...]
        [--binary PATH] [--sleighpath DIR] [--datatests-dir DIR]
        [--baseline FILE] [--save-baseline FILE] [--json]

Exit code mirrors the harness (number of failed tests, clamped at 255), or is
forced non-zero on a baseline regression.
"""
import argparse
import json
import os
import re
import subprocess
import sys

from . import paths

# --- parsing -----------------------------------------------------------------

_UNIT_OPEN = re.compile(r"^testing : (?P<name>.+) \.\.\.\s*$")
_UNIT_PASS = re.compile(r"^  passed\.\s*$")
_UNIT_FAIL = re.compile(r"^  fail(?:: (?P<msg>.*))?\s*$")
_UNIT_FOOT = re.compile(r"^(?P<passed>\d+)/(?P<total>\d+) tests passed\.\s*$")

_DATA_PASS = re.compile(r"^Success -- (?P<name>.+?)\s*$")
_DATA_FAIL = re.compile(r"^FAIL -- (?P<name>.+?)\s*$")
_DATA_APPLIED = re.compile(r"^Total tests applied = (?P<n>\d+)\s*$")
_DATA_PASSING = re.compile(r"^Total passing tests = (?P<n>\d+)\s*$")
_DATA_ERR = re.compile(
    r"^(?:Error (?:parsing|executing) (?P<f1>.+?): .*"
    r"|Error: Did not apply tests in (?P<f2>.+?)"
    r"|No output for (?P<f3>.+?))\s*$"
)


def _parse_unit(stderr):
    """Return (passing_names, failing[(name,msg)], (passed,total) or None)."""
    passing, failing = [], []
    footer = None
    current = None
    for line in stderr.splitlines():
        m = _UNIT_OPEN.match(line)
        if m:
            current = m.group("name")
            continue
        if current is not None:
            if _UNIT_PASS.match(line):
                passing.append(current)
                current = None
                continue
            mf = _UNIT_FAIL.match(line)
            if mf:
                failing.append((current, mf.group("msg") or ""))
                current = None
                continue
        mfoot = _UNIT_FOOT.match(line)
        if mfoot:
            footer = (int(mfoot.group("passed")), int(mfoot.group("total")))
    return passing, failing, footer


def _parse_data(stdout):
    """Return (passing_names, failing_names, file_errors, (applied,passing) or None)."""
    passing, failing, errors = [], [], []
    applied = passing_total = None
    for line in stdout.splitlines():
        m = _DATA_PASS.match(line)
        if m:
            passing.append(m.group("name"))
            continue
        m = _DATA_FAIL.match(line)
        if m:
            failing.append(m.group("name"))
            continue
        m = _DATA_ERR.match(line)
        if m:
            errors.append(m.group("f1") or m.group("f2") or m.group("f3"))
            continue
        m = _DATA_APPLIED.match(line)
        if m:
            applied = int(m.group("n"))
            continue
        m = _DATA_PASSING.match(line)
        if m:
            passing_total = int(m.group("n"))
            continue
    footer = (applied, passing_total) if applied is not None else None
    return passing, failing, errors, footer


# --- running -----------------------------------------------------------------


def run(mode="all", names=None, binary=None, sleighpath=None, datatests_dir=None):
    """Invoke decomp_test_dbg and return a structured result dict."""
    names = names or []
    bin_path = str(paths.decomp_test_dbg()) if binary is None else os.path.abspath(str(binary))
    if not os.path.exists(bin_path):
        raise FileNotFoundError(
            "decomp_test_dbg not built at %s -- run `make binaries`" % bin_path
        )
    specs = str(paths.specs_dir()) if sleighpath is None else os.path.abspath(str(sleighpath))
    dtdir = str(paths.datatests_dir()) if datatests_dir is None else os.path.abspath(str(datatests_dir))

    argv = [bin_path, "-sleighpath", specs, "-path", dtdir]

    # Datatest names must be bare *.xml filenames; warn on ones not present.
    if mode == "unittests":
        argv += ["unittests"] + names
    elif mode == "datatests":
        fixed = []
        for n in names:
            n2 = n if n.endswith(".xml") else n + ".xml"
            if not os.path.exists(os.path.join(dtdir, n2)):
                print("warning: datatest %r not found in %s (will be skipped)" % (n2, dtdir),
                      file=sys.stderr)
            fixed.append(n2)
        argv += ["datatests"] + fixed
    # mode == "all": no extra token -> runs both unit and data tests.

    proc = subprocess.run(argv, capture_output=True, text=True)

    up, uf, ufoot = _parse_unit(proc.stderr)
    dp, df, derr, dfoot = _parse_data(proc.stdout)

    return {
        "returncode": proc.returncode,
        "argv": argv,
        "unit": {"passing": up, "failing": uf, "footer": ufoot},
        "data": {"passing": dp, "failing": df, "errors": derr, "footer": dfoot},
        "stdout": proc.stdout,
        "stderr": proc.stderr,
    }


def _pass_keyset(result):
    """Fully-qualified set of passing test keys (for baseline diffing)."""
    keys = set("unit:" + n for n in result["unit"]["passing"])
    keys |= set("data:" + n for n in result["data"]["passing"])
    return keys


def _summary(result):
    u, d = result["unit"], result["data"]
    lines = []
    if u["footer"]:
        lines.append("unittests: %d/%d passed" % (u["footer"][0], u["footer"][1]))
    elif u["passing"] or u["failing"]:
        lines.append("unittests: %d passed, %d failed" % (len(u["passing"]), len(u["failing"])))
    if d["footer"]:
        lines.append("datatests: %d/%d assertions passed" % (d["footer"][1], d["footer"][0]))
    elif d["passing"] or d["failing"]:
        lines.append("datatests: %d passed, %d failed" % (len(d["passing"]), len(d["failing"])))
    fails = [("[unit] %s: %s" % (n, m)) for n, m in u["failing"]]
    fails += [("[data] %s" % n) for n in d["failing"]]
    fails += [("[data] ERROR in %s" % f) for f in d["errors"]]
    if fails:
        lines.append("FAILURES:")
        lines.extend("  " + f for f in fails)
    lines.append("exit: %d" % result["returncode"])
    return "\n".join(lines)


def main(argv=None):
    p = argparse.ArgumentParser(
        prog="python -m kuna.run_tests",
        description="Run the extracted Ghidra C++ decompiler test suite.",
    )
    g = p.add_mutually_exclusive_group()
    g.add_argument("--all", action="store_const", const="all", dest="mode")
    g.add_argument("--unittests", action="store_const", const="unittests", dest="mode")
    g.add_argument("--datatests", action="store_const", const="datatests", dest="mode")
    p.set_defaults(mode="all")
    p.add_argument("--name", action="append", default=[], dest="names",
                   help="run only the named test(s); repeatable. Requires --unittests/--datatests")
    p.add_argument("--binary", default=None, help="path to decomp_test_dbg")
    p.add_argument("--sleighpath", default=None, help="SLEIGH specs root")
    p.add_argument("--datatests-dir", default=None, help="datatests directory")
    p.add_argument("--baseline", default=None, help="compare against a saved baseline JSON")
    p.add_argument("--save-baseline", default=None, help="write results as a baseline JSON")
    p.add_argument("--json", action="store_true", help="emit structured JSON instead of text")
    args = p.parse_args(argv)

    if args.names and args.mode == "all":
        p.error("--name requires --unittests or --datatests")

    try:
        result = run(args.mode, args.names, args.binary, args.sleighpath, args.datatests_dir)
    except FileNotFoundError as e:
        print("error: %s" % e, file=sys.stderr)
        return 2

    passkeys = sorted(_pass_keyset(result))
    record = {
        "unit_footer": result["unit"]["footer"],
        "data_footer": result["data"]["footer"],
        "passing": passkeys,
        "returncode": result["returncode"],
    }

    if args.save_baseline:
        with open(args.save_baseline, "w") as fh:
            json.dump(record, fh, indent=2, sort_keys=True)
        print("baseline written to %s (%d passing keys)" % (args.save_baseline, len(passkeys)),
              file=sys.stderr)

    parity_fail = False
    parity_report = None
    if args.baseline:
        with open(args.baseline) as fh:
            base = json.load(fh)
        base_pass = set(base.get("passing", []))
        now_pass = set(passkeys)
        regressed = sorted(base_pass - now_pass)   # passed before, not now
        newly_pass = sorted(now_pass - base_pass)  # informational
        parity_fail = bool(regressed) or result["returncode"] != 0
        parity_report = {
            "regressed": regressed,
            "newly_passing": newly_pass,
            "verdict": "PARITY OK" if not parity_fail else "PARITY FAIL",
        }

    if args.json:
        out = dict(record)
        out["failures"] = {
            "unit": result["unit"]["failing"],
            "data": result["data"]["failing"],
            "data_errors": result["data"]["errors"],
        }
        if parity_report:
            out["parity"] = parity_report
        print(json.dumps(out, indent=2, sort_keys=True))
    else:
        print(_summary(result))
        if parity_report:
            print("\n=== baseline parity ===")
            if parity_report["regressed"]:
                print("REGRESSED (%d):" % len(parity_report["regressed"]))
                for k in parity_report["regressed"]:
                    print("  " + k)
            if parity_report["newly_passing"]:
                print("newly passing (%d): %s" % (
                    len(parity_report["newly_passing"]),
                    ", ".join(parity_report["newly_passing"][:10])
                    + (" ..." if len(parity_report["newly_passing"]) > 10 else "")))
            print(parity_report["verdict"])

    if parity_fail:
        return 1
    return min(result["returncode"], 255)


if __name__ == "__main__":
    sys.exit(main())
