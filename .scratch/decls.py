import re, sys, os, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import castlib as C

DECL = re.compile(r'^\s{2,}((?:const\s+|volatile\s+|struct\s+|union\s+|unsigned\s+|signed\s+|long\s+|short\s+)*[A-Za-z_]\w*(?:\s*\*)*)\s+(\*?\s*[A-Za-z_]\w*)\s*(?:\[[^\]]*\])?\s*;', re.M)
SIG  = re.compile(r'^[A-Za-z_][\w \*]*\s\**([A-Za-z_]\w*)\s*\(([^)]*)\)\s*$', re.M)

WIDTH = {'char':1,'signed char':1,'unsigned char':1,'bool':1,'_BOOL1':1,'_BYTE':1,'int8_t':1,'uint8_t':1,'uint1':1,'int1':1,'undefined1':1,'byte':1,
         'short':2,'unsigned short':2,'short int':2,'_WORD':2,'int16_t':2,'uint16_t':2,'uint2':2,'int2':2,'undefined2':2,'word':2,'__int16':2,'unsigned __int16':2,
         'int':4,'unsigned int':4,'unsigned':4,'_DWORD':4,'int32_t':4,'uint32_t':4,'uint4':4,'int4':4,'undefined4':4,'dword':4,'uint':4,'__int32':4,'unsigned __int32':4,'float':4,'_BOOL4':4,
         'long':8,'unsigned long':8,'long long':8,'unsigned long long':8,'long int':8,'unsigned long int':8,'long long int':8,'_QWORD':8,'int64_t':8,'uint64_t':8,'uint8':8,'int8':8,'undefined8':8,'qword':8,'ulong':8,'size_t':8,'ssize_t':8,'__int64':8,'unsigned __int64':8,'double':8,'time_t':8,'off_t':8,'__int128':16,'_OWORD':16}
UNSIGNED = {'unsigned char','unsigned short','unsigned int','unsigned','unsigned long','unsigned long long','unsigned long int','_BYTE','_WORD','_DWORD','_QWORD','uint8_t','uint16_t','uint32_t','uint64_t','uint1','uint2','uint4','uint8','size_t','byte','word','dword','qword','ulong','uint','bool','_BOOL1','_BOOL4','undefined1','undefined2','undefined4','undefined8','unsigned __int8','unsigned __int16','unsigned __int32','unsigned __int64'}

def norm(t):
    t = re.sub(r'\bconst\b|\bvolatile\b','',t).strip()
    t = re.sub(r'\s+',' ',t)
    return t

def _split_params(params):
    out=[]; depth=0; cur=''
    for ch in params:
        if ch in '([': depth+=1
        elif ch in ')]': depth-=1
        if ch==',' and depth==0: out.append(cur); cur=''
        else: cur+=ch
    if cur.strip(): out.append(cur)
    return out

def _params_of(text):
    """text starts at the function signature; return the parameter list string"""
    i = text.find('(')
    if i < 0: return ''
    depth = 0
    for j in range(i, min(len(text), i + 4000)):
        if text[j] == '(': depth += 1
        elif text[j] == ')':
            depth -= 1
            if depth == 0: return text[i+1:j]
    return ''

def func_decls(body, signature_line):
    """map var name -> declared type string, for one function body"""
    d = {}
    for params in (_params_of(body),):
        for p in _split_params(params):
            p = p.strip()
            if not p or p == 'void': continue
            mm = re.match(r'^(.*?[\w\)])\s*\**\s*([A-Za-z_]\w*)\s*(\[\s*\d*\s*\])?$', p)
            if mm:
                ty = norm(mm.group(1)) + ('*'*p.count('*'))
                d[mm.group(2)] = ty
    # locals: only the leading declaration block
    m = re.search(r'\n[ \t]*\n', body)
    head = body[:m.start()] if m and m.start() < 8000 else body[:3000]
    for m in DECL.finditer(head):
        ty = norm(m.group(1)); nm = m.group(2).replace('*','').strip()
        if m.group(2).lstrip().startswith('*'): ty += '*'
        if nm in ('return','if','while','else'): continue
        d[nm] = ty
    return d

def sig_and_body(text, addr):
    """return (signature_line, body) for the function at addr"""
    for n,a,b in C.split_funcs(text):
        if a==addr:
            lines=b.split('\n')
            sig='\n'.join(lines[:3])
            return sig,b
    return None,None

def relation(cast_ty, decl_ty):
    ct, dt = norm(cast_ty), norm(decl_ty)
    if ct.endswith('*') or dt.endswith('*'): return 'ptr'
    cw, dw = WIDTH.get(ct), WIDTH.get(dt)
    if cw is None or dw is None: return 'unknown'
    cu, du = ct in UNSIGNED, dt in UNSIGNED
    if ct == dt: return 'identity'
    if cw == dw: return 'sign-only' if cu != du else 'same-width'
    if cw > dw:  return 'widen-samesign' if cu == du else ('widen-signflip')
    return 'narrow'
