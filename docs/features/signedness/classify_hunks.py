#!/usr/bin/env python3
"""Classify every hunk of a whole-binary decompile-all diff, default(auto) vs upstream.

Usage:

    kuna decompile-all <bin>                            > DUMPS/<bin>.<opt>.auto.c
    kuna decompile-all <bin> --option signedness upstream > DUMPS/<bin>.<opt>.upstream.c
    python3 classify_hunks.py DUMPS <bin>[,<bin>...] <opt>[,<opt>...]

Every changed line-pair is a `decl-signedness` flip (same declaration, same name,
same base type at a different signedness), a `cast-dropped` hunk (identical after
deleting every integer cast token, with fewer of them on the new side), a
`CAST-ADDED` hunk, or it lands in an `OTHER` bucket that is printed with samples.
The claim the sweep makes is that the last two buckets stay empty.

The cast/declaration vocabulary covers all three printer spellings: `unsigned int`
(the `aggressive` mode the CLI selects under 500 KiB), `uint4` (`reliable`, what a
larger binary gets) and `unsigned long long` (an 8-byte local on a 32-bit target,
where an LP64 target says `unsigned long`).
"""
import re, sys, os, json, difflib
from collections import Counter, defaultdict

D = sys.argv[1]
BINS = sys.argv[2].split(",")
OPTS = sys.argv[3].split(",")

FN = re.compile(r"^// Function: (\S+) @ (0x[0-9a-f]+)")
BASE = r"(?:(?:unsigned |signed )?(?:char|short|int|long long|long)|u?int[1248]|u?char)"
DECL = re.compile(r"^  (" + BASE + r") (\*?)(\w+);(.*)$")
CAST = re.compile(r"\(" + BASE + r"\)")

def unsigned_stripped(t):
    return t.replace("unsigned ", "").replace("signed ", "").lstrip("u") if t.startswith("uint") else t.replace("unsigned ", "").replace("signed ", "")

def same_base(a, b):
    return unsigned_stripped(a) == unsigned_stripped(b) and a != b

def split_funcs(path):
    out, cur, name = {}, [], None
    for line in open(path, errors="replace"):
        m = FN.match(line)
        if m:
            if name: out[name] = cur
            name, cur = m.group(1) + "@" + m.group(2), [line]
        else:
            cur.append(line)
    if name: out[name] = cur
    return out

def classify(a, b):
    if len(a) == len(b) == 1:
        la, lb = a[0], b[0]
        ma, mb = DECL.match(la.rstrip("\n")), DECL.match(lb.rstrip("\n"))
        if ma and mb and ma.group(3) == mb.group(3) and ma.group(2) == mb.group(2):
            ta, tb = ma.group(1), mb.group(1)
            if same_base(ta, tb):
                return ("decl-signedness", f"{ta} -> {tb}")
            return ("decl-OTHER", f"{ta} -> {tb}")
        na, nb = len(CAST.findall(la)), len(CAST.findall(lb))
        if CAST.sub("", la) == CAST.sub("", lb):
            if nb < na: return ("cast-dropped", f"-{na-nb} cast(s): {la.strip()[:90]}")
            if nb > na: return ("CAST-ADDED", f"+{nb-na} cast(s): {la.strip()[:90]}")
        return ("OTHER-1to1", la.strip()[:110] + "  ||  " + lb.strip()[:110])
    return ("OTHER-multi", "".join(a).strip()[:110] + " || " + "".join(b).strip()[:110])

def main():
    grand = Counter(); rows = []; samples = defaultdict(list); dirs = Counter(); subint = Counter()
    for opt in OPTS:
        for b in BINS:
            pu, pa = f"{D}/{b}.{opt}.upstream.c", f"{D}/{b}.{opt}.auto.c"
            if not (os.path.exists(pu) and os.path.exists(pa)): continue
            fu, fa = split_funcs(pu), split_funcs(pa)
            flips = casts = other = 0; fn_changed = set()
            for name in fu:
                if name not in fa: continue
                A, B = fu[name], fa[name]
                if A == B: continue
                fn_changed.add(name)
                sm = difflib.SequenceMatcher(None, A, B, autojunk=False)
                for tag, i1, i2, j1, j2 in sm.get_opcodes():
                    if tag == "equal": continue
                    if tag == "replace":
                        la, lb = A[i1:i2], B[j1:j2]
                        pairs = [([x], [y]) for x, y in zip(la, lb)] if len(la) == len(lb) else [(la, lb)]
                    elif tag == "delete": pairs = [(A[i1:i2], [])]
                    else: pairs = [([], B[j1:j2])]
                    for pa_, pb_ in pairs:
                        k, detail = classify(pa_, pb_)
                        grand[k] += 1
                        if k == "decl-signedness":
                            flips += 1; dirs[detail] += 1
                            src = detail.split(" -> ")[0]
                            if src in ("char","unsigned char","signed char","short","unsigned short","int1","uint1","int2","uint2","uchar"):
                                subint[detail] += 1
                        elif k == "cast-dropped": casts += 1
                        else:
                            other += 1
                            if len(samples[k]) < 10: samples[k].append(f"{b}.{opt} {name}: {detail}")
            ntok_u = sum(len(CAST.findall(l)) for lines in fu.values() for l in lines)
            ntok_a = sum(len(CAST.findall(l)) for lines in fa.values() for l in lines)
            rows.append(dict(bin=b, opt=opt, funcs=len(fu), funcs_changed=len(fn_changed),
                             decl_flips=flips, cast_hunks=casts, other_hunks=other,
                             cast_tokens_upstream=ntok_u, cast_tokens_auto=ntok_a))
    print(f"{'bin':6}{'opt':5}{'funcs':>7}{'chg':>6}{'flip':>6}{'cast-':>7}{'other':>7}{'castTokU':>10}{'castTokA':>10}")
    for r in rows:
        print(f"{r['bin']:6}{r['opt']:5}{r['funcs']:>7}{r['funcs_changed']:>6}{r['decl_flips']:>6}{r['cast_hunks']:>7}{r['other_hunks']:>7}{r['cast_tokens_upstream']:>10}{r['cast_tokens_auto']:>10}")
    tot = lambda k: sum(r[k] for r in rows)
    print(f"\nTOTAL funcs={tot('funcs')} changed={tot('funcs_changed')} decl_flips={tot('decl_flips')} "
          f"cast_hunks={tot('cast_hunks')} other={tot('other_hunks')} "
          f"cast_tokens {tot('cast_tokens_upstream')} -> {tot('cast_tokens_auto')} "
          f"(net removed {tot('cast_tokens_upstream')-tot('cast_tokens_auto')})")
    print("hunk classes:", dict(grand))
    print("flip directions:", dict(dirs.most_common()))
    print("SUB-INT flips:", dict(subint) or 0)
    for k, v in samples.items():
        print(f"\n--- {k} ---")
        for x in v: print("   ", x)
    json.dump(dict(rows=rows, classes=dict(grand), dirs=dict(dirs), subint=dict(subint)),
              open(f"{D}/summary-auto-vs-upstream.json", "w"), indent=2)

main()
