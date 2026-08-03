#!/usr/bin/env python3
"""Mechanical audit of the itecondlist A/B diffs.

Every changed hunk must be one of:
  (a) a function-header comment line whose only delta is a ternary/iteboolean
      count going UP, or
  (b) a removed `if (C)\n  V = A;\nelse {\n  V = B;\n}` block replaced by exactly
      one added line `V = (C) ? A : B;` / `V = (C);` / `V = !(C);` with the SAME
      V and the SAME C and the SAME A/B.
Anything else is reported.
"""
import re
import subprocess
import sys

D = "/tmp/claude-1000/-home-mahaloz-github-kuna/e58fd9a6-240a-4f79-bc8d-6f268eb007bf/scratchpad/itecondlist/ab"

HDR = re.compile(r"^// Function: |^\S.*\)\s*(//.*)?$")


def hunks(off, on):
    out = subprocess.run(
        ["diff", "-U0", off, on], capture_output=True, text=True
    ).stdout.splitlines()
    cur = None
    for ln in out:
        if ln.startswith("@@"):
            if cur:
                yield cur
            cur = ([], [])
        elif cur is not None and ln.startswith("-"):
            cur[0].append(ln[1:])
        elif cur is not None and ln.startswith("+"):
            cur[1].append(ln[1:])
    if cur:
        yield cur


def norm(s):
    return re.sub(r"\s+", " ", s).strip()


def audit(tag):
    off, on = f"{D}/{tag}.off.c", f"{D}/{tag}.on.c"
    ok_hdr = ok_fold = 0
    problems = []
    for rem, add in hunks(off, on):
        r = [norm(x) for x in rem]
        a = [norm(x) for x in add]
        # (a) header-comment line: same prototype, more ternaries/re-rolls
        if len(r) == 1 and len(a) == 1 and ("// " in r[0] or "// " in a[0]):
            proto_r = r[0].split("//")[0].strip()
            proto_a = a[0].split("//")[0].strip()
            if proto_r == proto_a:
                nr = sum(int(m or 1) for m in re.findall(r"ternary(?: x(\d+))?", r[0]))
                na = sum(int(m or 1) for m in re.findall(r"ternary(?: x(\d+))?", a[0]))
                br = sum(int(m) for m in re.findall(r"re-rolled (\d+)", r[0]))
                ba = sum(int(m) for m in re.findall(r"re-rolled (\d+)", a[0]))
                if na >= nr and ba >= br and (na > nr or ba > br):
                    ok_hdr += 1
                    continue
            problems.append(("header", r, a))
            continue
        # (b) diamond fold: removed lines are the if/else block, added is 1 stmt
        if len(a) != 1:
            problems.append(("multi-add", r, a))
            continue
        stmt = a[0]
        m = re.match(r"^(.*?) = (.*);(\s*//.*)?$", stmt)
        if not m:
            problems.append(("not-an-assignment", r, a))
            continue
        dest, rhs = m.group(1), m.group(2)
        joined = " ".join(r)
        if not joined.startswith("if ("):
            problems.append(("not-an-if", r, a))
            continue
        # the removed block must assign the SAME destination in both arms and
        # contain no other statement
        arms = re.findall(re.escape(dest) + r" = ([^;]*);", joined)
        if len(arms) != 2:
            problems.append(("arm-count", r, a))
            continue
        # every removed line must be if/else/brace/one of the two arm assignments
        for ln in r:
            if re.match(r"^if \(", ln) or ln in ("else {", "}", "else"):
                continue
            if re.match(re.escape(dest) + r" = [^;]*;$", ln):
                continue
            problems.append(("stray-line", r, a))
            break
        else:
            # ternary: both arms must appear in the RHS; boolean: 0/1 arms
            tern = re.match(r"^\((.*)\) \? (.*) : (.*)$", rhs) or re.match(
                r"^(.*) \? (.*) : (.*)$", rhs
            )
            if tern:
                if norm(tern.group(2)) == arms[0] and norm(tern.group(3)) == arms[1]:
                    ok_fold += 1
                    continue
                problems.append(("arm-mismatch", r, a))
                continue
            if set(arms) == {"1", "0"}:
                neg = rhs.startswith("!")
                want_true = "0" if neg else "1"
                if arms[0] == want_true:
                    ok_fold += 1
                    continue
                problems.append(("bool-polarity", r, a))
                continue
            problems.append(("unrecognized-rhs", r, a))
    print(f"{tag}: header-only hunks={ok_hdr} folded diamonds={ok_fold} problems={len(problems)}")
    for p in problems[:12]:
        print("   PROBLEM", p[0])
        for x in p[1]:
            print("     -", x)
        for x in p[2]:
            print("     +", x)
    return len(problems)


bad = 0
for tag in sys.argv[1:]:
    bad += audit(tag)
print("TOTAL PROBLEMS:", bad)
