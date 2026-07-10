#!/usr/bin/env python3
"""comment_diff_check.py — verify that a Rust file's diff is comment-only.

For each file argument, compares `git show <rev>:<path>` (default rev: HEAD)
against the worktree copy after stripping ALL comments with a Rust-aware
state machine (line comments, nested block comments, string/char/raw-string
literals respected) and normalizing whitespace. If the stripped sources are
not byte-identical, the edit touched code and the check FAILS.

Also lints doc-comment code fences: the per-file ``` fence-line parity must
not change, and the count of bare ``` fences (potential live doctests) must
not increase.

Usage:  python3 tools/comment_diff_check.py [--against REV] file.rs [...]
Exit:   0 = all files comment-only;  1 = violation;  2 = usage/IO error.
"""

import subprocess
import sys
import re
import os


def strip_comments(src: str) -> str:
    """Remove all comments from Rust source; keep everything else."""
    out = []
    i, n = 0, len(src)
    while i < n:
        c = src[i]
        nxt = src[i + 1] if i + 1 < n else ""
        # line comment (//, ///, //!)
        if c == "/" and nxt == "/":
            j = src.find("\n", i)
            i = n if j == -1 else j  # keep the newline itself
            continue
        # block comment, nested
        if c == "/" and nxt == "*":
            depth = 1
            i += 2
            while i < n and depth:
                if src.startswith("/*", i):
                    depth += 1
                    i += 2
                elif src.startswith("*/", i):
                    depth -= 1
                    i += 2
                else:
                    i += 1
            continue
        # raw string: r"..." / r#"..."# / br##"..."## (b before r)
        m = re.match(r'b?r(#*)"', src[i:])
        if m and (c == "r" or (c == "b" and nxt == "r")):
            hashes = m.group(1)
            close = '"' + hashes
            j = src.find(close, i + m.end())
            if j == -1:
                out.append(src[i:])
                break
            out.append(src[i : j + len(close)])
            i = j + len(close)
            continue
        # normal / byte string
        if c == '"' or (c == "b" and nxt == '"'):
            start = i
            i += 2 if c == "b" else 1
            while i < n:
                if src[i] == "\\":
                    i += 2
                elif src[i] == '"':
                    i += 1
                    break
                else:
                    i += 1
            out.append(src[start:i])
            continue
        # char literal vs lifetime: only consume 'x' / '\...' shapes
        if c == "'":
            m = re.match(r"'(\\.[^']*|[^\\'])'", src[i:])
            if m:
                out.append(src[i : i + m.end()])
                i += m.end()
                continue
            out.append(c)  # lifetime tick
            i += 1
            continue
        out.append(c)
        i += 1
    return re.sub(r"\s+", " ", "".join(out)).strip()


FENCE_RE = re.compile(r"^\s*(?://[/!]?\s*)?(```\S*)\s*$")


def fence_stats(src: str):
    total, bare = 0, 0
    for line in src.splitlines():
        m = FENCE_RE.match(line)
        if m:
            total += 1
            if m.group(1) == "```":
                bare += 1
    return total, bare


def git_show(rev: str, relpath: str):
    r = subprocess.run(
        ["git", "show", f"{rev}:{relpath}"], capture_output=True, text=False
    )
    return r.stdout.decode("utf-8", "replace") if r.returncode == 0 else None


def main() -> int:
    args = sys.argv[1:]
    rev = "HEAD"
    if args and args[0] == "--against":
        if len(args) < 3:
            print("usage: comment_diff_check.py [--against REV] file.rs [...]")
            return 2
        rev, args = args[1], args[2:]
    if not args:
        print("usage: comment_diff_check.py [--against REV] file.rs [...]")
        return 2

    root = subprocess.run(
        ["git", "rev-parse", "--show-toplevel"], capture_output=True, text=True
    ).stdout.strip()
    if not root:
        print("error: not inside a git repository")
        return 2

    failures = 0
    for path in args:
        ap = os.path.abspath(path)
        rel = os.path.relpath(ap, root)
        old = git_show(rev, rel)
        if old is None:
            print(f"SKIP {rel}: not in {rev} (new file — not a comment-only edit)")
            failures += 1
            continue
        try:
            with open(ap, encoding="utf-8") as f:
                new = f.read()
        except OSError as e:
            print(f"ERROR {rel}: {e}")
            return 2

        so, sn = strip_comments(old), strip_comments(new)
        if so != sn:
            k = next(
                (i for i, (a, b) in enumerate(zip(so, sn)) if a != b),
                min(len(so), len(sn)),
            )
            print(f"FAIL {rel}: non-comment change near stripped offset {k}:")
            print(f"  old: ...{so[max(0, k - 40) : k + 40]!r}...")
            print(f"  new: ...{sn[max(0, k - 40) : k + 40]!r}...")
            failures += 1
            continue

        (to, bo), (tn, bn) = fence_stats(old), fence_stats(new)
        if tn % 2 != to % 2:
            print(f"FAIL {rel}: code-fence parity changed ({to} -> {tn} fence lines)")
            failures += 1
            continue
        if bn > bo:
            print(f"FAIL {rel}: bare ``` fences increased ({bo} -> {bn}) — doctest risk")
            failures += 1
            continue
        print(f"OK   {rel}")

    if failures:
        print(f"\n{failures} file(s) failed the comment-only check.")
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
