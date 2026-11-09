"""Decompile + score orchestration, reports and run comparison."""

from __future__ import annotations

import json
import sys
import time
from pathlib import Path

from . import projects as P
from .decompilers import count_java, get_adapter, run_measured
from .index import load_index, selection, subset_jar
from .score import aggregate, render_markdown, score_project


def _sha(path: Path) -> str:
    from .fetch import sha256_file

    return sha256_file(path)


def _apply_javac(proj: P.Project, javac_jdk: int | None) -> None:
    if javac_jdk:
        proj.javac_jdk = javac_jdk


def run(decompiler: str, names: list[str] | None, full: bool, jobs: int, run_name: str | None,
        kuna_bin: str | None = None, score: bool = True, skip_bytes: bool = False, timeout: int = 7200,
        javac_jdk: int | None = None) -> Path:
    adapter = get_adapter(decompiler, kuna_bin)
    ok, why = adapter.available()
    stamp = time.strftime("%Y%m%d-%H%M%S")
    name = run_name or f"{decompiler}-{'full' if full else 'sample'}-{stamp}"
    rdir = P.RESULTS_DIR / name
    if rdir.exists():
        rdir = P.RESULTS_DIR / f"{name}-{stamp}"
    rdir.mkdir(parents=True)
    meta = {"run": rdir.name, "decompiler": decompiler, "version": adapter.version() if ok else "",
            "full": full, "jobs": jobs, "started": stamp, "projects": {}}
    if not ok:
        meta["status"] = "skipped"
        meta["reason"] = why
        (rdir / "run.json").write_text(json.dumps(meta, indent=1) + "\n")
        print(f"[run] {decompiler} unavailable: {why} -> skipped", file=sys.stderr)
        return rdir
    for proj in P.selected(names):
        _apply_javac(proj, javac_jdk)
        pdir = rdir / proj.name
        pdir.mkdir()
        index = load_index(proj, jobs=jobs)
        keys, tag = selection(proj, index, full)
        jar = subset_jar(proj, index, keys, tag)
        out = pdir / "decompiled"
        out.mkdir()
        info = {"project": proj.name, "decompiler": decompiler, "selection": tag, "input_jar": str(jar),
                "input_sha256": _sha(jar), "javac_jdk": proj.javac_jdk, "release_mode": proj.release_mode,
                "selected": keys}
        print(f"[run] {decompiler} {proj.name}: {len(keys)} methods ({tag}), input {jar.name}", file=sys.stderr, flush=True)
        if decompiler == "source":
            t0 = time.monotonic()
            adapter.materialize(proj, jar, out)
            m = {"exit_code": 0, "timed_out": False, "wall_seconds": round(time.monotonic() - t0, 2),
                 "peak_rss_mb": None, "cpu_seconds": None}
        else:
            cmd = adapter.command(jar, out, jobs)
            info["command"] = cmd
            m = run_measured(cmd, pdir / "decompile.log", timeout)
        info.update(m)
        info["java_files"] = count_java(out)
        if m["exit_code"] != 0 or info["java_files"] == 0:
            info["status"] = "skipped"
            info["reason"] = (f"decompiler exit {m['exit_code']}" + (" (timeout)" if m["timed_out"] else "")
                              + f", {info['java_files']} .java files; see {pdir / 'decompile.log'}")
            print(f"[run] {decompiler} {proj.name}: rejected input -> skipped ({info['reason']})", file=sys.stderr)
        else:
            info["status"] = "decompiled"
        (pdir / "run.json").write_text(json.dumps(info, indent=1) + "\n")
        meta["projects"][proj.name] = {k: v for k, v in info.items() if k != "selected"}
        (rdir / "run.json").write_text(json.dumps(meta, indent=1) + "\n")
        if score and info["status"] == "decompiled":
            score_project(proj, pdir, adapter.failure_regex, jobs, skip_bytes=skip_bytes)
            info["status"] = "scored"
            (pdir / "run.json").write_text(json.dumps(info, indent=1) + "\n")
            meta["projects"][proj.name]["status"] = "scored"
            (rdir / "run.json").write_text(json.dumps(meta, indent=1) + "\n")
    if score:
        report([str(rdir)], quiet=False)
    return rdir


def resolve_run(r: str) -> Path:
    p = Path(r)
    if not p.exists():
        p = P.RESULTS_DIR / r
    if not (p / "run.json").exists():
        raise SystemExit(f"not a javabench run: {r}")
    return p


def score_run(r: str, names: list[str] | None, jobs: int, kuna_bin: str | None = None, skip_bytes: bool = False,
              javac_jdk: int | None = None, reuse_bytes: bool = False) -> None:
    rdir = resolve_run(r)
    meta = json.loads((rdir / "run.json").read_text())
    adapter = get_adapter(meta["decompiler"], kuna_bin)
    for proj in P.selected(names):
        _apply_javac(proj, javac_jdk)
        pdir = rdir / proj.name
        if not (pdir / "run.json").exists():
            continue
        info = json.loads((pdir / "run.json").read_text())
        if info.get("status") == "skipped":
            print(f"[score] {proj.name}: skipped run ({info.get('reason')})", file=sys.stderr)
            continue
        score_project(proj, pdir, adapter.failure_regex, jobs, skip_bytes=skip_bytes, reuse_bytes=reuse_bytes)
    report([str(rdir)], quiet=False)


def _load_methods(pdir: Path) -> list[dict]:
    p = pdir / "methods.jsonl"
    if not p.exists():
        return []
    return [json.loads(line) for line in p.read_text().splitlines() if line.strip()]


def report(runs: list[str], quiet: bool = False) -> str:
    texts = []
    for r in runs:
        rdir = resolve_run(r)
        meta = json.loads((rdir / "run.json").read_text())
        boards = {}
        all_recs: list[dict] = []
        all_files: dict = {}
        n_tops = 0
        tot_wall = 0.0
        peak = 0.0
        for proj in P.PROJECTS:
            pdir = rdir / proj
            recs = _load_methods(pdir)
            if not recs:
                continue
            # re-aggregate from the per-method records (single source of truth)
            cj = pdir / "classes.json"
            files = json.loads(cj.read_text()) if cj.exists() else {}
            pinfo = json.loads((pdir / "run.json").read_text())
            skip = all(r["bytes"] is None for r in recs)
            b = aggregate(recs, files, pinfo, n_tops=len({r["class"].split("$")[0] for r in recs}), skip_bytes=skip)
            b.update({"project": proj, "decompiler": pinfo["decompiler"], "selection": pinfo["selection"]})
            (pdir / "scoreboard.json").write_text(json.dumps(b, indent=1, sort_keys=True) + "\n")
            (pdir / "scoreboard.md").write_text(render_markdown({proj: b}, title=f"{pinfo['decompiler']} / {proj}"))
            boards[proj] = b
            all_recs.extend(recs)
            for k, v in files.items():
                all_files[f"{proj}/{k}"] = v
            n_tops += (b.get("classes") or {}).get("denominator") or 0
            d = b.get("decompile") or {}
            tot_wall += d.get("wall_seconds") or 0.0
            peak = max(peak, d.get("peak_rss_mb") or 0.0)
        if not boards:
            why = meta.get("reason") or "; ".join(
                f"{p}: {v.get('status')} ({v.get('reason', '')})" for p, v in meta.get("projects", {}).items())
            texts.append(f"# {rdir.name}\n\n(no scored projects -- {why})\n")
            continue
        skip_bytes = all(b.get("bytes") is None for b in boards.values())
        overall = aggregate(all_recs, all_files, None, n_tops, skip_bytes=skip_bytes)
        overall["decompile"] = {"wall_seconds": round(tot_wall, 2), "peak_rss_mb": peak}
        overall["projects"] = list(boards)
        (rdir / "scoreboard.json").write_text(json.dumps({"overall": overall, "projects": boards}, indent=1, sort_keys=True) + "\n")
        md = render_markdown({**boards, "**overall**": overall}, title=f"javabench: {meta['decompiler']} ({rdir.name})")
        md += "\n" + _detail_md(boards, overall)
        (rdir / "scoreboard.md").write_text(md)
        texts.append(md)
    text = "\n".join(texts)
    if not quiet:
        print(text)
    return text


def _detail_md(boards: dict, overall: dict) -> str:
    lines = ["## Details", ""]
    for name, b in {**boards, "overall": overall}.items():
        c = b.get("classes") or {}
        g = b["ged"]
        lines.append(f"- **{name}**: GED denominator {g['denominator']} (scored {g['scored']}, unscored {g['unscored']}); "
                     f"mean GED over scored only {g['mean_scored_only']}; types over {b['types']['denominator']} methods / "
                     f"{b['types']['variables']} vars (method erasure-perfect {b['types']['method_erasure_perfect_pct']}%); "
                     f"classes {c.get('denominator')} (after stub salvage {c.get('after_salvage_pct')}%), "
                     f"method compiled {(b.get('bytes') or {}).get('method_compiled_pct')}%; "
                     f"classes failing per javac error code {dict(sorted((c.get('javac_error_classes') or {}).items(), key=lambda kv: -kv[1])[:6])}")
    lines.append("")
    return "\n".join(lines)


METRICS = {
    "ged": ("ged_perfect", lambda r: None if r["ged"] is None else r["ged"].get("value")),
    "types": ("types_perfect", lambda r: None if r["types"] is None else f"{r['types']['exact']}/{r['types']['total']}"),
    "bytes": ("bytes_perfect", lambda r: None if r["bytes"] is None else r["bytes"].get("similarity")),
}


def compare(run_a: str, run_b: str, names: list[str] | None, metrics: list[str], limit: int,
            json_out: str | None) -> dict:
    """Methods where A is perfect and B is not (and vice versa), per project and metric."""
    ra, rb = resolve_run(run_a), resolve_run(run_b)
    out: dict = {"a": ra.name, "b": rb.name, "projects": {}}
    lines = [f"# compare A={ra.name} vs B={rb.name}", ""]
    for proj in P.selected(names):
        A = {r["key"]: r for r in _load_methods(ra / proj.name)}
        B = {r["key"]: r for r in _load_methods(rb / proj.name)}
        common = sorted(set(A) & set(B))
        if not common:
            continue
        pj = {"common_methods": len(common), "only_in_a": len(set(A) - set(B)), "only_in_b": len(set(B) - set(A))}
        lines.append(f"## {proj.name} ({len(common)} shared methods)")
        for metric in metrics:
            flag, val = METRICS[metric]
            a_wins = [k for k in common if A[k][flag] is True and B[k][flag] is False]
            b_wins = [k for k in common if B[k][flag] is True and A[k][flag] is False]
            both = sum(1 for k in common if A[k][flag] is True and B[k][flag] is True)
            pj[metric] = {"both_perfect": both, "a_only_perfect": [], "b_only_perfect": []}
            lines.append(f"### {metric}: both perfect {both}, A-only {len(a_wins)}, B-only {len(b_wins)}")
            for label, wins, x, y in (("A perfect, B not", a_wins, A, B), ("B perfect, A not", b_wins, B, A)):
                wins = sorted(wins, key=lambda k: (-A[k]["code_len"], k))
                lines.append(f"#### {label} ({len(wins)})")
                for n_listed, k in enumerate(wins):
                    entry = {
                        "key": k, "class": A[k]["class"], "method": A[k]["name"] + A[k]["desc"],
                        "metric": metric, "a_score": val(A[k]), "b_score": val(B[k]),
                        "a_status": A[k]["status"], "b_status": B[k]["status"],
                        "a_file": _loc(A[k]), "b_file": _loc(B[k]),
                        "source": f"{A[k]['source_file']}:{A[k]['source_span'][0]}-{A[k]['source_span'][1]}",
                        "code_len": A[k]["code_len"],
                    }
                    if metric == "types":
                        entry["a_misses"] = (A[k]["types"] or {}).get("misses")
                        entry["b_misses"] = (B[k]["types"] or {}).get("misses")
                    if metric == "bytes":
                        entry["a_error"] = (A[k]["bytes"] or {}).get("error")
                        entry["b_error"] = (B[k]["bytes"] or {}).get("error")
                    pj[metric]["a_only_perfect" if x is A else "b_only_perfect"].append(entry)
                    if n_listed >= limit:
                        continue
                    lines.append(f"- `{k}` A={entry['a_score']} B={entry['b_score']} (A {entry['a_status']}, B {entry['b_status']})\n"
                                 f"  - A: {entry['a_file']}\n  - B: {entry['b_file']}\n  - src: {entry['source']}")
                if len(wins) > limit:
                    lines.append(f"- ... {len(wins) - limit} more (raise --limit or use --json)")
            lines.append("")
        out["projects"][proj.name] = pj
    text = "\n".join(lines)
    print(text)
    if json_out:
        Path(json_out).write_text(json.dumps(out, indent=1) + "\n")
    return out


def _loc(r: dict) -> str | None:
    if not r.get("decompiled_file"):
        return None
    sp = r.get("decompiled_span")
    return f"{r['decompiled_file']}:{sp[0]}-{sp[1]}" if sp else r["decompiled_file"]
