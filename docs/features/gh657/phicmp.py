"""Per-function CONTENT comparison of the `--option foldcallretphi on` dumps.

Usage: python3 phicmp.py <tag> <dir>, where <dir> holds
<tag>.{main,fix}.{off,on}.c produced by `kuna decompile-all <bin>` with and
without `--option foldcallretphi on`, on a main build and on this branch.

The acceptance test is the last line: a function may render differently under
`foldcallretphi on` only where the DEFAULT rendering also changed.
"""
import re, sys, collections
def split(p):
    out={}; cur=None; buf=[]
    for line in open(p, errors='replace'):
        m=re.match(r'^// Function: (\S+)', line)
        if m:
            if cur: out[cur]=''.join(buf)
            cur=m.group(1); buf=[]
        elif cur is not None: buf.append(line)
    if cur: out[cur]=''.join(buf)
    return out
tag=sys.argv[1]; d=sys.argv[2]
mo=split(f'{d}/{tag}.main.off.c'); mn=split(f'{d}/{tag}.main.on.c')
fo=split(f'{d}/{tag}.fix.off.c');  fn=split(f'{d}/{tag}.fix.on.c')
keys=set(mo)|set(mn)|set(fo)|set(fn)
main_delta={k for k in keys if mo.get(k)!=mn.get(k)}
fix_delta={k for k in keys if fo.get(k)!=fn.get(k)}
off_diff={k for k in keys if mo.get(k)!=fo.get(k)}
on_diff={k for k in keys if mn.get(k)!=fn.get(k)}
leak=sorted(on_diff-off_diff)
print(f'{tag}: fns={len(keys)} phi-delta main={len(main_delta)} fix={len(fix_delta)} sets_equal={main_delta==fix_delta}')
print(f'  DEFAULT arm main!=fix: {len(off_diff)}')
print(f'  ON      arm main!=fix: {len(on_diff)}')
print(f'  ON-only (ON differs where the DEFAULT does not) = {len(leak)} {leak[:8]}')
