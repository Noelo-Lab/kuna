import sys,re,collections
sys.path.insert(0,".")
import castcount as CC
from pathlib import Path
W={"i8":1,"u8":1,"char":1,"uchar":1,"bool":1,"undef":1,"i16":2,"u16":2,
   "i32":4,"u32":4,"f32":4,"i64":8,"u64":8,"f64":8,"i128":16,"u128":16}
def width(t):
    return 8 if t.endswith("*") else W.get(t)
# *(T *)((long)V + CONST)  -- capture T and CONST
PAT=re.compile(r'\*\(\s*([A-Za-z_][A-Za-z_0-9 ]*?)\s*(\*+)\s*\)\(\(\s*(?:unsigned\s+)?long\s*\)\s*([A-Za-z_][A-Za-z_0-9]*)\s*\+\s*(0x[0-9a-fA-F]+|\d+)\s*\)')
SPELL={"unsigned char":"u8","char":"char","unsigned short":"u16","short":"i16",
       "unsigned int":"u32","int":"i32","unsigned long":"u64","long":"i64",
       "float":"f32","double":"f64","void":"void","bool":"bool"}
tot=collections.Counter(); byw=collections.Counter()
for f in sys.argv[1:]:
    for m in PAT.finditer(Path(f).read_text(errors="replace")):
        base,stars,var,off=m.group(1).strip(),m.group(2),m.group(3),m.group(4)
        o=int(off,16) if off.startswith("0x") else int(off)
        ty=SPELL.get(base,"?")
        w=8 if len(stars)>1 else width(ty)
        tot["sites"]+=1
        if w and o%w==0: tot["offset divides the access width -> (T *)P + K/w"]+=1; byw[(ty+stars,w)]+=1
        elif w: tot["offset does NOT divide -> stays byte arithmetic"]+=1
        else: tot["unknown width"]+=1
for k,v in tot.most_common(): print(f"  {k:<50}{v:>7}")
print("\n  by access type (divisible sites):")
for (t,w),n in byw.most_common(12): print(f"    {t:<16} w={w:<3} {n:>6}")
