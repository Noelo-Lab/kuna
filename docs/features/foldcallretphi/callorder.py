import re,sys,collections
FUNC=re.compile(r"^// Function: (\S+) @ (\S+)$")
KW={"if","while","for","switch","return","do","else","sizeof","break","continue","goto","bool","int","long","char","unsigned","void","float","double","short"}
def split_funcs(p):
    out,cur,name={}, [], None
    for line in open(p,errors="replace").read().splitlines():
        m=FUNC.match(line)
        if m:
            if name: out[name]=cur
            name,cur=f"{m.group(1)}@{m.group(2)}",[]
        elif name is not None: cur.append(line)
    if name: out[name]=cur
    return out
CALL=re.compile(r"([A-Za-z_]\w*)\s*\(")
def calls_in_eval_order(lines):
    seq=[]
    for l in lines:
        s=re.sub(r'"(\\.|[^"\\])*"','""',l)          # kill string literals
        s=re.sub(r"'(\\.|[^'\\])*'","''",s)
        found=[]
        depth=0; i=0
        # compute nesting depth at each call site
        for m in CALL.finditer(s):
            name=m.group(1)
            if name in KW: continue
            d=s.count("(",0,m.start())-s.count(")",0,m.start())
            found.append((d,m.start(),name))
        # evaluation order: deeper first; among equal depth, left to right
        found.sort(key=lambda t:(-t[0],t[1]))
        seq.extend(n for _,_,n in found)
    return seq
bad=0; tot=0
for b in sys.argv[1:]:
    fo,fn=split_funcs(f"sweep/{b}.off.c"),split_funcs(f"sweep/{b}.on.c")
    for name in fo:
        if name not in fn: continue
        tot+=1
        a=calls_in_eval_order(fo[name]); c=calls_in_eval_order(fn[name])
        if a!=c:
            bad+=1
            print(f"CALL-ORDER CHANGED {b} {name}")
            sm=collections.Counter(a); sn=collections.Counter(c)
            if sm!=sn: print("   multiset differs:", (sm-sn), (sn-sm))
            else:
                for i,(x,y) in enumerate(zip(a,c)):
                    if x!=y: print("   first divergence at",i,x,"->",y, "\n   off:",a[max(0,i-3):i+4],"\n   on :",c[max(0,i-3):i+4]); break
print(f"functions compared={tot} call-order-changed={bad}")
