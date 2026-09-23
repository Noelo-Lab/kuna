import os,sys,re,collections
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
import castlib as C, decls as D, classify as CL

S=[('O0','coreutils','fmt'),('O0','coreutils','ls'),('O0','coreutils','sort'),('O0','coreutils','du'),
   ('O2','coreutils','fmt'),('O2','coreutils','ls'),('O2','coreutils','sort'),('O2','coreutils','du'),
   ('O0','grep','grep'),('O0','gzip','gzip'),('O2','grep','grep'),('O2','gzip','gzip'),
   ('O0','diffutils','diff'),('O2','diffutils','diff'),('O0','findutils','find'),('O2','findutils','find')]

IDENT=re.compile(r'^([A-Za-z_]\w*)\s*(?![\w(\[])')
for dec in ('kuna','ida'):
    rel=collections.Counter(); ex=collections.defaultdict(list); tot=0; noident=0
    pairs=collections.Counter()
    for o,p,b in S:
        t=C.load(o,p,dec,b)
        if not t: continue
        kt=C.declared_types(t); lines=t.split('\n')
        for n,a,body in C.split_funcs(t):
            sig='\n'.join(body.split('\n')[:4])
            dm=D.func_decls(body,sig)
            off=t.find(body)
            for pos,ty,nxt in C.find_casts(body,kt):
                if CL.bucket(body,pos,ty,nxt)!='L_scalar_var': continue
                tot+=1
                m=IDENT.match(nxt.lstrip())
                if not m: noident+=1; rel['<expr>']+=1; continue
                v=m.group(1)
                if v not in dm: rel['<unknown-var>']+=1; continue
                r=D.relation(ty,dm[v]); rel[r]+=1
                pairs[(D.norm(ty),D.norm(dm[v]))]+=1
                if len(ex[r])<6:
                    ln=body[:pos].count('\n'); ex[r].append(f'{b} {n}: ({ty}){v} [{v}:{dm[v]}] | '+body.split(chr(10))[ln].strip()[:90])
    print('=====',dec,'L_scalar_var total',tot)
    for k,v in rel.most_common(): print(f'   {v:6} {k}')
    print('   top (cast,decl) pairs:')
    for (a,b_),v in pairs.most_common(14): print(f'      {v:5} ({a}) on {b_}')
    for k in ('identity','widen-samesign','widen-signflip','sign-only','narrow','same-width'):
        for e in ex[k][:3]: print(f'   {k:16}| {e}')
    print()
