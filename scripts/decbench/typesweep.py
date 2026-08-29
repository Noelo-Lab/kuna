"""Corpus-scale, bidirectional A/B for one option flip on **type_match** --
the type analog of ``scripts.decbench.optsweep``.

``optsweep`` answers the same question for GED. This one answers it for the
metric kuna is weakest on: over the whole benchmark, which functions did the
change move to a PERFECT type recovery, and which did it move off?

Both arms are the SAME kuna binary over the SAME slices, one invocation per
(slice, arm) exactly as decbench's ``kuna_raw`` backend runs it::

    kuna decompile-all <stripped> --json --max-fn-seconds 120 [--option N V ...]

Each arm's JSON is rebuilt into a ``DecompilationResult`` whose function set and
names come from the tree's own checkpoint for kuna (the addresses decbench
narrowed to, relabeled to their DWARF names -- ``run_benchmark._relabel_to_dwarf``'s
convention) and whose ``variables`` come from the fresh run, then scored by
``TypeMatchMetric.compute_for_binary`` -- the SAME call ``reeval_typematch.py``
makes, so the per-binary offset calibration is identical to the published one.

Two harness controls are computed and printed on every run; a non-zero *control*
line means the numbers are wrong, not interesting:

* the baseline arm is compared function-by-function against the tree's published
  ``type_match`` verdict, and the agreement rate is reported (it is < 100% only
  by kuna-build drift since the tree was scored);
* functions whose two arms have byte-identical ``variables`` must score
  identically.

Usage (needs the decbench venv python)::

    ~/.virtualenvs/decbench/bin/python -m scripts.decbench.typesweep \
        --option framelayout on --opt O0 --workers 16 \
        --out docs/decbench/framelayout-o0

    ... --opt O0 --opt O2 --opt O2-noinline     # all three levels
    ... --project zlib --project coreutils      # restrict the sweep
    ... --baseline-only                         # just measure the current build
    ... --report-only                           # re-report from the cache
"""
from __future__ import annotations

import argparse
import collections
import json
import os
import pickle
import re
import signal
import subprocess
import sys
from concurrent.futures import ProcessPoolExecutor, as_completed
from pathlib import Path

from . import config

CACHE = Path(os.environ.get(
    "KUNA_TYPESWEEP_CACHE",
    str(Path.home() / ".cache" / "decbench" / "kuna-typesweep"),
))


def _imports():
    try:
        import decbench.decompilers  # noqa: F401  (register backends so pickles load)
        from decbench.metrics.type_match import TypeMatchMetric
        from decbench.models.decompilation import (
            DecompilationResult,
            DecompilerMetadata,
            FunctionDecompilation,
            VariableInfo,
        )
    except ImportError as e:
        sys.exit(
            f"cannot import decbench ({e}) -- run under the decbench venv python:\n"
            f"  {config.decbench_python()} -m scripts.decbench.typesweep ..."
        )
    return (TypeMatchMetric, DecompilationResult, DecompilerMetadata,
            FunctionDecompilation, VariableInfo)


def stripped_of(unstripped: Path) -> Path:
    return Path(str(unstripped).replace("/compiled/", "/stripped/"))


def run_kuna(stripped: Path, options: list[tuple[str, str]], timeout: int) -> dict:
    cmd = [config.kuna_bin(), "decompile-all", str(stripped), "--json",
           "--max-fn-seconds", "120"]
    for k, v in options:
        cmd += ["--option", k, v]
    p = subprocess.Popen(cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE,
                         text=True, start_new_session=True)
    try:
        out, err = p.communicate(timeout=timeout)
    except subprocess.TimeoutExpired:
        try:
            os.killpg(os.getpgid(p.pid), signal.SIGKILL)
        except Exception:  # noqa: BLE001
            p.kill()
        p.wait(timeout=15)
        raise
    if p.returncode != 0 and not (out or "").strip():
        raise RuntimeError(f"kuna exited {p.returncode}: {(err or '')[-400:]}")
    return json.loads(out)


def build_result(payload: dict, addr2name: dict[int, str], unstripped: Path, base: int):
    """A DecompilationResult over exactly the checkpoint's function set."""
    (_TM, DecompilationResult, DecompilerMetadata, FunctionDecompilation,
     VariableInfo) = _imports()
    byaddr: dict[int, dict] = {}
    for r in payload.get("functions") or []:
        try:
            byaddr[int(r.get("address") or 0)] = r
        except Exception:  # noqa: BLE001
            continue
    funcs = {}
    for addr, name in addr2name.items():
        rec = (byaddr.get(addr) or byaddr.get(addr & ~1) or byaddr.get(addr | 1)
               or byaddr.get(addr - base) or byaddr.get((addr - base) & ~1))
        if rec is None or not rec.get("code"):
            continue
        variables = []
        for v in rec.get("variables") or []:
            try:
                variables.append(VariableInfo(
                    name=str(v.get("name") or ""),
                    type=str(v.get("type") or ""),
                    stack_offset=v.get("stack_offset"),
                    size=v.get("size"),
                    kind=v.get("kind"),
                    arg_index=v.get("arg_index"),
                ))
            except Exception:  # noqa: BLE001
                continue
        code = str(rec.get("code"))
        own = str(rec.get("name") or "")
        if own and own != name:
            code = re.sub(r"\b" + re.escape(own) + r"\b", name, code)
        funcs[name] = FunctionDecompilation(
            name=name, address=int(rec.get("address") or addr),
            decompiled_code=code, line_count=code.count("\n") + 1,
            line_mappings=[], variables=variables,
        )
    return DecompilationResult(
        binary_path=unstripped, binary_name=unstripped.stem,
        decompiler=DecompilerMetadata(decompiler_name="kuna", decompiler_version="sweep"),
        functions=funcs,
    )


def slice_key(project: str, opt: str, stem: str) -> str:
    return f"{project}::{opt}::{stem}"


def install_decision_recorder(sink: list) -> None:
    """Replace ``TypeMatchMetric._match_structured`` with a verbatim copy that also
    appends one record per GROUND-TRUTH variable to ``sink``.

    The metric reports only tp/fp/fn counts, which cannot say WHICH decision cost a
    function its perfect score.  The copy below is the upstream body with a
    ``rec(...)`` call added at each verdict site and nothing else changed, so the
    counts it returns are identical.  Needs ``DECBENCH_NO_CACHE=1``: the metric's
    content-addressed cache otherwise serves a stored value and never runs this.
    """
    from decbench.metrics.type_match import (
        TypeMatchMetric, normalize_type, _uncommitted_size, _effective_offset,
        _SIZE_SCALARS, _calibrate_shift,
    )

    def _match_structured(self, decompiled, ground_truth_vars, gt_stack_vars,
                          decomp_stack_vars, calibration_shift=None):
        gt_offsets = []
        for gv in ground_truth_vars:
            gt_offsets.extend(gv.get("rbp_offset", []))
        var_offsets = [_effective_offset(v) for v in decompiled.variables]
        decomp_offsets = [o for o in var_offsets if o is not None]
        gt_off_set = set(gt_offsets)

        def _aligned(kk):
            if kk is None or not decomp_offsets:
                return 0
            return len({d + kk for d in decomp_offsets} & gt_off_set)

        shift = calibration_shift if calibration_shift is not None else 0
        fs = _calibrate_shift(gt_offsets, decomp_offsets)
        if fs is not None and _aligned(shift) == 0 and _aligned(fs) > 0:
            shift = fs
        k = shift if shift is not None else 0
        var_types = [normalize_type(v.type) for v in decompiled.variables]
        var_unc = [_uncommitted_size(v) for v in decompiled.variables]
        by_arg_index, by_off, by_name = {}, {}, {}
        for i, v in enumerate(decompiled.variables):
            if v.arg_index is not None and v.arg_index not in by_arg_index:
                by_arg_index[v.arg_index] = i
            if var_offsets[i] is not None:
                by_off.setdefault(var_offsets[i] + k, []).append(i)
            if v.name:
                by_name.setdefault(v.name, []).append(i)

        def _matches(gt_forms, i):
            if gt_forms & var_types[i]:
                return True
            sz = var_unc[i]
            return sz is not None and bool(_SIZE_SCALARS.get(sz, set()) & gt_forms)

        used = set()

        def claim(cands, gt_types):
            avail = [i for i in cands if i not in used]
            if not avail:
                return None, None
            hit = next((i for i in avail if _matches(gt_types, i)), None)
            if hit is not None:
                used.add(hit)
                return True, hit
            used.add(avail[0])
            return False, avail[0]

        n = len(ground_truth_vars)
        verdicts = [None] * n
        decided = [False] * n

        def rec(gv, ok, di, pas):
            v = decompiled.variables[di] if di is not None else None
            sink.append({
                "fn": decompiled.name, "ok": ok, "pass": pas,
                "kuna_type": (v.type if v else None), "kuna_size": (v.size if v else None),
                "kuna_name": (v.name if v else None),
                "gt_type": (gv.get("type") or [None])[0], "gt_size": gv.get("size"),
                "gt_name": gv.get("name"), "gt_is_arg": bool(gv.get("is_arg")),
                "gt_has_off": bool(gv.get("rbp_offset")), "gt_vars": n,
            })

        for gi, gv in enumerate(ground_truth_vars):
            ai = gv.get("arg_index")
            if not gv.get("is_arg") or ai is None:
                continue
            di = by_arg_index.get(ai)
            if di is None or di in used:
                continue
            used.add(di)
            decided[gi] = True
            verdicts[gi] = _matches(set(gv.get("type", [])), di)
            rec(gv, verdicts[gi], di, "arg")
        for gi, gv in enumerate(ground_truth_vars):
            if decided[gi]:
                continue
            c = []
            for off in gv.get("rbp_offset", []):
                c.extend(by_off.get(off, []))
            if not c:
                continue
            v, di = claim(c, set(gv.get("type", [])))
            if v is not None:
                decided[gi] = True
                verdicts[gi] = v
                rec(gv, v, di, "offset")
        for gi, gv in enumerate(ground_truth_vars):
            if decided[gi]:
                continue
            gn = gv.get("name", "")
            if not gn:
                continue
            v, di = claim(by_name.get(gn, []), set(gv.get("type", [])))
            if v is not None:
                decided[gi] = True
                verdicts[gi] = v
                rec(gv, v, di, "name")
        for gi, gv in enumerate(ground_truth_vars):
            if not decided[gi]:
                rec(gv, None, None, "miss")
        tp = sum(1 for d, v in zip(decided, verdicts) if d and v)
        fp = sum(1 for d, v in zip(decided, verdicts) if d and not v)
        fn = sum(1 for d in decided if not d)
        return self._build_result(tp, fp, fn, ground_truth_vars, len(decompiled.variables),
                                  "structured", calibration_shift, gt_stack_vars,
                                  decomp_stack_vars)

    TypeMatchMetric._match_structured = _match_structured


def score_slice(task):
    """Worker: run both arms for one (project, opt, binary) and score each."""
    (project, opt, stem, unstripped, addr2name, options, timeout, arms,
     dump_decisions) = task
    (TypeMatchMetric, *_rest) = _imports()
    from decbench.decompilers.raw import common
    unstripped = Path(unstripped)
    stripped = stripped_of(unstripped)
    if not stripped.exists():
        return slice_key(project, opt, stem), {"error": f"no stripped copy: {stripped}"}
    try:
        base = common.elf_min_vaddr(unstripped)
    except Exception:  # noqa: BLE001
        base = 0
    sink: list = []
    if dump_decisions:
        install_decision_recorder(sink)
    out = {}
    for arm in arms:
        opts = [] if arm == "base" else options
        try:
            payload = run_kuna(stripped, opts, timeout)
        except Exception as e:  # noqa: BLE001
            out[arm] = {"error": str(e)[:200]}
            continue
        res = build_result(payload, addr2name, unstripped, base)
        try:
            mr = TypeMatchMetric().compute_for_binary(res)
        except Exception as e:  # noqa: BLE001
            out[arm] = {"error": "score: " + str(e)[:200]}
            continue
        if dump_decisions:
            out.setdefault("decisions", {})[arm] = list(sink)
            sink.clear()
        out[arm] = {
            "values": {fn: float(mv.value) for fn, mv in mr.function_results.items()},
            "meta": {fn: {k: mv.metadata.get(k) for k in ("tp", "fp", "fn", "gt_vars",
                                                          "decomp_vars", "matched_by")}
                     for fn, mv in mr.function_results.items()},
            "nvars": {fn: len(f.variables) for fn, f in res.functions.items()},
        }
    return slice_key(project, opt, stem), out


def collect_slices(results_root: Path, projects, opts):
    """(project, opt, stem, unstripped, addr2name) for every kuna slice in the tree."""
    import decbench.decompilers  # noqa: F401
    ckpt = results_root / "checkpoints"
    out = []
    repo = config.decbench_repo()
    for pk in sorted(ckpt.glob("*.pkl")):
        project = pk.stem
        if projects and project not in projects:
            continue
        try:
            data = pickle.loads(pk.read_bytes())
        except Exception:  # noqa: BLE001
            continue
        for opt, bins in (data.get("decompile") or {}).items():
            optn = getattr(opt, "value", str(opt))
            if opts and optn not in opts:
                continue
            for stem, decs in bins.items():
                r = decs.get("kuna")
                if r is None or not getattr(r, "functions", None):
                    continue
                bp = Path(r.binary_path)
                if not bp.is_absolute():
                    bp = repo / bp
                if not bp.exists():
                    continue
                a2n = {}
                for f in r.functions.values():
                    try:
                        a2n[int(f.address)] = f.name
                    except Exception:  # noqa: BLE001
                        continue
                if a2n:
                    out.append((project, optn, stem, str(bp), a2n))
    return out


def published(results_root: Path) -> dict:
    """kuna's published per-function type_match, keyed project::opt::stem::fn."""
    doc = json.loads((results_root / "function_results.json").read_text())
    out = {}
    for g in doc.get("groups", []):
        stem = Path(g["binary"]).stem
        for f in g.get("functions", []):
            v = (f.get("values") or {}).get("kuna") or {}
            if v.get("type_match") is not None:
                out[f"{g['project']}::{g['opt_level']}::{stem}::{f['function']}"] = \
                    float(v["type_match"])
    return out


def main() -> None:
    ap = argparse.ArgumentParser(description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--option", nargs=2, action="append", metavar=("NAME", "VALUE"),
                    default=[], help="option flip for the test arm (repeatable)")
    ap.add_argument("--opt", action="append", default=[], help="optimisation level filter")
    ap.add_argument("--project", action="append", default=[], help="project filter")
    ap.add_argument("--workers", type=int, default=12)
    ap.add_argument("--timeout", type=int, default=1800)
    ap.add_argument("--out", type=Path, required=True)
    ap.add_argument("--baseline-only", action="store_true")
    ap.add_argument("--report-only", action="store_true")
    ap.add_argument("--limit", type=int, default=0, help="cap the number of slices (debug)")
    ap.add_argument("--dump-decisions", action="store_true",
                    help="also record one row per ground-truth variable decision "
                         "(needs DECBENCH_NO_CACHE=1); written into rows.json")
    args = ap.parse_args()

    _imports()
    root = config.results_root()
    args.out.mkdir(parents=True, exist_ok=True)
    tag = "base" if args.baseline_only else "+".join(f"{k}={v}" for k, v in args.option)
    cachef = args.out / "rows.json"

    rows = {}
    if cachef.exists():
        rows = json.loads(cachef.read_text())
    if not args.report_only:
        slices = collect_slices(root, set(args.project), set(args.opt))
        if args.limit:
            slices = slices[:args.limit]
        arms = ["base"] if args.baseline_only else ["base", "test"]
        todo = [(p, o, s, b, a, args.option, args.timeout, arms, args.dump_decisions)
                for (p, o, s, b, a) in slices
                if slice_key(p, o, s) not in rows]
        print(f"slices: {len(slices)} total, {len(todo)} to run, arms={arms}", flush=True)
        done = 0
        with ProcessPoolExecutor(max_workers=args.workers) as ex:
            futs = {ex.submit(score_slice, t): t for t in todo}
            for fut in as_completed(futs):
                try:
                    k, v = fut.result()
                except Exception as e:  # noqa: BLE001
                    done += 1
                    print(f"[{done}/{len(todo)}] WORKER-ERR {e}", flush=True)
                    continue
                rows[k] = v
                done += 1
                if done % 10 == 0 or done == len(todo):
                    cachef.write_text(json.dumps(rows))
                print(f"[{done}/{len(todo)}] {k}", flush=True)
        cachef.write_text(json.dumps(rows))

    pub = published(root)
    agree = dis = 0
    b_perf = t_perf = 0
    b_sum = t_sum = 0.0
    n = 0
    onto, off, better, worse = [], [], [], []
    for k, v in rows.items():
        b = v.get("base") or {}
        t = v.get("test") or {}
        for fn, bv in (b.get("values") or {}).items():
            n += 1
            b_sum += bv
            if bv == 1.0:
                b_perf += 1
            pv = pub.get(f"{k}::{fn}")
            if pv is not None:
                if abs(pv - bv) < 1e-9:
                    agree += 1
                else:
                    dis += 1
            if not t:
                continue
            tv = (t.get("values") or {}).get(fn)
            if tv is None:
                continue
            t_sum += tv
            if tv == 1.0:
                t_perf += 1
            if tv == 1.0 and bv != 1.0:
                onto.append((k, fn, bv, tv))
            elif bv == 1.0 and tv != 1.0:
                off.append((k, fn, bv, tv))
            elif tv > bv:
                better.append((k, fn, bv, tv))
            elif tv < bv:
                worse.append((k, fn, bv, tv))

    lines = [f"# typesweep {tag}", "",
             f"slices scored: {len(rows)}", f"functions scored: {n}", "",
             "## control", "",
             f"- baseline vs published type_match: {agree} agree / {dis} differ "
             f"({100.0*agree/max(1,agree+dis):.2f}% agreement)", ""]
    if rows and any(v.get("test") for v in rows.values()):
        lines += ["## result", "",
                  f"- type_match PERFECT: base {b_perf} -> test {t_perf} "
                  f"({t_perf-b_perf:+d})",
                  f"- aggregate type_match: base {b_sum:.2f} -> test {t_sum:.2f} "
                  f"({t_sum-b_sum:+.2f})",
                  f"- moved ONTO perfect: {len(onto)}",
                  f"- moved OFF perfect: {len(off)}",
                  f"- improved (not perfect): {len(better)}",
                  f"- worsened (not perfect): {len(worse)}", ""]
    else:
        lines += ["## baseline", "",
                  f"- type_match PERFECT: {b_perf} / {n}",
                  f"- aggregate type_match: {b_sum:.2f} (mean {b_sum/max(1,n):.4f})", ""]
    (args.out / "report.md").write_text("\n".join(lines))
    with open(args.out / "moved.csv", "w") as fh:
        fh.write("direction,slice,function,base,test\n")
        for name, xs in (("onto", onto), ("off", off), ("better", better), ("worse", worse)):
            for k, fn, bv, tv in xs:
                fh.write(f"{name},{k},{fn},{bv},{tv}\n")
    print("\n".join(lines))


if __name__ == "__main__":
    main()
