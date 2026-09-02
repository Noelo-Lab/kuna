"""The durable RE-friction backlog: one committed Markdown record per need.

A *need* is a clustered, gated unit of work that survived the two-arm gate (its probe PASSES
and its acceptance FAILS on a freshly built main). It lives at ``docs/re-needs/<need_id>.md``
and is the only thing a builder is ever dispatched against; rejected filings move to
``docs/re-needs/rejected/`` so the denominator stays honest.

The file dialect is deliberately the same one ``docs/decbench/triage/<case-id>.md`` uses and
``scripts/decbench/status.py`` already parses: a ``---`` fenced block of flat ``key: value``
lines, then fixed ``##`` sections. Deliberate, because an agent that has read one of these
records can read the other without being taught a second format, and because a 12-line regex
parser is a feature -- the records stay hand-editable and diffable in review.

Why the front-matter parser is hand-written: kuna's Python tooling is stdlib-only (no
pyproject.toml, no requirements.txt, and ~/.virtualenvs/kuna has zero third-party packages),
so PyYAML is not available. The supported subset is exactly what a need record needs:

    key: value          strings, ints, floats, true/false, null (also ~), and flow lists
    key: [a, b, c]      one line each, no nesting, no block scalars, no anchors
    key: "quoted"       double quotes with \\" and \\\\ escapes, for values that would
                        otherwise parse as a number/bool/null or start with a YAML sigil

Round-trip stability is a hard requirement: ``write(parse(p)) == p`` byte for byte for every
record this module produces, so the round-N commit of ``docs/re-needs/`` shows only the fields
that actually changed. write() normalizes (canonical field order, canonical scalar spelling,
one blank line around every heading), so a hand-edited record becomes canonical the first time
the pipeline touches it and is stable from then on.

CLI:
    python3 -m scripts.repipe.needs list [--status S] [--track T] [--json]
    python3 -m scripts.repipe.needs show <need_id> [--json]
    python3 -m scripts.repipe.needs reindex [--json]
    python3 -m scripts.repipe.needs rank [--json]
    python3 -m scripts.repipe.needs opportunities [--out FILE] [--json]
    python3 -m scripts.repipe.needs reject <need_id> --reason R [--json]

``opportunities`` emits the backlog in the exact ``{"ranked": [...]}`` shape of
``docs/improvement-pipeline/opportunities.json`` so ``scripts/pipeline/select.py`` consumes it
unchanged -- that is the whole reuse story for worker dispatch: the RE loop gets claims,
leases, slugs and the driver's select/claim/launch dance for free.
"""
from __future__ import annotations

import argparse
import hashlib
import json
import math
import os
import re
import sys
from collections import OrderedDict

from . import config

SCHEMA = "re-need/1"

# Front-matter field order. write() emits exactly this order, then any unknown keys the record
# carried, in the order they were read -- so a field added by a future module survives a
# round-trip instead of being silently dropped.
FIELDS = [
    "need_id", "title", "track", "status", "severity",
    "probe_id", "acceptance_id", "hypothesis_status", "credibility",
    "instances", "challenges", "rounds", "first_seen_round", "attempts",
    "covered_by_option", "touches", "scope", "regression_of",
    "pr", "closed_in_round", "closing_pr", "reject_reason",
]

SECTIONS = [
    "Symptom",       # verbatim tester quotes + the replayed transcript
    "Reproduction",  # the probe, and its replay result at the pinned SHA
    "Acceptance",    # the acceptance probe, and its (failing) replay result
    "Hypothesis",    # ADVISORY -- the builder is contractually NOT bound to it
    "Refutation",    # the refuter's verdict, recorded whether or not it changed anything
    "Reference",     # what IDA/declib does instead, if anything
    "Instances",     # per-challenge, per-tester
    "Decision log",
]

TRACKS = ("tooling", "quality", "perf", "loader")
STATUSES = ("open", "claimed", "building", "proposal", "closed", "regressed", "blocked", "rejected")
SEVERITIES = ("blocker", "major", "minor")
HYPOTHESIS_STATUSES = ("upheld", "overturned", "inconclusive")
SCOPES = ("small", "large")

# Statuses a builder may be dispatched against. `claimed`/`building`/`proposal` are already in
# flight (state.py holds the claim), `closed`/`blocked`/`rejected` are done or parked.
DISPATCHABLE = ("open", "regressed")

LIST_FIELDS = ("challenges", "rounds", "touches")
INT_FIELDS = ("instances", "first_seen_round", "attempts", "closed_in_round")
FLOAT_FIELDS = ("credibility",)

DEFAULTS = {
    "track": "tooling",
    "status": "open",
    "severity": "minor",
    "hypothesis_status": "inconclusive",
    "credibility": 0.0,
    "instances": 0,
    "challenges": [],
    "rounds": [],
    "first_seen_round": 0,
    "attempts": 0,
    "covered_by_option": None,
    "touches": [],
    "scope": "small",
    "regression_of": None,
    "pr": None,
    "closed_in_round": None,
    "closing_pr": None,
    "reject_reason": None,
}

# --- rank weights -----------------------------------------------------------

SEVERITY_WEIGHT = {"blocker": 3.0, "major": 2.0, "minor": 1.0}
# Scale keeps the smallest real need (minor / 1 instance / 1 challenge = 0.69 raw) above
# select.py's `--min-score 1` integer floor, which would otherwise silently drop it.
SCORE_SCALE = 10.0
ATTEMPT_PENALTY = 0.5      # each failed builder attempt divides the score by 1 + 0.5*attempts
REGRESSION_BOOST = 8.0
REGRESSION_FLOOR = 1000.0  # additive, so a regression outranks any non-regression outright


# --- the front-matter dialect ----------------------------------------------

_KV_RE = re.compile(r"^([A-Za-z_][A-Za-z0-9_-]*):(?:[ \t]+(.*))?$")
_INT_RE = re.compile(r"^[+-]?\d+$")
_FLOAT_RE = re.compile(r"^[+-]?(?:\d+\.\d*|\.\d+|\d+)(?:[eE][+-]?\d+)?$")
_HEADING_RE = re.compile(r"^##[ \t]+(.+?)[ \t]*$", re.M)
_JSON_FENCE_RE = re.compile(r"^```json[ \t]*\n(.*?)^```[ \t]*$", re.S | re.M)
_NULLS = ("null", "~")
_SIGILS = "\"'[]{}&*!|>%@`#-?:,"


def _unescape(s):
    out = []
    i = 0
    while i < len(s):
        c = s[i]
        if c == "\\" and i + 1 < len(s):
            i += 1
            nxt = s[i]
            out.append({"n": "\n", "t": "\t", "r": "\r"}.get(nxt, nxt))
        else:
            out.append(c)
        i += 1
    return "".join(out)


def _parse_scalar(raw):
    s = raw.strip()
    if s.startswith('"') and s.endswith('"') and len(s) >= 2:
        return _unescape(s[1:-1])
    if s == "":
        return None
    if s in _NULLS:
        return None
    if s == "true":
        return True
    if s == "false":
        return False
    if _INT_RE.match(s):
        return int(s)
    if _FLOAT_RE.match(s):
        return float(s)
    return s


def _split_flow(inner):
    """Split a flow-list body on commas that are not inside a double-quoted item."""
    parts, buf, quoted, esc = [], [], False, False
    for ch in inner:
        if esc:
            buf.append(ch)
            esc = False
        elif ch == "\\" and quoted:
            buf.append(ch)
            esc = True
        elif ch == '"':
            quoted = not quoted
            buf.append(ch)
        elif ch == "," and not quoted:
            parts.append("".join(buf))
            buf = []
        else:
            buf.append(ch)
    parts.append("".join(buf))
    return [p for p in parts if p.strip() != ""] if len(parts) > 1 or parts[0].strip() else []


def _parse_value(raw):
    s = raw.strip()
    if s.startswith("[") and s.endswith("]"):
        return [_parse_scalar(p) for p in _split_flow(s[1:-1])]
    return _parse_scalar(s)


def _must_quote(s, in_list=False):
    if s == "" or s != s.strip():
        return True
    if s in _NULLS or s in ("true", "false"):
        return True
    if _INT_RE.match(s) or _FLOAT_RE.match(s):
        return True
    if s[0] in _SIGILS:
        return True
    if "#" in s or "\n" in s or "\r" in s or "\t" in s:
        return True
    if in_list and ("," in s or "[" in s or "]" in s):
        return True
    return False


def _emit_scalar(v, in_list=False):
    if v is None:
        return "null"
    if v is True:
        return "true"
    if v is False:
        return "false"
    if isinstance(v, int):
        return str(v)
    if isinstance(v, float):
        return repr(v)
    s = str(v)
    if _must_quote(s, in_list):
        # Control characters must be ESCAPED, not merely quoted. _unescape() already maps
        # \n and \t back on the way in, but emitting a raw newline inside quotes ends the
        # line mid-value: everything after it is re-read as further front-matter keys, so a
        # title containing a newline silently destroys every field below it -- and titles come
        # from LLM testers.
        s = (s.replace("\\", "\\\\").replace('"', '\\"')
              .replace("\n", "\\n").replace("\r", "\\r").replace("\t", "\\t"))
        return '"' + s + '"'
    return s


def _emit_value(v):
    if isinstance(v, (list, tuple)):
        return "[" + ", ".join(_emit_scalar(x, in_list=True) for x in v) + "]"
    return _emit_scalar(v)


def _coerce(key, value):
    """Pull a parsed value into the type its field declares, so `instances: 9` never arrives
    as the string "9" from a hand edit."""
    if key in LIST_FIELDS:
        if value is None:
            return []
        if not isinstance(value, list):
            return [value]
        return list(value)
    if key in INT_FIELDS and isinstance(value, (str, float, bool)):
        try:
            return int(value)
        except (TypeError, ValueError):
            return value
    if key in FLOAT_FIELDS and isinstance(value, (str, int)) and not isinstance(value, bool):
        try:
            return float(value)
        except (TypeError, ValueError):
            return value
    return value


# --- the probe id contract --------------------------------------------------

def probe_id_of(probe, is_acceptance=False):
    """The probe-id derivation, identical in every module of the loop (docs/re-pipeline.md).

    Keyed on cmd + expect only: two filings that run the same argv and assert the same thing
    ARE the same probe, whatever prose the tester wrapped around them. That is what makes
    dedup deterministic instead of an LLM call.
    """
    payload = json.dumps({"cmd": probe["cmd"], "expect": probe["expect"]},
                         sort_keys=True, separators=(",", ":")).encode()
    return ("a-" if is_acceptance else "p-") + hashlib.sha1(payload).hexdigest()[:12]


# --- the record -------------------------------------------------------------

class Need:
    """One backlog record: flat front-matter fields plus the fixed ``##`` sections.

    Every name in FIELDS is readable as an attribute (``need.severity``), falling back to
    DEFAULTS when the record omits it. Writes go through ``need.fields[...]`` so the
    unknown-field passthrough stays honest.
    """

    def __init__(self, fields=None, sections=None, path=None):
        self.fields = OrderedDict()
        for k, v in (fields or {}).items():
            self.fields[k] = _coerce(k, v)
        self.sections = OrderedDict(sections or {})
        self.path = path

    def __getattr__(self, name):
        if name in FIELDS:
            return self.__dict__.get("fields", {}).get(name, DEFAULTS.get(name))
        raise AttributeError(name)

    def __repr__(self):
        return "<Need %s %s/%s score=%.2f>" % (
            self.fields.get("need_id"), self.status, self.severity, rank_score(self))

    def get(self, key, default=None):
        return self.fields.get(key, DEFAULTS.get(key, default))

    def set(self, key, value):
        self.fields[key] = _coerce(key, value)

    def section(self, name):
        return self.sections.get(name, "")

    def probe(self):
        """The probe object embedded as the first ```json fence under ## Reproduction."""
        return _first_json_fence(self.section("Reproduction"))

    def acceptance(self):
        """The acceptance object embedded as the first ```json fence under ## Acceptance."""
        return _first_json_fence(self.section("Acceptance"))

    def log(self, line):
        body = self.sections.get("Decision log", "").rstrip()
        self.sections["Decision log"] = (body + "\n" + line).strip() if body else line

    def to_dict(self, score=True):
        d = OrderedDict()
        for k in FIELDS:
            d[k] = self.fields.get(k, DEFAULTS.get(k))
        for k in self.fields:
            if k not in d:
                d[k] = self.fields[k]
        if score:
            d["score"] = rank_score(self)
        d["path"] = str(self.path) if self.path else None
        return d


def _first_json_fence(text):
    m = _JSON_FENCE_RE.search(text or "")
    if not m:
        return None
    try:
        return json.loads(m.group(1))
    except json.JSONDecodeError:
        return None


def parse(path):
    """Read one record. Raises ValueError if the front-matter fence is missing."""
    path = str(path)
    with open(path, encoding="utf-8", errors="replace") as fh:
        text = fh.read()
    m = re.match(r"^---\n(.*?)\n---\n?", text, re.S)
    if not m:
        raise ValueError("%s: no --- front matter" % path)

    fields = OrderedDict()
    for line in m.group(1).splitlines():
        if not line.strip() or line.lstrip().startswith("#"):
            continue
        kv = _KV_RE.match(line)
        if not kv:
            continue
        fields[kv.group(1)] = _coerce(kv.group(1), _parse_value(kv.group(2) or ""))

    body = text[m.end():]
    sections = OrderedDict()
    marks = list(_HEADING_RE.finditer(body))
    for i, mk in enumerate(marks):
        end = marks[i + 1].start() if i + 1 < len(marks) else len(body)
        sections[mk.group(1)] = body[mk.end():end].strip("\n").rstrip()

    need = Need(fields=fields, sections=sections, path=path)
    if not need.fields.get("need_id"):
        need.fields["need_id"] = os.path.splitext(os.path.basename(path))[0]
    return need


def render(need):
    """The canonical text of a record. parse() of this returns an equal Need."""
    lines = ["---"]
    for key in FIELDS:
        if key in need.fields:
            lines.append("%s: %s" % (key, _emit_value(need.fields[key])))
    for key in need.fields:
        if key not in FIELDS:
            lines.append("%s: %s" % (key, _emit_value(need.fields[key])))
    lines.append("---")

    names = [s for s in SECTIONS if s in need.sections]
    names += [s for s in need.sections if s not in SECTIONS]
    for name in names:
        lines.append("")
        lines.append("## " + name)
        body = (need.sections.get(name) or "").strip("\n").rstrip()
        if body:
            lines.append("")
            lines.extend(body.split("\n"))
    return "\n".join(lines) + "\n"


def validate(need):
    """Return a list of human-readable problems. Empty means the record is well-formed."""
    problems = []
    nid = need.fields.get("need_id")
    if not nid or not re.match(r"^[a-z0-9][a-z0-9-]*$", str(nid)):
        problems.append("need_id must be a lowercase slug, got %r" % (nid,))
    for key, allowed in (("track", TRACKS), ("status", STATUSES), ("severity", SEVERITIES),
                         ("hypothesis_status", HYPOTHESIS_STATUSES), ("scope", SCOPES)):
        val = need.get(key)
        if val is not None and val not in allowed:
            problems.append("%s must be one of %s, got %r" % (key, "|".join(allowed), val))
    cred = need.get("credibility")
    if cred is not None and not (isinstance(cred, (int, float)) and 0.0 <= float(cred) <= 1.0):
        problems.append("credibility must be 0..1, got %r" % (cred,))
    for key in ("probe_id", "acceptance_id"):
        val = need.get(key)
        want = "a-" if key == "acceptance_id" else "p-"
        if val is not None and not re.match("^%s[0-9a-f]{12}$" % want, str(val)):
            problems.append("%s must match %s<12 hex>, got %r" % (key, want, val))
    return problems


def path_for(need_id, rejected=False):
    base = config.rejected_dir() if rejected else config.needs_dir()
    return base / ("%s.md" % need_id)


def write(need, path=None):
    """Render and atomically replace the record. Returns the path written.

    Atomic because the captain, the clusterer and a status reader can all touch
    docs/re-needs/ at once; a half-written record would fail the next parse().
    """
    problems = validate(need)
    if problems:
        raise ValueError("%s: %s" % (need.fields.get("need_id"), "; ".join(problems)))
    target = str(path or need.path or path_for(need.need_id, need.status == "rejected"))
    os.makedirs(os.path.dirname(target) or ".", exist_ok=True)
    tmp = target + ".tmp"
    with open(tmp, "w", encoding="utf-8") as fh:
        fh.write(render(need))
    os.replace(tmp, target)
    need.path = target
    return target


def load_all(include_rejected=False):
    """Every record on disk, sorted by need_id. Unparseable files are skipped, not fatal."""
    out = []
    dirs = [config.needs_dir()]
    if include_rejected:
        dirs.append(config.rejected_dir())
    for d in dirs:
        if not d.is_dir():
            continue
        for f in sorted(d.glob("*.md")):
            try:
                out.append(parse(f))
            except (ValueError, OSError):
                continue
    out.sort(key=lambda n: str(n.need_id))
    return out


def load(need_id):
    """One record by id, from the backlog or from rejected/. None if absent."""
    for rejected in (False, True):
        p = path_for(need_id, rejected)
        if p.exists():
            return parse(p)
    return None


def _union(old, new):
    seen, out = set(), []
    for item in list(old or []) + list(new or []):
        key = json.dumps(item, sort_keys=True) if isinstance(item, (dict, list)) else item
        if key in seen:
            continue
        seen.add(key)
        out.append(item)
    return out


def upsert(need):
    """Merge a filing into the backlog and write it. Returns the path.

    Merge rules, all deterministic (bumping instances on a re-observed need must never cost an
    LLM call): list fields union, `instances` and `attempts` take the max, `first_seen_round`
    takes the min, a non-empty incoming section replaces the stored one and an empty one keeps
    it (so a re-filing never erases a refuter's verdict or the decision log), and any other
    incoming non-None scalar wins.
    """
    existing = load(need.need_id)
    if existing is None:
        merged = Need(fields=need.fields, sections=need.sections)
        for k, v in DEFAULTS.items():
            merged.fields.setdefault(k, v)
        return write(merged, path_for(merged.need_id, merged.status == "rejected"))

    merged = Need(fields=existing.fields, sections=existing.sections, path=existing.path)
    for key, value in need.fields.items():
        if key in LIST_FIELDS:
            merged.fields[key] = _union(existing.get(key), value)
        elif key in ("instances", "attempts"):
            merged.fields[key] = max(int(existing.get(key) or 0), int(value or 0))
        elif key == "first_seen_round":
            cur = existing.get(key)
            merged.fields[key] = int(value) if cur is None else min(int(cur), int(value or 0))
        elif value is not None:
            merged.fields[key] = _coerce(key, value)
    for name, body in need.sections.items():
        if (body or "").strip():
            merged.sections[name] = body
        merged.sections.setdefault(name, existing.sections.get(name, ""))
    return write(merged, merged.path)


def apply_acceptance(suite, round_n=None, closing_pr=None):
    """Write an acceptance-suite result back into the backlog. The round-ending mechanism.

    This is the whole mechanical answer to "have the builders fixed what the testers asked
    for?". A need is closed **iff its acceptance probe, which FAILED when the need was filed,
    now PASSES on a freshly built main** -- there is no judgment in it, and nothing else may
    set `closed`.

    The mirror is just as load-bearing: a need that WAS closed and whose acceptance fails
    again becomes `regressed`, which rank_score puts at the front of the queue. Without it the
    backlog would rot silently, because a merged fix that a later PR undoes would stay marked
    closed forever.

    Returns {"closed": [...], "regressed": [...], "unchanged": [...]}.
    """
    results = suite.get("results") or suite.get("needs") or []
    out = {"closed": [], "regressed": [], "unchanged": []}
    sha = str(suite.get("sha") or "?")[:12]
    for row in results:
        nid = row.get("need_id")
        if not nid:
            continue
        need = load(nid)
        if need is None:
            continue
        passed = bool(row.get("passed"))
        # An unrunnable or flaky replay is INDETERMINATE, not a failure. Treating it as one
        # would mark a healthy closed need `regressed`, and rank_score sends a regression
        # straight to the front of the builder queue -- so a flaky probe could starve the
        # backlog with phantom work.
        if row.get("unrunnable") or row.get("flaky") or row.get("error"):
            out["unchanged"].append(nid)
            continue
        if passed and need.status not in ("closed", "rejected"):
            need.set("status", "closed")
            if round_n is not None:
                need.set("closed_in_round", int(round_n))
            if closing_pr:
                need.set("closing_pr", closing_pr)
            need.log("- closed: acceptance %s now PASSES at %s" % (need.acceptance_id or "?", sha))
            upsert(need)
            out["closed"].append(nid)
        elif not passed and need.status == "closed":
            need.set("status", "regressed")
            # closed_in_round is kept on purpose: "this shipped in round 1 and broke in
            # round 2" is the useful record, and clearing it would lose that.
            need.log("- REGRESSED: acceptance %s fails again at %s" % (need.acceptance_id or "?", sha))
            upsert(need)
            out["regressed"].append(nid)
        else:
            out["unchanged"].append(nid)
    return out


def reject(need_id, reason):
    """Move a need out of the backlog into docs/re-needs/rejected/ with its reason recorded.

    The rejected pile is the honest denominator -- `already-supported` filings in particular
    are the evidence that the two-arm gate is doing its job -- so this never deletes anything.
    """
    need = load(need_id)
    if need is None:
        raise KeyError(need_id)
    need.set("status", "rejected")
    need.set("reject_reason", reason)
    need.log("- rejected: %s" % reason)
    old = need.path
    new = str(path_for(need.need_id, rejected=True))
    need.path = new
    write(need, new)
    if old and os.path.abspath(old) != os.path.abspath(new) and os.path.exists(old):
        os.remove(old)
    return new


# --- ranking ----------------------------------------------------------------

def rank_score(need):
    """Deterministic priority for one need. Higher is dispatched first.

        raw    = SCORE_SCALE * severity_weight * log1p(instances) * distinct_challenges
        score  = raw / (1 + ATTEMPT_PENALTY * attempts)
        if regression_of:  score = REGRESSION_FLOOR + score * REGRESSION_BOOST

    severity_weight is blocker 3 / major 2 / minor 1. log1p(instances) is why the tenth
    tester to hit the same wall barely moves the needle while the second one moves it a lot --
    breadth is evidence, repetition is not. distinct_challenges (floored at 1, so a need whose
    only witness is an in-repo fixture still ranks) multiplies because a friction that
    reproduces on unrelated binaries is a property of kuna, not of one crackme. The attempt
    penalty demotes a need two builders have already failed on rather than looping on it, and
    the additive REGRESSION_FLOOR makes any regression outrank every fresh need outright:
    breaking something that was shipped and verified is the worst thing this loop can do.

    No wall-clock, no randomness, no LLM: the same backlog always produces the same order.
    """
    weight = SEVERITY_WEIGHT.get(str(need.severity), 1.0)
    instances = max(0, int(need.instances or 0))
    challenges = max(1, len(set(need.challenges or [])))
    attempts = max(0, int(need.attempts or 0))

    score = SCORE_SCALE * weight * math.log1p(instances) * challenges
    score /= 1.0 + ATTEMPT_PENALTY * attempts
    # Both regression paths get the floor: `regression_of` is a tester noticing that a shipped
    # capability broke, and `status == regressed` is apply_acceptance noticing the same thing
    # mechanically when a closed need's acceptance flips back. Boosting only the first would
    # leave the pipeline's own detection ranking below fresh work.
    if need.regression_of or need.status == "regressed":
        score = REGRESSION_FLOOR + score * REGRESSION_BOOST
    return round(score, 6)


def ranked(needs=None, statuses=None):
    """Needs sorted by descending score, ties broken by need_id so the order is total."""
    pool = list(needs if needs is not None else load_all())
    if statuses is not None:
        pool = [n for n in pool if n.status in statuses]
    pool.sort(key=lambda n: (-rank_score(n), str(n.need_id)))
    return pool


# --- the select.py seam -----------------------------------------------------

def _opportunity_row(need, index):
    probe = need.probe() or {}
    acceptance = need.acceptance() or {}
    target = (probe.get("target") or acceptance.get("target") or {})
    kinds = [need.track, "re-need"]
    if probe.get("kind") and probe["kind"] not in kinds:
        kinds.append(probe["kind"])
    if need.regression_of:
        kinds.append("regression")

    reasons = [str(need.title or need.need_id),
               "%s severity; %d instance(s) over %d challenge(s)" % (
                   need.severity, int(need.instances or 0), len(set(need.challenges or []))),
               "probe %s PASSES / acceptance %s FAILS" % (need.probe_id, need.acceptance_id)]
    if need.regression_of:
        reasons.append("REGRESSION of %s" % need.regression_of)

    return OrderedDict([
        # --- the fields scripts/pipeline/select.py reads, in its own vocabulary -------
        ("rank", index),
        ("score", rank_score(need)),
        ("kinds", kinds),
        ("reasons", reasons),
        ("covered_hint", need.covered_by_option or ""),
        ("comparable", True),
        ("test_name", need.need_id),
        ("selector", need.acceptance_id),
        ("binary", target.get("binary_rel") or ""),
        ("arch", None),
        ("func_name", None),
        ("func_addr", None),
        ("custom_options", None),
        ("confidence", need.credibility),
        ("kuna_mode", None),
        ("slug", need.need_id),
        # --- the RE-loop payload the builder prompt renders from ---------------------
        ("need_id", need.need_id),
        ("need_path", str(need.path) if need.path else None),
        ("title", need.title),
        ("track", need.track),
        ("status", need.status),
        ("severity", need.severity),
        ("probe_id", need.probe_id),
        ("acceptance_id", need.acceptance_id),
        ("probe_kind", probe.get("kind")),
        ("hypothesis_status", need.hypothesis_status),
        ("credibility", need.credibility),
        ("instances", need.instances),
        ("challenges", list(need.challenges or [])),
        ("rounds", list(need.rounds or [])),
        ("first_seen_round", need.first_seen_round),
        ("attempts", need.attempts),
        ("touches", list(need.touches or [])),
        ("scope", need.scope),
        ("regression_of", need.regression_of),
    ])


def to_opportunities(needs=None, statuses=DISPATCHABLE):
    """The backlog in docs/improvement-pipeline/opportunities.json's exact shape.

    select.py requires `comparable` truthy and `score >= --min-score`, keys the claim on
    `test_name::selector` and derives the branch slug from `test_name` -- so a need's id is its
    test_name and its acceptance id is its selector, making OPP_ID `<need_id>::<acceptance_id>`:
    re-filing the same need against a NEW acceptance is a new unit of work, which is the
    behaviour we want. `kinds` carries the track, so `needs list --track <t>` (scripts/pipeline/select.py's --kind filter keys off the `kinds` column this module emits into opportunities.json) (whose filter is
    already generic) partitions the backlog by builder track with no change to that module.
    """
    rows = [_opportunity_row(n, i)
            for i, n in enumerate(ranked(needs, statuses=statuses), 1)]
    return OrderedDict([
        ("schema", "re-opportunities/1"),
        ("generated_by", "scripts.repipe.needs"),
        ("note", "RE-friction backlog. Every row is a need whose probe PASSES and whose "
                 "acceptance FAILS on the pinned main. Consumed by scripts.pipeline.select."),
        ("count", len(rows)),
        ("ranked", rows),
    ])


# --- the index --------------------------------------------------------------

_INDEX_KEYS = ["need_id", "title", "track", "status", "severity", "probe_id", "acceptance_id",
               "hypothesis_status", "credibility", "instances", "challenges", "rounds",
               "first_seen_round", "attempts", "covered_by_option", "touches", "scope",
               "regression_of", "pr", "closed_in_round", "closing_pr", "reject_reason"]


def _index_row(need):
    row = OrderedDict((k, need.get(k)) for k in _INDEX_KEYS)
    row["score"] = rank_score(need)
    row["path"] = os.path.relpath(str(need.path), str(config.repo_root())) if need.path else None
    return row


def reindex(path=None):
    """Rebuild docs/re-needs/index.json from the records and return it.

    A cache, never a source of truth -- delete it and this rebuilds it. It carries no timestamp
    on purpose: the file is committed every round, and a clock in it would make every round's
    diff noisy even when no need changed.
    """
    active = ranked(load_all(include_rejected=False), statuses=None)
    rejected = [n for n in load_all(include_rejected=True)
                if n.path and os.path.dirname(str(n.path)) == str(config.rejected_dir())]
    by = {}
    for key in ("status", "track", "severity"):
        counts = {}
        for n in active:
            counts[str(n.get(key))] = counts.get(str(n.get(key)), 0) + 1
        by[key] = OrderedDict(sorted(counts.items()))
    reasons = {}
    for n in rejected:
        reasons[str(n.reject_reason)] = reasons.get(str(n.reject_reason), 0) + 1

    doc = OrderedDict([
        ("schema", "re-needs-index/1"),
        ("generated_by", "scripts.repipe.needs"),
        ("count", len(active)),
        ("rejected_count", len(rejected)),
        ("by_status", by["status"]),
        ("by_track", by["track"]),
        ("by_severity", by["severity"]),
        ("by_reject_reason", OrderedDict(sorted(reasons.items()))),
        ("needs", [_index_row(n) for n in active]),
        ("rejected", [_index_row(n) for n in sorted(rejected, key=lambda n: str(n.need_id))]),
    ])
    target = str(path or (config.needs_dir() / "index.json"))
    _write_json(target, doc)
    return doc


def _write_json(target, doc):
    os.makedirs(os.path.dirname(target) or ".", exist_ok=True)
    tmp = target + ".tmp"
    with open(tmp, "w", encoding="utf-8") as fh:
        json.dump(doc, fh, indent=2)
        fh.write("\n")
    os.replace(tmp, target)
    return target


# --- CLI --------------------------------------------------------------------

def _print_table(needs):
    if not needs:
        print("no needs")
        return
    width = max(len(str(n.need_id)) for n in needs)
    print("%-*s  %-8s  %-10s  %-7s  %8s  %s" % (width, "NEED", "TRACK", "STATUS", "SEV",
                                                "SCORE", "TITLE"))
    for n in needs:
        print("%-*s  %-8s  %-10s  %-7s  %8.2f  %s" % (
            width, n.need_id, n.track, n.status, n.severity, rank_score(n),
            (n.title or "")[:72]))


def main(argv=None):
    ap = argparse.ArgumentParser(prog="python3 -m scripts.repipe.needs",
                                 description=__doc__.splitlines()[0])
    sub = ap.add_subparsers(dest="cmd")

    p_list = sub.add_parser("list", help="the backlog, highest-ranked first")
    p_list.add_argument("--status", choices=STATUSES)
    p_list.add_argument("--track", choices=TRACKS)
    p_list.add_argument("--severity", choices=SEVERITIES)
    p_list.add_argument("--include-rejected", action="store_true")
    p_list.add_argument("--json", action="store_true")

    p_show = sub.add_parser("show", help="one record (raw markdown, or --json)")
    p_show.add_argument("need_id")
    p_show.add_argument("--json", action="store_true")

    p_reindex = sub.add_parser("reindex", help="rebuild docs/re-needs/index.json")
    p_reindex.add_argument("--out")
    p_reindex.add_argument("--json", action="store_true")

    p_rank = sub.add_parser("rank", help="dispatchable needs in dispatch order")
    p_rank.add_argument("--all", action="store_true", help="include non-dispatchable statuses")
    p_rank.add_argument("--json", action="store_true")

    p_opp = sub.add_parser("opportunities",
                           help="emit the backlog in opportunities.json shape for select.py")
    p_opp.add_argument("--out", help="default docs/re-needs/opportunities.json; '-' for stdout")
    p_opp.add_argument("--all", action="store_true")
    p_opp.add_argument("--json", action="store_true", help="also print the payload")

    p_acc = sub.add_parser("apply-acceptance",
                           help="close/regress needs from an acceptance-suite result")
    p_acc.add_argument("suite", help="the suite JSON, or - for stdin")
    p_acc.add_argument("--round", type=int, default=None)
    p_acc.add_argument("--pr", default=None)
    p_acc.add_argument("--json", action="store_true")

    p_rej = sub.add_parser("reject", help="move a need to docs/re-needs/rejected/")
    p_rej.add_argument("need_id")
    p_rej.add_argument("--reason", required=True)
    p_rej.add_argument("--json", action="store_true")

    args = ap.parse_args(argv)
    if not args.cmd:
        ap.print_help()
        return 2

    if args.cmd == "list":
        pool = ranked(load_all(include_rejected=args.include_rejected), statuses=None)
        for key in ("status", "track", "severity"):
            want = getattr(args, key)
            if want:
                pool = [n for n in pool if n.get(key) == want]
        if args.json:
            print(json.dumps({"count": len(pool), "needs": [n.to_dict() for n in pool]}, indent=2))
        else:
            _print_table(pool)
        return 0

    if args.cmd == "show":
        need = load(args.need_id)
        if need is None:
            print("no such need: %s" % args.need_id, file=sys.stderr)
            return 1
        if args.json:
            doc = need.to_dict()
            doc["sections"] = need.sections
            doc["probe"] = need.probe()
            doc["acceptance"] = need.acceptance()
            print(json.dumps(doc, indent=2))
        else:
            sys.stdout.write(render(need))
        return 0

    if args.cmd == "reindex":
        doc = reindex(args.out)
        if args.json:
            print(json.dumps(doc, indent=2))
        else:
            print("indexed %d needs (%d rejected) -> %s"
                  % (doc["count"], doc["rejected_count"],
                     args.out or (config.needs_dir() / "index.json")))
        return 0

    if args.cmd == "rank":
        pool = ranked(statuses=None if args.all else DISPATCHABLE)
        if args.json:
            print(json.dumps([n.to_dict() for n in pool], indent=2))
        else:
            _print_table(pool)
        return 0

    if args.cmd == "opportunities":
        doc = to_opportunities(statuses=None if args.all else DISPATCHABLE)
        if args.out == "-":
            print(json.dumps(doc, indent=2))
            return 0
        target = _write_json(args.out or str(config.needs_dir() / "opportunities.json"), doc)
        if args.json:
            print(json.dumps(doc, indent=2))
        else:
            print("wrote %d opportunities -> %s" % (doc["count"], target))
        return 0

    if args.cmd == "apply-acceptance":
        raw = sys.stdin.read() if args.suite == "-" else open(args.suite).read()
        out = apply_acceptance(json.loads(raw), round_n=args.round, closing_pr=args.pr)
        reindex()
        if args.json:
            print(json.dumps(out, indent=2))
        else:
            for kind in ("closed", "regressed"):
                for nid in out[kind]:
                    print("%-10s %s" % (kind, nid))
            print("unchanged: %d" % len(out["unchanged"]))
        return 0

    if args.cmd == "reject":
        try:
            new = reject(args.need_id, args.reason)
        except KeyError:
            print("no such need: %s" % args.need_id, file=sys.stderr)
            return 1
        if args.json:
            print(json.dumps({"need_id": args.need_id, "reason": args.reason, "path": new}))
        else:
            print("rejected %s (%s) -> %s" % (args.need_id, args.reason, new))
        return 0

    ap.print_help()
    return 2


if __name__ == "__main__":
    sys.exit(main())
