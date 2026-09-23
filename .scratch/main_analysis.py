import os,sys,json,re,collections
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
import castlib as C, decls as D, classify as CL
from remeasure import cur_funcs
CUR=os.path.join(os.path.dirname(os.path.abspath(__file__)),'cur')

def corpus():
    """yield (opt,proj,bin,addr,ida_body,kuna_body,ida_types,kuna_types)"""
    for fn in sorted(os.listdir(CUR)):
        if not fn.endswith('.json'): continue
        opt,proj,b=fn[:-5].split('__')
        ti=C.load(opt,proj,'ida',b); cn=cur_funcs(opt,proj,b)
        if not (ti and cn): continue
        kti=C.declared_types(ti)
        ktk=C.declared_types('\n'.join(v[1] for v in cn.values()))
        fi={a:(n,bd) for n,a,bd in C.split_funcs(ti)}
        for a in sorted(set(fi)&set(cn)):
            yield (opt,proj,b,a,fi[a][0],fi[a][1],cn[a][0],cn[a][1],kti,ktk)

if __name__=='__main__':
    rows=[]; bi=collections.Counter(); bk=collections.Counter()
    exi=collections.defaultdict(list); exk=collections.defaultdict(list)
    argi=collections.Counter(); argk=collections.Counter()
    for opt,proj,b,a,ni,body_i,nk,body_k,kti,ktk in corpus():
        ci=C.find_casts(body_i,kti); ck=C.find_casts(body_k,ktk)
        for pos,ty,nxt in ci:
            kk=CL.bucket(body_i,pos,ty,nxt); bi[kk]+=1
            if CL.in_arg_position(body_i,pos): argi[kk]+=1
            if len(exi[kk])<8:
                ln=body_i[:pos].count('\n'); exi[kk].append(f'{b}/{ni}: '+body_i.split('\n')[ln].strip()[:110])
        for pos,ty,nxt in ck:
            kk=CL.bucket(body_k,pos,ty,nxt); bk[kk]+=1
            if CL.in_arg_position(body_k,pos): argk[kk]+=1
            if len(exk[kk])<8:
                ln=body_k[:pos].count('\n'); exk[kk].append(f'{b}/{nk}: '+body_k.split('\n')[ln].strip()[:110])
        rows.append(dict(opt=opt,proj=proj,bin=b,addr=a,ni=ni,nk=nk,ic=len(ci),kc=len(ck),
                         il=body_i.count('\n'),kl=body_k.count('\n')))
    json.dump(rows,open(os.path.join(os.path.dirname(os.path.abspath(__file__)),'rows_cur.json'),'w'))
    print('matched functions', len(rows))
    print(f"{'bucket':20} {'ida':>7} {'kuna':>7} {'delta':>7}")
    for k in sorted(set(bi)|set(bk)):
        print(f'{k:20} {bi[k]:7} {bk[k]:7} {bk[k]-bi[k]:+7}   argpos ida={argi[k]} kuna={argk[k]}')
    print('TOTAL', sum(bi.values()), sum(bk.values()))
    print()
    for k in sorted(set(bi)|set(bk)):
        print('==',k)
        for e in exi[k][:3]: print('   ida |',e)
        for e in exk[k][:3]: print('   kun |',e)
