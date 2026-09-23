#!/usr/bin/env python3
"""Drive `kuna decompile-all` on the decbench corpus and count casts against
the frozen ida/ghidra/angr/kuna outputs.

Emits one .c per (opt, project, binary) with the same `// Function: n @ 0xA`
banner the frozen results use, so castcount.py treats them identically.
"""
import json, os, re, subprocess, sys, collections, concurrent.futures as cf
from pathlib import Path

RES = Path("/home/mahaloz/github/decbench/results/full_run_address_2026-09-11")
OUT = Path("/home/mahaloz/kwt/castcensus/.scratch/out")
SC = Path("/home/mahaloz/kwt/castcensus/.scratch")
sys.path.insert(0, str(SC))
import castcount as CC

TARGETS = {
    "coreutils": ["fmt", "ls", "sort", "du", "cp", "tail", "wc"],
    "grep": ["grep"], "gzip": ["gzip"],
    "diffutils": ["cmp", "diff", "diff3", "sdiff"],
    "tar": ["tar"], "findutils": ["find"],
}
OPTS = ["O0", "O2", "O2-noinline"]


def kuna_c(kbin, binary, outc, extra=()):
    env = dict(os.environ)
    env["SLEIGHHOME"] = "/home/mahaloz/kwt/castcensus/specs"
    env["KUNA_SPECS"] = "/home/mahaloz/kwt/castcensus/specs"
    cmd = [str(kbin), "decompile-all", str(binary), "--json", *extra]
    try:
        p = subprocess.run(cmd, capture_output=True, text=True, env=env,
                           timeout=1800)
    except subprocess.TimeoutExpired:
        return None, "timeout"
    if p.returncode != 0:
        return None, (p.stderr or "")[-300:]
    try:
        d = json.loads(p.stdout)
    except Exception as e:
        return None, f"json: {e}"
    parts, addrs = [], {}
    for f in d.get("functions", []):
        code = f.get("code")
        if not code:
            continue
        a = f["address"]
        parts.append(f"// Function: {f['name']} @ {hex(a)}\n{code}\n")
        addrs[a] = code
    outc.parent.mkdir(parents=True, exist_ok=True)
    outc.write_text("\n".join(parts))
    return addrs, None


def per_func(src):
    """{addr: (code_lines, statements, casts, Counter(shape))}"""
    out = {}
    for name, addr, text in CC.split_functions(src):
        a = CC.analyze(text)
        out[addr] = (a["code_lines"], a["statements"], a["casts"],
                     collections.Counter((t, s) for t, s, _, _ in a["cast_list"]),
                     [(t, s, txt) for t, s, _, txt in a["cast_list"]])
    return out


def job(args):
    opt, proj, b, kbin, tag = args
    binary = RES / opt / proj / "stripped" / b
    if not binary.exists():
        return None
    outc = OUT / tag / opt / proj / f"{b}.c"
    addrs, err = kuna_c(kbin, binary, outc)
    return (opt, proj, b, tag, err, str(outc))


def main():
    which = sys.argv[1] if len(sys.argv) > 1 else "new"
    kbin = {"new": "/home/mahaloz/kwt/castcensus/decompiler/target/release/kuna",
            "base": "/home/mahaloz/kwt/_baseline/kuna"}[which]
    jobs = [(o, p, b, kbin, which)
            for o in OPTS for p, bs in TARGETS.items() for b in bs
            if (RES / o / p / "stripped" / b).exists()]
    print(f"{len(jobs)} jobs -> {OUT/which}", flush=True)
    ok = fail = 0
    with cf.ThreadPoolExecutor(max_workers=10) as ex:
        for r in ex.map(job, jobs):
            if r is None:
                continue
            if r[4]:
                fail += 1
                print(f"  FAIL {r[0]}/{r[1]}/{r[2]}: {r[4]}", flush=True)
            else:
                ok += 1
    print(f"done ok={ok} fail={fail}", flush=True)


if __name__ == "__main__":
    main()
