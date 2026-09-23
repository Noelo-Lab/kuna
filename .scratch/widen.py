import os,sys,re,collections
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
import castlib as C, decls as D, classify as CL
S=[('O0','coreutils','fmt'),('O0','coreutils','ls'),('O0','coreutils','sort'),('O0','coreutils','du'),
   ('O2','coreutils','fmt'),('O2','coreutils','ls'),('O2','coreutils','sort'),('O2','coreutils','du'),
   ('O0','grep','grep'),('O0','gzip','gzip'),('O2','grep','grep'),('O2','gzip','gzip'),
   ('O0','diffutils','diff'),('O2','diffutils','diff'),('O0','findutils','find'),('O2','findutils','find')]
IDENT=re.compile(r'^([A-Za-z_]\w*)\s*(?![\w(\[])')
ctxcnt=collections.Counter(); rows=[]
for o,p,b in S:
    t=C.load(o,p,'kuna',b)
    if not t: continue
    kt=C.declared_types(t)
    for n,a,body in C.split_funcs(t):
        sig='\n'.join(body.split('\n')[:4]); dm=D.func_decls(body,sig)
        for pos,ty,nxt in C.find_casts(body,kt):
            if CL.bucket(body,pos,ty,nxt)!='L_scalar_var': continue
            m=IDENT.match(nxt.lstrip())
            if not m or m.group(1) not in dm: continue
            if D.relation(ty,dm[m.group(1)])!='widen-samesign': continue
            arg = CL.in_arg_position(body,pos)
            pre=body[max(0,pos-30):pos]
            if arg: ctx='call-arg'
            elif re.search(r'=\s*$',pre): ctx='assign-rhs'
            elif re.search(r'[-+*/%<>=!&|^]\s*$',pre): ctx='binop-operand'
            elif re.search(r'return\s*$',pre): ctx='return'
            elif re.search(r'\(\s*$',pre): ctx='paren'
            else: ctx='other'
            ctxcnt[ctx]+=1
            ln=body[:pos].count('\n')
            rows.append((ctx,ty,dm[m.group(1)],f'{b}/{n}',body.split('\n')[ln].strip()[:100]))
print(ctxcnt)
seen=collections.Counter()
for r in rows:
    if seen[r[0]]<8:
        seen[r[0]]+=1; print(f'{r[0]:14} ({r[1]}) on {r[2]:16} {r[3]:34} | {r[4]}')
