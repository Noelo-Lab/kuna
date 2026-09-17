import difflib, re, sys, pathlib, json, collections
off=pathlib.Path('.scratch/sweep/off'); on=pathlib.Path('.scratch/sweep/on')
CATS=collections.Counter(); DETAIL=collections.defaultdict(list)
def cat(a,b):
    # a = off line, b = on line (one may be None)
    if a is None or b is None: return 'added/removed'
    x,y=a,b
    # the GOT slot gained a name
    if re.sub(r'dat_[0-9a-f]+', 'S', x) == re.sub(r'(stdin|stdout|stderr)_ptr','S',y): return 'got-slot-named'
    # a cast to FILE * removed alongside the name
    x2=x.replace('(FILE *)','')
    if re.sub(r'dat_[0-9a-f]+','S',x2)==re.sub(r'(stdin|stdout|stderr)_ptr','S',y): return 'got-slot-named+cast-removed'
    # declaration retyped to FILE * / FILE **
    d=re.match(r'\s*(.*?)\s*\*?\s*(v\d+)\s*(\[\d+\])?;',x)
    if re.match(r'\s*(unsigned long|long|void|undefined8|int8|uint8)\s*\*?\s*v\d+',x) and re.match(r'\s*FILE\s*\*+\s*v\d+',y):
        return 'decl-typed-FILE'
    if x.replace('(FILE *)','')==y.replace('(FILE *)',''): return 'cast-moved'
    # offset form -> field form on a stream
    if re.sub(r'\*\(\w[\w ]*\*+\)\(\(?\w*\)?\s*(stdout|stderr|stdin)\s*\+\s*0x[0-9a-f]+\)','F',x)!=x: return 'field-form'
    if x.strip().startswith(('long ','unsigned long ','void ')) and y.strip().startswith('FILE '): return 'proto-ret-FILE'
    return 'OTHER'
for f in sorted(off.glob('*.c')):
    g=on/f.name
    A=f.read_text(errors='replace').splitlines(); B=g.read_text(errors='replace').splitlines()
    sm=difflib.SequenceMatcher(None,A,B,autojunk=False)
    for tag,i1,i2,j1,j2 in sm.get_opcodes():
        if tag=='equal': continue
        la=A[i1:i2]; lb=B[j1:j2]
        n=max(len(la),len(lb))
        for k in range(n):
            a=la[k] if k<len(la) else None
            b=lb[k] if k<len(lb) else None
            c=cat(a,b); CATS[c]+=1
            if c=='OTHER': DETAIL[f.name].append((a,b))
for c,n in CATS.most_common(): print(f"{n:6d}  {c}")
print()
for fn,rows in DETAIL.items():
    print("== OTHER in",fn)
    for a,b in rows: print("  -",a); print("  +",b)
