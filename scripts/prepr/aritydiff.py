"""Compare the call arities two kuna builds print over real binaries.

    python -m scripts.prepr.aritydiff BASE_KUNA HEAD_KUNA [--corpus msvc32] [BINARY ...]
        [--option NAME VALUE] [--verbose] [--json] [--max-fn-seconds N]

Both builds run `kuna decompile-all --json` over every binary. In each function
whose printed code differs, the calls are matched by callee and order, and each
argument count that changed is reported. The run fails (exit status 1) when any
call loses an argument, when a function stops decompiling, or when a binary
prints fewer calls than it did; arguments gained are reported but do not fail
the run. Both kinds have to be checked against the disassembly before a PR: a
gained argument can be a stray stack slot, a lost one is usually a regression.

`--option` applies to the head build only, so a default-off option can be
measured before it ships.
"""
import argparse
import collections
import json
import os
import re
import subprocess
import sys
from dataclasses import dataclass, field
from pathlib import Path
from typing import Optional

from scripts.paths import specs_dir
from scripts.prepr import corpus

NOT_CALLS = {"if", "while", "for", "switch", "return", "sizeof", "do", "else",
             "void", "char", "short", "int", "long", "float", "double", "unsigned", "signed", "bool", "code"}
NOT_CALL_PREFIXES = ("CONCAT", "SUB", "ZEXT", "SEXT", "SBORROW", "SCARRY", "CARRY")
NAMED_CALL = re.compile(r"\b([A-Za-z_][A-Za-z0-9_:.$@?]*)\s*\(")
STRING = re.compile(r'"(\\.|[^"\\])*"')
CHAR = re.compile(r"'(\\.|[^'\\])*'")


def _argument_count(body: str, start: int) -> tuple[int, int]:
    """The arguments of the call whose `(` ends at `start`, and where it closes."""
    depth, commas, seen, i = 1, 0, False, start
    while i < len(body) and depth:
        c = body[i]
        if c in "([{":
            depth += 1
        elif c in ")]}":
            depth -= 1
        elif c == "," and depth == 1:
            commas += 1
        if depth and not c.isspace():
            seen = True
        i += 1
    return (commas + 1 if seen else 0), i


def _group_start(body: str, close: int) -> int:
    """The index of the `(` matching the `)` at `close`, or -1."""
    depth = 0
    for i in range(close, -1, -1):
        if body[i] == ")":
            depth += 1
        elif body[i] == "(":
            depth -= 1
            if depth == 0:
                return i
    return -1


def calls(code: str) -> list[tuple[str, int]]:
    """The calls in a function's printed body, in order, as (callee, argument count).

    A call through a pointer, `(*expr)(...)`, is named `(*)`.
    """
    brace = code.find("{")
    body = code[brace:] if brace >= 0 else code
    body = CHAR.sub("0", STRING.sub('""', body))
    found = []
    for m in NAMED_CALL.finditer(body):
        name = m.group(1)
        if name in NOT_CALLS or name.startswith(NOT_CALL_PREFIXES):
            continue
        found.append((m.start(), name, _argument_count(body, m.end())[0]))
    for m in re.finditer(r"\)\s*\(", body):
        start = _group_start(body, m.start())
        inside = body[start + 1:m.start()] if start >= 0 else ""
        if inside.lstrip().startswith("*") and inside.strip(" *"):
            found.append((start, "(*)", _argument_count(body, m.end())[0]))
    return [(name, count) for _, name, count in sorted(found)]


@dataclass
class Change:
    function: str
    callee: str
    before: int
    after: int


@dataclass
class BinaryReport:
    binary: str
    functions: int = 0
    changed: int = 0
    calls_before: int = 0
    calls_after: int = 0
    gained: list = field(default_factory=list)
    lost: list = field(default_factory=list)
    vanished: list = field(default_factory=list)
    errored: list = field(default_factory=list)

    @property
    def blocking(self) -> bool:
        return bool(self.lost or self.vanished or self.errored or self.calls_after < self.calls_before)


def functions(output: str) -> dict[str, dict]:
    """The `decompile-all --json` records, keyed by entry address."""
    return {f["address_hex"]: f for f in json.loads(output).get("functions", [])}


def compare(binary: str, before: dict[str, dict], after: dict[str, dict]) -> BinaryReport:
    report = BinaryReport(binary, functions=len(before))
    for address, old in before.items():
        new = after.get(address)
        old_calls = calls(old.get("code") or "")
        report.calls_before += len(old_calls)
        if new is None:
            report.vanished.append(address)
            report.changed += 1
            continue
        new_calls = calls(new.get("code") or "")
        report.calls_after += len(new_calls)
        if new.get("error") and not old.get("error"):
            report.errored.append(f"{address}: {new['error']}")
        if old.get("code") == new.get("code"):
            continue
        report.changed += 1
        label = f"{old.get('name')} @ {address}"
        by_callee_old, by_callee_new = collections.defaultdict(list), collections.defaultdict(list)
        for name, count in old_calls:
            by_callee_old[name].append(count)
        for name, count in new_calls:
            by_callee_new[name].append(count)
        for name, counts in by_callee_old.items():
            others = by_callee_new.get(name, [])
            if len(others) != len(counts):
                continue
            for x, y in zip(counts, others):
                if y > x:
                    report.gained.append(Change(label, name, x, y))
                elif y < x:
                    report.lost.append(Change(label, name, x, y))
    for address, new in after.items():
        if address not in before:
            report.calls_after += len(calls(new.get("code") or ""))
    return report


def decompile(kuna: str, binary: Path, options: list[str], max_fn_seconds: int) -> dict[str, dict]:
    env = dict(os.environ)
    env.setdefault("KUNA_SPECS", str(specs_dir()))
    result = subprocess.run(
        [kuna, "decompile-all", str(binary), "--json", "--max-fn-seconds", str(max_fn_seconds), *options],
        capture_output=True, text=True, errors="replace", env=env,
    )
    if result.returncode != 0 or not result.stdout.strip():
        raise RuntimeError(f"{kuna} decompile-all {binary} failed: {result.stderr.strip()[:400]}")
    return functions(result.stdout)


def main(argv: Optional[list[str]] = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    parser.add_argument("base", help="the kuna binary built from the base commit")
    parser.add_argument("head", help="the kuna binary built from the change")
    parser.add_argument("binaries", nargs="*", type=Path, help="extra binaries to compare")
    parser.add_argument("--corpus", action="append", default=[],
                        help=f"a corpus to include: {', '.join(sorted(corpus.CORPORA))} (pinned), or local "
                             "(the binaries listed in $KUNA_PREPR_LOCAL)")
    parser.add_argument("--option", nargs=2, action="append", default=[], metavar=("NAME", "VALUE"),
                        help="an option for the head build only")
    parser.add_argument("--max-fn-seconds", type=int, default=30)
    parser.add_argument("--verbose", "-v", action="store_true", help="list every changed call")
    parser.add_argument("--json", action="store_true", help="print the reports as JSON")
    args = parser.parse_args(argv)
    binaries = [path for name in args.corpus for path in corpus.resolve(name)] + args.binaries
    if not binaries:
        parser.error("no binaries: name a --corpus or pass binaries")
    head_options = [token for name, value in args.option for token in ("--option", name, value)]

    reports = []
    for binary in binaries:
        before = decompile(args.base, binary, [], args.max_fn_seconds)
        after = decompile(args.head, binary, head_options, args.max_fn_seconds)
        reports.append(compare(binary.name, before, after))

    if args.json:
        print(json.dumps([report.__dict__ | {"blocking": report.blocking} for report in reports],
                         default=lambda o: o.__dict__, indent=2))
    else:
        print(f"{'binary':32} {'fns':>5} {'changed':>7} {'calls':>13} {'gained':>6} {'lost':>5} {'lost all':>8}")
        for r in reports:
            lost_all = sum(1 for c in r.lost if c.after == 0)
            print(f"{r.binary[:32]:32} {r.functions:5} {r.changed:7} {r.calls_before:6}->{r.calls_after:<6} "
                  f"{len(r.gained):6} {len(r.lost):5} {lost_all:8}")
            for problem in r.vanished:
                print(f"    vanished: {problem}")
            for problem in r.errored:
                print(f"    errored:  {problem}")
            if args.verbose:
                for c in r.lost:
                    print(f"    lost   {c.function}: {c.callee} {c.before} -> {c.after}")
                for c in r.gained:
                    print(f"    gained {c.function}: {c.callee} {c.before} -> {c.after}")
    blocking = [r.binary for r in reports if r.blocking]
    if blocking:
        print(f"FAIL: calls lost arguments, functions failed, or the call count dropped in {', '.join(blocking)}",
              file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
