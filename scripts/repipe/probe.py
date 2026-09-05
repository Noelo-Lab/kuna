"""The predicate evaluator: run one probe and decide whether its assertion holds.

A *probe* is an executable assertion about kuna's behaviour
(``tools/repipe/schema/probe.schema.json``). Two of them define a need: the **probe** asserts
the CURRENT BAD behaviour and must PASS at filing time; the **acceptance** asserts the DESIRED
behaviour and must FAIL. This module is the atom the rest of the loop rests on -- ``verify.py``'s
two-arm gate, the builder's definition of done, the dedup key, and the acceptance probes promoted
verbatim into ``tests/cli/`` all reduce to ``check()`` returning the right boolean -- so it is
written to be strict rather than forgiving: an assertion that cannot be evaluated FAILS, it never
passes by default.

Three rules the rest of the pipeline depends on:

* **Every present clause must hold, on every repeat.** ``expect`` is a conjunction, and silence is
  not assent: an empty or all-absent ``expect`` is rejected by :func:`normalize`, because a probe
  that asserts nothing always passes and is therefore not evidence.
* **Disagreement is not evidence either.** The ``repeat`` runs are evaluated individually; if they
  do not all reach the same verdict the probe is ``flaky: true`` (and therefore ``passed: false``,
  since a passing verdict requires every run to satisfy every clause). Callers must surface flaky
  separately from an honest failure -- "the gate could not decide" is a different fact from
  "the behaviour is not there".
* **An observation is always produced.** A timeout, or a failure to even exec the command, is
  recorded as a run rather than raised: it lands as a failing synthetic ``run`` clause, so
  "kuna hung for 300 s" is data the captain can read instead of a crashed gate. The one thing
  that does raise is a *pre-flight* problem -- a malformed probe, or a ``{{BIN}}`` whose sha256
  is not the binary the probe was written against -- because that is "unrunnable", not "not
  reproducible", and the two must never be confused.

Resource accounting uses ``/usr/bin/time -v``, not ``resource.getrusage(RUSAGE_CHILDREN)``.
The getrusage route is wrong for exactly the number these probes care about: ``ru_maxrss`` under
RUSAGE_CHILDREN is a **high-water mark over every child this process has ever reaped**, not an
additive counter, so a before/after subtraction yields 0 for any run smaller than an earlier one
-- and probes run in a ``repeat`` loop by construction, next to a captain that has already reaped
cargo. ``/usr/bin/time -v -o FILE`` measures one child in isolation and writes its report to FILE
instead of into the probe's captured stderr; its ``Exit status:`` / ``Command terminated by
signal`` lines are parsed back so a killed child reports ``-SIGNAL`` the way ``subprocess`` would
(time(1) itself would report 128+N). Wall time comes from ``time.monotonic()`` around the whole
spawn, not from time(1)'s centisecond field, which is too coarse for a sub-100 ms kuna call. When
``/usr/bin/time`` is absent, ``max_rss_kb`` is None and ``rusage_source`` says so, rather than a
silently wrong getrusage delta.

Substitution happens on ``cmd``, ``cwd`` and ``env`` only (never on ``stdin``), and the probe_id
is derived from the *unsubstituted* cmd so the same probe keeps its identity in every work dir.

CLI::

    python3 -m scripts.repipe.probe check    <probe.json> [--work DIR] [--bin PATH] [--json]
    python3 -m scripts.repipe.probe validate <probe.json> [--json]
    python3 -m scripts.repipe.probe id       <probe.json> [--acceptance]

``check`` exits 0 when the probe passed, 1 when it failed, 2 when it was structurally unusable.
"""
from __future__ import annotations

import argparse
import copy
import hashlib
import json
import os
import re
import shutil
import threading
import signal
import statistics
import subprocess
import sys
import tempfile
import time

from . import config

SCHEMA_ID = "re-probe/1"
TIME_BIN = "/usr/bin/time"
HEAD_CHARS = 2000
TERM_GRACE_S = 2.0

TOKENS = ("KUNA", "BIN", "SPECS", "WORK", "TMP")

_KINDS = ("cli", "cli-pair", "timing", "memory", "absence")
_BINARY_SOURCES = ("dataset", "in-repo", "synthesized")
_SELECTOR_KINDS = ("name", "addr", "none")
_STATS = ("median", "min", "max", "mean")
_NUM_OPS = ("eq", "ne", "lt", "gt", "le", "ge", "in")
_ORDER_OPS = ("lt", "gt", "le", "ge")
_JSON_OPS = ("eq", "ne", "lt", "gt", "le", "ge", "len_eq", "len_lt", "len_gt",
             "contains", "not_contains", "exists", "absent", "matches")
_JSON_OPS_NO_VALUE = ("exists", "absent")
# Operators whose meaning under `[*]` is "for EVERY branch", not "for some branch".
_JSON_OPS_NEGATIVE = ("absent", "not_contains", "ne")
_JSON_OPS_NUMERIC = ("lt", "gt", "le", "ge", "len_eq", "len_lt", "len_gt")
_REGEX_LISTS = ("stdout_matches", "stdout_absent", "stderr_matches", "stderr_absent")
_STAT_CLAUSES = ("wall_ms", "max_rss_kb")
_EXPECT_KEYS = ("exit_code", "stdout_matches", "stdout_absent", "stderr_matches",
                "stderr_absent", "stdout_is_json", "json", "wall_ms", "max_rss_kb",
                "stdout_bytes")
_PROBE_KEYS = ("schema", "probe_id", "kind", "cmd", "cwd", "env", "stdin", "timeout_s",
               "repeat", "target", "expect", "notes")
_TARGET_KEYS = ("binary_rel", "binary_sha256", "binary_size", "binary_source",
                "in_repo_path", "selector", "selector_kind")

_SHA256_RE = re.compile(r"^[0-9a-f]{64}$")
_PROBE_ID_RE = re.compile(r"^[pa]-[0-9a-f]{12}$")
_TOKEN_RE = re.compile(r"\{\{([A-Za-z_][A-Za-z0-9_]*)\}\}")
_RE_MAXRSS = re.compile(r"Maximum resident set size \(kbytes\):\s*(\d+)")
_RE_EXIT = re.compile(r"^\s*Exit status:\s*(\d+)\s*$", re.MULTILINE)
_RE_SIGNAL = re.compile(r"^Command terminated by signal (\d+)", re.MULTILINE)

MISSING = object()


class ProbeError(ValueError):
    """A probe that is structurally unusable: it cannot be run, or it asserts nothing."""


# --- identity ---------------------------------------------------------------

def probe_id(probe, is_acceptance=False):
    """Derive the probe id: 'p-'/'a-' + sha1(canonical(cmd)+canonical(expect))[:12].

    Derived, never authored. Hashed over the raw cmd with its ``{{TOKENS}}`` intact, so the same
    assertion has the same id in an arena, a scratch checkout and ``tests/cli/``; that identity is
    what dedup and the acceptance matrix key off.
    """
    if not isinstance(probe, dict) or "cmd" not in probe or "expect" not in probe:
        raise ProbeError("probe_id needs both 'cmd' and 'expect'")
    body = json.dumps({"cmd": probe["cmd"], "expect": probe["expect"]},
                      sort_keys=True, separators=(",", ":")).encode()
    return ("a-" if is_acceptance else "p-") + hashlib.sha1(body).hexdigest()[:12]


# --- validation -------------------------------------------------------------

def _is_int(v):
    return isinstance(v, int) and not isinstance(v, bool)


def _is_num(v):
    return isinstance(v, (int, float)) and not isinstance(v, bool)


def _unknown_keys(node, allowed, where, errs):
    for k in sorted(node):
        if k not in allowed:
            errs.append("%s: unknown key %r (additionalProperties is false)" % (where, k))


def _check_regex(pattern, where, errs):
    if not isinstance(pattern, str):
        errs.append("%s: must be a string regex" % where)
        return
    try:
        re.compile(pattern)
    except re.error as exc:
        errs.append("%s: not a valid Python regex (%s)" % (where, exc))


def _validate_numpred(node, where, errs):
    if not isinstance(node, dict):
        errs.append("%s: must be an object" % where)
        return
    if not node:
        errs.append("%s: must have at least one of %s" % (where, "/".join(_NUM_OPS)))
    _unknown_keys(node, _NUM_OPS, where, errs)
    for op in _ORDER_OPS:
        if op in node and not _is_num(node[op]):
            errs.append("%s.%s: must be a number" % (where, op))
    if "in" in node and not isinstance(node["in"], list):
        errs.append("%s.in: must be an array" % where)


def _validate_statpred(node, where, errs):
    """Stricter than the schema on purpose: a statpred with no bound asserts nothing."""
    if not isinstance(node, dict):
        errs.append("%s: must be an object" % where)
        return
    _unknown_keys(node, ("stat", "lt", "gt", "rel_to", "ratio_lt", "ratio_gt"), where, errs)
    if node.get("stat") not in _STATS:
        errs.append("%s.stat: required, one of %s" % (where, "|".join(_STATS)))
    for key in ("lt", "gt", "ratio_lt", "ratio_gt"):
        if key in node and not _is_num(node[key]):
            errs.append("%s.%s: must be a number" % (where, key))
    if "rel_to" in node and node["rel_to"] is not None and not isinstance(node["rel_to"], str):
        errs.append("%s.rel_to: must be a probe id string or null" % where)
    bounds = [k for k in ("lt", "gt", "ratio_lt", "ratio_gt") if k in node]
    if not bounds:
        errs.append("%s: needs at least one of lt/gt/ratio_lt/ratio_gt, else it asserts nothing"
                    % where)
    if ("ratio_lt" in node or "ratio_gt" in node) and not node.get("rel_to"):
        errs.append("%s: ratio_lt/ratio_gt require rel_to (the baseline probe id)" % where)


def _validate_jsonpred(node, where, errs):
    if not isinstance(node, dict):
        errs.append("%s: must be an object" % where)
        return
    _unknown_keys(node, ("path", "op", "value"), where, errs)
    path = node.get("path")
    if not isinstance(path, str) or not path.strip():
        errs.append("%s.path: required, a non-empty dotted path" % where)
    else:
        try:
            parse_path(path)
        except ProbeError as exc:
            errs.append("%s.path: %s" % (where, exc))
    op = node.get("op")
    if op not in _JSON_OPS:
        errs.append("%s.op: required, one of %s" % (where, "|".join(_JSON_OPS)))
        return
    if op in _JSON_OPS_NO_VALUE:
        return
    if "value" not in node:
        errs.append("%s.value: required for op %r" % (where, op))
        return
    if op in _JSON_OPS_NUMERIC and not _is_num(node["value"]):
        errs.append("%s.value: must be a number for op %r" % (where, op))
    if op == "matches":
        _check_regex(node["value"], where + ".value", errs)


def _validate_expect(node, errs):
    if not isinstance(node, dict):
        errs.append("expect: must be an object")
        return
    _unknown_keys(node, _EXPECT_KEYS, "expect", errs)
    for key in ("exit_code", "stdout_bytes"):
        if key in node:
            _validate_numpred(node[key], "expect." + key, errs)
    for key in _REGEX_LISTS:
        if key in node:
            if not isinstance(node[key], list):
                errs.append("expect.%s: must be an array of regex strings" % key)
                continue
            for i, pat in enumerate(node[key]):
                _check_regex(pat, "expect.%s[%d]" % (key, i), errs)
    if "stdout_is_json" in node and not isinstance(node["stdout_is_json"], bool):
        errs.append("expect.stdout_is_json: must be a boolean")
    if "json" in node:
        if not isinstance(node["json"], list):
            errs.append("expect.json: must be an array of json predicates")
        else:
            for i, jp in enumerate(node["json"]):
                _validate_jsonpred(jp, "expect.json[%d]" % i, errs)
    for key in _STAT_CLAUSES:
        if key in node:
            _validate_statpred(node[key], "expect." + key, errs)


def _validate_target(node, errs):
    if not isinstance(node, dict):
        errs.append("target: must be an object")
        return
    _unknown_keys(node, _TARGET_KEYS, "target", errs)
    for key in ("binary_rel", "binary_sha256", "binary_size"):
        if key not in node:
            errs.append("target.%s: required" % key)
    if "binary_rel" in node and not isinstance(node["binary_rel"], str):
        errs.append("target.binary_rel: must be a string")
    sha = node.get("binary_sha256")
    if "binary_sha256" in node and (not isinstance(sha, str) or not _SHA256_RE.match(sha)):
        errs.append("target.binary_sha256: must be 64 lowercase hex chars")
    if "binary_size" in node and not _is_int(node["binary_size"]):
        errs.append("target.binary_size: must be an integer")
    if "binary_source" in node and node["binary_source"] not in _BINARY_SOURCES:
        errs.append("target.binary_source: must be one of %s" % "|".join(_BINARY_SOURCES))
    if "selector_kind" in node and node["selector_kind"] not in _SELECTOR_KINDS:
        errs.append("target.selector_kind: must be one of %s" % "|".join(_SELECTOR_KINDS))
    for key in ("in_repo_path", "selector"):
        if key in node and node[key] is not None and not isinstance(node[key], str):
            errs.append("target.%s: must be a string or null" % key)


def validate(probe):
    """Structural errors against tools/repipe/schema/probe.schema.json, as a list of strings.

    Hand-written because kuna's Python tooling is stdlib-only (no jsonschema anywhere on this
    machine). Empty list means valid. Two checks are deliberately *stricter* than the JSON Schema,
    which cannot express them: every regex clause must actually compile, and a statpred must carry
    a bound (a statpred with only ``stat`` asserts nothing, which is the failure mode this whole
    module exists to prevent).
    """
    errs = []
    if not isinstance(probe, dict):
        return ["probe: must be a JSON object"]
    _unknown_keys(probe, _PROBE_KEYS, "probe", errs)
    for key in ("schema", "kind", "cmd", "timeout_s", "expect"):
        if key not in probe:
            errs.append("%s: required" % key)
    if "schema" in probe and probe["schema"] != SCHEMA_ID:
        errs.append("schema: must be %r" % SCHEMA_ID)
    if "probe_id" in probe:
        pid = probe["probe_id"]
        if not isinstance(pid, str) or not _PROBE_ID_RE.match(pid):
            errs.append("probe_id: must match ^[pa]-[0-9a-f]{12}$")
    if "kind" in probe and probe["kind"] not in _KINDS:
        errs.append("kind: must be one of %s" % "|".join(_KINDS))
    cmd = probe.get("cmd")
    if "cmd" in probe:
        if not isinstance(cmd, list) or not cmd:
            errs.append("cmd: must be a non-empty array of strings")
        elif not all(isinstance(x, str) for x in cmd):
            errs.append("cmd: every element must be a string")
    if "cwd" in probe and not isinstance(probe["cwd"], str):
        errs.append("cwd: must be a string")
    if "env" in probe:
        env = probe["env"]
        if not isinstance(env, dict):
            errs.append("env: must be an object of string->string")
        else:
            for k, v in env.items():
                if not isinstance(v, str):
                    errs.append("env.%s: must be a string" % k)
    if "stdin" in probe and probe["stdin"] is not None and not isinstance(probe["stdin"], str):
        errs.append("stdin: must be a string or null")
    if "timeout_s" in probe:
        t = probe["timeout_s"]
        if not _is_int(t) or not (1 <= t <= 1800):
            errs.append("timeout_s: must be an integer 1..1800")
    if "repeat" in probe:
        r = probe["repeat"]
        if not _is_int(r) or not (1 <= r <= 11):
            errs.append("repeat: must be an integer 1..11")
    if "target" in probe:
        _validate_target(probe["target"], errs)
    if "notes" in probe:
        if not isinstance(probe["notes"], str):
            errs.append("notes: must be a string")
        elif len(probe["notes"]) > 400:
            errs.append("notes: must be at most 400 characters")
    if "expect" in probe:
        _validate_expect(probe["expect"], errs)
    return errs


def asserted_clause_count(expect):
    """How many clauses actually assert something. Zero means the probe is not evidence."""
    if not isinstance(expect, dict):
        return 0
    n = 0
    for key in ("exit_code", "stdout_bytes"):
        if isinstance(expect.get(key), dict) and expect[key]:
            n += 1
    for key in _REGEX_LISTS:
        n += len(expect.get(key) or [])
    if isinstance(expect.get("stdout_is_json"), bool):
        n += 1
    n += len(expect.get("json") or [])
    for key in _STAT_CLAUSES:
        sp = expect.get(key)
        if isinstance(sp, dict) and any(b in sp for b in ("lt", "gt", "ratio_lt", "ratio_gt")):
            n += 1
    return n


# Clause names an author reaches for that are not the schema's. A probe is written by an LLM
# under time pressure and `"stdout": "no functions"` is the obvious way to say what it means;
# rejecting it discards a real bug report over a synonym. In round 1 this silently made 13 of
# 16 observations unrunnable -- 80% of a round's evidence -- so the evaluator canonicalises
# instead of refusing. Only unambiguous synonyms are mapped; anything else still errors.
_EXPECT_ALIASES = {
    "stdout": "stdout_matches",
    "stdout_contains": "stdout_matches",
    "stdout_matches_all": "stdout_matches",
    "stderr": "stderr_matches",
    "stderr_contains": "stderr_matches",
    "stdout_not_contains": "stdout_absent",
    "stderr_not_contains": "stderr_absent",
    "exit": "exit_code",
    "returncode": "exit_code",
    "is_json": "stdout_is_json",
}
_REGEX_CLAUSES = ("stdout_matches", "stdout_absent", "stderr_matches", "stderr_absent")


def canonical_expect(expect):
    """Rewrite an `expect` block's near-miss clause names to the schema's, non-destructively.

    A bare string where the schema wants a list is wrapped, and a bare int where it wants a
    numpred becomes ``{"eq": n}`` -- both are the same class of mistake. A clause that would
    collide with one the author already wrote is left alone rather than silently merged.
    """
    if not isinstance(expect, dict):
        return expect
    out = {}
    for key, value in expect.items():
        target = _EXPECT_ALIASES.get(key, key)
        if target != key and target in expect:
            target = key                      # author wrote both; do not merge, let validate speak

        # The nested form -- {"stdout": {"contains": "switch(0)"}} -- is the commonest shape a
        # tester reaches for, and it decides between two DIFFERENT clauses. `contains` is a
        # LITERAL and must be escaped: mapped naively, `switch(0)` becomes a regex whose
        # parentheses are a capture group and it would match "switch0" instead, turning a
        # correct bug report into a wrong verdict. That is worse than rejecting it.
        if target in ("stdout_matches", "stderr_matches") and isinstance(value, dict):
            stream = target.split("_")[0]
            for op, operand in value.items():
                items = operand if isinstance(operand, list) else [operand]
                items = [x for x in items if isinstance(x, str)]
                if not items:
                    continue
                if op in ("contains", "includes", "has"):
                    out.setdefault(stream + "_matches", []).extend(re.escape(x) for x in items)
                elif op in ("not_contains", "excludes", "absent"):
                    out.setdefault(stream + "_absent", []).extend(re.escape(x) for x in items)
                elif op in ("matches", "regex", "re"):
                    out.setdefault(stream + "_matches", []).extend(items)
                elif op in ("equals", "eq"):
                    out.setdefault(stream + "_matches",
                                   []).extend(r"\A%s\Z" % re.escape(x) for x in items)
                else:
                    out[target] = value       # unknown op: leave it for validate to reject
            continue

        if target in _REGEX_CLAUSES and isinstance(value, str):
            value = [value]
        elif target in ("exit_code", "stdout_bytes") and isinstance(value, (int, float)) \
                and not isinstance(value, bool):
            value = {"eq": value}
        out[target] = value
    return out


def normalize(probe, is_acceptance=False):
    """Fill defaults, reject a structurally unusable probe, and stamp the derived probe_id.

    Defaults are filled *before* validation so a hand-written probe may omit ``schema``, ``kind``
    (-> "cli"), ``cwd`` (-> "{{WORK}}") and ``repeat`` (-> 1); everything else the schema requires
    must be present. Raises :class:`ProbeError` on any structural error, and -- the check the whole
    two-arm gate leans on -- on an ``expect`` that is empty or whose clauses are all absent/empty:
    a probe that asserts nothing always passes and is not evidence.

    ``is_acceptance`` picks the id prefix. A probe already carrying an ``a-`` id is treated as an
    acceptance even when the caller did not say so, so loading a file off disk round-trips; an
    authored id that disagrees with the derived one is an error rather than a silent overwrite,
    because dedup keys off that id. Idempotent: normalizing a normalized probe is a no-op.
    """
    if not isinstance(probe, dict):
        raise ProbeError("probe must be a JSON object, got %s" % type(probe).__name__)
    out = copy.deepcopy(probe)
    out.setdefault("schema", SCHEMA_ID)
    out.setdefault("kind", "cli")
    out.setdefault("cwd", "{{WORK}}")
    out.setdefault("repeat", 1)
    out["expect"] = canonical_expect(out.get("expect"))
    errs = validate(out)
    if errs:
        raise ProbeError("invalid probe: " + "; ".join(errs))
    if asserted_clause_count(out["expect"]) == 0:
        raise ProbeError("expect asserts nothing: a probe with no effective clause always "
                         "passes and is not evidence")
    authored = probe.get("probe_id")
    if not is_acceptance and isinstance(authored, str) and authored.startswith("a-"):
        is_acceptance = True
    pid = probe_id(out, is_acceptance)
    if isinstance(authored, str) and authored and authored != pid:
        raise ProbeError("probe_id %s does not match the derived id %s (it is derived from "
                         "cmd+expect, not authored)" % (authored, pid))
    out["probe_id"] = pid
    return out


# --- substitution -----------------------------------------------------------

def context(work=None, binary=None, tmp=None, baselines=None, kuna=None, specs=None):
    """Build the ctx dict :func:`run` and :func:`substitute` take. Every field is optional."""
    return {"work": work, "bin": binary, "tmp": tmp, "kuna": kuna, "specs": specs,
            "baselines": baselines or {}}


def _token_values(ctx):
    ctx = ctx or {}
    return {
        "KUNA": str(ctx.get("kuna") or config.kuna_bin()),
        "SPECS": str(ctx.get("specs") or config.specs_dir()),
        "WORK": str(ctx["work"]) if ctx.get("work") else os.getcwd(),
        "TMP": str(ctx["tmp"]) if ctx.get("tmp") else None,
        "BIN": str(ctx["bin"]) if ctx.get("bin") else None,
    }


def _subst_str(text, values):
    def repl(m):
        name = m.group(1)
        if name not in values:
            raise ProbeError("unknown substitution token {{%s}} (known: %s)"
                             % (name, " ".join(TOKENS)))
        if values[name] is None:
            raise ProbeError("{{%s}} used but the context supplies no %s"
                             % (name, name.lower()))
        return values[name]
    return _TOKEN_RE.sub(repl, text)


def _subst(value, values):
    if isinstance(value, str):
        return _subst_str(value, values)
    if isinstance(value, (list, tuple)):
        return [_subst(v, values) for v in value]
    if isinstance(value, dict):
        return {k: _subst(v, values) for k, v in value.items()}
    return value


def substitute(value, ctx):
    """Expand {{KUNA}} {{BIN}} {{SPECS}} {{WORK}} {{TMP}} in a string, list or dict of values.

    Non-string leaves pass through untouched. An unknown token, or a known token the context
    cannot fill, raises :class:`ProbeError` -- running a command with a literal ``{{BIN}}`` in its
    argv would produce a confident, meaningless observation. ``{{BIN}}`` is the caller's job to
    supply; :func:`run` derives it from ``target.binary_rel`` inside ``{{WORK}}`` when the context
    leaves it unset.
    """
    return _subst(value, _token_values(ctx))


def resolve(probe, ctx):
    """The substituted ``{cmd, cwd, env}`` a run would use. Handy for logging a transcript."""
    p = normalize(probe)
    return {"cmd": [str(x) for x in substitute(p["cmd"], ctx)],
            "cwd": str(substitute(p.get("cwd") or "{{WORK}}", ctx)),
            "env": substitute(p.get("env") or {}, ctx)}


# --- execution --------------------------------------------------------------

def _parse_time_report(path):
    """Pull max RSS and the true child status out of a /usr/bin/time -v report file."""
    try:
        with open(path, "r", errors="replace") as fh:
            text = fh.read()
    except OSError:
        return None
    if not text.strip():
        return None
    out = {"max_rss_kb": None, "exit_code": None, "signal": None}
    m = _RE_MAXRSS.search(text)
    if m:
        out["max_rss_kb"] = int(m.group(1))
    m = _RE_SIGNAL.search(text)
    if m:
        out["signal"] = int(m.group(1))
        out["exit_code"] = -int(m.group(1))
        return out
    m = _RE_EXIT.search(text)
    if m:
        out["exit_code"] = int(m.group(1))
    return out


def _kill_group(proc):
    try:
        pgid = os.getpgid(proc.pid)
    except OSError:
        proc.kill()
        return
    for sig in (signal.SIGTERM, signal.SIGKILL):
        try:
            os.killpg(pgid, sig)
        except OSError:
            return
        try:
            proc.wait(timeout=TERM_GRACE_S)
            return
        except subprocess.TimeoutExpired:
            continue


# --- what a probe is allowed to execute ------------------------------------
#
# A probe's argv is authored by an LLM tester and is replayed by verify.py in the MAIN tree,
# outside the tester's bwrap sandbox. Without a guard, `{"cmd": ["bash", "-c", "..."]}` in a
# tester report is remote code execution against the operator's checkout. So argv[0] is
# checked against an allowlist of read-only analysis tools before anything runs, and the
# interpreters and shells that would launder arbitrary code are denied outright.
#
# A violation raises, which makes the observation `unrunnable` -- exactly the right verdict
# for "this is not a probe".

DEFAULT_ALLOWED = (
    "kuna", "decomp_dbg", "decomp_test_dbg", "slacomp",
    "objdump", "readelf", "strings", "nm", "file", "size", "xxd", "od", "cmp",
    "decompiler",          # declib/IDA, the reference leg
)

# Denied even if someone adds them to the allowlist: each one runs code given as an argument,
# so allowing it would make the allowlist meaningless.
DENIED_ALWAYS = frozenset("""
sh bash dash zsh ksh csh fish env eval exec xargs
python python2 python3 perl ruby node deno bun lua tclsh
make cmake cargo rustc cc gcc clang ld
curl wget nc ncat socat ssh scp rsync git gh sudo doas su
rm mv cp dd chmod chown mkfs mount umount kill pkill systemctl crontab at
""".split())


def allowed_executables():
    """The allowlist, overridable with REPIPE_PROBE_ALLOW (colon-separated basenames)."""
    extra = os.environ.get("REPIPE_PROBE_ALLOW", "")
    names = set(DEFAULT_ALLOWED)
    names.update(x.strip() for x in extra.split(":") if x.strip())
    return names


def _allowed_realpaths():
    """The allowlist resolved to real absolute paths, once.

    Matching on the BASENAME alone is not a control: a tester has workspace-write in its
    arena, so it can drop a shell script called `kuna` there and name it by path. What is
    allowed is a specific FILE, so every candidate is resolved (realpath, defeating symlinks)
    and compared against the resolved allowlist.
    """
    paths = set()
    from . import config as _cfg
    for cand in (_cfg.kuna_bin(),
                 _cfg.repo_root() / "decompiler" / "target" / "release" / "decomp_dbg",
                 _cfg.repo_root() / "decompiler" / "target" / "release" / "decomp_test_dbg",
                 _cfg.repo_root() / "decompiler" / "target" / "release" / "slacomp",
                 _cfg.decompiler_cli()):
        try:
            if os.path.exists(str(cand)):
                paths.add(os.path.realpath(str(cand)))
        except Exception:
            pass
    for name in allowed_executables():
        found = shutil.which(name)
        if found:
            paths.add(os.path.realpath(found))
    for extra in os.environ.get("REPIPE_PROBE_ALLOW_PATH", "").split(":"):
        if extra.strip() and os.path.exists(extra.strip()):
            paths.add(os.path.realpath(extra.strip()))
    return paths


def check_executable(argv, env=None):
    """None if this argv may run, else the reason it may not.

    Two gates, and the second is the real one: the basename must not be an interpreter, and
    the RESOLVED FILE must be on the allowlist. Basename-only matching would let
    `/tmp/anything/kuna` through, which a tester can create.
    """
    if not argv:
        return "empty cmd"
    raw = str(argv[0])
    base = os.path.basename(raw)
    if base in DENIED_ALWAYS:
        return ("probe may not execute %r: it runs code passed as an argument, which would "
                "make the allowlist meaningless" % base)
    path = raw if os.sep in raw else shutil.which(raw, path=(env or os.environ).get("PATH"))
    if not path or not os.path.exists(path):
        return "probe may not execute %r: no such executable" % raw
    real = os.path.realpath(path)
    if real not in _allowed_realpaths():
        return ("probe may not execute %s: it resolves to %s, which is not one of the "
                "allowed analysis tools. Allowed basenames: %s. Extend with "
                "REPIPE_PROBE_ALLOW (name) or REPIPE_PROBE_ALLOW_PATH (absolute path)."
                % (raw, real, ", ".join(sorted(allowed_executables()))))
    return None


def _exec_once(index, argv, cwd, env, stdin_data, timeout_s, use_time):
    """One execution. Never raises: an exec failure or a timeout is recorded, not thrown."""
    rec = {"index": index, "exit_code": None, "signal": None, "stdout": "", "stderr": "",
           "stdout_bytes": 0, "stderr_bytes": 0, "wall_ms": 0.0, "max_rss_kb": None,
           "timed_out": False, "error": None}
    exe = argv[0]
    resolved = exe if os.sep in exe else shutil.which(exe, path=env.get("PATH"))
    if not resolved or not os.path.exists(resolved):
        rec["error"] = "cannot execute %r: no such file" % exe
        return rec
    if not os.access(resolved, os.X_OK):
        rec["error"] = "cannot execute %r: not executable" % resolved
        return rec
    if not os.path.isdir(cwd):
        rec["error"] = "cwd %r is not a directory" % cwd
        return rec

    report_path = None
    wrapped = argv
    if use_time:
        fd, report_path = tempfile.mkstemp(prefix="repipe-rusage-", suffix=".txt")
        os.close(fd)
        wrapped = [TIME_BIN, "-v", "-o", report_path, "--"] + argv

    payload = stdin_data.encode() if isinstance(stdin_data, str) else None
    t0 = time.monotonic()
    try:
        proc = subprocess.Popen(
            wrapped, cwd=cwd, env=env,
            stdin=subprocess.PIPE if payload is not None else subprocess.DEVNULL,
            stdout=subprocess.PIPE, stderr=subprocess.PIPE, start_new_session=True)
    except OSError as exc:
        rec["error"] = "spawn failed: %s" % exc
        rec["wall_ms"] = (time.monotonic() - t0) * 1000.0
        if report_path:
            os.unlink(report_path)
        return rec
    try:
        out, err = proc.communicate(input=payload, timeout=timeout_s)
    except subprocess.TimeoutExpired:
        rec["timed_out"] = True
        _kill_group(proc)
        try:
            out, err = proc.communicate(timeout=30)
        except subprocess.TimeoutExpired:
            out, err = b"", b""
    rec["wall_ms"] = (time.monotonic() - t0) * 1000.0

    # The true byte counts are recorded BEFORE truncation, so a `stdout_bytes` clause still
    # sees reality even when the text is capped; only content past the cap is unavailable,
    # and the observation says so.
    rec["stdout_bytes"] = len(out or b"")
    rec["stderr_bytes"] = len(err or b"")
    rec["stdout"] = (out or b"")[:CAPTURE_MAX_BYTES].decode("utf-8", "replace")
    rec["stderr"] = (err or b"")[:CAPTURE_MAX_BYTES].decode("utf-8", "replace")
    rec["stdout_truncated"] = rec["stdout_bytes"] > CAPTURE_MAX_BYTES
    rec["stderr_truncated"] = rec["stderr_bytes"] > CAPTURE_MAX_BYTES

    report = _parse_time_report(report_path) if report_path else None
    if report_path:
        try:
            os.unlink(report_path)
        except OSError:
            pass
    if report and report["exit_code"] is not None:
        rec["exit_code"] = report["exit_code"]
        rec["signal"] = report["signal"]
        rec["max_rss_kb"] = report["max_rss_kb"]
    else:
        rc = proc.returncode
        rec["exit_code"] = rc
        if rc is not None and rc < 0:
            rec["signal"] = -rc
        if report:
            rec["max_rss_kb"] = report["max_rss_kb"]
    if rec["timed_out"]:
        rec["error"] = "timed out after %ss" % timeout_s
    return rec


def sha256_file(path, chunk=1 << 20):
    h = hashlib.sha256()
    with open(path, "rb") as fh:
        for block in iter(lambda: fh.read(chunk), b""):
            h.update(block)
    return h.hexdigest()


def target_mismatch(probe, ctx):
    """None if the resolved {{BIN}} is the binary the probe was written against.

    Returns a human-readable reason otherwise. Checked before every run so a misresolved
    path becomes `unrunnable` instead of a confident false verdict.
    """
    target = (probe or {}).get("target") or {}
    want = target.get("binary_sha256")
    if not want:
        return None
    path = ctx.get("bin")
    if not path:
        return "target.binary_sha256 given but {{BIN}} could not be resolved"
    if not os.path.exists(str(path)):
        return "target binary not found at %s (is {{WORK}} right?)" % path
    size = target.get("binary_size")
    actual_size = os.path.getsize(str(path))
    if size is not None and int(size) != actual_size:
        return ("target size mismatch at %s: probe expects %s bytes, found %s"
                % (path, size, actual_size))
    got = sha256_file(str(path))
    if got != want:
        return ("target sha256 mismatch at %s: probe expects %s..., found %s..."
                % (path, want[:12], got[:12]))
    return None


def run(probe, ctx=None, timeout=None):
    """Execute the probe ``repeat`` times and return an Observation.

    The Observation records, per run: exit_code, signal, stdout, stderr, stdout_bytes, wall_ms,
    max_rss_kb, timed_out and error. It is a plain dict, JSON-serialisable, and is the only thing
    :func:`evaluate` reads -- ``ctx["baselines"]`` is copied into it so evaluate stays a pure
    function of (probe, observation) and a verdict can be recomputed from a stored observation.

    ``timeout`` overrides the probe's ``timeout_s``. One ``{{TMP}}`` directory is created per
    ``run()`` call and shared by the repeats -- the repeats exist to detect flakiness, so they must
    see identical conditions -- and is removed again unless the caller supplied ``ctx["tmp"]``.
    ``{{BIN}}`` defaults to ``target.binary_rel`` resolved inside ``{{WORK}}`` and is checked
    against ``target.binary_sha256``/``binary_size`` before the first spawn (see
    :func:`target_mismatch`); a mismatch raises rather than producing a confident verdict about
    the wrong file. The child inherits
    this process's environment updated with the probe's ``env``; nothing is scrubbed, because kuna
    needs its ambient SLEIGHHOME/PATH to run at all.

    ``repeat`` is honoured verbatim; the REPLAY_REPS / TIMING_REPS policy lives in verify.py.
    """
    p = normalize(probe)
    ctx = dict(ctx or {})
    owned_tmp = None
    if not ctx.get("tmp"):
        owned_tmp = tempfile.mkdtemp(prefix="repipe-probe-")
        ctx["tmp"] = owned_tmp
    if not ctx.get("bin"):
        target = p.get("target") or {}
        if target.get("binary_rel"):
            ctx["bin"] = os.path.join(str(ctx.get("work") or os.getcwd()),
                                      target["binary_rel"])
    denied = check_executable([str(x) for x in substitute(p["cmd"], ctx)])
    if denied:
        if owned_tmp:
            shutil.rmtree(owned_tmp, ignore_errors=True)
        raise ProbeError(denied)
    mismatch = target_mismatch(p, ctx)
    if mismatch:
        # Refuse rather than run. A probe pointed at the wrong file still produces a
        # verdict, and a FAIL there reads as "not reproducible" -- i.e. it would silently
        # discard a real need because {{WORK}} was wrong. binary_sha256 is in the schema to
        # be checked, not to be decoration.
        if owned_tmp:
            shutil.rmtree(owned_tmp, ignore_errors=True)
        raise ProbeError(mismatch)
    try:
        argv = [str(x) for x in substitute(p["cmd"], ctx)]
        cwd = str(substitute(p.get("cwd") or "{{WORK}}", ctx))
        env_extra = {k: str(v) for k, v in substitute(p.get("env") or {}, ctx).items()}
        env = dict(os.environ)
        env.update(env_extra)
        tmo = int(timeout) if timeout is not None else int(p["timeout_s"])
        reps = int(p.get("repeat") or 1)
        use_time = os.path.exists(TIME_BIN) and os.access(TIME_BIN, os.X_OK)
        obs = {
            "schema": "re-observation/1",
            "probe_id": p["probe_id"],
            "kind": p.get("kind"),
            "cmd": argv,
            "cwd": cwd,
            "env_extra": env_extra,
            "repeat": reps,
            "timeout_s": tmo,
            "rusage_source": "/usr/bin/time -v" if use_time else "unavailable",
            "started_at": time.time(),
            "baselines": ctx.get("baselines") or {},
            "runs": [],
        }
        for i in range(reps):
            obs["runs"].append(_exec_once(i, argv, cwd, env, p.get("stdin"), tmo, use_time))
        obs["duration_s"] = time.time() - obs["started_at"]
        return obs
    finally:
        if owned_tmp:
            shutil.rmtree(owned_tmp, ignore_errors=True)


# --- json paths -------------------------------------------------------------

def parse_path(path):
    """Parse a dotted json path into steps: ('key', name) | ('index', i) | ('any', None)."""
    if not isinstance(path, str) or not path.strip():
        raise ProbeError("json path must be a non-empty string")
    s = path.strip()
    if s.startswith("$"):
        s = s[1:]
    if s.startswith("."):
        s = s[1:]
    steps = []
    i, n = 0, len(s)
    while i < n:
        c = s[i]
        if c == "[":
            j = s.find("]", i)
            if j < 0:
                raise ProbeError("unbalanced '[' in json path %r" % path)
            tok = s[i + 1:j]
            if tok == "*":
                steps.append(("any", None))
            else:
                try:
                    steps.append(("index", int(tok)))
                except ValueError:
                    raise ProbeError("bad index %r in json path %r" % (tok, path))
            i = j + 1
        elif c == ".":
            i += 1
        else:
            j = i
            while j < n and s[j] not in ".[":
                j += 1
            steps.append(("key", s[i:j]))
            i = j
    if not steps:
        raise ProbeError("json path %r resolves to no steps" % path)
    return steps


def resolve_path(doc, steps):
    """Resolve a parsed path to a list of candidate values; MISSING marks a branch that is absent.

    ``[*]`` fans out over a list (or a dict's values); every other step keeps one branch. An empty
    container under ``[*]`` yields a single MISSING, so `exists` is false and `absent` is true.
    """
    cur = [doc]
    for kind, arg in steps:
        nxt = []
        for v in cur:
            if v is MISSING:
                nxt.append(MISSING)
            elif kind == "key":
                nxt.append(v[arg] if isinstance(v, dict) and arg in v else MISSING)
            elif kind == "index":
                if isinstance(v, list) and -len(v) <= arg < len(v):
                    nxt.append(v[arg])
                else:
                    nxt.append(MISSING)
            elif isinstance(v, (list, tuple)):
                nxt.extend(v if v else [MISSING])
            elif isinstance(v, dict):
                nxt.extend(list(v.values()) if v else [MISSING])
            else:
                nxt.append(MISSING)
        cur = nxt
    return cur


def _one_json_op(op, value, want):
    try:
        if op == "eq":
            return value == want
        if op == "ne":
            return value != want
        if op == "lt":
            return value < want
        if op == "gt":
            return value > want
        if op == "le":
            return value <= want
        if op == "ge":
            return value >= want
        if op == "len_eq":
            return len(value) == want
        if op == "len_lt":
            return len(value) < want
        if op == "len_gt":
            return len(value) > want
        if op == "contains":
            return (str(want) in value) if isinstance(value, str) else (want in value)
        if op == "not_contains":
            return not ((str(want) in value) if isinstance(value, str) else (want in value))
        if op == "matches":
            subject = value if isinstance(value, str) else json.dumps(value, sort_keys=True)
            return bool(re.search(str(want), subject, re.MULTILINE))
    except (TypeError, ValueError, re.error):
        return False
    return False


def _shown(values):
    out = ["<absent>" if v is MISSING else v for v in values]
    if len(out) == 1:
        return out[0]
    if len(out) > 6:
        return out[:6] + ["<%d more>" % (len(out) - 6)]
    return out


# --- clause evaluation ------------------------------------------------------

def _numpred_ok(pred, value):
    if value is None:
        return False, None
    ok = True
    for op, want in pred.items():
        try:
            if op == "eq":
                ok = ok and value == want
            elif op == "ne":
                ok = ok and value != want
            elif op == "in":
                ok = ok and value in want
            elif op == "lt":
                ok = ok and value < want
            elif op == "gt":
                ok = ok and value > want
            elif op == "le":
                ok = ok and value <= want
            elif op == "ge":
                ok = ok and value >= want
        except TypeError:
            return False, value
    return ok, value


class RegexTimeout(Exception):
    pass


# Seconds any single regex may spend. Patterns come from an LLM tester and `timeout_s` bounds
# only the SUBPROCESS -- a catastrophic-backtracking pattern like `(a+)+$` against 40 bytes
# runs inside the evaluator and hangs the gate forever, taking the whole round with it.
REGEX_BUDGET_S = float(os.environ.get("REPIPE_REGEX_BUDGET_S", "5"))
# re is applied to whole streams; a huge stdout multiplies any pattern's cost.
REGEX_MAX_BYTES = int(os.environ.get("REPIPE_REGEX_MAX_BYTES", str(4 << 20)))

# How much of a probe's output is kept in memory. `kuna decompile-all --json` on a large
# binary is already tens of MB, and an allowlisted tool pointed at the wrong thing can emit
# far more; with repeat up to 11 and no cap, one bad probe could exhaust the box. The real
# byte count is still measured and asserted on (`stdout_bytes`), so a truncated capture
# never silently changes a verdict about SIZE -- only about content past the cap.
CAPTURE_MAX_BYTES = int(os.environ.get("REPIPE_CAPTURE_MAX_BYTES", str(64 << 20)))


def _search(pattern, text, budget=None):
    """re.search with a wall-clock bound. Returns None on error, timeout or no match.

    The bound uses SIGALRM, which only arms in the main thread; off the main thread the input
    is truncated instead, which caps the cost without pretending to interrupt.
    """
    text = text or ""
    if len(text) > REGEX_MAX_BYTES:
        text = text[:REGEX_MAX_BYTES]
    budget = REGEX_BUDGET_S if budget is None else budget

    def _raise(signum, frame):
        raise RegexTimeout()

    armed = False
    if budget > 0 and threading.current_thread() is threading.main_thread():
        try:
            old_handler = signal.signal(signal.SIGALRM, _raise)
            signal.setitimer(signal.ITIMER_REAL, budget)
            armed = True
        except (ValueError, OSError):
            armed = False
    try:
        return re.search(pattern, text, re.MULTILINE)
    except (re.error, RegexTimeout):
        return None
    finally:
        if armed:
            signal.setitimer(signal.ITIMER_REAL, 0)
            signal.signal(signal.SIGALRM, old_handler)


def _per_run_checkers(expect):
    """(clause, expected, fn(run, parsed_json) -> (ok, actual)) for every non-aggregate clause."""
    checks = []
    if "exit_code" in expect:
        pred = expect["exit_code"]
        checks.append(("exit_code", pred,
                       lambda r, d, pred=pred: _numpred_ok(pred, r.get("exit_code"))))
    if "stdout_bytes" in expect:
        pred = expect["stdout_bytes"]
        checks.append(("stdout_bytes", pred,
                       lambda r, d, pred=pred: _numpred_ok(pred, r.get("stdout_bytes"))))
    for stream in ("stdout", "stderr"):
        for i, pat in enumerate(expect.get(stream + "_matches") or []):
            def _m(r, d, pat=pat, s=stream):
                hit = _search(pat, r.get(s) or "")
                return (bool(hit), hit.group(0)[:200] if hit else "<no match>")
            checks.append(("%s_matches[%d]" % (stream, i), pat, _m))
        for i, pat in enumerate(expect.get(stream + "_absent") or []):
            def _a(r, d, pat=pat, s=stream):
                hit = _search(pat, r.get(s) or "")
                return (hit is None, "<absent>" if hit is None else hit.group(0)[:200])
            checks.append(("%s_absent[%d]" % (stream, i), pat, _a))
    if "stdout_is_json" in expect:
        want = bool(expect["stdout_is_json"])
        checks.append(("stdout_is_json", want, lambda r, d, want=want: (d[0] == want, d[0])))
    for i, jp in enumerate(expect.get("json") or []):
        def _j(r, d, jp=jp):
            if not d[0]:
                return False, "<stdout is not JSON>"
            values = resolve_path(d[1], parse_path(jp["path"]))
            op = jp["op"]
            if op == "exists":
                return any(v is not MISSING for v in values), _shown(values)
            if op == "absent":
                # A NEGATIVE operator must be universally quantified over [*]: "size is
                # absent" means no element has it, not "some element lacks it". Existential
                # negation let a probe and its own negation both pass on a mixed array, and
                # the gate reads that as already-supported -- silently discarding a real need.
                return all(v is MISSING for v in values), _shown(values)
            found = [v for v in values if v is not MISSING]
            # Same rule for the other negative operators.
            quantifier = all if op in _JSON_OPS_NEGATIVE else any
            return (quantifier(_one_json_op(op, v, jp.get("value")) for v in found),
                    _shown(values))
        checks.append(("json[%d]" % i, jp, _j))
    return checks


def _samples(runs, metric):
    return [r[metric] for r in runs if _is_num(r.get(metric))]


def _stat_of(samples, stat):
    if not samples:
        return None
    if stat == "median":
        return float(statistics.median(samples))
    if stat == "mean":
        return float(statistics.fmean(samples))
    if stat == "min":
        return float(min(samples))
    return float(max(samples))


def _baseline_stat(baselines, ref, metric, stat):
    """Look a baseline probe's aggregate up in ctx['baselines'] (a verdict, or an observation)."""
    b = (baselines or {}).get(ref)
    if not isinstance(b, dict):
        return None
    key = "%s_%s" % (metric, stat)
    if _is_num(b.get(key)):
        return float(b[key])
    if _is_num(b.get(metric)):
        return float(b[metric])
    return _stat_of(_samples(b.get("runs") or [], metric), stat)


def _stat_clause(metric, sp, runs, baselines):
    stat = sp["stat"]
    samples = _samples(runs, metric)
    value = _stat_of(samples, stat)
    actual = {"stat": stat, "value": value, "samples": samples}
    if value is None:
        actual["why"] = "%s was not measured on any run" % metric
        return False, actual
    ok = True
    if "lt" in sp:
        ok = ok and value < sp["lt"]
    if "gt" in sp:
        ok = ok and value > sp["gt"]
    if "ratio_lt" in sp or "ratio_gt" in sp:
        base = _baseline_stat(baselines, sp.get("rel_to"), metric, stat)
        actual["baseline_probe"] = sp.get("rel_to")
        actual["baseline"] = base
        if base is None or base == 0:
            actual["why"] = ("no baseline for %s in ctx['baselines'] -- the caller must supply "
                             "it" % sp.get("rel_to"))
            return False, actual
        ratio = value / base
        actual["ratio"] = ratio
        if "ratio_lt" in sp:
            ok = ok and ratio < sp["ratio_lt"]
        if "ratio_gt" in sp:
            ok = ok and ratio > sp["ratio_gt"]
    return ok, actual


def _collapse(actuals):
    keys = set()
    for a in actuals:
        keys.add(json.dumps(a, sort_keys=True, default=str))
    if len(keys) <= 1:
        return actuals[0] if actuals else None
    return actuals


def evaluate(probe, observation):
    """Turn an Observation into a Verdict. Every present clause must hold, on every repeat.

    ``flaky`` is True when the repeats disagree about the (non-aggregate) clauses: that is the
    pipeline's "this is not evidence" signal, and it is reported separately from ``passed`` so a
    caller can tell "the behaviour is not there" from "the gate could not decide". A flaky probe
    never passes, since passing requires every run to satisfy every clause.

    ``wall_ms``/``max_rss_kb`` are aggregates over the repeats and so cannot be attributed to one
    run; they are excluded from the flakiness verdict and evaluated once. A run that timed out or
    could not be executed adds a synthetic failing ``run`` clause: an incomplete observation is not
    a pass. ``rel_to`` baselines are read from ``observation["baselines"]``, which :func:`run`
    copies out of ``ctx["baselines"]`` -- the caller must supply them.
    """
    p = normalize(probe)
    expect = p["expect"]
    runs = list(observation.get("runs") or [])
    baselines = observation.get("baselines") or {}

    parsed = []
    for r in runs:
        try:
            parsed.append((True, json.loads(r.get("stdout") or "")))
        except (ValueError, TypeError):
            parsed.append((False, None))

    checkers = _per_run_checkers(expect)
    clauses = []
    for name, expected, fn in checkers:
        results = [fn(r, parsed[i]) for i, r in enumerate(runs)]
        clauses.append({
            "clause": name,
            "expected": expected,
            "actual": _collapse([x[1] for x in results]),
            "ok": bool(results) and all(x[0] for x in results),
        })
    for metric in _STAT_CLAUSES:
        if metric in expect:
            ok, actual = _stat_clause(metric, expect[metric], runs, baselines)
            clauses.append({"clause": metric, "expected": expect[metric],
                            "actual": actual, "ok": ok})

    broken = [r for r in runs if r.get("timed_out") or r.get("error")]
    if not runs:
        clauses.append({"clause": "run", "expected": "at least one run",
                        "actual": "no runs recorded", "ok": False})
    elif broken:
        clauses.append({"clause": "run",
                        "expected": "%d run(s) completing within %ss"
                                    % (len(runs), observation.get("timeout_s")),
                        "actual": "; ".join(sorted({str(r.get("error")) for r in broken})),
                        "ok": False})

    per_run = [all(fn(r, parsed[i])[0] for _, _, fn in checkers) for i, r in enumerate(runs)]
    wall = _samples(runs, "wall_ms")
    rss = _samples(runs, "max_rss_kb")
    return {
        "schema": "re-verdict/1",
        "probe_id": p["probe_id"],
        "kind": p.get("kind"),
        "passed": bool(runs) and all(c["ok"] for c in clauses),
        "flaky": len(set(per_run)) > 1,
        "repeat": len(runs),
        "run_passed": per_run,
        "clauses": clauses,
        "timed_out": any(r.get("timed_out") for r in runs),
        "errors": [r["error"] for r in runs if r.get("error")],
        "exit_codes": [r.get("exit_code") for r in runs],
        "wall_ms": wall,
        "wall_ms_median": _stat_of(wall, "median"),
        "wall_ms_min": _stat_of(wall, "min"),
        "wall_ms_max": _stat_of(wall, "max"),
        "wall_ms_mean": _stat_of(wall, "mean"),
        # The statpred enum allows median|min|max|mean, so all four must exist for BOTH
        # metrics. Emitting only median and max meant a `max_rss_kb` clause with
        # stat "min" or "mean" could never find its value and failed as unresolvable.
        "max_rss_kb_median": _stat_of(rss, "median"),
        "max_rss_kb_min": _stat_of(rss, "min"),
        "max_rss_kb_max": _stat_of(rss, "max"),
        "max_rss_kb_mean": _stat_of(rss, "mean"),
        "rusage_source": observation.get("rusage_source"),
        "cmd": observation.get("cmd"),
        "cwd": observation.get("cwd"),
        "stdout_head": (runs[0].get("stdout") or "")[:HEAD_CHARS] if runs else "",
        "stderr_head": (runs[0].get("stderr") or "")[:HEAD_CHARS] if runs else "",
    }


def check(probe, ctx=None, timeout=None):
    """Run the probe and evaluate it: the one call the two-arm gate and the builder both make."""
    p = normalize(probe)
    return evaluate(p, run(p, ctx, timeout=timeout))


# --- CLI --------------------------------------------------------------------

def _load(path):
    with open(path) as fh:
        return json.load(fh)


def _short(value, width=64):
    text = value if isinstance(value, str) else json.dumps(value, default=str)
    return text if len(text) <= width else text[:width - 1] + "…"


def _print_verdict(v):
    head = "%s  %s" % (v["probe_id"], "PASS" if v["passed"] else "FAIL")
    if v["flaky"]:
        head += "  FLAKY"
    if v["wall_ms_median"] is not None:
        head += "  (%d run(s), %.1f ms median" % (v["repeat"], v["wall_ms_median"])
        if v["max_rss_kb_median"] is not None:
            head += ", %d kB max rss" % v["max_rss_kb_median"]
        head += ")"
    print(head)
    print("  cmd: %s" % " ".join(v.get("cmd") or []))
    for c in v["clauses"]:
        print("  %-4s %-22s expected %-34s actual %s"
              % ("ok" if c["ok"] else "FAIL", c["clause"],
                 _short(c["expected"], 34), _short(c["actual"], 90)))


def main(argv=None):
    ap = argparse.ArgumentParser(
        prog="python3 -m scripts.repipe.probe",
        description="Evaluate a RE-friction probe (tools/repipe/schema/probe.schema.json).")
    sub = ap.add_subparsers(dest="cmd", required=True)

    pc = sub.add_parser("check", help="run the probe and print its verdict")
    pc.add_argument("probe")
    pc.add_argument("--work", help="{{WORK}} (default: cwd)")
    pc.add_argument("--bin", dest="binary", help="{{BIN}} (default: {{WORK}}/target.binary_rel)")
    pc.add_argument("--tmp", help="{{TMP}} (default: a fresh temp dir, removed afterwards)")
    pc.add_argument("--timeout", type=int, help="override the probe's timeout_s")
    pc.add_argument("--baselines", help="JSON file of {probe_id: verdict} for statpred rel_to")
    pc.add_argument("--json", action="store_true")

    pv = sub.add_parser("validate", help="structural check only, no execution")
    pv.add_argument("probe")
    pv.add_argument("--json", action="store_true")

    pi = sub.add_parser("id", help="print the derived probe id")
    pi.add_argument("probe")
    pi.add_argument("--acceptance", action="store_true", help="derive an 'a-' id")

    args = ap.parse_args(argv)
    try:
        raw = _load(args.probe)
    except (OSError, ValueError) as exc:
        print("cannot read %s: %s" % (args.probe, exc), file=sys.stderr)
        return 2

    if args.cmd == "validate":
        errs = validate(raw)
        if not errs and asserted_clause_count(raw.get("expect")) == 0:
            errs = ["expect: asserts nothing (a probe with no effective clause is not evidence)"]
        if args.json:
            print(json.dumps({"probe": args.probe, "valid": not errs, "errors": errs}, indent=2))
        elif errs:
            for e in errs:
                print("error: %s" % e)
        else:
            print("ok: %s" % args.probe)
        return 1 if errs else 0

    if args.cmd == "id":
        try:
            print(normalize(raw, is_acceptance=args.acceptance)["probe_id"])
        except ProbeError as exc:
            print("error: %s" % exc, file=sys.stderr)
            return 2
        return 0

    baselines = {}
    if args.baselines:
        try:
            baselines = _load(args.baselines)
        except (OSError, ValueError) as exc:
            print("cannot read %s: %s" % (args.baselines, exc), file=sys.stderr)
            return 2
    ctx = context(work=args.work, binary=args.binary, tmp=args.tmp, baselines=baselines)
    try:
        verdict = check(raw, ctx, timeout=args.timeout)
    except ProbeError as exc:
        if args.json:
            print(json.dumps({"probe": args.probe, "error": str(exc)}, indent=2))
        else:
            print("error: %s" % exc, file=sys.stderr)
        return 2
    if args.json:
        print(json.dumps(verdict, indent=2, default=str))
    else:
        _print_verdict(verdict)
    return 0 if verdict["passed"] else 1


if __name__ == "__main__":
    sys.exit(main())
