import re,difflib,random,pathlib,sys,collections
base=pathlib.Path(sys.argv[1]); new=pathlib.Path(sys.argv[2])
INT=r'(?:unsigned )?(?:char|short|int|long)|bool|u?int[1248]'
def fmap(A):
    """line index -> (params, locals) of the enclosing function"""
    out={}; cur=None
    i=0
    while i<len(A):
        if A[i].startswith('// Function:'):
            hdr=A[i+1] if i+1<len(A) else ''
            params={}
            m=re.search(r'\((.*)\)',hdr)
            if m:
                for p in m.group(1).split(','):
                    pm=re.match(r'\s*(.*?)\s*\**\s*(\w+)\s*$',p)
                    if pm:
                        t=p.strip(); n=pm.group(2); params[n]=t[:len(t)-len(n)].strip()
            locs={}; j=i+3
            while j<len(A) and A[j].strip():
                dm=re.match(r'^  ([A-Za-z_][\w ]*?(?: ?\*+)?) ?(\w+)( \[\d+\])?;( //.*)?$',A[j])
                if dm and '=' not in dm.group(1): locs[dm.group(2)]=dm.group(1).strip()
                j+=1
            cur=(params,locs)
        out[i]=cur; i+=1
    return out
stats=collections.Counter(); flagged=[]
for f in sorted(new.rglob('*.c')):
    A=(base/f.relative_to(new)).read_text(errors='replace').split('\n'); B=f.read_text(errors='replace').split('\n')
    fm=fmap(A)
    sm=difflib.SequenceMatcher(None,A,B,autojunk=False)
    for tag,i1,i2,j1,j2 in sm.get_opcodes():
        if tag!='replace' or i2-i1!=j2-j1: continue
        for k in range(i1,i2):
            m=re.match(r'\s*(\w+) = \(('+INT+r')\)(\w+);( //.*)?$',A[k])
            if not m: continue
            lhs,t,rhs=m.group(1),m.group(2),m.group(3)
            params,locs=fm.get(k) or ({},{})
            dl=locs.get(lhs) or params.get(lhs); dr=locs.get(rhs) or params.get(rhs)
            stats['assign var=(T)var']+=1
            if dl!=t: flagged.append(('LHS-DECL',str(f.relative_to(new)),A[k].strip(),dl,dr)); continue
            if dr is None: stats['rhs not a local/param (global)']+=1; continue
            # sign/width policy on declared spellings
            W={'char':1,'short':2,'int':4,'long':8,'bool':1,'int1':1,'int2':2,'int4':4,'int8':8,'uint1':1,'uint2':2,'uint4':4,'uint8':8}
            def info(s):
                u = s.startswith('unsigned') or s.startswith('uint') or s=='bool'
                return (W.get(s.split()[-1]), u)
            (ws,us),(wt,ut)=info(dr),info(t)
            if ws is None: flagged.append(('RHS-NONINT',str(f.relative_to(new)),A[k].strip(),dl,dr)); continue
            okp = (ws<=wt and us==ut) or (us and not ut and ws<wt)
            stats['policy ok' if okp else 'POLICY VIOLATION']+=1
            if not okp: flagged.append(('POLICY',str(f.relative_to(new)),A[k].strip(),dl,dr))
print(dict(stats)); print('flagged',len(flagged))
for x in flagged[:25]: print(x)
