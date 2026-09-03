"""Re-derive every shared counter from the tree, then check it against each hard-coded site.

WHY this module exists, and why it never does arithmetic: this repo runs several
output-changing PRs at once and every one of them touches the same handful of hard-coded
numbers. Round 2 shipped a wrong count because both branches made the *identical*
`85 -> 86` edit, so git auto-merged them cleanly with no conflict and no diff to review --
and the right answer was 87 (docs/decbench-loop.md, "Shared counters in a busy queue").
The rule written down there is this file's whole design: **derive every shared counter from
a fresh capture on the rebased tree, never by arithmetic.** There is deliberately no
"base + mine + theirs" anywhere below -- every number is measured now, from phases.toml,
from a live `kuna catalog --json`, and from the files on disk.

The counters, how each is measured, and where each is hard-coded:

    settables (127)         `[[settable]]` blocks in kuna-decomp/phases.toml
                            -> kuna_phases/tests.rs   2 asserts + the test fn NAME
                            -> catalog_bytecompat.rs  3 FIXTURE.matches asserts + the fn NAME
    tiers (28/52/47)        row["tier"] tally from `kuna catalog --json`
                            -> kuna_phases/tests.rs   the tuple assert + the test fn NAME
    catalog buckets         field/value tallies over the same catalog rows
                            -> tests/stages/kuna-catalog.xml  per-bucket stringmatch min/max
    stage corpus (222)      tests/datatests/*.xml + tests/stages/*.xml, non-recursive
                            -> kuna-base/src/xml.rs   "corpus file count drifted"
    next ElementId (4132)   max `ElementId::new(_, N)` for N >= 4000, plus one. No hard-coded
                            site: the convention is "grep for the current highest"
                            (docs/agents.md, docs/improvement-pipeline.md).

The test function NAMES are checked as sites in their own right. They encode the number, no
compiler or test run disagrees with a stale one, and `settable_count_is_117` guarding an
`assert_eq!(..., 127)` is exactly the artifact a silent auto-merge leaves behind.

`decompiler/crates/kuna-decomp/tests/fixtures/phase_catalog.json` is NOT rewritten here. It
is *captured bytes* -- guessing at them is how the byte-compat gate stops meaning anything.
When it disagrees with the derived settable count this module reports an unfixable drift and
prints the recapture recipe, read out of catalog_bytecompat.rs's own module header so the
recipe cannot drift away from the gate that consumes it.

CLI:
    python -m scripts.repipe.counters                     # check; exit 1 on drift
    python -m scripts.repipe.counters --rederive --json   # measure now, report; read-only
    python -m scripts.repipe.counters --fix               # rewrite every fixable site
    python -m scripts.repipe.counters --recipe            # the phase_catalog.json recapture
"""
from __future__ import annotations

import argparse
import json
import os
import re
import subprocess
import sys
from dataclasses import asdict, dataclass, field
from pathlib import Path

from . import config

CATALOG_TIMEOUT = int(os.environ.get("REPIPE_CATALOG_TIMEOUT", "300"))

# kuna's own marshaling ids live in the 4000+ range; below that is the ported Ghidra table.
ELEMENT_ID_BASE = int(os.environ.get("REPIPE_ELEMENT_ID_BASE", "4000"))

CORPUS_DIRS = ("tests/datatests", "tests/stages")

REL_PHASES_TOML = "decompiler/crates/kuna-decomp/phases.toml"
REL_TESTS_RS = "decompiler/crates/kuna-decomp/src/p0_knowledge/kuna_phases/tests.rs"
REL_BYTECOMPAT_RS = "decompiler/crates/kuna-decomp/tests/catalog_bytecompat.rs"
REL_CATALOG_FIXTURE = "decompiler/crates/kuna-decomp/tests/fixtures/phase_catalog.json"
REL_CATALOG_XML = "tests/stages/kuna-catalog.xml"
REL_BASE_XML_RS = "decompiler/crates/kuna-base/src/xml.rs"

TIERS = ("core", "transform", "analysis")


class DeriveError(RuntimeError):
    """A counter could not be measured, so nothing downstream may be trusted."""


@dataclass
class Drift:
    """One hard-coded site that disagrees with the derived truth.

    ``fixable`` is False for a site whose correct value is a capture (phase_catalog.json) or
    an intentionally loose range -- those need an operator, not a rewrite.
    """
    site: str
    path: str
    line: int
    counter: str
    found: str
    expected: str
    fixable: bool = True
    detail: str = ""

    def as_dict(self) -> dict:
        return asdict(self)

    def oneline(self) -> str:
        tail = "  " + self.detail if self.detail else ""
        return "{}:{}  {}: found {} expected {}{}".format(
            self.path, self.line, self.site, self.found, self.expected, tail)


# --- paths ------------------------------------------------------------------

def _root(repo=None) -> Path:
    return Path(repo) if repo else config.repo_root()


def path_of(rel: str, repo=None) -> Path:
    return _root(repo) / rel


# --- generic site primitives (shared with mergecheck's self-test) ------------

def line_of(text: str, offset: int) -> int:
    """1-based line number of a byte offset, so every finding carries a file:line."""
    return text.count("\n", 0, offset) + 1


_line_of = line_of


def _replace_groups(text: str, m, newvals) -> str:
    """Return ``text`` with match ``m``'s numbered groups replaced, byte-exact elsewhere."""
    out = []
    pos = m.start()
    for i, nv in enumerate(newvals, start=1):
        out.append(text[pos:m.start(i)])
        out.append(str(nv))
        pos = m.end(i)
    out.append(text[pos:m.end()])
    return text[:m.start()] + "".join(out) + text[m.end():]


def _atomic_write(path: Path, text: str) -> None:
    tmp = str(path) + ".tmp"
    with open(tmp, "w", encoding="utf-8") as fh:
        fh.write(text)
    os.replace(tmp, str(path))


def assert_literal(path, pattern, expected, site="literal", counter="literal"):
    """Every match of ``pattern`` in ``path`` must carry ``expected`` in its numbered groups.

    This is the whole shape-B guard reduced to one call: a number written in a file is
    compared against a number measured now. A pattern that matches nothing is itself a
    finding -- the file changed shape and the guard has gone blind.
    """
    path = Path(path)
    expected = tuple(str(x) for x in (expected if isinstance(expected, (list, tuple)) else (expected,)))
    if not path.exists():
        return [Drift(site, str(path), 0, counter, "<missing file>", ",".join(expected), False,
                      "the site file does not exist")]
    text = path.read_text(encoding="utf-8", errors="replace")
    rx = re.compile(pattern)
    out, seen = [], 0
    for m in rx.finditer(text):
        seen += 1
        found = tuple(m.group(i) for i in range(1, len(expected) + 1))
        if found != expected:
            out.append(Drift(site, str(path), _line_of(text, m.start()), counter,
                             ",".join(found), ",".join(expected)))
    if not seen:
        out.append(Drift(site, str(path), 0, counter, "<no match>", ",".join(expected), False,
                         "pattern no longer matches; the guard is blind, fix the pattern"))
    return out


def rewrite_literal(path, pattern, expected) -> int:
    """Rewrite every match of ``pattern`` in ``path`` to ``expected``. Returns the edit count."""
    path = Path(path)
    expected = tuple(str(x) for x in (expected if isinstance(expected, (list, tuple)) else (expected,)))
    text = path.read_text(encoding="utf-8")
    rx = re.compile(pattern)
    edits = 0
    while True:
        for m in rx.finditer(text):
            found = tuple(m.group(i) for i in range(1, len(expected) + 1))
            if found != expected:
                text = _replace_groups(text, m, expected)
                edits += 1
                break
        else:
            break
    if edits:
        _atomic_write(path, text)
    return edits


# --- the sites --------------------------------------------------------------
#
# (site id, repo-relative path, regex whose numbered groups hold the number(s),
#  key into the derived dict supplying the expected value(s)).

SITES = (
    ("phases-tests.fn-name-settables", REL_TESTS_RS,
     r"fn settable_count_is_(\d+)\(\)", "settables"),
    ("phases-tests.kuna_num_settables", REL_TESTS_RS,
     r"assert_eq!\(kuna_num_settables\(\), (\d+)\)", "settables"),
    ("phases-tests.SETTABLE_TABLE-len", REL_TESTS_RS,
     r"assert_eq!\(SETTABLE_TABLE\.len\(\), (\d+)\)", "settables"),
    ("phases-tests.fn-name-tiers", REL_TESTS_RS,
     r"fn tier_counts_are_(\d+)_core_(\d+)_transform_(\d+)_analysis\(\)", "tier_tuple"),
    ("phases-tests.tier-tuple", REL_TESTS_RS,
     r"assert_eq!\(\(core, transform, analysis\), \((\d+), (\d+), (\d+)\)\)", "tier_tuple"),
    ("bytecompat.fn-name-settables", REL_BYTECOMPAT_RS,
     r"fn fixture_has_all_(\d+)_settables\(\)", "settables"),
    ("bytecompat.fixture-matches", REL_BYTECOMPAT_RS,
     r'FIXTURE\.matches\("\\"(?:option|tier|symptoms)\\": "\)\.count\(\), (\d+)\)', "settables"),
    ("phases-tests.catalog-json-records", REL_TESTS_RS,
     r'assert_eq!\(json\.matches\("\},\\n"\)\.count\(\), (\d+)\)', "settables_minus_one"),
    ("base-xml.corpus-count", REL_BASE_XML_RS,
     r'assert_eq!\(count, (\d+), "corpus file count drifted"\)', "corpus_files"),
)


# --- derivation -------------------------------------------------------------

def settables_from_phases_toml(repo=None) -> int:
    """`grep -c '^\\[\\[settable\\]\\]' phases.toml` -- the authoritative per-option list."""
    p = path_of(REL_PHASES_TOML, repo)
    if not p.exists():
        raise DeriveError("phases.toml missing at {}".format(p))
    n = 0
    with open(p, "r", encoding="utf-8", errors="replace") as fh:
        for line in fh:
            if line.startswith("[[settable]]"):
                n += 1
    return n


def catalog_rows(repo=None, kuna=None) -> list:
    """The live `kuna catalog --json` rows. A capture, not a parse of the source of truth."""
    exe = Path(kuna) if kuna else config.kuna_bin()
    if not exe.exists():
        raise DeriveError("kuna binary not built at {} (cargo build --release -p kuna-cli)".format(exe))
    try:
        proc = subprocess.run([str(exe), "catalog", "--json"], cwd=str(_root(repo)),
                              stdout=subprocess.PIPE, stderr=subprocess.PIPE,
                              timeout=CATALOG_TIMEOUT)
    except subprocess.TimeoutExpired:
        raise DeriveError("`kuna catalog --json` timed out after {}s".format(CATALOG_TIMEOUT))
    if proc.returncode != 0:
        raise DeriveError("`kuna catalog --json` exited {}: {}".format(
            proc.returncode, proc.stderr.decode("utf-8", "replace").strip()[:400]))
    try:
        data = json.loads(proc.stdout.decode("utf-8"))
    except ValueError as exc:
        raise DeriveError("`kuna catalog --json` emitted unparseable JSON: {}".format(exc))
    rows = data.get("settables", data) if isinstance(data, dict) else data
    if not isinstance(rows, list) or not rows:
        raise DeriveError("`kuna catalog --json` returned no settable rows")
    return rows


def corpus_counts(repo=None) -> dict:
    """Non-recursive `*.xml` per corpus dir -- exactly what kuna-base/src/xml.rs walks."""
    out = {}
    for rel in CORPUS_DIRS:
        d = path_of(rel, repo)
        if not d.is_dir():
            raise DeriveError("corpus dir missing: {}".format(d))
        out[rel] = sum(1 for p in d.iterdir() if p.is_file() and p.suffix == ".xml")
    return out


_ELEMENT_ID_RX = re.compile(r'ElementId::new\(\s*"([A-Za-z0-9_]+)"\s*,\s*(\d+)\s*\)')


def element_ids(repo=None) -> dict:
    """Every kuna-range ElementId in the crates, so the next free one can be handed out."""
    crates = _root(repo) / "decompiler" / "crates"
    if not crates.is_dir():
        raise DeriveError("crates dir missing: {}".format(crates))
    used = {}
    for dirpath, dirnames, filenames in os.walk(crates):
        dirnames[:] = [d for d in dirnames if d not in ("target", ".git")]
        for fn in filenames:
            if not fn.endswith(".rs"):
                continue
            fp = os.path.join(dirpath, fn)
            try:
                text = open(fp, "r", encoding="utf-8", errors="replace").read()
            except OSError:
                continue
            for m in _ELEMENT_ID_RX.finditer(text):
                num = int(m.group(2))
                if num < ELEMENT_ID_BASE:
                    continue
                used.setdefault(num, []).append("{}:{}".format(
                    os.path.relpath(fp, str(_root(repo))), m.group(1)))
    if not used:
        raise DeriveError("no ElementId in the {}+ range found".format(ELEMENT_ID_BASE))
    dupes = {str(k): v for k, v in sorted(used.items()) if len(v) > 1}
    return {"max": max(used), "next_free": max(used) + 1, "count": len(used), "duplicates": dupes}


_BUCKET_PAT_RX = re.compile(r'"(?P<key>[a-z_]+)": (?:"(?P<val>[A-Za-z0-9_ .:+/-]*)")?\Z')
_STRINGMATCH_RX = re.compile(
    r'<stringmatch\s+name="(?P<name>[^"]*)"\s+min="(?P<min>\d+)"\s+max="(?P<max>\d+)"\s*>'
    r'(?P<pat>.*?)</stringmatch>', re.S)
_SINGLE_ROW_RX = re.compile(r'<com>\s*phase catalog\s+(\S+?)\s*</com>')


def _catalog_xml_text(repo=None) -> str:
    p = path_of(REL_CATALOG_XML, repo)
    if not p.exists():
        raise DeriveError("catalog stage test missing: {}".format(p))
    return p.read_text(encoding="utf-8")


def _bucket_expect(rows, key, val, single_opts) -> int:
    """How many times a bucket pattern must appear in the whole console transcript.

    The script emits the full catalog once and then one `phase catalog <opt>` row per
    single-row command, so an option named in the script contributes twice. That is why
    KUNA-CATALOG #8 pins 15 while the catalog itself carries 14 opt-in-tool rows.
    """
    def hit(row):
        if key not in row:
            return False
        return True if val is None else row.get(key) == val
    n = sum(1 for r in rows if hit(r))
    by_opt = {r.get("option"): r for r in rows}
    for opt in single_opts:
        r = by_opt.get(opt)
        if r is not None and hit(r):
            n += 1
    return n


def catalog_buckets(rows, repo=None) -> list:
    """Derive the expected count for every *bucket* stringmatch in kuna-catalog.xml.

    A bucket is a bare `"<field>": ` or `"<field>": "<value>"` pattern -- a tally over rows.
    The per-option row assertions (patterns that open `"option": "<name>"`) are not counters
    and are reported as skipped rather than silently ignored.
    """
    text = _catalog_xml_text(repo)
    singles = _SINGLE_ROW_RX.findall(text)
    out = []
    for m in _STRINGMATCH_RX.finditer(text):
        pat = m.group("pat")
        bm = _BUCKET_PAT_RX.match(pat)
        rec = {
            "name": m.group("name"),
            "pattern": pat,
            "min": int(m.group("min")),
            "max": int(m.group("max")),
            "line": _line_of(text, m.start()),
        }
        if not bm or bm.group("key") == "option":
            rec.update({"bucket": False, "expected": None,
                        "reason": "per-option row assertion, not a counter"})
        else:
            key, val = bm.group("key"), bm.group("val") or None
            rec.update({"bucket": True, "field": key, "value": val,
                        "expected": _bucket_expect(rows, key, val, singles),
                        "exact": int(m.group("min")) == int(m.group("max"))})
        out.append(rec)
    return out


def derive(repo=None, kuna=None) -> dict:
    """Measure every shared counter now, on this tree. The only source of truth in here."""
    rows = catalog_rows(repo=repo, kuna=kuna)
    tiers = dict((t, 0) for t in TIERS)
    untyped = []
    for r in rows:
        t = r.get("tier")
        if t in tiers:
            tiers[t] += 1
        else:
            untyped.append(r.get("option"))
    corpus = corpus_counts(repo=repo)
    toml_n = settables_from_phases_toml(repo=repo)
    eids = element_ids(repo=repo)
    fixture = path_of(REL_CATALOG_FIXTURE, repo)
    fixture_n = (fixture.read_text(encoding="utf-8").count('"option": ')
                 if fixture.exists() else None)
    return {
        "repo": str(_root(repo)),
        "settables": len(rows),
        # The static catalog JSON separates records with "},\n", so the last record
        # has none: one fewer than the settable count. Its own site, because a
        # regex that fixed it to `settables` would write the wrong number.
        "settables_minus_one": len(rows) - 1,
        "settables_phases_toml": toml_n,
        "settables_catalog": len(rows),
        "settables_fixture": fixture_n,
        "tiers": tiers,
        "tier_tuple": [tiers[t] for t in TIERS],
        "tier_order": list(TIERS),
        "untiered_options": untyped,
        "corpus_files": sum(corpus.values()),
        "corpus_by_dir": corpus,
        "next_element_id": eids["next_free"],
        "element_ids": eids,
        "catalog_buckets": catalog_buckets(rows, repo=repo),
    }


# --- checking ---------------------------------------------------------------

def check(derived=None, repo=None, kuna=None) -> list:
    """Compare the derived truth against every hard-coded site. Never trusts a merged value."""
    d = derived if derived is not None else derive(repo=repo, kuna=kuna)
    out = []

    if d["settables_phases_toml"] != d["settables_catalog"]:
        out.append(Drift("phases-toml.vs-catalog", REL_PHASES_TOML, 0, "settables",
                         str(d["settables_phases_toml"]), str(d["settables_catalog"]), False,
                         "phases.toml and the built binary disagree; rebuild before merging"))
    if sum(d["tier_tuple"]) != d["settables"]:
        out.append(Drift("catalog.tier-coverage", REL_PHASES_TOML, 0, "tiers",
                         str(sum(d["tier_tuple"])), str(d["settables"]), False,
                         "options with no core/transform/analysis tier: {}".format(
                             ", ".join(x or "?" for x in d["untiered_options"]) or "none")))

    for site, rel, pattern, key in SITES:
        out.extend(assert_literal(path_of(rel, repo), pattern, d[key], site=site, counter=key))

    for b in d["catalog_buckets"]:
        if not b.get("bucket"):
            continue
        exp, lo, hi = b["expected"], b["min"], b["max"]
        if b["exact"]:
            if exp != lo:
                out.append(Drift("catalog-xml.bucket:{}".format(b["pattern"].strip()),
                                 REL_CATALOG_XML, b["line"], "catalog-bucket",
                                 "min={} max={}".format(lo, hi), "min={0} max={0}".format(exp),
                                 True, b["name"][:80]))
        elif not (lo <= exp <= hi):
            out.append(Drift("catalog-xml.bucket:{}".format(b["pattern"].strip()),
                             REL_CATALOG_XML, b["line"], "catalog-bucket",
                             "min={} max={} (derived {})".format(lo, hi, exp),
                             "a range containing {}".format(exp), False,
                             "deliberately loose bucket; widen it by hand"))

    if d["settables_fixture"] is not None and d["settables_fixture"] != d["settables"]:
        out.append(Drift("bytecompat.fixture-capture", REL_CATALOG_FIXTURE, 0, "settables",
                         str(d["settables_fixture"]), str(d["settables"]), False,
                         "captured bytes: recapture, never edit (see --recipe)"))
    return out


# --- fixing -----------------------------------------------------------------

def recapture_recipe(repo=None) -> str:
    """The phase_catalog.json recapture, read out of catalog_bytecompat.rs's own header.

    Reading it rather than restating it means the recipe cannot drift away from the gate
    that consumes the capture -- and the "no program loaded" condition, which is the part a
    guess always gets wrong, travels with it.
    """
    p = path_of(REL_BYTECOMPAT_RS, repo)
    if not p.exists():
        return "(catalog_bytecompat.rs missing; cannot read the recipe)"
    header = []
    for line in p.read_text(encoding="utf-8").splitlines():
        if line.startswith("//!"):
            header.append(line[3:].lstrip(" "))
        elif header:
            break
    doc = "\n".join(header)
    idx = doc.find("## Regenerating the fixture")
    body = doc[idx:] if idx >= 0 else doc
    fences = re.findall(r"```sh\n(.*?)```", body, re.S)
    lead = ("Recapture decompiler/crates/kuna-decomp/tests/fixtures/phase_catalog.json.\n"
            "It is CAPTURED BYTES, with NO PROGRAM LOADED so kunaLiveValue returns \"\" and no\n"
            "`current` field is emitted. Never hand-edit it, and never re-capture with a program\n"
            "loaded -- `current` is architecture-dependent and is not a byte contract.\n")
    if not fences:
        return lead + "\n(no ```sh block found in catalog_bytecompat.rs's header)\n"
    return lead + "\n" + fences[0].rstrip() + "\n"


def fix(derived=None, repo=None, kuna=None):
    """Rewrite every fixable site to the derived value; report the rest with the recipe.

    Returns ``(applied, remaining)``: the drifts this rewrote, and the drifts that need an
    operator (a capture, or a loose range whose correct widening is a judgement call).
    """
    d = derived if derived is not None else derive(repo=repo, kuna=kuna)
    drifts = check(derived=d, repo=repo)
    applied = []
    for site, rel, pattern, key in SITES:
        hits = [x for x in drifts if x.site == site and x.fixable]
        if hits:
            rewrite_literal(path_of(rel, repo), pattern, d[key])
            applied.extend(hits)
    xml_path = path_of(REL_CATALOG_XML, repo)
    xml_hits = [x for x in drifts if x.site.startswith("catalog-xml.bucket:") and x.fixable]
    if xml_hits:
        text = xml_path.read_text(encoding="utf-8")
        want = dict((b["name"], b["expected"]) for b in d["catalog_buckets"]
                    if b.get("bucket") and b.get("exact") and b["expected"] != b["min"])
        for m in reversed(list(_STRINGMATCH_RX.finditer(text))):
            exp = want.get(m.group("name"))
            if exp is None:
                continue
            text = (text[:m.start("min")] + str(exp) + text[m.end("min"):m.start("max")]
                    + str(exp) + text[m.end("max"):])
        _atomic_write(xml_path, text)
        applied.extend(xml_hits)
    remaining = [x for x in drifts if not x.fixable]
    return applied, remaining


# --- CLI --------------------------------------------------------------------

def _print_derived(d) -> None:
    e = d["element_ids"]
    print("settables            {}   (phases.toml {} / catalog {} / fixture {})".format(
        d["settables"], d["settables_phases_toml"], d["settables_catalog"],
        d["settables_fixture"]))
    print("tiers                ({}, {}, {})  core/transform/analysis".format(*d["tier_tuple"]))
    print("stage-corpus files   {}   ({})".format(
        d["corpus_files"],
        ", ".join("{} {}".format(v, k) for k, v in sorted(d["corpus_by_dir"].items()))))
    print("next free ElementId  {}   (highest in use {}, {} allocated)".format(
        d["next_element_id"], e["max"], e["count"]))
    if e["duplicates"]:
        for num, who in e["duplicates"].items():
            print("  ! ElementId {} allocated twice: {}".format(num, ", ".join(who)))
    for b in d["catalog_buckets"]:
        if b.get("bucket"):
            print("catalog bucket       {:<3}  {}  [min={} max={}]".format(
                b["expected"], b["pattern"].strip(), b["min"], b["max"]))


def main(argv=None) -> int:
    ap = argparse.ArgumentParser(
        prog="python -m scripts.repipe.counters",
        description="Re-derive kuna's shared counters and check every hard-coded site.")
    ap.add_argument("--rederive", action="store_true",
                    help="measure now and report; read-only")
    ap.add_argument("--check", action="store_true",
                    help="the default: report drift, exit 1 if any")
    ap.add_argument("--fix", action="store_true",
                    help="rewrite every fixable site to the derived value")
    ap.add_argument("--recipe", action="store_true",
                    help="print the phase_catalog.json recapture recipe and exit")
    ap.add_argument("--repo", default=None, help="repo root (default: config.repo_root())")
    ap.add_argument("--kuna", default=None, help="kuna binary (default: config.kuna_bin())")
    ap.add_argument("--json", action="store_true", help="machine-readable output")
    args = ap.parse_args(argv)

    if args.recipe:
        if args.json:
            print(json.dumps({"recipe": recapture_recipe(args.repo)}, indent=2))
        else:
            print(recapture_recipe(args.repo))
        return 0

    try:
        d = derive(repo=args.repo, kuna=args.kuna)
    except DeriveError as exc:
        if args.json:
            print(json.dumps({"ok": False, "error": str(exc)}, indent=2))
        else:
            print("derive failed: {}".format(exc), file=sys.stderr)
        return 2

    if args.fix:
        applied, remaining = fix(derived=d, repo=args.repo)
        payload = {"ok": not remaining, "derived": d,
                   "applied": [x.as_dict() for x in applied],
                   "remaining": [x.as_dict() for x in remaining],
                   "recipe": recapture_recipe(args.repo)}
        if args.json:
            print(json.dumps(payload, indent=2, sort_keys=True))
        else:
            _print_derived(d)
            print("")
            for x in applied:
                print("fixed   {}".format(x.oneline()))
            for x in remaining:
                print("MANUAL  {}".format(x.oneline()))
            if not applied and not remaining:
                print("no drift; nothing to fix")
            print("")
            print(recapture_recipe(args.repo))
        return 0 if not remaining else 1

    drifts = check(derived=d, repo=args.repo)
    # A duplicate ElementId is a FAILURE, not a note. Two options sharing an id compare
    # equal (ElementId's PartialEq is an id comparison, ported from C++), so the first
    # code that dispatches on one silently takes the wrong branch. Nothing consumes the
    # colliding ids today, which is exactly why this went unnoticed through three
    # separate races -- 4110, 4122, and 4132, the last of which two concurrent builders
    # in one round created by each grepping for "the next free id" and getting the same
    # answer. Reporting it while exiting 0 is what let all three land.
    dupes = (d.get("element_ids") or {}).get("duplicates") or {}
    payload = {"ok": not drifts and not dupes, "derived": d,
               "drift": [x.as_dict() for x in drifts]}
    if args.json:
        print(json.dumps(payload, indent=2, sort_keys=True))
    else:
        _print_derived(d)
        print("")
        if drifts:
            for x in drifts:
                print("DRIFT  {}".format(x.oneline()))
        else:
            print("no drift: every hard-coded site agrees with the derived truth")
        if dupes:
            for num, who in dupes.items():
                print("FAIL   ElementId {} allocated twice: {}".format(num, ", ".join(who)))
    if args.rederive:
        return 0
    return 1 if (drifts or dupes) else 0


if __name__ == "__main__":
    sys.exit(main())
