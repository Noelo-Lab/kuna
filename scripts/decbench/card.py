"""Write the DecBench results card (light and dark SVG) from the public scoreboard.

    python3 -m scripts.decbench.card                     # fetch decbench.com, write both SVGs
    python3 -m scripts.decbench.card --json agg.json     # from a saved aggregates.json

The numbers are the scoreboard's own: ``https://decbench.com/data/aggregates.json``,
the file the decbench.com leaderboard renders. For each dataset the card shows
kuna's *Union* (the leaderboard's default ranking: the share of functions a
decompiler scores perfectly on at least one metric), kuna's rank among the
decompilers the leaderboard lists for that dataset, and the best other entry:
the leader when kuna is not first, the runner-up when it is. The scoreboard's
``generated_at`` date and kuna's benchmarked version are printed on the card.
Nothing is edited by hand; rerun this after a scoreboard refresh.

It writes a wide card and a narrow one for phone widths, each in a light and a
dark version; the README and the landing page pick one through ``<picture>``.
"""
from __future__ import annotations

import argparse
import json
import sys
import urllib.request
from pathlib import Path
from xml.sax.saxutils import escape

URL = "https://decbench.com/data/aggregates.json"
DATASETS = ("unoptimized", "optimized", "inlined", "large")
OUT = Path(__file__).resolve().parents[2] / "integrations" / "web" / "assets" / "img"

PALETTES = {
    "light": dict(bg="#F7F4F3", rule="#E4DDDB", ink="#141110", ink2="#3A3230",
                  muted="#6E6663", red="#B80D1E", track="#E4DDDB"),
    "dark": dict(bg="#1B1716", rule="#3A3230", ink="#EDE7E5", ink2="#C9C0BD",
                 muted="#A39894", red="#E8404E", track="#3A3230"),
}
MONO = "'Roboto Mono', ui-monospace, SFMono-Regular, Menlo, Consolas, monospace"
DISP = "Jost, Futura, 'Avenir Next', system-ui, sans-serif"


def display_name(agg: dict, key: str) -> str:
    name = agg.get("decompiler_registry", {}).get(key, {}).get("display_name", key)
    if "@" in key:
        name += " " + key.split("@", 1)[1]
    return name


def listed(agg: dict, key: str, dataset: str) -> bool:
    """Whether the leaderboard lists `key` on `dataset` (sample-set-only and preset-limited entries are not)."""
    if key in agg.get("sample_set_only", ()):
        return False
    presets = agg.get("decompiler_presets", {}).get(key)
    return presets is None or dataset in presets


def standings(agg: dict, dataset: str, subject: str = "kuna") -> dict:
    """Union standings on one dataset, at the leaderboard's default (failures not normalized)."""
    overall = agg["combos"][f"{dataset}|0"]["overall"]
    rows = [(hit / total if total else 0.0, hit, key) for key, (hit, total) in overall.items()
            if listed(agg, key, dataset)]
    rows.sort(key=lambda r: (-r[1], r[2]))
    rank = next(i for i, r in enumerate(rows) if r[2] == subject) + 1
    other = rows[1] if rank == 1 else rows[0]
    return {
        "dataset": dataset,
        "share": rows[rank - 1][0],
        "rank": rank,
        "of": len(rows),
        "other": {"name": display_name(agg, other[2]), "share": other[0], "rank": 2 if rank == 1 else 1},
    }


def pct(share: float) -> str:
    return f"{share * 100:.1f}%"


def render(agg: dict, palette: dict, subject: str = "kuna") -> str:
    rows = [standings(agg, d, subject) for d in DATASETS if f"{d}|0" in agg["combos"]]
    version = agg.get("decompiler_versions", {}).get(subject, "?")
    date = str(agg.get("generated_at", "?"))[:10]
    w, x_bar, bar_w, top, step = 680, 132, 180, 84, 30
    h = top + step * (len(rows) - 1) + 78
    p = palette
    out = [
        f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {w} {h}" role="img" '
        f'aria-label="{escape(summary(rows, date), {chr(34): "&quot;"})}">',
        f"<title>kuna on DecBench ({date})</title>",
        f'<rect x="0.5" y="0.5" width="{w - 1}" height="{h - 1}" fill="{p["bg"]}" stroke="{p["rule"]}"/>',
        f'<text x="24" y="38" font-family="{DISP}" font-size="22" fill="{p["ink"]}">DecBench</text>',
        f'<text x="{w - 24}" y="37" text-anchor="end" font-family="{MONO}" font-size="11" fill="{p["muted"]}">'
        f"Union: functions scored perfect on at least one metric</text>",
        f'<line x1="24" y1="54.5" x2="{w - 24}" y2="54.5" stroke="{p["rule"]}"/>',
    ]
    for i, r in enumerate(rows):
        y = top + step * i
        fill = max(1.0, bar_w * r["share"])
        tick = x_bar + bar_w * r["other"]["share"]
        out += [
            f'<text x="24" y="{y}" font-family="{MONO}" font-size="12" fill="{p["ink2"]}">{r["dataset"]}</text>',
            f'<rect x="{x_bar}" y="{y - 9}" width="{bar_w}" height="8" fill="{p["track"]}"/>',
            f'<rect x="{x_bar}" y="{y - 9}" width="{fill:.1f}" height="8" fill="{p["red"]}"/>',
            f'<rect x="{tick - 0.75:.1f}" y="{y - 12}" width="1.5" height="14" fill="{p["ink"]}"/>',
            f'<text x="324" y="{y}" font-family="{MONO}" font-size="13" font-weight="700" fill="{p["ink"]}">'
            f'{pct(r["share"])}</text>',
            f'<text x="388" y="{y}" font-family="{MONO}" font-size="12" font-weight="700" '
            f'fill="{p["red"] if r["rank"] == 1 else p["ink"]}">#{r["rank"]} of {r["of"]}</text>',
            f'<text x="462" y="{y}" font-family="{MONO}" font-size="11" fill="{p["muted"]}">'
            f'#{r["other"]["rank"]} {escape(r["other"]["name"])} {pct(r["other"]["share"])}</text>',
        ]
    y = top + step * (len(rows) - 1) + 24
    out += [
        f'<line x1="24" y1="{y + 0.5}" x2="{w - 24}" y2="{y + 0.5}" stroke="{p["rule"]}"/>',
        f'<text x="24" y="{y + 22}" font-family="{MONO}" font-size="10.5" fill="{p["muted"]}">'
        f"kuna {escape(version)} · red bar: kuna · tick: the other entry named on the row</text>",
        f'<text x="24" y="{y + 38}" font-family="{MONO}" font-size="10.5" fill="{p["muted"]}">'
        f"source: decbench.com scoreboard, generated {date}</text>",
        "</svg>",
    ]
    return "".join(out) + "\n"


def render_narrow(agg: dict, palette: dict, subject: str = "kuna") -> str:
    """The same card stacked for phone widths: label and rank, then the bar, then the other entry."""
    rows = [standings(agg, d, subject) for d in DATASETS if f"{d}|0" in agg["combos"]]
    version = agg.get("decompiler_versions", {}).get(subject, "?")
    date = str(agg.get("generated_at", "?"))[:10]
    w, x_bar, bar_w, top, step = 360, 20, 320, 98, 60
    h = top + step * (len(rows) - 1) + 104
    p = palette
    out = [
        f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {w} {h}" role="img" '
        f'aria-label="{escape(summary(rows, date), {chr(34): "&quot;"})}">',
        f"<title>kuna on DecBench ({date})</title>",
        f'<rect x="0.5" y="0.5" width="{w - 1}" height="{h - 1}" fill="{p["bg"]}" stroke="{p["rule"]}"/>',
        f'<text x="20" y="36" font-family="{DISP}" font-size="20" fill="{p["ink"]}">DecBench</text>',
        f'<text x="20" y="56" font-family="{MONO}" font-size="10.5" fill="{p["muted"]}">'
        f"Union: perfect on at least one metric</text>",
        f'<line x1="20" y1="68.5" x2="{w - 20}" y2="68.5" stroke="{p["rule"]}"/>',
    ]
    for i, r in enumerate(rows):
        y = top + step * i
        tick = x_bar + bar_w * r["other"]["share"]
        out += [
            f'<text x="20" y="{y}" font-family="{MONO}" font-size="12" fill="{p["ink2"]}">{r["dataset"]}</text>',
            f'<text x="262" y="{y}" text-anchor="end" font-family="{MONO}" font-size="13" font-weight="700" '
            f'fill="{p["ink"]}">{pct(r["share"])}</text>',
            f'<text x="{w - 20}" y="{y}" text-anchor="end" font-family="{MONO}" font-size="12" font-weight="700" '
            f'fill="{p["red"] if r["rank"] == 1 else p["ink"]}">#{r["rank"]} of {r["of"]}</text>',
            f'<rect x="{x_bar}" y="{y + 8}" width="{bar_w}" height="8" fill="{p["track"]}"/>',
            f'<rect x="{x_bar}" y="{y + 8}" width="{max(1.0, bar_w * r["share"]):.1f}" height="8" fill="{p["red"]}"/>',
            f'<rect x="{tick - 0.75:.1f}" y="{y + 5}" width="1.5" height="14" fill="{p["ink"]}"/>',
            f'<text x="20" y="{y + 34}" font-family="{MONO}" font-size="11" fill="{p["muted"]}">'
            f'#{r["other"]["rank"]} {escape(r["other"]["name"])} {pct(r["other"]["share"])}</text>',
        ]
    y = top + step * (len(rows) - 1) + 50
    out += [
        f'<line x1="20" y1="{y + 0.5}" x2="{w - 20}" y2="{y + 0.5}" stroke="{p["rule"]}"/>',
        f'<text x="20" y="{y + 22}" font-family="{MONO}" font-size="10.5" fill="{p["muted"]}">'
        f"kuna {escape(version)} \u00b7 bar: kuna \u00b7 tick: the entry named</text>",
        f'<text x="20" y="{y + 38}" font-family="{MONO}" font-size="10.5" fill="{p["muted"]}">'
        f"source: decbench.com, generated {date}</text>",
        "</svg>",
    ]
    return "".join(out) + "\n"


def summary(rows: list, date: str) -> str:
    parts = [f'{r["dataset"]}: {pct(r["share"])}, rank {r["rank"]} of {r["of"]}' for r in rows]
    return f"kuna on DecBench, Union, scoreboard of {date}. " + "; ".join(parts)


def load(path: str | None) -> dict:
    if path:
        return json.loads(Path(path).read_text())
    req = urllib.request.Request(URL, headers={"User-Agent": "kuna-decbench-card"})
    with urllib.request.urlopen(req, timeout=60) as res:
        return json.load(res)


def main(argv: list[str] | None = None) -> int:
    ap = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    ap.add_argument("--json", help="read a saved aggregates.json instead of fetching it")
    ap.add_argument("--out", default=str(OUT), help="directory for decbench-card[-narrow]-{light,dark}.svg")
    args = ap.parse_args(argv)
    agg = load(args.json)
    out = Path(args.out)
    for name, palette in PALETTES.items():
        (out / f"decbench-card-{name}.svg").write_text(render(agg, palette))
        (out / f"decbench-card-narrow-{name}.svg").write_text(render_narrow(agg, palette))
    rows = [standings(agg, d) for d in DATASETS if f"{d}|0" in agg["combos"]]
    print(summary(rows, str(agg.get("generated_at", "?"))[:10]))
    return 0


if __name__ == "__main__":
    sys.exit(main())
