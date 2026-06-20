"""Generate a real before/after demonstration for a feature's PR body.

Given a feature record (docs/features/<slug>/record.json), this captures the ACTUAL kuna
decompilation of the feature's target function with the option off (before) and on (after),
plus the reference decompiler's rendering of the same function (the goal), and emits a
markdown block the pipeline embeds in the PR's opening comment. Real captured output, not a
hand-written sketch -- so every PR demonstrably shows what the feature does on a relevant
target.

    python -m scripts.pipeline.prdemo --record docs/features/<slug>/record.json
    python -m scripts.pipeline.prdemo --option stackguard --binary B --selector authenticate
"""
from __future__ import annotations

import argparse
import json
import os
import sys

from . import config
from .compare import metrics
from .. import decompile as kdecompile

_FENCE = "```"


def _kuna(binary, selector, func_addr, option, value):
    """Decompile in kuna with one option flipped; name-first, address-fallback."""
    opts = [(option, value)]
    name_sel = selector and not str(selector).lower().startswith("0x") and selector != "@entry"
    if name_sel:
        try:
            return kdecompile.decompile(binary, selector, options=opts,
                                        timeout=config.KUNA_TIMEOUT)
        except kdecompile.DecompileError:
            pass
    if func_addr:
        try:
            return kdecompile.decompile(binary, func_addr, by_address=True, options=opts,
                                        timeout=config.KUNA_TIMEOUT)
        except kdecompile.DecompileError as e:
            return "// kuna decompilation failed: %s" % e
    return "// kuna decompilation unavailable (no name/address)"


def _angr(binary, selector, test_name):
    """Best-effort reference (angr) rendering of the same function, for the 'goal' block."""
    try:
        from .reference import get_reference
        ref = get_reference("angr")
        if not ref.available():
            return None, None
        arch = load_debug = None
        try:  # recover arch/load_debug from the worklist if we can
            from .worklist import build_worklist
            for e in build_worklist():
                if e.test_name == test_name:
                    arch, load_debug = e.arch, e.load_debug
                    break
        except Exception:
            pass
        rr = ref.decompile(binary, selector, arch=arch, load_debug=bool(load_debug),
                           structurer=config.DEFAULT_STRUCTURER or None)
        if rr.ok:
            return rr.text, rr.version
    except Exception:
        pass
    return None, None


def _metric_row(name, before, after):
    b, a = before.get(name, 0), after.get(name, 0)
    arrow = ""
    if b != a:
        arrow = " ⬇️" if a < b else " ⬆️"
    return "| %s | %s | %s%s |" % (name, b, a, arrow)


def generate(record=None, *, option=None, binary=None, selector=None, func_addr=None,
             default_on=False, source_decompiler="angr", inspiration="", test_name="",
             with_angr=True):
    if record is not None:
        option = record.get("option")
        binary = record.get("binary")
        selector = record.get("selector")
        func_addr = record.get("func_addr")
        default_on = bool(record.get("default_on"))
        source_decompiler = record.get("source_decompiler", "angr")
        inspiration = record.get("inspiration", "")
        test_name = record.get("test_name", "")

    func = selector
    binbase = os.path.basename(binary) if binary else "?"

    before = _kuna(binary, selector, func_addr, option, "off")
    after = _kuna(binary, selector, func_addr, option, "on")
    mb, ma = metrics(before), metrics(after)

    default_lbl_off = " *(default)*" if not default_on else ""
    default_lbl_on = " *(default)*" if default_on else ""

    out = []
    out.append("## Before / after — `%s` in `%s`" % (func, binbase))
    out.append("")
    out.append("Real captured kuna output on the function this feature was built for, with "
               "`option %s` flipped. Ported from **%s**%s."
               % (option, source_decompiler,
                  (" — %s" % inspiration) if inspiration else ""))
    out.append("")
    out.append("| metric | `%s off`%s | `%s on`%s |" % (option, default_lbl_off, option, default_lbl_on))
    out.append("|---|---|---|")
    for m in ("gotos", "labels", "loc", "switches"):
        out.append(_metric_row(m, mb, ma))
    out.append("")
    out.append("<details open><summary><b>kuna — before (<code>option %s off</code>)</b></summary>\n" % option)
    out.append("%sc\n%s\n%s" % (_FENCE, before.strip(), _FENCE))
    out.append("\n</details>")
    out.append("")
    out.append("<details open><summary><b>kuna — after (<code>option %s on</code>)</b></summary>\n" % option)
    out.append("%sc\n%s\n%s" % (_FENCE, after.strip(), _FENCE))
    out.append("\n</details>")

    if with_angr:
        atext, aver = _angr(binary, selector, test_name)
        if atext:
            out.append("")
            out.append("<details><summary><b>%s reference (the goal, v%s)</b></summary>\n"
                       % (source_decompiler, aver or "?"))
            out.append("%sc\n%s\n%s" % (_FENCE, atext.strip(), _FENCE))
            out.append("\n</details>")
    out.append("")
    return "\n".join(out)


def main(argv=None):
    p = argparse.ArgumentParser(prog="python -m scripts.pipeline.prdemo",
                                description="Generate a before/after PR demo for a feature.")
    p.add_argument("--record", default=None, help="feature record.json")
    p.add_argument("--option", default=None)
    p.add_argument("--binary", default=None)
    p.add_argument("--selector", default=None)
    p.add_argument("--func-addr", default=None)
    p.add_argument("--default-on", action="store_true")
    p.add_argument("--no-angr", action="store_true")
    args = p.parse_args(argv)

    if args.record:
        with open(args.record) as fh:
            rec = json.load(fh)
        print(generate(rec, with_angr=not args.no_angr))
        return 0
    if not (args.option and args.binary and args.selector):
        p.error("need --record, or --option --binary --selector")
    print(generate(option=args.option, binary=args.binary, selector=args.selector,
                   func_addr=args.func_addr, default_on=args.default_on,
                   with_angr=not args.no_angr))
    return 0


if __name__ == "__main__":
    sys.exit(main())
