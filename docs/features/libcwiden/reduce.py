"""Reduce gcc -aux-info NC declarations to the protos::Ty vocabulary.

Fixed rule, applied mechanically; a declaration with any slot that has no honest
width-stable Ty spelling is REJECTED whole (kuna_libcsigs.rs' own rule).
"""
import re,json,sys

NC=re.compile(r'^/\* (?P<file>[^:]+):(?P<line>\d+):NC \*/ extern (?P<decl>.*);\s*$')

def split_params(s):
    out=[];d=0;cur=''
    for ch in s:
        if ch=='(' : d+=1
        if ch==')' : d-=1
        if ch==',' and d==0: out.append(cur.strip()); cur=''; continue
        cur+=ch
    if cur.strip(): out.append(cur.strip())
    return out

INT4={'int','unsigned int','signed int','__pid_t','pid_t','__uid_t','uid_t','__gid_t','gid_t','__mode_t','mode_t','wint_t','__socklen_t','socklen_t','sig_atomic_t','__sig_atomic_t','_Bool','__id_t','id_t','__key_t','key_t','__useconds_t','useconds_t','clockid_t','__clockid_t','idtype_t','__int32_t','int32_t','__uint32_t','uint32_t','unsigned','enum __socket_type'}
UNSIGNED4={'unsigned int','wint_t','__mode_t','mode_t','__uid_t','uid_t','__gid_t','gid_t','__socklen_t','socklen_t','__id_t','id_t','__useconds_t','useconds_t','__uint32_t','uint32_t','unsigned'}
PTRW_U={'size_t','__size_t','unsigned long','unsigned long int','long unsigned int','uintptr_t','__uintptr_t'}
PTRW_S={'long','long int','ssize_t','__ssize_t','ptrdiff_t','__ptrdiff_t','intptr_t','__intptr_t','long signed int','long signed'}
I64={'long long','long long int','long long signed int','unsigned long long','unsigned long long int','long long unsigned int','intmax_t','__intmax_t','uintmax_t','__uintmax_t','int64_t','__int64_t','uint64_t','__uint64_t'}

def norm(t):
    t=t.replace('const ','').replace('volatile ','').replace('restrict','').strip()
    t=re.sub(r'\s+',' ',t)
    return t

def cls(t, ret=False):
    t=norm(t)
    if t in ('void',) and ret: return 'Void'
    if t=='void': return None
    if t.endswith('*'):
        base=norm(t[:-1])
        if base=='void': return 'VoidPtr'
        if base in ('char','signed char','unsigned char'): return 'CharPtr'
        if base=='char *': return 'CharPtrPtr'
        if base in ('int','__int32_t','int32_t'): return 'IntPtr'
        if base in UNSIGNED4: return 'UIntPtr'
        if base in ('wchar_t','__wchar_t'): return 'WCharPtr'
        if base.endswith('*'): return 'VoidPtr'          # T** other than char** -> void*
        if base.startswith('struct ') or base.startswith('union ') or base.startswith('enum '): return 'VoidPtr'
        if base in ('FILE','DIR','__FILE','glob_t','regex_t','regmatch_t','iconv_t','sigset_t','fd_set','__sigset_t','va_list','__gnuc_va_list','obstack','_IO_FILE'): return 'VoidPtr'
        # any other named pointee is still a pointer slot -> void*
        if re.match(r'^[A-Za-z_][A-Za-z0-9_ ]*$', base): return 'VoidPtr'
        return None
    if t in UNSIGNED4: return 'UInt'
    if t in INT4: return 'Int'
    if t in PTRW_U: return 'Size'
    if t in PTRW_S: return 'Long'
    if t in I64: return 'I64:'+('u' if 'unsigned' in t or t.startswith('uint') or t.startswith('__uint') else 's')
    return None

def parse(decl):
    # decl like: "char *__fgets_chk (char *, size_t, int, FILE *)"
    m=re.match(r'^(?P<ret>.*?)\b(?P<name>[A-Za-z_][A-Za-z0-9_]*)\s*\((?P<params>.*)\)$', decl.strip())
    if not m: return None
    ret=m.group('ret').strip(); name=m.group('name'); params=m.group('params').strip()
    if '(' in ret or ')' in ret: return None
    ps=split_params(params)
    vararg=-1; fixed=[]
    for i,p in enumerate(ps):
        if p=='...': vararg=len(fixed); continue
        if p=='void' and len(ps)==1: continue
        if '(' in p and '*' in p: return None    # function pointer parameter
        fixed.append(p)
    return name, ret, fixed, vararg

def reduce_decl(decl):
    r=parse(decl)
    if r is None: return None,'unparsed'
    name,ret,ps,va=r
    rc=cls(ret,ret=True)
    if rc is None: return None,'ret:'+norm(ret)
    out=[]
    for p in ps:
        c=cls(p)
        if c is None: return None,'param:'+norm(p)
        out.append(c)
    return (name,rc,out,va),None

if __name__=='__main__':
    res={}; rej={}
    for line in open(sys.argv[1]):
        m=NC.match(line.rstrip('\n'))
        if not m: continue
        s,why=reduce_decl(m.group('decl'))
        if s is None:
            rej.setdefault(re.match(r'^(.*?)\b([A-Za-z_][A-Za-z0-9_]*)\s*\(',m.group('decl')).group(2) if re.match(r'^(.*?)\b([A-Za-z_][A-Za-z0-9_]*)\s*\(',m.group('decl')) else '?', why)
            continue
        res[s[0]]={'ret':s[1],'params':s[2],'vararg':s[3],'src':f"{m.group('file')}:{m.group('line')}"}
    json.dump(res,open('reduced.json','w'),indent=0)
    json.dump(rej,open('rejected.json','w'),indent=0)
    print('reduced',len(res),'rejected',len(rej))
