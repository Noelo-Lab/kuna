import json,subprocess,collections,os,sys
R='/home/mahaloz/github/decbench/results/full_run_address_2026-09-11'
rows=json.load(open('/home/mahaloz/kwt/_final-f/sweep-f/rows.json'))
per={}
missing=[]
for k in rows:
    proj,opt,b=k.split('::')
    p=f'{R}/{opt}/{proj}/stripped/{b}'
    if not os.path.exists(p): missing.append(p); continue
    out=subprocess.run(['readelf','-sW','--dyn-syms',p],capture_output=True,text=True).stdout
    und=set(); defd=set()
    for line in out.splitlines():
        f=line.split()
        if len(f)<8 or not f[0].endswith(':'): continue
        if f[3]!='FUNC': continue
        nm=f[7].split('@')[0]
        if f[6]=='UND': und.add(nm)
        else: defd.add(nm)
    per[k]=sorted(und-defd)
json.dump(per,open('/home/mahaloz/kwt/charptail/.scratch/gap/imports.json','w'))
print('slices',len(per),'missing_files',len(missing))
c=collections.Counter()
for k,v in per.items():
    for n in v: c[n]+=1
print('distinct imports',len(c))
