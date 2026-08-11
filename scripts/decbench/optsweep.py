"""Corpus-scale, **bidirectional** A/B for one option flip -- the campaign's ablation instrument.

`rescore` answers "what does this change do to one mined case". This answers the
question standing requirement 8 actually asks: *over the whole benchmark, which
functions did the change move to perfect, and which did it move off perfect?* A
feature that flips 400 losses into wins and 400 wins into losses moves the
scoreboard by zero, and nothing else in the pipeline shows that.

Both arms are the SAME kuna binary over the SAME slices, one invocation per
(slice, arm) exactly as decbench's `kuna_raw` backend runs it::

    kuna decompile-all <stripped> --json --max-fn-seconds 120 [--option N V ...]

Each arm's JSON is rebuilt into a decbench artifact -- for every
``// Function: <name> @ <addr>`` marker of the stored run artifact, the fresh
record at that address with its own ``sub_<addr>`` placeholder renamed to the
marker name, which is `run_benchmark._relabel_to_dwarf`'s convention -- and then
scored by replaying `reeval_ged.eval_one` verbatim: `extract_cfgs_from_source`
per artifact, `resolved_source_for_binary` against the tree's own
``ged_src/<project>.pkl``, `GEDMetric`, non-finite dropped.

Three harness controls are computed and printed on every run, and a
non-zero *control* line means the numbers are wrong, not interesting:

* functions whose two arms are byte-identical must score identically;
* functions whose bodies differ only by a ``// warning`` slug must score
  identically (comments carry no CFG);
* the baseline arm is compared function-by-function against the tree's published
  ``ged_new.json`` verdict, and the agreement rate is reported (it is < 100% only
  by kuna-build drift since the tree was scored).

Usage (scoring needs the decbench venv python)::

    ~/.virtualenvs/decbench/bin/python -m scripts.decbench.optsweep \\
        --option returndup off --opt O0 --workers 16 --joern-workers 8 \\
        --out docs/decbench/returndup-o0

    ... --opt O0 --opt O2 --opt O2-noinline      # all three levels
    ... --project zlib --project coreutils       # restrict the sweep
    ... --report-only                            # re-report from the cache

Outputs, under ``--out``: ``report.md`` (the tables), ``moved.csv`` (every
function that changed GED, by name, both directions), ``changed.txt`` (the
changed-function set for a requirement-8 diff audit) and ``rows.json`` (the
per-function table, for ad-hoc partitioning).
"""
from __future__ import annotations

import argparse
import collections
import csv
import json
import math
import os
import pickle
import re
import signal
import struct
import subprocess
import sys
import time
from concurrent.futures import ProcessPoolExecutor
from pathlib import Path

from . import config

MARKER = re.compile(r"^// Function: (\S+) @ (0x[0-9a-fA-F]+)\s*$", re.M)
COMMENT = re.compile(r"//[^\n]*|/\*.*?\*/", re.S)
ARMS = ("base", "var")
DEFAULT_CACHE = Path.home() / ".cache" / "kuna-optsweep"

EM = {0x03: "x86", 0x3E: "x86-64", 0x28: "arm", 0xB7: "aarch64", 0x08: "mips",
      0x14: "ppc", 0x15: "ppc64", 0xF3: "riscv", 0x02: "sparc", 0x2B: "sparcv9"}


def arch_of(path: Path) -> str:
    """ELF e_machine / PE machine, without a dependency."""
    try:
        head = path.open("rb").read(0x400)
    except OSError:
        return "?"
    if head[:4] == b"\x7fELF":
        return EM.get(struct.unpack_from("<H", head, 18)[0], "elf?")
    if head[:2] == b"MZ":
        off = struct.unpack_from("<I", head, 0x3C)[0]
        if off + 6 < len(head) and head[off:off + 4] == b"PE\0\0":
            return {0x14C: "pe-x86", 0x8664: "pe-x86-64"}.get(
                struct.unpack_from("<H", head, off + 4)[0], "pe?")
    return "?"


# ---------------------------------------------------------------- plan


def build_plan(opts: list[str], projects: set[str] | None) -> list[dict]:
    """One entry per (opt, project, binary) kuna slice the tree has scores for."""
    root = config.results_root()
    ged = json.loads((root / "ged_new.json").read_text())
    scored: dict[tuple[str, str, str], set[str]] = collections.defaultdict(set)
    for key, _v in ged.items():
        p = key.split("::")
        if len(p) < 5 or p[0] not in opts or p[3] != "kuna":
            continue
        scored[(p[0], p[1], p[2])].add("::".join(p[4:]))
    plan = []
    for (opt, proj, stem), fns in sorted(scored.items()):
        if projects and proj not in projects:
            continue
        art = root / opt / proj / "decompiled" / f"kuna_{stem}.c"
        cands = [p for p in (root / opt / proj / "stripped").iterdir()
                 if p.is_file() and p.stem == stem]
        if not art.exists() or len(cands) != 1:
            print(f"[plan] skip {opt}/{proj}/{stem}: artifact={art.exists()} "
                  f"stripped={len(cands)}", file=sys.stderr)
            continue
        plan.append({
            "opt": opt, "project": proj, "stem": stem, "binary": str(cands[0]),
            "arch": arch_of(cands[0]), "artifact": str(art),
            "markers": MARKER.findall(art.read_text(errors="replace")),
            "scored": sorted(fns),
        })
    return plan


# ---------------------------------------------------------------- decompile


def _decompile(job) -> dict:
    slice_, arm, options, cache, kuna_bin, max_fn, timeout = job
    out = cache / "artifacts" / slice_["opt"] / arm / slice_["project"]
    dst = out / f"kuna_{slice_['stem']}.c"
    meta = out / f"kuna_{slice_['stem']}.meta.json"
    if meta.exists():
        return json.loads(meta.read_text())
    out.mkdir(parents=True, exist_ok=True)
    cmd = [kuna_bin, "decompile-all", slice_["binary"], "--json",
           "--max-fn-seconds", str(max_fn)]
    if arm == "var":
        for name, value in options:
            cmd += ["--option", name, value]
    t0 = time.time()
    proc = subprocess.Popen(cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE,
                            text=True, start_new_session=True)
    try:
        stdout, _err = proc.communicate(timeout=timeout)
        status = "ok" if (proc.returncode == 0 or stdout.strip()) else f"exit{proc.returncode}"
    except subprocess.TimeoutExpired:
        try:
            os.killpg(os.getpgid(proc.pid), signal.SIGKILL)
        except OSError:
            proc.kill()
        proc.wait()
        stdout, status = "", "timeout"
    recs: dict[int, dict] = {}
    if status == "ok":
        try:
            for r in json.loads(stdout).get("functions") or []:
                if r.get("address") is not None:
                    recs[int(r["address"])] = r
        except (ValueError, TypeError) as e:
            status = f"badjson:{e}"
    blocks, matched, missing, nocode = [], 0, 0, 0
    for fn, ahex in slice_["markers"]:
        rec = recs.get(int(ahex, 16))
        if rec is None:
            missing += 1
            continue
        code = rec.get("code")
        if not code:
            nocode += 1
            continue
        name = str(rec.get("name") or "")
        if name and name != fn:
            code = re.sub(r"\b" + re.escape(name) + r"\b", fn, code)
        blocks.append(f"// Function: {fn} @ {ahex}\n{code}\n")
        matched += 1
    dst.write_text("\n".join(blocks) + "\n")
    m = {"opt": slice_["opt"], "project": slice_["project"], "stem": slice_["stem"],
         "arm": arm, "status": status, "wall": time.time() - t0,
         "markers": len(slice_["markers"]), "matched": matched,
         "missing": missing, "no_code": nocode}
    meta.write_text(json.dumps(m))
    return m


def decompile_all(plan, options, cache, workers, kuna_bin, max_fn, timeout) -> list[dict]:
    jobs = [(s, arm, options, cache, kuna_bin, max_fn, timeout)
            for arm in ARMS for s in sorted(plan, key=lambda s: -len(s["markers"]))]
    metas, done = [], 0
    with ProcessPoolExecutor(max_workers=workers) as ex:
        for m in ex.map(_decompile, jobs):
            done += 1
            metas.append(m)
            if m["status"] != "ok" or m["missing"] or done % 25 == 0:
                print(f"[decompile] {done}/{len(jobs)} {m['opt']}/{m['project']}/{m['stem']}"
                      f" {m['arm']} {m['status']} matched={m['matched']}/{m['markers']}",
                      flush=True)
    return metas


# ---------------------------------------------------------------- score

_SRC: dict[str, tuple[dict, dict]] = {}


def _load_src(pkl: str):
    if pkl not in _SRC:
        from decbench.utils.cfg import best_source_by_name
        per_stem = pickle.loads(Path(pkl).read_bytes())
        _SRC[pkl] = (per_stem, best_source_by_name(per_stem))
    return _SRC[pkl]


def _score(job):
    """Replays reeval_ged.eval_one for both arms of one slice."""
    opt, proj, stem, cache = job
    from decbench.metrics.ged import GEDMetric
    from decbench.utils.cfg import extract_cfgs_from_source, resolved_source_for_binary
    dst = Path(cache) / "scores" / opt / f"{proj}__{stem}.json"
    if dst.exists():
        return opt, proj, stem, -1, 0.0
    t0 = time.time()
    per_stem, best = _load_src(str(config.results_root() / "ged_src" / f"{proj}.pkl"))
    src = resolved_source_for_binary(stem, per_stem, best)
    metric = GEDMetric()
    out: dict[str, dict] = {}
    for arm in ARMS:
        c = Path(cache) / "artifacts" / opt / arm / proj / f"kuna_{stem}.c"
        try:
            dec = extract_cfgs_from_source(c, sanitize_decompiled=True) or {}
        except Exception:  # noqa: BLE001 -- a Joern failure is a dropped slice, not a crash
            dec = {}
        # MARKER captures (name, addr) -- keep only the names.
        markers = {m[0] for m in MARKER.findall(c.read_text(errors="replace"))}
        for fn, dcfg in dec.items():
            scfg = src.get(fn)
            if fn not in markers or scfg is None:
                continue
            try:
                mv = metric.compute_for_function(None, source_cfg=scfg, decompiled_cfg=dcfg)
            except Exception:  # noqa: BLE001
                continue
            if not math.isfinite(mv.value):
                continue
            rec = out.setdefault(fn, {})
            rec[arm] = float(mv.value)
            rec["src_nodes"] = int((mv.metadata or {}).get("source_nodes") or 0)
            rec["approx"] = bool(rec.get("approx") or (mv.metadata or {}).get("approximated"))
    dst.parent.mkdir(parents=True, exist_ok=True)
    dst.write_text(json.dumps(out))
    return opt, proj, stem, len(out), time.time() - t0


def score_all(plan, cache, workers):
    import multiprocessing as mp
    jobs = [(s["opt"], s["project"], s["stem"], str(cache))
            for s in sorted(plan, key=lambda s: -len(s["markers"]))]
    ctx = mp.get_context("spawn")
    done = 0
    # maxtasksperchild bounds the per-worker source-CFG cache: without recycling a
    # worker accumulates every project pickle it touches (betaflight alone is 200 MB).
    with ctx.Pool(processes=workers, maxtasksperchild=4) as pool:
        for opt, proj, stem, n, dt in pool.imap_unordered(_score, jobs):
            done += 1
            if done % 20 == 0 or n == 0:
                print(f"[score] {done}/{len(jobs)} {opt}/{proj}/{stem} {n} fns {dt:.0f}s",
                      flush=True)


# ---------------------------------------------------------------- report


def blocks_of(path: Path) -> dict[str, str]:
    text = path.read_text(errors="replace")
    hits = list(MARKER.finditer(text))
    return {m.group(1): text[m.end():(hits[i + 1].start() if i + 1 < len(hits) else len(text))]
            for i, m in enumerate(hits)}


def collect_rows(plan, cache) -> list[dict]:
    rows = []
    for s in plan:
        opt, proj, stem = s["opt"], s["project"], s["stem"]
        sc = Path(cache) / "scores" / opt / f"{proj}__{stem}.json"
        if not sc.exists():
            continue
        scores = json.loads(sc.read_text())
        arms = {a: blocks_of(Path(cache) / "artifacts" / opt / a / proj / f"kuna_{stem}.c")
                for a in ARMS}
        for fn, v in scores.items():
            if "base" not in v or "var" not in v:
                continue
            b, w = arms["base"].get(fn, ""), arms["var"].get(fn, "")
            rows.append({
                "opt": opt, "project": proj, "binary": stem, "arch": s["arch"], "function": fn,
                "base": v["base"], "var": v["var"], "src_nodes": v.get("src_nodes", 0),
                "approx": v.get("approx", False),
                "text_changed": b != w,
                "body_changed": COMMENT.sub("", b) != COMMENT.sub("", w),
            })
    return rows


def _bucket(rows, keyfn):
    out = collections.defaultdict(lambda: [0, 0.0, 0, 0])
    for r in rows:
        c = out[keyfn(r)]
        c[0] += 1
        c[1] += r["var"] - r["base"]
        c[2] += r["base"] != 0 and r["var"] == 0
        c[3] += r["base"] == 0 and r["var"] != 0
    return out


def report(rows, plan, options, cache, out_dir: Path) -> str:
    flip = ", ".join(f"{n} {v}" for n, v in options)
    L = [f"# optsweep: `--option {flip}`", ""]
    L.append(f"kuna `{_git_head()}` -- baseline arm = shipped defaults, variant arm = the flip. "
             f"{len(plan)} slices, {len(rows)} functions scored in both arms.")
    L.append("")

    # controls
    quiet = [r for r in rows if not r["body_changed"]]
    quiet_bad = [r for r in quiet if r["base"] != r["var"]]
    banner = [r for r in rows if r["text_changed"] and not r["body_changed"]]
    banner_bad = [r for r in banner if r["base"] != r["var"]]
    pub = _published_agreement(rows)
    L += ["## Controls", "",
          f"- unchanged bodies: **{len(quiet)}**; scoring differently: **{len(quiet_bad)}** "
          f"({'OK' if not quiet_bad else 'HARNESS BROKEN -- STOP'})",
          f"- slug-only diffs: **{len(banner)}**; scoring differently: **{len(banner_bad)}** "
          f"({'OK' if not banner_bad else 'HARNESS BROKEN -- STOP'})",
          f"- baseline arm agrees with the tree's published verdict on "
          f"**{pub[0]}/{pub[1]}** ({100 * pub[0] / max(1, pub[1]):.1f}%)", ""]

    # headline, per opt level
    L += ["## Headline", "",
          "| scope | n | changed | GED base | GED var | perfect base | perfect var | dPerfect | dGED |",
          "|---|---:|---:|---:|---:|---:|---:|---:|---:|"]
    for scope in sorted({r["opt"] for r in rows}) + ["ALL"]:
        rs = rows if scope == "ALL" else [r for r in rows if r["opt"] == scope]
        gb, gv = sum(r["base"] for r in rs), sum(r["var"] for r in rs)
        pb = sum(1 for r in rs if r["base"] == 0)
        pv = sum(1 for r in rs if r["var"] == 0)
        L.append(f"| {scope} | {len(rs)} | {sum(1 for r in rs if r['body_changed'])} | "
                 f"{gb:.0f} | {gv:.0f} | {pb} | {pv} | {pv - pb:+d} | {gv - gb:+.0f} |")
    L.append("")

    # bidirectional movement
    to_perfect = [r for r in rows if r["base"] != 0 and r["var"] == 0]
    off_perfect = [r for r in rows if r["base"] == 0 and r["var"] != 0]
    improved = [r for r in rows if r["var"] < r["base"]]
    worsened = [r for r in rows if r["var"] > r["base"]]
    z = (len(to_perfect) - len(off_perfect)) / math.sqrt(max(1, len(to_perfect) + len(off_perfect)))
    L += ["## Both directions", "",
          f"- **to perfect: {len(to_perfect)}**, **off perfect: {len(off_perfect)}** "
          f"(ratio {len(to_perfect) / max(1, len(off_perfect)):.2f} : 1, McNemar z = {z:+.2f})",
          f"- improved {len(improved)} / worsened {len(worsened)}", ""]

    for title, keyfn in (("per opt level", lambda r: r["opt"]),
                         ("per architecture", lambda r: r["arch"]),
                         ("per project", lambda r: r["project"])):
        L += [f"### {title}", "",
              "| cell | n | dGED | to perfect | off perfect | net |", "|---|---:|---:|---:|---:|---:|"]
        b = _bucket(rows, keyfn)
        for k in sorted(b, key=lambda k: b[k][2] - b[k][3]):
            n, d, g, l = b[k]
            L.append(f"| {k} | {n} | {d:+.0f} | {g} | {l} | {g - l:+d} |")
        L.append("")

    L += ["### Biggest single movements", "",
          "| direction | function | base | var |", "|---|---|---:|---:|"]
    for r in sorted(to_perfect, key=lambda r: -r["base"])[:10]:
        L.append(f"| to perfect | {r['opt']} {r['project']}/{r['binary']}::{r['function']} "
                 f"| {r['base']:.0f} | 0 |")
    for r in sorted(off_perfect, key=lambda r: -r["var"])[:10]:
        L.append(f"| off perfect | {r['opt']} {r['project']}/{r['binary']}::{r['function']} "
                 f"| 0 | {r['var']:.0f} |")
    L.append("")

    out_dir.mkdir(parents=True, exist_ok=True)
    with (out_dir / "moved.csv").open("w", newline="") as fh:
        w = csv.writer(fh)
        w.writerow(["direction", "opt", "project", "binary", "function", "arch",
                    "ged_base", "ged_var", "src_nodes", "approximated"])
        for r in sorted(rows, key=lambda r: (r["opt"], r["project"], r["function"])):
            if r["base"] == r["var"]:
                continue
            d = ("to_perfect" if r["var"] == 0 else "off_perfect" if r["base"] == 0
                 else "improved" if r["var"] < r["base"] else "worsened")
            w.writerow([d, r["opt"], r["project"], r["binary"], r["function"], r["arch"],
                        r["base"], r["var"], r["src_nodes"], r["approx"]])
    (out_dir / "changed.txt").write_text("\n".join(
        f"{r['opt']}::{r['project']}::{r['binary']}::{r['function']}"
        for r in rows if r["body_changed"]) + "\n")
    (out_dir / "rows.json").write_text(json.dumps(rows))
    text = "\n".join(L)
    (out_dir / "report.md").write_text(text)
    return text


def _published_agreement(rows) -> tuple[int, int]:
    ged = json.loads((config.results_root() / "ged_new.json").read_text())
    ok = tot = 0
    for r in rows:
        v = ged.get(f"{r['opt']}::{r['project']}::{r['binary']}::kuna::{r['function']}")
        if v is None:
            continue
        tot += 1
        ok += bool(v.get("perfect")) == (r["base"] == 0)
    return ok, tot


def _git_head() -> str:
    return subprocess.run(["git", "-C", str(config.repo_root()), "rev-parse", "--short", "HEAD"],
                          capture_output=True, text=True).stdout.strip() or "?"


# ---------------------------------------------------------------- main


def main(argv=None) -> None:
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument("--option", nargs=2, action="append", required=True,
                    metavar=("NAME", "VALUE"), help="the flip under test (variant arm)")
    ap.add_argument("--opt", action="append", default=None,
                    help="optimisation level (repeatable; default all three)")
    ap.add_argument("--project", action="append", default=None)
    ap.add_argument("--workers", type=int, default=12, help="parallel kuna decompiles")
    ap.add_argument("--joern-workers", type=int, default=8,
                    help="parallel Joern parses -- JVM-heavy, keep near 8")
    ap.add_argument("--max-fn-seconds", type=int, default=120)
    ap.add_argument("--binary-timeout", type=int, default=7200)
    ap.add_argument("--cache", type=Path, default=None)
    ap.add_argument("--out", type=Path, required=True)
    ap.add_argument("--report-only", action="store_true")
    args = ap.parse_args(argv)

    opts = args.opt or ["O0", "O2", "O2-noinline"]
    slug = "-".join(f"{n}_{v}" for n, v in args.option) + "--" + "-".join(opts)
    cache = args.cache or (DEFAULT_CACHE / slug)
    cache.mkdir(parents=True, exist_ok=True)
    plan = build_plan(opts, set(args.project) if args.project else None)
    print(f"[optsweep] {len(plan)} slices, "
          f"{sum(len(s['markers']) for s in plan)} markers, cache {cache}", flush=True)

    if not args.report_only:
        metas = decompile_all(plan, args.option, cache, args.workers,
                              config.kuna_bin(), args.max_fn_seconds, args.binary_timeout)
        bad = [m for m in metas if m["status"] != "ok"]
        print(f"[decompile] done; {len(bad)} non-ok, "
              f"{sum(m['missing'] for m in metas)} markers unmatched", flush=True)
        score_all(plan, cache, args.joern_workers)

    rows = collect_rows(plan, cache)
    print(report(rows, plan, args.option, cache, args.out))
    print(f"[optsweep] wrote {args.out}/report.md, moved.csv, changed.txt, rows.json",
          file=sys.stderr)


if __name__ == "__main__":
    os.environ.setdefault("PYTHONWARNINGS", "ignore")
    main()
