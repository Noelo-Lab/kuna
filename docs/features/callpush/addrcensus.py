#!/usr/bin/env python3
"""Census of `(T *)&<obj>` casts on the castbench shared set, by what <obj> is.

usage: addrcensus.py ARM_DIR [--set full|small] [--examples N] [--json OUT]
"""
import collections, json, re, sys
from pathlib import Path

sys.path.insert(0, "/home/mahaloz/kwt/castbench")
import castbench as CB
import castcount as CC

DECL = re.compile(r"^\s*([A-Za-z_][\w ]*?[\w*\] ]*?)\s*(\**)\s*([av]\d+|dat_\w+)\s*(\[[^\]]*\])?\s*;(\s*//\s*(.*))?$")


def decls(text):
    d = {}
    for L in text.splitlines():
        m = re.match(r"^\s{2}(?!return|if|while|for|do|goto|switch)([A-Za-z_][\w ]*?)\s*(\**)\s*(v\d+)\s*(\[[^\]]*\])?\s*;(?:\s*//\s*(.*))?$", L)
        if m:
            base, stars, name, arr, com = m.groups()
            d[name] = (base.strip(), len(stars), bool(arr), (com or "").strip())
    m = re.search(r"\(([^)]*)\)\s*(//.*)?\n\{", text)
    if m:
        for p in m.group(1).split(","):
            p = p.strip()
            mm = re.match(r"(.*?)(\**)\s*(a\d+)$", p)
            if mm:
                d[mm.group(3)] = (mm.group(1).strip(), len(mm.group(2)), False, "param")
    return d


def classify(toks, k, dd):
    """toks[k] is '&'. Return (klass, detail)."""
    n = len(toks)
    j = k + 1
    if j >= n:
        return "other", ""
    root = toks[j].text
    nxt = toks[j + 1].text if j + 1 < n else ""
    dec = dd.get(root)
    dtxt = "?" if dec is None else (dec[0] + "*" * dec[1] + ("[]" if dec[2] else "") + (" //" + dec[3] if dec[3] else ""))
    if root.startswith("dat_") or (dec is None and toks[j].kind == "id" and not re.match(r"[av]\d+$", root)):
        form = "global"
    elif nxt == "->":
        form = "ptrfield"
    elif nxt == ".":
        form = "field"
    elif nxt == "[":
        # index: negative?
        neg = j + 2 < n and toks[j + 2].text == "-"
        form = "index-neg" if neg else "index"
    else:
        form = "plain"
    # is it written (cast is at statement start, '*' before cast, and '=' follows the operand)?
    return form, dtxt


def main():
    arm = sys.argv[1]
    which = "full"
    if "--set" in sys.argv:
        which = sys.argv[sys.argv.index("--set") + 1]
    nex = int(sys.argv[sys.argv.index("--examples") + 1]) if "--examples" in sys.argv else 3
    cnt = collections.Counter()
    by_ty = collections.Counter()
    ex = collections.defaultdict(list)
    per_bin = collections.Counter()
    total = 0
    for opt, proj, b in CB.corpus(which):
        path = Path(arm) / opt / proj / f"{b}.c"
        ida = CB.funcs(CB.RES / opt / proj / "decompiled" / f"ida_{b}.c")
        if not path.exists() or not ida:
            continue
        src = path.read_text(errors="replace")
        vocab = CC.harvest_types(CC.tokenize(src), src)
        for name, addr, text in CC.split_functions(src):
            if addr not in ida:
                continue
            toks = CC.tokenize(text)
            dd = decls(text)
            lines = text.splitlines()
            match = CC.matching(toks)
            for ty, shape, line, ctext in CC.find_casts(toks, vocab):
                if shape != "<addr>" or "*" not in ty:
                    continue
                # locate the cast token index by line+text
                for i, t in enumerate(toks):
                    if t.line == line and t.text == "(" and match.get(i) is not None:
                        jj = match[i]
                        if "".join(x.text for x in toks[i:jj + 1]) == ctext and jj + 1 < len(toks) and toks[jj + 1].text == "&":
                            form, dtxt = classify(toks, jj + 1, dd)
                            break
                else:
                    form, dtxt = "unlocated", ""
                total += 1
                key = (form, dtxt.split(" //")[0] if form != "index-neg" else dtxt)
                cnt[form] += 1
                by_ty[(form, ty, dtxt)] += 1
                per_bin[(opt, b, name, form)] += 1
                if len(ex[form]) < nex:
                    ex[form].append(f"{opt}/{b}/{name}: {lines[line - 1].strip() if line - 1 < len(lines) else ''}  [{dtxt}]")
    print(f"total (T*)&x casts on shared set: {total}")
    for f, n in cnt.most_common():
        print(f"  {f:10} {n:6}")
        for e in ex[f]:
            print(f"      {e[:200]}")
    print("top (form, cast type, declared type):")
    for (f, ty, d), n in by_ty.most_common(40):
        print(f"  {n:5}  {f:10} ({ty})  decl={d}")
    print("top functions:")
    for (o, b, nm, f), n in per_bin.most_common(25):
        print(f"  {n:5}  {o}/{b} {nm} {f}")


if __name__ == "__main__":
    main()
