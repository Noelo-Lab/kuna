import json,collections,re,os
rows=json.load(open('/home/mahaloz/kwt/_final-f/sweep-f/rows.json'))
riv=json.load(open('/home/mahaloz/kwt/_final/rivals/rivals.json'))
cg=json.load(open('callgraph.json'))
va=json.load(open('variadic.json'))
cands=json.load(open('cands.json'))
red=json.load(open('reduced.json'))
CH=set(cands['char'])
def key(sk,d): return (sk,d["fn"],d.get("gt_name"),d.get("gt_is_arg"),d.get("gt_has_off"),d.get("gt_type"))
def keyed(items):
    out,seen={},collections.Counter()
    for sk,d in items:
        k=key(sk,d); i=seen[k]; seen[k]+=1; out[k+(i,)]=d
    return out
kun=keyed((sk,d) for sk,v in rows.items() for d in ((v.get("decisions") or {}).get("base") or []))
B=keyed((sk,d) for sk,v in riv.items() for d in (v.get("binja") or {}).get("decisions") or [])
pcs={k:d for k,d in kun.items() if d.get("cls")=="ptr_char"}
gap=[k for k in pcs if k in B and pcs[k]["ok"] is not True and B[k]["ok"] is True and pcs[k]["gt_type"]!="void"]
def storage(d): return "arg" if d["gt_is_arg"] else ("stack" if d["gt_has_off"] else "reg")
print("binja-only ptr_char rows:",len(gap))
buckets=collections.Counter(); per_import=collections.Counter(); per_import_fn=collections.defaultdict(set)
detail=collections.defaultdict(list)
for k in gap:
    sk,fn=k[0],k[1]; d=pcs[k]; s=storage(d)
    isva = va.get(sk,{}).get(fn,[False,0])[0]
    calls=set(cg.get(sk,{}).get(fn,[]))
    miss=sorted(calls&CH)
    if s=="reg": b="c3_register_only"
    elif isva: b="a_variadic_function"
    elif miss: b="b_missing_libc_import"
    else: b="c_other"
    buckets[(b,s)]+=1
    if b=="b_missing_libc_import":
        for n in miss: per_import[n]+=1; per_import_fn[n].add((sk,fn))
    detail[b].append((sk,fn,d.get("gt_name"),s,d.get("kuna_type"),sorted(miss)[:4],isva))
print()
for (b,s),n in sorted(buckets.items()): print(f"  {b:22s} {s:5s} {n}")
tot=collections.Counter(b for (b,s) in buckets for _ in range(buckets[(b,s)]))
print("\ntotals:",dict(tot))
print("\n--- imports ranked by binja-only ptr_char vars in a calling function (upper bound) ---")
for n,c in per_import.most_common(45):
    print(f"{c:4d} vars / {len(per_import_fn[n]):3d} fns  {n:24s} {red[n]['ret']:8s} {red[n]['params']}")
json.dump({'buckets':{f'{b}|{s}':n for (b,s),n in buckets.items()},
           'per_import':dict(per_import),
           'per_import_fns':{k:len(v) for k,v in per_import_fn.items()}},open('attrib.json','w'),indent=1)
json.dump({k:v for k,v in detail.items()},open('attrib-detail.json','w'),indent=0)
