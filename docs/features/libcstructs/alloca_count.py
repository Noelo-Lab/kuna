import sys, os, re, collections, json
sys.path.insert(0, os.path.dirname(__file__))
from framecheck_lib import analyze, args_of
keys=[k.strip() for k in open(sys.argv[3]) if k.strip()]
newnames=set(json.load(open(sys.argv[4])) if len(sys.argv) > 4 else {})
PROBE=re.compile(r'-0x1000\]|-0x1000\)|0xfffffffffffff000')
res={}
for arm,d in (('base',sys.argv[1]),('new',sys.argv[2])):
    tot=collections.Counter(); fnset=set(); per=collections.Counter()
    for k in keys:
        txt=open(f"{d}/{k.replace('::','__')}.c").read()
        r,p=analyze(txt)
        if arm=='base': pass
        rx=re.compile(r'(?<![\w.>])(' + '|'.join(sorted(map(re.escape,p),key=len,reverse=True)) + r')\(') if p else None
        for addr,f in r.items():
            b=f['body'].split('\n',2)[-1]
            if not PROBE.search(b) or not rx: continue
            tot['alloca_funcs']+=1
            for m in rx.finditer(b):
                n=m.group(1); a,_=args_of(b,m.end())
                if a is not None and a==p[n]+1:
                    tot['sites']+=1; fnset.add((k,addr)); per[n]+=1
                    if n in newnames: tot['sites_new_rows']+=1
    res[arm]=dict(tot=tot,funcs=len(fnset),top=per.most_common(8),new={n:c for n,c in per.items() if n in newnames})
print(json.dumps(res,indent=1))
