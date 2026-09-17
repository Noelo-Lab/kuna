"""Count what `kuna decompile-all` actually declares — the goal-2 instrument.

`type_match` cannot see variable *count*: decbench scores the JSON `variables[]`
array (args + stack symbols + framelayout slots) and a merged-away temporary was
never in it. So "kuna declares fewer/more locals than it should" needs its own
number, and this is it.

Over one binary's `kuna decompile-all` text, per function:

* **declarations** — a 2-space-indented line ending in `;` with no `=` and no `(`
  (comments stripped first), at least two tokens, not a bare `return/goto/…`;
* **single-def/single-read** — a declared name assigned exactly once and read
  exactly once, i.e. a temporary that could have been an expression. A compound
  assignment (`v7 &= x`) counts as both a def and a read of `v7`;
* **`[N]` blobs** — array declarations, tallied by element count. `char vN[16]`
  is the 128-bit-multiply operand pair and its own known wart;
* **declared-type histogram** — the declaration text with the name removed
  (`char *`, `unsigned long`, `char [16]`);
* **params** — the arity of the emitted signature.

A function counts as a *thunk* when its body carries kuna's `jump-as-call`
marker; those are the PLT stubs and they are excluded from the non-thunk
declaration total, which is the number to quote.

With `--json` the same run's `decompile-all --json` is parsed too, so the text
counts sit next to the scored surface (args / stack / framelayout slots with no
line or address — the "useless slot" count).

Usage::

    python3 -m scripts.pipeline.varcensus <binary>                  # roll-up
    python3 -m scripts.pipeline.varcensus <binary> --json --out c.json
    python3 -m scripts.pipeline.varcensus <binary> --option foldcallret off
    python3 -m scripts.pipeline.varcensus <binary> --baseline before.json
    python3 -m scripts.pipeline.varcensus --baseline a.json --compare b.json
    python3 -m scripts.pipeline.varcensus --text captured.txt        # no kuna run
    python3 -m scripts.pipeline.varcensus --selftest

Nothing is written anywhere unless ``--out`` is given.

Pinned baseline (kuna main @809712e9, coreutils `fmt` -O2, the **unstripped**
twin — the stripped copy is one function and 37 declarations away)::

    functions 152 · declarations 332 (295 non-thunk) · single-def/single-read 31
    · [16] blobs 23 ;  --option foldcallret off -> 367 declarations / 52 sdsu

(The design lane quotes 335/298/370 for the same run: its declaration rule kept
three lines this one drops. The deltas, which are what a PR argues from, agree.)
"""
from __future__ import annotations

import argparse
import collections
import json
import re
import subprocess
import sys
from pathlib import Path

from .. import paths

FUNC_SPLIT = "// Function: "
HEADER_RE = re.compile(r"^(?P<name>\S+) @ (?P<addr>0x[0-9a-fA-F]+)")
# `<type tokens> <name>;` / `<type> *<name>;` / `<type> <name> [N];`
DECL_RE = re.compile(r"^(?P<type>.*?)\s*(?P<stars>\**)(?P<name>[A-Za-z_]\w*)"
                     r"\s*(?:\[(?P<count>\d+)\])?;$")
NOT_DECL = {"return", "goto", "break", "continue", "do", "else", "case", "default"}
THUNK_MARK = "jump-as-call"


def strip_comment(line: str) -> str:
    return line.split("//")[0].rstrip()


def decl_lines(body: list[str]) -> list[str]:
    """The declaration lines of one function body (see the module docstring)."""
    out = []
    for raw in body:
        line = strip_comment(raw)
        if not line.startswith("  ") or line.startswith("   "):
            continue
        if not line.endswith(";") or "=" in line or "(" in line:
            continue
        tokens = line.strip().split()
        if len(tokens) < 2 or tokens[0] in NOT_DECL:
            continue
        out.append(line.strip())
    return out


def param_count(signature: str) -> int:
    """Arity of an emitted prototype line, `(void)` and `()` counting as 0."""
    depth = 0
    inner = None
    for i, ch in enumerate(signature):
        if ch == "(":
            depth += 1
            if depth == 1:
                start = i + 1
        elif ch == ")":
            depth -= 1
            if depth == 0:
                inner = signature[start:i]
                break
    if inner is None:
        return 0
    inner = inner.strip()
    if not inner or inner == "void":
        return 0
    depth = 0
    n = 1
    for ch in inner:
        if ch in "([":
            depth += 1
        elif ch in ")]":
            depth -= 1
        elif ch == "," and depth == 0:
            n += 1
    return n


def _uses(body_text: str, name: str) -> tuple[int, int]:
    """(defs, reads) of one declared name over a function body.

    `N = x` is a def; `N op= x`, `N++` and `++N` are a def **and** a read; every
    other occurrence is a read. The declaration itself is neither.
    """
    word = r"\b%s\b" % re.escape(name)
    occurrences = len(re.findall(word, body_text))
    plain = len(re.findall(word + r"\s*=(?!=)", body_text))
    compound = len(re.findall(
        word + r"\s*(?:[+\-*/%&|^]|<<|>>)=", body_text)) + \
        len(re.findall(word + r"\s*(?:\+\+|--)", body_text)) + \
        len(re.findall(r"(?:\+\+|--)\s*" + word, body_text))
    defs = plain + compound
    reads = occurrences - 1 - plain
    return defs, reads


def census_function(chunk: str) -> dict:
    """One `// Function: ` chunk -> its census row."""
    lines = chunk.split("\n")
    head = HEADER_RE.match(lines[0].strip())
    body = lines[1:]
    body_text = "\n".join(strip_comment(l) for l in body)
    signature = next((strip_comment(l) for l in body if l and not l.startswith(" ")
                      and "(" in l), "")
    thunk = THUNK_MARK in chunk
    types: collections.Counter = collections.Counter()
    arrays: collections.Counter = collections.Counter()
    names, unparsed = [], []
    for decl in decl_lines(body):
        m = DECL_RE.match(decl)
        if not m:
            unparsed.append(decl)
            continue
        names.append(m.group("name"))
        spelling = m.group("type") + (" " + m.group("stars") if m.group("stars") else "")
        if m.group("count"):
            spelling += " [%s]" % m.group("count")
            arrays[int(m.group("count"))] += 1
        types[spelling.strip()] += 1
    sdsu = []
    for name in names:
        defs, reads = _uses(body_text, name)
        if defs == 1 and reads == 1:
            sdsu.append(name)
    return {
        "name": head.group("name") if head else "?",
        "address": head.group("addr") if head else None,
        "thunk": thunk,
        "params": param_count(signature),
        "declarations": len(names) + len(unparsed),
        "single_def_single_use": len(sdsu),
        "single_def_single_use_names": sdsu,
        "arrays": {str(k): v for k, v in sorted(arrays.items())},
        "types": dict(types),
        "unparsed": unparsed,
    }


def census_text(text: str) -> list[dict]:
    return [census_function(c) for c in text.split(FUNC_SPLIT)[1:]]


def roll_up(rows: list[dict]) -> dict:
    types: collections.Counter = collections.Counter()
    arrays: collections.Counter = collections.Counter()
    tot = collections.Counter()
    for r in rows:
        tot["functions"] += 1
        tot["thunks"] += int(r["thunk"])
        tot["declarations"] += r["declarations"]
        tot["params"] += r["params"]
        tot["single_def_single_use"] += r["single_def_single_use"]
        if not r["thunk"]:
            tot["declarations_non_thunk"] += r["declarations"]
            tot["single_def_single_use_non_thunk"] += r["single_def_single_use"]
        for k, v in r["types"].items():
            types[k] += v
        for k, v in r["arrays"].items():
            arrays[int(k)] += v
            tot["array_declarations"] += v
    tot["blob16"] = arrays.get(16, 0)
    return {
        "totals": dict(sorted(tot.items())),
        "types": dict(types.most_common()),
        "arrays": {str(k): arrays[k] for k in sorted(arrays)},
    }


def json_surface(payload: dict) -> dict:
    """The scored surface's own counts, for contrast with the declared ones."""
    tot = collections.Counter()
    types: collections.Counter = collections.Counter()
    for fn in payload.get("functions") or []:
        for v in fn.get("variables") or []:
            tot["json_variables"] += 1
            tot["json_" + str(v.get("kind") or "?")] += 1
            types[str(v.get("type"))] += 1
            if not v.get("line_numbers") and not v.get("addresses"):
                tot["json_useless_slots"] += 1
                if v.get("kind") == "stack":
                    tot["json_useless_stack_slots"] += 1
    return {"totals": dict(sorted(tot.items())), "json_types": dict(types.most_common())}


def run_kuna(binary: Path, options: list[list[str]], mode: str | None,
             want_json: bool, timeout: int) -> tuple[str, dict | None]:
    kuna = paths.binary("kuna", "KUNA_BIN")
    base = [str(kuna), "decompile-all", str(binary)]
    if mode:
        base += ["--mode", mode]
    for k, v in options:
        base += ["--option", k, v]
    text = subprocess.run(base, capture_output=True, text=True, timeout=timeout)
    if not text.stdout.strip():
        sys.exit(f"kuna produced no output ({text.returncode}): {text.stderr[-400:]}")
    payload = None
    if want_json:
        js = subprocess.run(base + ["--json"], capture_output=True, text=True,
                            timeout=timeout)
        try:
            payload = json.loads(js.stdout)
        except json.JSONDecodeError:
            sys.exit(f"kuna --json produced no JSON: {js.stderr[-400:]}")
    return text.stdout, payload


def build(binary: Path | None, text: str, payload: dict | None,
          options: list[list[str]], mode: str | None) -> dict:
    rows = census_text(text)
    doc = {
        "schema": 1,
        "binary": str(binary) if binary else None,
        "options": options,
        "mode": mode,
        **roll_up(rows),
        "functions": rows,
    }
    if payload is not None:
        js = json_surface(payload)
        doc["totals"].update(js.pop("totals"))
        doc.update(js)
    return doc


def _fmt_delta(a, b) -> str:
    """`before -> after (delta)`, or `n/a` on whichever side did not count it.

    A `--json` run counts keys a text-only run does not, so a missing side is
    "not measured", never zero.
    """
    if a is None or b is None:
        return f"{'n/a' if a is None else a} -> {'n/a' if b is None else b}"
    return f"{a} -> {b} ({b - a:+d})"


def report(doc: dict, baseline: dict | None, top: int) -> str:
    t = doc["totals"]
    lines = [f"# varcensus {doc.get('binary') or '(text)'}"]
    if doc.get("options"):
        lines.append("options: " + " ".join(f"{k}={v}" for k, v in doc["options"]))
    lines.append("")
    keys = sorted(set(t) | set((baseline or {}).get("totals", {})))
    lines.append("| count | " + ("before -> after |" if baseline else "value |"))
    lines.append("|---|---|")
    for k in keys:
        if baseline:
            lines.append(f"| {k} | {_fmt_delta(baseline['totals'].get(k), t.get(k))} |")
        else:
            lines.append(f"| {k} | {t.get(k, 0)} |")
    lines += ["", "## declared types", ""]
    for name, n in list(doc["types"].items())[:top]:
        if baseline:
            lines.append(f"- {name}: {_fmt_delta(baseline['types'].get(name, 0), n)}")
        else:
            lines.append(f"- {name}: {n}")
    if doc.get("arrays"):
        lines += ["", "arrays: " + ", ".join(f"[{k}]x{v}" for k, v in doc["arrays"].items())]
    if baseline:
        before = {f["name"]: f for f in baseline.get("functions", [])}
        moved = []
        for f in doc.get("functions", []):
            b = before.get(f["name"])
            if b and b["declarations"] != f["declarations"]:
                moved.append((f["declarations"] - b["declarations"], f["name"],
                              b["declarations"], f["declarations"]))
        moved.sort(key=lambda m: (-abs(m[0]), m[1]))
        lines += ["", f"## functions whose declaration count moved ({len(moved)})", ""]
        for d, name, b, a in moved[:top]:
            lines.append(f"- {name}: {b} -> {a} ({d:+d})")
    return "\n".join(lines) + "\n"


SELFTEST_TEXT = """// Function: sub_1000 @ 0x1000
unsigned long sub_1000(int a0,char **a1)
{
  unsigned long v1;
  char *v2; // rax
  char v3 [16];
  int v4;
  v1 = sub_2000(a0);
  v2 = (char *)v1;
  v4 = 0;
  while (v4 < 3) {
    v4 = v4 + 1;
    v3[v4] = *v2;
  }
  return v4;
}

// Function: free @ 0x2000
void free(void *a0)
{
  (*dat_be48)(); // jump-as-call
}
"""


def selftest() -> int:
    rows = census_text(SELFTEST_TEXT)
    doc = build(None, SELFTEST_TEXT, None, [], None)
    t = doc["totals"]
    checks = [
        ("two functions", t["functions"] == 2),
        ("one thunk", t["thunks"] == 1),
        ("4 declarations", t["declarations"] == 4),
        ("4 non-thunk declarations", t["declarations_non_thunk"] == 4),
        ("2 params on sub_1000", rows[0]["params"] == 2),
        ("1 param on the thunk `free(void *a0)`", rows[1]["params"] == 1),
        ("one [16] blob", t["blob16"] == 1),
        # v1 and v2 are each written once and read once -> both foldable. v4 is
        # written twice, and v3 is only ever element-stored (a read of the base).
        ("v1 and v2 are the single-def/single-read pair",
         rows[0]["single_def_single_use_names"] == ["v1", "v2"]),
        ("type histogram keeps the pointer on the type side",
         doc["types"].get("char *") == 1 and doc["types"].get("char [16]") == 1),
        ("no unparsed declarations", rows[0]["unparsed"] == []),
        ("a `return;` is not a declaration",
         decl_lines(["  return;", "  goto LAB_1;", "  int v9;"]) == ["int v9;"]),
        ("param_count", [param_count("void f(void)"), param_count("int g(int a0,char **a1)"),
                         param_count("void h(int (*a0)(int,int),long a1)")] == [0, 2, 2]),
    ]
    bad = [name for name, ok in checks if not ok]
    for name, ok in checks:
        print(f"[{'ok ' if ok else 'FAIL'}] {name}")
    print(f"selftest: {len(checks) - len(bad)}/{len(checks)} passed")
    return 1 if bad else 0


def main(argv=None) -> int:
    ap = argparse.ArgumentParser(
        description=__doc__.splitlines()[0],
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="Writes nothing unless --out is given.")
    ap.add_argument("binary", nargs="?", type=Path, help="binary to decompile")
    ap.add_argument("--text", type=Path, help="parse a captured decompile-all dump instead")
    ap.add_argument("--json", action="store_true",
                    help="also count the scored JSON surface (a second kuna run)")
    ap.add_argument("--option", nargs=2, action="append", metavar=("NAME", "VALUE"),
                    default=[], help="kuna option for this run (repeatable)")
    ap.add_argument("--mode", help="kuna --mode for this run")
    ap.add_argument("--timeout", type=int, default=1800)
    ap.add_argument("--out", type=Path, help="write the full census JSON here")
    ap.add_argument("--baseline", type=Path, help="census JSON to diff against (the 'before')")
    ap.add_argument("--compare", type=Path,
                    help="census JSON to use as the 'after' instead of a fresh run")
    ap.add_argument("--top", type=int, default=20, help="rows per list in the report")
    ap.add_argument("--selftest", action="store_true", help="run the built-in checks")
    args = ap.parse_args(argv)

    if args.selftest:
        return selftest()

    baseline = json.loads(args.baseline.read_text()) if args.baseline else None
    if args.compare:
        doc = json.loads(args.compare.read_text())
    elif args.text:
        doc = build(args.text, args.text.read_text(), None, args.option, args.mode)
    elif args.binary:
        text, payload = run_kuna(args.binary, args.option, args.mode, args.json,
                                 args.timeout)
        doc = build(args.binary, text, payload, args.option, args.mode)
    else:
        ap.error("need a binary, --text, or --compare")

    if args.out:
        args.out.parent.mkdir(parents=True, exist_ok=True)
        args.out.write_text(json.dumps(doc, indent=1) + "\n")
    print(report(doc, baseline, args.top), end="")
    return 0


if __name__ == "__main__":
    sys.exit(main())
