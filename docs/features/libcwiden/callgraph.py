import json,subprocess,re,os,sys,collections
R='/home/mahaloz/github/decbench/results/full_run_address_2026-09-11'
rows=json.load(open('/home/mahaloz/kwt/_final-f/sweep-f/rows.json'))
FN=re.compile(r'^[0-9a-f]+ <([^>]+)>:')
CALL=re.compile(r'\bcall\w*\s+[0-9a-f]+ <([^>+]+?)(@plt)?>')
out={}
for k in sorted(rows):
    proj,opt,b=k.split('::')
    p=f'{R}/{opt}/{proj}/compiled/{b}'
    if not os.path.exists(p): continue
    d=subprocess.run(['objdump','-d','--no-show-raw-insn',p],capture_output=True,text=True).stdout
    cur=None; m={}
    for line in d.splitlines():
        g=FN.match(line)
        if g: cur=g.group(1); m.setdefault(cur,set()); continue
        if cur is None: continue
        c=CALL.search(line)
        if c: m[cur].add(c.group(1))
    out[k]={f:sorted(v) for f,v in m.items()}
json.dump(out,open('callgraph.json','w'))
print('slices',len(out))
