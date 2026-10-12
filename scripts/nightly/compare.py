"""Compare tonight's nightly decbench results with the previous night's and write the report.

Scores are paired per function: the sample is fixed (same dataset revision, same
seed, same decbench), so every function is measured on both nights and the question
is whether the functions that moved moved in one direction more often than chance
allows. Two exact tests per metric, under the null that a change is equally likely
to be a gain or a loss:

- perfect rate: McNemar's exact test on the functions that became / stopped being
  perfect (GED 0, type_match 1.0);
- value: the sign test on the functions whose value improved / worsened.

The four p-values are Holm-adjusted together. A function decbench can measure that
kuna produced no value for counts as the worst value (GED inf, type_match below 0).

Speed is the sample's total decompile CPU time. When tonight's run also timed last
night's build on the same runner (`baseline_time`), the comparison is that A/B;
otherwise it falls back to last night's recorded times, which come from a different
runner and are reported as such. The 95% interval is a bootstrap over binaries.

    python3 -m scripts.nightly.compare --current out/results.json \
        [--previous prev/results.json] [--history prev/history.jsonl] \
        --report report.md --history-out out/history.jsonl [--fail-on-regression]

Stdlib only.
"""
from __future__ import annotations

import argparse
import json
import math
import os
import random
import sys
from pathlib import Path

METRICS = {
    "ged": {"label": "GED", "lower_is_better": True, "perfect": 0.0},
    "type_match": {"label": "type_match", "lower_is_better": False, "perfect": 1.0},
}
COMPAT_KEYS = ("dataset_revision", "config", "sample_digest", "decbench_commit", "rust_joern",
               "max_fn_seconds")
HISTORY_ROWS = 14
MIN_SPEED_BINARIES = 10
SAME_RUNNER = "same runner"
PREVIOUS_RUNNER = "previous night's runner"


def binom_two_sided(k: int, n: int) -> float:
    """Exact two-sided sign-test p-value for k successes in n fair coin flips."""
    if n == 0:
        return 1.0
    m = min(k, n - k)
    log_half_n = -n * math.log(2)
    tail = 0.0
    for i in range(m + 1):
        tail += math.exp(math.lgamma(n + 1) - math.lgamma(i + 1) - math.lgamma(n - i + 1)
                         + log_half_n)
    return min(1.0, 2 * tail)


def holm(pvalues: dict[str, float]) -> dict[str, float]:
    """Holm-Bonferroni adjusted p-values (monotone, capped at 1)."""
    ordered = sorted(pvalues.items(), key=lambda kv: kv[1])
    adjusted: dict[str, float] = {}
    running = 0.0
    for rank, (key, p) in enumerate(ordered):
        running = max(running, min(1.0, (len(ordered) - rank) * p))
        adjusted[key] = running
    return adjusted


def is_perfect(metric: str, value: float | None) -> bool:
    return value is not None and abs(value - METRICS[metric]["perfect"]) < 1e-9


def ordering_value(metric: str, value: float | None) -> float:
    """The value on a higher-is-better scale, with a failure below every real value."""
    if value is None or (isinstance(value, float) and math.isnan(value)):
        return -math.inf
    return -value if METRICS[metric]["lower_is_better"] else value


def metric_values(results: dict, metric: str) -> dict[str, float | None]:
    return {k: f[metric] for k, f in results.get("functions", {}).items() if metric in f}


def absolute_scores(results: dict) -> dict[str, dict]:
    out = {}
    for metric in METRICS:
        values = metric_values(results, metric)
        finite = [v for v in values.values() if v is not None and math.isfinite(v)]
        out[metric] = {
            "measured": len(values),
            "perfect": sum(is_perfect(metric, v) for v in values.values()),
            "failed": sum(v is None for v in values.values()),
            "mean": sum(finite) / len(finite) if finite else None,
        }
    return out


def paired_scores(prev: dict, cur: dict) -> dict[str, dict]:
    out = {}
    for metric in METRICS:
        old = metric_values(prev, metric)
        new = metric_values(cur, metric)
        keys = sorted(set(old) | set(new))
        gained, lost, better, worse = [], [], [], []
        deltas = []
        for key in keys:
            a, b = old.get(key), new.get(key)
            if is_perfect(metric, b) and not is_perfect(metric, a):
                gained.append(key)
            elif is_perfect(metric, a) and not is_perfect(metric, b):
                lost.append(key)
            oa, ob = ordering_value(metric, a), ordering_value(metric, b)
            if ob != oa:
                (better if ob > oa else worse).append((key, a, b, ob - oa))
            if a is not None and b is not None and math.isfinite(a) and math.isfinite(b):
                deltas.append(b - a)
        out[metric] = {
            "functions": len(keys),
            "gained": gained,
            "lost": lost,
            "improved": sorted(better, key=lambda t: -t[3]),
            "worsened": sorted(worse, key=lambda t: t[3]),
            "p_perfect": binom_two_sided(len(gained), len(gained) + len(lost)),
            "p_value": binom_two_sided(len(better), len(better) + len(worse)),
            "mean_delta": sum(deltas) / len(deltas) if deltas else 0.0,
        }
    return out


def bootstrap_ratio(pairs: list[tuple[float, float]], iterations: int = 2000,
                    seed: int = 0) -> tuple[float, float, float]:
    """sum(new)/sum(old) over (old, new) pairs, with a 95% percentile bootstrap interval."""
    total_old = sum(o for o, _ in pairs)
    total_new = sum(n for _, n in pairs)
    point = total_new / total_old if total_old > 0 else math.nan
    rng = random.Random(seed)
    ratios = []
    for _ in range(iterations):
        draw = [pairs[rng.randrange(len(pairs))] for _ in pairs]
        so = sum(o for o, _ in draw)
        if so > 0:
            ratios.append(sum(n for _, n in draw) / so)
    ratios.sort()
    if not ratios:
        return point, math.nan, math.nan
    return point, ratios[int(0.025 * len(ratios))], ratios[min(len(ratios) - 1, int(0.975 * len(ratios)))]


def speed(cur: dict, prev: dict | None) -> dict | None:
    """Total-CPU comparison, preferring tonight's same-runner A/B against last night's build."""
    pairs, names, source = [], [], None
    same_runner = [(k, b) for k, b in cur["binaries"].items()
                   if b.get("baseline_time") and b["time"].get("exit") == 0
                   and b["baseline_time"].get("exit") == 0]
    if same_runner:
        source = SAME_RUNNER
        for key, b in same_runner:
            pairs.append((b["baseline_time"]["cpu"], b["time"]["cpu"]))
            names.append(key)
    elif prev is not None:
        source = PREVIOUS_RUNNER
        for key, b in cur["binaries"].items():
            p = prev.get("binaries", {}).get(key)
            if p and p["time"].get("exit") == 0 and b["time"].get("exit") == 0:
                pairs.append((p["time"]["cpu"], b["time"]["cpu"]))
                names.append(key)
    if not pairs:
        return None
    ratio, lo, hi = bootstrap_ratio(pairs)
    movers = sorted(zip(names, pairs), key=lambda t: t[1][1] - t[1][0])
    return {
        "source": source,
        "binaries": len(pairs),
        "old_total": sum(o for o, _ in pairs),
        "new_total": sum(n for _, n in pairs),
        "ratio": ratio,
        "ci": (lo, hi),
        "slower": [m for m in reversed(movers) if m[1][1] > m[1][0]][:8],
        "faster": [m for m in movers if m[1][1] < m[1][0]][:5],
    }


def failed_binaries(results: dict) -> dict[str, str]:
    out = {}
    for key, b in results.get("binaries", {}).items():
        t = b["time"]
        if t.get("exit") != 0 or t.get("error") or b.get("fatal"):
            out[key] = (t.get("error") or b.get("fatal") or f"exit {t.get('exit')}").strip()
    return out


def incompatibility(prev: dict | None, cur: dict) -> str | None:
    if prev is None:
        return "no previous nightly result"
    if prev.get("schema") != cur.get("schema"):
        return "results schema changed"
    for key in COMPAT_KEYS:
        if prev["meta"].get(key) != cur["meta"].get(key):
            return f"`{key}` changed ({prev['meta'].get(key)} -> {cur['meta'].get(key)})"
    return None


def evaluate(cur: dict, prev: dict | None, alpha: float, speed_threshold: float) -> dict:
    why_not = incompatibility(prev, cur)
    paired = None if why_not else paired_scores(prev, cur)
    adjusted = {}
    if paired:
        raw = {}
        for metric, r in paired.items():
            raw[f"{metric}:perfect"] = r["p_perfect"]
            raw[f"{metric}:value"] = r["p_value"]
        adjusted = holm(raw)
    verdicts = {}
    regressions = []
    for metric, r in (paired or {}).items():
        for test, up, down in (("perfect", len(r["gained"]), len(r["lost"])),
                               ("value", len(r["improved"]), len(r["worsened"]))):
            p = adjusted[f"{metric}:{test}"]
            if p < alpha and up != down:
                verdict = "better" if up > down else "worse"
                if verdict == "worse":
                    regressions.append(f"{METRICS[metric]['label']} {test}")
            else:
                verdict = "no significant change"
            verdicts[f"{metric}:{test}"] = (verdict, p)
    new_failures = sorted(set(failed_binaries(cur)) - set(failed_binaries(prev or {})))
    if prev is not None and new_failures:
        regressions.append(f"{len(new_failures)} binaries newly failing")
    sp = speed(cur, prev)
    if sp and sp["source"] == SAME_RUNNER and sp["binaries"] >= MIN_SPEED_BINARIES:
        lo = sp["ci"][0]
        if not math.isnan(lo) and lo > 1.0 and sp["ratio"] > 1.0 + speed_threshold:
            regressions.append(f"decompile CPU time +{(sp['ratio'] - 1) * 100:.1f}%")
    return {
        "incompatible": why_not,
        "absolute": absolute_scores(cur),
        "previous_absolute": absolute_scores(prev) if prev else None,
        "paired": paired,
        "verdicts": verdicts,
        "speed": sp,
        "failed": failed_binaries(cur),
        "new_failures": new_failures,
        "regressions": regressions,
    }


def pct(part: int, whole: int) -> str:
    return f"{100 * part / whole:.2f}%" if whole else "n/a"


def fmt_value(v: float | None) -> str:
    if v is None:
        return "fail"
    if math.isinf(v):
        return "inf"
    return f"{v:g}" if v == int(v) else f"{v:.3f}"


def fmt_p(p: float) -> str:
    return f"{p:.2g}" if p >= 1e-4 else "<1e-4"


def history_entry(cur: dict, ev: dict) -> dict:
    meta = cur["meta"]
    entry = {
        "date": meta.get("started"),
        "label": meta.get("label"),
        "kuna_version": meta.get("kuna_version"),
        "sample_digest": meta.get("sample_digest"),
        "cpu_model": meta.get("cpu_model"),
        "cpu_total": round(sum(b["time"]["cpu"] for b in cur["binaries"].values()), 2),
        "failed_binaries": len(ev["failed"]),
        "regressions": ev["regressions"],
    }
    for metric, a in ev["absolute"].items():
        entry[metric] = {"measured": a["measured"], "perfect": a["perfect"], "failed": a["failed"],
                         "mean": None if a["mean"] is None else round(a["mean"], 5)}
    if ev["speed"]:
        entry["speed_ratio"] = round(ev["speed"]["ratio"], 5)
        entry["speed_source"] = ev["speed"]["source"]
    return entry


def render(cur: dict, prev: dict | None, ev: dict, history: list[dict], alpha: float,
           speed_threshold: float, run_url: str | None) -> str:
    meta = cur["meta"]
    lines = [f"## Nightly decbench: {meta.get('label') or meta.get('kuna_version')}", ""]
    nfun = len(cur["functions"])
    lines.append(
        f"Sample `{meta['sample_digest']}`: {len(cur['binaries'])} binaries, {nfun} scored "
        f"functions from `{meta['config']}` (dataset `{meta['dataset_revision'][:10]}`, "
        f"decbench `{(meta.get('decbench_commit') or '?')[:10]}`). Runner: {meta['cpu_model']}, "
        f"{meta['nproc']} CPUs.")
    if prev:
        lines.append(f"Compared with: {prev['meta'].get('label') or prev['meta'].get('kuna_version')} "
                     f"({prev['meta'].get('started')}).")
    lines.append("")
    if ev["regressions"]:
        lines.append(f"**Regression:** {'; '.join(ev['regressions'])}.")
    elif prev is not None:
        lines.append("**No significant regression.**")
    lines.append("")

    lines.append("### Scores")
    lines.append("")
    if ev["incompatible"]:
        lines.append(f"No paired comparison: {ev['incompatible']}. Tonight sets the baseline.")
        lines.append("")
    lines.append("| metric | measured | perfect | previous | change | gained / lost | p (Holm) | verdict |")
    lines.append("|---|---:|---:|---:|---:|---:|---:|---|")
    for metric, a in ev["absolute"].items():
        label = METRICS[metric]["label"]
        now = pct(a["perfect"], a["measured"])
        if ev["paired"]:
            pa = ev["previous_absolute"][metric]
            before = pct(pa["perfect"], pa["measured"])
            delta = a["perfect"] - pa["perfect"]
            r = ev["paired"][metric]
            verdict, p = ev["verdicts"][f"{metric}:perfect"]
            lines.append(f"| {label} perfect | {a['measured']} | {now} | {before} | {delta:+d} | "
                         f"+{len(r['gained'])} / -{len(r['lost'])} | {fmt_p(p)} | {verdict} |")
        else:
            lines.append(f"| {label} perfect | {a['measured']} | {now} | | | | | |")
    if ev["paired"]:
        lines.append("")
        lines.append("| metric | mean now | mean change | improved / worsened | p (Holm) | verdict |")
        lines.append("|---|---:|---:|---:|---:|---|")
        for metric, r in ev["paired"].items():
            a = ev["absolute"][metric]
            verdict, p = ev["verdicts"][f"{metric}:value"]
            mean = "n/a" if a["mean"] is None else f"{a['mean']:.4f}"
            lines.append(f"| {METRICS[metric]['label']} | {mean} | {r['mean_delta']:+.4f} | "
                         f"{len(r['improved'])} / {len(r['worsened'])} | {fmt_p(p)} | {verdict} |")
        lines.append("")
        lines.append(f"Significance: Holm-adjusted p < {alpha}. GED is lower-is-better; a missing "
                     f"function counts as the worst value.")
        for metric, r in ev["paired"].items():
            label = METRICS[metric]["label"]
            for title, rows in ((f"{label}: worsened ({len(r['worsened'])})", r["worsened"]),
                                (f"{label}: improved ({len(r['improved'])})", r["improved"])):
                if not rows:
                    continue
                lines.append("")
                lines.append(f"<details><summary>{title}</summary>")
                lines.append("")
                lines.append("| function | previous | now |")
                lines.append("|---|---:|---:|")
                for key, a_, b_, _d in rows[:40]:
                    lines.append(f"| `{key}` | {fmt_value(a_)} | {fmt_value(b_)} |")
                if len(rows) > 40:
                    lines.append(f"| ... {len(rows) - 40} more | | |")
                lines.append("")
                lines.append("</details>")
    lines.append("")

    lines.append("### Speed")
    lines.append("")
    sp = ev["speed"]
    total = sum(b["time"]["cpu"] for b in cur["binaries"].values())
    wall = sum(b["time"]["wall"] for b in cur["binaries"].values())
    lines.append(f"Tonight: {total:.1f} s CPU ({wall:.1f} s wall) to decompile the sample, one "
                 f"binary at a time.")
    if sp:
        lo, hi = sp["ci"]
        change = (sp["ratio"] - 1) * 100
        ci = "" if math.isnan(lo) else f" (95% CI {(lo - 1) * 100:+.1f}% .. {(hi - 1) * 100:+.1f}%)"
        against = (f"{meta.get('baseline_label') or 'the baseline build'}, timed on this runner"
                   if sp["source"] == SAME_RUNNER else "last night's recorded times")
        lines.append(f"Against {against}: {sp['old_total']:.1f} s -> {sp['new_total']:.1f} s "
                     f"over {sp['binaries']} binaries, **{change:+.1f}%**{ci}.")
        if sp["source"] != SAME_RUNNER:
            prev_cpu = prev["meta"].get("cpu_model") if prev else None
            lines.append(f"Different runners (previous: {prev_cpu}); treat as indicative only, "
                         f"never as a regression.")
        elif sp["binaries"] < MIN_SPEED_BINARIES:
            lines.append(f"Fewer than {MIN_SPEED_BINARIES} binaries: never called a regression.")
        elif not math.isnan(lo) and (lo > 1.0 or hi < 1.0):
            lines.append(f"The interval excludes zero; flagged only when slower by more than "
                         f"{speed_threshold * 100:.0f}%.")
        if sp["slower"] or sp["faster"]:
            lines.append("")
            lines.append("| binary | before (s) | now (s) | change |")
            lines.append("|---|---:|---:|---:|")
            for key, (o, n) in sp["slower"] + sp["faster"]:
                rel = f"{(n / o - 1) * 100:+.0f}%" if o > 0 else ""
                lines.append(f"| `{key}` | {o:.2f} | {n:.2f} | {rel} |")
    lines.append("")

    if ev["failed"]:
        lines.append("### Binaries kuna failed on")
        lines.append("")
        for key, why in sorted(ev["failed"].items()):
            new = " **(new tonight)**" if key in ev["new_failures"] else ""
            lines.append(f"- `{key}`{new}: {why.splitlines()[-1][:200] if why else ''}")
        lines.append("")

    if history:
        lines.append("### Recent nights")
        lines.append("")
        lines.append("| date | build | GED perfect | type_match perfect | CPU (s) | speed vs prior |")
        lines.append("|---|---|---:|---:|---:|---:|")
        for h in history[-HISTORY_ROWS:]:
            g, t = h.get("ged", {}), h.get("type_match", {})
            ratio = h.get("speed_ratio")
            sr = "" if ratio is None else f"{(ratio - 1) * 100:+.1f}%"
            lines.append(f"| {(h.get('date') or '')[:10]} | {h.get('label') or h.get('kuna_version')} | "
                         f"{pct(g.get('perfect', 0), g.get('measured', 0))} | "
                         f"{pct(t.get('perfect', 0), t.get('measured', 0))} | "
                         f"{h.get('cpu_total', 0):.0f} | {sr} |")
        lines.append("")
    if run_url:
        lines.append(f"Full per-function results: the `decbench-nightly` artifact of {run_url}.")
    return "\n".join(lines) + "\n"


def load_history(path: Path | None) -> list[dict]:
    if path is None or not path.is_file():
        return []
    return [json.loads(line) for line in path.read_text().splitlines() if line.strip()]


def main(argv: list[str] | None = None) -> int:
    ap = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    ap.add_argument("--current", type=Path, required=True)
    ap.add_argument("--previous", type=Path)
    ap.add_argument("--history", type=Path, help="the previous night's history.jsonl")
    ap.add_argument("--history-out", type=Path)
    ap.add_argument("--report", type=Path, required=True)
    ap.add_argument("--summary-json", type=Path)
    ap.add_argument("--alpha", type=float, default=0.01)
    ap.add_argument("--speed-threshold", type=float, default=0.05)
    ap.add_argument("--run-url")
    ap.add_argument("--fail-on-regression", action="store_true")
    args = ap.parse_args(argv)

    cur = json.loads(args.current.read_text())
    prev = None
    if args.previous and args.previous.is_file():
        prev = json.loads(args.previous.read_text())
    ev = evaluate(cur, prev, args.alpha, args.speed_threshold)
    history = load_history(args.history) + [history_entry(cur, ev)]
    report = render(cur, prev, ev, history, args.alpha, args.speed_threshold, args.run_url)
    args.report.write_text(report)
    if args.history_out:
        args.history_out.write_text("".join(json.dumps(h, sort_keys=True) + "\n" for h in history))
    if args.summary_json:
        args.summary_json.write_text(json.dumps(history[-1], indent=1, sort_keys=True) + "\n")
    if os.environ.get("GITHUB_ACTIONS"):
        for reg in ev["regressions"]:
            print(f"::warning title=nightly decbench regression::{reg}")
    print(report)
    return 1 if args.fail_on_regression and ev["regressions"] else 0


if __name__ == "__main__":
    raise SystemExit(main())
