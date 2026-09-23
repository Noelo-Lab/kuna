import os,sys,collections,random
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
import castlib as C
random.seed(7)
SAMPLE=[('O0','coreutils','fmt'),('O0','coreutils','ls'),('O0','coreutils','sort'),('O0','coreutils','du'),
        ('O2','coreutils','fmt'),('O2','coreutils','ls'),('O2','coreutils','sort'),('O2','coreutils','du'),
        ('O0','grep','grep'),('O0','gzip','gzip'),('O0','tar','tar'),('O0','bash','bash')]
for dec in ('ida','kuna'):
    cnt=collections.Counter(); ex=collections.defaultdict(list); tot=0
    for opt,proj,b in SAMPLE:
        t=C.load(opt,proj,dec,b)
        if not t: continue
        kt=C.declared_types(t); lines=t.split('\n')
        for pos,ty,nxt in C.find_casts(t,kt):
            cnt[ty]+=1; tot+=1
            if len(ex[ty])<3:
                ln=t[:pos].count('\n'); ex[ty].append(lines[ln].strip()[:100])
    print('=====',dec,'total',tot)
    for k,v in cnt.most_common(30):
        print(f'  {v:6} {k:20} | {ex[k][0] if ex[k] else ""}')
