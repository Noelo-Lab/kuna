"""Sub-classify the OTHER functions of hunks.py by the signals each changed line carries."""
import sys, re, difflib, collections
from pathlib import Path
src=open(Path(__file__).resolve().parent / 'hunks.py').read().replace('\nmain()\n','\n'); ns={}; exec(compile(src,'hunks','exec'),ns)
sys.path.insert(0,'/home/mahaloz/kwt/castbench')
import castcount as CC
from pathlib import Path
RES={'coreutils':['fmt','ls','sort','du','cp','tail','wc'],'grep':['grep'],'gzip':['gzip'],'diffutils':['cmp','diff','diff3','sdiff'],'tar':['tar'],'findutils':['find']}
proj={b:p for p,bs in RES.items() for b in bs}
off,on=sys.argv[1],sys.argv[2]
tags=[l.split()[1] for l in open(sys.argv[3]) if l.startswith('=====')]
agg=collections.Counter(); resid=[]
def sig(a,b):
    s=set()
    if '[' in b and '[' not in a or b.count('[')>a.count('['): s.add('subscript')
    if '[' in a and b.count('[')<a.count('['): s.add('subscript')
    if 'NULL' in b and 'NULL' not in a: s.add('null')
    if '&dat_' in b and '&dat_' not in a: s.add('dat')
    if 'field_0x0' in b and 'field_0x0' not in a: s.add('firstmember')
    if re.sub(r'\bV\d+\b','V',a)==re.sub(r'\bV\d+\b','V',b): s.add('rename')
    ta=sorted(re.findall(r'[A-Za-z_]\w*',re.sub(r'\bV\d+\b','V',a))); tb=sorted(re.findall(r'[A-Za-z_]\w*',re.sub(r'\bV\d+\b','V',b)))
    if ta==tb: s.add('same-names')
    return s
for tag in tags:
    opt,rest=tag.split('/'); bn,addr=rest.split('@'); addr=int(addr,16)
    get=lambda arm: next(t for n,a,t in CC.split_functions(Path(arm,opt,proj[bn],f'{bn}.c').read_text()) if a==addr)
    x=ns['norm'](get(off),2); y=ns['norm'](get(on),2)
    sm=difflib.SequenceMatcher(None,x,y)
    fs=set(); unexplained=[]
    for op,i1,i2,j1,j2 in sm.get_opcodes():
        if op=='equal': continue
        A=x[i1:i2]; Bb=y[j1:j2]
        if op=='replace' and len(A)==len(Bb):
            for a,b in zip(A,Bb):
                s=sig(a,b)
                if i1==1 or a.startswith(('void','int','long','unsigned','char','struct')) and '(' in a and not a.startswith(' '): s.add('signature')
                if not s: unexplained.append((a,b))
                fs|=s
        else:
            fs.add('merge')
    agg[tuple(sorted(fs))]+=1
    if unexplained: resid.append((tag,unexplained[:3]))
for k,v in agg.most_common(): print(v,k)
print('functions with a line no signal explains:',len(resid))
for t,u in resid:
    print('==',t)
    for a,b in u: print('   -',a[:150]); print('   +',b[:150])
