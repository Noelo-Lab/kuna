"""Per-parameter layout diff between two kuna binaries on one build (why layout F1 moved D -> E)."""
import os
import sys
from pathlib import Path

sys.path.insert(0, "/home/mahaloz/kwt/_final-g/tools")
import layoutdiff as LD  # noqa: E402  (pins decbench, imports structscore)

SS = LD.SS


def arm(kuna, binary):
    os.environ["KUNA_BIN"] = kuna
    return LD.arm(binary, [])


def main():
    binary = Path(sys.argv[1])
    functions = SS.dwarf_functions(SS.twin_of(binary))
    pd, ld = arm("/home/mahaloz/kwt/_final-f/kuna", binary)
    pe, le = arm("/home/mahaloz/kwt/_final-g/kuna", binary)
    rd, re_ = LD.rows(binary, functions, pd, ld), LD.rows(binary, functions, pe, le)
    print(f"params typed as a struct with a header layout: F {len(rd)}  G {len(re_)}")
    by_addr = {int(f.get("address") or -1): f for f in pe.get("functions") or []}
    name_to_low = {info["name"]: low for low, info in functions.items()}
    for k in sorted(set(rd) | set(re_), key=str):
        a, b = rd.get(k), re_.get(k)
        if a == b or (a and b and a[1:] == b[1:] and a[4] == b[4] and a[3] == b[3]):
            continue
        etype = None
        fn = by_addr.get(name_to_low.get(k[0], -1))
        if fn:
            for v in fn.get("variables") or []:
                if v.get("arg_index") == k[1]:
                    etype = v.get("type")
        print(f"  {k[0]}(arg {k[1]}): F {a} | G {b} | G type {etype!r}")


if __name__ == "__main__":
    main()
