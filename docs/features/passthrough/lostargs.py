"""Find calls and functions that lose arguments between two `decompile-all --json` runs.

    python3 lostargs.py DIR        # DIR holds <tag>.off.json / <tag>.on.json pairs

For every function present in both runs it lists the direct calls in the C text
(`name(args)`, callee by name, arity by top-level commas) and reports

  CALL_LOST_ARGS  a call site with fewer arguments in the second run, against a
                  callee whose own recovered arity is the same in both runs
                  (or that is not in the output, e.g. an import);
  PARAM_LOST      a function whose own parameter list got shorter;
  CALLSITES_MOVED a callee called a different number of times (printed for
                  reading, not counted as a loss).

Exit status is 1 when any CALL_LOST_ARGS or PARAM_LOST row is found.
"""
import glob
import json
import os
import re
import sys

KEYWORDS = {"if", "while", "for", "switch", "return", "sizeof", "do", "else", "case"}
CALL = re.compile(r"\b([A-Za-z_][A-Za-z0-9_]*)\s*\(")


def split_args(text, start):
    """Count top-level arguments of the call whose '(' is at text[start]."""
    depth, n, seen = 0, 1, False
    for c in text[start:]:
        if c == "(":
            depth += 1
            if depth == 1:
                continue
        elif c == ")":
            depth -= 1
            if depth == 0:
                return n if seen else 0
        elif c == "," and depth == 1:
            n += 1
        if depth >= 1 and not c.isspace():
            seen = True
    return None


def header_arity(code):
    head = (code or "").split("{", 1)[0]
    m = re.search(r"\(", head)
    if not m:
        return None
    n = split_args(head, m.start())
    return n


def calls(code):
    body = (code or "").split("{", 1)[1] if "{" in (code or "") else ""
    body = re.sub(r"//[^\n]*", "", body)
    body = re.sub(r'"(\\.|[^"\\])*"', '""', body)
    out = {}
    for m in CALL.finditer(body):
        name = m.group(1)
        if name in KEYWORDS:
            continue
        n = split_args(body, m.end() - 1)
        if n is not None:
            out.setdefault(name, []).append(n)
    return out


def load(path):
    try:
        return {f["address"]: f for f in json.load(open(path))["functions"]}
    except Exception:
        return None


def names(funcs):
    out = {}
    for f in funcs.values():
        a = header_arity(f.get("code"))
        if f.get("name") and a is not None:
            out[f["name"]] = a
    return out


def main(d):
    lost, plost, moved = [], [], []
    for off in sorted(glob.glob(os.path.join(d, "*.off.json"))):
        tag = os.path.basename(off)[: -len(".off.json")]
        on = off[: -len("off.json")] + "on.json"
        A, B = load(off), load(on) if os.path.exists(on) else None
        if not A or not B:
            continue
        an, bn = names(A), names(B)
        for addr, fa in A.items():
            fb = B.get(addr)
            if not fb or fa.get("code") == fb.get("code"):
                continue
            pa, pb = header_arity(fa.get("code")), header_arity(fb.get("code"))
            if pa is not None and pb is not None and pb < pa:
                plost.append((tag, hex(addr), pa, pb))
            ca, cb = calls(fa.get("code")), calls(fb.get("code"))
            for callee, sites in ca.items():
                other = cb.get(callee)
                if other is None:
                    continue
                if len(other) != len(sites):
                    moved.append((tag, hex(addr), callee, sites, other))
                    continue
                if an.get(callee) != bn.get(callee):
                    continue
                for k, (x, y) in enumerate(zip(sites, other)):
                    if y < x:
                        lost.append((tag, hex(addr), callee, k, x, y))
    for r in lost:
        print("CALL_LOST_ARGS %s %s %s site#%d %d->%d" % r)
    for r in plost:
        print("PARAM_LOST %s %s %d->%d" % r)
    for r in moved:
        print("CALLSITES_MOVED %s %s %s %s -> %s" % r)
    print("SUMMARY call_lost_args=%d param_lost=%d callsites_moved=%d" % (len(lost), len(plost), len(moved)))
    return 1 if lost or plost else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1]))
