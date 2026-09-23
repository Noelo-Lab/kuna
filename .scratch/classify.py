import os, sys, re, json, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import castlib as C

PTR = re.compile(r'\*$')

def ctx_before(body, pos, n=40):
    return body[max(0,pos-n):pos]

def bucket(body, pos, ty, nxt):
    """Classify one cast by its syntactic context."""
    isptr = ty.endswith('*')
    pre = ctx_before(body, pos)
    deref = bool(re.search(r'\*\s*$', pre))          # *(T *)...
    addrof = bool(re.search(r'&\s*$', pre))
    nx = nxt.lstrip()
    # operand shape
    op_paren  = nx.startswith('(')
    op_addr   = nx.startswith('&')
    op_const  = bool(re.match(r'(0x[0-9a-fA-F]+|\d+)\b', nx))
    op_str    = nx.startswith('"')
    op_call   = bool(re.match(r'[A-Za-z_]\w*\s*\(', nx))
    op_glob   = bool(re.match(r'(dat_|qword_|dword_|byte_|word_|off_|unk_|sub_|stdout|stderr|stdin)', nx))

    if deref and isptr:
        if op_paren: return 'A_deref_computed'      # *(T *)(x + k)
        if op_addr:  return 'B_deref_addrof'        # *(T *)&v
        if op_const: return 'C_deref_const'         # *(T *)0x1234
        return 'D_deref_var'                        # *(T *)p
    if isptr:
        if op_const: return 'E_ptr_from_const'
        if op_addr:  return 'F_ptr_from_addrof'
        if op_call:  return 'G_ptr_from_call'
        return 'H_ptr_repoint'
    # scalar target
    if op_addr or op_str:      return 'I_int_from_addr'
    if op_const:               return 'J_scalar_const'
    if op_call:                return 'K_scalar_from_call'
    return 'L_scalar_var'

CALLARG = re.compile(r'[A-Za-z_]\w*\s*\($')
def in_arg_position(body, pos):
    """cast sits directly after '(' or ',' of a call"""
    j = pos-1
    while j>=0 and body[j] in ' \t\n': j-=1
    if j<0: return False
    if body[j]==',': return True
    if body[j]=='(':
        k=j-1
        while k>=0 and body[k] in ' \t': k-=1
        m=re.search(r'([A-Za-z_]\w*)\s*$', body[:j])
        return bool(m and m.group(1) not in ('if','while','for','switch','return','sizeof'))
    return False

SAMPLE = None
def run(sample):
    res = {}
    for dec in ('ida','kuna'):
        cnt = collections.Counter(); ex = collections.defaultdict(list); arg = collections.Counter(); tot=0
        for opt,proj,b in sample:
            t = C.load(opt,proj,dec,b)
            if not t: continue
            kt = C.declared_types(t); lines = t.split('\n')
            for pos,ty,nxt in C.find_casts(t,kt):
                bk = bucket(t,pos,ty,nxt); cnt[bk]+=1; tot+=1
                if in_arg_position(t,pos): arg[bk]+=1
                if len(ex[bk])<6:
                    ln=t[:pos].count('\n'); ex[bk].append(f'{b}:{lines[ln].strip()[:110]}')
        res[dec]=(cnt,ex,arg,tot)
    return res

if __name__=='__main__':
    SAMPLE=[('O0','coreutils','fmt'),('O0','coreutils','ls'),('O0','coreutils','sort'),('O0','coreutils','du'),
            ('O2','coreutils','fmt'),('O2','coreutils','ls'),('O2','coreutils','sort'),('O2','coreutils','du'),
            ('O0','grep','grep'),('O0','gzip','gzip'),('O2','grep','grep'),('O2','gzip','gzip'),
            ('O0','diffutils','diff'),('O2','diffutils','diff'),('O0','findutils','find'),('O2','findutils','find')]
    res = run(SAMPLE)
    keys = sorted(set(res['ida'][0])|set(res['kuna'][0]))
    print(f"{'bucket':20} {'ida':>7} {'kuna':>7} {'delta':>7}   ida_arg kuna_arg")
    for k in keys:
        i=res['ida'][0][k]; u=res['kuna'][0][k]
        print(f'{k:20} {i:7} {u:7} {u-i:+7}   {res["ida"][2][k]:6} {res["kuna"][2][k]:6}')
    print('TOTAL', res['ida'][3], res['kuna'][3])
    print()
    for k in keys:
        print('==',k)
        for d in ('ida','kuna'):
            for e in res[d][1][k][:3]: print(f'   {d:5}| {e}')
