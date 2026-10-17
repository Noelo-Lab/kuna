"""Render the observatory pages: each template in pages/ becomes one
self-contained HTML file with the shared CSS/JS and its data JSON inlined
(file:// pages cannot fetch), plus a status object naming which datasets
exist so every page can link its siblings and degrade gracefully."""
import json
from pathlib import Path

from scripts.observatory import common

PAGES_DIR = Path(__file__).parent / "pages"

# page name -> datasets inlined into it; each later workstream adds its row.
PAGES = {
    "index": [],
    "atlas": ["atlas", "coverage"],
    "churn": ["churn"],
    "coverage": ["coverage"],
    "walkthrough": ["walkthrough"],
}

# dataset -> nav/launcher card, for the status object every page embeds.
DATASETS = {
    "atlas": {
        "title": "Architecture atlas",
        "nav": "Atlas",
        "blurb": "The P0–P9 pipeline, each phase's modules, options and spec chapter, "
        "the crate graph, and the real pass schedule.",
        "regen": "make observatory",
    },
    "coverage": {
        "title": "Test coverage",
        "nav": "Coverage",
        "blurb": "Merged line coverage from the workspace suite, the datatest/stages "
        "corpora and the CLI probes, per crate and per phase folder.",
        "regen": "make coverage",
    },
    "walkthrough": {
        "title": "Bytes \u2192 C walkthrough",
        "nav": "Walkthrough",
        "blurb": "One real function traced through the engine: bytes, disassembly, "
        "raw p-code, final IR, region tree, C.",
        "regen": "make observatory",
    },
    "churn": {
        "title": "Churn & ownership",
        "nav": "Churn",
        "blurb": "A treemap of where change concentrates: live file size × windowed "
        "churn × who changed it (agents vs people).",
        "regen": "make observatory",
    },
}


def _status(data_dir: Path) -> dict:
    datasets = {}
    for name, info in DATASETS.items():
        path = data_dir / f"{name}.json"
        entry = {"page": f"{name}.html", "available": path.is_file(), **info}
        if entry["available"]:
            entry["meta"] = json.loads(path.read_text(encoding="utf-8")).get("meta", {})
        datasets[name] = entry
    return {"generated": common.utc_now(), "datasets": datasets}


def _data_tag(name: str, payload) -> str:
    return f'<script type="application/json" id="data-{name}">{common.inline_json(payload)}</script>'


def render(out_dir: Path) -> list:
    data_dir = out_dir / "data"
    status = _status(data_dir)
    base_css = (PAGES_DIR / "base.css").read_text(encoding="utf-8")
    base_js = (PAGES_DIR / "base.js").read_text(encoding="utf-8")

    written = []
    for page, datasets in PAGES.items():
        html = (PAGES_DIR / f"{page}.html").read_text(encoding="utf-8")
        html = html.replace("@@BASE_CSS@@", base_css).replace("@@BASE_JS@@", base_js)
        tags = [_data_tag("status", status)]
        for name in datasets:
            path = data_dir / f"{name}.json"
            if path.is_file():
                tags.append(_data_tag(name, json.loads(path.read_text(encoding="utf-8"))))
        html = html.replace("@@DATA@@", "\n".join(tags))
        target = out_dir / f"{page}.html"
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_text(html, encoding="utf-8")
        written.append(target)
    return written
