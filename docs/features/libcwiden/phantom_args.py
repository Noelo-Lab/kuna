import re,glob,os,json,collections
def split(s):
    out=[];d=0;cur=''
    for ch in s:
        if ch in '([': d+=1
        if ch in ')]': d-=1
        if ch==',' and d==0: out.append(cur.strip()); cur=''; continue
        cur+=ch
    if cur.strip(): out.append(cur.strip())
    return out
HDR=re.compile(r'^[A-Za-z_][\w \*]*?\s\**([A-Za-z_]\w*)\((.*)\)\s*;?\s*$')
PH=re.compile(r'^v\d+\[-\d+\]$')
def scan(p):
    txt=open(p).read().splitlines(); ar={}
    for l in txt:
        if l and not l.startswith(' ') and not l.startswith('//') and not l.endswith('{'):
            h=HDR.match(l)
            if h and '...' not in h.group(2):
                ps=split(h.group(2)); ar.setdefault(h.group(1), 0 if ps in ([],['void']) else len(ps))
    n=0
    for l in txt:
        for m in re.finditer(r'\b([A-Za-z_]\w*)\(([^;{}]*)\)', l):
            nm=m.group(1)
            if nm in ar:
                a=split(m.group(2))
                if len(a)==ar[nm]+1 and PH.match(a[-1]): n+=1
    return n
O='/home/mahaloz/kwt/charptail/.scratch/rev/out2'
names=sorted({os.path.basename(p).split('.')[0] for p in glob.glob(O+'/*.B2.on.c')})
res={}; tot=collections.Counter()
for n in names:
    r={a:scan(f'{O}/{n}.{a}.c') for a in ('B2.on','B2.off','C2.on','C2.off')}
    res[n]=r
    for k,v in r.items(): tot[k]+=v
print(json.dumps({'per_binary':res,'total':dict(tot)},indent=1))
