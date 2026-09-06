"""The RE-friction loop's state machines: deterministic Python the LLM captain drives.

Three cooperating machines — Supervisor, TestTrack, BuildTrack — whose transitions are
guarded here and appended to `<state>/rounds/<n>/transitions.jsonl`. **An illegal transition
raises and exits 2.** That split is the whole point: the captain is a Claude Code session
doing the judgment work (clustering residue, scope calls, proposal approval, deciding a
round is finished), but it cannot talk the machine into skipping a gate, because the machine
is not made of prose.

The captain runs ONE BOUNDED TICK AT A TIME. A tick reads state, performs at most one
transition, writes state, and exits; `tools/repipe/run.sh` re-invokes it. A multi-day LLM
process is a reliability risk, whereas a stateless-per-tick captain costs at most one tick
when it dies and resumes from `inventory.json` — which is also why `--recover` can simply
re-enter the RECORDED state and never a later one.

Agents are spawned DETACHED and a tick never blocks on one. `tester.sh` and `builder.sh`
heartbeat into the shared inventory the same way the angr fleet's workers do.
"""
from __future__ import annotations

import argparse
import contextlib
import fcntl
import json
import os
import shutil
import subprocess
import sys
import time

from . import config, select as select_mod
from ..pipeline import state as pstate

# --- the machines -----------------------------------------------------------

SUPERVISOR = {
    "BOOT": ["RUNNING", "HALTED"],
    "RUNNING": ["DRAINING", "HALTED"],
    "DRAINING": ["STOPPED", "HALTED"],
    "HALTED": [],
    "STOPPED": [],
}

TEST_TRACK = {
    "T_IDLE": ["T_PLAN"],
    "T_PLAN": ["T_WORKSPACE", "T_IDLE"],
    "T_WORKSPACE": ["T_FANOUT"],
    "T_FANOUT": ["T_DRAIN"],
    "T_DRAIN": ["T_GATE"],
    "T_GATE": ["T_DEDUP"],
    "T_DEDUP": ["T_REFUTE"],
    "T_REFUTE": ["T_TRIAGE"],
    "T_TRIAGE": ["T_READY"],
    "T_READY": ["T_IDLE"],
}

BUILD_TRACK = {
    "B_IDLE": ["B_PLAN", "B_DONE"],
    "B_PLAN": ["B_FANOUT", "B_IDLE"],
    "B_FANOUT": ["B_DRAIN"],
    "B_DRAIN": ["B_MERGE", "B_PROPOSAL_REVIEW"],
    "B_PROPOSAL_REVIEW": ["B_FANOUT", "B_MERGE", "B_IDLE"],
    "B_MERGE": ["B_VERIFY"],
    "B_VERIFY": ["B_DONE", "B_ROLLBACK"],
    "B_ROLLBACK": ["B_VERIFY", "HALTED"],
    "B_DONE": ["B_IDLE"],
}

MACHINES = {"supervisor": SUPERVISOR, "test": TEST_TRACK, "build": BUILD_TRACK}


class IllegalTransition(Exception):
    pass


def _round_dir(n):
    d = config.rounds_dir() / str(n)
    os.makedirs(d, exist_ok=True)
    return d


def _round_path(n):
    return _round_dir(n) / "round.json"


def load_round(n):
    p = _round_path(n)
    if not p.exists():
        return {"round": n, "supervisor": "BOOT", "test": "T_IDLE", "build": "B_IDLE",
                "started_at": time.time(), "slate": [], "spend_usd": 0.0, "notes": []}
    with open(p) as fh:
        return json.load(fh)


@contextlib.contextmanager
def _round_lock(n):
    """flock around a round's read-modify-write, in scripts/pipeline/state.py's idiom.

    Two captain ticks can overlap (a slow one plus run.sh's next poll), and an unlocked
    read-modify-write silently drops one of their transitions.
    """
    lock = _round_dir(n) / ".lock"
    fh = open(lock, "w")
    try:
        fcntl.flock(fh, fcntl.LOCK_EX)
        yield
    finally:
        fcntl.flock(fh, fcntl.LOCK_UN)
        fh.close()


def save_round(doc):
    p = _round_path(doc["round"])
    # A per-process temp name: a shared "<path>.tmp" lets two writers interleave into the same
    # file and os.replace a half-written document into place.
    tmp = "%s.tmp.%d" % (p, os.getpid())
    with open(tmp, "w") as fh:
        json.dump(doc, fh, indent=2)
    os.replace(tmp, p)


def transition(doc, machine, to, note=None):
    """Advance one machine, or refuse. This is the guard the LLM cannot argue with.

    The whole read-check-write runs under the round lock and re-reads the document inside it,
    so a caller holding a stale copy cannot resurrect an earlier state or skip a gate that
    another tick has already passed.
    """
    table = MACHINES[machine]
    with _round_lock(doc["round"]):
        fresh = load_round(doc["round"])
        cur = fresh[machine]
        if to not in table.get(cur, []):
            raise IllegalTransition("%s: %s -> %s is not a legal transition (legal: %s)"
                                    % (machine, cur, to, ", ".join(table.get(cur, [])) or "none"))
        fresh[machine] = to
        rec = {"ts": time.time(), "machine": machine, "from": cur, "to": to,
               "note": note, "pid": os.getpid()}
        with open(_round_dir(fresh["round"]) / "transitions.jsonl", "a") as fh:
            fh.write(json.dumps(rec) + "\n")
        save_round(fresh)
    doc.update(fresh)
    return doc


# --- environment guards -----------------------------------------------------

def free_gb(path=None):
    st = os.statvfs(str(path or config.repo_root()))
    return int(st.f_bavail * st.f_frsize / (1024 ** 3))


def stop_requested():
    return (config.state_dir() / "STOP").exists()


def abort_requested():
    return (config.state_dir() / "ABORT").exists()


def paused():
    return (config.state_dir() / "PAUSE").exists()


def preflight():
    """Hard preconditions. A missing one is a HALT, not a warning — the loop would otherwise
    fail slowly, hours in, having spent real money."""
    bad, warn = [], []
    for tool in ("git", "gh", "codex", "claude", "python3"):
        if not shutil.which(tool):
            bad.append("missing tool: %s" % tool)
    if not config.kuna_bin().exists():
        bad.append("no kuna binary at %s (run `make binaries`)" % config.kuna_bin())
    if not config.dataset_root().exists():
        bad.append("no dataset at %s" % config.dataset_root())
    if not list(config.specs_dir().rglob("*.sla"))[:1]:
        bad.append("no compiled .sla under %s (run `make specs`)" % config.specs_dir())
    if config.sandbox_mode() != "bwrap":
        if os.environ.get("REPIPE_SANDBOX") == "none":
            # An explicit, recorded choice is allowed -- but it is never the default, and
            # tester.sh refuses to reach this state on its own.
            warn.append("SANDBOX EXPLICITLY OFF (REPIPE_SANDBOX=none): the dataset and $HOME "
                        "are readable to every tester. Runs are flagged low-trust and the "
                        "post-hoc tripwire is the only contamination check.")
        else:
            bad.append("SANDBOX IS OFF: the dataset leaks the answer four ways and bwrap is "
                       "the only thing that actually hides it. Install bwrap, or set "
                       "REPIPE_SANDBOX=none to accept prompt-only containment explicitly.")
    gb = free_gb()
    if gb < config.MIN_FREE_GB:
        bad.append("only %dG free, need %dG (a cargo worktree costs 20-30G)"
                   % (gb, config.MIN_FREE_GB))
    for w in warn:
        print("WARNING: %s" % w, file=sys.stderr)
    return bad


# --- agent supervision ------------------------------------------------------

def live_agents(pool):
    data = pstate.snapshot()
    return list((data.get("slots", {}).get(pool, {}).get("held") or {}).keys())


def set_caps(split):
    pstate.slot_cap("captain", split["captain"])
    pstate.slot_cap("tester", split["testers"])
    pstate.slot_cap("builder", split["builders"])


def spawn_tester(round_n, hexid):
    env = dict(os.environ, ROUND=str(round_n), HEXID=hexid,
               KUNA_PIPELINE_STATE_DIR=str(config.state_dir()),
               PYTHONPATH=str(config.repo_root()))
    log = config.logs_dir() / ("spawn-t-%s.log" % hexid[:8])
    os.makedirs(str(config.logs_dir()), exist_ok=True)
    with open(log, "ab") as fh:
        subprocess.Popen(["bash", str(config.repo_root() / "tools" / "repipe" / "tester.sh")],
                         env=env, stdout=fh, stderr=fh, stdin=subprocess.DEVNULL,
                         start_new_session=True)
    return str(log)


def spawn_builder(round_n, need, resources):
    """Reuse tools/pipeline/worker.sh through its seams rather than forking it.

    The seams (WORKER_PROMPT / WORKER_BRANCH_PREFIX / WORKER_EXTRA_PROMPT / …) all default to
    the angr fleet's behaviour, so the two pipelines share one driver and one set of
    worktree-hygiene fixes.
    """
    wid = "b-r%s-%s" % (round_n, need.need_id[:16])

    # A builder must hold a `builder` slot or REPIPE_MAX_AGENTS means nothing for the half of
    # the fleet that costs the most, and run.sh's drain has nothing to wait on.
    if not pstate.slot_acquire("builder", wid, pid=os.getpid(), kind="builder"):
        return None

    # ONE contracts file PER BUILDER. A single shared path was worse than useless: each spawn
    # overwrote it, so every builder ended up reading the list that excluded whoever was
    # spawned last -- i.e. usually its own entry rather than its siblings'.
    contracts = config.state_dir() / ("contracts-%s.md" % wid)
    with open(contracts, "w") as fh:
        fh.write(select_mod.contracts_markdown(exclude_need=need.need_id))
    env = dict(
        os.environ,
        WORKER_ID=wid, OPP_ID=need.need_id, TEST_NAME=need.need_id,
        SELECTOR=getattr(need, "selector", "") or "-", BINARY=getattr(need, "binary", "") or "-",
        SLUG=need.need_id, ARCH="",
        WORKER_PROMPT=str(config.repo_root() / "tools" / "repipe" / "builder_prompt.md"),
        WORKER_BRANCH_PREFIX="feat/re-",
        WORKER_EXTRA_PROMPT=str(contracts),
        WORKER_BUDGET_USD=str(config.BUILDER_USD),
        WORKER_TIMEOUT=str(config.BUILDER_TIMEOUT),
        WORKER_MODEL=config.BUILDER_MODEL,
        PIPELINE_STATE_DIRNAME=config.STATE_DIRNAME,
        KUNA_PIPELINE_STATE_DIR=str(config.state_dir()),
        PYTHONPATH=str(config.repo_root()),
    )
    # Provisional leases with the tick's pid, re-stamped with the worker's below. Taken before
    # the spawn so a second pick in the same tick cannot claim the same resource.
    for r in resources:
        pstate.lease_acquire(r, wid, ttl=config.BUILDER_TIMEOUT + 1800, pid=os.getpid())
    log = config.logs_dir() / ("spawn-%s.log" % wid)
    os.makedirs(str(config.logs_dir()), exist_ok=True)
    with open(log, "ab") as fh:
        proc = subprocess.Popen(["bash", str(config.repo_root() / "tools" / "pipeline" / "worker.sh")],
                                env=env, stdout=fh, stderr=fh, stdin=subprocess.DEVNULL,
                                start_new_session=True)
    # Re-stamp the slot and the leases with the WORKER's pid, not the captain tick's. The tick
    # exits in seconds; stamping its pid would make reap() free every lease on the next pass
    # and void the "at most one option-adding builder" guarantee entirely.
    pstate.slot_release("builder", wid)
    pstate.slot_acquire("builder", wid, pid=proc.pid, kind="builder")
    for r in resources:
        pstate.lease_release(r, wid)
        pstate.lease_acquire(r, wid, ttl=config.BUILDER_TIMEOUT + 1800, pid=proc.pid)
    return wid


def release_all_leases(worker):
    data = pstate.snapshot()
    for res, lease in (data.get("leases") or {}).items():
        if lease.get("holder") == worker:
            pstate.lease_release(res, worker)


# --- the tick ---------------------------------------------------------------

def tick(round_n=None, dry_run=False):
    """Advance the loop by at most one transition per machine. Returns a status dict."""
    pstate.reap()
    n = round_n if round_n is not None else current_round()
    doc = load_round(n)
    split = config.agent_split()
    set_caps(split)
    acted = []

    if abort_requested():
        doc["notes"].append("ABORT file present")
        if doc["supervisor"] in ("BOOT", "RUNNING"):
            transition(doc, "supervisor", "HALTED", "ABORT")
        return {"round": n, "doc": doc, "acted": ["abort"]}

    if doc["supervisor"] == "BOOT":
        problems = preflight()
        if problems:
            transition(doc, "supervisor", "HALTED", "; ".join(problems))
            _write_halt(problems)
            return {"round": n, "doc": doc, "acted": ["halt"], "problems": problems}
        transition(doc, "supervisor", "RUNNING", "preflight OK")
        acted.append("boot")

    if doc["supervisor"] == "RUNNING":
        gb = free_gb()
        if gb < config.HALT_FREE_GB:
            transition(doc, "supervisor", "HALTED", "disk %dG < %dG" % (gb, config.HALT_FREE_GB))
            _write_halt(["disk %dG below the halt floor %dG" % (gb, config.HALT_FREE_GB)])
            return {"round": n, "doc": doc, "acted": ["halt-disk"]}
        if stop_requested():
            transition(doc, "supervisor", "DRAINING", "STOP file")
            acted.append("drain")
        elif doc.get("spend_usd", 0.0) >= config.ROUND_USD:
            transition(doc, "supervisor", "DRAINING", "round budget $%.2f reached"
                       % doc["spend_usd"])
            acted.append("drain-budget")

    return {"round": n, "doc": doc, "acted": acted, "split": split,
            "free_gb": free_gb(), "paused": paused(),
            "live": {"tester": live_agents("tester"), "builder": live_agents("builder")}}


def _write_halt(problems):
    p = config.state_dir() / "HALT_REASON"
    os.makedirs(str(config.state_dir()), exist_ok=True)
    with open(p, "w") as fh:
        fh.write("\n".join(problems) + "\n")


def current_round():
    d = config.rounds_dir()
    if not d.exists():
        return 1
    ns = [int(x.name) for x in d.iterdir() if x.is_dir() and x.name.isdigit()]
    return max(ns) if ns else 1


def recover():
    """Resume at the RECORDED state, never a later one.

    Reap first so a crashed agent's slot, claim and leases are freed; then every state is
    re-enterable because each one's entry action is idempotent (T_GATE/T_DEDUP are pure
    functions of a pinned SHA; B_MERGE re-checks whether the squash actually landed before
    doing anything).
    """
    reaped = pstate.reap(stale_seconds=0)
    n = current_round()
    doc = load_round(n)
    doc["notes"].append("recovered at %s/%s/%s" % (doc["supervisor"], doc["test"], doc["build"]))
    save_round(doc)
    return {"round": n, "reaped": reaped, "resumed_at": {k: doc[k] for k in MACHINES}}


def main(argv=None):
    ap = argparse.ArgumentParser(prog="python -m scripts.repipe.captain")
    ap.add_argument("--tick", action="store_true")
    ap.add_argument("--recover", action="store_true")
    ap.add_argument("--preflight", action="store_true")
    ap.add_argument("--status", action="store_true")
    ap.add_argument("--round", type=int, default=None)
    ap.add_argument("--transition", nargs=2, metavar=("MACHINE", "TO"))
    ap.add_argument("--note", default=None)
    ap.add_argument("--notes", type=int, default=6,
                    help="how many of the round's most recent notes --status carries (0 = none)")
    ap.add_argument("--json", action="store_true")
    args = ap.parse_args(argv)

    if args.preflight:
        problems = preflight()
        out = {"ok": not problems, "problems": problems, "free_gb": free_gb(),
               "split": config.agent_split(), "sandbox": config.sandbox_mode()}
        print(json.dumps(out, indent=2) if args.json else
              ("preflight OK" if not problems else "\n".join("FAIL: " + p for p in problems)))
        return 0 if not problems else 1

    if args.recover:
        out = recover()
        print(json.dumps(out, indent=2, default=str))
        return 0

    if args.transition:
        machine, to = args.transition
        if machine not in MACHINES:
            print("unknown machine %r (expected one of %s)" % (machine, ", ".join(MACHINES)),
                  file=sys.stderr)
            return 2
        doc = load_round(args.round if args.round is not None else current_round())
        try:
            transition(doc, machine, to, args.note)
        except IllegalTransition as e:
            print("ILLEGAL: %s" % e, file=sys.stderr)
            return 2
        print(json.dumps({k: doc[k] for k in MACHINES}, indent=2))
        return 0

    if args.status or not args.tick:
        doc = load_round(args.round if args.round is not None else current_round())
        # The prompt says "record every non-obvious decision in the round doc's `notes`; the
        # next tick is a different session with none of your context" -- and then nothing put
        # those notes in front of the next tick. Neither this payload nor
        # `scripts.repipe.status --json` carried them, so a note was only ever read by a tick
        # that happened to open rounds/N/round.json by hand. Some did; most did not, and an
        # operator note sat unread through ~110 ticks of round 4. Notes ARE the continuity
        # mechanism of this design, so `--status`, the first thing the prompt tells a tick to
        # read, has to carry them.
        #
        # Most recent last, because that is reading order. The NEWEST note is never
        # truncated: it is the actual handoff, and the round-4 captain measured the cost of
        # cutting it -- "NEXT-TICK ORDERS (--status truncates at ~700 chars -- read the note
        # above WHOLE from rounds/4/round.json before deciding anything)". A 700-char cap on
        # a paragraph-long handoff does not save the reader anything; it just sends them to
        # the file, which is the problem this was meant to remove. Older notes stay capped,
        # generously, because they are context rather than instructions.
        notes = []
        # `[-0:]` is the WHOLE list, not an empty one, so --notes 0 has to be handled before
        # the slice or it does the opposite of what it says.
        recent = (doc.get("notes") or [])[-args.notes:] if args.notes > 0 else []
        for entry in recent:
            if isinstance(entry, dict):
                text, by, ts = entry.get("note") or "", entry.get("by"), entry.get("ts")
            else:
                text, by, ts = str(entry), None, None
            text = " ".join(str(text).split())
            notes.append({"by": by, "ts": ts, "note": text})
        if notes:
            newest = notes[-1]["note"]
            for n in notes[:-1]:
                if len(n["note"]) > 2000:
                    n["note"] = n["note"][:2000] + " ...[truncated, full text in rounds/N/round.json]"
            notes[-1]["note"] = newest      # the handoff, whole
        out = {"round": doc["round"], "states": {k: doc[k] for k in MACHINES},
               "free_gb": free_gb(), "split": config.agent_split(),
               "stop": stop_requested(), "pause": paused(), "abort": abort_requested(),
               "live": {"tester": live_agents("tester"), "builder": live_agents("builder")},
               "notes_total": len(doc.get("notes") or []), "notes": notes}
        print(json.dumps(out, indent=2))
        return 0

    out = tick(args.round)
    print(json.dumps({k: v for k, v in out.items() if k != "doc"}, indent=2, default=str))
    return 0


if __name__ == "__main__":
    sys.exit(main())
