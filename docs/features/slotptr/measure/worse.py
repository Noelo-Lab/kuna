import json, collections, sys
a0,a1=sys.argv[1],sys.argv[2]
R={a: json.load(open(f"sweep-{a}/rows.json")) for a in (a0,a1)}
def decs(rows):
    out={}
    for k,v in rows.items():
        seen=collections.Counter()
        for d in (v.get("decisions") or {}).get("base") or []:
            kk=(k,d["fn"],d.get("gt_name"),d.get("gt_is_arg"),d.get("gt_has_off"),d.get("gt_type"))
            i=seen[kk]; seen[kk]+=1
            out[kk+(i,)]=d
    return out
D0,D1=decs(R[a0]),decs(R[a1])
lost=[(k,D0[k],D1[k]) for k in D0 if k in D1 and D0[k]["ok"] is True and D1[k]["ok"] is not True]
gain=[(k,D0[k],D1[k]) for k in D0 if k in D1 and D0[k]["ok"] is not True and D1[k]["ok"] is True]
print("gained", len(gain), "lost", len(lost))
for k,d0,d1 in lost:
    print(" LOST", k[0], k[1], "gt", d0["gt_name"], d0["gt_type"], d0.get("cls"), "|", d0.get("kuna_name"), d0["kuna_type"], "->", d1.get("kuna_name"), d1["kuna_type"], d1["pass"])
c=collections.Counter((d0.get("kuna_type"), d1.get("kuna_type"), d1.get("cls")) for k,d0,d1 in gain)
print("gain spelling moves (top):")
for x,n in c.most_common(25): print("  ",n,x)
