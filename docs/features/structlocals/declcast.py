"""Declaration- and cast-aware classification of two decompile-all arms.

    python3 declcast.py <dir> <armA> <armB> [--report out.txt]

canon.py compares bodies with declarations dropped and casts stripped, so it
cannot see a record type that became the declared type of a variable holding
other values. This classifier reads what canon.py drops. For every function
that differs beyond struct_N numbering it compares:

* the signature and every declaration (struct numbering collapsed);
* for each variable declared `struct_N *` in B but not in A, every value
  assigned to it in B: one call's result (cast to the record or not), a copy of
  another such variable, or zero is the documented effect; anything else --
  a second call, a cast to another type, a string, a length -- is flagged;
* every use of such a variable cast to a non-record type in B (`(char *)v11`);
* every `return` of a function whose return type became `struct_N *`: a
  variable of that record or zero is clean, `return (struct_N *)count();` is
  flagged;
* every `(struct_N *)` cast new in B that is not the defining call of such a
  variable.

A function with no flag whose bodies are equal after canon.py's respelling is
"clean"; one with no flag and a canon residue is "residue" (read by hand); one
with a flag is "flagged" and listed with its flags.
"""
import collections
import pathlib
import re
import sys

sys.path.insert(0, str(pathlib.Path(__file__).parent))
import canon  # noqa: E402

S = re.compile(r"\bstruct_\d+\b")
DECL = re.compile(r"^\s*((?:unsigned |signed |const |struct )*[A-Za-z_]\w*)\s*(\**)\s*([av]\d+)(\s*\[\w+\])?;")
CASTTYPE = r"(?:unsigned |signed |const |struct )*[A-Za-z_]\w*(?: \w+)?\s*\**"


def norm(t):
    return S.sub("struct_N", t.strip())


def parse(lines):
    sig = next((l for l in lines if "(" in l), "")
    m = re.match(r"\s*(.*?)\s*\b([A-Za-z_]\w*)\s*\((.*)\)", sig)
    ret = norm(m.group(1)) if m else ""
    decls = {}
    if m:
        for p in m.group(3).split(","):
            mm = re.match(r"\s*(.*?)\s*(\**)\s*(a\d+)\s*$", p)
            if mm:
                decls[mm.group(3)] = norm(mm.group(1) + " " + mm.group(2)).replace(" *", " *")
    body = []
    for l in lines[1:]:
        mm = DECL.match(l)
        if mm and not mm.group(4) and mm.group(1) not in ("return", "goto", "case", "else") and mm.group(3) not in decls:
            decls[mm.group(3)] = norm(mm.group(1) + " " + mm.group(2))
        else:
            body.append(re.sub(r"\s*//.*$", "", l))
    # Locals are numbered by the printer, so one split variable renumbers every
    # later one; name them by first use instead, so the two arms line up.
    order = {}
    for l in body:
        for m in re.finditer(r"\bv\d+\b", l):
            order.setdefault(m.group(0), f"v{len(order) + 1:03d}")
    for v in decls:
        if v.startswith("v"):
            order.setdefault(v, f"v9{len(order):02d}")
    ren = lambda l: re.sub(r"\bv\d+\b", lambda m: order.get(m.group(0), m.group(0)), l)
    return ret, {order.get(k, k): t for k, t in decls.items()}, [ren(l) for l in body]


def is_record(t):
    return t.replace(" ", "") == "struct_N*"


def strip_record_cast(e):
    return re.sub(r"^\(struct_\d+ \*\)", "", e.strip())


def flags_for(A, B):
    ra, da, ba = parse(A)
    rb, db, bb = parse(B)
    before = {norm(l).strip() for l in ba}
    flags, changes = [], []
    new_line = lambda l: norm(l).strip() not in before
    if set(da) != set(db):
        flags.append(f"variables differ: {len(da)} -> {len(db)}")
    for v in sorted(set(da) & set(db)):
        if da[v] != db[v]:
            changes.append((v, da[v], db[v]))
            if not is_record(db[v]):
                flags.append(f"declaration {v}: {da[v]} -> {db[v]}")
    rec_vars = {v for v, t in db.items() if is_record(t)}
    defining = set()
    for v in sorted(rec_vars):
        rhs = []
        for l in bb:
            m = re.match(rf"^\s*{v} = (.+);$", l)
            if m:
                rhs.append((m.group(1), l))
        callees = set()
        for e, l in rhs:
            x = strip_record_cast(e)
            if re.fullmatch(r"(0|NULL|\(\w+ \*\)0)", x):
                continue
            if re.fullmatch(r"[av]\d+", x) and x in rec_vars:
                continue
            if re.fullmatch(r"&?[av]\d+(->field_0x[0-9a-f]+|\[\d+\]\.field_0x[0-9a-f]+)*", x):
                continue
            c = re.fullmatch(r"([A-Za-z_]\w*|\(\*[av]\d+\))\(.*\)", x)
            if c:
                callees.add(c.group(1))
                defining.add(l_key(v, e))
                continue
            if new_line(l):
                flags.append(f"{v} (record) assigned {e}")
        if len(callees) > 1:
            flags.append(f"{v} (record) assigned the results of {sorted(callees)}")
        for l in bb:
            if re.search(rf"\b{v}->field_0x[0-9a-f]+(\[\w+\])? = (s_\w+|\")", l) and new_line(l):
                flags.append(f"{v} (record) field stores a string literal: {l.strip()}")
                break
        for l in bb:
            if not new_line(l):
                continue
            for m in re.finditer(rf"\(({CASTTYPE})\)\s*{v}\b(?!->|\[)", l):
                t = norm(m.group(1))
                if is_record(t) or t == "void *":
                    continue
                if not t.endswith("*") and re.search(rf"= \({re.escape(m.group(1))}\)\s*{v};$", l):
                    continue
                flags.append(f"{v} (record) used as ({t}): {l.strip()}")
                break
    if is_record(rb):
        for l in bb:
            m = re.match(r"^\s*return (.+);$", l)
            if not m or not new_line(l):
                continue
            x = m.group(1).strip()
            if x in rec_vars or re.fullmatch(r"(0|NULL|\(struct_\d+ \*\)0)", x):
                continue
            flags.append(f"record return of {x}")
    if rb != ra and not is_record(rb):
        flags.append(f"return type {ra} -> {rb}")
    for l in bb:
        if not re.search(r"\(struct_\d+ \*\)", l) or not new_line(l):
            continue
        mm = re.match(r"^\s*([av]\d+) = (\(struct_\d+ \*\).+);$", l)
        if mm and l_key(mm.group(1), mm.group(2)) in defining:
            continue
        flags.append(f"new record cast: {l.strip()}")
    return changes, sorted(set(flags))


def l_key(v, e):
    return (v, strip_record_cast(e))


def main():
    d, A, B = sys.argv[1:4]
    rep = sys.argv[5] if len(sys.argv) > 5 and sys.argv[4] == "--report" else None
    tot = collections.Counter()
    out = []
    for f in sorted(pathlib.Path(d).glob(f"*.{A}.c")):
        t = f.name[: -len(f".{A}.c")]
        a, b = canon.split(f), canon.split(pathlib.Path(d) / f"{t}.{B}.c")
        for k in sorted(set(a) | set(b)):
            X, Y = a.get(k, []), b.get(k, [])
            if X == Y:
                continue
            tot["changed"] += 1
            if [S.sub("S", l) for l in X] == [S.sub("S", l) for l in Y]:
                tot["numbering only"] += 1
                continue
            changes, flags = flags_for(X, Y)
            same = canon.canonical(X) == canon.canonical(Y)
            cls = "flagged" if flags else ("clean" if same else "residue")
            tot[cls] += 1
            if cls != "clean":
                out.append((t, k, cls, changes, flags))
    print(dict(tot))
    if rep:
        with open(rep, "w") as fo:
            for t, k, cls, changes, flags in out:
                fo.write(f"=== {t} {k} {cls}\n")
                for v, x, y in changes:
                    fo.write(f"  decl {v}: {x} -> {y}\n")
                for fl in flags:
                    fo.write(f"  FLAG {fl}\n")


if __name__ == "__main__":
    main()
