"""Classify every hunk between two `kuna decompile-all` dumps (foldcallretphi off vs on).

Per function, the two bodies are compared with every `vN` renamed to `V`, so a pure
renumbering is not a hunk at all, and each remaining hunk gets a class:

* fold -- a `V = f(...);` spill statement disappears and `f(` appears in the added lines;
* fold (split) -- the spill and its use fall in two different hunks;
* fold-in-condition -- the same inside a condition's comma expression,
  `(V = f(..), V == K)` -> `(f(..) == K)`; `+operand-hoist` when an argument load
  moved into its own comma clause ahead of the call;
* decl-only / blank -- declaration lines (and the blank line after an emptied list);
* uninline/reshape -- a value gains its own `V = expr;` statement at its defining op;
* other -- anything else, which has to be read by hand.

    python3 hunkclass.py [-v] <dir> <name>...   # reads <dir>/<name>.off.c, <dir>/<name>.on.c
"""
import difflib, re, sys, collections
FUNC = re.compile(r"^// Function: (\S+) @ (\S+)$")
V = re.compile(r"\bv\d+\b")
CALLSPILL = re.compile(r"^\s*V = (\(.*?\))?\s*([A-Za-z_]\w*)\((.*)\);$")
DECL = re.compile(r"^\s+[A-Za-z_][\w\s\*]*?\s\**V\s*(\[\d+\])?;(\s*//.*)?$")

def split(path):
    out, cur, name = {}, [], None
    for line in open(path, errors="replace").read().splitlines():
        m = FUNC.match(line)
        if m:
            if name: out[name] = cur
            name, cur = f"{m.group(1)}@{m.group(2)}", []
        elif name: cur.append(line)
    if name: out[name] = cur
    return out

def norm(lines): return [V.sub("V", l) for l in lines]

def classify(rem, add):
    """rem/add: normalized removed/added lines of one hunk."""
    rs = [l for l in rem if not DECL.match(l)]
    ad = [l for l in add if not DECL.match(l)]
    decl_r = [l for l in rem if DECL.match(l)]
    decl_a = [l for l in add if DECL.match(l)]
    rs = [l for l in rs if l.strip()]
    ad = [l for l in ad if l.strip()]
    if not rs and not ad:
        return "decl-only" if (decl_r or decl_a) else "blank"
    spills = [CALLSPILL.match(l) for l in rs]
    spills = [m for m in spills if m]
    if spills:
        ok = all(any(f"{m.group(2)}(" in a for a in ad) for m in spills)
        return "fold" if ok else "fold?"
    # a spill inside a condition's comma expression: `(V = f(..), V op K)` -> `(f(..) op K)`
    comma = [m.group(1) for l in rs for m in re.finditer(r"V = (?:\([\w \*]+\))?([A-Za-z_]\w*)\(", l)]
    if comma and len(rs) == len(ad) and all(any(f"{c}(" in a and f"V = {c}(" not in a for a in ad) for c in comma):
        clause = lambda ls: {m.group(0) for l in ls for m in re.finditer(r"V = [^,()]*(\[|\*)[^,]*, ", l)}
        hoist = bool(clause(ad) - clause(rs))
        return "fold-in-condition+operand-hoist" if hoist else "fold-in-condition"
    # un-inline: added lines `V = expr;` whose expr appeared in a removed line
    if ad and all(re.match(r"^\s*V = .*;$", a) or any(a.strip() and a.strip() in r for r in rs) for a in ad):
        return "uninline/reshape"
    return "other"

def main(d, names):
    tot = collections.Counter(); per = []
    for b in names:
        fo, fn = split(f"{d}/{b}.off.c"), split(f"{d}/{b}.on.c")
        for k in fo:
            if k not in fn: tot["missing_on"] += 1; continue
            if fo[k] == fn[k]: continue
            tot["changed_fn"] += 1
            a, c = norm(fo[k]), norm(fn[k])
            if a == c:
                tot["renumber-only_fn"] += 1; per.append((b, k, ["renumber-only"])); continue
            sm = difflib.SequenceMatcher(None, a, c, autojunk=False)
            hunks = [(a[i1:i2], c[j1:j2]) for tag, i1, i2, j1, j2 in sm.get_opcodes() if tag != "equal"]
            classes = [classify(r, ad) for r, ad in hunks]
            # a spill and its use landing in two different hunks
            for i, cl in enumerate(classes):
                if cl != "fold?": continue
                callees = [m.group(2) for m in map(CALLSPILL.match, hunks[i][0]) if m]
                for j in range(i + 1, len(hunks)):
                    if classes[j] == "other" and all(any(f"{f}(" in l for l in hunks[j][1]) for f in callees):
                        classes[i] = classes[j] = "fold (split)"
                        break
            for (r, ad), cl in zip(hunks, classes):
                tot[cl] += 1
                if "-v" in sys.argv:
                    print(f"--- {b} {k} [{cl}]")
                    for l in r: print("-" + l)
                    for l in ad: print("+" + l)
            per.append((b, k, classes))
        for k in fn:
            if k not in fo: tot["missing_off"] += 1
    for b, k, cl in per:
        print(f"{b:8s} {k:24s} {' '.join(cl)}")
    print("TOTALS", dict(tot))

if __name__ == "__main__":
    args = [a for a in sys.argv[1:] if a != "-v"]
    main(args[0], args[1:])
