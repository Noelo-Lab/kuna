#!/usr/bin/env python3
"""Build the cast-census tables from the frozen rival outputs + regenerated kuna."""
import json, sys, collections, os
from pathlib import Path

RES = Path("/home/mahaloz/github/decbench/results/full_run_address_2026-09-11")
SC = Path("/home/mahaloz/kwt/castcensus/.scratch")
OUT = SC / "out"
sys.path.insert(0, str(SC))
import castcount as CC

TARGETS = {
    "coreutils": ["fmt", "ls", "sort", "du", "cp", "tail", "wc"],
    "grep": ["grep"], "gzip": ["gzip"],
    "diffutils": ["cmp", "diff", "diff3", "sdiff"],
    "tar": ["tar"], "findutils": ["find"],
}
OPTS = ["O0", "O2", "O2-noinline"]
RIVALS = ["ida", "ghidra", "angr"]


def load_funcs(path):
    """{addr: dict} for one .c file."""
    try:
        src = Path(path).read_text(errors="replace")
    except OSError:
        return None
    out = {}
    for name, addr, text in CC.split_functions(src):
        a = CC.analyze(text)
        out[addr] = {
            "name": name, "code_lines": a["code_lines"],
            "stmts": a["statements"], "casts": a["casts"],
            "shapes": collections.Counter((t, s) for t, s, _, _ in a["cast_list"]),
            "list": [(t, s, txt) for t, s, _, txt in a["cast_list"]],
        }
    return out


def agg(funcs, addrs=None):
    L = S = C = 0
    for a, f in funcs.items():
        if addrs is not None and a not in addrs:
            continue
        L += f["code_lines"]; S += f["stmts"]; C += f["casts"]
    return L, S, C


def dens(L, S, C):
    return (round(1000.0 * C / L, 1) if L else 0.0,
            round(100.0 * C / S, 1) if S else 0.0)


def main():
    rows = []            # whole-binary density, all tools
    shared_rows = []     # address-intersection kuna vs ida
    shape_hist = collections.Counter()
    shape_ex = {}
    ida_shape_hist = collections.Counter()
    matched_construct = collections.Counter()  # kuna shape -> ida-had-a-cast?
    perfunc = []         # (opt, proj, bin, addr, name, kuna_casts, ida_casts, lines)
    base_rows = []

    for opt in OPTS:
        for proj, bins in TARGETS.items():
            for b in bins:
                dec = RES / opt / proj / "decompiled"
                tools = {}
                for t in RIVALS:
                    f = load_funcs(dec / f"{t}_{b}.c")
                    if f:
                        tools[t] = f
                f = load_funcs(dec / f"kuna_{b}.c")
                if f:
                    tools["kuna_0911"] = f
                f = load_funcs(OUT / "new" / opt / proj / f"{b}.c")
                if f:
                    tools["kuna_main"] = f
                f = load_funcs(OUT / "base" / opt / proj / f"{b}.c")
                if f:
                    tools["kuna_base"] = f
                if "kuna_main" not in tools:
                    continue
                row = {"opt": opt, "proj": proj, "bin": b}
                for t, fs in tools.items():
                    L, S, C = agg(fs)
                    row[t] = {"funcs": len(fs), "lines": L, "stmts": S,
                              "casts": C, "kloc": dens(L, S, C)[0],
                              "per100s": dens(L, S, C)[1]}
                rows.append(row)

                # address intersection with ida
                if "ida" in tools:
                    shared = set(tools["ida"]) & set(tools["kuna_main"])
                    r = {"opt": opt, "proj": proj, "bin": b, "n": len(shared)}
                    for t in ("ida", "ghidra", "angr", "kuna_main", "kuna_base",
                              "kuna_0911"):
                        if t in tools:
                            L, S, C = agg(tools[t], shared)
                            k, p = dens(L, S, C)
                            r[t] = {"lines": L, "stmts": S, "casts": C,
                                    "kloc": k, "per100s": p}
                    shared_rows.append(r)
                    for a in shared:
                        kf, idf = tools["kuna_main"][a], tools["ida"][a]
                        perfunc.append((opt, proj, b, a, kf["name"],
                                        kf["casts"], idf["casts"],
                                        kf["code_lines"], idf["code_lines"]))
                        for (t, s), n in kf["shapes"].items():
                            shape_hist[(t, s)] += n
                            if (t, s) not in shape_ex:
                                for tt, ss, txt in kf["list"]:
                                    if (tt, ss) == (t, s):
                                        shape_ex[(t, s)] = (
                                            f"{opt}/{b}@{hex(a)}", txt)
                                        break
                            # does ida cast at all in this same function?
                            matched_construct[((t, s), idf["casts"] > 0)] += n
                        for (t, s), n in idf["shapes"].items():
                            ida_shape_hist[(t, s)] += n

    json.dump({"whole": rows, "shared": shared_rows,
               "shapes": [[f"({t}){s}", n, shape_ex.get((t, s), ("", ""))[0],
                           shape_ex.get((t, s), ("", ""))[1],
                           ida_shape_hist.get((t, s), 0)]
                          for (t, s), n in shape_hist.most_common(60)],
               "ida_shapes": [[f"({t}){s}", n]
                              for (t, s), n in ida_shape_hist.most_common(40)],
               "perfunc": perfunc},
              open(SC / "tables.json", "w"))
    print("wrote tables.json", len(rows), "rows", len(perfunc), "shared funcs")


if __name__ == "__main__":
    main()
