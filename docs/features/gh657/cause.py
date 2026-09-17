"""Attribute every default-arm change to the clause that caused it.

Variant builds (KUNA_GH657_VARIANT, instrumentation reverted before commit):
  main  origin/main 520443d7
  v1    clause 1 only (tied-write barrier on the call->use span)
  v2    v1 + the call->print-point span, structural conditions only
  v3    v2 + the tied-write test on that span
  fix   v3 + call/STORE/CALLOTHER on that span  (== the branch)
"""
import os, re, sys, collections, difflib
D = '/home/mahaloz/kwt/gh657/.scratch/sweep36'
FN = re.compile(r'^// Function: (\S+) @ (\S+)')
def funcs(path):
    out={}; cur=None; buf=[]
    for line in open(path, errors='replace'):
        m=FN.match(line)
        if m:
            if cur: out[cur]='\n'.join(buf)
            cur=m.group(2); buf=[]
        buf.append(line.rstrip('\n'))
    if cur: out[cur]='\n'.join(buf)
    return out
ARMS=['main','v1','v2','v3','fix']
cause=collections.Counter(); per=collections.defaultdict(list); total=0; changed=0
for name in sorted(os.listdir(os.path.join(D,'main'))):
    if not name.endswith('.c'): continue
    t={a: funcs(os.path.join(D,a,name)) for a in ARMS}
    keys=set()
    for a in ARMS: keys |= set(t[a])
    total += len(t['main'])
    for k in sorted(keys):
        m,f = t['main'].get(k), t['fix'].get(k)
        if m==f: continue
        changed += 1
        v1,v2,v3 = t['v1'].get(k), t['v2'].get(k), t['v3'].get(k)
        tags=[]
        if m!=v1: tags.append('i-call-to-use')      # tied write between call and use
        if v1!=v2: tags.append('iii-structural')    # no derivable/same-block print point
        if v2!=v3: tags.append('i-past-use')        # tied write between use and print point
        if v3!=f:  tags.append('ii-opcode-past-use')# call/STORE/CALLOTHER past the use
        key='+'.join(tags) if tags else 'UNATTRIBUTED'
        cause[key]+=1
        per[key].append((name,k))
print(f"functions in the corpus (main arm) = {total}")
print(f"functions whose DEFAULT output the fix changes = {changed}")
print()
buckets=collections.Counter()
for key,n in cause.items():
    for t in (key.split('+') if key!='UNATTRIBUTED' else ['UNATTRIBUTED']):
        buckets[t]+=n
print("by clause (a function can be counted in more than one):")
for k,v in buckets.most_common(): print(f"  {k}: {v}")
print()
print("exact combinations:")
for k,v in cause.most_common(): print(f"  {k}: {v}   e.g. {per[k][:3]}")
