import sys,collections
sys.path.insert(0,".")
import castcount as CC
from pathlib import Path
# merge signedness: the SIGN of a cast never changes whether a cast exists
M={"i8":"8","u8":"8","char":"8","uchar":"8","bool":"8","undef":"8",
   "i16":"16","u16":"16","i32":"32","u32":"32","i64":"64","u64":"64",
   "i128":"128","u128":"128","f32":"f32","f64":"f64","f80":"f80","void":"void",
   "code":"code","fnptr":"fnptr"}
def norm(t):
    s=0
    while t.endswith("*"): t=t[:-1]; s+=1
    if t.startswith("struct:") or t.startswith("named:"): b="REC"
    else: b=M.get(t,t)
    # char* vs u8* are the same shape for counting, but keep char distinct at depth 0
    return b+"*"*s
def hist(files):
    h=collections.Counter(); ex={}
    for f in files:
        src=Path(f).read_text(errors="replace")
        t=CC.tokenize(src); v=CC.harvest_types(t,src)
        for ty,s,ln,txt in CC.find_casts(t,v):
            k=(norm(ty),s); h[k]+=1
            if k not in ex: ex[k]=(txt,f,ln)
    return h,ex
K,KE=hist(sorted(Path("shared/kuna").rglob("*.c")))
I,_=hist(sorted(Path("shared/ida").rglob("*.c")))
B,_=hist(sorted(Path("shared/kbase").rglob("*.c")))
tk,ti=sum(K.values()),sum(I.values())
print(f"kuna {tk} casts, ida {ti} casts, kuna_base {sum(B.values())} (same 4,815 functions)")
print(f"\n{'#':>3} {'shape (sign-merged)':<22}{'kuna':>7}{'%':>6}{'base':>7}{'ida':>7}{'k-ida':>7}  example")
for i,((ty,s),n) in enumerate(K.most_common(30),1):
    ida=I.get((ty,s),0); b=B.get((ty,s),0)
    txt=KE[(ty,s)][0]
    print(f"{i:>3} {ty+s:<22}{n:>7}{100.0*n/tk:>5.1f}%{b:>7}{ida:>7}{n-ida:>+7}  {txt[:26]}")
print("\n  --- shapes where IDA has MORE than kuna (top 8)")
for (ty,s),n in sorted(I.items(), key=lambda kv:-(kv[1]-K.get(kv[0],0)))[:8]:
    print(f"    {ty+s:<22} ida={n:<7} kuna={K.get((ty,s),0):<7} {K.get((ty,s),0)-n:+}")
