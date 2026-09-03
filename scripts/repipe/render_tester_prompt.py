"""Render tools/repipe/tester_prompt.md for one challenge.

Kept out of tester.sh because three of the placeholders are computed, not substituted:
the newly-closed needs the tester is asked to re-exercise, the already-filed needs it must
NOT write up again, and the per-challenge time budget derived from the binary's size.

The KNOWN_NEEDS block is the cheapest duplicate-suppression the pipeline has. Without it,
round 4's three testers each write a fresh essay about kuna having no xrefs; with it they
note the hit and move on, and the need's `instances` count does the talking.
"""
from __future__ import annotations

import argparse
import json
import os
import sys
from pathlib import Path

from . import config

TOKENS_DOC = "{{KUNA}} and {{BIN}}"


def _meta(hexid):
    p = config.dataset_root() / "challenges" / hexid / "meta.json"
    with open(p) as fh:
        return json.load(fh)


def _time_budget_minutes(meta):
    """Sized from the binary, because kuna's cost is dominated by whole-binary work.

    The `auto` mode cutovers are 512,000 and 2,097,152 bytes; the worst measured case on
    this machine is 445 s at 1,476 MiB RSS for a 3.4 MB PE, so a big target needs a bigger
    budget or the tester spends it all in one decompile-all.
    """
    size = (meta.get("detected", {}).get("primary") or {}).get("size", 0)
    base = config.TESTER_TIMEOUT // 60
    if size >= 2_097_152:
        return base
    if size >= 512_000:
        return max(20, int(base * 0.8))
    return max(15, int(base * 0.6))


def _recently_shipped(rounds_back=1):
    try:
        from . import needs as needs_mod
    except Exception:
        return ""
    try:
        closed = [n for n in needs_mod.load_all() if getattr(n, "status", None) == "closed"]
    except Exception:
        return ""
    if not closed:
        return ""
    lines = ["## Newly available capabilities — exercise them",
             "",
             "These gaps were closed since the last round. Try them, and if one does not",
             "behave as described that is a **regression**: file it with",
             "`regression_of: <need_id>`, the highest-priority class we accept.",
             ""]
    for n in closed[:8]:
        # Render the CAPABILITY, not the defect title. A closed need's title states the
        # problem ("kuna cannot list strings"); printing that under a heading that says
        # "newly available" tells the tester the opposite of the truth. The builder
        # records what shipped as a "Shipped:" line in the Acceptance section.
        lines.append("- %s" % (_shipped_line(n) or ("`%s` (closed need `%s`)"
                                                    % (n.fields.get("title", n.need_id), n.need_id))))
    return "\n".join(lines)


def _shipped_line(need):
    """The `Shipped: ...` line a builder left in the record's Acceptance section, if any."""
    body = ""
    try:
        body = (need.sections or {}).get("Acceptance", "") or ""
    except Exception:
        return ""
    for line in body.split("\n"):
        marker = "Shipped:"
        if marker in line:
            text = line.split(marker, 1)[1].strip()
            if text:
                return "%s  _(closed need `%s`)_" % (text, need.fields.get("need_id", "?"))
    return ""


def _known_needs(limit=12):
    try:
        from . import needs as needs_mod
    except Exception:
        return ""
    try:
        open_needs = [n for n in needs_mod.load_all()
                      if getattr(n, "status", "open") in ("open", "claimed", "building")]
    except Exception:
        return ""
    if not open_needs:
        return ""
    try:
        open_needs.sort(key=needs_mod.rank_score, reverse=True)
    except Exception:
        pass
    lines = ["## Already filed — do NOT write these up again",
             "",
             "If you hit one of these, say so in one line in `what_kuna_did` and move on.",
             "Re-filing it does not help; the pipeline counts your hit either way.",
             ""]
    for n in open_needs[:limit]:
        lines.append("- `%s` — %s" % (n.need_id, getattr(n, "title", "")))
    return "\n".join(lines)


def _obfuscation_line(meta):
    """What this challenge is actually doing to you, from the corpus labels.

    Named rather than left abstract because "this binary is virtualised and string-encrypted"
    changes which interface a tester should ask for: a VM wants control-flow structuring
    control, encrypted strings want a way to define decoded data.
    """
    classes = (meta.get("obfuscation") or {}).get("classes") or []
    if not classes:
        return ("this one is not labelled as obfuscated, so a wrong answer here is a plain "
                "bug rather than a defence")
    return "this one is labelled " + ", ".join(c.lower() for c in classes)


def _ida_line():
    if not config.ENABLE_IDA:
        return "- IDA is **not** available this round. kuna and the binutils are all you have."
    return (
        "- `ida-decompile` is IDA Pro 9.2 via declib, and it is a **last resort**, not a\n"
        "  parallel track. Every call is logged. When you use it, you must record what you\n"
        "  wanted from kuna and why kuna could not give it to you — that comparison is the\n"
        "  single most useful thing you can produce."
    )


def render(hexid, round_n, arena, out=None):
    meta = _meta(hexid)
    primary = meta.get("detected", {}).get("primary") or {}
    target = os.path.basename(primary.get("path", "target"))
    tmpl = (config.repo_root() / "tools" / "repipe" / "tester_prompt.md").read_text()
    body = (tmpl
            .replace("{{ARENA}}", str(arena))
            .replace("{{ROUND}}", str(round_n))
            .replace("{{HEXID}}", hexid)
            .replace("{{TARGET}}", target)
            .replace("{{REPO}}", str(config.repo_root()))
            .replace("{{TIME_BUDGET}}", str(_time_budget_minutes(meta)))
            .replace("{{OBFUSCATION}}", _obfuscation_line(meta))
            .replace("{{IDA_LINE}}", _ida_line())
            .replace("{{RECENTLY_SHIPPED}}", _recently_shipped())
            .replace("{{KNOWN_NEEDS}}", _known_needs()))
    if out:
        tmp = str(out) + ".tmp"
        Path(tmp).write_text(body)
        os.replace(tmp, out)
    return body


def main(argv=None):
    ap = argparse.ArgumentParser(prog="python -m scripts.repipe.render_tester_prompt")
    ap.add_argument("--hexid", required=True)
    ap.add_argument("--round", type=int, required=True)
    ap.add_argument("--arena", required=True)
    ap.add_argument("--out", default=None)
    args = ap.parse_args(argv)
    body = render(args.hexid, args.round, args.arena, args.out)
    if not args.out:
        sys.stdout.write(body)
    return 0


if __name__ == "__main__":
    sys.exit(main())
