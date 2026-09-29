import re, sys, collections
sys.path.insert(0,'/home/mahaloz/kwt/castbench')
import castcount as CC, castbench as CB
from pathlib import Path
a_arm,b_arm=sys.argv[1],sys.argv[2]
def params(sig):
    m=re.match(r'^(.*?)\b(\w+)\((.*)\)', sig)
    if not m: return None
    ret=m.group(1).strip(); ps=[p.strip() for p in m.group(3).split(',')] if m.group(3).strip() not in ('','void') else []
    return ret, [re.sub(r'\s*\b[a-z]+\d+$','',p) for p in ps]
def kind(t):
    t=re.sub(r'struct_\d+','S',t)
    if '*' in t: return 'ptr' if not t.startswith('void') else 'void*'
    return 'int'
c=collections.Counter(); ex=collections.defaultdict(list)
for opt,proj,b in CB.corpus('full'):
    fa={a:t.splitlines()[1] for _,a,t in CC.split_functions((Path(a_arm)/opt/proj/f'{b}.c').read_text()) if len(t.splitlines())>1}
    fb={a:t.splitlines()[1] for _,a,t in CC.split_functions((Path(b_arm)/opt/proj/f'{b}.c').read_text()) if len(t.splitlines())>1}
    for a in set(fa)&set(fb):
        pa,pb=params(fa[a]),params(fb[a])
        if not pa or not pb: continue
        items=[('ret',pa[0],pb[0])]+[(f'p{i}',x,y) for i,(x,y) in enumerate(zip(pa[1],pb[1]))]
        for pos,x,y in items:
            x2=re.sub(r'struct_\d+','S',x); y2=re.sub(r'struct_\d+','S',y)
            if x2==y2: continue
            key=(kind(x),kind(y))
            c[key]+=1
            if len(ex[key])<6: ex[key].append(f'{opt}/{b}@{hex(a)} {pos}: {x} -> {y}')
for k,v in c.most_common(): print(v,k)
for k in ex:
    print('==',k); [print('   ',e) for e in ex[k]]
