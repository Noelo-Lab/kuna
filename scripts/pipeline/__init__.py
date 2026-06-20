"""scripts.pipeline -- the continuous, reference-decompiler-inspired feature loop.

This subpackage turns the hand-driven "study angr, find where it is better, port one
improvement at a time" methodology (the loweredswitch work) into an observable,
mostly-autonomous pipeline:

    worklist  -> enumerate a reference decompiler's testcases as (binary, function) targets
    compare   -> for one target, diff the reference decompilation against kuna's
    sweep     -> run compare across a corpus, rank where the reference is better
    worker    -> implement ONE option-gated kuna feature closing a gap, open a PR
    state/status -> observe the running loop (how many workers, what phase, which PR)

The reference side is abstracted (`scripts.pipeline.reference`) so angr can later be joined
or replaced by reko / Binary Ninja / IDA without touching compare/sweep/worker.

Config is environment-driven (see `scripts.pipeline.config`); the defaults target this
machine's angr checkout + the installed angr 9.2.213 runtime (decbench venv).
"""

from . import config  # noqa: F401

__all__ = ["config"]
