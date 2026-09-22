import json, collections, sys
arms = sys.argv[1:] or ["off","on","void"]
R = {a: json.load(open(f"sweep-{a}/rows.json")) for a in arms}
def vals(rows):
    out={}
    for k,v in rows.items():
        for fn,x in ((v.get("base") or {}).get("values") or {}).items(): out[(k,fn)]=x
    return out
def nv(rows):
    out={}
    for k,v in rows.items():
        for fn,x in ((v.get("base") or {}).get("nvars") or {}).items(): out[(k,fn)]=x
    return out
V={a:vals(R[a]) for a in arms}
base=arms[0]
errs={a: sum(1 for v in R[a].values() if (v.get("base") or {}).get("error") or v.get("error")) for a in arms}
print("errors", errs)
for a in arms:
    v=V[a]; n=len(v); p=sum(1 for x in v.values() if x==1.0)
    print(f"{a}: n={n} perfect={p} sum={sum(v.values()):.2f} mean={sum(v.values())/max(1,n):.4f}")
for a in arms[1:]:
    imp=[k for k in V[base] if k in V[a] and V[a][k]>V[base][k]+1e-9]
    wor=[k for k in V[base] if k in V[a] and V[a][k]<V[base][k]-1e-9]
    onto=sum(1 for k in imp if V[a][k]==1.0); off_=sum(1 for k in wor if V[base][k]==1.0)
    print(f"{base}->{a}: improved {len(imp)} worse {len(wor)} onto-perfect {onto} off-perfect {off_} agg {sum(V[a].values())-sum(V[base].values()):+.2f}")
    N0=nv(R[base]); N1=nv(R[a])
    print("  nvars differ:", sum(1 for k in N0 if k in N1 and N0[k]!=N1[k]))
    # per project
    pp=collections.defaultdict(lambda: collections.Counter())
    for k in V[base]:
        if k not in V[a]: continue
        proj,opt,_=k[0].split("::")
        g=f"{proj}@{opt}"
        pp[g]["n"]+=1; pp[g]["p0"]+=V[base][k]==1.0; pp[g]["p1"]+=V[a][k]==1.0
        pp[g]["imp"]+=V[a][k]>V[base][k]+1e-9; pp[g]["wor"]+=V[a][k]<V[base][k]-1e-9
        pp[g]["s0"]+=V[base][k]; pp[g]["s1"]+=V[a][k]
    for g in sorted(pp): c=pp[g]; print(f"  {g:24s} n={c['n']:5d} perfect {c['p0']}->{c['p1']} imp {c['imp']} wor {c['wor']} mean {c['s0']/c['n']:.4f}->{c['s1']/c['n']:.4f}")
    if wor:
        print("  WORSE rows:")
        for k in sorted(wor): print("   ", k, f"{V[base][k]:.3f}->{V[a][k]:.3f}")
# class table
def decs(rows):
    out={}
    for k,v in rows.items():
        seen=collections.Counter()
        for d in (v.get("decisions") or {}).get("base") or []:
            kk=(k,d["fn"],d.get("gt_name"),d.get("gt_is_arg"),d.get("gt_has_off"),d.get("gt_type"))
            i=seen[kk]; seen[kk]+=1
            out[kk+(i,)]=d
    return out
D={a:decs(R[a]) for a in arms}
def st(d): return "arg" if d["gt_is_arg"] else ("stack" if d["gt_has_off"] else "reg")
print("class table (TP):")
cls=collections.defaultdict(collections.Counter)
for a in arms:
    for k,d in D[a].items():
        c=(d.get("cls"),st(d)); cls[c]["n_"+a]+=1; cls[c]["tp_"+a]+=d["ok"] is True
rows=sorted(cls.items(), key=lambda x:-x[1]["n_"+base])
for (c,s),cc in rows[:40]:
    print(f"  {str(c):12s} {s:5s} n={cc['n_'+base]:6d} "+" ".join(f"{a}={cc['tp_'+a]}" for a in arms))
tot={a: sum(1 for d in D[a].values() if d["ok"] is True) for a in arms}
print("all TP", tot)
