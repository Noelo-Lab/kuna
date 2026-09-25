import sys, collections, re, json
exec(open(__import__('os').path.join(__import__('os').path.dirname(__file__), 'addrcensus.py')).read().replace('if __name__ == "__main__":\n    main()',''))
from pathlib import Path
arm=sys.argv[1]; which=sys.argv[2] if len(sys.argv)>2 else "full"
rows=[]
for opt, proj, b in CB.corpus(which):
    path = Path(arm)/opt/proj/f"{b}.c"
    ida = CB.funcs(CB.RES/opt/proj/"decompiled"/f"ida_{b}.c")
    if not path.exists() or not ida: continue
    src = path.read_text(errors="replace"); vocab = CC.harvest_types(CC.tokenize(src), src)
    for name, addr, text in CC.split_functions(src):
        if addr not in ida: continue
        toks = CC.tokenize(text); dd = decls(text); match = CC.matching(toks); lines=text.splitlines()
        # dynamic-sp roots: char* locals assigned from a stack array or declared // rsp, or written as X[..-8] = 0x<code addr>
        dyn=set()
        for v,(base,st,arr,com) in dd.items():
            if base=="char" and st==1 and com!="param":
                if com=="rsp" or re.search(r"\b%s\s*=\s*(v\d+);"%v, text) and any(dd.get(m,("",0,False,""))[2] for m in re.findall(r"\b%s\s*=\s*(v\d+);"%v, text)):
                    dyn.add(v)
        # propagate: char* locals assigned from a dyn root
        for _ in range(4):
            for v,(base,st,arr,com) in dd.items():
                if v in dyn or not(base=="char" and st==1): continue
                for m in re.findall(r"\b%s\s*=\s*&?(v\d+)"%v, text):
                    if m in dyn: dyn.add(v)
        for ty, shape, line, ctext in CC.find_casts(toks, vocab):
            if shape != "<addr>" or "*" not in ty: continue
            for i,t in enumerate(toks):
                if t.line==line and t.text=="(" and match.get(i) is not None:
                    jj=match[i]
                    if "".join(x.text for x in toks[i:jj+1])==ctext and jj+1<len(toks) and toks[jj+1].text=="&":
                        form,dtxt=classify(toks,jj+1,dd); root=toks[jj+2].text; break
            else: continue
            dec=dd.get(root)
            if root in dyn and form.startswith("index"): k="dynsp"
            elif form=="ptrfield": k="field:"+("struct_N" if dec and dec[0].startswith("struct_") else (dec[0] if dec else "?"))
            elif form=="field": k="dotfield"
            elif form=="global": k="global"
            elif form in("index","index-neg") and dec and dec[2]: k="array-local"
            elif form in("index","index-neg") and dec and dec[1]>=1: k="ptr-index-"+("param" if dec[3]=="param" else "local")
            elif form=="plain": k="plain-"+("param" if dec and dec[3]=="param" else "local")
            else: k=form
            rows.append(dict(opt=opt,bin=b,fn=name,addr=addr,ty=ty,k=k,decl=dtxt,line=lines[line-1].strip()[:160]))
json.dump(rows,open(sys.argv[3] if len(sys.argv)>3 else "addr_rows.json","w"))
c=collections.Counter(r['k'] for r in rows)
print("total",len(rows))
for k,n in c.most_common(): 
    u=sum(1 for r in rows if r['k']==k and r['ty']=='u64*')
    print(f"  {k:28} {n:6}  (u64* {u})")
