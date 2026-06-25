"""Measure a feature option's decompile-speed impact (requirement: speed is critical).

Every shipped feature must record what its option costs at decompile time. We time the
feature's target function decompiled with the option OFF vs ON, ``repeat`` times each, and
report the **median** of each plus ``delta_pct = (on - off) / off * 100``.

Why the delta is the honest number: each ``kuna decompile`` is a fresh ``decomp_dbg``
subprocess whose wall-clock includes process spawn + ``load file`` + ``read symbols`` +
SLEIGH spec mmap, which dwarf the decompile step. But OFF and ON share that exact fixed
cost, so subtracting them cancels load and isolates the option's marginal cost -- with no
engine change. The absolute medians are reported too (for scale), with the caveat that they
include the fixed load.

The gate (applied by the worker, see tools/pipeline/worker_prompt.md): a feature that the
0/675 ablation would let ship default-ON, but which regresses its target beyond
``config.SPEED_BUDGET_PCT`` (default +5%), is held to default-OFF opt-in instead. The speed
gate can only ever push default-ON -> opt-in; it never re-pins docs/baseline.json.

    python -m scripts.pipeline.timeit --option <opt> --binary B --selector main [--func-addr 0x..]
    python -m scripts.pipeline.timeit --record docs/features/<slug>/record.json   # splices fields back
"""
from __future__ import annotations

import argparse
import json
import statistics
import sys
import time

from . import config
from .. import decompile as kdecompile

# The record.json fields this module owns (always written, even when within budget).
SPEED_FIELDS = (
    "speed_off_ms", "speed_on_ms", "speed_delta_pct", "speed_samples",
    "speed_budget_pct", "speed_within_budget", "speed_forced_off",
)


def _decompile_once(binary, selector, func_addr, option, value, *, timeout):
    """Wall-clock one decompile with the option flipped; return elapsed ms, or None on failure.

    Name-first, address-fallback selection (mirrors prdemo._kuna), but timed rather than
    captured. A failed decompile yields None so it is excluded from the median rather than
    silently counted as fast.
    """
    opts = [(option, value)]
    name_sel = (
        selector
        and not str(selector).lower().startswith("0x")
        and selector != "@entry"
    )
    if name_sel:
        t0 = time.perf_counter()
        try:
            kdecompile.decompile(binary, selector, options=opts, timeout=timeout)
            return (time.perf_counter() - t0) * 1000.0
        except kdecompile.DecompileError:
            pass  # fall through to the address form
    if func_addr:
        t0 = time.perf_counter()
        try:
            kdecompile.decompile(binary, str(func_addr), by_address=True,
                                 options=opts, timeout=timeout)
            return (time.perf_counter() - t0) * 1000.0
        except kdecompile.DecompileError:
            return None
    return None


def time_decompile(binary, selector, func_addr, option, value, *,
                   repeat=None, timeout=None):
    """Return up to ``repeat`` wall-clock ms samples for one (option, value) setting."""
    repeat = config.SPEED_REPEAT if repeat is None else repeat
    timeout = config.KUNA_TIMEOUT if timeout is None else timeout
    samples = []
    for _ in range(repeat):
        ms = _decompile_once(binary, selector, func_addr, option, value, timeout=timeout)
        if ms is not None:
            samples.append(ms)
    return samples


def _median(samples):
    return round(statistics.median(samples), 2) if samples else None


def measure(record=None, *, option=None, binary=None, selector=None, func_addr=None,
            repeat=None, warmup=1, budget_pct=None):
    """Measure off/on decompile timing for a feature and return the speed_* record block.

    Accepts a loaded ``record`` dict (reads option/binary/selector/func_addr from it) or the
    explicit kwargs. Returns a dict of the SPEED_FIELDS (``speed_forced_off`` left False here;
    the worker sets it when the ablation was clean but this gate failed).
    """
    if record is not None:
        option = option or record.get("option")
        binary = binary or record.get("binary")
        selector = selector or record.get("selector")
        func_addr = func_addr or record.get("func_addr")
    repeat = config.SPEED_REPEAT if repeat is None else repeat
    budget_pct = config.SPEED_BUDGET_PCT if budget_pct is None else budget_pct

    # Warm up (discarded): settle page cache + spec mmap so the first timed run isn't an outlier.
    for _ in range(max(0, warmup)):
        _decompile_once(binary, selector, func_addr, option, "off", timeout=config.KUNA_TIMEOUT)

    off = time_decompile(binary, selector, func_addr, option, "off", repeat=repeat)
    on = time_decompile(binary, selector, func_addr, option, "on", repeat=repeat)
    off_ms, on_ms = _median(off), _median(on)

    if off_ms and on_ms:
        delta = round((on_ms - off_ms) / off_ms * 100.0, 2)
    else:
        delta = None  # unmeasured -> the gate treats it conservatively (not within budget)

    within = gate(delta, budget_pct=budget_pct)["within_budget"]
    return {
        "speed_off_ms": off_ms,
        "speed_on_ms": on_ms,
        "speed_delta_pct": delta,
        "speed_samples": repeat,
        "speed_budget_pct": budget_pct,
        "speed_within_budget": within,
        "speed_forced_off": False,
    }


def gate(speed_delta_pct, *, budget_pct=None):
    """Pure gate: within budget iff the delta is measured and <= budget. Unmeasured -> False."""
    budget_pct = config.SPEED_BUDGET_PCT if budget_pct is None else budget_pct
    within = speed_delta_pct is not None and speed_delta_pct <= budget_pct
    return {"within_budget": within, "budget_pct": budget_pct}


def _summary(block):
    off, on, d = block["speed_off_ms"], block["speed_on_ms"], block["speed_delta_pct"]
    verdict = "within budget" if block["speed_within_budget"] else "OVER BUDGET / unmeasured"
    return ("speed: off=%s ms  on=%s ms  delta=%s%%  (budget %s%%) -> %s"
            % (off, on, d, block["speed_budget_pct"], verdict))


def main(argv=None):
    p = argparse.ArgumentParser(prog="python -m scripts.pipeline.timeit",
                                description="Measure a feature option's decompile-speed impact.")
    p.add_argument("--record", default=None, help="feature record.json (splices speed_* back in)")
    p.add_argument("--option", default=None)
    p.add_argument("--binary", default=None)
    p.add_argument("--selector", default=None)
    p.add_argument("--func-addr", default=None)
    p.add_argument("--repeat", type=int, default=None)
    p.add_argument("--budget-pct", type=float, default=None)
    args = p.parse_args(argv)

    if args.record:
        with open(args.record) as fh:
            rec = json.load(fh)
        block = measure(rec, option=args.option, binary=args.binary, selector=args.selector,
                        func_addr=args.func_addr, repeat=args.repeat, budget_pct=args.budget_pct)
        rec.update(block)
        tmp = args.record + ".tmp"
        with open(tmp, "w") as fh:
            json.dump(rec, fh, indent=2)
        import os
        os.replace(tmp, args.record)
        print(json.dumps(block, indent=2))
        print(_summary(block), file=sys.stderr)
        return 0

    if not (args.option and args.binary and args.selector):
        p.error("need --record, or --option --binary --selector")
    block = measure(option=args.option, binary=args.binary, selector=args.selector,
                    func_addr=args.func_addr, repeat=args.repeat, budget_pct=args.budget_pct)
    print(json.dumps(block, indent=2))
    print(_summary(block), file=sys.stderr)
    return 0


if __name__ == "__main__":
    sys.exit(main())
