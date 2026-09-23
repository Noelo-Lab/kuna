#!/usr/bin/env python3
"""Cast density per option: flip one option away from its default and re-count."""
import json, os, subprocess, sys, collections, concurrent.futures as cf
from pathlib import Path

RES = Path("/home/mahaloz/github/decbench/results/full_run_address_2026-09-11")
SC = Path("/home/mahaloz/kwt/castcensus/.scratch")
sys.path.insert(0, str(SC))
import castcount as CC

KUNA = "/home/mahaloz/kwt/castcensus/decompiler/target/release/kuna"
BINS = [("O0", "coreutils", b) for b in ("fmt", "ls", "sort", "du")] + \
       [("O2", "coreutils", b) for b in ("fmt", "ls", "sort", "du")] + \
       [("O0", "grep", "grep"), ("O0", "gzip", "gzip"),
        ("O2", "grep", "grep"), ("O2", "gzip", "gzip")]

FLIPS = [("baseline", None, None),
         ("structsynth=off", "structsynth", "off"),
         ("structsynth=all", "structsynth", "all"),
         ("slotptr=off", "slotptr", "off"),
         ("calleevote=off", "calleevote", "off"),
         ("protoorder=off", "protoorder", "off"),
         ("ptrfromuse=off", "ptrfromuse", "off"),
         ("ptrfromuse=byte", "ptrfromuse", "byte"),
         ("charptr=on", "charptr", "on"),
         ("libctypes=off", "libctypes", "off"),
         ("libctypes=glibc", "libctypes", "glibc"),
         ("framelayout=off", "framelayout", "off"),
         ("boolbyte=off", "boolbyte", "off"),
         ("charbyte=off", "charbyte", "off"),
         ("typedepth=off", "typedepth", "off"),
         ("passthrough=off", "passthrough", "off"),
         ("libcwiden=off", "libcwiden", "off")]


def run(args):
    label, opt, val, o, p, b = args
    env = dict(os.environ)
    env["SLEIGHHOME"] = "/home/mahaloz/kwt/castcensus/specs"
    env["KUNA_SPECS"] = "/home/mahaloz/kwt/castcensus/specs"
    cmd = [KUNA, "decompile-all", str(RES / o / p / "stripped" / b), "--json"]
    if opt:
        cmd += ["--option", opt, val]
    try:
        r = subprocess.run(cmd, capture_output=True, text=True, env=env,
                           timeout=1800)
    except subprocess.TimeoutExpired:
        return label, o, b, None, "timeout"
    if r.returncode != 0:
        return label, o, b, None, (r.stderr or "")[-200:]
    d = json.loads(r.stdout)
    src = "\n".join(f"// Function: {f['name']} @ {hex(f['address'])}\n{f['code']}\n"
                    for f in d["functions"] if f.get("code"))
    a = CC.analyze(src)
    return label, o, b, (a["code_lines"], a["statements"], a["casts"]), None


def main():
    jobs = [(lab, opt, val, o, p, b)
            for (lab, opt, val) in FLIPS for (o, p, b) in BINS]
    acc = collections.defaultdict(lambda: [0, 0, 0])
    per = collections.defaultdict(dict)
    errs = []
    with cf.ThreadPoolExecutor(max_workers=10) as ex:
        for lab, o, b, res, err in ex.map(run, jobs):
            if err:
                errs.append((lab, o, b, err))
                continue
            L, S, C = res
            acc[lab][0] += L; acc[lab][1] += S; acc[lab][2] += C
            per[lab][f"{o}/{b}"] = [L, S, C]
    base = acc["baseline"]
    out = []
    for lab, (L, S, C) in sorted(acc.items(),
                                 key=lambda kv: -1000.0 * kv[1][2] / max(kv[1][0], 1)):
        k = 1000.0 * C / max(L, 1)
        bk = 1000.0 * base[2] / max(base[0], 1)
        out.append({"flip": lab, "lines": L, "stmts": S, "casts": C,
                    "kloc": round(k, 1), "d_casts": C - base[2],
                    "d_kloc": round(k - bk, 1),
                    "pct": round(100.0 * (C - base[2]) / max(base[2], 1), 1)})
    json.dump({"agg": out, "per": per, "errs": errs},
              open(SC / "ablate.json", "w"), indent=1)
    for r in out:
        print(f"{r['flip']:<20} casts={r['casts']:<7} /kloc={r['kloc']:<7} "
              f"d={r['d_casts']:+6d} ({r['pct']:+.1f}%)  lines={r['lines']}")
    for e in errs:
        print("ERR", e)


if __name__ == "__main__":
    main()
