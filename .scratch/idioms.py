#!/usr/bin/env python3
"""Classify casts into IDIOMS (composite patterns), not just single shapes.

  ptr_int_ptr   *(T *)((long)P + K)   -- a pointer is cast to an integer to do
                                         byte arithmetic, then back to a
                                         pointer.  TWO casts for one field
                                         access; IDA writes one, or zero with a
                                         struct field.
  offset_access *(T *)(X + K)         -- one cast: the access type on an
                                         integer base (what IDA does).
  int_to_ptr    (T *)V   where V is an integer-typed value
  ptr_to_int    (long)P  where P is a pointer-typed value, NOT inside a ptr cast
  widen/narrow  (intT)V  integer<->integer
  other
"""
import sys, collections, json
from pathlib import Path
sys.path.insert(0, str(Path(__file__).parent))
import castcount as CC

INT_T = {"i8", "u8", "i16", "u16", "i32", "u32", "i64", "u64", "i128", "u128",
         "char", "uchar", "bool", "undef"}


def is_ptr(t):
    return t.endswith("*")


def decl_types(toks, vocab):
    """{var name: normalized declared type} from declarations in this text."""
    out, n = {}, len(toks)
    for i in range(n):
        t = toks[i]
        if t.kind != "id":
            continue
        if i > 0 and toks[i - 1].text not in (";", "{", "}", "(", ",", ")"):
            continue
        # specifier list
        j, specs, tag, saw = i, [], None, False
        while j < n and toks[j].kind == "id":
            x = toks[j].text
            if x in CC.QUAL:
                j += 1; continue
            if x in CC.TAGKW and j + 1 < n and toks[j + 1].kind == "id":
                tag = (x, toks[j + 1].text); j += 2; saw = True; continue
            if x in CC.BASE:
                specs.append(x); j += 1; saw = True; continue
            if not saw and x in vocab:
                specs.append(x); j += 1; saw = True; continue
            break
        if not saw:
            continue
        stars = 0
        while j < n and (toks[j].text == "*" or toks[j].text in CC.QUAL):
            if toks[j].text == "*":
                stars += 1
            j += 1
        if j < n and toks[j].kind == "id" and toks[j].text not in CC.BASE:
            nm = toks[j].text
            k = j + 1
            if k < n and toks[k].text in (";", "=", ",", ")", "["):
                ty = CC.normalize(specs, tag, stars, False)
                if ty:
                    out.setdefault(nm, ty)
    return out


def classify(src):
    toks = CC.tokenize(src)
    vocab = CC.harvest_types(toks, src)
    dt = decl_types(toks, vocab)
    match = CC.matching(toks)
    casts = []
    cast_at = {}
    cast_close = set()
    n = len(toks)
    for i in range(n):
        if toks[i].text != "(":
            continue
        j = match.get(i)
        if j is None or j == i + 1:
            continue
        if not CC.cast_allowed_before(toks, i, cast_close):
            continue
        k = j + 1
        if k >= n:
            continue
        nt = toks[k]
        if not (nt.kind in ("id", "num", "str", "chr")
                or nt.text in CC.UNARY_START_PUNCT):
            continue
        ty = CC.parse_type_name(toks, i + 1, j, vocab)
        if ty is None:
            continue
        cast_close.add(j)
        casts.append((i, j, ty))
        cast_at[i] = (j, ty)

    kinds = collections.Counter()
    inner_of = set()
    # a pointer cast whose operand is a paren containing an int cast
    for (i, j, ty) in casts:
        if not is_ptr(ty):
            continue
        k = j + 1
        if k >= n or toks[k].text != "(":
            continue
        close = match.get(k)
        if close is None:
            continue
        found = None
        for (i2, j2, ty2) in casts:
            if k < i2 < close and ty2 in INT_T and not is_ptr(ty2):
                # is its operand a declared pointer?
                o = toks[j2 + 1]
                found = (i2, o.kind == "id" and is_ptr(dt.get(o.text, "")))
                break
        if found:
            inner_of.add(found[0])
            kinds["ptr_int_ptr" if found[1] else "ptr_intexpr_ptr"] += 1

    for (i, j, ty) in casts:
        if i in inner_of:
            kinds["--inner-int-cast-of-the-above"] += 1
            continue
        op = toks[j + 1]
        opty = dt.get(op.text) if op.kind == "id" else None
        if is_ptr(ty):
            if opty and is_ptr(opty):
                kinds["ptr_to_ptr"] += 1
            elif opty and not is_ptr(opty):
                kinds["int_to_ptr(declared int)"] += 1
            elif toks[j + 1].text == "&":
                kinds["addr_to_ptr"] += 1
            elif toks[j + 1].text == "(":
                kinds["ptr_cast_of_expr"] += 1
            else:
                kinds["ptr_cast_other"] += 1
        else:
            if opty and is_ptr(opty):
                kinds["ptr_to_int"] += 1
            elif opty:
                kinds["int_to_int(declared)"] += 1
            elif ty == "void":
                kinds["void_discard"] += 1
            else:
                kinds["int_cast_other"] += 1
    return kinds, len(casts)


def main():
    files = sys.argv[1:]
    tot = collections.Counter()
    allc = 0
    for f in files:
        k, c = classify(Path(f).read_text(errors="replace"))
        tot += k
        allc += c
    print(f"total casts {allc}")
    for k, v in tot.most_common():
        print(f"  {k:<36}{v:>8}  {100.0*v/allc:5.1f}%")


if __name__ == "__main__":
    main()
