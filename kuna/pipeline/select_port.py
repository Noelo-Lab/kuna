"""Pick the next Rust-port checklist item for a port-mode worker (PIPELINE_MODE=port).

The port-mode analogue of ``kuna.pipeline.select``: instead of the angr-gap backlog it
reads the Rust-port backlog (``docs/rust-port/checklist.json``), skips anything already
claimed or done (kuna.pipeline.state, keyed by item id), and returns the next eligible
item. Emits shell-eval-able assignments so the driver loop can launch a worker without jq.

Eligibility (an item is selectable when ALL hold):
  - ``kind`` is ``port`` or ``verify`` — ``infra`` items are NEVER auto-selected
    (wave gates / scaffolding are orchestrator-only);
  - ``status == "todo"`` and ``attempts < 3`` (the verifier protocol's budget);
  - not already claimed by a live worker and not in the state dir's ``done`` set;
  - every ``depends_on`` entry is satisfied: the dep's checklist ``status`` is one of
    ``landed`` / ``verified`` / ``done``, OR the dep id is recorded done in the state
    dir (a worker finished it and its PR is open but not yet merged+flipped centrally);
  - for ``kind == verify``: the port branch ``rport/<port-item-id>`` must also resolve
    (locally or on origin) — the verifier checks the ported code out from it.

Preference order among eligible items: verify before port (verification keeps pace
with porting), then lowest wave, then smallest ``loc_estimate``, then id (stable).

    python -m kuna.pipeline.select_port --shell   # KEY='val' lines; exit 1 if nothing left
    python -m kuna.pipeline.select_port --json
"""
from __future__ import annotations

import argparse
import json
import re
import subprocess
import sys

from . import config, state

# Checklist statuses that satisfy a dependency edge. ``landed``/``verified`` are the
# orchestrator's centrally-flipped statuses (PR merged); ``done`` is the schema's
# original terminal status, kept for compatibility.
SATISFIED_STATUSES = {"landed", "verified", "done"}

MAX_ATTEMPTS = 3


def checklist_path():
    return config.repo_root() / "docs" / "rust-port" / "checklist.json"


def load_checklist(path=None):
    p = path or checklist_path()
    with open(p) as fh:
        return json.load(fh).get("items", [])


def _slug(item_id):
    """Sanitize an item id into a branch-safe slug (ids are already this shape)."""
    return re.sub(r"[^a-z0-9]+", "-", str(item_id).lower()).strip("-")


def port_branch(port_item_id):
    return "rport/%s" % _slug(port_item_id)


def _wave_num(wave):
    m = re.match(r"^W(\d+)$", wave or "")
    return int(m.group(1)) if m else 99


def branch_exists(branch, repo=None):
    """True if <branch> resolves locally or as origin/<branch>."""
    root = str(repo or config.repo_root())
    for ref in ("refs/heads/%s" % branch, "refs/remotes/origin/%s" % branch):
        rc = subprocess.run(
            ["git", "-C", root, "rev-parse", "--verify", "--quiet", ref],
            capture_output=True,
        ).returncode
        if rc == 0:
            return True
    return False


def _port_dep(item, by_id):
    """The verify item's paired port item id (its first port-kind dependency)."""
    for dep in item.get("depends_on", []):
        d = by_id.get(dep)
        if d is not None and d.get("kind") == "port":
            return dep
    return None


def pick(items=None, *, repo=None):
    items = load_checklist() if items is None else items
    by_id = {it["id"]: it for it in items}

    snap = state.snapshot()
    taken = set(snap["claims"].keys()) | set(snap["done"].keys())
    state_done = set(snap["done"].keys())

    def dep_satisfied(dep):
        d = by_id.get(dep)
        if d is not None and d.get("status") in SATISFIED_STATUSES:
            return True
        return dep in state_done

    eligible = []
    for it in items:
        if it.get("kind") not in ("port", "verify"):
            continue  # infra is orchestrator-only
        if it.get("status") != "todo":
            continue
        if it.get("attempts", 0) >= MAX_ATTEMPTS:
            continue
        if it["id"] in taken:
            continue
        if not all(dep_satisfied(dep) for dep in it.get("depends_on", [])):
            continue
        target_branch = ""
        if it["kind"] == "verify":
            port_id = _port_dep(it, by_id)
            if port_id is None:
                continue
            target_branch = port_branch(port_id)
            if not branch_exists(target_branch, repo=repo):
                continue
        it = dict(it)
        it["slug"] = _slug(it["id"])
        it["target_branch"] = target_branch
        eligible.append(it)

    if not eligible:
        return None
    eligible.sort(key=lambda it: (
        0 if it["kind"] == "verify" else 1,   # verification keeps pace
        _wave_num(it.get("wave")),
        it.get("loc_estimate", 0),
        it["id"],
    ))
    return eligible[0]


def main(argv=None):
    p = argparse.ArgumentParser(prog="python -m kuna.pipeline.select_port")
    p.add_argument("--shell", action="store_true", help="emit shell KEY='val' assignments")
    p.add_argument("--json", action="store_true")
    p.add_argument("--checklist", default=None, help="alternate checklist.json path")
    args = p.parse_args(argv)

    items = load_checklist(args.checklist) if args.checklist else None
    item = pick(items)
    if item is None:
        if not (args.shell or args.json):
            print("no eligible checklist items", file=sys.stderr)
        return 1

    if args.json:
        print(json.dumps(item, indent=2))
        return 0
    if args.shell:
        def sh(v):
            return "'" + str(v).replace("'", "'\\''") + "'"
        print("ITEM_ID=%s" % sh(item["id"]))
        print("KIND=%s" % sh(item["kind"]))
        print("CRATE=%s" % sh(item.get("crate", "")))
        print("MODULES=%s" % sh(" ".join(item.get("modules", []))))
        print("GATE=%s" % sh(item.get("gate", "")))
        print("SLUG=%s" % sh(item["slug"]))
        print("TARGET_BRANCH=%s" % sh(item["target_branch"]))
        return 0
    # default human
    print("next: [%s %s] %s  (crate %s, ~%d loc)"
          % (item.get("wave"), item["kind"], item["id"],
             item.get("crate"), item.get("loc_estimate", 0)))
    if item["target_branch"]:
        print("  verifies branch: %s" % item["target_branch"])
    print("  gate: %s" % item.get("gate", ""))
    return 0


if __name__ == "__main__":
    sys.exit(main())
