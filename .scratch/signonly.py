"""Count casts whose target type EQUALS the operand variable's declared type.
Such a cast is a no-op in C and can be deleted with no semantic change."""
import sys,re,collections
sys.path.insert(0,".")
import castcount as CC
S={"i8":"8","u8":"8","char":"8","uchar":"8","i16":"16","u16":"16","i32":"32",
   "u32":"32","i64":"64","u64":"64","i128":"128","u128":"128"}
def SIGN(t):
    st=0
    while t.endswith("*"): t=t[:-1]; st+=1
    return (S.get(t,t), st)
from pathlib import Path
def run(root):
    tot=red=0; byty=collections.Counter(); ex={}
    for f in sorted(Path(root).rglob("*.c")):
        src=f.read_text(errors="replace")
        for name,addr,txt in CC.split_functions(src):
            toks=CC.tokenize(txt); vocab=CC.harvest_types(toks,src)
            # declared type of each local/param, normalized
            decl={}
            n=len(toks)
            for i in range(n):
                t=toks[i]
                if t.kind!="id": continue
                if i>0 and toks[i-1].text not in (";","{","}","(",","): continue
                j,specs,tag,saw=i,[],None,False
                while j<n and toks[j].kind=="id":
                    x=toks[j].text
                    if x in CC.QUAL: j+=1; continue
                    if x in CC.TAGKW and j+1<n and toks[j+1].kind=="id": tag=(x,toks[j+1].text); j+=2; saw=True; continue
                    if x in CC.BASE: specs.append(x); j+=1; saw=True; continue
                    if not saw and x in vocab: specs.append(x); j+=1; saw=True; continue
                    break
                if not saw: continue
                st=0
                while j<n and (toks[j].text=="*" or toks[j].text in CC.QUAL):
                    if toks[j].text=="*": st+=1
                    j+=1
                if j<n and toks[j].kind=="id" and toks[j].text not in CC.BASE:
                    if j+1<n and toks[j+1].text in (";","=",",",")"):
                        ty=CC.normalize(specs,tag,st,False)
                        if ty: decl.setdefault(toks[j].text,ty)
            match=CC.matching(toks); cc=set()
            for i in range(n):
                if toks[i].text!="(": continue
                j=match.get(i)
                if j is None or j==i+1: continue
                if not CC.cast_allowed_before(toks,i,cc): continue
                k=j+1
                if k>=n: continue
                nt=toks[k]
                if not (nt.kind in ("id","num","str","chr") or nt.text in CC.UNARY_START_PUNCT): continue
                ty=CC.parse_type_name(toks,i+1,j,vocab)
                if ty is None: continue
                cc.add(j); tot+=1
                # operand must be a BARE variable (next token after it ends the operand)
                if nt.kind=="id" and (k+1>=n or toks[k+1].text not in ("(",".","->","[")):
                    d=decl.get(nt.text)
                    if d is not None and SIGN(d)==SIGN(ty) and d!=ty:
                        red+=1; byty[(d,ty)]+=1
                        if (d,ty) not in ex:
                            ex[(d,ty)]=(f"{f.parts[-3]}/{f.stem}@{hex(addr)}",
                                    "".join(t.text for t in toks[i:k+1]), nt.text, d)
    return tot,red,byty,ex
for root in sys.argv[1:]:
    tot,red,byty,ex=run(root)
    print(f"### {root}: {red} signedness-only casts of {tot} ({100.0*red/tot:.1f}%)")
    for (d,ty),n in byty.most_common(6):
        e=ex[(d,ty)]
        print(f"    {d:>6} -> {ty:<7}{n:>6}   e.g. {e[1]}  with `{e[3]} {e[2]};`   [{e[0]}]")
