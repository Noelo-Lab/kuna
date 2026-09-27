#!/usr/bin/env python3
"""Census of '(T *)&x' casts on the castbench shared set: what x is, the context, x's declared type."""
import sys, re, json, collections
from pathlib import Path
sys.path.insert(0, "/home/mahaloz/kwt/castbench")
import castcount as CC
import castbench as CB

def casts_idx(toks, vocab):
    n = len(toks); match = CC.matching(toks); out = []; cast_close = set()
    for i in range(n):
        if toks[i].text != "(": continue
        j = match.get(i)
        if j is None or j == i + 1: continue
        if not CC.cast_allowed_before(toks, i, cast_close): continue
        k = j + 1
        if k >= n: continue
        nt = toks[k]
        if not (nt.kind in ("id","num","str","chr") or nt.text in CC.UNARY_START_PUNCT): continue
        ty = CC.parse_type_name(toks, i+1, j, vocab)
        if ty is None: continue
        cast_close.add(j)
        out.append((i, j, ty))
    return out, match

def operand_after_amp(toks, k):
    # toks[k] == '&'; collect id ( -> id | . id | [ ... ] )*
    n = len(toks); p = k + 1; parts = []
    if p >= n or toks[p].kind != "id": return None, p
    parts.append(toks[p].text); p += 1
    while p < n:
        if toks[p].text in ("->", ".") and p + 1 < n and toks[p+1].kind == "id":
            parts.append(toks[p].text + toks[p+1].text); p += 2
        elif toks[p].text == "[":
            d = 0; q = p
            while q < n:
                if toks[q].text == "[": d += 1
                elif toks[q].text == "]":
                    d -= 1
                    if d == 0: break
                q += 1
            parts.append("".join(t.text for t in toks[p:q+1])); p = q + 1
        else: break
    return "".join(parts), p

DECL_RE = re.compile(r"^\s+([A-Za-z_][\w ]*?[\w*]\s*\**)\s*\b([av]\d+)\s*(\[[^\]]*\])?\s*;(.*)$")

def decls(text):
    d = {}
    for L in text.splitlines():
        m = DECL_RE.match(L)
        if m:
            d[m.group(2)] = (m.group(1).strip() + (" " + m.group(3) if m.group(3) else ""), m.group(4).strip())
    # params from header
    hdr = text.splitlines()[1] if len(text.splitlines()) > 1 else ""
    m = re.search(r"\((.*)\)", hdr)
    if m:
        for p in m.group(1).split(","):
            mm = re.match(r"\s*(.*?)\b(a\d+)\s*$", p)
            if mm: d[mm.group(2)] = (mm.group(1).strip(), "param")
    return d

def context(toks, i, j, match, end):
    # what encloses the cast
    prev = toks[i-1].text if i > 0 else ""
    if prev == "*": 
        # deref: is it an lvalue (followed by '=' not '==')?
        # find end of the deref expression = end
        nx = toks[end].text if end < len(toks) else ""
        if nx == "=": return "deref-write"
        if nx in ("+=","-=","|=","&=","^=","<<=",">>=","*=","/=","%=","++","--"): return "deref-rmw"
        return "deref-read"
    if prev in ("(", ","):
        # call argument? find the enclosing '(' and check an identifier before it
        d = 0; q = i - 1
        while q >= 0:
            t = toks[q].text
            if t == ")": d += 1
            elif t == "(":
                if d == 0: break
                d -= 1
            q -= 1
        if q > 0 and toks[q-1].kind == "id" and toks[q-1].text not in ("if","while","for","switch","return","sizeof"):
            nx = toks[end].text if end < len(toks) else ""
            if nx in (",", ")"): return "call-arg:" + toks[q-1].text
        if prev == "(" and end < len(toks) and toks[end].text == ")" and end + 1 < len(toks) and toks[end+1].text == "[":
            return "paren-index"
        return "paren"
    if prev == "=": return "assign-rhs"
    if prev == "return": return "return"
    return "other:" + prev

def main():
    out = sys.argv[1]; which = sys.argv[2] if len(sys.argv) > 2 else "full"
    rows = []
    for opt, proj, b in CB.corpus(which):
        kp = Path(out)/opt/proj/f"{b}.c"; ip = CB.RES/opt/proj/"decompiled"/f"ida_{b}.c"
        if not kp.exists() or not ip.exists(): continue
        ks = kp.read_text(errors="replace"); isrc = ip.read_text(errors="replace")
        ida_addrs = {a for _, a, _ in CC.split_functions(isrc)}
        vocab = CC.harvest_types(CC.tokenize(ks), ks)
        for name, addr, text in CC.split_functions(ks):
            shared = addr in ida_addrs
            toks = CC.tokenize(text)
            cl, match = casts_idx(toks, vocab)
            dd = None
            for i, j, ty in cl:
                if not ty.endswith("*"): continue
                if j + 1 >= len(toks) or toks[j+1].text != "&": continue
                opnd, end = operand_after_amp(toks, j+1)
                if opnd is None: continue
                if dd is None: dd = decls(text)
                base = re.match(r"[A-Za-z_]\w*", opnd).group(0)
                kind = ("field" if ("->" in opnd or "." in opnd) else
                        "index" if "[" in opnd else
                        "local" if re.fullmatch(r"v\d+", base) else
                        "param" if re.fullmatch(r"a\d+", base) else "global")
                dty, dcom = dd.get(base, ("?", ""))
                ctx = context(toks, i, j, match, end)
                rows.append(dict(opt=opt, proj=proj, bin=b, addr=hex(addr), fn=name, shared=shared,
                                 ty=ty, opnd=opnd, kind=kind, decl=dty, declc=dcom, ctx=ctx,
                                 line=text.splitlines()[toks[i].line-1].strip()[:160] if toks[i].line-1 < len(text.splitlines()) else ""))
    json.dump(rows, open(sys.argv[3] if len(sys.argv) > 3 else "/dev/stdout", "w"), indent=0)

if __name__ == "__main__":
    main()
