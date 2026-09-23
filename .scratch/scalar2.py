import os,sys,re,collections,json
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
import castlib as C, decls as D, classify as CL
from main_analysis import corpus
IDENT=re.compile(r'^([A-Za-z_]\w*)\s*(?![\w(\[])')
GLOB={'qword_':'long long','dword_':'int','word_':'short','byte_':'char','off_':'void*','unk_':'?'}
def gtype(v):
    for p,t in GLOB.items():
        if v.startswith(p): return t
    return None
res={}
for side in ('ida','kuna'):
    rel=collections.Counter(); ex=collections.defaultdict(list); pairs=collections.Counter(); ctx=collections.Counter()
    for opt,proj,b,a,ni,body_i,nk,body_k,kti,ktk in corpus():
        body = body_i if side=='ida' else body_k
        kt   = kti if side=='ida' else ktk
        sig='\n'.join(body.split('\n')[:4]); dm=D.func_decls(body,sig)
        for pos,ty,nxt in C.find_casts(body,kt):
            if CL.bucket(body,pos,ty,nxt)!='L_scalar_var': continue
            m=IDENT.match(nxt.lstrip())
            if not m: rel['<expr>']+=1; continue
            v=m.group(1); dt=dm.get(v) or gtype(v)
            if dt is None: rel['<unknown-var>']+=1; continue
            if dt=='?': rel['<unk-global>']+=1; continue
            r=D.relation(ty,dt); rel[r]+=1; pairs[(D.norm(ty),D.norm(dt))]+=1
            if r in ('widen-samesign','identity','sign-only','narrow'):
                pre=body[max(0,pos-30):pos]
                if CL.in_arg_position(body,pos): c='call-arg'
                elif re.search(r'=\s*$',pre): c='assign-rhs'
                elif re.search(r'return\s*$',pre): c='return'
                elif re.search(r'[-+*/%<>=!&|^]\s*$',pre): c='binop'
                else: c='other'
                ctx[(r,c)]+=1
            if len(ex[r])<10:
                ln=body[:pos].count('\n'); ex[r].append(f'{b}/{nk if side=="kuna" else ni}: ({ty}){v} [{v}:{dt}] | '+body.split('\n')[ln].strip()[:95])
    res[side]=(rel,ex,pairs,ctx)
print(f"{'relation':18} {'ida':>7} {'kuna':>7} {'delta':>7}")
for k in sorted(set(res['ida'][0])|set(res['kuna'][0]), key=lambda k:-res['kuna'][0][k]):
    print(f"{k:18} {res['ida'][0][k]:7} {res['kuna'][0][k]:7} {res['kuna'][0][k]-res['ida'][0][k]:+7}")
print()
for side in ('kuna','ida'):
    print('===',side,'top (cast,decl) pairs')
    for (a,b_),v in res[side][2].most_common(16): print(f'   {v:5} ({a}) applied to {b_}')
    print('   contexts:', dict(res[side][3].most_common(12)))
print()
for k in ('identity','widen-samesign','sign-only','narrow','widen-signflip'):
    print('==',k)
    for e in res['kuna'][1][k][:6]: print('   kun |',e)
    for e in res['ida'][1][k][:3]: print('   ida |',e)
json.dump({s:{'rel':dict(res[s][0]),'pairs':{f'{a}|{b}':v for (a,b),v in res[s][2].items()},
              'ctx':{f'{a}|{b}':v for (a,b),v in res[s][3].items()}} for s in res},
          open(os.path.join(os.path.dirname(os.path.abspath(__file__)),'scalar2.json'),'w'), indent=1)
