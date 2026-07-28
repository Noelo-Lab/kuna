"""Mine the decbench results tree for functions kuna decompiles *better* than the rest.

The mirror image of ``scripts.decbench.mine``: that one hunts kuna's losses to
drive the improvement campaign, this one hunts kuna's **wins** to stock the
landing page's compare section (``integrations/web/compare-samples.js``).

    python3 -m scripts.decbench.showcase                       # rank the pool
    python3 -m scripts.decbench.showcase --dump /tmp/cands     # + per-case bundles
    python3 -m scripts.decbench.showcase --emit picks.json     # + compare-samples.js entries

A candidate is a single function, on an **optimized** build, that

  * every one of kuna / ida / ghidra / binja / angr scored (so no pane is empty),
  * kuna beats ``--rival`` (default ``ida``) on GED, and
  * with ``--sweep`` (default) kuna beats *all four* rivals, so the sample holds
    up whichever one the visitor picks from the dropdown.

Ranking is by mean GED margin over the rivals; ``--perfect`` narrows to kuna
GED = 0 (structurally exact). One row per ``(project, function)`` — the same
source function compiled into six openssh binaries is one candidate, not six.

**GED is a starting filter, not the verdict.** Roughly half of decbench's
cross-decompiler GED gaps are scoring artifacts rather than differences a human
can see (``docs/decbench-loop.md`` → *GED quirks*), so a mined candidate is only
a showcase after someone reads all six panes. ``--dump`` exists for exactly
that: it writes ``<case-id>/{kuna,ida,ghidra,binja,angr,source}.c`` so an agent
can be pointed at the directory. See ``docs/decbench-loop.md`` → *Finding good
kuna examples* for the full recipe.

Provenance, which the page must not misstate: decbench decompiled the
**stripped** binary with every decompiler, then renamed each function's own
``sub_<addr>`` placeholder to its DWARF name (``_relabel_to_dwarf``). Callees
stay ``sub_``/``dat_`` in every pane. That is the only edit; the bodies are
verbatim tool output.
"""
from __future__ import annotations

import argparse
import json
import math
import sys
from collections import Counter
from pathlib import Path

from . import config

DECS = ("kuna", "ida", "ghidra", "binja", "angr")
RIVALS = ("ida", "ghidra", "binja", "angr")
OPTIMIZED = ("O2", "O2-noinline")


# ── mining ────────────────────────────────────────────────────────────────────

def _finite(v) -> bool:
    return v is not None and math.isfinite(v)


def load_rows() -> list[dict]:
    """Flatten function_results.json into one row per function, GED only."""
    fr = json.loads((config.results_root() / "function_results.json").read_text())
    rows = []
    for g in fr["groups"]:
        for f in g["functions"]:
            values = f.get("values") or {}
            rows.append({
                "project": g["project"],
                "opt_level": g["opt_level"],
                "binary": g["binary"],
                "function": f["function"],
                "size": f.get("size"),
                "labels": f.get("labels") or g.get("labels") or [],
                "ged": {d: (values.get(d) or {}).get("ged") for d in DECS},
            })
    return rows


def find_wins(
    rows: list[dict],
    *,
    rival: str = "ida",
    opts: tuple[str, ...] = OPTIMIZED,
    min_size: int = 22,
    max_size: int = 100,
    sweep: bool = True,
    perfect: bool = False,
) -> list[dict]:
    """Candidates where kuna out-scores ``rival`` (and, with ``sweep``, everyone)."""
    pool = []
    for r in rows:
        if opts and r["opt_level"] not in opts:
            continue
        ged = r["ged"]
        if not all(_finite(ged[d]) for d in DECS):
            continue
        if not ged["kuna"] < ged[rival]:
            continue
        if perfect and ged["kuna"] != 0.0:
            continue
        size = r["size"] or 0
        if not (min_size <= size <= max_size):
            continue
        wins = [d for d in RIVALS if ged["kuna"] < ged[d]]
        if sweep and len(wins) != len(RIVALS):
            continue
        c = dict(r)
        c["case_id"] = config.case_id(r["opt_level"], r["project"], r["binary"], r["function"])
        c["group_id"] = config.group_id(r["project"], r["function"])
        c["beats"] = wins
        c["margin_rival"] = ged[rival] - ged["kuna"]
        c["margin_mean"] = sum(ged[d] for d in RIVALS) / len(RIVALS) - ged["kuna"]
        pool.append(c)

    # One row per source function: keep the strongest instance (widest sweep,
    # then plain O2 over O2-noinline, then margin).
    best: dict[str, dict] = {}
    for c in pool:
        cur = best.get(c["group_id"])
        rank = (len(c["beats"]), c["opt_level"] == "O2", c["margin_mean"])
        if cur is None or rank > (len(cur["beats"]), cur["opt_level"] == "O2", cur["margin_mean"]):
            best[c["group_id"]] = c
    uniq = list(best.values())
    uniq.sort(key=lambda c: (-(c["ged"]["kuna"] == 0.0), -c["margin_mean"], c["size"]))
    return uniq


def as_candidate(row: dict) -> dict:
    """Promote a raw results row into a candidate (no filtering applied).

    Lets ``--emit`` / ``--verify`` reach a function the shortlist filters would
    exclude — the ``fmt`` ``main`` switch showcase is 218 lines, well past the
    default pane-fit cap, and is on the page by editorial choice.
    """
    g = row["ged"]
    c = dict(row)
    c["case_id"] = config.case_id(row["opt_level"], row["project"], row["binary"], row["function"])
    c["group_id"] = config.group_id(row["project"], row["function"])
    c["beats"] = [d for d in RIVALS if _finite(g[d]) and _finite(g["kuna"]) and g["kuna"] < g[d]]
    c["margin_mean"] = 0.0
    return c


def spread(cands: list[dict], per_project: int, limit: int) -> list[dict]:
    """Cap per project so one big corpus (openssh) cannot fill the shortlist."""
    seen: Counter[str] = Counter()
    out = []
    for c in cands:
        if seen[c["project"]] >= per_project:
            continue
        seen[c["project"]] += 1
        out.append(c)
        if limit and len(out) >= limit:
            break
    return out


# ── bundles ───────────────────────────────────────────────────────────────────

def _decbench_imports():
    """decbench's results-tree readers, importable without its venv."""
    sys.path.insert(0, str(config.decbench_repo()))
    from decbench.utils import results_tree, source_extract  # noqa: PLC0415
    return results_tree, source_extract


def describe_binary(path: Path | None) -> str:
    """``ELF x86-64`` / ``ELF ARM`` — the format+arch half of a sample's `meta:` line."""
    if path is None or not Path(path).is_file():
        return ""
    try:
        from elftools.elf.elffile import ELFFile  # noqa: PLC0415

        with open(path, "rb") as f:
            elf = ELFFile(f)
            machine = elf.header["e_machine"]
            bits = elf.elfclass
    except Exception:  # noqa: BLE001
        return ""
    arch = {
        "EM_X86_64": "x86-64", "EM_386": "i386", "EM_AARCH64": "AArch64",
        "EM_ARM": "ARM", "EM_RISCV": f"RISC-V {bits}", "EM_MIPS": "MIPS",
        "EM_PPC64": "PowerPC 64", "EM_SPARCV9": "SPARC v9",
    }.get(machine, machine.removeprefix("EM_"))
    return f"ELF {arch}"


def dump_bundles(cands: list[dict], outdir: Path) -> list[dict]:
    """Write ``<outdir>/<case-id>/{<dec>.c, source.c, meta.json}`` per candidate."""
    results_tree, source_extract = _decbench_imports()
    root = config.results_root()
    outdir.mkdir(parents=True, exist_ok=True)

    cache: dict[tuple, dict] = {}
    index = []
    for c in cands:
        opt, proj, stem = c["opt_level"], c["project"], c["binary"]
        d = outdir / c["case_id"]
        d.mkdir(exist_ok=True)
        lines = {}
        address = None
        for dec in DECS:
            key = (opt, proj, stem, dec)
            if key not in cache:
                cf = results_tree.decompiled_c_path(root, opt, proj, dec, stem)
                cache[key] = results_tree.split_functions(cf) if cf.is_file() else {}
            entry = cache[key].get(c["function"])
            if entry:
                addr, code = entry
                if dec == "kuna":
                    address = addr
                (d / f"{dec}.c").write_text(code)
                lines[dec] = len(code.splitlines())
        binary = results_tree.resolve_binary(results_tree.compiled_dir(root, opt, proj), stem)
        src, status = source_extract.function_source_ex(binary, c["function"])
        if src:
            (d / "source.c").write_text(src)
        meta = dict(c)
        meta["dir"] = str(d)
        meta["address"] = address
        meta["address_hex"] = f"0x{address:x}" if address is not None else None
        meta["source_status"] = status
        meta["dec_lines"] = lines
        meta["binary_path"] = str(binary) if binary else None
        meta["stripped_path"] = config.stripped_path(str(binary)) if binary else None
        meta["binary_desc"] = describe_binary(binary)
        arch = meta["binary_desc"].removeprefix("ELF ").strip()
        meta["label"] = f"{c['function']}() — {stem if stem.startswith(proj) else f'{proj} {stem}'}" + (
            f", {arch}" if arch else "")
        where = stem if stem.startswith(proj) else f"{proj} {stem}"
        meta["meta_line"] = " · ".join(x for x in (
            where, meta["binary_desc"],
            f"gcc -{opt.replace('O2-noinline', 'O2 -fno-inline')}, stripped",
            meta["address_hex"] or "") if x)
        (d / "meta.json").write_text(json.dumps(meta, indent=1) + "\n")
        index.append(meta)

    (outdir / "index.json").write_text(json.dumps(index, indent=1) + "\n")
    return index


# ── verification ──────────────────────────────────────────────────────────────

def _defined_name(code: str) -> str | None:
    """The identifier of the function *definition* kuna printed.

    The signature is the first line that is neither blank nor one of the
    ``/* WARNING: ... */`` banners kuna emits above it; the name is the last
    identifier before that line's opening paren.
    """
    import re  # noqa: PLC0415

    for line in code.splitlines():
        s = line.strip()
        if not s or s.startswith("/*") or s.startswith("//") or s.startswith("*"):
            continue
        m = re.match(r"[^()]*?(\w+)\s*\(", s)
        return m.group(1) if m else None
    return None


def verify_bundle(d: Path, timeout: int = 900) -> dict:
    """Re-decompile one bundle's function with the CURRENT kuna and diff the pane.

    The recorded pane came from decbench's kuna 0.1.0 run, so before a sample
    ships the page must still be what kuna prints *today*. Re-runs

        kuna decompile-all <stripped binary> --addr <0xVMA>

    — the same whole-binary, load-once surface decbench's backend drives
    (``decbench/decompilers/raw/kuna_raw.py``), Listing tier on by default, which
    is what stripped ARM firmware needs to find the function at all — then applies
    decbench's one edit (rename ``sub_<addr>`` to the DWARF name) and diffs.
    """
    import difflib  # noqa: PLC0415
    import re  # noqa: PLC0415
    import subprocess  # noqa: PLC0415

    meta = json.loads((d / "meta.json").read_text())
    recorded = (d / "kuna.c").read_text().rstrip()
    stripped, addr = meta.get("stripped_path"), meta.get("address")
    out = {"case_id": meta["case_id"], "function": meta["function"]}
    if not stripped or addr is None or not Path(stripped).is_file():
        return out | {"status": "no-binary", "identical": False}

    cmd = [config.kuna_bin(), "decompile-all", stripped, "--addr", hex(addr)]
    try:
        proc = subprocess.run(cmd, capture_output=True, text=True, timeout=timeout)
    except subprocess.TimeoutExpired:
        return out | {"status": "timeout", "identical": False}
    if proc.returncode != 0:
        return out | {"status": "kuna-failed", "identical": False,
                      "detail": (proc.stderr or "").strip()[-400:]}

    # strip the `// Function: <name> @ <addr>` header decompile-all prints
    fresh = re.sub(r"^//\s*Function:.*\n", "", proc.stdout.strip()).strip()
    placeholder = _defined_name(fresh)
    if placeholder and placeholder != meta["function"]:
        fresh = re.sub(r"\b" + re.escape(placeholder) + r"\b", meta["function"], fresh)
    identical = fresh == recorded
    diff = [] if identical else list(difflib.unified_diff(
        recorded.splitlines(), fresh.splitlines(),
        "recorded", "current", lineterm="", n=1))[:60]
    return out | {"status": "ok", "identical": identical, "cmd": " ".join(cmd),
                  "diff": diff}


# ── compare-samples.js emission ───────────────────────────────────────────────

def _js_str(text: str) -> str:
    """A double-quoted JS string; non-ASCII stays readable in the source."""
    return json.dumps(text, ensure_ascii=False)


def _js_literal(code: str) -> str:
    """A JS template literal body: escape only what breaks one."""
    return code.replace("\\", "\\\\").replace("`", "\\`").replace("${", "\\${")


def emit_samples_js(picks: list[dict], bundles: Path) -> str:
    """Render ``SAMPLES`` entries for the picks (paste into compare-samples.js).

    ``picks`` rows are just ``{case_id}``; ``name`` and ``meta`` default to the
    generated dropdown label and provenance line and only need overriding to
    disambiguate. Code comes off disk from the ``--dump`` bundles so nothing is
    retyped by hand.

    Deliberately no per-sample commentary, and the default label is a neutral
    identifier (``fn() — project binary, arch``) rather than a claim about what
    the sample shows: the page gives provenance and the measured GED and lets the
    panes speak. Anything editorial would be the one thing on the page that is
    not machine-derived.
    """
    out = []
    for p in picks:
        d = bundles / p["case_id"]
        meta = json.loads((d / "meta.json").read_text())
        ged = meta["ged"]
        vs = []
        for dec in ("source", "ghidra", "ida", "binja", "angr"):
            f = d / ("source.c" if dec == "source" else f"{dec}.c")
            if not f.is_file():
                continue
            vs.append(f"      {dec}:\n`{_js_literal(f.read_text().rstrip())}`,")
        kuna = _js_literal((d / "kuna.c").read_text().rstrip())
        gedjs = ", ".join(f"{d}: {ged[d]:g}" for d in DECS if _finite(ged[d]))
        out.append(
            f"  {{\n"
            f"    id: '{p['case_id']}',\n"
            f"    name: {_js_str(p.get('name') or meta.get('label', p['case_id']))},\n"
            f"    meta: {_js_str(p.get('meta') or meta.get('meta_line', ''))},\n"
            f"    ged: {{ {gedjs} }},\n"
            f"    kuna:\n`{kuna}`,\n"
            f"    vs: {{\n" + "\n".join(vs) + "\n    },\n"
            f"  }},\n"
        )
    return "\n".join(out)


# ── cli ───────────────────────────────────────────────────────────────────────

def _render_table(cands: list[dict]) -> str:
    head = (f"{'case-id':64s} {'ln':>4s}  " +
            "  ".join(f"{d:>6s}" for d in DECS))
    lines = [head, "-" * len(head)]
    for c in cands:
        g = c["ged"]
        lines.append(f"{c['case_id']:64s} {c['size'] or 0:4d}  " +
                     "  ".join(f"{g[d]:6.0f}" for d in DECS))
    return "\n".join(lines)


def main(argv=None) -> None:
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument("--rival", default="ida", choices=RIVALS,
                    help="the decompiler kuna must beat (default: ida)")
    ap.add_argument("--opt", default=",".join(OPTIMIZED),
                    help="comma-separated opt levels (default: the optimized ones)")
    ap.add_argument("--min-size", type=int, default=22, help="min decompiled lines")
    ap.add_argument("--max-size", type=int, default=100, help="max decompiled lines")
    ap.add_argument("--no-sweep", action="store_true",
                    help="accept beating only --rival, not all four")
    ap.add_argument("--perfect", action="store_true", help="require kuna GED == 0")
    ap.add_argument("--per-project", type=int, default=3, help="cap per project")
    ap.add_argument("--limit", type=int, default=40, help="shortlist size (0 = all)")
    ap.add_argument("--dump", metavar="DIR",
                    help="write per-candidate bundles (source + all five panes)")
    ap.add_argument("--emit", metavar="PICKS.JSON",
                    help="render compare-samples.js entries for the picks in this "
                         "file ([{case_id,name}], optional meta); needs --dump's DIR")
    ap.add_argument("--verify", action="store_true",
                    help="re-decompile every dumped bundle with the CURRENT kuna "
                         "and report whether the recorded pane still reproduces")
    ap.add_argument("--cases", metavar="ID,ID,...",
                    help="restrict --verify to these case ids (any function, even "
                         "one the shortlist filters exclude)")
    ap.add_argument("--json", action="store_true", help="machine-readable shortlist")
    args = ap.parse_args(argv)

    opts = tuple(o for o in args.opt.split(",") if o)
    rows = load_rows()
    cands = find_wins(
        rows,
        rival=args.rival,
        opts=opts,
        min_size=args.min_size,
        max_size=args.max_size,
        sweep=not args.no_sweep,
        perfect=args.perfect,
    )
    short = spread(cands, args.per_project, args.limit)

    if args.emit:
        if not args.dump:
            sys.exit("--emit needs --dump DIR (the bundles the code is read from)")
        picks = json.loads(Path(args.emit).read_text())
        wanted = {p["case_id"] for p in picks}
        chosen = [c for c in cands if c["case_id"] in wanted]
        # a pick outside the shortlist filters (editorial choice) still emits
        missing = wanted - {c["case_id"] for c in chosen}
        if missing:
            chosen += [as_candidate(r) for r in rows
                       if config.case_id(r["opt_level"], r["project"], r["binary"],
                                         r["function"]) in missing]
        by_id = {c["case_id"]: c for c in chosen}
        unknown = wanted - set(by_id)
        if unknown:
            sys.exit(f"unknown case_id(s): {', '.join(sorted(unknown))}")
        dump_bundles([by_id[p["case_id"]] for p in picks], Path(args.dump))
        print(emit_samples_js(picks, Path(args.dump)))
        return

    if args.verify:
        if not args.dump:
            sys.exit("--verify needs --dump DIR (the bundles to re-decompile)")
        outdir = Path(args.dump)
        if args.cases:
            wanted = set(args.cases.split(","))
            short = [c for c in cands if c["case_id"] in wanted]
            short += [as_candidate(r) for r in rows
                      if config.case_id(r["opt_level"], r["project"], r["binary"], r["function"])
                      in wanted - {c["case_id"] for c in short}]
        dump_bundles(short, outdir)
        bad = 0
        for c in short:
            v = verify_bundle(outdir / c["case_id"])
            mark = "OK  " if v.get("identical") else "DRIFT"
            bad += 0 if v.get("identical") else 1
            print(f"[{mark}] {v['case_id']} ({v['status']})")
            for line in v.get("diff", [])[:12]:
                print(f"        {line}")
        print(f"\n[showcase] {len(short) - bad}/{len(short)} panes reproduce "
              f"with the current kuna build")
        sys.exit(1 if bad else 0)

    if args.json:
        print(json.dumps(short, indent=1))
    else:
        print(f"[showcase] {len(cands)} candidates beat "
              f"{'all four rivals' if not args.no_sweep else args.rival}"
              f"{' with kuna GED 0' if args.perfect else ''}"
              f" on {'/'.join(opts)}; showing {len(short)}\n")
        print(_render_table(short))

    if args.dump:
        index = dump_bundles(short, Path(args.dump))
        got = sum(1 for e in index if e.get("source_status") != "binary_not_found")
        print(f"\n[showcase] {len(index)} bundles -> {args.dump} "
              f"({got} with source). Read every pane before shipping one.")


if __name__ == "__main__":
    main()
