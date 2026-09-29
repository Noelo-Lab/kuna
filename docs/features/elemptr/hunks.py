"""Classify every changed hunk between two castbench arms of one build (`--option elemptr off`
vs the default). Variable names are canonicalized per function by first use in the body, so a
renumbering is not a hunk. A hunk is classified when its non-cast identifier multiset is
unchanged -- except that a constant address may become the `dat_<addr>` it names -- and the
numbers it loses are element widths (1, 2, 4, 8) or those addresses:

  decl       only declarations moved (a type, or the storage comment)
  subscript  an access became a subscript (more `[`), or its casts moved
  casts      the same tokens up to casts
  loaddup    an explicit byte/word read became implied at its uses (one declaration fewer,
             the read's expression now printed at each use; upstream's term-count bound)
  loop       a while loop became the for loop that absorbs its iterator
Anything else is printed for reading.

usage: python3 hunks.py <off-arm> <on-arm> [--show N]
"""
import collections
import difflib
import re
import sys
from pathlib import Path

sys.path.insert(0, "/home/mahaloz/kwt/castbench")
import castbench as CB  # noqa: E402
import castcount as CC  # noqa: E402

TYPEWORDS = set(CC.BASE) | set(CC.QUAL) | set(CC.TAGKW) | set(CC.SEED)


def canon(text):
    lines = text.splitlines()
    k = next((i for i, l in enumerate(lines) if l.strip() == ""), 0)
    decl, rest = lines[:k + 1], lines[k + 1:]
    order = []
    for l in rest:
        for m in re.finditer(r"\bv\d+\b", l):
            if m.group(0) not in order:
                order.append(m.group(0))
    mp = {v: f"V{i}" for i, v in enumerate(order)}
    sub = lambda l: re.sub(r"\bv\d+\b", lambda m: mp.get(m.group(0), "Vx"), l)
    return sorted(sub(l) for l in decl if l.strip()), [sub(l) for l in rest]


def ids_nums(text, voc):
    toks = CC.tokenize(text)
    casts = CC.find_casts(toks, voc)
    ids = collections.Counter(t.text for t in toks if t.kind == "id" and t.text not in TYPEWORDS and t.text not in voc)
    nums = collections.Counter(t.text for t in toks if t.kind == "num")
    return ids, nums, len(casts)


def strip_casts(text, voc):
    toks = [t.text for t in CC.tokenize(text)]
    for c in CC.find_casts(CC.tokenize(text), voc):
        want = c[3]
        for i, t in enumerate(toks):
            if t != "(":
                continue
            acc = ""
            for j in range(i, len(toks)):
                acc += toks[j]
                if acc == want:
                    del toks[i:j + 1]
                    break
                if len(acc) >= len(want):
                    break
            else:
                continue
            break
    return [t for t in toks if t not in ("(", ")")]


def num(x):
    try:
        return int(x.rstrip("uUlL"), 0)
    except ValueError:
        return None


def classify(old, new, voc):
    io, no, co = ids_nums(old, voc)
    inn, nn, cn = ids_nums(new, voc)
    named = {f"dat_{num(x):x}" for x in no if x.startswith("0x") and num(x) is not None}
    gained_ids = inn - io
    lost_ids = io - inn
    if any(g not in named for g in gained_ids) or any(l for l in lost_ids):
        return None, co, cn
    lost = no - nn
    gained = nn - no
    lost_ok = all(num(x) is not None and (num(x) in (1, 2, 4, 8) or f"dat_{num(x):x}" in gained_ids)
                  for x in lost.elements())
    if gained or not lost_ok:
        return None, co, cn
    if strip_casts(old, voc) == strip_casts(new, voc):
        return "casts", co, cn
    if new.count("[") > old.count("[") or gained_ids or new.count("[") == old.count("["):
        return "subscript", co, cn
    return None, co, cn


def main():
    off, on = Path(sys.argv[1]), Path(sys.argv[2])
    show = int(sys.argv[sys.argv.index("--show") + 1]) if "--show" in sys.argv else 40
    cls = collections.Counter()
    fns = collections.Counter()
    odd = []
    bins = set()
    for opt, proj, b in CB.corpus("full"):
        try:
            a_src = (off / opt / proj / f"{b}.c").read_text(errors="replace")
            b_src = (on / opt / proj / f"{b}.c").read_text(errors="replace")
        except OSError:
            continue
        voc = CC.harvest_types(CC.tokenize(a_src), a_src) | CC.harvest_types(CC.tokenize(b_src), b_src)
        af = {a: t for _, a, t in CC.split_functions(a_src)}
        bf = {a: t for _, a, t in CC.split_functions(b_src)}
        for addr in af:
            if addr not in bf or af[addr] == bf[addr]:
                continue
            fns["changed"] += 1
            bins.add((opt, b))
            da, ra = canon(af[addr])
            db, rb = canon(bf[addr])
            if da != db:
                cls["decl"] += 1
            if len(da) > len(db):
                fns["one_declaration_fewer"] += 1
            for tag, i1, i2, j1, j2 in difflib.SequenceMatcher(None, ra, rb, autojunk=False).get_opcodes():
                if tag == "equal":
                    continue
                old = "\n".join(ra[i1:i2])
                new = "\n".join(rb[j1:j2])
                kind, co, cn = classify(old, new, voc)
                if kind is None and ("for (" in new and "while" in old):
                    kind = "loop"
                if kind is None and len(da) > len(db) and new.count("[") >= 2 and (i2 - i1) >= (j2 - j1):
                    kind = "loaddup"
                if kind is None:
                    cls["UNCLASSIFIED"] += 1
                    odd.append(f"{opt}/{b}@{hex(addr)}\n  - " + old.replace("\n", "\n  - ") + "\n  + " + new.replace("\n", "\n  + "))
                else:
                    cls[kind] += 1
    print(f"binaries with changes: {len(bins)}; functions changed: {fns['changed']}; "
          f"functions with a declaration fewer: {fns['one_declaration_fewer']}")
    print("hunks:", dict(cls))
    for o in odd[:show]:
        print(o)


if __name__ == "__main__":
    main()
