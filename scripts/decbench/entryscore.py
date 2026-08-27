"""Function-entry precision/recall scorer -- the *discovery-tier* measurement instrument.

The GED loop (`optsweep`) measures the shape of the C kuna emits for a function
it already found. Nothing there measures **which functions it finds**, so a P1
discovery change (`aif`, `aifstrict`, `aifcorroborate`, `ptrentry`, `poolentry`,
`tailcallentry`, `funcstart_patterns`, ...) is invisible to it. This scores that
directly: kuna's function-entry set for a STRIPPED image against ground truth
recovered from the same image's UNSTRIPPED twin.

The decbench corpus carries a `compiled/` twin for every `stripped/` binary, so
ground truth here is an exact symbol table, not a reference decompiler's guess.

Ground truth sources (`--gt-kind`, auto-detected by default):

* an unstripped ELF   -> pyelftools (`STT_FUNC`, `st_value` + `st_size`; the
  ARM/Thumb low bit masked), falling back to `readelf -sW`;
* an unstripped PE    -> `nm` (`T`/`t`); it reports no sizes, so extents are
  filled from the next start and PE mid-body counts are therefore UPPER BOUNDS;
* a JSON inventory    -> a list of `{"addr": int, "size": int}` (or a
  `{hex: {size}}` map), which is how a reference-decompiler inventory (IDA,
  Ghidra) is consumed for an image that has no twin at all.

Metrics for one (image, arm) pair::

    entries        function entries kuna reports
    exact          entries that ARE a ground-truth start
    midbody        entries strictly inside a ground-truth body (start < a < end)
    midbody_known  the subset of `midbody` whose ENCLOSING function kuna also
                   has an entry for -- the "two overlapping functions decompile
                   the same instructions twice" population
    unknown        neither: a PLT/thunk, a real function the twin has no symbol
                   for, or a ground-truth gap
    recall         exact / gt

`midbody` is the number GH-299/GH-313 are about, and it is only meaningful
*marginally*: `--marginal REF` re-scores each arm's entries MINUS the reference
arm's entries, which for `REF = aif off` is exactly "the entries AIF added, and
how many of them landed inside a body".

Usage (from the repo root)::

    python3 -m scripts.decbench.entryscore <stripped> <ground-truth> \\
        [--mode aggressive] [--option NAME VALUE]...
    python3 -m scripts.decbench.entryscore --gt <ground-truth>   # dump GT as JSON

Knobs: `KUNA_BIN`, `KUNA_SPECS` (exported to the child as `SLEIGHHOME` too).
"""
import argparse
import bisect
import json
import os
import re
import subprocess
import sys
import time

from . import config


def _kuna_env() -> dict:
    specs = str(config.repo_root() / "specs")
    specs = os.environ.get("KUNA_SPECS", specs)
    return dict(os.environ, KUNA_SPECS=specs, SLEIGHHOME=specs)


# ---------------------------------------------------------------- ground truth
def _elf_gt_pyelftools(path):
    from elftools.elf.elffile import ELFFile
    from elftools.elf.sections import SymbolTableSection
    fns = {}
    with open(path, "rb") as f:
        elf = ELFFile(f)
        for sec in elf.iter_sections():
            if not isinstance(sec, SymbolTableSection):
                continue
            for sym in sec.iter_symbols():
                if sym["st_info"]["type"] != "STT_FUNC":
                    continue
                val = sym["st_value"]
                if not val:
                    continue
                val &= ~1  # ARM/Thumb low bit
                size = sym["st_size"]
                if val not in fns or size > fns[val][0]:
                    fns[val] = (size, sym.name)
    return fns


def _elf_gt_readelf(path):
    out = subprocess.run(["readelf", "-sW", path], capture_output=True, text=True).stdout
    fns = {}
    for ln in out.splitlines():
        p = ln.split()
        if len(p) < 8 or p[3] != "FUNC":
            continue
        try:
            val, size = int(p[1], 16), int(p[2], 0)
        except ValueError:
            continue
        if not val:
            continue
        val &= ~1
        if val not in fns or size > fns[val][0]:
            fns[val] = (size, p[7])
    return fns


def _pe_gt(path):
    """`nm` over a non-stripped PE. No sizes -- extents are next-start-filled."""
    out = subprocess.run(["nm", path], capture_output=True, text=True).stdout
    fns = {}
    for ln in out.splitlines():
        m = re.match(r"^([0-9a-fA-F]+)\s+([Tt])\s+(\S+)", ln)
        if m:
            fns[int(m.group(1), 16)] = (0, m.group(3))
    return fns


def _json_gt(path):
    """A reference-decompiler inventory: [{"addr":int,"size":int,...}] or {hex:{size}}."""
    d = json.load(open(path))
    fns = {}
    if isinstance(d, dict):
        for k, v in d.items():
            a = int(k, 0)
            fns[a] = (int(v.get("size", 0)) if isinstance(v, dict) else 0,
                      (v or {}).get("name", "") if isinstance(v, dict) else "")
    else:
        for r in d:
            a = int(r["addr"])
            fns[a] = (int(r.get("size", 0)), r.get("short") or r.get("raw") or "")
    return fns


def ground_truth(path, kind="auto"):
    """-> {addr: (size, name)}; every size filled (next start when unknown)."""
    if kind == "auto":
        if str(path).endswith(".json"):
            kind = "json"
        else:
            with open(path, "rb") as f:
                kind = "elf" if f.read(4) == b"\x7fELF" else "pe"
    if kind == "json":
        g = _json_gt(path)
    elif kind == "elf":
        try:
            g = _elf_gt_pyelftools(path)
        except ImportError:
            g = _elf_gt_readelf(path)
    else:
        g = _pe_gt(path)
    starts = sorted(g)
    for i, a in enumerate(starts):
        sz, nm = g[a]
        if sz == 0:
            g[a] = ((starts[i + 1] - a) if i + 1 < len(starts) else 0, nm)
    return g


# ---------------------------------------------------------------------- kuna
def kuna_entries(binary, options=(), mode="aggressive", timeout=1800):
    """-> (set of entry addresses, seconds). Raises on a failed/empty run."""
    cmd = [config.kuna_bin(), "functions", binary, "--json"]
    if mode:
        cmd += ["--mode", mode]
    for name, val in options:
        cmd += ["--option", name, val]
    t = time.time()
    r = subprocess.run(cmd, capture_output=True, text=True, env=_kuna_env(), timeout=timeout)
    dt = time.time() - t
    if r.returncode != 0 or not r.stdout.strip():
        raise RuntimeError(f"kuna failed rc={r.returncode}: {r.stderr[-400:]}")
    return {f["address"] for f in json.loads(r.stdout)["functions"]}, dt


# --------------------------------------------------------------------- scoring
def score(entries, gt, other_entries=None):
    """`entries`: kuna addresses. `gt`: a `ground_truth()` dict.

    `other_entries` answers "is the ENCLOSING function known?" for `midbody_known`;
    it defaults to `entries` itself, and in a marginal scoring is the reference
    arm's entry set.
    """
    starts = sorted(gt)
    gtset = set(starts)
    known = entries if other_entries is None else other_entries

    def enclosing(a):
        i = bisect.bisect_right(starts, a) - 1
        if i < 0:
            return None
        s = starts[i]
        sz = gt[s][0]
        if s == a or not sz or a >= s + sz:
            return None
        return s

    exact = entries & gtset
    rest = entries - gtset
    mid = {a: enclosing(a) for a in rest}
    mid = {a: s for a, s in mid.items() if s is not None}
    return dict(
        entries=len(entries),
        gt=len(gtset),
        exact=len(exact),
        midbody=len(mid),
        midbody_known=sum(1 for s in mid.values() if s in known),
        unknown=len(rest) - len(mid),
        recall=(len(exact) / len(gtset)) if gtset else None,
        precision_exact=(len(exact) / len(entries)) if entries else None,
    )


def run_one(stripped, gt_path, options=(), mode="aggressive", gt_kind="auto"):
    gt = ground_truth(gt_path, gt_kind)
    ents, dt = kuna_entries(stripped, options, mode)
    row = score(ents, gt)
    row["seconds"] = round(dt, 2)
    return row, ents, gt


def main(argv=None):
    ap = argparse.ArgumentParser(prog="entryscore", description=__doc__.splitlines()[0])
    ap.add_argument("stripped", nargs="?")
    ap.add_argument("gt", nargs="?", help="unstripped twin, or a JSON inventory")
    ap.add_argument("--gt-kind", default="auto", choices=["auto", "elf", "pe", "json"])
    ap.add_argument("--mode", default="aggressive")
    ap.add_argument("--option", nargs=2, action="append", default=[], metavar=("NAME", "VALUE"))
    ap.add_argument("--dump-gt", metavar="PATH", help="dump ground truth as JSON and exit")
    a = ap.parse_args(argv)

    if a.dump_gt:
        g = ground_truth(a.dump_gt, a.gt_kind)
        print(json.dumps({hex(k): {"size": s, "name": n} for k, (s, n) in sorted(g.items())}, indent=1))
        return 0
    if not a.stripped or not a.gt:
        ap.error("stripped and gt are required (or use --dump-gt)")
    row, _, _ = run_one(a.stripped, a.gt, [tuple(o) for o in a.option], a.mode, a.gt_kind)
    print(json.dumps(row, indent=1))
    return 0


if __name__ == "__main__":
    sys.exit(main())
