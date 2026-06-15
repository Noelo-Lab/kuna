"""Compare a reference decompiler against kuna on the same (binary, function).

Given a target, this fetches the reference decompilation (angr by default) and kuna's
decompilation of the SAME function, computes structural readability metrics on both, and
emits a set of machine signals describing where the reference looks better. The final
"is the reference actually better, and what kuna feature would close the gap" judgment is
made by an LLM in ``rank.py`` / the worker -- this module only produces the evidence.

Because kuna renders address-mode functions as ``sub_<addr>`` and uses different variable
names than angr, we compare by *structure* (gotos, labels, switch recovery, loop/branch
shape, line count), never by raw text equality.

    python -m kuna.pipeline.compare <binary> <selector> [--arch A] [--load-debug] [--reference angr]
    python -m kuna.pipeline.compare --entry <test_name>     # by worklist test name
"""
from __future__ import annotations

import argparse
import json
import os
import re
import sys
from dataclasses import dataclass, field, asdict
from typing import Optional

from . import config
from .reference import get_reference
from .. import decompile as kdecompile


# --- structural metrics -----------------------------------------------------

def metrics(text: str) -> dict:
    """Structural readability metrics computed identically on any C text."""
    if not text:
        return {"loc": 0, "gotos": 0, "labels": 0, "switches": 0, "cases": 0,
                "ifs": 0, "loops": 0, "ternaries": 0, "casts": 0}
    lines = [ln for ln in text.splitlines() if ln.strip()]
    # label definitions like "LAB_00401000:" or "label_4:" (not case/default/struct)
    label_def = re.compile(r"^\s*[A-Za-z_]\w*:\s*$")
    labels = sum(1 for ln in lines
                 if label_def.match(ln) and not ln.strip().startswith(("case ", "default")))
    return {
        "loc": len(lines),
        "gotos": len(re.findall(r"\bgoto\b", text)),
        "labels": labels,
        "switches": len(re.findall(r"\bswitch\s*\(", text)),
        "cases": len(re.findall(r"\bcase\b", text)),
        "ifs": len(re.findall(r"\bif\s*\(", text)),
        "loops": len(re.findall(r"\b(?:while|for)\s*\(", text)),
        "ternaries": len(re.findall(r"\?", text)),
        "casts": len(re.findall(r"\bundefined\d*\b", text)),  # Ghidra-ism proxy
    }


def signals(ref_m: dict, kuna_m: dict, ref_text: str, kuna_text: str) -> list:
    """Heuristic, machine-readable hints about where the reference looks better.

    These are *candidates* for an LLM to confirm -- deliberately conservative and
    structural. Each is a short tag plus a magnitude.
    """
    out = []
    if kuna_m["gotos"] > ref_m["gotos"]:
        out.append("ref has fewer gotos (%d vs %d)" % (ref_m["gotos"], kuna_m["gotos"]))
    if kuna_m["labels"] > ref_m["labels"]:
        out.append("ref has fewer labels (%d vs %d)" % (ref_m["labels"], kuna_m["labels"]))
    if ref_m["switches"] > kuna_m["switches"]:
        out.append("ref recovered a switch kuna did not (%d vs %d)"
                   % (ref_m["switches"], kuna_m["switches"]))
    if ref_m["loops"] > kuna_m["loops"]:
        out.append("ref recovered more loops (%d vs %d)" % (ref_m["loops"], kuna_m["loops"]))
    # meaningfully shorter (>15%) usually means better structuring, not lost code
    if kuna_m["loc"] and ref_m["loc"] and ref_m["loc"] < 0.85 * kuna_m["loc"]:
        out.append("ref is %d%% shorter (%d vs %d loc)"
                   % (round(100 * (1 - ref_m["loc"] / kuna_m["loc"])), ref_m["loc"], kuna_m["loc"]))
    if kuna_m["casts"] > ref_m["casts"] + 2:
        out.append("kuna has more undefined/cast noise (%d vs %d)"
                   % (kuna_m["casts"], ref_m["casts"]))
    # Ghidra control-flow-failure markers
    if kuna_text and ("Could not" in kuna_text or "WARNING:" in kuna_text
                      or "halt_baddata" in kuna_text):
        out.append("kuna emitted a recovery-failure marker")
    return out


@dataclass
class ComparisonResult:
    binary: str
    selector: str
    reference: str
    test_name: str = ""
    func_name: Optional[str] = None
    func_addr: Optional[str] = None
    ref_version: Optional[str] = None
    ref_text: Optional[str] = None
    ref_error: Optional[str] = None
    kuna_text: Optional[str] = None
    kuna_error: Optional[str] = None
    kuna_mode: str = ""               # "name" | "addr"
    kuna_engine: str = ""             # which kuna port ran under test ("cpp" | "rust")
    ref_metrics: dict = field(default_factory=dict)
    kuna_metrics: dict = field(default_factory=dict)
    signals: list = field(default_factory=list)

    @property
    def comparable(self) -> bool:
        return bool(self.ref_text) and bool(self.kuna_text)

    def to_dict(self) -> dict:
        return asdict(self)


def _kuna_decompile(binary, selector, ref_addr, engine=None):
    """Decompile in kuna by name when the selector is a name, else by address.

    ``engine`` (``"cpp"`` / ``"rust"`` / None) selects which kuna port runs as the
    decompiler-under-test; None uses the ambient default (``KUNA_ENGINE``, else cpp).
    Returns (text, error, mode). Falls back name->addr when a name lookup fails.
    """
    name_sel = selector and not selector.lower().startswith("0x") and selector != "@entry"
    if name_sel:
        try:
            return kdecompile.decompile(binary, selector, engine=engine,
                                        timeout=config.KUNA_TIMEOUT), None, "name"
        except kdecompile.DecompileError as e:
            # fall through to address mode using the reference's resolved entry
            name_err = str(e)
    else:
        name_err = None
    if ref_addr:
        try:
            txt = kdecompile.decompile(binary, ref_addr, by_address=True, engine=engine,
                                       timeout=config.KUNA_TIMEOUT)
            return txt, None, "addr"
        except kdecompile.DecompileError as e:
            return None, "addr-mode: %s%s" % (e, (" (name-mode: %s)" % name_err) if name_err else ""), "addr"
    return None, name_err or "no address available for kuna", "name"


def compare(binary, selector, *, reference="angr", arch=None, load_debug=False,
            structurer=None, test_name="", kuna_engine=None) -> ComparisonResult:
    """Compare ``reference`` against the kuna engine under test on one function.

    ``reference`` is any registered adapter -- ``angr`` (default), or a kuna engine
    itself (``kuna-cpp`` / ``kuna-rust``) to produce a pure cross-port differential.
    ``kuna_engine`` (``"cpp"`` / ``"rust"`` / None) chooses which kuna port runs as the
    decompiler-under-test; e.g. ``reference="kuna-cpp", kuna_engine="rust"`` is the W11
    kuna-rust-vs-kuna-cpp comparison row on a real ELF.
    """
    ref = get_reference(reference)
    rr = ref.decompile(binary, selector, arch=arch, load_debug=load_debug,
                       structurer=structurer or (config.DEFAULT_STRUCTURER or None))
    res = ComparisonResult(
        binary=rr.binary, selector=selector, reference=reference, test_name=test_name,
        func_name=rr.func_name, func_addr=rr.func_addr, ref_version=rr.version,
        ref_text=rr.text, ref_error=rr.error,
        kuna_engine=kuna_engine or os.environ.get("KUNA_ENGINE") or "cpp",
    )
    if not rr.ok:
        return res

    kuna_text, kuna_err, mode = _kuna_decompile(binary, selector, rr.func_addr,
                                                engine=kuna_engine)
    res.kuna_text, res.kuna_error, res.kuna_mode = kuna_text, kuna_err, mode

    res.ref_metrics = metrics(rr.text)
    res.kuna_metrics = metrics(kuna_text) if kuna_text else {}
    if res.comparable:
        res.signals = signals(res.ref_metrics, res.kuna_metrics, rr.text, kuna_text)
    return res


def _print_human(res: ComparisonResult):
    print("=" * 78)
    print("%s :: %s   (%s %s @ %s)" % (
        res.test_name or res.binary.split("/")[-1], res.selector,
        res.reference, res.ref_version, res.func_addr))
    print("=" * 78)
    print("\n--- %s ---" % res.reference)
    if res.ref_text:
        print(res.ref_text)
    else:
        print("  (no output: %s)" % res.ref_error)
    print("\n--- kuna-%s (%s mode) ---" % (res.kuna_engine or "cpp", res.kuna_mode))
    if res.kuna_text:
        print(res.kuna_text)
    else:
        print("  (no output: %s)" % res.kuna_error)
    if res.comparable:
        print("\n--- metrics (reference | kuna) ---")
        for k in res.ref_metrics:
            print("  %-12s %4s | %-4s" % (k, res.ref_metrics[k], res.kuna_metrics.get(k, "-")))
        print("\n--- signals (candidate 'reference is better' hints) ---")
        if res.signals:
            for s in res.signals:
                print("  * %s" % s)
        else:
            print("  (none -- structurally comparable)")


def main(argv=None):
    p = argparse.ArgumentParser(prog="python -m kuna.pipeline.compare",
                                description="Compare a reference decompiler vs kuna on one function.")
    p.add_argument("binary", nargs="?", help="binary path")
    p.add_argument("selector", nargs="?", help="function name / 0xaddr / @entry")
    p.add_argument("--entry", default=None, help="look the target up in the worklist by test name")
    p.add_argument("--reference", default="angr",
                   help="reference decompiler: angr (default), or a kuna engine "
                        "(kuna-cpp / kuna-rust) for a cross-port differential")
    p.add_argument("--kuna-engine", dest="kuna_engine", choices=("cpp", "rust"),
                   default=None,
                   help="which kuna port runs as the decompiler-under-test "
                        "(default: KUNA_ENGINE, else cpp). e.g. "
                        "`--reference kuna-cpp --kuna-engine rust` is the W11 "
                        "kuna-rust-vs-kuna-cpp comparison on a real ELF")
    p.add_argument("--arch", default=None)
    p.add_argument("--load-debug", action="store_true")
    p.add_argument("--structurer", default=None)
    p.add_argument("--json", action="store_true", help="emit the ComparisonResult as JSON")
    args = p.parse_args(argv)

    arch, load_debug, test_name = args.arch, args.load_debug, ""
    binary, selector = args.binary, args.selector
    if args.entry:
        from .worklist import build_worklist
        match = [e for e in build_worklist() if e.test_name == args.entry]
        if not match:
            print("no worklist entry named %r" % args.entry, file=sys.stderr)
            return 2
        e = match[0]
        binary, selector = e.binary_path, e.func_selector
        arch, load_debug, test_name = e.arch, e.load_debug, e.test_name
    if not binary or not selector:
        p.error("need <binary> <selector> or --entry <test_name>")

    res = compare(binary, selector, reference=args.reference, arch=arch,
                  load_debug=load_debug, structurer=args.structurer, test_name=test_name,
                  kuna_engine=args.kuna_engine)
    if args.json:
        print(json.dumps(res.to_dict(), indent=2))
    else:
        _print_human(res)
    return 0


if __name__ == "__main__":
    sys.exit(main())
