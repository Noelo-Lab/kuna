"""Classify what a foldcallret change did to a whole-binary `decompile-all` dump.

Usage: CA=<before-dir> CB=<after-dir> python3 foldclassify.py

Both directories hold one `<name>.c` per binary, produced by
`kuna decompile-all <binary> > <name>.c`.  Functions are split on the
`// Function: <name> @ <addr>` banner and compared one by one, and each changed
function lands in exactly one bucket:

* `defold`        - the multiset of call names is unchanged and the function
                    gained at least one `x = f(...)` binding: a call that was
                    printed inside an expression got its statement back.
* `defold-dedup`  - a call name occurs FEWER times and none occurs more: the
                    folded form was being emitted at two sinks and now is not.
* `renumber-only` - identical once `vN` names are erased.
* `FLAG-...`      - a call gained an occurrence or vanished: a real defect.
* `OTHER-...`     - same calls, no new binding, text differs: read by hand.

The buckets are the PR's hunk classification; `FLAG` must stay empty.
"""

import difflib, re, os, collections
A = os.environ['CA']
B = os.environ['CB']
FN = re.compile(r'^// Function: (\S+) @ (\S+)')
CALL = re.compile(r'\b([A-Za-z_]\w*)\s*\(')
KW = {'if','while','for','switch','return','sizeof','do','else'}
# a call bound to a name: `x = f(..)` or `x = (cast)f(..)`, statement or comma form
ASSIGN_CALL = re.compile(r'=\s*(?:\([^()]*\)\s*)?[A-Za-z_]\w*\s*\(')

def funcs(path):
    out={}; cur=None; buf=[]
    for line in open(path):
        m=FN.match(line)
        if m:
            if cur: out[cur]=buf
            cur=m.group(2); buf=[]
        buf.append(line.rstrip('\n'))
    if cur: out[cur]=buf
    return out
def callms(lines):
    c=collections.Counter()
    for l in lines:
        if l.strip().startswith('//'): continue
        for n in CALL.findall(l):
            if n not in KW: c[n]+=1
    return c
def norm(lines):
    return [re.sub(r'\bv\d+\b','v',l) for l in lines if l.strip() and not l.strip().startswith('//')]
def nassign(lines):
    return sum(len(ASSIGN_CALL.findall(l)) for l in lines if not l.strip().startswith('//'))

summary=collections.Counter(); eyes=[]; changed=0; total=0; gained=0; dedup_saved=0
for name in sorted(os.listdir(A)):
    if not name.endswith('.c'): continue
    fa,fb=funcs(os.path.join(A,name)),funcs(os.path.join(B,name))
    keys=set(fa)|set(fb); total+=len(keys)
    for k in sorted(keys):
        la,lb=fa.get(k),fb.get(k)
        if la is None or lb is None:
            summary['FUNCTION-APPEARED-OR-VANISHED']+=1; eyes.append((name,k,'FUNCTION-APPEARED-OR-VANISHED',la or [],lb or [])); continue
        if la==lb: continue
        changed+=1
        ca,cb=callms(la),callms(lb)
        d=nassign(lb)-nassign(la)
        if ca==cb:
            if d>0: summary['defold']+=1; gained+=d
            elif norm(la)==norm(lb): summary['renumber-only']+=1
            else: summary['OTHER-same-calls']+=1; eyes.append((name,k,'OTHER-same-calls',la,lb))
        else:
            gainers={n:cb[n]-ca[n] for n in set(ca)|set(cb) if cb[n]>ca[n]}
            losers={n:cb[n]-ca[n] for n in set(ca)|set(cb) if cb[n]<ca[n]}
            vanished=[n for n in losers if cb.get(n,0)==0]
            if gainers or vanished:
                summary['FLAG-call-gained-or-vanished']+=1; eyes.append((name,k,f'FLAG {gainers} {losers}',la,lb))
            else:
                summary['defold-dedup']+=1; dedup_saved+= -sum(losers.values())
print(f"functions total={total} changed={changed}")
print(f"call bindings gained (folds lost) = {gained}; duplicated call texts removed = {dedup_saved}")
for k,v in summary.most_common(): print(f"  {k}: {v}")
print(f"\n=== {len(eyes)} functions needing eyes ===")
for name,k,kind,la,lb in eyes:
    print(f"--- {name} {k} [{kind}]")
    for l in difflib.unified_diff(la,lb,lineterm='',n=1): print(l)
    print()
