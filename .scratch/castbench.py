#!/usr/bin/env python3
"""castbench -- the cast campaign's one shared instrument.

Runs `kuna decompile-all --json` over a fixed decbench corpus and counts casts
with castcount.py (the census counter: C cast grammar, per-file vocabulary,
0 FP / 0 FN on a 696-span hand audit). Every number is on the functions that
BOTH kuna and IDA emit at the same address, so the function set is identical
across arms and rivals.

  castbench.py run   --kuna BIN --out DIR [--option NAME VALUE]... [--set small|full] [--workers N]
  castbench.py table --out DIR [--base DIR]      # kuna vs ida (and vs a base arm)
  castbench.py diff  --base DIR --out DIR [--top N]   # per-function deltas, both directions

Specs come from $SLEIGHHOME / $KUNA_SPECS (export them on separate lines).
Never writes into the decbench results tree.
"""
import argparse, collections, concurrent.futures as cf, json, os, subprocess, sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE))
import castcount as CC

RES = Path("/home/mahaloz/github/decbench/results/full_run_address_2026-09-11")
FULL = {
    "coreutils": ["fmt", "ls", "sort", "du", "cp", "tail", "wc"],
    "grep": ["grep"], "gzip": ["gzip"],
    "diffutils": ["cmp", "diff", "diff3", "sdiff"],
    "tar": ["tar"], "findutils": ["find"],
}
SMALL = {"coreutils": ["fmt", "ls", "sort", "du"]}
OPTS_FULL = ["O0", "O2", "O2-noinline"]
OPTS_SMALL = ["O0", "O2"]


def corpus(which):
    t, o = (FULL, OPTS_FULL) if which == "full" else (SMALL, OPTS_SMALL)
    for opt in o:
        for proj, bins in t.items():
            for b in bins:
                if (RES / opt / proj / "stripped" / b).exists():
                    yield opt, proj, b


def run_one(kbin, opt, proj, b, out, options):
    binary = RES / opt / proj / "stripped" / b
    outc = out / opt / proj / f"{b}.c"
    cmd = [kbin, "decompile-all", str(binary), "--json"]
    for n, v in options:
        cmd += ["--option", n, v]
    try:
        p = subprocess.run(cmd, capture_output=True, text=True, timeout=3600)
    except subprocess.TimeoutExpired:
        return f"{opt}/{proj}/{b}: timeout"
    if p.returncode != 0:
        return f"{opt}/{proj}/{b}: rc={p.returncode} {(p.stderr or '')[-300:]}"
    d = json.loads(p.stdout)
    parts = []
    for f in d.get("functions", []):
        if f.get("code"):
            parts.append(f"// Function: {f['name']} @ {hex(f['address'])}\n{f['code']}\n")
    outc.parent.mkdir(parents=True, exist_ok=True)
    outc.write_text("\n".join(parts))
    return None


def funcs(path):
    """{addr: (code_lines, statements, casts, Counter(shape))}, vocabulary harvested per FILE."""
    try:
        src = Path(path).read_text(errors="replace")
    except OSError:
        return None
    vocab = CC.harvest_types(CC.tokenize(src), src)
    out = {}
    for _, addr, text in CC.split_functions(src):
        toks = CC.tokenize(text)
        cl = CC.find_casts(toks, vocab)
        code = sum(1 for L in text.splitlines() if L.strip() and not L.strip().startswith("//"))
        out[addr] = (code, CC.count_statements(toks), len(cl),
                     collections.Counter((t, s) for t, s, _, _ in cl))
    return out


def shared(which, out, base=None):
    """yield (opt, proj, b, addr, kuna_row, ida_row, base_row) over the address-matched set."""
    for opt, proj, b in corpus(which):
        k = funcs(Path(out) / opt / proj / f"{b}.c")
        i = funcs(RES / opt / proj / "decompiled" / f"ida_{b}.c")
        if not k or not i:
            continue
        bb = funcs(Path(base) / opt / proj / f"{b}.c") if base else None
        for a in sorted(set(k) & set(i) & (set(bb) if bb else set(k))):
            yield opt, proj, b, a, k[a], i[a], (bb[a] if bb else None)


def cmd_run(a):
    out = Path(a.out)
    out.mkdir(parents=True, exist_ok=True)
    (out / "arm.json").write_text(json.dumps({"kuna": a.kuna, "options": a.option, "set": a.set}))
    jobs = list(corpus(a.set))
    errs = []
    with cf.ThreadPoolExecutor(max_workers=a.workers) as ex:
        for e in ex.map(lambda j: run_one(a.kuna, *j, out, a.option or []), jobs):
            if e:
                errs.append(e)
                print("FAIL", e, flush=True)
    print(f"castbench run: {len(jobs) - len(errs)}/{len(jobs)} binaries -> {out}")


def cmd_table(a):
    tot = collections.defaultdict(lambda: [0, 0, 0])
    per_opt = collections.defaultdict(lambda: collections.defaultdict(lambda: [0, 0, 0]))
    n = 0
    for opt, proj, b, addr, k, i, bb in shared(a.set, a.out, a.base):
        n += 1
        for tag, r in (("kuna", k), ("ida", i), ("base", bb)):
            if r is None:
                continue
            for j in range(3):
                tot[tag][j] += r[j]
                per_opt[opt][tag][j] += r[j]
    def line(tag, r, ref):
        c, s = r[2], r[1]
        return (f"{tag:6} casts {c:7,}  lines {r[0]:8,}  stmts {s:8,}  /kloc {1000*c/max(r[0],1):6.1f}"
                f"  /100stmt {100*c/max(s,1):5.1f}  vs ida {c/max(ref[2],1):.3f}")
    print(f"shared functions (kuna ∩ ida{' ∩ base' if a.base else ''}): {n}")
    for tag in ("ida", "base", "kuna"):
        if tag in tot:
            print(line(tag, tot[tag], tot["ida"]))
    for opt in sorted(per_opt):
        print(f"-- {opt}")
        for tag in ("ida", "base", "kuna"):
            if tag in per_opt[opt]:
                print("   " + line(tag, per_opt[opt][tag], per_opt[opt]["ida"]))


def cmd_diff(a):
    up, down, same = [], [], 0
    shape_delta = collections.Counter()
    for opt, proj, b, addr, k, i, bb in shared(a.set, a.out, a.base):
        d = k[2] - bb[2]
        shape_delta.update(k[3]); shape_delta.subtract(bb[3])
        if d > 0:
            up.append((d, opt, b, addr))
        elif d < 0:
            down.append((d, opt, b, addr))
        else:
            same += 1
    print(f"functions: fewer casts {len(down)} ({-sum(x[0] for x in down):,} casts)  "
          f"more casts {len(up)} (+{sum(x[0] for x in up):,})  unchanged {same}")
    print("top MORE-cast functions (read every one):")
    for d, opt, b, addr in sorted(up, reverse=True)[:a.top]:
        print(f"   +{d:3}  {opt}/{b} @{hex(addr)}")
    print("top fewer-cast functions:")
    for d, opt, b, addr in sorted(down)[:a.top]:
        print(f"   {d:4}  {opt}/{b} @{hex(addr)}")
    print("shape deltas (type, operand):")
    for (t, s), n in sorted(shape_delta.items(), key=lambda x: x[1])[:a.top]:
        if n:
            print(f"   {n:+6}  ({t}) {s}")
    for (t, s), n in sorted(shape_delta.items(), key=lambda x: -x[1])[:a.top]:
        if n > 0:
            print(f"   {n:+6}  ({t}) {s}")


def main():
    p = argparse.ArgumentParser()
    sp = p.add_subparsers(dest="cmd", required=True)
    r = sp.add_parser("run"); r.add_argument("--kuna", required=True); r.add_argument("--out", required=True)
    r.add_argument("--option", nargs=2, action="append"); r.add_argument("--set", default="full")
    r.add_argument("--workers", type=int, default=12)
    t = sp.add_parser("table"); t.add_argument("--out", required=True); t.add_argument("--base")
    t.add_argument("--set", default="full")
    d = sp.add_parser("diff"); d.add_argument("--out", required=True); d.add_argument("--base", required=True)
    d.add_argument("--set", default="full"); d.add_argument("--top", type=int, default=25)
    a = p.parse_args()
    {"run": cmd_run, "table": cmd_table, "diff": cmd_diff}[a.cmd](a)


if __name__ == "__main__":
    main()
