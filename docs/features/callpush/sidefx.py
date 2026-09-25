import re,sys,collections,json
exec(open(__import__('os').path.join(__import__('os').path.dirname(__file__), 'hunks2.py')).read().split("def main():")[0])
from pathlib import Path
KW={"if","while","for","switch","return","sizeof","do"}
def fx(lines, ra):
    c=collections.Counter()
    for L in lines:
        L=re.sub(r"//.*$","",L)
        m=CONST_STORE.match(L)
        if m and int(m.group(2),0) in ra and int(m.group(2),0)>0x1000: continue
        for mm in re.finditer(r"\b([A-Za-z_]\w*)\s*\(",L):
            n=mm.group(1)
            if n not in KW and not re.match(r"^(v|a)\d+$",n) and not re.match(r"^(SUB|ZEXT|SEXT|CONCAT|CARRY|SCARRY|SBORROW)\d*$",n):
                c["call:"+n]+=1
        if re.match(r"^\s*(\*|\(\()",L) and re.search(r"[^=!<>]=[^=]",L): c["memstore"]+=1
        if re.match(r"^\s*v\d+\[[^\]]*\]\s*=[^=]",L) or re.match(r"^\s*[av]\d+->",L): c["memstore"]+=1
        if re.match(r"^\s*return\b",L): c["return"]+=1
    return c
off,on=Path('cb-off'),Path('cb-on')
bad=[]
for fo in sorted(off.rglob('*.c')):
    rel=fo.relative_to(off); opt,proj,b=rel.parts[0],rel.parts[1],rel.stem
    a,c=split(fo),split(on/rel)
    ch=[k for k in a if a[k]!=c.get(k)]
    if not ch: continue
    ra=retaddrs(RES/opt/proj/'stripped'/b)
    for k in ch:
        A,_=strip_bytepush_long(strip_pushes(a[k],ra)[0],ra); C,_=strip_bytepush_long(strip_pushes(c[k],ra)[0],ra)
        fa,fc=fx(A,ra),fx(C,ra)
        if fa!=fc:
            d={x:(fa[x],fc[x]) for x in set(fa)|set(fc) if fa[x]!=fc[x]}
            bad.append((f"{opt}/{b}/{k[0]}",d))
print(len(bad),"functions whose calls/stores/returns differ")
for f,d in bad: print(f,d)
