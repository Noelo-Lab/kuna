"""Structural safety check for the indirectonly A/B.

For every function whose text changed, compare two skeletons:
  * CONTROL: the sequence of control keywords (if/else/while/do/for/switch/case/
    goto/label/break/continue/return) with their nesting depth;
  * CALLS:   the ordered sequence of callee tokens appearing in call position.
If either differs, a statement moved across a branch or a call -- the BUG class.
"""
import re, sys
from collections import OrderedDict

def split_funcs(path):
    funcs=OrderedDict(); cur=None; buf=[]
    for line in open(path):
        if not line.startswith((' ','\t','}','{')) and '(' in line and ')' in line and not line.rstrip().endswith(';'):
            if cur is not None: funcs[cur]=buf
            cur=line.strip(); buf=[line.rstrip('\n')]
        elif cur is not None: buf.append(line.rstrip('\n'))
    if cur is not None: funcs[cur]=buf
    return funcs

def nm(h):
    m=re.search(r'([A-Za-z_][A-Za-z0-9_]*)\s*\(',h); return m.group(1) if m else h

KW=re.compile(r'\b(if|else|while|do|for|switch|case|default|goto|break|continue|return)\b')
LBL=re.compile(r'^\s*(label_[0-9a-f]+):')
CALL=re.compile(r'\b([A-Za-z_][A-Za-z0-9_]*)\s*\(')
NOTCALL={'if','while','for','switch','sizeof','return','CONCAT44','CONCAT22','CONCAT31','SUB84','SUB41'}

def skel(lines):
    ctrl=[]; calls=[]
    for l in lines[1:]:
        d=len(l)-len(l.lstrip())
        m=LBL.match(l)
        if m: ctrl.append((d,'LABEL')); continue
        for k in KW.findall(l): ctrl.append((d,k))
        for c in CALL.findall(l):
            if c in NOTCALL: continue
            calls.append(c)
    return ctrl, calls

a,b,lbl=sys.argv[1],sys.argv[2],sys.argv[3]
fa,fb=split_funcs(a),split_funcs(b)
ka={nm(h):h for h in fa}; kb={nm(h):h for h in fb}
bad=0
for k in ka:
    if k not in kb: print("%s %s MISSING"%(lbl,k)); bad+=1; continue
    la,lb=fa[ka[k]],fb[kb[k]]
    if la==lb: continue
    ca,la2=skel(la); cb,lb2=skel(lb)
    if ca!=cb:
        print("%s %s BUG: control skeleton moved"%(lbl,k)); bad+=1
        for i,(x,y) in enumerate(zip(ca,cb)):
            if x!=y: print("    first divergence at #%d: %s vs %s"%(i,x,y)); break
        print("    lens %d vs %d"%(len(ca),len(cb)))
    elif la2!=lb2:
        print("%s %s BUG: call sequence moved"%(lbl,k)); bad+=1
        for i,(x,y) in enumerate(zip(la2,lb2)):
            if x!=y: print("    first divergence at #%d: %s vs %s"%(i,x,y)); break
        print("    lens %d vs %d"%(len(la2),len(lb2)))
    else:
        print("%s %s OK: same control skeleton, same call sequence"%(lbl,k))
print("%s bad=%d"%(lbl,bad))
