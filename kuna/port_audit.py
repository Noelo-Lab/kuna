"""Audit the Rust port's unit-test coverage against the C++ oracle suites.

The C++ tree is the test oracle for the Rust port (``rust/``): every
``TEST(name)`` in ``decompiler/unittests/*.cc`` must eventually have a
same-named ``#[test] fn name`` somewhere under ``rust/crates/``. This tool
extracts both sides by regex and reports, per C++ suite file, how many tests
are ported and which names are still missing.

CLI:
    python -m kuna.port_audit [--json] [--suite testmarshal,testfloatemu]

Exit code is 0 only when every gated C++ test name is matched in the Rust
tree (all suites by default; ``--suite``/``--waves`` gates only the listed
suites, for per-wave use -- ungated suites are still reported).
"""
import argparse
import json
import re
import sys

from . import paths

_CPP_TEST = re.compile(r"TEST\((\w+)\)")
# `#[test]`, then any further attributes between it and the fn (e.g. #[ignore],
# #[should_panic], #[cfg_attr(...)]), then the function name.
_RUST_TEST = re.compile(r"#\[test\](?:\s*#\[[^\]]*\])*\s*(?:pub\s+)?fn\s+(\w+)")


def cpp_suites():
    """Map C++ suite name (file stem, e.g. ``testmarshal``) -> list of TEST names."""
    suites = {}
    udir = paths.repo_root() / "decompiler" / "unittests"
    for f in sorted(udir.glob("*.cc")):
        names = _CPP_TEST.findall(f.read_text())
        if names:
            suites[f.stem] = names
    return suites


def rust_tests():
    """Set of all ``#[test]`` fn names under rust/crates/ (empty if absent)."""
    root = paths.repo_root() / "rust" / "crates"
    found = set()
    if not root.is_dir():
        return found
    for f in sorted(root.rglob("*.rs")):
        found.update(_RUST_TEST.findall(f.read_text()))
    return found


def audit(gate_suites=None):
    """Return the audit as a dict (suites, totals, gate verdict).

    ``gate_suites`` limits which suites count toward the pass/fail gate
    (None = all). Every suite is always reported.
    """
    suites = cpp_suites()
    rust = rust_tests()
    if gate_suites is not None:
        unknown = sorted(set(gate_suites) - set(suites))
        if unknown:
            raise ValueError("unknown suite(s): %s (have: %s)"
                             % (", ".join(unknown), ", ".join(sorted(suites))))

    report = {}
    for suite, names in suites.items():
        missing = [n for n in names if n not in rust]
        report[suite] = {
            "total": len(names),
            "ported": len(names) - len(missing),
            "missing": missing,
            "gated": gate_suites is None or suite in gate_suites,
        }

    gated = [s for s in report if report[s]["gated"]]
    gate_missing = sum(len(report[s]["missing"]) for s in gated)
    return {
        "suites": report,
        "cpp_total": sum(r["total"] for r in report.values()),
        "ported_total": sum(r["ported"] for r in report.values()),
        "rust_tests_found": len(rust),
        "gate": {
            "suites": sorted(gated),
            "missing": gate_missing,
            "verdict": "PORT COMPLETE" if gate_missing == 0 else "PORT INCOMPLETE",
        },
    }


def _summary(result):
    lines = []
    for suite in sorted(result["suites"]):
        r = result["suites"][suite]
        mark = "" if r["gated"] else "  (not gated)"
        lines.append("%-18s %3d/%3d ported%s" % (suite, r["ported"], r["total"], mark))
        for n in r["missing"]:
            lines.append("  missing: " + n)
    lines.append("total: %d/%d C++ unit tests ported (%d #[test] fns found in rust/crates)"
                 % (result["ported_total"], result["cpp_total"], result["rust_tests_found"]))
    g = result["gate"]
    lines.append("gate (%d suites): %d missing -- %s"
                 % (len(g["suites"]), g["missing"], g["verdict"]))
    return "\n".join(lines)


def main(argv=None):
    p = argparse.ArgumentParser(
        prog="python -m kuna.port_audit",
        description="Report which C++ unit tests have a same-named Rust #[test] port.",
    )
    p.add_argument("--suite", "--waves", dest="suites", default=None, metavar="NAMES",
                   help="comma-separated suite names to gate on (e.g. "
                        "testmarshal,testfloatemu); others are reported but not gated")
    p.add_argument("--json", action="store_true", help="emit structured JSON instead of text")
    args = p.parse_args(argv)

    gate_suites = None
    if args.suites:
        gate_suites = [s.strip() for s in args.suites.split(",") if s.strip()]

    try:
        result = audit(gate_suites)
    except ValueError as e:
        print("error: %s" % e, file=sys.stderr)
        return 2

    if args.json:
        print(json.dumps(result, indent=2, sort_keys=True))
    else:
        print(_summary(result))
    return 0 if result["gate"]["missing"] == 0 else 1


if __name__ == "__main__":
    sys.exit(main())
