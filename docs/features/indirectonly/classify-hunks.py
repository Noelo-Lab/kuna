"""Classify indirectonly A/B hunks.

Storage substitution (from storsub.py) removes vN renumbering for varnodes that
carry a `// stack - X` or `// reg` comment.  Varnodes with no storage comment get
an XN token by first-appearance order, so REMOVING one shifts every later XN --
pure noise.  This pass canonicalises XN to X before diffing, then reports the
hunks that still differ as the real ones.
"""
import re, sys, difflib
from collections import OrderedDict

def split_funcs(path):
    funcs = OrderedDict(); cur=None; buf=[]
    for line in open(path):
        if not line.startswith((' ','\t','}','{')) and '(' in line and ')' in line and not line.rstrip().endswith(';'):
            if cur is not None: funcs[cur]=buf
            cur=line.strip(); buf=[line.rstrip('\n')]
        elif cur is not None: buf.append(line.rstrip('\n'))
    if cur is not None: funcs[cur]=buf
    return funcs

def name(h):
    m=re.search(r'([A-Za-z_][A-Za-z0-9_]*)\s*\(',h); return m.group(1) if m else h

DECL = re.compile(r'^  (?P<ty>[A-Za-z_][^=;]*?)\bv(?P<n>\d+)\b(?P<arr>\s*\[[^\]]*\])?;\s*(//\s*(?P<cm>.*))?$')

def storage_map(lines):
    m={}
    for l in lines:
        d=DECL.match(l)
        if not d: continue
        n=d.group('n'); cm=(d.group('cm') or '').strip()
        if cm.startswith('stack'): m[n]='S['+cm.replace(' ','')+']'
        elif cm: m[n]='R['+cm+']'
        else: m[n]=None
    return m

def subst(lines, sm, canon):
    body=[l for l in lines[1:] if not DECL.match(l)]
    txt='\n'.join(body)
    order=OrderedDict()
    for mo in re.finditer(r'\bv(\d+)\b', txt): order.setdefault(mo.group(1), len(order))
    def rep(mo):
        n=mo.group(1); s=sm.get(n)
        if s: return s
        return 'X' if canon else 'X%d' % order[n]
    return re.sub(r'\bv(\d+)\b', rep, txt)

def decls(lines):
    """multiset of declaration storage comments (what a removed local shows up as)"""
    out=[]
    for l in lines:
        d=DECL.match(l)
        if d: out.append((d.group('ty').strip(), (d.group('cm') or '').strip()))
    return out

a,b,lbl=sys.argv[1],sys.argv[2],sys.argv[3]
fa,fb=split_funcs(a),split_funcs(b)
ka={name(h):h for h in fa}; kb={name(h):h for h in fb}
nchanged=0
for k in ka:
    if k not in kb: print("%s %s MISSING_IN_ON"%(lbl,k)); continue
    la,lb=fa[ka[k]],fb[kb[k]]
    if la==lb: continue
    nchanged+=1
    sa,sb=storage_map(la),storage_map(lb)
    ca,cb=subst(la,sa,True),subst(lb,sb,True)
    da,db=decls(la),decls(lb)
    ddecl = len(db)-len(da)
    if ca==cb and da==db:
        print("%s %s NOISE-ONLY (X-renumbering)"%(lbl,k)); continue
    print("%s %s REAL  decl_delta=%+d"%(lbl,k,ddecl))
    lostd=[d for d in da if d not in db]; gaind=[d for d in db if d not in da]
    if lostd: print("    declarations gone: %s"%lostd)
    if gaind: print("    declarations new : %s"%gaind)
    for l in difflib.unified_diff(ca.split('\n'),cb.split('\n'),lineterm='',n=1):
        if l.startswith(('---','+++')): continue
        print("   ",l)
print("%s changed_functions=%d"%(lbl,nchanged))
