"""The RE-friction loop's live dashboard: one refresher thread, many viewers.

    PYTHONPATH=<repo> python3 -m scripts.repipe.webui --port 8787 --bind 127.0.0.1
    PYTHONPATH=<repo> python3 -m scripts.repipe.webui --json     # one snapshot, no server

Stdlib only (http.server), loopback, no auth, and READ-ONLY by design: the STOP control
displays the path to ``touch`` rather than posting anything, so a stray browser tab can
never stop a run.

Why a refresher thread instead of collecting per request
--------------------------------------------------------
``scripts.pipeline.status.collect()`` shells out to ``git worktree list`` AND ``gh api``.
A cold ``collect()`` measured **10.65 s** on this repo. At SSE cadence, or with three
browser tabs open, per-request collection is a fork bomb and a GitHub rate-limit incident.
So exactly one background thread owns every expensive read, publishes an in-memory payload
plus ``.kuna-repipe/webui-cache.json`` (atomic, monotonic ``seq``), and **request handlers
never shell out** — they slice the cache. N viewers cost what one costs.

The per-source TTLs are ``status.py``'s own cache, not a second one: ``status.collect()``
already holds ``git worktree list`` for ``KUNA_PIPELINE_WORKTREE_TTL`` (20 s) and the open-PR
``gh api`` for ``KUNA_PIPELINE_PR_TTL`` (60 s), so wrapping the whole call at a 1 s TTL gives
inventory 1 s / worktrees 20 s / PRs 60 s in one place. Check-runs (60 s, and only for PRs the
inventory believes are in flight) and ``du`` (60 s) go through the same ``status._cached``.
A failed fetch is cached as ``None`` with ``stale_since`` and rendered "(gh unavailable)",
exactly as ``status.py`` renders it today. Every subprocess this module starts gets
``timeout=20``.

``seq`` advances only when the payload actually changes: elapsed/stale counters are derived
in the browser from ``started_at``/``updated_at`` so a quiet pipeline produces no churn, and
an SSE client that sees ``{"seq": N}`` knows a refetch is worth making.

State dir binding
-----------------
``scripts.pipeline.state`` is multi-pipeline through ``KUNA_PIPELINE_STATE_DIR``; the RE loop
points it at ``.kuna-repipe``. This module sets that binding (plus ``status.py``'s worktree
and branch match seams) before it reads anything, so the inventory it shows is the RE loop's,
not the angr fleet's.

Degradation is the normal case, not the error case
--------------------------------------------------
``needs.py`` / ``verify.py`` / ``sample.py`` are siblings under construction and are imported
defensively; ``.kuna-repipe/`` may not exist at all. Every route answers 200 with an empty
shape rather than 500, because "the pipeline has never run" is the state this is booted in.
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
import threading
import time
import urllib.parse
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path

from . import config as rconfig
from ..pipeline import status as pstatus

# Siblings written in parallel with this file. A dashboard that 500s because a module it
# wants does not exist yet is worse than one that shows an empty pane.
try:
    from . import needs as needs_mod
except Exception:
    needs_mod = None
try:
    from . import verify as verify_mod
except Exception:
    verify_mod = None
try:
    from . import sample as sample_mod
except Exception:
    sample_mod = None


WEBUI_DIR = rconfig.repo_root() / "tools" / "repipe" / "webui"
SITE_CSS = rconfig.repo_root() / "integrations" / "web" / "assets" / "css" / "site.css"
SITE_ASSETS = rconfig.repo_root() / "integrations" / "web" / "assets"
SITE_FONTS = SITE_ASSETS / "fonts"

TEST_LANE = ["T_PLAN", "T_WORKSPACE", "T_FANOUT", "T_DRAIN", "T_GATE",
             "T_DEDUP", "T_REFUTE", "T_TRIAGE", "T_READY"]
BUILD_LANE = ["B_IDLE", "B_PLAN", "B_FANOUT", "B_DRAIN", "B_MERGE", "B_VERIFY", "B_DONE"]
STALE_SECONDS = 120          # status.py's convention: past this a worker's row goes red
DISK_RED_GB = 100

CHECKS_TTL = 60.0
DU_TTL = 60.0
COLLECT_TTL = 1.0            # the inventory tier; worktrees/PRs keep status.py's own TTLs
MAX_CHECK_CALLS = 6          # hard ceiling on gh calls per refresh tick

NEED_ID_RE = re.compile(r"^[A-Za-z0-9][A-Za-z0-9._-]{0,79}$")
PROBE_ID_RE = re.compile(r"^[pa]-[0-9a-f]{12}$")
AGENT_ID_RE = re.compile(r"^[A-Za-z0-9][A-Za-z0-9._@:+-]{0,63}$")

# Dropped before the change-digest is taken: these tick every second by construction and
# would otherwise advance seq forever on an idle pipeline.
VOLATILE_KEYS = {"ts", "now", "age_s", "uptime_s", "cache", "elapsed_s", "stale_s",
                 "round_elapsed_s", "since_s", "generated_at", "seq"}


# --- environment binding ----------------------------------------------------

def bind_state_dir(state_dir=None):
    """Point scripts.pipeline.state/status at the RE loop's state dir and worktrees."""
    if state_dir:
        os.environ["KUNA_PIPELINE_STATE_DIR"] = str(Path(state_dir).expanduser())
    else:
        os.environ.setdefault("KUNA_PIPELINE_STATE_DIR",
                              str(rconfig.repo_root() / rconfig.STATE_DIRNAME))
    os.environ.setdefault("KUNA_PIPELINE_WORKTREE_MATCH", rconfig.STATE_DIRNAME)
    os.environ.setdefault("KUNA_PIPELINE_BRANCH_MATCH", "feat/re-")
    return Path(os.environ["KUNA_PIPELINE_STATE_DIR"])


# --- small readers ----------------------------------------------------------

def _ttl(key, ttl, fetch):
    """status.py's process-wide TTL cache. Deliberately not a second implementation."""
    return pstatus._cached(key, ttl, fetch)


def _read_json(path, default=None):
    try:
        with open(path) as fh:
            return json.load(fh)
    except Exception:
        return default


def _mtime_sig(paths):
    """A cheap change signature: (count, newest mtime, total size) over a file set."""
    n = 0
    newest = 0.0
    total = 0
    for p in paths:
        try:
            st = os.stat(p)
        except OSError:
            continue
        n += 1
        newest = max(newest, st.st_mtime)
        total += st.st_size
    return (n, round(newest, 3), total)


TAIL_MAX_BYTES = 4 << 20


def _tail_lines(path, n):
    """Last n lines, read backwards, and never more than TAIL_MAX_BYTES.

    A codex rollout log is newline-dense, but a crashed writer can leave one enormous
    line; the byte cap is what stops `tail=5000` from paging a GB into the response.
    """
    try:
        size = os.path.getsize(path)
    except OSError:
        return []
    block = 65536
    want = max(1, int(n))
    data = b""
    with open(path, "rb") as fh:
        pos = size
        while pos > 0 and data.count(b"\n") <= want and len(data) < TAIL_MAX_BYTES:
            step = min(block, pos)
            pos -= step
            fh.seek(pos)
            data = fh.read(step) + data
    text = data.decode("utf-8", "replace")
    lines = text.splitlines()
    return lines[-want:]


def _jsonl(path, limit=None):
    out = []
    try:
        with open(path, errors="replace") as fh:
            for line in fh:
                line = line.strip()
                if not line:
                    continue
                try:
                    out.append(json.loads(line))
                except ValueError:
                    out.append({"raw": line})
    except OSError:
        return []
    if limit:
        return out[-limit:]
    return out


def _scalar(v):
    v = v.strip()
    if v in ("null", "~", ""):
        return None
    if v in ("true", "True"):
        return True
    if v in ("false", "False"):
        return False
    if v.startswith("[") and v.endswith("]"):
        inner = v[1:-1].strip()
        if not inner:
            return []
        return [_scalar(x) for x in inner.split(",")]
    if len(v) >= 2 and v[0] == v[-1] and v[0] in "\"'":
        return v[1:-1]
    try:
        return int(v)
    except ValueError:
        pass
    try:
        return float(v)
    except ValueError:
        pass
    return v


def _front_matter(text):
    """The docs/decbench/triage/*.md dialect that scripts/decbench/status.py already parses,
    plus inline ``[a, b]`` lists, which the need record uses for challenges/rounds/touches."""
    m = re.match(r"---\n(.*?)\n---", text, re.S)
    if not m:
        return {}
    fm = {}
    for line in m.group(1).splitlines():
        kv = re.match(r"^([A-Za-z_][\w.-]*):\s*(.*?)\s*$", line)
        if kv:
            fm[kv.group(1)] = _scalar(kv.group(2))
    return fm


def _sections(text):
    """The fixed ``## `` sections of a need record, as {heading: body}."""
    body = re.sub(r"^---\n.*?\n---\n?", "", text, flags=re.S)
    out = {}
    cur = None
    buf = []
    for line in body.splitlines():
        h = re.match(r"^##\s+(.*?)\s*$", line)
        if h:
            if cur:
                out[cur] = "\n".join(buf).strip()
            cur = h.group(1)
            buf = []
        elif cur:
            buf.append(line)
    if cur:
        out[cur] = "\n".join(buf).strip()
    return out


def _lexically_under(root, candidate):
    """True iff ``candidate`` is under ``root`` by path arithmetic, without following links.

    Used where the leaf name is already constrained by a regex and the tree is allowed to
    contain symlinks — a docs/ checkout that symlinks its need records is legitimate, and
    resolving those would reject it. Where the path comes from recorded state instead of a
    validated id (an agent's log), use _safe_under, which does follow links.
    """
    r = os.path.normpath(str(root))
    c = os.path.normpath(str(candidate))
    return c == r or c.startswith(r + os.sep)


# An id may only ever be these characters. No separator, no dot-dot, no NUL, nothing that can
# leave a directory -- so a tainted URL component cannot reach a filesystem path at all. This
# is deliberately stricter than _safe_under, which stays as defence in depth.
_ID_RE = re.compile(r"^[A-Za-z0-9][A-Za-z0-9._-]{0,63}$")


def safe_id(value):
    """The id if it is well-formed, else None. Applied at every route that names a file."""
    if not isinstance(value, str) or ".." in value:
        return None
    return value if _ID_RE.match(value) else None


def _safe_under(root, candidate):
    """True iff ``candidate`` really lives under ``root`` after symlinks are resolved."""
    try:
        r = os.path.realpath(str(root))
        c = os.path.realpath(str(candidate))
    except OSError:
        return False
    return c == r or c.startswith(r + os.sep)


# --- expensive sources, each behind status.py's TTL cache -------------------

def _collect_snapshot():
    """Inventory (1 s) + worktrees (20 s) + open PRs (60 s), all via status.py's own TTLs."""
    try:
        return _ttl("webui:collect", COLLECT_TTL, pstatus.collect) or {}
    except Exception:
        return {}


def _check_runs(branch, gh_repo):
    def fetch():
        out = subprocess.run(
            ["gh", "api", "repos/%s/commits/%s/check-runs" % (gh_repo, branch),
             "--jq", "[.check_runs[] | {name, status, conclusion}]"],
            capture_output=True, text=True, timeout=20,
        )
        if out.returncode != 0:
            return None
        return json.loads(out.stdout or "[]")
    return _ttl("webui:checks:" + branch, CHECKS_TTL, fetch)


def _disk():
    def fetch():
        try:
            usage = shutil.disk_usage(str(rconfig.repo_root()))
        except OSError:
            return None
        return {"free_gb": round(usage.free / 1e9, 1),
                "total_gb": round(usage.total / 1e9, 1),
                "used_pct": round(100.0 * usage.used / max(1, usage.total), 1)}
    return _ttl("webui:df", DU_TTL, fetch)


def _state_dir_bytes(state_dir, enabled):
    if not enabled or not os.path.isdir(state_dir):
        return None

    def fetch():
        out = subprocess.run(["du", "-sx", "-B1", str(state_dir)],
                             capture_output=True, text=True, timeout=20)
        if out.returncode != 0 or not out.stdout.strip():
            return None
        return int(out.stdout.split()[0])
    return _ttl("webui:du", DU_TTL, fetch)


# --- supervisor, rounds, transitions ---------------------------------------

def _supervisor(state_dir, agents):
    """The Supervisor machine's state, from its own file or inferred from the stop files."""
    sup = _read_json(state_dir / "supervisor.json", {}) or {}
    flags = {name: (state_dir / name).exists()
             for name in ("STOP", "PAUSE", "ABORT", "HALT_REASON")}
    state = sup.get("state")
    if not state:
        if flags["ABORT"]:
            state = "ABORTED"
        elif flags["HALT_REASON"]:
            state = "HALTED"
        elif flags["STOP"]:
            state = "DRAINING"
        elif any(a.get("status") == "running" for a in agents):
            state = "RUNNING"
        else:
            state = "IDLE"
    halt_reason = None
    if flags["HALT_REASON"]:
        try:
            halt_reason = (state_dir / "HALT_REASON").read_text(errors="replace").strip()[:400]
        except OSError:
            halt_reason = "(unreadable)"
    return {
        "state": state,
        "run_id": sup.get("run_id"),
        "started_at": sup.get("started_at"),
        "rounds_planned": sup.get("rounds", rconfig.ROUNDS),
        "flags": flags,
        "halt_reason": halt_reason,
        "stop_path": str(state_dir / "STOP"),
        "abort_path": str(state_dir / "ABORT"),
    }


def _round_dirs(state_dir):
    rd = state_dir / "rounds"
    if not rd.is_dir():
        return []
    out = []
    for child in rd.iterdir():
        if child.is_dir() and child.name.isdigit():
            out.append((int(child.name), child))
    return sorted(out)


def _lane_state(transitions, track, lane):
    """Current state of one track: the destination of its last recorded transition."""
    for t in reversed(transitions):
        if t.get("track") == track and t.get("to") in lane:
            return t["to"]
    for t in reversed(transitions):
        if t.get("track") == track and t.get("to"):
            return t["to"]
    return None


def _collect_rounds(state_dir):
    rounds = []
    for num, path in _round_dirs(state_dir):
        meta = _read_json(path / "round.json", {}) or {}
        trans = _jsonl(path / "transitions.jsonl")
        tracks = meta.get("tracks") or {}
        test_state = (tracks.get("test") or {}).get("state") or _lane_state(trans, "test", TEST_LANE)
        build_state = (tracks.get("build") or {}).get("state") or _lane_state(trans, "build", BUILD_LANE)
        acceptance = _read_json(path / "acceptance.json")
        slate = _read_json(path / "slate.json", []) or []
        if isinstance(slate, dict):
            slate = slate.get("challenges") or slate.get("slate") or []
        started = meta.get("started_at")
        if started is None and trans:
            started = trans[0].get("ts")
        rounds.append({
            "round": num,
            "started_at": started,
            "ended_at": meta.get("ended_at"),
            "test_state": test_state,
            "build_state": build_state,
            "barrier": meta.get("barrier") or ("INTEGRATE" if acceptance is not None else None),
            "integrated": acceptance is not None,
            "slate": [s if isinstance(s, str) else (s.get("hexid") or s.get("id"))
                      for s in slate],
            "transitions": len(trans),
            "tail": trans[-40:],
            "usd": meta.get("usd"),
            "needs_filed": meta.get("needs_filed"),
            "needs_closed": meta.get("needs_closed"),
            "path": str(path),
        })
    return rounds


# --- agents -----------------------------------------------------------------

def _agent_role(worker, slots):
    for pool, entry in (slots or {}).items():
        if worker.get("worker") in (entry.get("held") or {}):
            return pool, (entry["held"][worker["worker"]] or {}).get("kind") or pool
    explicit = worker.get("role") or worker.get("pool")
    if explicit:
        return explicit, explicit
    wid = worker.get("worker") or ""
    if wid.startswith("t-") or wid.startswith("tester"):
        return "tester", "tester"
    if wid.startswith("c-") or "captain" in wid:
        return "captain", "captain"
    return "builder", "builder"


def _agent_log_path(state_dir, agent_id, recorded=None):
    """Resolve an agent's log strictly inside the state dir.

    The returned path is built from a DIRECTORY LISTING, never from ``agent_id``: the id is
    only ever compared, by equality, against names the filesystem produced. That is what
    makes traversal structurally impossible rather than merely guarded -- and it is what a
    taint analysis can actually see, which a regex guard on the id is not.
    """
    if recorded:
        cand = Path(recorded)
        if not cand.is_absolute():
            cand = state_dir / cand
        if _safe_under(state_dir, cand) and cand.is_file():
            return cand
    if safe_id(agent_id) is None:
        return None
    logs = state_dir / "logs"
    wanted = {"%s.jsonl" % agent_id, "%s.log" % agent_id,
              "driver-%s.log" % agent_id, "%s.out" % agent_id}
    try:
        entries = sorted(os.listdir(str(logs)))
    except OSError:
        return None
    for entry in entries:
        if entry in wanted:
            cand = logs / entry          # `entry` came from the filesystem, not the request
            if cand.is_file() and _safe_under(state_dir, cand):
                return cand
    return None


def _collect_agents(state_dir, snap, gh_repo, want_checks=True):
    workers = snap.get("workers") or []
    slots = snap.get("slots") or {}
    leases = snap.get("leases") or {}
    prs = snap.get("prs")
    pr_by_branch = {}
    for pr in (prs or []):
        pr_by_branch[pr.get("headRefName")] = pr
    now = time.time()
    calls = 0
    agents = []
    for w in workers:
        wid = w.get("worker") or "?"
        pool, role = _agent_role(w, slots)
        started = w.get("started_at") or now
        updated = w.get("updated_at") or started
        held = sorted(res for res, l in leases.items() if l.get("holder") == wid)
        sidecar = _read_json(state_dir / "agents" / ("%s.json" % wid), {}) or {} \
            if AGENT_ID_RE.match(wid) else {}
        rec = {
            "id": wid,
            "role": role,
            "pool": pool,
            "phase": w.get("phase"),
            "status": w.get("status"),
            "started_at": started,
            "updated_at": updated,
            "elapsed_s": int(now - started),
            "stale_s": int(now - updated),
            "pid": w.get("pid"),
            "slug": w.get("slug"),
            "need": w.get("need") or w.get("opportunity") or sidecar.get("need"),
            "note": w.get("note"),
            "usd": w.get("usd", sidecar.get("usd")),
            "tokens_in": w.get("tokens_in", sidecar.get("tokens_in")),
            "tokens_out": w.get("tokens_out", sidecar.get("tokens_out")),
            "model": w.get("model") or sidecar.get("model"),
            "leases": held,
            "log": None,
            "has_log": False,
        }
        if role == "tester":
            rec["challenge"] = w.get("challenge") or sidecar.get("challenge")
            rec["stratum"] = w.get("stratum") or sidecar.get("stratum")
            rec["outcome"] = w.get("outcome") or sidecar.get("outcome")
            rec["gave_up_reason"] = w.get("gave_up_reason") or sidecar.get("gave_up_reason")
            rec["fallbacks"] = w.get("fallbacks", sidecar.get("fallbacks"))
        else:
            branch = w.get("branch")
            rec["branch"] = branch
            rec["pr_url"] = w.get("pr_url")
            pr = pr_by_branch.get(branch)
            rec["pr"] = pr.get("number") if pr else None
            rec["ci"] = None
            in_flight = bool(branch) and w.get("status") in ("running", "pr", "proposal")
            if want_checks and in_flight and pr and calls < MAX_CHECK_CALLS:
                calls += 1
                runs = _check_runs(branch, gh_repo)
                rec["ci"] = _ci_verdict(runs)
        lp = _agent_log_path(state_dir, wid, w.get("log") or sidecar.get("log"))
        if lp is not None:
            rec["log"] = str(lp)
            rec["has_log"] = True
        agents.append(rec)
    agents.sort(key=lambda a: (a["role"] != "captain", a.get("started_at") or 0))
    return agents


def _ci_verdict(runs):
    if runs is None:
        return {"state": "unknown", "detail": "(gh unavailable)"}
    if not runs:
        return {"state": "none", "detail": "no checks"}
    done = [r for r in runs if r.get("status") == "completed"]
    bad = [r for r in done if r.get("conclusion") not in ("success", "neutral", "skipped")]
    if bad:
        return {"state": "fail", "detail": ", ".join(r.get("name", "?") for r in bad[:3])}
    if len(done) < len(runs):
        return {"state": "running", "detail": "%d/%d complete" % (len(done), len(runs))}
    return {"state": "pass", "detail": "%d checks" % len(runs)}


# --- needs ------------------------------------------------------------------
#
# Three sources, in descending fidelity: needs.py's own reader when that module has landed,
# the docs/re-needs/index.json cache, then a front-matter scan of the records themselves.
# The scan is the floor, so an empty or half-written docs/re-needs/ still renders.

_NEEDS_API = ("load_all", "all_records", "load_index")


def _needs_from_module():
    if needs_mod is None:
        return None
    for name in _NEEDS_API:
        fn = getattr(needs_mod, name, None)
        if not callable(fn):
            continue
        try:
            recs = fn()
        except Exception:
            continue
        if isinstance(recs, dict):
            recs = recs.get("needs") or recs.get("ranked") or list(recs.values())
        if isinstance(recs, list) and all(isinstance(r, dict) for r in recs):
            return recs
    return None


def _needs_from_index(needs_dir):
    doc = _read_json(needs_dir / "index.json")
    if doc is None:
        return None
    if isinstance(doc, dict):
        doc = doc.get("needs") or doc.get("ranked") or doc.get("records")
    if isinstance(doc, list) and all(isinstance(r, dict) for r in doc):
        return doc
    return None


def _needs_from_disk(needs_dir, rejected_dir):
    recs = []
    for directory, rejected in ((needs_dir, False), (rejected_dir, True)):
        if not directory.is_dir():
            continue
        for f in sorted(directory.glob("*.md")):
            try:
                text = f.read_text(errors="replace")
            except OSError:
                continue
            fm = _front_matter(text)
            if not fm:
                continue
            fm.setdefault("need_id", f.stem)
            if rejected:
                fm["status"] = "rejected"
            fm["_file"] = str(f)
            recs.append(fm)
    return recs


def _norm_need(rec):
    """One shape for the table, whatever source produced the record."""
    status = str(rec.get("status") or "open")
    return {
        "need_id": rec.get("need_id") or rec.get("id") or rec.get("slug") or "?",
        "title": rec.get("title") or "",
        "track": rec.get("track") or "?",
        "status": status,
        "severity": rec.get("severity") or "?",
        "probe_id": rec.get("probe_id"),
        "acceptance_id": rec.get("acceptance_id"),
        "probe_status": rec.get("probe_status"),
        "acceptance_status": rec.get("acceptance_status"),
        "hypothesis_status": rec.get("hypothesis_status") or "inconclusive",
        "credibility": rec.get("credibility"),
        "instances": rec.get("instances") or 0,
        "testers": rec.get("testers") or rec.get("distinct_testers"),
        "challenges": rec.get("challenges") or [],
        "rounds": rec.get("rounds") or [],
        "first_seen_round": rec.get("first_seen_round"),
        "attempts": rec.get("attempts") or 0,
        "scope": rec.get("scope"),
        "rank": rec.get("rank", rec.get("score")),
        "pr": rec.get("pr"),
        "regression_of": rec.get("regression_of"),
        "covered_by_option": rec.get("covered_by_option"),
        "reject_reason": rec.get("reject_reason") or rec.get("reason")
                         or rec.get("rejected_because"),
        "closed_in_round": rec.get("closed_in_round"),
        "touches": rec.get("touches") or [],
        "file": rec.get("_file") or rec.get("file"),
    }


_SEVERITY_ORDER = {"blocker": 0, "major": 1, "minor": 2}


def _rank_key(n):
    r = n.get("rank")
    return (0 if isinstance(r, (int, float)) else 1,
            -(r if isinstance(r, (int, float)) else 0),
            _SEVERITY_ORDER.get(n.get("severity"), 9),
            -float(n.get("credibility") or 0),
            -int(n.get("instances") or 0),
            n.get("need_id") or "")


def _collect_needs(needs_dir, rejected_dir):
    recs = _needs_from_module()
    source = "needs.py"
    if recs is None:
        recs = _needs_from_index(needs_dir)
        source = "index.json"
    if recs is None:
        recs = _needs_from_disk(needs_dir, rejected_dir)
        source = "records"
    items = [_norm_need(r) for r in recs]
    backlog = sorted((n for n in items if n["status"] != "rejected"), key=_rank_key)
    rejected = [n for n in items if n["status"] == "rejected"]
    by_reason = {}
    for n in rejected:
        by_reason.setdefault(n.get("reject_reason") or "unclassified", []).append(n)
    by_status = {}
    for n in backlog:
        by_status[n["status"]] = by_status.get(n["status"], 0) + 1
    filed = len(items)
    return {
        "source": source,
        "backlog": backlog,
        "rejected": rejected,
        "rejected_by_reason": {k: sorted(v, key=lambda n: n["need_id"])
                               for k, v in sorted(by_reason.items())},
        "by_status": by_status,
        "totals": {
            "filed": filed,
            "admitted": len(backlog),
            "rejected": len(rejected),
            # The honest denominator: how much of what the testers filed was the tester
            # being wrong rather than kuna being bad.
            "rejected_pct": round(100.0 * len(rejected) / filed, 1) if filed else 0.0,
            "overturned": sum(1 for n in backlog if n["hypothesis_status"] == "overturned"),
        },
    }


# --- corpus -----------------------------------------------------------------
#
# Coverage per stratum. This route reads manifest.json, which carries the plaintext answer in
# ground_truth.flag for 98 of the 250 challenges, so records are rebuilt field by field from
# an allowlist and NEVER copied wholesale. There is no code path here that emits a flag.

_CORPUS_FIELDS = ("hexid", "name", "bucket", "selected_for")


def _manifest_records():
    path = rconfig.manifest_path()
    try:
        sig = _mtime_sig([path])
    except Exception:
        return []
    cached = getattr(_manifest_records, "_cache", None)
    if cached and cached[0] == sig:
        return cached[1]
    doc = _read_json(path, [])
    if isinstance(doc, dict):
        doc = doc.get("records") or []
    out = []
    for r in doc if isinstance(doc, list) else []:
        det = r.get("detected") or {}
        prim = det.get("primary") or {}
        gt = r.get("ground_truth") or {}
        out.append({
            "hexid": r.get("hexid"),
            "name": r.get("name"),
            "bucket": r.get("bucket"),
            "stratum": r.get("selected_for") or "unassigned",
            "format": prim.get("format"),
            "arch": prim.get("arch"),
            "size": prim.get("size"),
            "difficulty": (r.get("declared") or {}).get("difficulty"),
            "machine_checkable": bool(gt.get("machine_checkable")),
            "verifier_self_test_pass": bool(gt.get("verifier_self_test_pass")),
            "ships_source_code": bool((r.get("contamination") or {}).get("ships_source_code")),
        })
    _manifest_records._cache = (sig, out)
    return out


def _collect_corpus(state_dir, rounds):
    records = _manifest_records()
    attempts = {}
    for rnd in rounds:
        for hexid in rnd.get("slate") or []:
            if hexid:
                attempts.setdefault(hexid, []).append({"round": rnd["round"], "outcome": None})
    arena = state_dir / "arena"
    if arena.is_dir():
        for rdir in arena.iterdir():
            if not rdir.is_dir():
                continue
            try:
                rnum = int(rdir.name)
            except ValueError:
                rnum = None
            for cdir in rdir.iterdir():
                if not cdir.is_dir():
                    continue
                rep = _read_json(cdir / "report.json", {}) or {}
                lst = attempts.setdefault(cdir.name, [])
                for a in lst:
                    if a["round"] == rnum:
                        a["outcome"] = rep.get("outcome")
                        break
                else:
                    lst.append({"round": rnum, "outcome": rep.get("outcome")})
    strata = {}
    for rec in records:
        s = strata.setdefault(rec["stratum"], {
            "stratum": rec["stratum"], "total": 0, "attempted": 0, "solved": 0,
            "gave_up": 0, "machine_checkable": 0})
        s["total"] += 1
        if rec["machine_checkable"]:
            s["machine_checkable"] += 1
        tries = attempts.get(rec["hexid"]) or []
        rec["attempts"] = len(tries)
        rec["rounds"] = [t["round"] for t in tries]
        rec["last_outcome"] = tries[-1]["outcome"] if tries else None
        if tries:
            s["attempted"] += 1
            outcomes = [t["outcome"] for t in tries]
            if "solved" in outcomes:
                s["solved"] += 1
            if "gave_up" in outcomes:
                s["gave_up"] += 1
    for s in strata.values():
        s["coverage_pct"] = round(100.0 * s["attempted"] / s["total"], 1) if s["total"] else 0.0
    total = len(records)
    attempted = sum(1 for r in records if r["attempts"])
    return {
        "strata": [strata[k] for k in sorted(strata)],
        "totals": {"total": total, "attempted": attempted,
                   "coverage_pct": round(100.0 * attempted / total, 1) if total else 0.0,
                   "unattempted": total - attempted},
        "challenges": records,
    }


# --- acceptance matrix ------------------------------------------------------
#
# Rows = every acceptance probe ever filed, columns = rounds, cells = pass/fail. A cell that
# was green and is now red is a regression, which is the whole point of the screen: it is the
# one view that answers "is the loop actually improving kuna".

def _acceptance_rows(doc):
    """rounds/<n>/acceptance.json in any of the shapes verify.py might reasonably emit."""
    if doc is None:
        return []
    if isinstance(doc, dict):
        for key in ("results", "probes", "acceptance"):
            if isinstance(doc.get(key), list):
                return doc[key]
        if all(isinstance(v, dict) for v in doc.values()) and doc:
            return [dict(v, probe_id=v.get("probe_id", k)) for k, v in doc.items()]
        return []
    if isinstance(doc, list):
        return doc
    return []


def _cell(row):
    for key in ("status", "result", "verdict", "outcome"):
        v = row.get(key)
        if isinstance(v, str):
            v = v.lower()
            if v in ("pass", "passed", "ok", "green"):
                return "pass"
            if v in ("fail", "failed", "red"):
                return "fail"
            if v in ("error", "timeout", "skip", "skipped"):
                return v
    if isinstance(row.get("passed"), bool):
        return "pass" if row["passed"] else "fail"
    if isinstance(row.get("ok"), bool):
        return "pass" if row["ok"] else "fail"
    return "unknown"


def _collect_acceptance(state_dir, rounds, need_index):
    by_probe = {}
    round_nums = []
    for rnd in rounds:
        n = rnd["round"]
        doc = _read_json(Path(rnd["path"]) / "acceptance.json")
        rows = _acceptance_rows(doc)
        if doc is None and not rows:
            continue
        round_nums.append(n)
        for row in rows:
            pid = row.get("probe_id") or row.get("acceptance_id") or row.get("id")
            if not pid:
                continue
            ent = by_probe.setdefault(pid, {"probe_id": pid, "cells": {}})
            ent["cells"][str(n)] = _cell(row)
            if row.get("need_id"):
                ent["need_id"] = row["need_id"]
    for need in need_index.values():
        aid = need.get("acceptance_id")
        if aid:
            ent = by_probe.setdefault(aid, {"probe_id": aid, "cells": {}})
            ent.setdefault("need_id", need["need_id"])
    probes = []
    for pid, ent in by_probe.items():
        need = need_index.get(ent.get("need_id") or "")
        cells = ent["cells"]
        seq = [(int(k), v) for k, v in sorted(cells.items(), key=lambda kv: int(kv[0]))]
        regressed = any(seq[i][1] == "fail" and any(p == "pass" for _, p in seq[:i])
                        for i in range(len(seq)))
        probes.append({
            "probe_id": pid,
            "need_id": ent.get("need_id"),
            "title": (need or {}).get("title") or "",
            "track": (need or {}).get("track"),
            "need_status": (need or {}).get("status"),
            "cells": cells,
            "first_pass_round": next((r for r, v in seq if v == "pass"), None),
            "latest": seq[-1][1] if seq else None,
            "regressed": regressed,
        })
    probes.sort(key=lambda p: (not p["cells"], not p["regressed"],
                               p["latest"] != "fail",
                               p.get("need_id") or "", p["probe_id"]))
    totals = {}
    for n in round_nums:
        col = [p["cells"].get(str(n)) for p in probes]
        totals[str(n)] = {"pass": col.count("pass"), "fail": col.count("fail"),
                          "other": sum(1 for c in col if c not in (None, "pass", "fail"))}
    return {
        "rounds": round_nums,
        "probes": probes,
        "totals": totals,
        "regressions": sum(1 for p in probes if p["regressed"]),
        "closed": sum(1 for p in probes if p["latest"] == "pass"),
        "outstanding": sum(1 for p in probes if p["latest"] == "fail"),
        "never_run": sum(1 for p in probes if not p["cells"]),
    }


# --- spend ------------------------------------------------------------------

def _replay_status(state_dir):
    """probe_id -> the status of its most recent replay, from .kuna-repipe/replays/*.jsonl.

    Rebuilt only when that directory's mtime signature moves, so the common case is a dict
    lookup rather than one open() per probe per tick.
    """
    d = state_dir / "replays"
    files = sorted(d.glob("*.jsonl")) if d.is_dir() else []
    sig = _mtime_sig(files)
    cached = getattr(_replay_status, "_cache", None)
    if cached and cached[0] == (str(d), sig):
        return cached[1]
    out = {}
    for f in files:
        rows = _jsonl(f, limit=1)
        if rows:
            out[f.stem] = _cell(rows[-1])
    _replay_status._cache = ((str(d), sig), out)
    return out


def _enrich_need_status(needs, acceptance, replays):
    """Fill a need's probe/acceptance status from the evidence rather than showing 'unrun'.

    A need record does not carry the replay verdict — the acceptance suite and the replay
    logs do — so the backlog row would otherwise read 'unrun' for a probe whose result the
    matrix on the next tab already knows.
    """
    latest = {p["probe_id"]: p["latest"] for p in acceptance.get("probes", [])
              if p.get("latest")}
    for n in needs.get("backlog", []) + needs.get("rejected", []):
        if not n.get("acceptance_status"):
            aid = n.get("acceptance_id")
            n["acceptance_status"] = latest.get(aid) or replays.get(aid)
        if not n.get("probe_status"):
            n["probe_status"] = replays.get(n.get("probe_id"))


def _collect_spend(state_dir, agents, rounds):
    doc = _read_json(state_dir / "spend.json", {}) or {}
    agent_usd = sum(float(a["usd"]) for a in agents
                    if isinstance(a.get("usd"), (int, float)))
    run_usd = doc.get("run_usd")
    if not isinstance(run_usd, (int, float)):
        run_usd = agent_usd
    by_round = doc.get("round_usd") or {}
    cur = rounds[-1]["round"] if rounds else None
    round_usd = by_round.get(str(cur))
    if not isinstance(round_usd, (int, float)) and rounds:
        round_usd = rounds[-1].get("usd")
    return {
        "run_usd": round(float(run_usd), 2),
        "run_cap": rconfig.RUN_USD,
        "round_usd": round(float(round_usd), 2) if isinstance(round_usd, (int, float)) else None,
        "round_cap": rconfig.ROUND_USD,
        "live_usd": round(agent_usd, 2),
        "tokens_in": sum(int(a["tokens_in"]) for a in agents
                         if isinstance(a.get("tokens_in"), (int, float))),
        "tokens_out": sum(int(a["tokens_out"]) for a in agents
                          if isinstance(a.get("tokens_out"), (int, float))),
    }


# --- the payload ------------------------------------------------------------

def build_payload(state_dir, *, gh_repo=None, want_checks=True, want_du=True):
    """Everything the dashboard shows, assembled once per refresh tick.

    This is the ONLY function in the module that is allowed to touch a subprocess. Request
    handlers slice the published copy of its result and never call it.
    """
    gh_repo = gh_repo or rconfig.GH_REPO
    now = time.time()
    snap = _collect_snapshot()
    agents = _collect_agents(state_dir, snap, gh_repo, want_checks=want_checks)
    rounds = _collect_rounds(state_dir)
    needs = _collect_needs(rconfig.needs_dir(), rconfig.rejected_dir())
    need_index = {n["need_id"]: n for n in needs["backlog"] + needs["rejected"]}
    acceptance = _collect_acceptance(state_dir, rounds, need_index)
    _enrich_need_status(needs, acceptance, _replay_status(state_dir))
    corpus = _collect_corpus(state_dir, rounds)
    supervisor = _supervisor(state_dir, agents)
    spend = _collect_spend(state_dir, agents, rounds)
    disk = _disk() or {}
    split = rconfig.agent_split()
    live = [a for a in agents if a.get("status") == "running"]
    current = rounds[-1] if rounds else None
    return {
        "schema": "re-webui/1",
        "now": now,
        "state_dir": str(state_dir),
        "repo": str(rconfig.repo_root()),
        "gh_repo": gh_repo,
        "supervisor": supervisor,
        "header": {
            "round": current["round"] if current else None,
            "rounds_planned": supervisor.get("rounds_planned"),
            "test_state": current["test_state"] if current else None,
            "build_state": current["build_state"] if current else None,
            "agents_live": len(live),
            "agents_max": split["max_agents"],
            "pools": {"testers": split["testers"], "builders": split["builders"]},
            "disk_free_gb": disk.get("free_gb"),
            "disk_red_gb": DISK_RED_GB,
            "disk_min_gb": rconfig.MIN_FREE_GB,
            "disk_halt_gb": rconfig.HALT_FREE_GB,
            "state_dir_bytes": _state_dir_bytes(state_dir, want_du),
            "run_usd": spend["run_usd"],
            "run_cap": spend["run_cap"],
        },
        "lanes": {"test": TEST_LANE, "build": BUILD_LANE},
        "rounds": rounds,
        "current_round": current,
        "agents": agents,
        "needs": needs,
        "acceptance": acceptance,
        "corpus": {"strata": corpus["strata"], "totals": corpus["totals"]},
        "spend": spend,
        "worktrees": snap.get("worktrees") or [],
        "prs": snap.get("prs"),
        "gh_ok": snap.get("prs") is not None,
        "leases": snap.get("leases") or {},
        "slots": snap.get("slots") or {},
        "proposals": snap.get("proposals") or {},
        "cache": pstatus.cache_ages(),
        "modules": {"needs": needs_mod is not None, "verify": verify_mod is not None,
                    "sample": sample_mod is not None},
    }


def _stable(obj):
    """The payload with its per-second counters removed, so a quiet loop does not churn seq."""
    if isinstance(obj, dict):
        return {k: _stable(v) for k, v in obj.items() if k not in VOLATILE_KEYS}
    if isinstance(obj, list):
        return [_stable(v) for v in obj]
    return obj


def _digest(payload):
    blob = json.dumps(_stable(payload), sort_keys=True, separators=(",", ":"), default=str)
    return hashlib.sha1(blob.encode()).hexdigest()


class Cache:
    """The single published snapshot. Readers take the payload; the refresher swaps it."""

    def __init__(self, state_dir):
        self.state_dir = Path(state_dir)
        self.cond = threading.Condition()
        self.seq = 0
        self.payload = {}
        self.digest = None
        self.updated_at = 0.0
        self.error = None
        self.refreshes = 0

    def publish(self, payload):
        digest = _digest(payload)
        with self.cond:
            changed = digest != self.digest
            if changed:
                self.seq += 1
                self.digest = digest
            payload["seq"] = self.seq
            self.payload = payload
            self.updated_at = time.time()
            self.refreshes += 1
            if changed:
                self.cond.notify_all()
        if changed:
            self._write_through(payload)
        return changed

    def _write_through(self, payload):
        path = self.state_dir / "webui-cache.json"
        tmp = str(path) + ".tmp"
        try:
            os.makedirs(self.state_dir, exist_ok=True)
            with open(tmp, "w") as fh:
                json.dump(payload, fh, default=str)
            os.replace(tmp, str(path))
        except OSError:
            pass

    def get(self):
        with self.cond:
            return self.payload, self.seq

    def wait(self, last_seq, timeout):
        with self.cond:
            if self.seq != last_seq:
                return self.seq
            self.cond.wait(timeout)
            return self.seq


class Refresher(threading.Thread):
    """One thread, all the expensive I/O. Started before the server accepts a connection."""

    daemon = True

    def __init__(self, cache, interval=1.0, want_checks=True, want_du=True, gh_repo=None):
        super().__init__(name="repipe-webui-refresher")
        self.cache = cache
        self.interval = float(interval)
        self.want_checks = want_checks
        self.want_du = want_du
        self.gh_repo = gh_repo
        self.stop_event = threading.Event()

    def tick(self):
        try:
            payload = build_payload(self.cache.state_dir, gh_repo=self.gh_repo,
                                   want_checks=self.want_checks, want_du=self.want_du)
            self.cache.error = None
        except Exception as exc:
            self.cache.error = "%s: %s" % (type(exc).__name__, exc)
            payload = dict(self.cache.payload or {})
            payload["refresh_error"] = self.cache.error
            payload["now"] = time.time()
        return self.cache.publish(payload)

    def run(self):
        while not self.stop_event.is_set():
            self.tick()
            self.stop_event.wait(self.interval)

    def stop(self):
        self.stop_event.set()


# --- detail readers (cheap, no subprocess) ---------------------------------

def read_need(need_id):
    """The full record: front-matter + the fixed ## sections. Rejected pile included."""
    if not NEED_ID_RE.match(need_id or "") or ".." in need_id:
        return None
    for directory in (rconfig.needs_dir(), rconfig.rejected_dir()):
        path = directory / ("%s.md" % need_id)
        if not _lexically_under(rconfig.needs_dir(), path) or not path.is_file():
            continue
        try:
            text = path.read_text(errors="replace")
        except OSError:
            continue
        fm = _front_matter(text)
        fm.setdefault("need_id", need_id)
        if directory == rconfig.rejected_dir():
            fm["status"] = "rejected"
        rec = _norm_need(fm)
        rec["file"] = str(path)
        rec["sections"] = _sections(text)
        rec["raw_front_matter"] = fm
        return rec
    return None


def _cli_probe_index():
    """probe_id -> tests/cli/*.json, rebuilt only when that directory changes."""
    d = rconfig.cli_tests_dir()
    files = sorted(d.glob("*.json")) if d.is_dir() else []
    sig = _mtime_sig(files)
    cached = getattr(_cli_probe_index, "_cache", None)
    if cached and cached[0] == sig:
        return cached[1]
    index = {}
    for f in files:
        doc = _read_json(f)
        if isinstance(doc, dict) and doc.get("probe_id"):
            index[doc["probe_id"]] = str(f)
    _cli_probe_index._cache = (sig, index)
    return index


def read_probe(state_dir, probe_id, replays=40):
    """The probe document plus its last N replay results.

    Replays come from .kuna-repipe/replays/<id>.jsonl (probe.py's own log) and from every
    round's acceptance.json, so an acceptance probe shows its per-round history even when
    no replay log exists yet.
    """
    if not PROBE_ID_RE.match(probe_id or ""):
        return None
    doc = None
    origin = None
    for cand in (state_dir / "probes" / ("%s.json" % probe_id),
                 rconfig.needs_dir() / "probes" / ("%s.json" % probe_id)):
        if cand.is_file():
            doc = _read_json(cand)
            origin = str(cand)
            break
    if doc is None:
        hit = _cli_probe_index().get(probe_id)
        if hit:
            doc = _read_json(hit)
            origin = hit
    if doc is None and verify_mod is not None:
        loader = getattr(verify_mod, "load_probe", None)
        if callable(loader):
            try:
                doc = loader(probe_id)
                origin = "verify.py"
            except Exception:
                doc = None
    history = _jsonl(state_dir / "replays" / ("%s.jsonl" % probe_id), limit=replays)
    for num, path in _round_dirs(state_dir):
        for row in _acceptance_rows(_read_json(path / "acceptance.json")):
            pid = row.get("probe_id") or row.get("acceptance_id") or row.get("id")
            if pid == probe_id:
                history.append({"round": num, "status": _cell(row),
                                "ts": row.get("ts"), "source": "acceptance.json"})
    if doc is None and not history:
        return None
    return {"probe_id": probe_id, "probe": doc, "origin": origin,
            "kind": (doc or {}).get("kind"),
            "is_acceptance": probe_id.startswith("a-"),
            "replays": history[-replays:], "replay_count": len(history)}


def read_agent_report(state_dir, agent_id, agents):
    """A tester's report.json, found through the arena its inventory record names."""
    if not AGENT_ID_RE.match(agent_id or ""):
        return None
    rec = next((a for a in agents if a["id"] == agent_id), None)
    cands = []
    if rec and rec.get("report"):
        cands.append(Path(rec["report"]))
    if rec and rec.get("challenge"):
        for num, _ in _round_dirs(state_dir):
            cands.append(state_dir / "arena" / str(num) / rec["challenge"] / "report.json")
    cands.append(state_dir / "reports" / ("%s.json" % agent_id))
    for cand in cands:
        if not cand.is_absolute():
            cand = state_dir / cand
        if _safe_under(state_dir, cand) and cand.is_file():
            return {"agent": agent_id, "path": str(cand), "report": _read_json(cand)}
    return None


# --- HTTP -------------------------------------------------------------------

_CTYPES = {".html": "text/html; charset=utf-8", ".css": "text/css; charset=utf-8",
           ".js": "text/javascript; charset=utf-8", ".json": "application/json",
           ".svg": "image/svg+xml", ".woff2": "font/woff2", ".png": "image/png",
           ".ico": "image/x-icon", ".webp": "image/webp"}


class Handler(BaseHTTPRequestHandler):
    """Read-only. Every route answers from Handler.cache; none of them starts a process."""

    protocol_version = "HTTP/1.1"
    server_version = "kuna-repipe-webui/1"
    cache = None
    state_dir = None
    log_path = None

    def log_message(self, fmt, *args):
        line = "%s - - [%s] %s\n" % (self.address_string(),
                                     self.log_date_time_string(), fmt % args)
        path = type(self).log_path
        if not path:
            return
        try:
            with open(path, "a") as fh:
                fh.write(line)
        except OSError:
            pass

    # --- reply helpers ---

    def _send(self, code, body, ctype="application/json", extra=None):
        if isinstance(body, str):
            body = body.encode("utf-8")
        self.send_response(code)
        self.send_header("Content-Type", ctype)
        self.send_header("Content-Length", str(len(body)))
        self.send_header("Cache-Control", "no-store")
        self.send_header("X-Content-Type-Options", "nosniff")
        for k, v in (extra or {}).items():
            self.send_header(k, v)
        self.end_headers()
        if self.command != "HEAD":
            try:
                self.wfile.write(body)
            except (BrokenPipeError, ConnectionResetError):
                self.close_connection = True

    def _json(self, obj, code=200):
        self._send(code, json.dumps(obj, default=str, sort_keys=False), "application/json")

    def _err(self, code, msg):
        self._json({"error": msg, "status": code}, code)

    def _file(self, path, ctype=None):
        try:
            with open(path, "rb") as fh:
                body = fh.read()
        except OSError:
            return self._err(404, "not found")
        ct = ctype or _CTYPES.get(Path(path).suffix, "application/octet-stream")
        self._send(200, body, ct)

    # --- routing ---

    def do_HEAD(self):
        self.do_GET()

    def do_GET(self):
        parsed = urllib.parse.urlsplit(self.path)
        segs = [urllib.parse.unquote(s) for s in parsed.path.split("/") if s != ""]
        query = urllib.parse.parse_qs(parsed.query)
        try:
            self.route(parsed.path, segs, query)
        except (BrokenPipeError, ConnectionResetError):
            self.close_connection = True
        except Exception as exc:
            self._err(500, "%s: %s" % (type(exc).__name__, exc))

    def route(self, raw_path, segs, query):
        payload, seq = type(self).cache.get()
        state_dir = type(self).state_dir

        if not segs:
            return self._file(WEBUI_DIR / "index.html", _CTYPES[".html"])

        if segs[0] == "healthz":
            c = type(self).cache
            return self._json({
                "ok": True, "seq": seq, "pid": os.getpid(),
                "age_s": round(time.time() - c.updated_at, 2) if c.updated_at else None,
                "refreshes": c.refreshes, "refresh_error": c.error,
                "state_dir": str(state_dir), "state_dir_exists": state_dir.is_dir(),
                "modules": (payload.get("modules") or {}),
            })

        if segs[0] == "assets":
            return self.serve_asset(segs[1:])

        # site.css is served verbatim, and its @font-face src is "../fonts/<name>" relative
        # to /assets/css/ — which is /fonts/<name> here. Serving it keeps the stylesheet
        # byte-identical to the live site instead of rewriting its URLs.
        if segs[0] in ("fonts", "img") and len(segs) == 2:
            name = safe_id(segs[1])
            if name is None:
                return self._err(400, "bad asset name")
            return self.serve_site_file(segs[0], name)

        if segs[0] == "favicon.ico":
            return self.serve_site_file("img", "favicon.png")

        if segs[0] != "api":
            return self._err(404, "no route %r" % raw_path)

        return self.api(segs[1:], query, payload, seq, state_dir)

    def serve_site_file(self, kind, name):
        # Charset-gate first: a tainted component must never reach a path at all, whatever
        # _safe_under would have said about the result.
        name = safe_id(name)
        if name is None or kind not in ("fonts", "img", "css", "js"):
            return self._err(400, "bad asset name")
        cand = SITE_ASSETS / kind / name
        if not _safe_under(SITE_ASSETS, cand) or not cand.is_file():
            return self._err(404, "not found")
        return self._file(cand)

    def serve_asset(self, rest):
        if not rest or any(s in ("..", ".", "") or "/" in s or "\\" in s for s in rest):
            return self._err(400, "bad asset path")
        # site.css is served verbatim from the live site tree so the dashboard cannot drift
        # from kuna.noelo.org's --paper/--ink/--red token system.
        if rest == ["site.css"]:
            return self._file(SITE_CSS, _CTYPES[".css"])
        if rest[0] == "fonts" and len(rest) == 2:
            name = safe_id(rest[1])
            if name is None:
                return self._err(400, "bad asset name")
            cand = SITE_FONTS / name
            if not _safe_under(SITE_FONTS, cand) or not cand.is_file():
                return self._err(404, "not found")
            return self._file(cand)
        parts = [safe_id(x) for x in rest]
        if any(x is None for x in parts):
            return self._err(400, "bad asset path")
        cand = WEBUI_DIR / "assets" / Path(*parts)
        if not _safe_under(WEBUI_DIR / "assets", cand) or not cand.is_file():
            return self._err(404, "not found")
        return self._file(cand)

    def api(self, rest, query, payload, seq, state_dir):
        if not rest:
            return self._err(404, "no route")
        head = rest[0]

        if head == "state":
            return self._json(payload or {"seq": seq, "empty": True})

        if head == "events":
            return self.sse(query)

        if head == "rounds":
            return self._json({"seq": seq, "rounds": payload.get("rounds", []),
                               "lanes": payload.get("lanes", {})})

        if head == "round" and len(rest) == 2:
            if not rest[1].lstrip("-").isdigit():
                return self._err(400, "round must be an integer")
            want = int(rest[1])
            for rnd in payload.get("rounds", []):
                if rnd["round"] == want:
                    full = dict(rnd)
                    full["transitions"] = _jsonl(Path(rnd["path"]) / "transitions.jsonl")
                    full["acceptance"] = _read_json(Path(rnd["path"]) / "acceptance.json")
                    return self._json(full)
            return self._err(404, "no round %d" % want)

        if head == "needs":
            return self._json(self.filter_needs(payload, query, seq))

        if head == "need" and len(rest) == 2:
            if not NEED_ID_RE.match(rest[1]) or ".." in rest[1]:
                return self._err(400, "invalid need id")
            rec = read_need(rest[1])
            if rec is None:
                return self._err(404, "no need %r" % rest[1])
            return self._json(rec)

        if head == "probe" and len(rest) == 2:
            if not PROBE_ID_RE.match(rest[1]):
                return self._err(400, "probe id must match ^[pa]-[0-9a-f]{12}$")
            rec = read_probe(state_dir, rest[1])
            if rec is None:
                return self._err(404, "no probe %r" % rest[1])
            return self._json(rec)

        if head == "agents":
            return self._json({"seq": seq, "agents": payload.get("agents", []),
                               "slots": payload.get("slots", {}),
                               "leases": payload.get("leases", {}),
                               "now": time.time()})

        if head == "agent" and len(rest) == 3 and rest[2] in ("log", "report"):
            agent_id = safe_id(rest[1])
            if agent_id is None:
                return self._err(400, "bad agent id")
            if not AGENT_ID_RE.match(agent_id):
                return self._err(400, "invalid agent id")
            if rest[2] == "report":
                rec = read_agent_report(state_dir, agent_id, payload.get("agents", []))
                if rec is None:
                    return self._err(404, "no report for %r" % agent_id)
                return self._json(rec)
            return self.agent_log(agent_id, query, payload, state_dir)

        if head == "corpus":
            return self._json(self.corpus(query, state_dir, payload))

        if head == "acceptance":
            return self._json(dict(payload.get("acceptance", {}), seq=seq))

        return self._err(404, "no route /api/%s" % "/".join(rest))

    def filter_needs(self, payload, query, seq):
        needs = payload.get("needs") or {}
        rows = list(needs.get("backlog", []))
        statuses = {s for v in query.get("status", []) for s in v.split(",") if s}
        if statuses:
            if "rejected" in statuses:
                rows += needs.get("rejected", [])
            rows = [n for n in rows if n["status"] in statuses]
        tracks = {t for v in query.get("track", []) for t in v.split(",") if t}
        if tracks:
            rows = [n for n in rows if n.get("track") in tracks]
        q = (query.get("q") or [""])[0].strip().lower()
        if q:
            rows = [n for n in rows
                    if q in (n.get("need_id") or "").lower()
                    or q in (n.get("title") or "").lower()
                    or q in (n.get("probe_id") or "").lower()
                    or q in (n.get("acceptance_id") or "").lower()]
        return {"seq": seq, "count": len(rows), "needs": rows,
                "totals": needs.get("totals", {}), "by_status": needs.get("by_status", {}),
                "rejected_by_reason": {k: len(v) for k, v in
                                       (needs.get("rejected_by_reason") or {}).items()},
                "source": needs.get("source"),
                "filters": {"status": sorted(statuses), "track": sorted(tracks), "q": q}}

    def corpus(self, query, state_dir, payload):
        full = _collect_corpus(state_dir, payload.get("rounds", []))
        stratum = (query.get("stratum") or [""])[0]
        rows = full["challenges"]
        if stratum:
            rows = [r for r in rows if r["stratum"] == stratum]
        if (query.get("attempted") or [""])[0] == "1":
            rows = [r for r in rows if r["attempts"]]
        limit = min(500, max(1, int((query.get("limit") or ["250"])[0] or 250)))
        return {"strata": full["strata"], "totals": full["totals"],
                "count": len(rows), "challenges": rows[:limit],
                "note": "coverage only; ground_truth.flag is never read into this payload"}

    def agent_log(self, agent_id, query, payload, state_dir):
        rec = next((a for a in payload.get("agents", []) if a["id"] == agent_id), None)
        path = _agent_log_path(state_dir, agent_id, (rec or {}).get("log"))
        if path is None:
            return self._err(404, "no log for agent %r" % agent_id)
        if not _safe_under(state_dir, path):
            return self._err(400, "log path escapes the state dir")
        try:
            tail = int((query.get("tail") or ["400"])[0])
        except ValueError:
            tail = 400
        tail = max(1, min(5000, tail))
        lines = _tail_lines(path, tail)
        return self._json({"agent": agent_id, "path": str(path), "tail": tail,
                           "lines": lines, "bytes": os.path.getsize(path)})

    def sse(self, query):
        """One long-lived response per viewer; the refresher stays the only I/O owner."""
        cache = type(self).cache
        self.send_response(200)
        self.send_header("Content-Type", "text/event-stream")
        self.send_header("Cache-Control", "no-store")
        self.send_header("Connection", "close")
        self.send_header("X-Accel-Buffering", "no")
        self.end_headers()
        self.close_connection = True
        try:
            limit = int((query.get("limit") or ["0"])[0])
        except ValueError:
            limit = 0
        sent = 0
        last = None
        deadline = time.time() + 15.0
        try:
            while True:
                _, seq = cache.get()
                if seq != last:
                    last = seq
                    self.wfile.write(("data: %s\n\n" % json.dumps({"seq": seq})).encode())
                    self.wfile.flush()
                    sent += 1
                    deadline = time.time() + 15.0
                    if limit and sent >= limit:
                        return
                remaining = deadline - time.time()
                if remaining <= 0:
                    self.wfile.write(b": keepalive\n\n")
                    self.wfile.flush()
                    deadline = time.time() + 15.0
                    continue
                cache.wait(last, min(remaining, 15.0))
        except (BrokenPipeError, ConnectionResetError, ValueError):
            return


# --- server -----------------------------------------------------------------

class Server(ThreadingHTTPServer):
    daemon_threads = True
    allow_reuse_address = True


def serve(bind="127.0.0.1", port=8787, *, state_dir=None, interval=1.0,
          want_checks=True, want_du=True, gh_repo=None, ready=None):
    """Start the refresher, then the server. Returns the exit code."""
    sdir = bind_state_dir(state_dir)
    logs = sdir / "logs"
    log_path = None
    try:
        os.makedirs(logs, exist_ok=True)
        log_path = logs / "webui.log"
    except OSError:
        log_path = None

    cache = Cache(sdir)
    refresher = Refresher(cache, interval=interval, want_checks=want_checks,
                          want_du=want_du, gh_repo=gh_repo)
    refresher.tick()          # first paint before the first connection is accepted
    refresher.start()

    handler = type("BoundHandler", (Handler,),
                   {"cache": cache, "state_dir": sdir, "log_path": log_path})
    httpd = Server((bind, port), handler)
    real_port = httpd.server_address[1]
    sys.stderr.write("repipe webui  http://%s:%d/   state=%s  log=%s\n"
                     % (bind, real_port, sdir, log_path or "(stderr disabled)"))
    sys.stderr.write("  read-only: to stop the pipeline, `touch %s`\n" % (sdir / "STOP"))
    sys.stderr.flush()
    if ready is not None:
        ready(httpd, cache, refresher)
    try:
        httpd.serve_forever(poll_interval=0.2)
    except KeyboardInterrupt:
        pass
    finally:
        refresher.stop()
        httpd.server_close()
    return 0


def main(argv=None):
    p = argparse.ArgumentParser(
        prog="python -m scripts.repipe.webui",
        description="Live dashboard for the RE-friction loop (read-only, loopback).")
    p.add_argument("--bind", default="127.0.0.1")
    p.add_argument("--port", type=int, default=8787, help="0 picks an ephemeral port")
    p.add_argument("--state-dir", default=None, help="default: $REPO/.kuna-repipe")
    p.add_argument("--interval", type=float, default=1.0, help="refresher tick, seconds")
    p.add_argument("--gh-repo", default=None)
    p.add_argument("--no-checks", action="store_true",
                   help="never call gh for check-runs (PR CI columns read 'unknown')")
    p.add_argument("--no-du", action="store_true",
                   help="skip the state-dir du (it can be slow with live arenas)")
    p.add_argument("--json", action="store_true",
                   help="print one payload snapshot and exit; starts no server")
    args = p.parse_args(argv)

    sdir = bind_state_dir(args.state_dir)
    if args.json:
        payload = build_payload(sdir, gh_repo=args.gh_repo,
                                want_checks=not args.no_checks, want_du=not args.no_du)
        payload["seq"] = 1
        print(json.dumps(payload, indent=2, default=str))
        return 0
    return serve(args.bind, args.port, state_dir=args.state_dir, interval=args.interval,
                 want_checks=not args.no_checks, want_du=not args.no_du,
                 gh_repo=args.gh_repo)


if __name__ == "__main__":
    sys.exit(main())
