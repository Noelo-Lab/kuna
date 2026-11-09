"""Score one decompiler run against the ground-truth index.

Outputs, per ``<run>/<project>/``: ``methods.jsonl`` (one record per scored
method), ``classes.json`` (per decompiled file compile results) and
``scoreboard.json`` / ``scoreboard.md``; the run-level ``scoreboard.{json,md}``
pools all projects (micro-average over methods).

Shared denominators (identical for every decompiler, from the index only):
GED -- selected methods whose source CFG is non-degenerate (>=1 statement);
types -- selected methods with >=1 scorable ground-truth variable;
bytes -- all selected methods; classes -- the selected top-level classes.
A method the decompiler did not produce (or produced unparseably, or marked as
failed) stays in every denominator and is never perfect; for MEAN GED it is
charged the cost of building the source CFG from nothing (|V|+|E|).
"""

from __future__ import annotations

import json
import re
import sys
import time
import zipfile
from collections import Counter, defaultdict
from concurrent.futures import ProcessPoolExecutor
from pathlib import Path

from . import projects as P
from .cfg import build_cfg, run_big_stack
from .ged import ged_task, missing_penalty
from .index import load_classes, load_index
from .javasrc import index_source
from .jtypes import compare_types, method_vars, pair_vars, Var
from .mapping import import_map, is_scored_method, map_method, map_types, method_key
from .recompile import _tops_index, recompile_and_compare

_SELECTED: set[str] = set()


def _init(selected):
    global _SELECTED
    _SELECTED = selected


def _extract(args):
    return run_big_stack(_extract_inner, *args)


def _extract_inner(jar, root, rel, regex):
    text = (Path(root) / rel).read_bytes()
    idx = index_source(text)
    tops = [t.binary for t in idx.types if t.parent is None and t.binary]
    tix = _tops_index(jar)
    names = [n for t in tops for n in tix.get(t.replace(".", "/"), [])]
    classes = load_classes(jar, names) if names else {}
    tmap = map_types(classes, [idx], use_lines=False)
    imports = import_map(idx)
    fail_rx = re.compile(regex)
    methods = {}
    for cf in classes.values():
        for m in cf.methods:
            ok, _ = is_scored_method(m)
            if not ok:
                continue
            key = method_key(cf, m)
            if key not in _SELECTED:
                continue
            t = tmap.get(cf.name)
            if t is None:
                # unparseable output is "unscored (parse error)", not "missing"
                methods[key] = {"status": "parse-error" if idx.has_error else "type-missing"}
                continue
            md, why = map_method(t, cf, m, use_lines=False, imports=imports)
            if md is None:
                acc = _implicit_accessor(t, cf, m)
                implicit = _implicit_ctor_params(t, cf, m) if acc is None else None
                if acc is not None:
                    # the output relies on the implicit record accessor (`return this.x;`)
                    methods[key] = {"status": "ok", "implicit": True, "span": None, "body_span": None,
                                    "failed": False, "cfg": _accessor_cfg(),
                                    "vars": [Var("return", 0, "<return>", acc.type_text).__dict__],
                                    "tparams": t.scope_type_params()}
                elif implicit is not None:
                    # the output relies on javac's implicit default / canonical constructor
                    methods[key] = {"status": "ok", "implicit": True, "span": None, "body_span": None,
                                    "failed": False, "cfg": build_cfg(None).to_json(),
                                    "vars": [Var("param", i, p.name, p.type_text).__dict__
                                             for i, p in enumerate(implicit)],
                                    "tparams": t.scope_type_params()}
                else:
                    methods[key] = {"status": "parse-error" if (t.has_error or idx.has_error) else "method-missing",
                                    "why": why}
                continue
            mtext = text[md.node.start_byte : md.node.end_byte].decode("utf-8", errors="replace")
            rec = {
                "status": "ok",
                "span": [md.start_line, md.end_line],
                "body_span": [md.body.start_byte, md.body.end_byte] if md.body is not None else None,
                "failed": bool(fail_rx.search(mtext)),
            }
            if md.has_error:
                rec["status"] = "parse-error"
            else:
                rec["cfg"] = build_cfg(md.body).to_json()
            rec["vars"] = [v.__dict__ for v in method_vars(md)]
            rec["tparams"] = md.scope_type_params()
            methods[key] = rec
    release = max([8] + [cf.major - 44 for cf in classes.values()])
    return {"rel": rel, "tops": tops, "parse_error": idx.has_error, "methods": methods, "release": release}


def _implicit_accessor(t, cf, m):
    """A record component accessor the output leaves implicit: (name, type) or None."""
    if getattr(t, "real_kind", t.kind) != "record" or not t.record_params or m.descriptor[:2] != "()":
        return None
    for p in t.record_params:
        if p.name == m.name and not any(md.name == m.name and not md.params for md in t.methods):
            return p
    return None


_ACCESSOR_CFG = None


def _accessor_cfg() -> dict:
    global _ACCESSOR_CFG
    if _ACCESSOR_CFG is None:
        _ACCESSOR_CFG = build_cfg(index_source(b"class A { Object f() { return x; } }").methods[0].body).to_json()
    return _ACCESSOR_CFG


def _implicit_ctor_params(t, cf, m):
    """Parameters of the constructor javac would generate for a decompiled type
    that omits it, or None. Two cases: the default constructor (no constructor
    declared at all) and a record's canonical constructor (no compact/canonical
    declaration; its parameters are the record header components)."""
    if m.name != "<init>" or t.kind in ("interface", "annotation"):
        return None
    from .mapping import _offsets
    from .jtypes import descriptor_param_types

    have = descriptor_param_types(m.descriptor)
    if getattr(t, "real_kind", t.kind) == "record" and t.record_params is not None:
        if not have and m.name != "<init>":
            return None
        if len(have) == len(t.record_params) and not any(
                md.kind == "compact" or (md.name == "<init>" and len(md.params) == len(have)) for md in t.methods):
            return list(t.record_params)
        return None
    offs, _trailing = _offsets(cf, m)
    if not any(len(have) == off for off in offs):
        return None
    return [] if not any(md.name == "<init>" for md in t.methods) else None


def _types_score(gt: dict, dec: dict | None) -> dict | None:
    gvars = [v for v in gt["vars"] if v["src"] != "unscorable-var"]
    if not gvars:
        return None
    total = len(gvars)
    if dec is None or "vars" not in dec:
        return {"total": total, "exact": 0, "erasure": 0, "misses": [f"{v['role']}:{v['name']}" for v in gvars][:20]}
    src = [Var(v["role"], v["index"], v["name"], v["type"], False) for v in gvars]
    dvars = [Var(v["role"], v["index"], v["name"], v["type_text"], v.get("is_var", False)) for v in dec["vars"]]
    exact = er = 0
    misses = []
    for s, d in pair_vars(src, dvars):
        e, r = compare_types(s.type_text, gt["tparams"], d.type_text if d else None, dec.get("tparams", {}))
        exact += e
        er += r
        if not e:
            misses.append(f"{s.role}:{s.name}: {s.type_text} vs {d.type_text if d else '<none>'}")
    return {"total": total, "exact": exact, "erasure": er, "misses": misses[:20]}


def score_project(proj: P.Project, run_dir: Path, adapter_regex: str, jobs: int, skip_bytes: bool = False,
                  reuse_bytes: bool = False) -> dict:
    run = json.loads((run_dir / "run.json").read_text())
    index = load_index(proj, jobs=jobs)
    gt = {m["key"]: m for m in index["methods"]}
    selected = [k for k in run["selected"] if k in gt]
    sel = set(selected)
    jar = str(proj.dir / "classes.jar")
    root = run_dir / "decompiled"
    rels = sorted(str(p.relative_to(root)) for p in root.rglob("*.java")) if root.exists() else []
    dec: dict[str, dict] = {}
    files = []
    print(f"[score] {proj.name}: extracting {len(rels)} decompiled files", file=sys.stderr, flush=True)
    with ProcessPoolExecutor(max_workers=jobs, initializer=_init, initargs=(sel,)) as ex:
        for r in ex.map(_extract, [(jar, str(root), rel, adapter_regex) for rel in rels], chunksize=4):
            files.append(r)
            for k, v in r["methods"].items():
                v["file"] = r["rel"]
                if k not in dec or dec[k]["status"] != "ok":
                    dec[k] = v
    # GED
    pairs = []
    for k in selected:
        g = gt[k]
        d = dec.get(k)
        if g["cfg"]["statements"] == 0:
            continue
        if d is not None and d["status"] == "ok" and not d["failed"]:
            pairs.append((k, g["cfg"], d["cfg"]))
    print(f"[score] {proj.name}: GED for {len(pairs)} pairs", file=sys.stderr, flush=True)
    ged_res = {}
    with ProcessPoolExecutor(max_workers=jobs) as ex:
        for k, r in ex.map(ged_task, pairs, chunksize=64):
            ged_res[k] = r
    # recompile + bytes
    by_top: dict[str, list[str]] = defaultdict(list)
    for k in selected:
        by_top[gt[k]["top"]].append(k)
    file_jobs = []
    covered_tops = set()
    for f in files:
        keys = {}
        for top in f["tops"]:
            if top in covered_tops:
                continue
            covered_tops.add(top)
            for k in by_top.get(top, []):
                d = dec.get(k)
                keys[k] = d.get("body_span") if d else None
        if any(t in by_top for t in f["tops"]):
            file_jobs.append({"rel": f["rel"], "tops": f["tops"], "keys": keys, "release": f["release"]})
    prev = run_dir / "methods.jsonl"
    if reuse_bytes and prev.exists() and (run_dir / "classes.json").exists():
        # re-score GED/types only; recompilation does not depend on the text mapping
        bytes_res = {}
        for line in prev.read_text().splitlines():
            r = json.loads(line)
            if r.get("bytes") is not None:
                bytes_res[r["key"]] = r["bytes"]
        file_results = {k: v for k, v in json.loads((run_dir / "classes.json").read_text()).items()
                        if not k.startswith("<missing>/")}
        print(f"[score] {proj.name}: reusing recompilation results", file=sys.stderr, flush=True)
    elif skip_bytes:
        file_results, bytes_res = {}, {}
    else:
        debug = "-g" if any(gt[k]["has_lvt"] for k in selected) else (
            "-g:source,lines" if any(gt[k]["has_lines"] for k in selected) else "-g:none")
        tag = "recompile"
        if (run_dir / tag).exists():  # re-score: fresh work dir, the old one is kept
            tag = f"recompile-{time.strftime('%Y%m%d-%H%M%S')}"
        file_results, bytes_res = recompile_and_compare(proj, run_dir, file_jobs, jobs, debug, tag=tag)
    # assemble per-method records
    out_dir = run_dir
    recs = []
    for k in selected:
        g = gt[k]
        d = dec.get(k)
        status = "class-missing" if d is None else d["status"]
        if d is not None and d["status"] == "ok" and d.get("failed"):
            status = "decompiler-failed"
        rec = {
            "key": k, "class": g["class"], "name": g["name"], "desc": g["desc"],
            "stratum": g["stratum"], "code_len": g["code_len"],
            "source_file": str(proj.dir / "sources" / g["file"]), "source_span": g["span"],
            "decompiled_file": str(root / d["file"]) if d is not None and "file" in d else None,
            "decompiled_span": d.get("span") if d else None,
            "status": status,
        }
        # GED
        if g["cfg"]["statements"] == 0:
            rec["ged"] = None
        elif k in ged_res and ged_res[k].get("value") is not None:
            rec["ged"] = ged_res[k]
        else:
            rec["ged"] = {"value": None, "method": "unscored", "reason": status,
                          "penalty": missing_penalty(g["cfg"]), "src_blocks": len(g["cfg"]["roles"])}
        rec["ged_perfect"] = None if rec["ged"] is None else rec["ged"].get("value") == 0.0
        rec["src_blocks"] = len(g["cfg"]["roles"])
        # types
        t = _types_score(g, d if status in ("ok", "decompiler-failed", "parse-error") else None)
        rec["types"] = t
        rec["types_perfect"] = None if t is None else t["exact"] == t["total"]
        rec["types_erasure_perfect"] = None if t is None else t["erasure"] == t["total"]
        # bytes
        if skip_bytes:
            rec["bytes"] = None
            rec["bytes_perfect"] = None
        else:
            b = bytes_res.get(k) or {"similarity": 0.0, "exact": False, "compiled": False, "error": "no-decompiled-class"}
            rec["bytes"] = b
            rec["bytes_perfect"] = bool(b.get("exact"))
        perf = [rec[m] for m in ("ged_perfect", "types_perfect", "bytes_perfect") if rec[m] is not None]
        rec["union_perfect"] = any(perf) if perf else None
        rec["all_perfect"] = all(perf) if perf else None
        recs.append(rec)
    with open(out_dir / "methods.jsonl", "w") as f:
        for r in recs:
            f.write(json.dumps(r, sort_keys=True) + "\n")
    tops_selected = sorted(by_top)
    for top in tops_selected:
        if top not in covered_tops:
            file_results.setdefault(f"<missing>/{top}", {"tops": [top], "raw_compiles": False,
                                                         "compiles_after_salvage": False, "missing": True,
                                                         "error_categories": {"class-missing": 1}})
    (out_dir / "classes.json").write_text(json.dumps(file_results, indent=1, sort_keys=True) + "\n")
    board = aggregate(recs, file_results, run, n_tops=len(tops_selected), skip_bytes=skip_bytes)
    board.update({"project": proj.name, "decompiler": run["decompiler"], "selection": run["selection"]})
    (out_dir / "scoreboard.json").write_text(json.dumps(board, indent=1, sort_keys=True) + "\n")
    (out_dir / "scoreboard.md").write_text(render_markdown({proj.name: board}, title=f"{run['decompiler']} / {proj.name}"))
    return board


def _pct(a, b):
    return round(100.0 * a / b, 2) if b else None


def aggregate(recs: list[dict], file_results: dict, run: dict | None, n_tops: int, skip_bytes: bool = False) -> dict:
    n = len(recs)
    ged_den = [r for r in recs if r["ged"] is not None]
    ged_scored = [r for r in ged_den if r["ged"].get("value") is not None]
    ged_vals = [r["ged"]["value"] if r["ged"].get("value") is not None else r["ged"]["penalty"] for r in ged_den]
    multi = [r for r in ged_den if r["src_blocks"] >= 2]
    unscored = Counter(r["ged"].get("reason") for r in ged_den if r["ged"].get("value") is None)
    t_den = [r for r in recs if r["types"] is not None]
    tv_total = sum(r["types"]["total"] for r in t_den)
    b_den = [r for r in recs if r["bytes"] is not None]
    status = Counter(r["status"] for r in recs)
    cats: Counter = Counter()
    cat_files: Counter = Counter()
    raw_ok = salv_ok = 0
    sel_tops = {r["class"].split("$")[0] for r in recs}
    for fr in file_results.values():
        # a file may define several top-level classes; each counts once
        weight = sum(1 for t in fr.get("tops") or [] if t in sel_tops) or (1 if not fr.get("tops") else 0)
        raw_ok += weight * bool(fr.get("raw_compiles"))
        salv_ok += weight * bool(fr.get("compiles_after_salvage"))
        for c, k in (fr.get("error_categories") or {}).items():
            cats[c] += k
            cat_files[c] += max(weight, 1)
    board = {
        "methods": n,
        "ged": {
            "denominator": len(ged_den),
            "mean": round(sum(ged_vals) / len(ged_vals), 4) if ged_vals else None,
            "mean_scored_only": round(sum(r["ged"]["value"] for r in ged_scored) / len(ged_scored), 4) if ged_scored else None,
            "scored": len(ged_scored),
            "perfect_pct": _pct(sum(1 for r in ged_den if r["ged_perfect"]), len(ged_den)),
            "perfect_multiblock_pct": _pct(sum(1 for r in multi if r["ged_perfect"]), len(multi)),
            "multiblock_denominator": len(multi),
            "unscored": dict(unscored),
        },
        "types": {
            "denominator": len(t_den),
            "variables": tv_total,
            "var_exact_pct": _pct(sum(r["types"]["exact"] for r in t_den), tv_total),
            "var_erasure_pct": _pct(sum(r["types"]["erasure"] for r in t_den), tv_total),
            "method_perfect_pct": _pct(sum(1 for r in t_den if r["types_perfect"]), len(t_den)),
            "method_erasure_perfect_pct": _pct(sum(1 for r in t_den if r["types_erasure_perfect"]), len(t_den)),
        },
        "bytes": None if skip_bytes else {
            "denominator": len(b_den),
            "mean_similarity": round(sum(r["bytes"]["similarity"] for r in b_den) / len(b_den), 4) if b_den else None,
            "exact_pct": _pct(sum(1 for r in b_den if r["bytes_perfect"]), len(b_den)),
            "method_compiled_pct": _pct(sum(1 for r in b_den if r["bytes"].get("compiled")), len(b_den)),
        },
        "classes": None if skip_bytes else {
            "denominator": n_tops,
            "raw_compile_pct": _pct(raw_ok, n_tops),
            "after_salvage_pct": _pct(salv_ok, n_tops),
            "javac_error_categories": dict(cats.most_common(15)),  # diagnostics
            "javac_error_classes": dict(cat_files.most_common(15)),  # top-level classes with >=1
        },
        "union_perfect_pct": _pct(sum(1 for r in recs if r["union_perfect"]), sum(1 for r in recs if r["union_perfect"] is not None)),
        "all_perfect_pct": _pct(sum(1 for r in recs if r["all_perfect"]), sum(1 for r in recs if r["all_perfect"] is not None)),
        "decompiler_failed_methods": status.get("decompiler-failed", 0),
        "missing_methods": status.get("class-missing", 0) + status.get("type-missing", 0) + status.get("method-missing", 0),
        "parse_error_methods": status.get("parse-error", 0),
        "status": dict(status),
    }
    if run is not None:
        board["decompile"] = {k: run.get(k) for k in ("wall_seconds", "peak_rss_mb", "cpu_seconds", "exit_code", "timed_out", "java_files")}
    return board


def pool_boards(per_project: dict[str, list[dict]], file_results: dict[str, dict], runs: dict[str, dict], n_tops: int) -> dict:
    recs = [r for rs in per_project.values() for r in rs]
    return aggregate(recs, file_results, None, n_tops)


def _f(v, suffix=""):
    return "-" if v is None else f"{v}{suffix}"


def render_markdown(boards: dict[str, dict], title: str) -> str:
    lines = [f"# {title}", ""]
    hdr = ("| project | methods | GED mean | GED perfect | GED perfect (>=2 blocks) | types var exact | "
           "types method perfect | types erasure | bytes mean | bytes exact | classes recompile | "
           "union perfect | all perfect | VF/dec failures | missing | time s | peak RSS MB |")
    lines.append(hdr)
    lines.append("|" + "---|" * hdr.count("|")[:-1] if False else "|" + "|".join(["---"] * (hdr.count("|") - 1)) + "|")
    for name, b in boards.items():
        g, t, by, c = b["ged"], b["types"], b.get("bytes") or {}, b.get("classes") or {}
        d = b.get("decompile") or {}
        lines.append(
            f"| {name} | {b['methods']} | {_f(g['mean'])} | {_f(g['perfect_pct'], '%')} | {_f(g['perfect_multiblock_pct'], '%')} | "
            f"{_f(t['var_exact_pct'], '%')} | {_f(t['method_perfect_pct'], '%')} | {_f(t['var_erasure_pct'], '%')} | "
            f"{_f(by.get('mean_similarity'))} | {_f(by.get('exact_pct'), '%')} | {_f(c.get('raw_compile_pct'), '%')} | "
            f"{_f(b['union_perfect_pct'], '%')} | {_f(b['all_perfect_pct'], '%')} | {b['decompiler_failed_methods']} | "
            f"{b['missing_methods']} | {_f(d.get('wall_seconds'))} | {_f(d.get('peak_rss_mb'))} |"
        )
    lines.append("")
    return "\n".join(lines)
