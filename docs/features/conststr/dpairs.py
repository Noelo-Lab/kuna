import json, collections, re, sys
from pathlib import Path
rows=json.load(open(sys.argv[2])); arm=Path(sys.argv[1])
DR=re.compile(r"^\[globalref-direct\] (\S+) 0x([0-9a-f]+) decl=(.*):(\d+) direct=0x([0-9a-f]+):(\d+):(.*)$")
cache={}
def dl(opt,proj,b):
    k=(opt,proj,b)
    if k in cache: return cache[k]
    names={}
    for line in (arm/opt/proj/f"{b}.c").read_text(errors="replace").splitlines():
        m=re.match(r"^// Function: (\S+) @ (0x[0-9a-f]+)",line)
        if m: names[m.group(1)]=int(m.group(2),16)
    v=collections.defaultdict(set)
    for line in (arm/opt/proj/f"{b}.trace").read_text().splitlines():
        m=DR.match(line)
        if m and m.group(1) in names:
            off=int(m.group(2),16); ds=int(m.group(5),16)
            v[(names[m.group(1)],off)].add((m.group(3),int(m.group(4)),ds-off,int(m.group(6)),m.group(7)))
    cache[k]=v; return v
c=collections.Counter(); ex={}
for r in rows:
    if r['why']!='direct-access': continue
    ds=dl(r['opt'],r['proj'],r['bin']).get((int(r['fn'],16),int(r['v'],16)),set())
    kinds=set()
    for decl,dsz,rel,sz,ty in ds:
        if rel==0 and sz==dsz: k=f'same:{decl}/{ty}'
        elif rel==0 and sz<dsz: k='narrower'
        elif rel==0: k=f'wider:{decl}/{ty}'
        elif rel>0 and rel+sz<=dsz: k='inside'
        else: k='straddle'
        kinds.add(k)
    key='+'.join(sorted(kinds)) or 'none'
    c[key]+=1; ex[key]=r
for k,n in c.most_common(): r=ex[k]; print(n,k,'|',r['opt'],r['bin'],r['fn'],r['v'],r['line'][:80])
