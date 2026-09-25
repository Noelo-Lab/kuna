#!/usr/bin/env python3
"""Census of casts at the top of a conditional's arm in kuna output (castbench arm)."""
import collections, re, sys
from pathlib import Path
sys.path.insert(0, "/home/mahaloz/kwt/castbench")
import castcount as CC
import castbench as CB

arm = Path(sys.argv[1])
which = sys.argv[2] if len(sys.argv) > 2 else "full"
show = int(sys.argv[3]) if len(sys.argv) > 3 else 0
DECL = re.compile(r"^\s*([A-Za-z_][\w ]*?[\w*])\s*\**\s*(\w+)\s*;", re.M)

def decls(text):
    d = {}
    for m in re.finditer(r"^\s*((?:unsigned |signed |const )*(?:char|short|int|long|bool|float|double|void|struct \w+|\w+)(?: long)?(?: int)?)\s*(\**)\s*(\w+)\s*(?:\[\w*\])?;", text, re.M):
        d[m.group(3)] = (m.group(1) + " " + m.group(2)).strip()
    hdr = text.split("{", 1)[0]
    for m in re.finditer(r"((?:unsigned |signed |const )*(?:char|short|int|long|bool|float|double|void|struct \w+|\w+)(?: long)?(?: int)?)\s*(\**)\s*(a\d+)\b", hdr):
        d[m.group(3)] = (m.group(1) + " " + m.group(2)).strip()
    return d

tot = collections.Counter(); shapes = collections.Counter(); examples = collections.defaultdict(list)
nfun = 0; nfun_shared = 0; sites = []
for opt, proj, b in CB.corpus(which):
    p = arm / opt / proj / f"{b}.c"
    ip = CB.RES / opt / proj / "decompiled" / f"ida_{b}.c"
    if not p.exists():
        continue
    ida = CB.funcs(ip) or {}
    src = p.read_text(errors="replace")
    vocab = CC.harvest_types(CC.tokenize(src), src)
    for name, addr, text in CC.split_functions(src):
        toks = CC.tokenize(text)
        casts = CC.find_casts(toks, vocab)
        if not any(t.text == "?" for t in toks):
            continue
        match = CC.matching(toks)
        opens = {}
        for (ty, sh, line, span) in casts:
            pass
        # map cast open token index -> cast
        castidx = {}
        n = len(toks)
        i = 0
        cset = set((c[2], c[3]) for c in casts)
        D = decls(text)
        for qi, t in enumerate(toks):
            if t.text != "?":
                continue
            # statement: find start (prev ';' or '{' or '}') and dest
            s = qi
            while s > 0 and toks[s - 1].text not in (";", "{", "}"):
                s -= 1
            dest = toks[s].text if s + 1 < n and toks[s + 1].text == "=" else None
            # find ':' at depth 0 after '?'
            depth = 0; ci = None; j = qi + 1
            while j < n:
                x = toks[j].text
                if x in "([": depth += 1
                elif x in ")]": depth -= 1
                elif x == "?" and depth == 0: depth += 100
                elif x == ":" and depth >= 100: depth -= 100
                elif x == ":" and depth == 0: ci = j; break
                elif x == ";": break
                j += 1
            if ci is None: continue
            e = ci + 1; depth = 0
            while e < n and not (toks[e].text in (";", ",") and depth == 0) and not (toks[e].text == ")" and depth == 0):
                if toks[e].text in "([": depth += 1
                elif toks[e].text in ")]": depth -= 1
                e += 1
            arms = [(qi + 1, ci), (ci + 1, e)]
            for k, (lo, hi) in enumerate(arms):
                if toks[lo].text != "(": continue
                cl = match.get(lo)
                if cl is None: continue
                span = "".join(x.text for x in toks[lo:cl + 1])
                if (toks[lo].line, span) not in cset: continue
                ty = span[1:-1]
                # whole-arm: cast operand is a unary expression ending at hi
                u = cl + 1
                while u < hi and toks[u].text in ("*","&","-","~","!","+"): u += 1
                while u < hi and toks[u].text == "(" and match.get(u) and (toks[u].line, "".join(x.text for x in toks[u:match[u]+1])) in cset:
                    u = match[u] + 1
                    while u < hi and toks[u].text in ("*","&","-","~","!","+"): u += 1
                if u < hi and toks[u].text == "(":
                    u = match.get(u, hi) + 1
                elif u < hi:
                    u += 1
                while u < hi and toks[u].text in ("[","(",".","->"):
                    if toks[u].text in ("[","("): u = match.get(u, hi) + 1
                    else: u += 2
                if u != hi: continue
                other = arms[1 - k]
                otxt = " ".join(x.text for x in toks[other[0]:other[1]])
                # does the cast cover the whole arm? (cast + unary operand)
                armtxt = " ".join(x.text for x in toks[lo:hi])
                oshape = ("const" if re.fullmatch(r"-?\s*(0x[0-9a-f]+|\d+)|'.*'", otxt) else
                          "cast" if otxt.startswith("(") and any((toks[other[0]].line, "".join(x.text for x in toks[other[0]:match.get(other[0], other[0]) + 1])) == c for c in cset) else
                          "NULL" if otxt == "NULL" else "str" if otxt.startswith('"') else "other")
                dty = D.get(dest, "?")
                ptr = "*" in ty
                cls = ("ptr" if ptr else "float" if ty in ("float", "double") else "bool" if ty == "bool" else "int")
                key = (cls, ty, oshape, "dest=" + ("same" if dty == ty else dty))
                tot[cls] += 1
                shapes[key] += 1
                sh = addr in ida
                sites.append((opt, proj, b, addr, sh, key, " ".join(x.text for x in toks[s:e + 1])))
                if len(examples[key]) < 3:
                    examples[key].append((opt, b, hex(addr), " ".join(x.text for x in toks[s:e + 1])[:200]))
print("arm-top casts by class:", dict(tot), " sites", len(sites), " in shared fns", sum(1 for s in sites if s[4]))
for k, v in shapes.most_common(60):
    print(f"{v:5}  {k}")
    if show:
        for ex in examples[k][:show]:
            print("        ", ex)
import json
Path(sys.argv[1].rstrip("/").split("/")[-1] + "-sites.json").write_text(json.dumps(sites))
