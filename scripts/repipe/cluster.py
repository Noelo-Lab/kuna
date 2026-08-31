"""Collapse gated observations into needs, deterministically first and by LLM only if asked.

Dedup here keys off the PROBE, not the prose. Two testers who hit the same gap will describe
it in two different essays but will run substantially the same command and assert
substantially the same thing, so the signature is built from the kuna subcommand, the
observation kind, and the shape of the expect clauses. Text similarity is a tie-breaker, not
the key.

That ordering is deliberate and it is what keeps the loop cheap: bumping `instances` and
appending a challenge to an existing need is pure Python and costs nothing, so only a
genuinely novel observation is ever worth an agent's attention.

Only `admitted` observations reach this module — verify.py's two-arm gate has already
discarded the ones that did not reproduce and the ones kuna could already do.
"""
from __future__ import annotations

import argparse
import json
import os
import re
import sys
from collections import defaultdict

from . import config

_WORD = re.compile(r"[a-z0-9]+")

# Words that carry no discriminating power in a kuna friction report; every observation has
# them, so leaving them in makes everything look similar to everything else.
_STOP = frozenset("""
a an the and or but of to in on for with is are was were be been it its this that these those
kuna decompiler binary function functions output cannot could would should does did not no
i my we you when then than there here what which how why very really just only also more most
""".split())


def _tokens(text):
    return {w for w in _WORD.findall((text or "").lower()) if w not in _STOP and len(w) > 2}


def _subcommand(probe):
    """The kuna subcommand a probe drives, which is most of a friction report's identity."""
    cmd = (probe or {}).get("cmd") or []
    known = {"decompile", "decompile-all", "decompile-project", "functions",
             "test", "catalog", "modes", "specs", "fid"}
    for tok in cmd[1:]:
        if tok in known:
            return tok
        if not tok.startswith("-"):
            continue
    return "?"


def _clause_shape(probe):
    """Which assertions a probe makes, ignoring their values.

    'exit_code + json' is a different complaint from 'wall_ms', even about the same command.
    """
    return tuple(sorted((probe or {}).get("expect", {}).keys()))


def signature(obs):
    """The deterministic dedup key."""
    return "|".join([
        obs.get("kind", "?"),
        _subcommand(obs.get("probe")),
        ",".join(_clause_shape(obs.get("acceptance"))),
    ])


def similarity(a, b):
    """Jaccard over content words of title + what_i_wanted. The tie-breaker, not the key."""
    ta = _tokens(a.get("title", "") + " " + a.get("what_i_wanted", ""))
    tb = _tokens(b.get("title", "") + " " + b.get("what_i_wanted", ""))
    if not ta or not tb:
        return 0.0
    return len(ta & tb) / float(len(ta | tb))


NEAR = float(os.environ.get("REPIPE_CLUSTER_NEAR", "0.45"))


def group(observations):
    """[[obs, ...], ...] — exact-signature groups, then near-duplicates merged within a kind."""
    by_sig = defaultdict(list)
    for o in observations:
        by_sig[signature(o)].append(o)
    groups = list(by_sig.values())

    merged, used = [], set()
    for i, gi in enumerate(groups):
        if i in used:
            continue
        cur = list(gi)
        for j in range(i + 1, len(groups)):
            if j in used:
                continue
            gj = groups[j]
            if gi[0].get("kind") != gj[0].get("kind"):
                continue
            if similarity(gi[0], gj[0]) >= NEAR:
                cur.extend(gj)
                used.add(j)
        used.add(i)
        merged.append(cur)
    return merged


# A need_id becomes a branch name, a tests/cli/ filename and the thing a builder is told to
# fix, so "kuna-has-no-way-to" is not good enough. Filler is dropped before truncation.
_SLUG_DROP = frozenset("""
kuna the a an is are was were be to of for on in at it its this that with and or but
no not cannot could would should does did has have had any all only just very
""".split())


def _slug(text, fallback="need"):
    words = [w for w in re.sub(r"[^a-z0-9]+", " ", (text or "").lower()).split()
             if w not in _SLUG_DROP]
    if not words:
        words = re.sub(r"[^a-z0-9]+", " ", (text or "").lower()).split()
    return "-".join(words[:4]) or fallback


def need_id_for(obs_group):
    return _slug(obs_group[0].get("title"), "need")


def _pick_witness(obs_group):
    """The instance a builder will actually work from: prefer the most severe, then the
    smallest binary, because a 3.4 MB PE costs 445 s per probe replay and an 11 KB ELF costs
    0.15 s."""
    order = {"blocker": 0, "major": 1, "minor": 2}
    def key(o):
        tgt = (o.get("probe") or {}).get("target") or {}
        return (order.get(o.get("severity"), 3), tgt.get("binary_size", 1 << 62))
    return sorted(obs_group, key=key)[0]


TRACK_BY_SUBCOMMAND = {
    "decompile": "quality", "decompile-all": "quality", "decompile-project": "quality",
    "functions": "tooling", "catalog": "tooling", "modes": "tooling",
}


def infer_track(obs_group):
    """tooling | quality | perf | loader — the field that decides the builder's whole protocol.

    A missing capability is tooling however it was found; a wrong-output complaint is quality
    however it was phrased; anything timing-shaped is perf. Only the residue is guessed from
    the subcommand, and the builder is told to re-check it.
    """
    kinds = {o.get("kind") for o in obs_group}
    if kinds & {"too-slow", "cost"}:
        return "perf"
    if kinds & {"missing-capability", "bad-ux"}:
        return "tooling"
    if "silent-failure" in kinds or "crash" in kinds:
        w = _pick_witness(obs_group)
        return "loader" if _subcommand(w.get("probe")) == "functions" else "tooling"
    if "wrong-output" in kinds:
        return "quality"
    return TRACK_BY_SUBCOMMAND.get(_subcommand(_pick_witness(obs_group).get("probe")), "tooling")


TOUCHES_BY_TRACK = {
    "tooling": ["decompiler/crates/kuna-cli"],
    "quality": ["decompiler/crates/kuna-decomp"],
    "loader": ["decompiler/crates/kuna-analysis"],
    "perf": [],
}


def build_needs(observations, round_n):
    """Groups -> Need records, merging into any existing need with the same id.

    An existing need is BUMPED, never rewritten: its instances and challenges grow, its
    rounds list gains this round, and a previously closed need that reappears becomes
    `regressed`, which rank_score puts at the front of the queue.
    """
    from . import needs as needs_mod
    out = []
    for grp in group(observations):
        nid = need_id_for(grp)
        witness = _pick_witness(grp)
        challenges = sorted({o.get("_hexid") for o in grp if o.get("_hexid")})
        existing = needs_mod.load(nid)
        if existing is not None:
            existing.fields["instances"] = int(existing.instances or 0) + len(grp)
            existing.fields["challenges"] = sorted(set(list(existing.challenges or []) + challenges))
            existing.fields["rounds"] = sorted(set(list(existing.rounds or []) + [round_n]))
            if existing.status == "closed":
                # A closed need whose symptom came back is a regression, and rank_score puts
                # a regression at the front of the queue.
                existing.fields["status"] = "regressed"
            needs_mod.upsert(existing)
            out.append(existing)
            continue
        track = infer_track(grp)
        need = needs_mod.Need(fields={
            "need_id": nid,
            "title": witness.get("title", nid),
            "track": track,
            "status": "open",
            "severity": witness.get("severity", "major"),
            "probe_id": _pid(witness.get("probe"), False),
            "acceptance_id": _pid(witness.get("acceptance"), True),
            "hypothesis_status": "inconclusive",
            "credibility": credibility(grp),
            "instances": len(grp),
            "challenges": challenges,
            "rounds": [round_n],
            "first_seen_round": round_n,
            "attempts": 0,
            "touches": TOUCHES_BY_TRACK.get(track, []),
            "scope": "small",
            "regression_of": witness.get("regression_of"),
        }, sections=_sections(grp, witness))
        needs_mod.upsert(need)
        out.append(need)
    return out


def credibility(grp):
    """0..1 — how much of this is corroboration rather than one agent's opinion.

    Distinct testers matter more than raw instance count: three complaints from one tester
    in one session is one opinion repeated.
    """
    testers = {o.get("_tester") for o in grp if o.get("_tester")}
    challenges = {o.get("_hexid") for o in grp if o.get("_hexid")}
    score = 0.3 + 0.25 * min(len(testers), 3) + 0.15 * min(len(challenges), 3)
    if any(o.get("reference_better") for o in grp):
        score += 0.15   # "IDA does this and kuna cannot" is the most concrete evidence there is
    return round(min(score, 1.0), 2)


def _pid(probe, is_acceptance):
    """The probe's id, derived if the tester did not carry one (they are derived, not authored)."""
    if not probe:
        return None
    if probe.get("probe_id"):
        return probe["probe_id"]
    try:
        from . import probe as probe_mod
        return probe_mod.probe_id(probe, is_acceptance=is_acceptance)
    except Exception:
        return None


def _quote(text, limit=4000):
    """Neutralise untrusted tester prose before embedding it in a need record.

    A need is Markdown whose `##` headings ARE the schema: `## Acceptance` holds the probe
    that decides when the need is closed and what gets promoted into tests/cli/. Tester prose
    is written by an LLM working on an adversarial corpus, so a report containing its own
    `## Acceptance` heading (or a stray ``` fence) would splice a chosen probe into the
    record. Headings and fences are defanged, and the text is length-capped.
    """
    if not text:
        return ""
    out = []
    for line in str(text)[:limit].splitlines():
        stripped = line.lstrip()
        if stripped.startswith("#"):
            line = line.replace("#", "\\#", 1)
        elif stripped.startswith("```") or stripped.startswith("~~~"):
            line = "`" + stripped[3:]
        out.append(line)
    return "\n".join(out)


def _sections(grp, witness):
    quotes = "\n".join(
        "> **%s** (%s, `%s`)\n> %s\n" % (_quote(o.get("title", ""), 200),
                                         o.get("severity", "?"), o.get("_hexid", "?"),
                                         _quote(o.get("what_kuna_did", ""), 600))
        for o in grp[:6])
    refs = [o for o in grp if o.get("reference_better")]
    ref_md = "\n".join("- `%s` — %s" % (_quote(r["reference_better"].get("command", ""), 200),
                                        _quote(r["reference_better"].get("evidence", ""), 400))
                       for r in refs) or "_none recorded_"
    hyps = "\n".join("- %s" % _quote(o["hypothesis"], 600)
                     for o in grp if o.get("hypothesis")) or "_none offered_"
    return {
        "Symptom": "%s\n\n%s" % (_quote(witness.get("what_i_wanted", ""), 800), quotes),
        "Reproduction": "```json\n%s\n```" % json.dumps(witness.get("probe"), indent=2),
        "Acceptance": "```json\n%s\n```" % json.dumps(witness.get("acceptance"), indent=2),
        "Hypothesis": ("**Advisory — the builder is not bound by this.** In the sibling "
                       "campaign 3 of 8 filed diagnoses were overturned while the symptom "
                       "stood in all 8.\n\n%s" % hyps),
        "Refutation": "_not yet refuted_",
        "Reference": ref_md,
        "Instances": "\n".join("- `%s` (round %s, tester %s)" % (
            o.get("_hexid", "?"), o.get("_round", "?"), o.get("_tester", "?")) for o in grp),
        "Decision log": "- filed by cluster.py from %d observation(s)" % len(grp),
    }


def load_round_observations(round_n):
    """Admitted observations for a round, stamped with where each came from."""
    gate_path = config.rounds_dir() / str(round_n) / "gate.json"
    if not gate_path.exists():
        raise SystemExit("no gate result at %s — run verify --gate --round %s first"
                         % (gate_path, round_n))
    with open(gate_path) as fh:
        gate = json.load(fh)
    out = []
    for row in gate.get("results", []):
        if row.get("verdict") != "admitted":
            continue
        obs = dict(row.get("observation") or {})
        obs["_hexid"] = row.get("hexid")
        obs["_tester"] = row.get("tester_id")
        obs["_round"] = round_n
        out.append(obs)
    return out


def main(argv=None):
    ap = argparse.ArgumentParser(prog="python -m scripts.repipe.cluster")
    ap.add_argument("--round", type=int, required=True)
    ap.add_argument("--from-file", default=None,
                    help="a JSON list of admitted observations, instead of the round's gate.json")
    ap.add_argument("--dry-run", action="store_true")
    ap.add_argument("--json", action="store_true")
    args = ap.parse_args(argv)

    if args.from_file:
        with open(args.from_file) as fh:
            obs = json.load(fh)
    else:
        obs = load_round_observations(args.round)

    if args.dry_run:
        groups = group(obs)
        out = [{"need_id": need_id_for(g), "track": infer_track(g), "instances": len(g),
                "challenges": sorted({o.get("_hexid") for o in g if o.get("_hexid")}),
                "credibility": credibility(g)} for g in groups]
        print(json.dumps(out, indent=2) if args.json else
              "\n".join("%-28s %-8s x%-3d %s" % (r["need_id"], r["track"], r["instances"],
                                                 ",".join(r["challenges"])) for r in out))
        return 0

    made = build_needs(obs, args.round)
    from . import needs as needs_mod
    needs_mod.reindex()
    if args.json:
        print(json.dumps([{"need_id": n.need_id, "track": n.track, "status": n.status,
                           "instances": n.instances} for n in made], indent=2))
    else:
        for n in made:
            print("%-28s %-8s %-10s x%d" % (n.need_id, n.track, n.status, n.instances))
    return 0


if __name__ == "__main__":
    sys.exit(main())
