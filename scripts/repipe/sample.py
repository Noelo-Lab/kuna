"""The stratified round slate: which challenges round N points its testers at.

Why stratify at all: the corpus is lopsided. Of the 250 records in `manifest.json` the
primary binary is PE 139 / ELF 83 / Mach-O 22 / DOS 6, the arch is x86-64 for 143 of them,
and 79 carry no obfuscation class whatsoever. Nine uniformly-drawn challenges are therefore
most often nine x86-64 PEs -- and a round that only ever sees one loader shape only ever
finds one loader's worth of friction. The four dimensions sampled here (format, primary
arch, declared difficulty, obfuscation-class count) are the ones that actually change which
part of kuna is exercised: the loader, the sleigh spec, the analysis depth, and the
structuring.

Why deterministic: a captain tick is bounded (REPIPE_CAPTAIN_TIMEOUT) and can die inside
T_PLAN with half the arenas built. Seeding `random.Random(f"{seed}:{round_n}")` means the
replay of a crashed T_PLAN produces the identical slate, so the arenas already on disk are
still the right arenas. The seed defaults to a fixed string rather than to entropy for the
same reason -- an unseeded run must still replay.

Why the large-binary cap: kuna's worst measured case on this machine is 445 s at 1,476 MiB
RSS for a 3.4 MB PE `decompile-project`, and 342 s for an 866 KB challenge ELF in
`aggressive` (99 s in `reliable`). Nine of those in one round is a round that spends its
entire wall-clock budget inside decompile-all and reports no friction at all. `--max-large`
(default 2) bounds it.

The per-challenge timeout returned with each record is derived from `detected.primary.size`
against the `auto` mode cutovers, and is deliberately the *same* arithmetic as
`render_tester_prompt._time_budget_minutes` -- the slate and the prompt the tester reads
must not disagree about how long it has.

CLI:
    python3 -m scripts.repipe.sample slate --round 3 [-k 9] [--seed S] [--max-large 2]
            [--exclude hexid,hexid] [--filter 'format=ELF,size<64k'] [--write] [--json]
    python3 -m scripts.repipe.sample coverage [--json]
"""
from __future__ import annotations

import argparse
import collections
import json
import os
import random
import sys

from . import config

# `kuna --mode auto` picks its pipeline from the input size at these byte boundaries.
AGGRESSIVE_MAX = 512_000
RELIABLE_MAX = 2_097_152

# "multi-MB", i.e. the band where a single decompile-all has been measured in minutes.
LARGE_BYTES = int(os.environ.get("REPIPE_LARGE_BYTES", RELIABLE_MAX))
MAX_LARGE = int(os.environ.get("REPIPE_MAX_LARGE", "2"))

# How many previous rounds' slates are off-limits, so a tester does not re-litigate a
# challenge whose friction is already filed.
RECENT_ROUNDS = int(os.environ.get("REPIPE_RECENT_ROUNDS", "3"))

DEFAULT_SEED = os.environ.get("REPIPE_SEED", "kuna-repipe")

# Ceiling for a single kuna invocation against a target of this size, used by probe.py when
# a tester does not supply one. Anchored on the two measured worst cases above, floored at
# REPIPE_PROBE_TIMEOUT so it is never tighter than the generic probe budget.
CALL_BUDGET_S = {"aggressive": 400, "reliable": 500, "fast": 600}

DIMENSIONS = ("format", "arch", "difficulty", "obfuscation")


# --- the corpus -------------------------------------------------------------

def load_manifest(path=None):
    """The 250 dataset records. Read-only: nothing here ever writes into the dataset."""
    p = path or config.manifest_path()
    with open(p) as fh:
        return json.load(fh)


def _primary(rec):
    return (rec.get("detected") or {}).get("primary") or {}


def difficulty_band(rec):
    """Declared difficulty is a crackmes.one vote average (1.0-6.0, 41 distinct values here).

    Rounded to the 1-6 band it was voted on; missing votes sort with the easy end rather
    than inventing a hard rating for a challenge nobody graded.
    """
    raw = (rec.get("declared") or {}).get("difficulty")
    if raw is None:
        return 1
    return max(1, min(6, int(round(float(raw)))))


def obfuscation_band(rec):
    """0 / 1-2 / 3-4 / 5+ classes. The count is what predicts how hard kuna's structuring
    is hit; which particular classes matter is the tester's problem, not the sampler's."""
    n = len((rec.get("obfuscation") or {}).get("classes") or [])
    if n == 0:
        return "none"
    if n <= 2:
        return "light"
    if n <= 4:
        return "heavy"
    return "extreme"


def arch_of(rec):
    """Primary arch. Fat binaries are recorded as 'x86-64+x86'; the first component is the
    one kuna will actually be pointed at, so that is the stratum."""
    return (_primary(rec).get("arch") or "unknown").split("+")[0]


def levels(rec):
    return {
        "format": _primary(rec).get("format") or "unknown",
        "arch": arch_of(rec),
        "difficulty": difficulty_band(rec),
        "obfuscation": obfuscation_band(rec),
    }


def mode_for(size):
    """The mode `kuna --mode auto` will select for a binary of this size."""
    if size < AGGRESSIVE_MAX:
        return "aggressive"
    if size < RELIABLE_MAX:
        return "reliable"
    return "fast"


def budget_for(size):
    """Suggested per-challenge tester budget, derived from size against the auto cutovers.

    Identical arithmetic to render_tester_prompt._time_budget_minutes so the slate and the
    rendered prompt cannot disagree: a >=2 MB target gets the full REPIPE_TESTER_TIMEOUT
    because one decompile-all on it has been measured at 445 s, and a small target gets 60%
    of it because on a <512 KB binary kuna is fast and the time goes on thinking instead.
    """
    base = max(1, config.TESTER_TIMEOUT // 60)
    if size >= RELIABLE_MAX:
        minutes = base
    elif size >= AGGRESSIVE_MAX:
        minutes = max(20, int(base * 0.8))
    else:
        minutes = max(15, int(base * 0.6))
    mode = mode_for(size)
    return {
        "mode": mode,
        "timeout_minutes": minutes,
        "timeout_s": minutes * 60,
        "call_timeout_s": max(config.PROBE_TIMEOUT, CALL_BUDGET_S[mode]),
    }


def describe(rec):
    """The slate record a tester, an arena builder and the dashboard all read."""
    prim = _primary(rec)
    gt = rec.get("ground_truth") or {}
    size = int(prim.get("size") or 0)
    lv = levels(rec)
    out = {
        "hexid": rec.get("hexid"),
        "name": rec.get("name"),
        "format": lv["format"],
        "arch": lv["arch"],
        "arch_full": prim.get("arch"),
        "size": size,
        "binary_rel": prim.get("path"),
        "difficulty": (rec.get("declared") or {}).get("difficulty"),
        "difficulty_band": lv["difficulty"],
        "obfuscation": list((rec.get("obfuscation") or {}).get("classes") or []),
        "obfuscation_count": len((rec.get("obfuscation") or {}).get("classes") or []),
        "obfuscation_band": lv["obfuscation"],
        "machine_checkable": bool(gt.get("machine_checkable")),
        "has_flag": bool(gt.get("flag")),
        "verifier": gt.get("verifier"),
        "ships_source_code": bool((rec.get("contamination") or {}).get("ships_source_code")),
        "large": size >= LARGE_BYTES,
    }
    out.update(budget_for(size))
    return out


# --- round bookkeeping ------------------------------------------------------

def slate_path(round_n):
    return config.rounds_dir() / str(round_n) / "slate.json"


def read_slate(round_n):
    p = slate_path(round_n)
    if not os.path.exists(p):
        return None
    with open(p) as fh:
        return json.load(fh)


def write_slate(round_n, slate, meta=None):
    """Persist atomically: T_PLAN is replayable and a torn slate would plan a torn round."""
    p = slate_path(round_n)
    os.makedirs(os.path.dirname(p), exist_ok=True)
    doc = {"round": int(round_n), "count": len(slate), "challenges": slate}
    if meta:
        doc.update(meta)
    tmp = str(p) + ".tmp"
    with open(tmp, "w") as fh:
        json.dump(doc, fh, indent=2, sort_keys=False)
        fh.write("\n")
    os.replace(tmp, p)
    return p


def _slate_hexids(doc):
    if not doc:
        return []
    return [c.get("hexid") for c in doc.get("challenges", []) if c.get("hexid")]


def known_rounds():
    d = config.rounds_dir()
    if not os.path.isdir(d):
        return []
    out = []
    for name in os.listdir(d):
        if name.isdigit() and os.path.exists(os.path.join(d, name, "slate.json")):
            out.append(int(name))
    return sorted(out)


def recent_hexids(round_n, back=None):
    """Everything attempted in the `back` rounds before round_n."""
    back = RECENT_ROUNDS if back is None else int(back)
    seen = set()
    if back <= 0:
        return seen
    for n in range(max(0, int(round_n) - back), int(round_n)):
        seen.update(_slate_hexids(read_slate(n)))
    return seen


def attempted_hexids():
    """Every challenge any persisted round has ever pointed a tester at."""
    seen = set()
    for n in known_rounds():
        seen.update(_slate_hexids(read_slate(n)))
    return seen


# --- filters ----------------------------------------------------------------

_FILTER_FIELDS = {
    "hexid": lambda r: r["hexid"],
    "name": lambda r: r["name"],
    "format": lambda r: r["format"],
    "arch": lambda r: r["arch"],
    "size": lambda r: r["size"],
    "difficulty": lambda r: r["difficulty_band"],
    "obfuscation": lambda r: r["obfuscation_count"],
    "machine_checkable": lambda r: r["machine_checkable"],
    "has_flag": lambda r: r["has_flag"],
    "large": lambda r: r["large"],
    "mode": lambda r: r["mode"],
}
_OPS = ("<=", ">=", "!=", "==", "~", "<", ">", "=")


def _as_number(text):
    t = text.strip().lower()
    mult = 1
    if t.endswith("k"):
        mult, t = 1024, t[:-1]
    elif t.endswith("m"):
        mult, t = 1024 * 1024, t[:-1]
    return float(t) * mult


def _coerce(value, want):
    if isinstance(want, bool):
        return str(value).strip().lower() in ("1", "true", "yes", "y")
    if isinstance(want, (int, float)):
        return _as_number(value)
    return value


def parse_filter(text):
    """`format=ELF,size<64k,machine_checkable=true` -> a predicate over describe() records.

    This is the `--challenge-filter` the runbook's Level-2 single-tester smoke passes; it is
    parsed here rather than in run.sh so both the shell and the captain get one dialect.
    """
    clauses = []
    for raw in (text or "").split(","):
        raw = raw.strip()
        if not raw:
            continue
        for op in _OPS:
            if op in raw:
                key, val = raw.split(op, 1)
                break
        else:
            raise ValueError("filter clause %r has no operator" % raw)
        key = key.strip()
        if key not in _FILTER_FIELDS:
            raise ValueError("unknown filter field %r (have: %s)"
                             % (key, ", ".join(sorted(_FILTER_FIELDS))))
        clauses.append((key, op, val.strip()))

    def pred(rec):
        for key, op, val in clauses:
            have = _FILTER_FIELDS[key](rec)
            if op == "~":
                if str(val).lower() not in str(have).lower():
                    return False
                continue
            want = _coerce(val, have)
            if isinstance(have, bool):
                ok = (have == want) if op in ("=", "==") else (have != want)
            elif isinstance(have, (int, float)) and not isinstance(want, str):
                have_f = float(have)
                ok = {"<": have_f < want, "<=": have_f <= want,
                      ">": have_f > want, ">=": have_f >= want,
                      "=": have_f == want, "==": have_f == want,
                      "!=": have_f != want}[op]
            else:
                ok = {"=": str(have) == want, "==": str(have) == want,
                      "!=": str(have) != want}.get(op)
                if ok is None:
                    raise ValueError("operator %r does not apply to %r" % (op, key))
            if not ok:
                return False
        return True

    return pred


# --- the sampler ------------------------------------------------------------

def _quota(values, k):
    """Largest-remainder allocation of k slots across the levels present in the pool.

    At k=9 over the real corpus this is PE 5 / ELF 3 / Mach-O 1 / DOS 0 -- DOS is 6 of 250,
    so it earns a slot roughly one round in four rather than every round.
    """
    counts = collections.Counter(values)
    n = sum(counts.values())
    if not n:
        return {}
    exact = dict((lv, k * c / float(n)) for lv, c in counts.items())
    base = dict((lv, int(v)) for lv, v in exact.items())
    left = k - sum(base.values())
    order = sorted(counts, key=lambda lv: (-(exact[lv] - base[lv]), str(lv)))
    for lv in order[:max(0, left)]:
        base[lv] += 1
    return base


def sample(round_n, k=None, exclude=None, seed=None, max_large=None,
           recent_rounds=None, records=None, filters=None):
    """Pick k challenges for round `round_n`, stratified over the four dimensions.

    Deterministic given (seed, round_n): the RNG is `random.Random(f"{seed}:{round_n}")`, so
    a T_PLAN that crashes and replays produces byte-identical output, and a different round
    with the same seed produces a different slate.

    Selection is greedy-by-deficit rather than per-cell quota because the composite cell
    space (4 formats x 13 arches x 6 difficulties x 4 obfuscation bands) is far larger than
    k: each slot goes to the candidate that closes the most quota deficit summed over the
    four dimensions independently, ties broken by the seeded RNG. That keeps every marginal
    distribution close to the corpus without demanding a candidate for a cell that has none.

    `exclude` (plus the last `recent_rounds` slates) is removed from the pool first; the
    `max_large` cap then stops applying once that many >=LARGE_BYTES targets are chosen.
    """
    k = int(config.ROUND_CHALLENGES if k is None else k)
    max_large = MAX_LARGE if max_large is None else int(max_large)
    seed = DEFAULT_SEED if seed is None else seed
    rng = random.Random("%s:%s" % (seed, round_n))

    raw = load_manifest() if records is None else records
    pool = [describe(r) for r in raw]

    blocked = set(exclude or ())
    blocked |= recent_hexids(round_n, recent_rounds)
    pool = [r for r in pool if r["hexid"] not in blocked]
    if filters:
        pred = filters if callable(filters) else parse_filter(filters)
        pool = [r for r in pool if pred(r)]
    pool.sort(key=lambda r: r["hexid"])
    if not pool:
        return []

    targets = dict((dim, _quota([levels_of(r)[dim] for r in pool], k)) for dim in DIMENSIONS)
    taken = dict((dim, collections.Counter()) for dim in DIMENSIONS)

    chosen = []
    chosen_ids = set()
    n_large = 0
    for _ in range(min(k, len(pool))):
        cands = [r for r in pool if r["hexid"] not in chosen_ids]
        if n_large >= max_large:
            cands = [r for r in cands if not r["large"]]
        if not cands:
            break
        best, tied = None, []
        for r in cands:
            lv = levels_of(r)
            score = sum(max(0, targets[d].get(lv[d], 0) - taken[d][lv[d]]) for d in DIMENSIONS)
            if best is None or score > best:
                best, tied = score, [r]
            elif score == best:
                tied.append(r)
        pick = tied[rng.randrange(len(tied))]
        chosen.append(pick)
        chosen_ids.add(pick["hexid"])
        n_large += 1 if pick["large"] else 0
        for d in DIMENSIONS:
            taken[d][levels_of(pick)[d]] += 1
    return chosen


def levels_of(desc):
    """levels() over an already-described record (describe() flattens the manifest shape)."""
    return {"format": desc["format"], "arch": desc["arch"],
            "difficulty": desc["difficulty_band"], "obfuscation": desc["obfuscation_band"]}


def mix(slate):
    """The marginal distribution of a slate, for the plan log and the dashboard."""
    out = {}
    for dim in DIMENSIONS:
        c = collections.Counter(levels_of(r)[dim] for r in slate)
        out[dim] = dict((str(kk), v) for kk, v in sorted(c.items(), key=lambda kv: str(kv[0])))
    out["large"] = sum(1 for r in slate if r["large"])
    out["machine_checkable"] = sum(1 for r in slate if r["machine_checkable"])
    return out


# --- coverage ---------------------------------------------------------------

def outcomes_dir(round_n):
    return config.rounds_dir() / str(round_n) / "outcomes"


def _outcomes():
    """hexid -> the graded outcome, preferring grade.py's record over the raw report.

    Two sources because they fail differently: `rounds/<N>/outcomes/<hexid>.json` is written
    challenge-side by `grade.py --record` and carries the tiered verdict and the tripwire
    result; `arena/<N>/<hexid>/report.json` is the tester's own self-report and survives a
    crash in T_DRAIN that happened before grading. `gave_up`, `partial` and `failed` are
    taken from either -- nobody but the tester can report a give-up -- but an ungraded
    `solved` CLAIM counts only as attempted, because the whole point of grade.py is that a
    model saying it solved something is not evidence that it did.
    """
    out = {}
    for n in known_rounds():
        arena = config.arena_dir() / str(n)
        if os.path.isdir(arena):
            for hexid in sorted(os.listdir(arena)):
                p = os.path.join(arena, hexid, "report.json")
                if not os.path.exists(p):
                    continue
                try:
                    with open(p) as fh:
                        rep = json.load(fh)
                except (ValueError, OSError):
                    continue
                out[hexid] = {"round": n, "outcome": rep.get("outcome"),
                              "verdict": None, "contaminated": False, "source": "report"}
        od = outcomes_dir(n)
        if os.path.isdir(od):
            for name in sorted(os.listdir(od)):
                if not name.endswith(".json"):
                    continue
                try:
                    with open(os.path.join(od, name)) as fh:
                        rec = json.load(fh)
                except (ValueError, OSError):
                    continue
                hexid = rec.get("hexid") or name[:-5]
                out[hexid] = {"round": n, "outcome": rec.get("outcome"),
                              "verdict": rec.get("verdict"),
                              "contaminated": bool(rec.get("contaminated")),
                              "source": "grade"}
    return out


_TALLIES = ("attempted", "solved", "partial", "gave_up", "failed", "ungraded", "contaminated")


def coverage(records=None):
    """attempted / solved / gave-up per stratum, over the whole corpus as denominator.

    The dashboard's `/api/corpus` route reads this. Solve rate is a SECONDARY metric here --
    ground truth is weak (only 22 of 250 challenges are machine-checkable and uncontaminated)
    and the primary output of a tester run is probes. What this view is actually for is
    spotting a stratum the sampler has never reached: 6 DOS records and 3 MIPS records will
    not show up on their own.
    """
    raw = load_manifest() if records is None else records
    pool = [describe(r) for r in raw]
    outs = _outcomes()
    planned = attempted_hexids()

    strata = {}
    for dim in DIMENSIONS:
        strata[dim] = {}
        for r in pool:
            lv = str(levels_of(r)[dim])
            cell = strata[dim].setdefault(lv, dict([("total", 0)] + [(t, 0) for t in _TALLIES]))
            cell["total"] += 1
            o = outs.get(r["hexid"])
            if not o and r["hexid"] not in planned:
                continue
            cell["attempted"] += 1
            if not o:
                cell["ungraded"] += 1
                continue
            if o.get("contaminated"):
                cell["contaminated"] += 1
            outcome = o.get("outcome")
            if outcome == "solved" and o.get("source") != "grade":
                outcome = None
            if outcome == "solved":
                cell["solved"] += 1
            elif outcome == "partial":
                cell["partial"] += 1
            elif outcome == "gave_up":
                cell["gave_up"] += 1
            elif outcome == "failed":
                cell["failed"] += 1
            else:
                cell["ungraded"] += 1
        for cell in strata[dim].values():
            cell["remaining"] = cell["total"] - cell["attempted"]

    totals = dict([("total", len(pool))] + [(t, 0) for t in _TALLIES])
    for cell in strata["format"].values():
        for key in _TALLIES:
            totals[key] += cell[key]
    totals["remaining"] = totals["total"] - totals["attempted"]
    return {"corpus": len(pool), "rounds": known_rounds(),
            "totals": totals, "strata": strata}


# --- CLI --------------------------------------------------------------------

def _print_slate(slate, round_n, seed):
    print("round %s  seed %s  %d challenges" % (round_n, seed, len(slate)))
    print("%-26s %-8s %-8s %9s %4s %4s %-11s %6s  %s"
          % ("hexid", "format", "arch", "size", "diff", "obf", "mode", "budget", "name"))
    for r in slate:
        print("%-26s %-8s %-8s %9d %4s %4d %-11s %5dm  %s"
              % (r["hexid"], r["format"], r["arch"], r["size"], r["difficulty_band"],
                 r["obfuscation_count"], r["mode"], r["timeout_minutes"], r["name"]))
    m = mix(slate)
    for dim in DIMENSIONS:
        print("  %-12s %s" % (dim, ", ".join("%s=%s" % kv for kv in m[dim].items())))
    print("  %-12s %s   machine_checkable=%s" % ("large", m["large"], m["machine_checkable"]))


def _print_coverage(cov):
    t = cov["totals"]
    print("corpus %d  attempted %d  solved %d  gave_up %d  remaining %d  rounds %s"
          % (cov["corpus"], t["attempted"], t["solved"], t["gave_up"], t["remaining"],
             cov["rounds"] or "-"))
    for dim in DIMENSIONS:
        print("[%s]" % dim)
        for lv in sorted(cov["strata"][dim], key=lambda s: (-cov["strata"][dim][s]["total"], s)):
            c = cov["strata"][dim][lv]
            print("  %-12s total %4d  attempted %3d  solved %3d  gave_up %3d  remaining %4d"
                  % (lv, c["total"], c["attempted"], c["solved"], c["gave_up"], c["remaining"]))


def main(argv=None):
    p = argparse.ArgumentParser(prog="python -m scripts.repipe.sample")
    sub = p.add_subparsers(dest="cmd", required=True)

    sp = sub.add_parser("slate", help="pick the stratified slate for a round")
    sp.add_argument("--round", type=int, required=True)
    sp.add_argument("-k", "--count", type=int, default=None)
    sp.add_argument("--seed", default=None)
    sp.add_argument("--max-large", type=int, default=None)
    sp.add_argument("--recent-rounds", type=int, default=None)
    sp.add_argument("--exclude", default="", help="comma-separated hexids")
    sp.add_argument("--filter", default=None,
                    help="e.g. 'format=ELF,size<64k,machine_checkable=true'")
    sp.add_argument("--write", action="store_true", help="persist to rounds/<N>/slate.json")
    sp.add_argument("--json", action="store_true")

    sp = sub.add_parser("coverage", help="attempted/solved/gave-up per stratum")
    sp.add_argument("--json", action="store_true")

    sp = sub.add_parser("show", help="the slate record for one hexid")
    sp.add_argument("hexid")
    sp.add_argument("--json", action="store_true")

    args = p.parse_args(argv)

    if args.cmd == "coverage":
        cov = coverage()
        if args.json:
            print(json.dumps(cov, indent=2))
        else:
            _print_coverage(cov)
        return 0

    if args.cmd == "show":
        for rec in load_manifest():
            if rec.get("hexid") == args.hexid:
                print(json.dumps(describe(rec), indent=2))
                return 0
        print("no such hexid: %s" % args.hexid, file=sys.stderr)
        return 1

    exclude = [h.strip() for h in (args.exclude or "").split(",") if h.strip()]
    seed = DEFAULT_SEED if args.seed is None else args.seed
    try:
        slate = sample(args.round, args.count, exclude=exclude, seed=seed,
                       max_large=args.max_large, recent_rounds=args.recent_rounds,
                       filters=args.filter)
    except ValueError as exc:
        print("error: %s" % exc, file=sys.stderr)
        return 2

    meta = {"seed": str(seed), "max_large": MAX_LARGE if args.max_large is None
            else args.max_large, "mix": mix(slate)}
    if args.write:
        path = write_slate(args.round, slate, meta)
        if not args.json:
            print("wrote %s" % path)
    if args.json:
        doc = {"round": args.round, "count": len(slate), "challenges": slate}
        doc.update(meta)
        print(json.dumps(doc, indent=2))
    else:
        _print_slate(slate, args.round, seed)
    return 0 if slate else 1


if __name__ == "__main__":
    sys.exit(main())
