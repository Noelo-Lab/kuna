"""The two-arm gate: the only path from a tester's narrative to a builder's queue.

No tester prose ever reaches the backlog. Two executable predicates do: the PROBE asserts
the CURRENT BAD behaviour and must PASS, the ACCEPTANCE asserts the DESIRED behaviour and
must FAIL -- both replayed on a freshly built main at a pinned SHA, which every result
records so it can be re-read later. Why the gate exists at all: in the decbench campaign
round 2's refuters overturned the filed diagnosis on 3 of 8 cases while the symptom stood
in all 8, so a pipeline that lets a model's story reach a builder burns roughly 3 of every
8 builder-hours on the wrong mechanism.

The five verdicts, in the precedence `gate()` decides them:

  unrunnable         an arm could not be executed at all -- target missing, sha256 of the
                     target does not match the one the probe was authored against, probe
                     malformed, evaluator raised. No claim is made about kuna either way.
  flaky              an arm disagreed with itself across its repeats. A probe that is not
                     stable is not evidence, so it neither admits nor rejects.
  not-reproducible   the probe FAILED: the bad behaviour it describes is not there.
  already-supported  the acceptance PASSED: kuna already does the desired thing. This is
                     the "the tester was wrong" ledger, and it is the honest denominator
                     for every other number on the dashboard -- an EMPTY already-supported
                     bucket means the gate is broken, not that the testers were perfect
                     (it is a stated round-1 success criterion). The two rejecting rules
                     can fire together, so `reasons` lists every rule that applied and
                     `already_supported` is recorded independently of `verdict`.
  admitted           probe PASS and acceptance FAIL. Only these reach a builder.

`acceptance_suite()` is the other half of the loop and the machine's entire answer to
"have the builders fixed what the testers asked for": it re-runs the acceptance probe of
every need in docs/re-needs/ against the CURRENT build. FAIL->PASS closes a need; PASS->
FAIL on a closed need is a regression that goes back at rank 0. Neither is a judgment
call, and neither fires off a flaky or unrunnable replay.

`promote()` copies a closed need's acceptance probe verbatim into tests/cli/<need_id>.json
so every shipped need leaves a permanent regression test behind it. It refuses a probe
whose target is not vendorable: CI has no dataset.

The evaluator itself lives in probe.py and is not duplicated here -- this module decides
what a pair of verdicts MEANS, resolves the {{BIN}} a probe needs, and persists the answer.

    PYTHONPATH=<repo> python3 -m scripts.repipe.verify --gate --round 3 [--json]
    PYTHONPATH=<repo> python3 -m scripts.repipe.verify --observation obs.json [--json]
    PYTHONPATH=<repo> python3 -m scripts.repipe.verify --acceptance-suite --all [--json]
    PYTHONPATH=<repo> python3 -m scripts.repipe.verify --acceptance-suite --need no-xrefs
    PYTHONPATH=<repo> python3 -m scripts.repipe.verify --promote no-xrefs

Exit codes: 0 ran, 1 refused or nothing to do, 2 usage / infrastructure error.
"""
from __future__ import annotations

import argparse
import hashlib
import json
import os
import re
import shutil
import subprocess
import sys
import time
from pathlib import Path

from . import config, probe

SCHEMA = "re-gate/1"
ROUND_SCHEMA = "re-gate-round/1"
SUITE_SCHEMA = "re-acceptance-suite/1"

ADMITTED = "admitted"
NOT_REPRODUCIBLE = "not-reproducible"
ALREADY_SUPPORTED = "already-supported"
FLAKY = "flaky"
UNRUNNABLE = "unrunnable"
VERDICTS = (ADMITTED, NOT_REPRODUCIBLE, ALREADY_SUPPORTED, FLAKY, UNRUNNABLE)

# A need in any of these is still owed to us; its acceptance flipping to PASS closes it.
# `closed` is the mirror image: its acceptance flipping to FAIL is a regression.
OPEN_STATUSES = ("open", "claimed", "building", "proposal", "blocked", "regressed")

_HEXID = re.compile(r"^[0-9a-f]{24}$")


class PromotionRefused(ValueError):
    """promote() refuses rather than vendoring a probe CI cannot run."""


# --- small shared helpers ---------------------------------------------------

def _utc():
    return time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime())


def head_sha(repo=None):
    """The pinned SHA every verdict is stamped with. None when git cannot answer."""
    root = str(repo or config.repo_root())
    try:
        r = subprocess.run(["git", "-C", root, "rev-parse", "HEAD"],
                           capture_output=True, text=True, timeout=20)
    except (OSError, subprocess.SubprocessError):
        return None
    return r.stdout.strip() or None if r.returncode == 0 else None


def _write_json(path, obj):
    path = Path(path)
    path.parent.mkdir(parents=True, exist_ok=True)
    tmp = str(path) + ".tmp"
    with open(tmp, "w") as fh:
        json.dump(obj, fh, indent=2)
        fh.write("\n")
    os.replace(tmp, path)
    return path


def _sha256(path):
    h = hashlib.sha256()
    with open(path, "rb") as fh:
        for chunk in iter(lambda: fh.read(1 << 20), b""):
            h.update(chunk)
    return h.hexdigest()


# --- the probe.py seam ------------------------------------------------------
#
# probe.py owns the predicate language and the executor; this module never re-implements
# either. It decides what a PAIR of verdicts means, resolves the {{BIN}} an arm needs, and
# persists the answer. Everything below is a thin, defensive wrapper: a malformed probe
# from an LLM tester must come back as a filed "unrunnable" verdict, never as a traceback
# that takes the round down.

def make_ctx(work=None, binary=None, tmp=None, baselines=None):
    """The substitution context probe.check() resolves {{KUNA}}/{{BIN}}/{{WORK}}/... against.

    `baselines` maps a probe_id to an already-computed verdict, which is what a `rel_to`
    timing or memory clause reads: the perf lane's idiom is "the acceptance must be some
    ratio of the probe's median", and the ratio is what cancels kuna's fixed cold-load cost.
    """
    return probe.context(work=str(work) if work else None,
                         binary=str(binary) if binary else None,
                         tmp=str(tmp) if tmp else None,
                         baselines=baselines or {},
                         kuna=str(config.kuna_bin()), specs=str(config.specs_dir()))


def _probe_id(p, is_acceptance):
    try:
        return probe.probe_id(p, is_acceptance)
    except (probe.ProbeError, AttributeError, TypeError, ValueError):
        return None


_TIMEOUT_PREFIX = "timed out"


def _verdict_unrunnable(v):
    """True when the arm never really ran: bad argv, bad cwd, spawn failure, our own stub.

    A TIMEOUT is deliberately not unrunnable. The command did run; it just did not finish,
    and for a perf need that is the evidence -- a `timing` acceptance that times out is a
    real FAIL, which is what admits the need.
    """
    if not isinstance(v, dict):
        return True
    if v.get("unrunnable") or v.get("error"):
        return True
    return any(e and not str(e).startswith(_TIMEOUT_PREFIX) for e in (v.get("errors") or []))


def _stub(pid, reason, arm=None):
    """The verdict shape probe.py would have returned, for an arm that never got that far."""
    return {"schema": "re-verdict/1", "probe_id": pid, "arm": arm, "passed": False,
            "flaky": False, "clauses": [], "wall_ms_median": None, "repeat": 0,
            "unrunnable": True, "error": reason}


# --- resolving the binary a probe points at ---------------------------------

def resolve_binary(p, work=None, challenges=()):
    """Absolute path for {{BIN}}, plus a note on how it was found.

    A probe may name no target at all (`kuna --help`, an `absence` probe); that is not an
    error and yields ("", "no target"). Otherwise the search order is: an in-repo vendored
    path, the work dir (an arena keeps the arena-relative shape), then the dataset, whose
    binaries are addressed challenges/<hexid>/<rel> -- `bin/` preserves recursive-extraction
    path shapes, so the rel is never globbed for.

    Two dataset facts are handled here rather than left to fail obscurely: exec bits are
    broken (4 shipped binaries are mode 600, 54 are 644), so a non-executable target is
    copied out to <state>/probebin and chmod 0755'd -- the dataset itself is read-only
    input and is never written to; and a binary_sha256 that does not match the file is a
    hard stop, because the probe was authored against something else.
    """
    t = (p or {}).get("target")
    if not t:
        return "", "no target"
    rel = t.get("binary_rel") or ""
    if not rel:
        return "", "target has no binary_rel"

    cands = []
    if t.get("binary_source") == "in-repo" and t.get("in_repo_path"):
        cands.append(config.repo_root() / t["in_repo_path"])
    if work:
        cands.append(Path(work) / rel)
        cands.append(Path(work) / "target" / rel)
    ds = config.dataset_root()
    trimmed = re.sub(r"^target/", "", rel)
    cands.append(ds / rel)
    cands.append(ds / trimmed)
    for hexid in challenges or ():
        cands.append(ds / "challenges" / hexid / rel)
        cands.append(ds / "challenges" / hexid / trimmed)

    found = None
    for c in cands:
        try:
            if c.is_file():
                found = c
                break
        except OSError:
            continue
    if found is None:
        raise FileNotFoundError("target %s not found (tried %d locations)" % (rel, len(cands)))

    want = t.get("binary_sha256")
    if want:
        got = _sha256(found)
        if got != want:
            raise ValueError("target sha256 mismatch at %s: probe wants %s, file is %s"
                             % (found, want[:12], got[:12]))
    if os.access(str(found), os.X_OK):
        return str(found), "found at %s" % found
    stage = config.state_dir() / "probebin" / (want or _sha256(found))[:16]
    stage.mkdir(parents=True, exist_ok=True)
    dest = stage / found.name
    if not dest.exists():
        shutil.copy2(str(found), str(dest))
    os.chmod(str(dest), 0o755)
    return str(dest), "copied from %s (not executable in place)" % found


# --- one arm of the gate ----------------------------------------------------

def gate_reps(p):
    """The rep floor the gate replays an arm at.

    The plan pins the gate at REPLAY_REPS (3): a probe that cannot agree with itself is
    not evidence, and with repeat 1 the flaky bucket can never fill. Timing and memory
    probes get TIMING_REPS instead, because single-target timing noise on this machine
    routinely exceeds 5% -- returncopysplit's own record measured a -20%/-12% floor on
    byte-identical output. acceptance_suite() does NOT apply a floor by default: at
    INTEGRATE it re-runs the whole suite, and kuna's worst measured case is 445 s.
    """
    kind = p.get("kind") if isinstance(p, dict) else None
    return config.TIMING_REPS if kind in ("timing", "memory") else config.REPLAY_REPS


def run_probe(p, is_acceptance=False, ctx=None, work=None, challenges=(), reps=None,
              baselines=None):
    """Replay one probe and return probe.py's verdict, augmented with what we know.

    `reps` raises the probe's own `repeat` to at least that many runs; it never lowers it,
    and it does not change the probe_id (derived from cmd + expect alone, so the id is the
    same in an arena, a scratch checkout and tests/cli/).

    Anything that stops the probe from running at all comes back as a verdict carrying
    `unrunnable` and a human reason, never as an exception: the gate must be able to file
    "we could not tell" as a result rather than lose the observation.
    """
    arm = "acceptance" if is_acceptance else "probe"
    if isinstance(p, str):
        return _stub(None, "%s did not parse as JSON, so it is not a probe" % arm, arm)
    if not isinstance(p, dict):
        return _stub(None, "observation carries no %s" % arm, arm)
    if not p.get("cmd"):
        return _stub(None, "%s has no cmd" % arm, arm)
    if not p.get("expect"):
        return _stub(None, "%s has an empty expect: it asserts nothing and would "
                           "always pass" % arm, arm)
    pid = _probe_id(p, is_acceptance)

    try:
        norm = probe.normalize(p, is_acceptance)
    except Exception as exc:  # noqa: BLE001 - a malformed probe must not kill the round
        return _stub(pid, "%s: %s" % (type(exc).__name__, exc), arm)
    if reps:
        norm = dict(norm)
        norm["repeat"] = max(int(norm.get("repeat") or 1), min(int(reps), 11))

    note = None
    if ctx is None:
        try:
            binary, note = resolve_binary(norm, work=work, challenges=challenges)
        except (FileNotFoundError, ValueError, OSError) as exc:
            return _stub(norm["probe_id"], str(exc), arm)
        ctx = make_ctx(work=work, binary=binary, baselines=baselines)
    elif baselines:
        ctx = dict(ctx)
        ctx["baselines"] = dict(ctx.get("baselines") or {}, **baselines)

    try:
        v = probe.check(norm, ctx)
    except Exception as exc:  # noqa: BLE001 - same
        return _stub(norm["probe_id"], "check raised: %s: %s" % (type(exc).__name__, exc), arm)
    if not isinstance(v, dict):
        v = {"passed": bool(v), "flaky": False, "clauses": [], "wall_ms_median": None}

    out = dict(v)
    out.setdefault("probe_id", norm.get("probe_id") or pid)
    out["arm"] = arm
    out["unrunnable"] = _verdict_unrunnable(v)
    if note:
        out["target_note"] = note
    return out


# --- the gate ---------------------------------------------------------------

def coerce_probes(observation):
    """Parse an observation's probe/acceptance if they arrived as JSON strings.

    They do: OpenAI's strict structured-output mode cannot express the probe schema (its
    numpred/jsonpred values are deliberately any-typed, and strict mode demands an explicit
    type everywhere), so the tester report carries them serialised and they are validated
    here by probe.validate() instead. A string that will not parse is left as-is, which makes
    the observation `unrunnable` -- the honest verdict for "this is not a probe".
    """
    out = dict(observation or {})
    for key in ("probe", "acceptance"):
        v = out.get(key)
        if isinstance(v, str):
            try:
                out[key] = json.loads(v)
            except (ValueError, TypeError):
                pass
    return out


def gate(observation, ctx=None, work=None, challenges=(), sha=None, reps=None):
    """Run an observation's two arms and say what the pair means.

    `ctx`, when given, is used verbatim for BOTH arms -- that is the caller asserting one
    substitution environment for the pair. Left None (the usual case) each arm resolves
    its own {{BIN}} from its own target.
    """
    obs = coerce_probes(observation)
    if not challenges and obs.get("hexid"):
        challenges = [obs["hexid"]]
    p, a = obs.get("probe"), obs.get("acceptance")
    pv = run_probe(p, False, ctx, work, challenges, reps or gate_reps(p))
    base = {pv["probe_id"]: pv} if pv.get("probe_id") else None
    av = run_probe(a, True, ctx, work, challenges, reps or gate_reps(a), base)

    reasons = []
    if pv.get("unrunnable"):
        reasons.append("probe-unrunnable")
    if av.get("unrunnable"):
        reasons.append("acceptance-unrunnable")
    if reasons:
        verdict = UNRUNNABLE
    else:
        if pv.get("flaky"):
            reasons.append("probe-flaky")
        if av.get("flaky"):
            reasons.append("acceptance-flaky")
        if reasons:
            verdict = FLAKY
        else:
            if not pv.get("passed"):
                reasons.append("probe-fail")
            if av.get("passed"):
                reasons.append("acceptance-pass")
            if pv.get("passed") and not av.get("passed"):
                reasons.append("probe-pass-acceptance-fail")
                verdict = ADMITTED
            elif "probe-fail" in reasons:
                verdict = NOT_REPRODUCIBLE
            else:
                verdict = ALREADY_SUPPORTED

    vend, vend_why = vendorable(a)
    return {
        "schema": SCHEMA,
        "verdict": verdict,
        "reasons": reasons,
        "already_supported": bool(av.get("passed")) and not (
            pv.get("unrunnable") or av.get("unrunnable") or av.get("flaky")),
        "sha": sha if sha is not None else head_sha(),
        "at": _utc(),
        "title": obs.get("title"),
        "kind": obs.get("kind"),
        "severity": obs.get("severity"),
        "regression_of": obs.get("regression_of"),
        "hexid": obs.get("hexid"),
        "probe_id": pv.get("probe_id"),
        "acceptance_id": av.get("probe_id"),
        "acceptance_vendorable": vend,
        "acceptance_vendorable_why": vend_why,
        "probe": pv,
        "acceptance": av,
    }


def _counts(results):
    c = dict((v, 0) for v in VERDICTS)
    for r in results:
        c[r["verdict"]] = c.get(r["verdict"], 0) + 1
    return c


def round_reports(round_n):
    """Every tester report.json belonging to a round, arena-first then round-dir."""
    n = str(round_n)
    seen, out = set(), []
    globs = [(config.arena_dir() / n).glob("*/report.json"),
             (config.rounds_dir() / n / "reports").glob("*.json"),
             (config.rounds_dir() / n).glob("*/report.json")]
    for g in globs:
        try:
            files = sorted(g)
        except OSError:
            continue
        for f in files:
            key = str(f.resolve())
            if key in seen:
                continue
            seen.add(key)
            out.append(f)
    return out


def _tester_id_for(report_path):
    """Recover which tester produced a report when the report itself does not say.

    The arena is `<state>/arena/<round>/<hexid>/report.json` and tester.sh derives its id
    from exactly those two components, so this reconstructs it rather than leaving the
    corroboration count blind -- credibility weighs DISTINCT testers, and a None there would
    make three findings from three agents look like one opinion.
    """
    try:
        hexid = report_path.parent.name
        rnd = report_path.parent.parent.name
        return "t-r%s-%s" % (rnd, hexid[:8])
    except Exception:
        return None


def _arena_primary(arena):
    """The arena's primary binary, from the build record workspace.py leaves behind.

    Falls back to the single file under target/ when there is exactly one, so a
    hand-assembled arena still resolves.
    """
    rec = Path(arena) / ".repipe" / "build.json"
    try:
        doc = json.loads(rec.read_text())
        rel = (doc.get("binaries") or [{}])[0].get("arena_rel")
        if rel:
            p = Path(arena) / rel
            if p.exists():
                return str(p)
    except Exception:
        pass
    tgt = Path(arena) / "target"
    if tgt.is_dir():
        files = [f for f in sorted(tgt.rglob("*")) if f.is_file()]
        if len(files) == 1:
            return str(files[0])
    return None


def gate_round(round_n, ctx=None, reps=None):
    """Gate every observation in every report of a round; persist rounds/<n>/gate.json."""
    sha = head_sha()
    results = []
    reports = []
    for rp in round_reports(round_n):
        try:
            report = json.loads(rp.read_text())
        except (OSError, json.JSONDecodeError) as exc:
            reports.append({"report": str(rp), "error": str(exc), "observations": 0})
            continue
        hexid = report.get("hexid") or (rp.parent.name if _HEXID.match(rp.parent.name) else None)
        obs = report.get("observations") or []
        reports.append({"report": str(rp), "hexid": hexid, "outcome": report.get("outcome"),
                        "observations": len(obs)})
        if not isinstance(obs, list):
            # A report whose `observations` is not a list is one bad report, not a bad round.
            # Crashing here would discard every other tester's evidence too.
            reports[-1]["error"] = "observations is %s, not a list" % type(obs).__name__
            reports[-1]["observations"] = 0
            continue
        for i, o in enumerate(obs):
            if not isinstance(o, dict):
                reports[-1].setdefault("skipped", []).append(i)
                continue
            o = dict(o or {})
            o.setdefault("hexid", hexid)
            # Supply {{BIN}} from the ARENA. The harness already knows which binary the
            # tester was working on, so requiring the tester to restate it in every probe's
            # `target` block just discards correct observations as unrunnable -- which is
            # exactly what happened to the first real report. An explicit `target` still
            # wins, because that is what makes a probe portable off the arena.
            rctx = dict(ctx or {})
            if not rctx.get("bin"):
                arena_bin = _arena_primary(rp.parent)
                if arena_bin:
                    rctx["bin"] = arena_bin
            r = gate(o, ctx=rctx, work=str(rp.parent),
                     challenges=[hexid] if hexid else (), sha=sha, reps=reps)
            r["report"] = str(rp)
            r["observation_index"] = i
            # cluster.py builds the need record from these, so the gate result must carry the
            # observation it gated and who filed it -- a verdict with no observation attached
            # would collapse every need into one empty record.
            # Record what was actually GATED, not what arrived: the probes cross the API
            # boundary as JSON strings, and downstream (cluster.py) needs them parsed.
            r["observation"] = coerce_probes(o)
            r["tester_id"] = report.get("tester_id") or _tester_id_for(rp)
            results.append(r)

    out = {"schema": ROUND_SCHEMA, "round": int(round_n), "sha": sha, "at": _utc(),
           "counts": _counts(results), "reports": reports, "results": results}
    out["path"] = str(_write_json(config.rounds_dir() / str(round_n) / "gate.json", out))
    return out


# --- need records -----------------------------------------------------------

def _scalar(v):
    if v.startswith("[") and v.endswith("]"):
        inner = v[1:-1].strip()
        return [x.strip().strip("\"'") for x in inner.split(",") if x.strip()] if inner else []
    if len(v) >= 2 and v[0] == v[-1] and v[0] in "\"'":
        return v[1:-1]
    if v in ("null", "~", ""):
        return None
    if v in ("true", "false"):
        return v == "true"
    if re.fullmatch(r"-?\d+", v):
        return int(v)
    if re.fullmatch(r"-?\d+\.\d+", v):
        return float(v)
    return v


def front_matter(text):
    """The docs/decbench/triage/*.md dialect: `---`, flat key: value, inline [a, b] lists."""
    m = re.match(r"---\n(.*?)\n---", text, re.S)
    if not m:
        return None
    fm = {}
    for line in m.group(1).splitlines():
        kv = re.match(r"^(\w+):\s*(.*?)\s*$", line)
        if kv:
            fm[kv.group(1)] = _scalar(kv.group(2))
    return fm


def _section(text, name):
    m = re.search(r"^##\s+%s\s*$(.*?)(?=^##\s|\Z)" % re.escape(name), text, re.S | re.M)
    return m.group(1) if m else ""


def _fenced_probe(chunk):
    """First fenced JSON object in a chunk that looks like a probe. Returns (dict, text)."""
    for m in re.finditer(r"```(?:json)?\n(.*?)\n```", chunk, re.S):
        raw = m.group(1)
        try:
            obj = json.loads(raw)
        except json.JSONDecodeError:
            continue
        if isinstance(obj, dict) and obj.get("cmd") and obj.get("expect"):
            return obj, raw
    return None, None


def need_records(need_ids=None):
    """Every filed need. docs/re-needs/rejected/ is a subdirectory, so *.md skips it."""
    want = set(need_ids or ())
    out = []
    d = config.needs_dir()
    if not d.is_dir():
        return out
    for f in sorted(d.glob("*.md")):
        try:
            text = f.read_text(errors="replace")
        except OSError:
            continue
        fm = front_matter(text)
        if fm is None:
            continue
        nid = fm.get("need_id") or f.stem
        if want and nid not in want and f.stem not in want:
            continue
        out.append({"need_id": nid, "path": f, "text": text, "front_matter": fm})
    return out


def probe_of(rec):
    """The need's PROBE arm, as (dict, verbatim_text, source) -- the mirror of acceptance_of.

    Needed because a `rel_to` acceptance is expressed relative to this arm's median, so the
    suite has to be able to find and replay it.
    """
    obj, raw = _fenced_probe(_section(rec["text"], "Reproduction"))
    if obj is not None:
        return obj, raw, "record"
    pid = rec["front_matter"].get("probe_id")
    if pid:
        for base in (config.needs_dir() / "probes", config.state_dir() / "probes"):
            f = base / ("%s.json" % pid)
            if f.is_file():
                try:
                    raw = f.read_text()
                    return json.loads(raw), raw, str(f)
                except (OSError, json.JSONDecodeError):
                    continue
    return None, None, None


def acceptance_of(rec):
    """The need's acceptance probe, as (dict, verbatim_text, source).

    The record is truth: a fenced probe in its `## Acceptance` section wins. A sidecar
    under docs/re-needs/probes/ or <state>/probes/ is the fallback, so a needs.py that
    keeps probes out of line still resolves.
    """
    obj, raw = _fenced_probe(_section(rec["text"], "Acceptance"))
    if obj is not None:
        return obj, raw, "record"
    aid = rec["front_matter"].get("acceptance_id")
    if aid:
        for base in (config.needs_dir() / "probes", config.state_dir() / "probes"):
            f = base / ("%s.json" % aid)
            if f.is_file():
                try:
                    raw = f.read_text()
                    return json.loads(raw), raw, str(f)
                except (OSError, json.JSONDecodeError):
                    continue
    return None, None, None


def _acceptance_baselines(rec, challenges, reps):
    """Measure the need's PROBE arm when its acceptance needs it as a `rel_to` baseline.

    Only paid for when a clause actually references one: replaying the probe of every need in
    the backlog on every suite run would triple the cost of the loop's most frequent
    operation for a feature almost no need uses.
    """
    p, _raw, _src = acceptance_of(rec)
    if not _wants_baseline(p):
        return None
    probe_p, _praw, _psrc = probe_of(rec)
    if probe_p is None:
        return None
    try:
        pv = run_probe(probe_p, False, None, None, challenges, reps)
    except Exception:
        return None
    # Key by BOTH the derived id and whatever the record calls it. A `rel_to` is written by
    # hand against the front-matter's probe_id, which need not equal the id derived from
    # cmd+expect -- and a baseline nobody can look up is the same as no baseline at all.
    out = {}
    for pid in (pv.get("probe_id"), (probe_p or {}).get("probe_id"),
                rec["front_matter"].get("probe_id")):
        if pid:
            out[pid] = pv
    return out or None


def _wants_baseline(probe_doc):
    for key in ("wall_ms", "max_rss_kb"):
        clause = ((probe_doc or {}).get("expect") or {}).get(key) or {}
        if isinstance(clause, dict) and clause.get("rel_to"):
            return True
    return False


def acceptance_suite(need_ids=None, reps=None):
    """Re-run every filed acceptance probe against the CURRENT build.

    This is the mechanism that answers "have the builders fixed what the testers asked
    for". A need whose acceptance flips FAIL->PASS is `closed`; a previously-closed need
    whose acceptance flips PASS->FAIL is `regressed` and must be re-queued at rank 0. A
    flaky or unrunnable replay is `indeterminate` and moves nothing -- closing a need on a
    coin-flip would be worse than not closing it.
    """
    sha = head_sha()
    rows, closed, regressed = [], [], []
    for rec in need_records(need_ids):
        fm = rec["front_matter"]
        status = fm.get("status") or "open"
        p, _raw, src = acceptance_of(rec)
        if p is None:
            rows.append({"need_id": rec["need_id"], "status": status,
                         "acceptance_id": fm.get("acceptance_id"), "passed": None,
                         "flaky": None, "unrunnable": True, "transition": "indeterminate",
                         "error": "no acceptance probe on the record or in the probe store",
                         "acceptance": None})
            continue
        chal = fm.get("challenges") or []
        if isinstance(chal, str):
            chal = [chal]
        # A `rel_to` acceptance compares its median against another probe's, so that
        # baseline has to be measured here or the clause can never resolve and the need is
        # unclosable forever. gate() already does this; the suite did not, which made the
        # whole perf idiom dead on arrival.
        baselines = _acceptance_baselines(rec, chal, reps)
        v = run_probe(p, True, None, None, chal, reps, baselines=baselines)
        if v.get("unrunnable") or v.get("flaky"):
            trans = "indeterminate"
        elif v.get("passed") and status in OPEN_STATUSES:
            trans = "closed"
            closed.append(rec["need_id"])
        elif not v.get("passed") and status == "closed":
            trans = "regressed"
            regressed.append(rec["need_id"])
        else:
            trans = "unchanged"
        rows.append({"need_id": rec["need_id"], "status": status,
                     "acceptance_id": v.get("probe_id"), "source": src,
                     "passed": v.get("passed"), "flaky": v.get("flaky"),
                     "unrunnable": v.get("unrunnable"), "error": v.get("error"),
                     "transition": trans, "requeue_rank": 0 if trans == "regressed" else None,
                     "acceptance": v})

    counts = {"total": len(rows), "pass": sum(1 for r in rows if r["passed"] is True),
              "fail": sum(1 for r in rows if r["passed"] is False),
              "closed": len(closed), "regressed": len(regressed),
              "indeterminate": sum(1 for r in rows if r["transition"] == "indeterminate")}
    return {"schema": SUITE_SCHEMA, "sha": sha, "at": _utc(), "counts": counts,
            "closed": closed, "regressed": regressed, "needs": rows}


# --- promotion into tests/cli/ ----------------------------------------------

def vendorable(p):
    """Can CI, which has no dataset, run this probe? Returns (bool, reason)."""
    if not isinstance(p, dict):
        return False, "not a probe"
    t = p.get("target")
    if not t:
        blob = json.dumps([p.get("cmd"), p.get("cwd"), p.get("env")])
        if "{{BIN}}" in blob:
            return False, "no target, but the command substitutes {{BIN}}"
        return True, "no binary target"
    src = t.get("binary_source")
    if src != "in-repo":
        return False, ("target.binary_source is %r, not 'in-repo' -- CI has no dataset, so "
                       "vendor the binary into the repo first" % (src,))
    rel = t.get("in_repo_path")
    if not rel:
        return False, "binary_source is 'in-repo' but in_repo_path is unset"
    abs_p = config.repo_root() / rel
    if not abs_p.is_file():
        return False, "in_repo_path does not exist: %s" % abs_p
    want = t.get("binary_sha256")
    if want and _sha256(abs_p) != want:
        return False, "in-repo target %s does not match target.binary_sha256" % rel
    return True, "in-repo target %s" % rel


def promote(need_id, force=False):
    """Copy a closed need's acceptance probe verbatim into tests/cli/<need_id>.json."""
    recs = need_records([need_id])
    if not recs:
        raise PromotionRefused("no need record for %r in %s" % (need_id, config.needs_dir()))
    rec = recs[0]
    status = rec["front_matter"].get("status")
    if status != "closed" and not force:
        raise PromotionRefused(
            "need %s is %r, not 'closed' -- promote the acceptance only once it has flipped "
            "to PASS (use --force to override)" % (need_id, status))
    p, raw, _src = acceptance_of(rec)
    if p is None:
        raise PromotionRefused("need %s carries no acceptance probe to promote" % need_id)
    ok, why = vendorable(p)
    if not ok:
        raise PromotionRefused("acceptance of %s is not vendorable: %s" % (need_id, why))

    dest = config.cli_tests_dir() / ("%s.json" % need_id)
    dest.parent.mkdir(parents=True, exist_ok=True)
    tmp = str(dest) + ".tmp"
    body = raw if raw is not None else json.dumps(p, indent=2)
    with open(tmp, "w") as fh:
        fh.write(body if body.endswith("\n") else body + "\n")
    os.replace(tmp, dest)
    return dest


# --- CLI --------------------------------------------------------------------

def _load_observations(path):
    """A single observation, a list of them, or a whole tester report.json."""
    obj = json.loads(Path(path).read_text())
    if isinstance(obj, list):
        return obj, None
    if isinstance(obj, dict) and isinstance(obj.get("observations"), list):
        return obj["observations"], obj.get("hexid")
    return [obj], obj.get("hexid") if isinstance(obj, dict) else None


def _arm_line(v):
    if v.get("unrunnable"):
        return "UNRUNNABLE(%s)" % (v.get("error") or "?")
    if v.get("flaky"):
        return "FLAKY"
    return "PASS" if v.get("passed") else "FAIL"


def _print_gate(r):
    print("%-17s %s" % (r["verdict"], r.get("title") or ""))
    print("    probe      %-12s %s" % (r.get("probe_id") or "-", _arm_line(r["probe"])))
    print("    acceptance %-12s %s" % (r.get("acceptance_id") or "-", _arm_line(r["acceptance"])))
    print("    reasons    %s" % (", ".join(r["reasons"]) or "-"))
    print("    sha        %s" % (r.get("sha") or "(unknown)"))


def main(argv=None):
    ap = argparse.ArgumentParser(prog="python -m scripts.repipe.verify",
                                 description=__doc__.splitlines()[0])
    ap.add_argument("--gate", action="store_true", help="run the two-arm gate")
    ap.add_argument("--round", type=int, default=None, metavar="N",
                    help="with --gate: gate every observation of round N")
    ap.add_argument("--observation", metavar="FILE",
                    help="gate one observation (or a whole report.json)")
    ap.add_argument("--acceptance-suite", dest="acceptance_suite", action="store_true",
                    help="re-run every filed acceptance probe against the current build")
    ap.add_argument("--all", action="store_true", help="--acceptance-suite: every need")
    ap.add_argument("--need", action="append", default=[], metavar="ID",
                    help="--acceptance-suite: only this need (repeatable)")
    ap.add_argument("--promote", metavar="ID",
                    help="vendor a closed need's acceptance into tests/cli/")
    ap.add_argument("--force", action="store_true",
                    help="--promote: allow a need that is not yet closed")
    ap.add_argument("--work", default=None, metavar="DIR",
                    help="--observation: the work dir {{WORK}} and {{BIN}} resolve against")
    ap.add_argument("--challenge", action="append", default=[], metavar="HEXID",
                    help="--observation: dataset challenge(s) to resolve a target through")
    ap.add_argument("--reps", type=int, default=None, metavar="N",
                    help="floor on how many times each probe is replayed (gate default: "
                         "REPIPE_REPLAY_REPS=%d, timing/memory %d)"
                         % (config.REPLAY_REPS, config.TIMING_REPS))
    ap.add_argument("--json", action="store_true")
    args = ap.parse_args(argv)

    # `--need <id>` on its own means "re-run that need's acceptance" -- the builder's
    # definition of done. Requiring --acceptance-suite next to it would make the command in
    # builder_prompt.md exit 2.
    if args.need and not (args.observation or args.gate or args.promote):
        args.acceptance_suite = True

    modes = [bool(args.observation), bool(args.gate and args.round is not None),
             args.acceptance_suite, bool(args.promote)]
    if sum(modes) != 1:
        ap.error("choose exactly one of --observation FILE, --gate --round N, "
                 "--acceptance-suite [--need ID], --promote ID")

    if args.observation:
        obs, hexid = _load_observations(args.observation)
        chal = args.challenge or ([hexid] if hexid else [])
        work = args.work or str(Path(args.observation).resolve().parent)
        sha = head_sha()
        results = []
        for o in obs:
            o = dict(o or {})
            if hexid:
                o.setdefault("hexid", hexid)
            results.append(gate(o, work=work, challenges=chal, sha=sha, reps=args.reps))
        if args.json:
            print(json.dumps(results[0] if len(results) == 1 else
                             {"counts": _counts(results), "results": results}, indent=2))
            return 0
        for r in results:
            _print_gate(r)
        if len(results) > 1:
            print(" ".join("%s=%d" % (k, v) for k, v in _counts(results).items()))
        return 0

    if args.gate:
        out = gate_round(args.round, reps=args.reps)
        if args.json:
            print(json.dumps(out, indent=2))
            return 0 if out["results"] else 1
        print("round %d  sha %s  %d report(s)" % (out["round"], out["sha"] or "?",
                                                  len(out["reports"])))
        for r in out["results"]:
            print("  %-17s %-12s %-12s %s" % (r["verdict"], r.get("probe_id") or "-",
                                              r.get("acceptance_id") or "-",
                                              r.get("title") or ""))
        print(" ".join("%s=%d" % (k, v) for k, v in out["counts"].items()))
        print("wrote %s" % out["path"])
        return 0 if out["results"] else 1

    if args.acceptance_suite:
        if not args.all and not args.need:
            ap.error("--acceptance-suite needs --all or --need ID")
        out = acceptance_suite(args.need or None, reps=args.reps)
        if args.round is not None:
            out["path"] = str(_write_json(
                config.rounds_dir() / str(args.round) / "acceptance.json", out))
        if args.json:
            print(json.dumps(out, indent=2))
            return 0
        print("acceptance suite  sha %s" % (out["sha"] or "?"))
        for r in out["needs"]:
            print("  %-6s %-14s %-14s %s" % (_arm_line(r["acceptance"] or r),
                                             r["transition"], r["status"], r["need_id"]))
        print(" ".join("%s=%d" % (k, v) for k, v in out["counts"].items()))
        return 0

    try:
        dest = promote(args.promote, force=args.force)
    except PromotionRefused as exc:
        if args.json:
            print(json.dumps({"promoted": None, "need_id": args.promote,
                              "refused": str(exc)}, indent=2))
        else:
            print("refused: %s" % exc, file=sys.stderr)
        return 1
    if args.json:
        print(json.dumps({"promoted": str(dest), "need_id": args.promote}, indent=2))
    else:
        print("promoted %s -> %s" % (args.promote, dest))
    return 0


if __name__ == "__main__":
    sys.exit(main())
