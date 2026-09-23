import json,os,sys
from elftools.elf.elffile import ELFFile
R='/home/mahaloz/github/decbench/results/full_run_address_2026-09-11'
rows=json.load(open('/home/mahaloz/kwt/_final-f/sweep-f/rows.json'))
out={}
for k in sorted(rows):
    proj,opt,b=k.split('::')
    p=f'{R}/{opt}/{proj}/compiled/{b}'
    if not os.path.exists(p): continue
    va={}
    with open(p,'rb') as fh:
        e=ELFFile(fh)
        if not e.has_dwarf_info(): continue
        dw=e.get_dwarf_info()
        for cu in dw.iter_CUs():
            for die in cu.iter_DIEs():
                if die.tag!='DW_TAG_subprogram': continue
                nm=die.attributes.get('DW_AT_name')
                if nm is None: continue
                nm=nm.value.decode('utf8','replace')
                if 'DW_AT_low_pc' not in die.attributes: continue
                v=any(c.tag=='DW_TAG_unspecified_parameters' for c in die.iter_children())
                nparams=sum(1 for c in die.iter_children() if c.tag=='DW_TAG_formal_parameter')
                if v or nm not in va: va[nm]=[v,nparams]
                elif v: va[nm]=[v,nparams]
    out[k]=va
json.dump(out,open('variadic.json','w'))
n=sum(1 for k,v in out.items() for f,(vv,_) in v.items() if vv)
print('slices',len(out),'variadic_fns',n)
