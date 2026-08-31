"""Tiered, honest solve verdicts -- and the contamination tripwire.

Solve rate is a SECONDARY metric for this pipeline; the primary output of a tester run is
probes, graded by replay. This module exists anyway because "did the tester actually solve
it?" is the only sanity check on whether the friction it reported was friction on the way to
somewhere, and because a contaminated run must be caught before its outcome is believed.

The ground truth is weak and the verdict says so out loud rather than averaging it away:

  flag-exact      HIGH  -- the answer equals meta.ground_truth.flag. 98 of 250 records have
                          one. This is the only tier that is simply true.
  binary-accepts  HIGH  -- the arena's copy of the binary was run with the tester's
                          name/serial under `timeout 10` and said something that reads as
                          success. Best-effort and heuristic: crackmes have no success
                          protocol, so this is keyword matching on stdout/stderr and it can
                          be fooled by a binary that prints "correct format, wrong key" or
                          that reports failure only through an exit code.
  verifier-agrees LOW   -- `verifier.py verify <name> <serial>` printed exactly `1`. These
                          70 files are LLM reconstructions from public writeups that were
                          NEVER validated against the binaries: 19 raise NotImplementedError,
                          4 are quarantined stubs, 56 carry an ASSUMPTION comment, and one
                          ends "We return True here provisionally" -- i.e. it accepts nearly
                          any well-formed serial. A pass here is a hint, not a fact, and the
                          returned dict is flagged low-confidence so no consumer can forget.
  unverifiable          -- nothing above could decide. 22 of 250 challenges are machine-
                          checkable AND uncontaminated; for the rest this is the honest
                          answer.

The verifier runs in a SUBPROCESS and is never imported: it is unvetted generated code, and
importing it would run it inside the one process that is holding the plaintext ground truth.
Its exit code is 0 whether it says yes or no, so only its STDOUT is read.

Nothing this module returns ever contains the flag or the tester's raw answer -- an answer
that matched IS the flag, and these dicts land in round state and on the dashboard. Answers
travel as a sha1 prefix.

CLI:
    python3 -m scripts.repipe.grade verdict --hexid H [--report R.json | --answer V [--name N]]
            [--arena DIR] [--round N] [--record] [--json]
    python3 -m scripts.repipe.grade tripwire --hexid H --transcript FILE [--json]
"""
from __future__ import annotations

import argparse
import datetime
import hashlib
import json
import os
import re
import shutil
import subprocess
import sys
import tempfile

from . import config

SCHEMA = "re-grade/1"

BINARY_TIMEOUT = int(os.environ.get("REPIPE_BINARY_TIMEOUT", "10"))
VERIFIER_TIMEOUT = int(os.environ.get("REPIPE_VERIFIER_TIMEOUT", "20"))

TIER_CONFIDENCE = {"flag-exact": "high", "binary-accepts": "high",
                   "verifier-agrees": "low", "unverifiable": "none"}

# The negative verdict each tier produces when it decides against the tester.
TIER_NEGATIVE = {"flag-exact": "flag-mismatch", "binary-accepts": "binary-rejects",
                 "verifier-agrees": "verifier-disagrees"}

LOW_CONFIDENCE_NOTE = (
    "verifier.py files are LLM reconstructions from public writeups, never validated "
    "against the binaries (19/70 raise NotImplementedError, 4 are quarantined stubs, one "
    "accepts nearly any well-formed serial). Treat this verdict as a hint.")

# Word boundaries are load-bearing, not decoration: "Invalid password!" contains the
# substring "valid password", and challenges/61ffb07c33c5d46c8bcbfc1d prints exactly that.
_SUCCESS = re.compile(
    r"congrat|well\s*done|good\s+(job|work|password|serial|key|pass)\b|\bcorrect\b|"
    r"success|accepted|access\s+granted|\bgranted\b|"
    r"\bvalid\s+(serial|key|password|licen[cs]e|pass)\b|you\s+(did|got)\s+it|"
    r"right\s+(serial|key|password)\b|solved|winner|unlocked|flag\{", re.I)
_FAILURE = re.compile(
    r"wrong|incorrect|invalid|denied|nope|try\s+again|failed|failure|"
    r"bad\s+(serial|key|password|licen[cs]e)|sorry|no\s+luck|not\s+(valid|correct)", re.I)


# --- inputs -----------------------------------------------------------------

def challenge_dir(hexid):
    return config.dataset_root() / "challenges" / hexid


def load_meta(hexid):
    """Read-only. This is the spoiler side of the fence: grade.py runs challenge-side and
    never inside the tester's mount namespace."""
    with open(challenge_dir(hexid) / "meta.json") as fh:
        return json.load(fh)


def load_report(report):
    """Accept a parsed report dict or a path to one, so the captain and the CLI agree."""
    if report is None:
        return {}
    if isinstance(report, dict):
        return report
    with open(report) as fh:
        return json.load(fh)


def _answer_of(report):
    """(name, value, kind) from a report.schema.json answer block, tolerating a bare string."""
    ans = report.get("answer")
    if isinstance(ans, str):
        return None, ans.strip() or None, "flag"
    if not isinstance(ans, dict):
        return None, None, "none"
    value = ans.get("value")
    value = value.strip() if isinstance(value, str) else None
    name = ans.get("name")
    name = name.strip() if isinstance(name, str) else None
    return (name or None), (value or None), (ans.get("kind") or "none")


def _digest(text):
    if text is None:
        return None
    return hashlib.sha1(text.encode("utf-8", "replace")).hexdigest()[:12]


def _norm(text):
    return text.strip().strip("'\"").strip() if isinstance(text, str) else text


# --- tier 1: the flag -------------------------------------------------------

def _tier_flag(meta, value):
    flag = (meta.get("ground_truth") or {}).get("flag")
    tier = {"tier": "flag-exact", "confidence": "high"}
    if not flag:
        tier.update(result="skipped", why="this challenge has no ground_truth.flag")
        return tier
    if not value:
        tier.update(result="skipped", why="the report carried no answer")
        return tier
    ok = _norm(value) == _norm(flag)
    tier.update(result="pass" if ok else "fail",
                why="answer equals ground_truth.flag" if ok
                    else "answer does not equal ground_truth.flag")
    return tier


# --- tier 2: the binary itself ----------------------------------------------

def arena_binary(arena, meta):
    """The arena's copy of meta.json -> detected.primary.path.

    workspace.py copies the primary into `<arena>/target/`; the dataset preserves recursive
    extraction shapes (`bin/CrackMe_3.zip.__x/CrackMe_3.exe`), so both the full relative
    shape and the flat basename are tried before giving up. The dataset original is never
    run: it is read-only input, and a crackme is untrusted code that belongs in the arena.
    """
    if not arena:
        return None
    rel = ((meta.get("detected") or {}).get("primary") or {}).get("path") or ""
    base = os.path.basename(rel)
    stripped = rel.split("/", 1)[1] if "/" in rel else rel
    for cand in (os.path.join(arena, "target", stripped),
                 os.path.join(arena, "target", rel),
                 os.path.join(arena, "target", base),
                 os.path.join(arena, rel)):
        if os.path.isfile(cand):
            return cand
    troot = os.path.join(arena, "target")
    for dirpath, _dirs, files in os.walk(troot):
        if base in files:
            return os.path.join(dirpath, base)
    return None


def _run_binary(path, argv, stdin_text, timeout):
    """`timeout -k 2 N` in front of the argv, not just a subprocess timeout.

    A crackme may fork, spawn a shell, or block on a tty; coreutils `timeout` signals the
    whole job and then SIGKILLs it, where subprocess.timeout only kills the direct child.
    """
    cmd = [path] + list(argv)
    if shutil.which("timeout"):
        cmd = ["timeout", "-k", "2", str(timeout)] + cmd
    env = {"PATH": "/usr/bin:/bin", "HOME": os.path.dirname(path), "LANG": "C",
           "TERM": "dumb"}
    with tempfile.TemporaryDirectory(prefix="repipe-grade-") as tmp:
        try:
            proc = subprocess.run(cmd, input=(stdin_text or "").encode(),
                                  stdout=subprocess.PIPE, stderr=subprocess.PIPE,
                                  cwd=tmp, env=env, timeout=timeout + 5)
        except (OSError, subprocess.SubprocessError) as exc:
            return None, "", "%s: %s" % (type(exc).__name__, exc)
    return (proc.returncode,
            proc.stdout.decode("utf-8", "replace"),
            proc.stderr.decode("utf-8", "replace"))


def _tier_binary(meta, arena, name, value, timeout=None):
    """Best-effort and frankly heuristic: crackmes share no success protocol.

    The name/serial is fed both on stdin (the common shape) and on argv, and the output is
    keyword-matched. A failure keyword beats a success keyword, because "invalid serial
    format -- correct length though" is a rejection. If neither fires the tier returns
    `inconclusive` rather than guessing from the exit code, which crackmes use arbitrarily.
    """
    timeout = BINARY_TIMEOUT if timeout is None else int(timeout)
    tier = {"tier": "binary-accepts", "confidence": "high", "heuristic": True}
    if not value:
        tier.update(result="skipped", why="the report carried no answer")
        return tier
    if not arena:
        tier.update(result="skipped", why="no arena given; the dataset copy is never run")
        return tier
    path = arena_binary(arena, meta)
    if not path:
        tier.update(result="skipped", why="no arena copy of the primary binary found")
        return tier
    if not os.access(path, os.X_OK):
        tier.update(result="skipped", path=path,
                    why="arena copy is not executable (workspace.py should chmod 0755; "
                        "4 of 287 shipped binaries are mode 600 and 54 are 644)")
        return tier

    fields = [name, value] if name else [value]
    attempts = [{"how": "stdin", "argv": [], "stdin": "\n".join(fields) + "\n"},
                {"how": "argv", "argv": fields, "stdin": ""}]
    tried = []
    for att in attempts:
        code, out, err = _run_binary(path, att["argv"], att["stdin"], timeout)
        blob = (out or "") + "\n" + (err or "")
        hit_ok = bool(_SUCCESS.search(blob))
        hit_no = bool(_FAILURE.search(blob))
        tried.append({"how": att["how"], "exit": code,
                      "success_keyword": hit_ok, "failure_keyword": hit_no,
                      "output_sha1": _digest(blob)})
        if hit_ok and not hit_no:
            tier.update(result="pass", path=path, attempts=tried,
                        why="the binary printed a success indication for this answer")
            return tier
    tier["path"] = path
    tier["attempts"] = tried
    if any(t["failure_keyword"] and not t["success_keyword"] for t in tried):
        tier.update(result="fail", why="the binary printed a rejection for this answer")
    else:
        tier.update(result="inconclusive",
                    why="the binary printed no recognisable accept or reject")
    return tier


# --- tier 3: the reconstructed verifier -------------------------------------

def _tier_verifier(meta, hexid, name, value, timeout=None):
    """`python3 verifier.py verify <name> <serial>` -- STDOUT only, subprocess only.

    The CLI contract: `verify` prints exactly `1` or `0` on stdout and exits 0 EITHER WAY, so
    the exit code carries no verdict at all; any other mode writes usage to stderr and exits
    2. Anything other than a clean `1`/`0` (a NotImplementedError traceback, a quarantined
    stub, a timeout) is `inconclusive`, never a rejection of the tester.
    """
    timeout = VERIFIER_TIMEOUT if timeout is None else int(timeout)
    gt = meta.get("ground_truth") or {}
    tier = {"tier": "verifier-agrees", "confidence": "low", "confidence_note": LOW_CONFIDENCE_NOTE}
    rel = gt.get("verifier")
    if not rel:
        tier.update(result="skipped", why="this challenge ships no verifier.py")
        return tier
    path = challenge_dir(hexid) / rel
    if not os.path.isfile(path):
        tier.update(result="skipped", why="meta names a verifier that is not on disk")
        return tier
    if not value:
        tier.update(result="skipped", why="the report carried no answer")
        return tier
    if not name and (gt.get("verifier_interface") or "") == "name+serial":
        tier.update(result="inconclusive",
                    why="verifier_interface is name+serial but the report gave no name")
        return tier

    tier["self_test_pass"] = gt.get("verifier_self_test_pass")
    cmd = [sys.executable, str(path), "verify", name or "", value]
    with tempfile.TemporaryDirectory(prefix="repipe-verify-") as tmp:
        try:
            proc = subprocess.run(cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE,
                                  cwd=tmp, timeout=timeout)
        except subprocess.TimeoutExpired:
            tier.update(result="inconclusive", why="verifier.py timed out after %ds" % timeout)
            return tier
        except OSError as exc:
            tier.update(result="inconclusive", why="could not run verifier.py: %s" % exc)
            return tier
    out = proc.stdout.decode("utf-8", "replace").strip()
    err = proc.stderr.decode("utf-8", "replace").strip()
    tier["exit"] = proc.returncode
    tier["stdout"] = out[:40]
    if out == "1":
        tier.update(result="pass", why="verifier.py printed 1 (LOW confidence, see note)")
    elif out == "0":
        tier.update(result="fail", why="verifier.py printed 0 (LOW confidence, see note)")
    else:
        reason = "NotImplementedError" if "NotImplementedError" in err else "unusable output"
        tier.update(result="inconclusive",
                    why="verifier.py produced %s, not a 1/0 verdict" % reason,
                    stderr_tail=err[-200:])
    return tier


# --- the verdict ------------------------------------------------------------

def _effective_outcome(reported, solved, confidence, contaminated):
    """What a round's coverage counts this attempt as.

    The tester's self-report is the default: it is the only possible source for `gave_up`,
    which is the outcome this pipeline most wants to see. Grading only ever DOWNGRADES it --
    a run claiming `solved` that the flag refutes is recorded `partial`, and a contaminated
    run is `voided` whatever it claimed. Grading never upgrades a `gave_up` into a `solved`,
    because a tester that quit did not solve anything no matter what its answer field says.
    """
    if contaminated:
        return "voided"
    if reported is None:
        return "solved" if solved else None
    if reported == "solved" and not solved and confidence == "high":
        return "partial"
    return reported


def grade(hexid, report, arena=None):
    """Grade one tester report against the strongest ground truth this challenge has.

    Tiers are tried in descending confidence and the FIRST decisive one wins; a lower tier
    never overrides a higher one, so a hallucinating verifier cannot promote an answer the
    flag already refuted. A tier that cannot run is `skipped` and a tier that ran without
    deciding is `inconclusive` -- neither counts against the tester.

    Returns a dict carrying every tier's outcome, so the dashboard can show *why* a run was
    called solved. It never carries the flag or the raw answer: an answer that matched IS the
    flag, and these records are persisted and rendered.
    """
    meta = load_meta(hexid)
    rep = load_report(report)
    name, value, kind = _answer_of(rep)
    gt = meta.get("ground_truth") or {}

    tiers = [_tier_flag(meta, value),
             _tier_binary(meta, arena, name, value),
             _tier_verifier(meta, hexid, name, value)]

    verdict, solved, confidence, why = "unverifiable", False, "none", None
    for tier in tiers:
        if tier["result"] == "pass":
            verdict, solved, confidence, why = tier["tier"], True, tier["confidence"], tier["why"]
            break
        if tier["result"] == "fail":
            verdict = TIER_NEGATIVE[tier["tier"]]
            solved, confidence, why = False, tier["confidence"], tier["why"]
            break
    if verdict == "unverifiable":
        if not value:
            verdict, why = "no-answer", "the tester reported no answer to grade"
        else:
            why = "no tier could decide: %s" % "; ".join(
                "%s %s" % (t["tier"], t["result"]) for t in tiers)

    out = {
        "schema": SCHEMA,
        "hexid": hexid,
        "challenge": meta.get("name"),
        "verdict": verdict,
        "solved": solved,
        "confidence": confidence,
        "why": why,
        "outcome": _effective_outcome(rep.get("outcome"), solved, confidence, False),
        "reported_outcome": rep.get("outcome"),
        "gave_up_reason": rep.get("gave_up_reason"),
        "answer_kind": kind,
        "answer_present": bool(value),
        "answer_sha1": _digest(value),
        "answer_name": name,
        "machine_checkable": bool(gt.get("machine_checkable")),
        "ground_truth": {
            "has_flag": bool(gt.get("flag")),
            "verifier": gt.get("verifier"),
            "verifier_self_test_pass": gt.get("verifier_self_test_pass"),
            "verifier_interface": gt.get("verifier_interface"),
        },
        "tiers": tiers,
        "contaminated": False,
        "outcome_voided": False,
        "graded_at": datetime.datetime.now().replace(microsecond=0).isoformat(),
    }
    if confidence == "low":
        out["confidence_note"] = LOW_CONFIDENCE_NOTE
    return out


# --- the tripwire -----------------------------------------------------------

def _builtin_scan(text, meta):
    """Fallback for when redact.py is not importable, so grading is never silently blind.

    Same four signals the runbook names: the dataset path, the literal flag, `crackmes.one`
    and `solutions/`. redact.scan_for_leak is preferred because it is the module that owns
    the spoiler vocabulary; this exists so a missing sibling degrades loudly, not quietly.
    """
    hits = []
    flag = (meta.get("ground_truth") or {}).get("flag")
    hexid = meta.get("hexid") or ""
    probes = [(str(config.dataset_root()), "dataset-path"),
              ("kuna-re-dataset", "dataset-name"),
              ("crackmes.one", "crackmes-one"),
              ("solutions/", "solutions-dir")]
    if hexid:
        probes.append((hexid, "hexid"))
    for needle, label in probes:
        if needle and needle in text:
            hits.append("%s: transcript contains %r" % (label, needle))
    if flag and flag in text:
        hits.append("flag: transcript contains the literal ground_truth.flag")
    return hits


def tripwire(transcript_text, hexid):
    """Did this run see the answer rather than work it out?

    A hit VOIDS THE OUTCOME but KEEPS THE OBSERVATIONS. That asymmetry is deliberate: a
    tester that read `solutions/<hexid>/` has told us nothing about whether the binary was
    solvable with kuna, but the twenty minutes it spent fighting `kuna functions` before it
    cheated are still twenty minutes of real friction, and friction is the product. Throwing
    the whole run away would delete evidence to punish a sandbox failure -- and with
    REPIPE_SANDBOX=none the sandbox failure is the pipeline's own.

    Hits are scrubbed of the flag before they are returned; these dicts are persisted.
    """
    meta = load_meta(hexid)
    text = transcript_text or ""
    try:
        from . import redact
        hits = list(redact.scan_for_leak(text, meta))
        scanner = "redact.scan_for_leak"
    except Exception:
        hits = _builtin_scan(text, meta)
        scanner = "builtin-fallback"
    flag = (meta.get("ground_truth") or {}).get("flag")
    if flag:
        hits = [h.replace(flag, "<flag>") if isinstance(h, str) else h for h in hits]
    contaminated = bool(hits)
    return {
        "hexid": hexid,
        "contaminated": contaminated,
        "hits": hits,
        "scanner": scanner,
        "outcome_voided": contaminated,
        "observations_kept": True,
        "sandbox": config.sandbox_mode(),
    }


def apply_tripwire(verdict, tw):
    """Fold a tripwire result into a verdict: the outcome dies, the observations do not."""
    out = dict(verdict)
    out["contaminated"] = bool(tw.get("contaminated"))
    out["tripwire"] = {"hits": tw.get("hits", []), "scanner": tw.get("scanner")}
    if tw.get("contaminated"):
        out["graded_verdict"] = verdict.get("verdict")
        out["verdict"] = "voided-contaminated"
        out["solved"] = False
        out["confidence"] = "none"
        out["outcome"] = "voided"
        out["outcome_voided"] = True
        out["observations_kept"] = True
        out["why"] = ("outcome voided: the transcript shows the answer was available to the "
                      "tester. Observations from this run are still ingested.")
    return out


# --- persistence ------------------------------------------------------------

def outcome_path(round_n, hexid):
    return config.rounds_dir() / str(round_n) / "outcomes" / ("%s.json" % os.path.basename(hexid))


def record(round_n, verdict):
    """Persist one graded outcome for sample.coverage() and the dashboard. Atomic, and one
    file per challenge so concurrent T_DRAIN grading never interleaves."""
    p = outcome_path(round_n, verdict["hexid"])
    os.makedirs(os.path.dirname(p), exist_ok=True)
    tmp = str(p) + ".tmp"
    with open(tmp, "w") as fh:
        json.dump(verdict, fh, indent=2)
        fh.write("\n")
    os.replace(tmp, p)
    return p


# --- CLI --------------------------------------------------------------------

def _print_verdict(v):
    print("%s  %s" % (v["hexid"], v["challenge"]))
    print("  verdict    %s   solved=%s   confidence=%s"
          % (v["verdict"], v["solved"], v["confidence"]))
    print("  why        %s" % v["why"])
    print("  answer     kind=%s present=%s sha1=%s"
          % (v["answer_kind"], v["answer_present"], v["answer_sha1"]))
    for t in v["tiers"]:
        print("  tier %-16s %-13s %s" % (t["tier"], t["result"], t.get("why", "")))
    if v.get("confidence_note"):
        print("  NOTE       %s" % v["confidence_note"])
    if v.get("contaminated"):
        print("  CONTAMINATED  outcome voided, observations kept")


def main(argv=None):
    p = argparse.ArgumentParser(prog="python -m scripts.repipe.grade")
    sub = p.add_subparsers(dest="cmd", required=True)

    sp = sub.add_parser("verdict", help="grade one tester report")
    sp.add_argument("--hexid", required=True)
    sp.add_argument("--report", default=None, help="path to the tester's report.json")
    sp.add_argument("--answer", default=None, help="answer value, if there is no report")
    sp.add_argument("--name", default=None, help="username, for name+serial crackmes")
    sp.add_argument("--arena", default=None, help="arena dir, enables the binary-accepts tier")
    sp.add_argument("--transcript", default=None, help="rollout to run the tripwire over")
    sp.add_argument("--round", type=int, default=None)
    sp.add_argument("--record", action="store_true", help="persist to rounds/<N>/outcomes/")
    sp.add_argument("--json", action="store_true")

    sp = sub.add_parser("tripwire", help="scan a rollout for spoiler leakage")
    sp.add_argument("--hexid", required=True)
    sp.add_argument("--transcript", required=True, help="file, or - for stdin")
    sp.add_argument("--json", action="store_true")

    args = p.parse_args(argv)

    if args.cmd == "tripwire":
        text = sys.stdin.read() if args.transcript == "-" else \
            open(args.transcript, errors="replace").read()
        tw = tripwire(text, args.hexid)
        if args.json:
            print(json.dumps(tw, indent=2))
        else:
            print("contaminated=%s  scanner=%s  sandbox=%s"
                  % (tw["contaminated"], tw["scanner"], tw["sandbox"]))
            for h in tw["hits"]:
                print("  hit  %s" % h)
            if tw["contaminated"]:
                print("  outcome voided; observations kept")
        return 1 if tw["contaminated"] else 0

    if args.report:
        report = load_report(args.report)
    else:
        report = {"answer": {"kind": "flag", "value": args.answer, "name": args.name}}
    arena = args.arena
    if arena is None and args.round is not None:
        cand = config.arena_dir() / str(args.round) / args.hexid
        arena = str(cand) if os.path.isdir(cand) else None

    verdict = grade(args.hexid, report, arena=arena)
    if args.transcript:
        text = sys.stdin.read() if args.transcript == "-" else \
            open(args.transcript, errors="replace").read()
        verdict = apply_tripwire(verdict, tripwire(text, args.hexid))
    if args.record:
        if args.round is None:
            print("--record needs --round", file=sys.stderr)
            return 2
        path = record(args.round, verdict)
        if not args.json:
            print("wrote %s" % path)
    if args.json:
        print(json.dumps(verdict, indent=2))
    else:
        _print_verdict(verdict)
    return 0 if verdict["solved"] else 1


if __name__ == "__main__":
    sys.exit(main())
