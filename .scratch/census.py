import os, sys, json, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import castlib as C

R = C.R
OPTS = ('O0','O2','O2-noinline')

def pairs_for(opt, proj, binname):
    ti = C.load(opt, proj, 'ida', binname)
    tk = C.load(opt, proj, 'kuna', binname)
    if ti is None or tk is None: return []
    kti, ktk = C.declared_types(ti), C.declared_types(tk)
    fi = {a: (n, b) for n, a, b in C.split_funcs(ti)}
    fk = {a: (n, b) for n, a, b in C.split_funcs(tk)}
    out = []
    for a in set(fi) & set(fk):
        ni, bi = fi[a]; nk, bk = fk[a]
        ci = C.find_casts(bi, kti); ck = C.find_casts(bk, ktk)
        out.append(dict(opt=opt, proj=proj, bin=binname, addr=a, ida_name=ni, kuna_name=nk,
                        ida_casts=len(ci), kuna_casts=len(ck),
                        ida_lines=bi.count('\n'), kuna_lines=bk.count('\n')))
    return out

def main():
    rows = []
    for opt in OPTS:
        base = os.path.join(R, opt)
        for proj in sorted(os.listdir(base)):
            d = os.path.join(base, proj, 'decompiled')
            if not os.path.isdir(d): continue
            idas = {f[4:-2] for f in os.listdir(d) if f.startswith('ida_') and f.endswith('.c')}
            kunas = {f[5:-2] for f in os.listdir(d) if f.startswith('kuna_') and f.endswith('.c')}
            for b in sorted(idas & kunas):
                rows.extend(pairs_for(opt, proj, b))
    json.dump(rows, open(os.path.join(os.path.dirname(os.path.abspath(__file__)), 'pairs.json'), 'w'))
    print('matched functions:', len(rows))
    for opt in OPTS:
        r = [x for x in rows if x['opt']==opt]
        ic = sum(x['ida_casts'] for x in r); kc = sum(x['kuna_casts'] for x in r)
        il = sum(x['ida_lines'] for x in r); kl = sum(x['kuna_lines'] for x in r)
        print(f'{opt:12} n={len(r):6}  ida {ic:6} casts /{il:7} lines = {1000*ic/max(il,1):6.1f}/1k   '
              f'kuna {kc:6} /{kl:7} = {1000*kc/max(kl,1):6.1f}/1k   ratio {kc/max(ic,1):.2f}x')
    print()
    for opt in OPTS:
        r=[x for x in rows if x['opt']==opt]
        worse=sum(1 for x in r if x['kuna_casts']>x['ida_casts'])
        same=sum(1 for x in r if x['kuna_casts']==x['ida_casts'])
        better=sum(1 for x in r if x['kuna_casts']<x['ida_casts'])
        print(f'{opt:12} kuna worse {worse} ({100*worse/max(len(r),1):.0f}%) same {same} better {better}')

main()
