#!/usr/bin/env python3
"""Standalone angr decompilation runner.

This file is executed by the ANGR interpreter ($KUNA_PIPELINE_ANGR_PYTHON, e.g. the
decbench venv's python) -- NOT by the kuna venv (which has no angr). It imports only the
stdlib + angr, and emits exactly one JSON object on stdout with the decompiled C text.
All angr / cle / loader log noise is silenced and kept off stdout so the parent can parse
stdout as pure JSON (mirrors how scripts.decompile isolates decomp_dbg's `print C`).

Usage:
  $ANGR_PYTHON _angr_runner.py <binary> <selector> [--arch ARCH] [--load-debug]
                                                    [--structurer NAME]
  selector: a function name, a 0x-hex address, or the literal @entry
"""
import argparse
import json
import logging
import sys


def _silence_logging():
    # angr/cle/pyvex are chatty; route everything away from stdout.
    logging.disable(logging.CRITICAL)
    try:
        for nm in list(logging.root.manager.loggerDict):
            logging.getLogger(nm).setLevel(logging.CRITICAL)
            logging.getLogger(nm).propagate = False
    except Exception:
        pass


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("binary")
    ap.add_argument("selector")
    ap.add_argument("--arch", default=None)
    ap.add_argument("--load-debug", action="store_true")
    ap.add_argument("--structurer", default=None)
    args = ap.parse_args()

    _silence_logging()

    out = {
        "binary": args.binary, "selector": args.selector, "arch": args.arch,
        "source_decompiler": "angr", "version": None,
        "text": None, "func_name": None, "func_addr": None, "error": None,
    }
    try:
        import angr
        from angr.analyses import CFGFast, Decompiler

        out["version"] = getattr(angr, "__version__", None)

        kw = {"auto_load_libs": False, "load_debug_info": bool(args.load_debug)}
        if args.arch:
            kw["arch"] = args.arch
        proj = angr.Project(args.binary, **kw)
        cfg = proj.analyses[CFGFast].prep()(normalize=True, data_references=True)

        sel = args.selector
        if sel == "@entry":
            key = proj.entry
        elif sel.lower().startswith("0x"):
            key = int(sel, 16)
        else:
            key = sel
        f = cfg.functions[key]
        out["func_name"] = f.name
        out["func_addr"] = "0x%x" % f.addr

        options = None
        if args.structurer:
            try:
                from angr.analyses.decompiler.decompilation_options import (
                    get_structurer_option,
                )
                options = [(get_structurer_option(), args.structurer)]
            except Exception:
                options = None  # fall back to angr's default (sailr)

        dec = proj.analyses[Decompiler].prep(fail_fast=True)(
            f, cfg=cfg.model, options=options
        )
        if dec.codegen is None:
            out["error"] = "decompilation produced no codegen"
        else:
            out["text"] = dec.codegen.text
    except Exception as e:  # noqa: BLE001 -- report everything as a JSON error
        import traceback
        out["error"] = "%s: %s" % (type(e).__name__, e)
        out["traceback"] = traceback.format_exc()[-2000:]

    sys.stdout.write(json.dumps(out))
    sys.stdout.flush()


if __name__ == "__main__":
    main()
