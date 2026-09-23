import json,collections
known=set(open('known-names.txt').read().split())
imports=json.load(open('imports.json'))
red=json.load(open('reduced.json'))
rej=json.load(open('rejected.json'))
slices=collections.Counter()
for k,v in imports.items():
    for n in set(v): slices[n]+=1
allnames=set(slices)
missing=sorted(allnames-known)
have=[n for n in missing if n in red]
norej=[n for n in missing if n not in red]
def charish(n):
    s=red[n]
    return s['ret'] in ('CharPtr','CharPtrPtr') or any(p in ('CharPtr','CharPtrPtr') for p in s['params'])
ch=[n for n in have if charish(n)]
print(f"corpus imports {len(allnames)}; already known {len(allnames&known)}; missing {len(missing)}")
print(f"  of the missing: reducible {len(have)}, not reducible {len(norej)}")
print(f"  reducible AND char-pointer bearing: {len(ch)}")
i64=[n for n in ch if red[n]['ret'].startswith('I64') or any(p.startswith('I64') for p in red[n]['params'])]
print(f"  of those, needing a fixed-64-bit Ty: {len(i64)} -> {sorted(i64, key=lambda x:-slices[x])[:12]}")
json.dump({'slices':dict(slices),'have':have,'char':ch,'norej':norej},open('cands.json','w'))
print('\n--- top 40 missing char-bearing by slice count ---')
for n in sorted(ch,key=lambda x:(-slices[x],x))[:40]:
    print(f"{slices[n]:4d}  {n:28s} {red[n]['ret']:9s} {red[n]['params']} va={red[n]['vararg']}")
print('\n--- top 25 missing NON-reducible by slice count ---')
for n in sorted(norej,key=lambda x:(-slices[x],x))[:25]:
    print(f"{slices[n]:4d}  {n:28s} {rej.get(n,'absent-from-headers')}")
