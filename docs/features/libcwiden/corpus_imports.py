import subprocess,collections,os,json,glob
R='/home/mahaloz/github/decbench/results/full_run_address_2026-09-11'
bins=[]
for opt in ('O0','O2','O2-noinline'):
    for p in glob.glob(f'{R}/{opt}/*/stripped/*'):
        if os.path.isfile(p): bins.append(p)
und=collections.Counter(); defined=collections.Counter(); nb=0
for p in bins:
    out=subprocess.run(['readelf','-sW','--dyn-syms',p],capture_output=True,text=True).stdout
    if not out: continue
    nb+=1; u=set(); d=set()
    for line in out.splitlines():
        f=line.split()
        if len(f)<8 or not f[0].endswith(':') or f[3]!='FUNC': continue
        nm=f[7].split('@')[0]
        (u if f[6]=='UND' else d).add(nm)
    for n in u-d: und[n]+=1
    for n in d: defined[n]+=1
json.dump({'und':dict(und),'defined':dict(defined),'nbins':nb,'ntotal':len(bins)},open('corpus-imports.json','w'))
print('binaries scanned',nb,'of',len(bins),'distinct UND names',len(und))
