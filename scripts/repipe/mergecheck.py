"""The three merge guards, one per failure shape this repo has actually produced.

docs/decbench-loop.md, "Shared counters in a busy queue -- the merge is not the hard part":
round 2 ran up to four output-changing PRs concurrently over the same handful of hard-coded
counters and produced three distinct failure shapes, **only one of which announced itself**.
This module is one guard per shape, run after the rebase and before the suite:

  shape A -- LOUD conflict. The DIV number for #257 raced 55 -> 56 -> 57 -> 58 as #252/#253/
    #254 claimed each in turn. git stops you, so the danger is not missing it: it is
    renumbering the registry row and forgetting the other references. DIV numbers are claimed
    at MERGE, not on the branch, so `check_div` re-derives the next free number and lists
    every place each claimed number is referenced -- the history row, the option's `use_when`
    prose, the spec chapter, the PR body.

  shape B -- SILENT identical-edit auto-merge. `catalog_bytecompat.rs` kept 86 because BOTH
    sides had made the identical `85 -> 86` edit; git merged cleanly, there was no conflict
    and no diff to review, and the answer was 87 (#254). The only guard that works is to
    never trust the merged value: `assert_rederived` re-derives every counter from a fresh
    capture on the rebased tree (scripts/repipe/counters.py) and asserts against the file.

  shape C -- SILENT keep-both auto-merge. A `docs/baseline-stages.json` auto-merge left a
    stale `data_footer: 375` against 381 real keys (#253), and five rounds of keep-both
    resolution duplicated a row in `tests/stages/README.md`. Keep-both is git's safe default
    and it is exactly wrong for a counter or a table. `assert_keepboth` diffs against
    `origin/main` and asserts that nothing was REMOVED and nothing was ADDED TWICE.

And one absolute: **`docs/baseline.json` is the upstream baseline and must never be
re-pinned.** `tests/stages/README.md` says so, the stage corpus exists in its own directory
so that it is never touched, and `assert_baseline_untouched` is a hard reject.

Nothing here is caught by reviewing the *conflict*; it is caught by the build and the suite,
so run this AFTER the rebase, not before it.

CLI:
    python -m scripts.repipe.mergecheck --against origin/main
    python -m scripts.repipe.mergecheck --json
    python -m scripts.repipe.mergecheck --self-test      # three synthetic git histories
"""
from __future__ import annotations

import argparse
import collections
import json
import os
import re
import shutil
import subprocess
import sys
import tempfile
from dataclasses import asdict, dataclass
from pathlib import Path

from . import config, counters
from .counters import line_of as _line_of

REL_BASELINE = "docs/baseline.json"
REL_BASELINE_STAGES = "docs/baseline-stages.json"
REL_HISTORY = "docs/history.md"

# The files a keep-both resolution silently corrupts: counters, tables and generated lists.
KEEPBOTH_PATHS = (
    REL_BASELINE_STAGES,
    "tests/stages/README.md",
    "docs/history.md",
    "docs/options.md",
    "decompiler/crates/kuna-decomp/phases.toml",
)

REJECT, WARN, INFO = "reject", "warn", "info"


@dataclass
class Finding:
    """One merge hazard. ``severity`` REJECT means do not merge until it is resolved."""
    shape: str
    check: str
    severity: str
    id: str
    path: str
    line: int
    message: str
    detail: str = ""

    def as_dict(self) -> dict:
        return asdict(self)

    def oneline(self) -> str:
        where = "{}:{}".format(self.path, self.line) if self.path else "-"
        tail = "\n         " + self.detail.replace("\n", "\n         ") if self.detail else ""
        return "[{}] shape-{} {:<22} {}  {}{}".format(
            self.severity.upper(), self.shape, self.check, where, self.message, tail)


# --- git --------------------------------------------------------------------

def _git(repo, *args):
    proc = subprocess.run(["git", "-C", str(repo)] + list(args),
                          stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    return (proc.returncode,
            proc.stdout.decode("utf-8", "replace"),
            proc.stderr.decode("utf-8", "replace"))


def _root(repo=None) -> Path:
    return Path(repo) if repo else config.repo_root()


def have_ref(base_ref, repo=None) -> bool:
    return _git(_root(repo), "rev-parse", "--verify", "--quiet", base_ref + "^{commit}")[0] == 0


def show(base_ref, rel, repo=None):
    """The base_ref content of ``rel``, or None if the ref has no such file."""
    rc, out, _ = _git(_root(repo), "show", "{}:{}".format(base_ref, rel))
    return out if rc == 0 else None


# --- shape A: the DIV number ------------------------------------------------

_DIV_ROW_RX = re.compile(r"^\|\s*DIV-(\d+)\s*\|", re.M)


def div_rows(text):
    """Every DIV number claimed by a registry row in ``docs/history.md``, in file order."""
    return [int(m.group(1)) for m in _DIV_ROW_RX.finditer(text or "")]


def next_free_div(text) -> int:
    rows = div_rows(text)
    return (max(rows) + 1) if rows else 1


def div_references(num, repo=None, cap=40, width=150):
    """Every tracked file:line mentioning DIV-<num>, so a renumber can rewrite all of them.

    Trimmed hard: a single `docs/history.md` registry row is multiple KB of prose, and a
    guard whose output nobody reads is not a guard. The file:line prefix is what the merger
    acts on; the text after it is only there to recognise the hit.
    """
    rc, out, _ = _git(_root(repo), "grep", "-nE", r"DIV-{}([^0-9]|$)".format(num))
    if rc not in (0, 1):
        return None
    hits = [ln for ln in out.splitlines() if ln.strip()]
    trimmed = [(ln[:width] + " ...") if len(ln) > width else ln for ln in hits[:cap]]
    if len(hits) > cap:
        trimmed.append("... and {} more references".format(len(hits) - cap))
    return trimmed


def check_div(base_ref="origin/main", repo=None):
    """Shape A. Re-derive the next free DIV and surface every reference to a claimed one.

    The number is claimed at merge, not on the branch, so this reports rather than rewrites:
    it prints the number the merger must use, and the exact set of lines that must change
    with it -- the registry row, the option's `use_when` prose, the spec chapter, the code
    comments. A number a sibling already merged is a hard reject; renumbering only the row
    and leaving the references behind is how the 55 -> 58 race stayed broken.
    """
    root = _root(repo)
    head_path = root / REL_HISTORY
    if not head_path.exists():
        return [Finding("A", "div-number", WARN, "history-missing", REL_HISTORY, 0,
                        "docs/history.md not found; the DIV guard cannot run")]
    head_text = head_path.read_text(encoding="utf-8", errors="replace")
    head_rows = div_rows(head_text)
    next_free = next_free_div(head_text)
    out = [Finding("A", "div-number", INFO, "next-free-div", REL_HISTORY, 0,
                   "next free DIV is DIV-{} ({} rows claimed, highest DIV-{})".format(
                       next_free, len(head_rows), max(head_rows or [0])),
                   "DIV numbers are claimed at MERGE, not on the branch. Re-check this "
                   "number after the rebase; every number this branch claims is listed "
                   "below with the references a renumber must rewrite.")]

    def rows_at(n):
        return [_line_of(head_text, m.start()) for m in _DIV_ROW_RX.finditer(head_text)
                if int(m.group(1)) == n]

    def renumber_detail(n):
        refs = div_references(n, repo)
        head = "renumber DIV-{} to DIV-{} here and in every reference:".format(n, next_free)
        return head + "\n" + ("\n".join(refs) if refs else "(no references found)")

    have_base = have_ref(base_ref, repo)
    base_text = show(base_ref, REL_HISTORY, repo) if have_base else None
    if base_text is None:
        out.append(Finding("A", "div-number", WARN, "base-ref-missing", REL_HISTORY, 0,
                           "no {} to diff against; claimed-number check skipped".format(base_ref)))
        base_rows = head_rows
    else:
        base_rows = div_rows(base_text)

    base_count = collections.Counter(base_rows)
    head_count = collections.Counter(head_rows)
    collided = set()

    if base_text is not None:
        for n in sorted(head_count):
            if head_count[n] > base_count.get(n, 0) and base_count.get(n, 0) > 0:
                collided.add(n)
                out.append(Finding(
                    "A", "div-number", REJECT, "div-collision", REL_HISTORY, rows_at(n)[-1],
                    "a sibling already merged DIV-{} on {}; this branch claims it again "
                    "(rows at lines {})".format(n, base_ref,
                                                ", ".join(str(x) for x in rows_at(n))),
                    renumber_detail(n)))
        for n in sorted(set(head_count) - set(base_count)):
            out.append(Finding(
                "A", "div-number", INFO, "div-claimed-by-branch", REL_HISTORY, rows_at(n)[0],
                "this branch claims DIV-{}; the next free number on {} is DIV-{}".format(
                    n, base_ref, next_free_div(base_text)),
                renumber_detail(n) if n != next_free_div(base_text)
                else "the number is still free; confirm it after the rebase, then check "
                     "every reference:\n" + "\n".join(div_references(n, repo) or [])))

    for n in sorted(n for n, c in head_count.items() if c > 1 and n not in collided):
        out.append(Finding("A", "div-number", WARN, "div-claimed-twice", REL_HISTORY,
                           rows_at(n)[-1],
                           "DIV-{} has {} registry rows (lines {})".format(
                               n, head_count[n], ", ".join(str(x) for x in rows_at(n))),
                           "already on {} the same way, so this is history to reconcile "
                           "rather than a merge blocker; do not reuse either number".format(
                               base_ref)))
    return out


# --- shape B: re-derive, never trust the merged value ------------------------

def assert_rederived(repo=None, kuna=None):
    """Shape B. Every shared counter is re-derived on the rebased tree and asserted.

    A clean auto-merge of two identical `85 -> 86` edits leaves a file that no conflict, no
    diff and no reviewer will flag. Only a measurement catches it, so this ignores the merged
    value entirely and compares against `counters.derive()`.
    """
    try:
        derived = counters.derive(repo=repo, kuna=kuna)
    except counters.DeriveError as exc:
        return [Finding("B", "rederive", REJECT, "derive-failed", "", 0,
                        "could not re-derive the counters: {}".format(exc),
                        "a counter that cannot be measured cannot be merged")]
    out = []
    for d in counters.check(derived=derived, repo=repo):
        out.append(Finding(
            "B", "rederive", REJECT, d.site,
            os.path.relpath(d.path, derived["repo"]) if os.path.isabs(d.path) else d.path,
            d.line,
            "{} says {}, the tree says {}".format(d.counter, d.found, d.expected),
            d.detail or ("run `python -m scripts.repipe.counters --fix`" if d.fixable
                         else "needs an operator, not a rewrite")))
    if not out:
        out.append(Finding("B", "rederive", INFO, "counters-agree", "", 0,
                           "all shared counters re-derived and agree: {} settables, "
                           "tiers ({}, {}, {}), {} corpus files, next ElementId {}".format(
                               derived["settables"], *derived["tier_tuple"],
                               derived["corpus_files"], derived["next_element_id"])))
    return out


# --- shape C: keep-both removed nothing and added nothing twice --------------

def _significant(line: str) -> bool:
    """Lines worth diffing as a multiset: real content, not JSON/markdown scaffolding."""
    s = line.strip()
    return len(s) >= 8 and any(c.isalnum() for c in s)


def _norm(line: str) -> str:
    """A separator is not content: the last element of a JSON list gains a `,` when the list
    grows, and counting that as a removal-plus-addition buries the real findings in noise."""
    return line.strip().rstrip(",")


def _footer_findings(path, rel, text, check):
    """A `[passing, total]` footer must equal the key count it summarises.

    This is the #253 shape exactly: keep-both merged both sides' new keys and left the
    footer at 375 while the file carried 381. The footer is not a merge artifact anyone
    reviews, so it is asserted against the thing it counts.
    """
    try:
        data = json.loads(text)
    except ValueError:
        return []
    if not isinstance(data, dict) or "data_footer" not in data or "passing" not in data:
        return []
    footer, passing = data["data_footer"], data["passing"]
    if not isinstance(passing, list) or not isinstance(footer, list) or not footer:
        return []
    out = []
    if footer[0] != len(passing):
        out.append(Finding("C", check, REJECT, "stale-footer", rel, 0,
                           "data_footer says {} passing, the file carries {} keys".format(
                               footer[0], len(passing)),
                           "re-RECORD it, never merge it:\n"
                           "  kuna test --datatests --datatests-dir tests/stages "
                           "--save-baseline {}".format(rel)))
    if len(footer) > 1 and footer[1] < footer[0]:
        out.append(Finding("C", check, REJECT, "impossible-footer", rel, 0,
                           "data_footer {} has more passing than total".format(footer)))
    dupes = [k for k, c in collections.Counter(
        x for x in passing if isinstance(x, str)).items() if c > 1]
    for k in sorted(dupes)[:20]:
        out.append(Finding("C", check, REJECT, "key-added-twice", rel, 0,
                           "baseline key appears more than once: {}".format(k[:120]),
                           "keep-both duplicated it; re-record the baseline"))
    return out


def assert_keepboth(path, base_ref="origin/main", repo=None):
    """Shape C. Diff ``path`` against ``base_ref``: nothing REMOVED, nothing ADDED TWICE.

    git's keep-both is the safe default for prose and exactly wrong for a counter or a
    table -- a duplicated table row survives every gate in this repo. Line multisets are
    compared rather than a textual diff, so a row re-inserted at a different offset is
    still caught. For a `[passing, total]`-footered baseline the footer is additionally
    asserted against its own key count.
    """
    root = _root(repo)
    p = Path(path)
    if not p.is_absolute():
        p = root / path
    rel = os.path.relpath(str(p), str(root))
    if not p.exists():
        return []
    head_text = p.read_text(encoding="utf-8", errors="replace")
    out = _footer_findings(p, rel, head_text, "assert-keepboth")

    if not have_ref(base_ref, repo):
        out.append(Finding("C", "assert-keepboth", WARN, "base-ref-missing", rel, 0,
                           "no {} to diff against; removed/added-twice check skipped".format(
                               base_ref)))
        return out
    base_text = show(base_ref, rel, repo)
    if base_text is None:
        return out

    base_c = collections.Counter(_norm(l) for l in base_text.splitlines() if _significant(l))
    head_c = collections.Counter(_norm(l) for l in head_text.splitlines() if _significant(l))

    removed = [(l, n - head_c.get(l, 0)) for l, n in base_c.items() if head_c.get(l, 0) < n]
    for line, missing in sorted(removed)[:20]:
        out.append(Finding("C", "assert-keepboth", REJECT, "line-removed", rel, 0,
                           "{} line(s) present on {} are gone".format(missing, base_ref),
                           line[:200]))
    if len(removed) > 20:
        out.append(Finding("C", "assert-keepboth", REJECT, "line-removed", rel, 0,
                           "... and {} more removed lines".format(len(removed) - 20)))

    twice = [(l, n) for l, n in head_c.items() if n >= 2 and n > base_c.get(l, 0)]
    for line, n in sorted(twice)[:20]:
        out.append(Finding("C", "assert-keepboth", REJECT, "line-added-twice", rel, 0,
                           "line appears {}x (was {}x on {})".format(
                               n, base_c.get(line, 0), base_ref),
                           line[:200]))
    if len(twice) > 20:
        out.append(Finding("C", "assert-keepboth", REJECT, "line-added-twice", rel, 0,
                           "... and {} more duplicated lines".format(len(twice) - 20)))
    return out


# --- the absolute: docs/baseline.json is never re-pinned --------------------

def assert_baseline_untouched(base_ref="origin/main", repo=None):
    """`docs/baseline.json` is the UPSTREAM baseline. Re-pinning it is a hard reject.

    The 675 upstream assertions are the port's oracle. `tests/stages/` exists in its own
    directory with its own baseline precisely so that this file is never touched; a branch
    that re-records it has hidden a regression rather than fixed one.
    """
    root = _root(repo)
    p = root / REL_BASELINE
    if not p.exists():
        return [Finding("-", "baseline-untouched", WARN, "baseline-missing", REL_BASELINE, 0,
                        "docs/baseline.json not found")]
    if not have_ref(base_ref, repo):
        return [Finding("-", "baseline-untouched", WARN, "base-ref-missing", REL_BASELINE, 0,
                        "no {} to compare against; the baseline guard could not run".format(
                            base_ref))]
    base_text = show(base_ref, REL_BASELINE, repo)
    head_text = p.read_text(encoding="utf-8", errors="replace")
    if base_text is None:
        return [Finding("-", "baseline-untouched", REJECT, "baseline-added", REL_BASELINE, 0,
                        "docs/baseline.json does not exist on {}".format(base_ref))]
    if base_text != head_text:
        try:
            b, h = json.loads(base_text), json.loads(head_text)
            delta = "base {} keys -> head {} keys".format(
                len(b.get("passing", [])), len(h.get("passing", [])))
        except (ValueError, AttributeError):
            delta = "{} bytes -> {} bytes".format(len(base_text), len(head_text))
        return [Finding("-", "baseline-untouched", REJECT, "baseline-repinned", REL_BASELINE, 0,
                        "docs/baseline.json was re-pinned ({})".format(delta),
                        "HARD REJECT. The upstream baseline is the oracle and is never "
                        "re-recorded. Revert it: git checkout {} -- {}".format(
                            base_ref, REL_BASELINE))]
    return [Finding("-", "baseline-untouched", INFO, "baseline-clean", REL_BASELINE, 0,
                    "docs/baseline.json is byte-identical to {}".format(base_ref))]


# --- the whole gate ---------------------------------------------------------

def run_all(base_ref="origin/main", repo=None, kuna=None, paths=None):
    """Every guard, in merge order. Any REJECT finding means: do not merge yet."""
    out = []
    out.extend(assert_baseline_untouched(base_ref, repo=repo))
    out.extend(check_div(base_ref, repo=repo))
    out.extend(assert_rederived(repo=repo, kuna=kuna))
    for rel in (paths if paths is not None else KEEPBOTH_PATHS):
        out.extend(assert_keepboth(rel, base_ref, repo=repo))
    return out


def rejects(findings):
    return [f for f in findings if f.severity == REJECT]


# --- self-test: three synthetic git histories -------------------------------

_GIT_ENV = ("-c", "user.name=repipe-selftest", "-c", "user.email=repipe@localhost",
            "-c", "commit.gpgsign=false", "-c", "init.defaultBranch=main")


def _sh(repo, *args):
    rc, out, err = _git(repo, *(_GIT_ENV + args))
    if rc != 0 and args[0] not in ("merge",):
        raise RuntimeError("git {} failed: {}".format(" ".join(args), err.strip()))
    return rc, out, err


def _write(repo, rel, text):
    p = Path(repo) / rel
    p.parent.mkdir(parents=True, exist_ok=True)
    p.write_text(text, encoding="utf-8")


def _new_repo(base):
    os.makedirs(base, exist_ok=True)
    _sh(base, "init", "-q", base)
    return base


def _selftest_shape_b(root):
    """base 85 / ours 86 / theirs 86 -> git merges cleanly, the truth is 87.

    Both branches make the byte-identical `85 -> 86` edit to the guard while adding a
    different item in a different hunk, which is the #254 shape: no conflict, no diff, and a
    count one short. The guard re-derives (count the items) instead of trusting the merge.
    """
    repo = _new_repo(os.path.join(root, "shape-b"))
    guard = 'assert_eq!(count, {}, "corpus file count drifted");\n'
    items = ["item-{:04d}".format(i) for i in range(1, 86)]
    _write(repo, "items.txt", "\n".join(items) + "\n")
    _write(repo, "guard.rs", guard.format(85))
    _sh(repo, "add", "-A"), _sh(repo, "commit", "-qm", "base: 85")
    _sh(repo, "branch", "theirs")

    _write(repo, "items.txt", "\n".join(items + ["item-ours"]) + "\n")
    _write(repo, "guard.rs", guard.format(86))
    _sh(repo, "add", "-A"), _sh(repo, "commit", "-qm", "ours: 85 -> 86")

    _sh(repo, "checkout", "-q", "theirs")
    _write(repo, "items.txt", "\n".join(["item-theirs"] + items) + "\n")
    _write(repo, "guard.rs", guard.format(86))
    _sh(repo, "add", "-A"), _sh(repo, "commit", "-qm", "theirs: 85 -> 86")

    _sh(repo, "checkout", "-q", "main")
    rc, _, _ = _sh(repo, "merge", "--no-edit", "-q", "theirs")

    pattern = r'assert_eq!\(count, (\d+), "corpus file count drifted"\)'
    merged = (Path(repo) / "guard.rs").read_text(encoding="utf-8")
    merged_value = int(re.search(pattern, merged).group(1))
    truth = sum(1 for l in (Path(repo) / "items.txt").read_text().splitlines() if l.strip())

    drift = counters.assert_literal(Path(repo) / "guard.rs", pattern, truth,
                                    site="selftest.guard", counter="corpus_files")
    counters.rewrite_literal(Path(repo) / "guard.rs", pattern, truth)
    after = int(re.search(pattern, (Path(repo) / "guard.rs").read_text()).group(1))

    caught = (rc == 0 and merged_value == 86 and truth == 87 and bool(drift) and after == 87)
    return {
        "case": "B",
        "name": "silent identical-edit auto-merge",
        "caught": caught,
        "log": "shape-B-averted" if caught else "shape-B-MISSED",
        "merge_conflicted": rc != 0,
        "merged_value": merged_value,
        "rederived_value": truth,
        "value_after_fix": after,
        "drift": [d.oneline() for d in drift],
        "repo": repo,
    }


def _selftest_shape_c(root):
    """keep-both merged both sides' new keys; data_footer stayed at the pre-merge count.

    The #253 shape: neither branch touched the footer, so there is nothing to conflict on
    and the merged file claims 375 while carrying 381 keys.
    """
    repo = _new_repo(os.path.join(root, "shape-c"))
    rel = "docs/baseline-stages.json"

    def doc(keys, footer):
        return json.dumps({"data_footer": [footer, footer], "passing": keys,
                           "returncode": 0, "unit_footer": None}, indent=2) + "\n"

    keys = ["data:STAGE #{:04d}: a stage assertion that is long enough to be significant".format(i)
            for i in range(1, 376)]
    _write(repo, rel, doc(keys, 375))
    _sh(repo, "add", "-A"), _sh(repo, "commit", "-qm", "base: 375 keys")
    _sh(repo, "branch", "theirs")

    ours = keys[:10] + ["data:OURS #{}: a new stage assertion from our branch".format(i)
                        for i in range(1, 4)] + keys[10:]
    _write(repo, rel, doc(ours, 375))
    _sh(repo, "add", "-A"), _sh(repo, "commit", "-qm", "ours: +3 keys")

    _sh(repo, "checkout", "-q", "theirs")
    theirs = keys + ["data:THEIRS #{}: a new stage assertion from their branch".format(i)
                     for i in range(1, 4)]
    _write(repo, rel, doc(theirs, 375))
    _sh(repo, "add", "-A"), _sh(repo, "commit", "-qm", "theirs: +3 keys")

    _sh(repo, "checkout", "-q", "main")
    rc, _, _ = _sh(repo, "merge", "--no-edit", "-q", "theirs")

    merged = json.loads((Path(repo) / rel).read_text(encoding="utf-8"))
    findings = assert_keepboth(rel, "theirs", repo=repo)
    stale = [f for f in findings if f.id == "stale-footer"]
    caught = rc == 0 and bool(stale) and merged["data_footer"][0] != len(merged["passing"])
    return {
        "case": "C",
        "name": "silent keep-both auto-merge (stale data_footer)",
        "caught": caught,
        "log": "shape-C-averted" if caught else "shape-C-MISSED",
        "merge_conflicted": rc != 0,
        "merged_footer": merged["data_footer"][0],
        "real_key_count": len(merged["passing"]),
        "findings": [f.oneline() for f in findings],
        "repo": repo,
    }


def _selftest_shape_c2(root):
    """Five rounds of keep-both duplicated a row in tests/stages/README.md.

    Both branches add the same row at different offsets, so git keeps both copies and the
    table carries it twice. No gate in this repo notices a duplicated table row.
    """
    repo = _new_repo(os.path.join(root, "shape-c2"))
    rel = "tests/stages/README.md"
    head = ["# Stage-model issue testcases", "", "| File | Issue | Stage / sub-stage |",
            "|---|---|---|"]
    rows = ["| `gh{n:04d}-case.xml` | [GH-{n}] | S{s} sub-stage decision point |".format(
        n=1000 + i, s=i % 9) for i in range(1, 21)]
    dup = "| `gh9999-newcase.xml` | [GH-9999] | S5 const-pointer decision point |"

    _write(repo, rel, "\n".join(head + rows) + "\n")
    _sh(repo, "add", "-A"), _sh(repo, "commit", "-qm", "base: 20 rows")
    _sh(repo, "branch", "theirs")

    _write(repo, rel, "\n".join(head + rows + [dup]) + "\n")
    _sh(repo, "add", "-A"), _sh(repo, "commit", "-qm", "ours: append the row")

    _sh(repo, "checkout", "-q", "theirs")
    _write(repo, rel, "\n".join(head + rows[:3] + [dup] + rows[3:]) + "\n")
    _sh(repo, "add", "-A"), _sh(repo, "commit", "-qm", "theirs: insert the same row")

    _sh(repo, "checkout", "-q", "main")
    rc, _, _ = _sh(repo, "merge", "--no-edit", "-q", "theirs")

    text = (Path(repo) / rel).read_text(encoding="utf-8")
    findings = assert_keepboth(rel, "theirs", repo=repo)
    twice = [f for f in findings if f.id == "line-added-twice"]
    caught = rc == 0 and text.count(dup) == 2 and bool(twice)
    return {
        "case": "C2",
        "name": "silent keep-both auto-merge (duplicated table row)",
        "caught": caught,
        "log": "shape-C2-averted" if caught else "shape-C2-MISSED",
        "merge_conflicted": rc != 0,
        "row_occurrences": text.count(dup),
        "findings": [f.oneline() for f in findings],
        "repo": repo,
    }


def self_test(keep=False):
    """Build the three histories in $TMPDIR and prove each shape is caught."""
    if not shutil.which("git"):
        return {"ok": False, "error": "git not on PATH"}
    root = tempfile.mkdtemp(prefix="repipe-mergecheck-selftest-")
    try:
        cases = [_selftest_shape_b(root), _selftest_shape_c(root), _selftest_shape_c2(root)]
        return {"ok": all(c["caught"] for c in cases), "scratch": root, "cases": cases}
    finally:
        if not keep:
            shutil.rmtree(root, ignore_errors=True)


# --- CLI --------------------------------------------------------------------

def _print_selftest(res) -> None:
    if not res.get("ok") and "error" in res:
        print("self-test could not run: {}".format(res["error"]))
        return
    print("scratch: {}".format(res["scratch"]))
    for c in res["cases"]:
        print("")
        print("shape {} -- {}".format(c["case"], c["name"]))
        print("  git merged cleanly: {}".format("no" if c["merge_conflicted"] else "yes"))
        if c["case"] == "B":
            print("  merged value {} / re-derived {} / after fix {}".format(
                c["merged_value"], c["rederived_value"], c["value_after_fix"]))
            for d in c["drift"]:
                print("  drift: {}".format(d))
        elif c["case"] == "C":
            print("  data_footer {} vs {} real keys".format(
                c["merged_footer"], c["real_key_count"]))
            for f in c["findings"]:
                print("  {}".format(f))
        else:
            print("  the row appears {}x after the merge".format(c["row_occurrences"]))
            for f in c["findings"]:
                print("  {}".format(f))
        print("  => {}  {}".format("CAUGHT" if c["caught"] else "MISSED", c["log"]))
    print("")
    print("self-test: {}".format("all three shapes caught" if res["ok"] else "FAILED"))


def main(argv=None) -> int:
    ap = argparse.ArgumentParser(
        prog="python -m scripts.repipe.mergecheck",
        description="The three shared-counter merge guards (docs/decbench-loop.md).")
    ap.add_argument("--against", "--base-ref", dest="base_ref", default="origin/main",
                    help="the ref this branch merges into (default: origin/main)")
    ap.add_argument("--repo", default=None, help="repo root (default: config.repo_root())")
    ap.add_argument("--kuna", default=None, help="kuna binary (default: config.kuna_bin())")
    ap.add_argument("--path", action="append", default=None,
                    help="extra keep-both path to check (repeatable; replaces the default set)")
    ap.add_argument("--self-test", action="store_true",
                    help="build three synthetic git histories and prove each shape is caught")
    ap.add_argument("--keep-scratch", action="store_true",
                    help="with --self-test, leave the scratch repos in place")
    ap.add_argument("--json", action="store_true", help="machine-readable output")
    args = ap.parse_args(argv)

    if args.self_test:
        res = self_test(keep=args.keep_scratch)
        if args.json:
            print(json.dumps(res, indent=2, sort_keys=True))
        else:
            _print_selftest(res)
        return 0 if res.get("ok") else 1

    findings = run_all(args.base_ref, repo=args.repo, kuna=args.kuna, paths=args.path)
    bad = rejects(findings)
    if args.json:
        print(json.dumps({"ok": not bad, "base_ref": args.base_ref,
                          "rejects": len(bad),
                          "findings": [f.as_dict() for f in findings]},
                         indent=2, sort_keys=True))
    else:
        for f in findings:
            print(f.oneline())
        print("")
        print("{} finding(s), {} reject(s) against {}".format(
            len(findings), len(bad), args.base_ref))
        print("MERGE BLOCKED" if bad else "merge guards clean")
    return 1 if bad else 0


if __name__ == "__main__":
    sys.exit(main())
