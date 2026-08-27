"""Corpus-scale, bidirectional A/B for a **function-discovery** option flip.

`optsweep` is the GED-tier ablation: it measures the C kuna emits for functions
it already found. This is its discovery-tier twin -- it measures *which functions
kuna finds* -- and it is the instrument GH-299 and GH-313 are scored on.

Every stripped binary in the decbench results tree has an unstripped `compiled/`
twin, so each arm's entry set is scored against an exact symbol table by
`entryscore`. An **arm** is `NAME:opt=val,opt=val` (an empty option list = the
shipped default under `--mode`), and every arm runs over the same targets in the
same process, so the comparison is paired.

Two things this exists to prevent, both of which have already produced a wrong
conclusion in this repo:

* **Aggregate blindness.** ~80% of the corpus mid-body mass sits in 8 of the 110
  non-x86-64 binaries, all ARM firmware, so a guard that trades PE precision for
  ARM recall reads as a clean win in the totals. The per-binary table and the
  `--tripwire` rows are printed on every run for that reason.
* **Baseline drift.** `--mode aggressive` carries the AGGRESSIVE_OVERRIDES preset,
  which for this corpus (every image is under the `auto` size threshold) is the
  shipped default path. A guard scored against a hand-disabled preset member
  books that member's win as its own.

Usage (from the repo root; plain python3, pyelftools optional)::

    # the GH-313 baseline command: 110 non-x86-64 twins, three arms
    python3 -m scripts.decbench.entrysweep --arch non-x86-64 \\
        --arms shipped: nostrict:aifstrict=off aifoff:aif=off \\
        --marginal aifoff --out /tmp/sweep.json

    # score a candidate guard: add an arm, keep `aifoff` as the marginal reference
    python3 -m scripts.decbench.entrysweep --arms shipped: g1:aifcorroborate=on \\
        aifoff:aif=off --marginal aifoff --out /tmp/sweep_g1.json

    # re-print a saved run (per-binary table + tripwires), no kuna runs
    python3 -m scripts.decbench.entrysweep --report /tmp/sweep_g1.json

    # a single witness with no decbench twin (a reference-decompiler inventory)
    python3 -m scripts.decbench.entrysweep --witness game.exe ida_game.json \\
        --arms shipped: g1:aifcorroborate=on aifoff:aif=off --marginal aifoff

`--only SUBSTR` filters targets by tag, `--limit N` caps them, `--arch x86-64`
runs the 693 x86-64 twins as the no-regression control, and `--tripwire SUBSTR`
(repeatable) names the per-binary rows to print first -- the images whose
regression vetoes a flip regardless of the totals.

Knobs: `KUNA_BIN`, `KUNA_SPECS`, `KUNA_DECBENCH_RESULTS` (see `config.py`).
"""
import argparse
import collections
import json
import os
import subprocess
import sys
from concurrent.futures import ThreadPoolExecutor

from . import config
from .entryscore import ground_truth, kuna_entries, score

SKIP_EXT = (".id0", ".id1", ".id2", ".nam", ".til", ".json", ".c", ".log", ".txt", ".idb", ".i64")
METRICS = ("entries", "exact", "midbody", "midbody_known", "unknown", "gt", "seconds")
MARGINALS = ("added", "added_exact", "added_midbody", "added_midbody_known", "lost")
DEFAULT_TRIPWIRES = ("u-boot", "CMSIS_DAP")


def targets(arch, opt_levels=("O0", "O2", "O2-noinline")):
    """-> [(tag, stripped_path, twin_path, filetype)] for every twinned image."""
    root_dir = config.results_root()
    out = []
    for opt in opt_levels:
        root = root_dir / opt
        if not root.is_dir():
            continue
        for proj in sorted(os.listdir(root)):
            sd, cd = root / proj / "stripped", root / proj / "compiled"
            if not sd.is_dir():
                continue
            for fn in sorted(os.listdir(sd)):
                p = sd / fn
                if not p.is_file() or fn.endswith(SKIP_EXT):
                    continue
                ft = subprocess.run(["file", "-b", str(p)], capture_output=True,
                                    text=True).stdout.strip()
                if not ("ELF" in ft or "PE32" in ft):
                    continue
                is64 = "x86-64" in ft
                if arch == "non-x86-64" and is64:
                    continue
                if arch == "x86-64" and not is64:
                    continue
                twin = cd / fn
                if not twin.is_file():
                    continue
                out.append((f"{opt}/{proj}/{fn}", str(p), str(twin), ft.split(",")[0]))
    return out


def parse_arm(s):
    name, _, rest = s.partition(":")
    opts = []
    for kv in filter(None, rest.split(",")):
        k, _, v = kv.partition("=")
        opts.append((k, v))
    return name, opts


def score_one(t, arms, mode, timeout, marginal):
    tag, sp, gtp, ft = t
    row = dict(tag=tag, ft=ft, size=os.path.getsize(sp))
    try:
        gt = ground_truth(gtp)
        ents = {}
        for name, opts in arms:
            ents[name], dt = kuna_entries(sp, opts, mode, timeout)
            row[name] = dict(seconds=round(dt, 2))
        for name, _ in arms:
            row[name].update(score(ents[name], gt))
        if marginal and marginal in ents:
            ref = ents[marginal]
            for name, _ in arms:
                if name == marginal:
                    continue
                m = score(ents[name] - ref, gt, other_entries=ref)
                row[name].update(added=m["entries"], added_exact=m["exact"],
                                 added_midbody=m["midbody"],
                                 added_midbody_known=m["midbody_known"],
                                 lost=len(ref - ents[name]))
    except Exception as e:  # noqa: BLE001
        row["error"] = f"{type(e).__name__}: {e}"[:300]
    return row


def report(rows, arms, tripwires=DEFAULT_TRIPWIRES, per_binary=True):
    ok = [r for r in rows if not r.get("error")]
    bad = [r for r in rows if r.get("error")]
    for r in bad:
        print(f"  ERR {r['tag']}: {r['error']}")
    print(f"\nscored {len(ok)}/{len(rows)} binaries")

    # Tripwires FIRST: an image whose regression vetoes the flip outright.
    tw = [r for r in ok if any(s in r["tag"] for s in tripwires)]
    if tw:
        print("\n--- tripwire rows (e=entries x=exact m=midbody) ---")
        for r in sorted(tw, key=lambda r: r["tag"]):
            cells = "  ".join(f"{n}: e{r[n]['entries']} x{r[n]['exact']} m{r[n]['midbody']}"
                              for n in arms)
            print(f"  {r['tag']:<34} {cells}  gt={r[arms[0]]['gt']}")

    tot = {n: collections.Counter() for n in arms}
    for r in ok:
        for n in arms:
            for k in METRICS:
                tot[n][k] += r[n][k]
    print()
    print(("%-12s" + "%14s" * len(METRICS) + "%22s") % (("arm",) + METRICS + ("recall",)))
    for n in arms:
        t = tot[n]
        rec = f"{t['exact']}/{t['gt']} ({100 * t['exact'] / t['gt']:.2f}%)" if t["gt"] else "-"
        print(("%-12s" + "%14s" * len(METRICS) + "%22s") % (
            (n,) + tuple(round(t[k], 1) for k in METRICS) + (rec,)))
    base = arms[0]
    for n in arms[1:]:
        print(f"delta {n} - {base}: midbody {tot[n]['midbody'] - tot[base]['midbody']:+d}, "
              f"exact {tot[n]['exact'] - tot[base]['exact']:+d}, "
              f"entries {tot[n]['entries'] - tot[base]['entries']:+d}")

    mt = {n: collections.Counter() for n in arms}
    has_m = False
    for r in ok:
        for n in arms:
            for k in MARGINALS:
                if k in r[n]:
                    mt[n][k] += r[n][k]
                    has_m = True
    if has_m:
        print(("\n%-12s" + "%20s" * len(MARGINALS)) % (("arm (marginal)",) + MARGINALS))
        for n in arms:
            if mt[n]:
                print(("%-12s" + "%20s" * len(MARGINALS)) % (
                    (n,) + tuple(mt[n][k] for k in MARGINALS)))

    if per_binary and len(ok) > 1:
        print(f"\nper-binary, sorted by |midbody| in `{arms[0]}` "
              "(the corpus totals are dominated by a handful of these)")
        hdr = ("tag",) + tuple(f"{n}.{k}" for n in arms for k in ("entries", "exact", "midbody"))
        print(("%-42s" + "%9s" * (len(hdr) - 1)) % hdr)
        for r in sorted(ok, key=lambda r: -r[arms[0]]["midbody"]):
            print(("%-42s" + "%9s" * (len(hdr) - 1)) % (
                (r["tag"][:42],) + tuple(r[n][k] for n in arms
                                         for k in ("entries", "exact", "midbody"))))


def main(argv=None):
    ap = argparse.ArgumentParser(prog="entrysweep", description=__doc__.splitlines()[0])
    ap.add_argument("--arch", default="non-x86-64", choices=["non-x86-64", "x86-64", "all"])
    ap.add_argument("--arms", nargs="+", default=["shipped:", "aifoff:aif=off"],
                    help="NAME:opt=val,opt=val (empty option list = the shipped default)")
    ap.add_argument("--mode", default="aggressive")
    ap.add_argument("--marginal", default="", help="arm to diff every other arm against")
    ap.add_argument("--only", default="", help="substring filter on the tag")
    ap.add_argument("--limit", type=int, default=0)
    ap.add_argument("--jobs", type=int, default=6)
    ap.add_argument("--timeout", type=int, default=1200)
    ap.add_argument("--out", default="", help="write the per-binary JSON here")
    ap.add_argument("--report", default="", help="re-print a saved JSON and exit")
    ap.add_argument("--witness", nargs=2, metavar=("IMAGE", "GROUND_TRUTH"), action="append",
                    default=[], help="score a single image against a twin or JSON inventory")
    ap.add_argument("--tripwire", action="append", default=[],
                    help=f"tag substrings to print first (default: {list(DEFAULT_TRIPWIRES)})")
    ap.add_argument("--no-per-binary", action="store_true")
    a = ap.parse_args(argv)

    if a.report:
        d = json.load(open(a.report))
        report(d["rows"], d["arms"], tuple(a.tripwire) or DEFAULT_TRIPWIRES,
               not a.no_per_binary)
        return 0

    arms = [parse_arm(s) for s in a.arms]
    if a.witness:
        tg = [(os.path.basename(img), img, gt, "witness") for img, gt in a.witness]
    else:
        tg = [t for t in targets(a.arch) if a.only in t[0]]
        if a.limit:
            tg = tg[: a.limit]
    print(f"targets={len(tg)} arms={[n for n, _ in arms]} mode={a.mode} "
          f"bin={config.kuna_bin()}", flush=True)

    rows = []
    with ThreadPoolExecutor(max_workers=a.jobs) as ex:
        futs = [ex.submit(score_one, t, arms, a.mode, a.timeout, a.marginal) for t in tg]
        for i, f in enumerate(futs):
            r = f.result()
            rows.append(r)
            if r.get("error"):
                print(f"  ERR {r['tag']}: {r['error']}", flush=True)
            elif i % 10 == 0:
                print(f"  [{i}/{len(tg)}] {r['tag']}", flush=True)

    if a.out:
        json.dump(dict(arms=[n for n, _ in arms], rows=rows), open(a.out, "w"), indent=1)
    report(rows, [n for n, _ in arms], tuple(a.tripwire) or DEFAULT_TRIPWIRES,
           not a.no_per_binary and not a.witness)
    return 0


if __name__ == "__main__":
    sys.exit(main())
