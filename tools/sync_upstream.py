#!/usr/bin/env python3
"""Port upstream Ghidra decompiler changes into kuna.

kuna vendors a byte-identical copy of Ghidra's C++ decompiler and its SLEIGH
processor specs (see UPSTREAM.md). This script diffs a Ghidra checkout between the
recorded ``GHIDRA_REV`` and a target revision, restricted to the vendored upstream
paths, rewrites the path prefixes to kuna's layout, and applies the patch with
``git apply`` (plain apply first, ``--3way`` fallback). On success it rewrites
``GHIDRA_REV`` in UPSTREAM.md.

The upstream diff is taken with ``--no-renames`` so every change arrives as a plain
add/delete/modify of a single path -- a rename can then never straddle the vendored
boundary. Anything unusual in the patch (quoted/unparseable headers, rename/copy
lines, binary changes) is a hard error, never a silent guess.

Usage:
    python3 tools/sync_upstream.py --ghidra ~/github/ghidra --to origin/master [--from REV]
                                   [--dry-run] [--root .]

The script is a thin, auditable wrapper around `git diff | rewrite | git apply`.
"""
import argparse
import os
import re
import subprocess
import sys

# Vendored path map: upstream prefix -> kuna prefix. Order matters (longest first).
PATH_MAP = [
    ("Ghidra/Features/Decompiler/src/decompile/cpp/", "decompiler/cpp/"),
    ("Ghidra/Features/Decompiler/src/decompile/unittests/", "decompiler/unittests/"),
    ("Ghidra/Features/Decompiler/src/decompile/datatests/", "decompiler/datatests/"),
    # Processors are matched generically below (per-module <P>).
]
# Generic processor-spec mapping handled via regex (any module P).
PROC_UP = "Ghidra/Processors/"
PROC_SUFFIX = "/data/languages/"

# Upstream subtrees we deliberately do NOT vendor.
EXCLUDE = ["Ghidra/Features/Decompiler/src/decompile/zlib/"]

UPSTREAM_PREFIXES = [
    "Ghidra/Features/Decompiler/src/decompile/cpp",
    "Ghidra/Features/Decompiler/src/decompile/unittests",
    "Ghidra/Features/Decompiler/src/decompile/datatests",
    "Ghidra/Processors",
]


def sh(args, cwd=None, check=True):
    return subprocess.run(args, cwd=cwd, check=check, capture_output=True, text=True)


def git(ghidra, *args, check=True):
    return sh(["git", "-C", ghidra, *args], check=check)


def read_ghidra_rev(upstream_md):
    with open(upstream_md) as fh:
        for line in fh:
            m = re.search(r"GHIDRA_REV=([0-9a-fA-F]+)", line)
            if m:
                return m.group(1)
    return None


def write_ghidra_rev(upstream_md, newrev):
    with open(upstream_md) as fh:
        text = fh.read()
    text2, nsubs = re.subn(r"GHIDRA_REV=[0-9a-fA-F]+", "GHIDRA_REV=" + newrev, text, count=1)
    if nsubs != 1:
        raise RuntimeError(
            "no GHIDRA_REV=<hex> line found in %s -- record %s there manually"
            % (upstream_md, newrev))
    with open(upstream_md, "w") as fh:
        fh.write(text2)


def map_path(up):
    """Map an upstream repo path to its kuna path, or None if not vendored/excluded."""
    for ex in EXCLUDE:
        if up.startswith(ex):
            return None
    for src, dst in PATH_MAP:
        if up.startswith(src):
            return dst + up[len(src):]
    # Generic processor spec: Ghidra/Processors/<P>/data/languages/...
    if up.startswith(PROC_UP):
        rest = up[len(PROC_UP):]
        slash = rest.find("/")
        if slash != -1:
            module = rest[:slash]
            tail = rest[slash:]
            if tail.startswith(PROC_SUFFIX):
                return "specs/" + PROC_UP + module + tail
    return None


class PatchRewriteError(Exception):
    """The upstream diff contains something we refuse to rewrite blindly."""


# Diff header forms git emits, with the path captured for rewriting.
_DIFF_GIT = re.compile(r"^diff --git a/(?P<a>.+?) b/(?P<b>.+)$")
_OLDNEW_PATTERNS = [
    re.compile(r"^(--- )a/(?P<p>.+)$"),
    re.compile(r"^(\+\+\+ )b/(?P<p>.+)$"),
]
# Lines that must never appear in a --no-renames, text-only diff. If one shows up,
# rewriting it blindly could plant files outside the kuna layout -- hard-fail instead.
_FORBIDDEN_MARKERS = (
    "rename from ", "rename to ", "copy from ", "copy to ",
    "similarity index ", "Binary files ", "GIT binary patch",
)


def rewrite_patch(diff_text):
    """Rewrite a/b paths from upstream layout to kuna layout.

    Returns (rewritten_text, skipped_paths). File sections whose path is not
    vendored are dropped wholesale (the diff is split into per-file sections on
    'diff --git'). The input diff MUST be produced with ``--no-renames`` and
    ``core.quotepath=false``: every section then names the SAME path on both sides
    of 'diff --git' and adds/deletes carry full content. Raises PatchRewriteError
    on anything that violates those assumptions (quoted/unparseable headers,
    rename/copy/binary markers) rather than guessing.
    """
    sections = []
    current = []
    for line in diff_text.splitlines(keepends=True):
        if line.startswith("diff --git ") and current:
            sections.append(current)
            current = []
        current.append(line)
    if current:
        sections.append(current)

    out = []
    skipped = []
    for sec in sections:
        head = sec[0].rstrip("\n")
        if not head.startswith("diff --git "):
            raise PatchRewriteError("unexpected patch preamble: %r" % head[:200])
        m = _DIFF_GIT.match(head)
        if m is None:
            raise PatchRewriteError(
                "unparseable diff header (quoted or unusual path?): %r" % head[:200])
        a_up, b_up = m.group("a"), m.group("b")
        if a_up != b_up:
            raise PatchRewriteError(
                "diff header names two different paths (rename/copy leak, or a path "
                "containing ' b/'?); the sync diff must use --no-renames: %r"
                % head[:200])
        mapped = map_path(a_up)
        if mapped is None:
            skipped.append(a_up)
            continue
        out.append("diff --git a/%s b/%s\n" % (mapped, mapped))
        in_header = True  # header/marker lines only occur before the first @@ hunk
        for line in sec[1:]:
            s = line.rstrip("\n")
            nl = "\n" if line.endswith("\n") else ""
            if in_header and s.startswith("@@ "):
                in_header = False
            if not in_header:
                # Hunk content: pass through VERBATIM. A deleted source line that
                # happens to read like a '--- a/...' header must not be rewritten.
                out.append(s + nl)
                continue
            for marker in _FORBIDDEN_MARKERS:
                if s.startswith(marker):
                    raise PatchRewriteError(
                        "unsupported %r line for %s (binary change, or rename/copy "
                        "leaked through?) -- vendor this change manually"
                        % (marker.strip(), a_up))
            new = None
            for pat in _OLDNEW_PATTERNS:
                pm = pat.match(s)
                # Only rewrite the header line that names THIS section's path
                # ('--- /dev/null' for adds/deletes doesn't match and stays as-is).
                if pm:
                    if pm.group("p") == a_up:
                        new = s[: pm.start("p")] + mapped
                    break
            out.append((new if new is not None else s) + nl)
    return "".join(out), skipped


def analyze(diff_text):
    """Report added/deleted VENDORED files and risky changes for the maintainer.

    Paths in the result are kuna paths; non-vendored files (which rewrite_patch
    drops) are excluded so the report matches what will actually be applied.
    """
    added, deleted, changed_y, changed_cc = [], [], set(), set()
    new_datatests = []
    cur_k = None
    for line in diff_text.splitlines():
        m = _DIFF_GIT.match(line)
        if m:
            cur_k = map_path(m.group("b"))
            if cur_k is not None:
                if cur_k.endswith((".y", ".l")):
                    changed_y.add(cur_k)
                if cur_k.endswith(".cc"):
                    changed_cc.add(cur_k)
            continue
        if cur_k is None:
            continue
        if line.startswith("new file mode"):
            added.append(cur_k)
            if "/datatests/" in cur_k and cur_k.endswith(".xml"):
                new_datatests.append(cur_k)
        elif line.startswith("deleted file mode"):
            deleted.append(cur_k)
    # .y/.l changed without its regenerated .cc in the same diff?
    y_without_cc = []
    for y in changed_y:
        stem = y[:-2]  # strip .y / .l (both two chars)
        if stem + ".cc" not in changed_cc:
            y_without_cc.append(y)
    return {
        "added": added,
        "deleted": deleted,
        "y_without_cc": y_without_cc,
        "new_datatests": new_datatests,
    }


def vendored_processor_modules(root):
    base = os.path.join(root, "specs", "Ghidra", "Processors")
    if not os.path.isdir(base):
        return set()
    return set(os.listdir(base))


def main(argv=None):
    p = argparse.ArgumentParser(description=__doc__,
                                formatter_class=argparse.RawDescriptionHelpFormatter)
    p.add_argument("--ghidra", required=True, help="path to a ghidra git checkout")
    p.add_argument("--to", default="origin/master", help="target revision (default origin/master)")
    p.add_argument("--from", dest="from_rev", default=None,
                   help="base revision (default: GHIDRA_REV from UPSTREAM.md)")
    p.add_argument("--root", default=".", help="kuna repo root (default: cwd)")
    p.add_argument("--dry-run", action="store_true", help="report and `git apply --check` only")
    args = p.parse_args(argv)

    root = os.path.abspath(args.root)
    upstream_md = os.path.join(root, "UPSTREAM.md")
    ghidra = os.path.abspath(args.ghidra)

    if not os.path.isdir(os.path.join(ghidra, ".git")):
        print("error: --ghidra is not a git checkout: %s" % ghidra, file=sys.stderr)
        return 2

    from_rev = args.from_rev or read_ghidra_rev(upstream_md)
    if not from_rev:
        print("error: no --from given and GHIDRA_REV not found in UPSTREAM.md", file=sys.stderr)
        return 2

    for rev in (from_rev, args.to):
        if git(ghidra, "cat-file", "-e", rev + "^{commit}", check=False).returncode != 0:
            print("error: revision not found in ghidra checkout: %s "
                  "(try `git -C %s fetch`)" % (rev, ghidra), file=sys.stderr)
            return 2

    to_full = git(ghidra, "rev-parse", args.to).stdout.strip()
    from_full = git(ghidra, "rev-parse", from_rev).stdout.strip()
    if from_full == to_full:
        print("Already at %s; nothing to sync." % to_full)
        return 0

    # --no-renames: renames arrive as plain delete+add pairs, so a rename can never
    # straddle the vendored boundary. quotepath=false: non-ASCII paths arrive
    # unquoted (anything git still quotes is rejected by rewrite_patch).
    diff = git(ghidra, "-c", "core.quotepath=false", "diff", "--no-renames",
               "%s..%s" % (from_rev, args.to), "--", *UPSTREAM_PREFIXES).stdout
    if not diff.strip():
        print("No changes to vendored paths between %s and %s." % (from_rev, args.to))
        if args.dry_run:
            print("Dry run; GHIDRA_REV would be updated -> %s" % to_full)
            return 0
        try:
            write_ghidra_rev(upstream_md, to_full)
        except RuntimeError as e:
            print("error: %s" % e, file=sys.stderr)
            return 1
        print("Updated GHIDRA_REV -> %s" % to_full)
        return 0

    try:
        rewritten, skipped = rewrite_patch(diff)
    except PatchRewriteError as e:
        print("error: %s" % e, file=sys.stderr)
        print("Nothing was applied. Handle this sync manually (see UPSTREAM.md).",
              file=sys.stderr)
        return 1
    if not rewritten.strip():
        # Diff was non-empty but touched only non-vendored files (e.g. processor
        # docs outside data/languages) -- same outcome as an empty diff.
        print("No changes to vendored paths between %s and %s "
              "(%d non-vendored file(s) skipped)." % (from_rev, args.to, len(skipped)))
        if args.dry_run:
            print("Dry run; GHIDRA_REV would be updated -> %s" % to_full)
            return 0
        try:
            write_ghidra_rev(upstream_md, to_full)
        except RuntimeError as e:
            print("error: %s" % e, file=sys.stderr)
            return 1
        print("Updated GHIDRA_REV -> %s" % to_full)
        return 0
    info = analyze(diff)

    print("Sync %s..%s (vendored paths only)" % (from_full[:12], to_full[:12]))
    print("  added:   %d   deleted: %d" % (len(info["added"]), len(info["deleted"])))
    if skipped:
        print("  skipped (not vendored, e.g. zlib): %d files" % len(skipped))
    for f in info["deleted"]:
        print("  ! deleted: %s (changes upstream Makefile $(wildcard *.cc); rebuild after)" % f)
    for y in info["y_without_cc"]:
        print("  ! grammar %s changed without its generated .cc -> bison/flex needed locally" % y)

    vendored = vendored_processor_modules(root)
    for dt in info["new_datatests"]:
        # crude: warn that a new datatest may need a processor we don't vendor
        print("  + new datatest: %s (verify its <binaryimage arch> module is vendored)" % dt)
    if info["new_datatests"] and vendored:
        print("    (vendored processor modules: %s)" % ", ".join(sorted(vendored)))

    patch_path = os.path.join(root, ".kuna_sync.patch")
    with open(patch_path, "w") as fh:
        fh.write(rewritten)

    check = sh(["git", "-C", root, "apply", "-p1", "--check", patch_path], check=False)
    if check.returncode != 0:
        print("\n`git apply --check` FAILED:\n%s" % check.stderr, file=sys.stderr)
        print("Patch left at %s for inspection." % patch_path, file=sys.stderr)
        return 1
    print("\n`git apply --check`: clean.")

    if args.dry_run:
        print("Dry run; patch left at %s (not applied)." % patch_path)
        return 0

    # Primary: plain apply to the working tree (the patch already passed --check).
    # This is the expected path on a clean kuna checkout (vendored files unmodified).
    apply = sh(["git", "-C", root, "apply", "-p1", patch_path], check=False)
    if apply.returncode != 0:
        # Fallback: 3-way content merge against the recorded base (needs the file to
        # match the git index). Only reachable if vendored files were locally edited.
        merge = sh(["git", "-C", root, "apply", "-p1", "--3way", patch_path], check=False)
        if merge.returncode != 0:
            print("apply failed:\n%s\n%s" % (apply.stderr, merge.stderr), file=sys.stderr)
            print("This usually means a vendored file was edited locally (violating the "
                  "no-edits policy). Resolve manually; patch left at %s." % patch_path,
                  file=sys.stderr)
            return 1

    os.unlink(patch_path)
    try:
        write_ghidra_rev(upstream_md, to_full)
    except RuntimeError as e:
        print("error: patch APPLIED, but GHIDRA_REV was not recorded: %s" % e,
              file=sys.stderr)
        return 1
    print("Applied. Updated GHIDRA_REV -> %s" % to_full)
    print("Next: `make test` and `python -m kuna.run_tests --baseline docs/baseline.json`, "
          "then review `git diff` and commit.")
    return 0


if __name__ == "__main__":
    sys.exit(main())
