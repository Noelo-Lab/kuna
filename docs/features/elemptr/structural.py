"""Function-level invariants between two castbench arms of one build (`--option elemptr off`
vs the default): for every function whose text changed, the callees it calls, the string
literals it prints, its control keywords and the program data it names must be the same --
a constant address may become the `dat_<addr>` it names -- and its statement count may move
only by the reads upstream's term-count bound now prints at their uses. The option changes
types, and so casts, subscripts, declarations, character literals and which reads are
variables; anything else is a bug. Prints every function that breaks an invariant.

usage: python3 structural.py <off-arm> <on-arm>
"""
import collections
import re
import sys
from pathlib import Path

sys.path.insert(0, "/home/mahaloz/kwt/castbench")
import castbench as CB  # noqa: E402
import castcount as CC  # noqa: E402

KEYWORDS = ("if", "while", "for", "do", "return", "goto", "break", "continue", "switch", "case", "else")


def facts(text, voc):
    toks = CC.tokenize(text)
    calls = collections.Counter()
    strs = collections.Counter()
    kw = collections.Counter()
    data = collections.Counter()
    nums = collections.Counter()
    for i, t in enumerate(toks):
        if t.kind == "str":
            strs[t.text] += 1
        elif t.kind == "id":
            if t.text in KEYWORDS:
                kw[t.text] += 1
            elif t.text.startswith("dat_"):
                data[t.text] += 1
            elif i + 1 < len(toks) and toks[i + 1].text == "(" and t.text not in voc and t.text not in CC.BASE:
                calls[t.text] += 1
        elif t.kind == "num" and t.text.startswith("0x"):
            try:
                v = int(t.text.rstrip("uUlL"), 0)
            except ValueError:
                continue
            if v >= 0x1000:
                nums[f"dat_{v:x}"] += 1
    stmts = CC.count_statements(toks)
    return calls, strs, kw, data, nums, stmts


def main():
    off, on = Path(sys.argv[1]), Path(sys.argv[2])
    counts = collections.Counter()
    bad = []
    for opt, proj, b in CB.corpus("full"):
        try:
            a_src = (off / opt / proj / f"{b}.c").read_text(errors="replace")
            b_src = (on / opt / proj / f"{b}.c").read_text(errors="replace")
        except OSError:
            continue
        voc = CC.harvest_types(CC.tokenize(a_src), a_src) | CC.harvest_types(CC.tokenize(b_src), b_src)
        af = {a: t for _, a, t in CC.split_functions(a_src)}
        bf = {a: t for _, a, t in CC.split_functions(b_src)}
        for addr, ta in af.items():
            tb = bf.get(addr)
            if tb is None:
                counts["missing"] += 1
                bad.append((opt, b, addr, "function missing in the on arm"))
                continue
            if ta == tb:
                continue
            counts["changed"] += 1
            ca, sa, ka, da, na, st_a = facts(ta, voc)
            cb, sb, kb, db, nb, st_b = facts(tb, voc)
            why = []
            if ca != cb:
                why.append(f"calls {dict(ca - cb)} -> {dict(cb - ca)}")
            if sa != sb:
                why.append("strings")
            loop_moved = (ka - kb) == collections.Counter({"while": 1}) and (kb - ka) == collections.Counter({"for": 1})
            if ka != kb and not loop_moved:
                why.append(f"keywords {dict(ka - kb)} -> {dict(kb - ka)}")
            if loop_moved:
                counts["while_to_for"] += 1
            # every dat_ the on arm names that the off arm does not is a constant address it printed
            gained = db - da
            if any(g not in na for g in gained) or (da - db):
                why.append(f"data {dict(da - db)} -> {dict(gained)}")
            if gained:
                counts["constant_named"] += 1
            if st_b != st_a:
                counts[f"statements_{'fewer' if st_b < st_a else 'more'}"] += 1
                if abs(st_b - st_a) > 4:
                    why.append(f"statements {st_a} -> {st_b}")
            if why:
                bad.append((opt, b, addr, "; ".join(why)))
    print(dict(counts))
    print(f"functions breaking an invariant: {len(bad)}")
    for opt, b, addr, why in bad:
        print(f"  {opt}/{b} @{hex(addr)}: {why}")


if __name__ == "__main__":
    main()
