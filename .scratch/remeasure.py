import os,sys,json,re,collections
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
import castlib as C, decls as D, classify as CL
CUR=os.path.join(os.path.dirname(os.path.abspath(__file__)),'cur')

def cur_funcs(opt,proj,b):
    p=os.path.join(CUR,f'{opt}__{proj}__{b}.json')
    if not os.path.exists(p): return None
    try: d=json.load(open(p))
    except Exception: return None
    out={}
    for f in d.get('functions') or []:
        code=f.get('code') or ''
        if not code.strip(): continue
        out[int(f['address'])]=(f['name'],code)
    return out

def go():
    print(f"{'target':28} {'ida/1k':>7} {'old/1k':>7} {'new/1k':>7} | {'ida n':>6} {'old n':>6} {'new n':>6} | fn")
    T=collections.Counter()
    for fn in sorted(os.listdir(CUR)):
        if not fn.endswith('.json'): continue
        opt,proj,b=fn[:-5].split('__')
        ti=C.load(opt,proj,'ida',b); to=C.load(opt,proj,'kuna',b); cn=cur_funcs(opt,proj,b)
        if not (ti and to and cn): continue
        kti,kto=C.declared_types(ti),C.declared_types(to)
        fi={a:bd for n,a,bd in C.split_funcs(ti)}
        fo={a:bd for n,a,bd in C.split_funcs(to)}
        addrs=set(fi)&set(fo)&set(cn)
        if not addrs: continue
        ic=il=oc=ol=nc=nl=0
        for a in addrs:
            ic+=len(C.find_casts(fi[a],kti)); il+=fi[a].count('\n')
            oc+=len(C.find_casts(fo[a],kto)); ol+=fo[a].count('\n')
            code=cn[a][1]; nc+=len(C.find_casts(code,kto)); nl+=code.count('\n')
        for k,v in (('ic',ic),('il',il),('oc',oc),('ol',ol),('nc',nc),('nl',nl),('fn',len(addrs))): T[k]+=v
        print(f'{opt+"/"+b:28} {1000*ic/max(il,1):7.1f} {1000*oc/max(ol,1):7.1f} {1000*nc/max(nl,1):7.1f} | {ic:6} {oc:6} {nc:6} | {len(addrs)}')
    print(f'{"POOLED":28} {1000*T["ic"]/T["il"]:7.1f} {1000*T["oc"]/T["ol"]:7.1f} {1000*T["nc"]/T["nl"]:7.1f} | {T["ic"]:6} {T["oc"]:6} {T["nc"]:6} | {T["fn"]}')
    print(f'lines: ida {T["il"]} old {T["ol"]} new {T["nl"]}')
if __name__=='__main__': go()
