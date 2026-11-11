import re,collections

token=re.compile(r'//[^\n]*|/\*[\s\S]*?\*/|"(?:\\.|[^"\\])*"|\x27(?:\\.|[^\x27\\])*\x27|`[^`]*`|[A-Za-z_][A-Za-z_0-9]*|[0-9][A-Za-z_0-9.]*|\s+|<<=|>>=|&\^=|\.\.\.|\+\+|--|:=|==|!=|<=|>=|&&|\|\||<<|>>|&\^|<-|[+*/%&|^\-]=|.',re.DOTALL)
ident=r'[A-Za-z_]\w*';number=r'(?:0x[0-9a-f]+|[0-9]+)'
path=re.compile(r'('+ident+r'(?:\s*\.\s*'+ident+r'|\s*\[\s*'+number+r'\s*\])*)$')

def lex(code):return [t for t in token.findall(code)if not t.isspace()]
def reverse_views(tokens):
    out=[];views=[];i=0
    prefix=['*','(','*'];middle=[')', '(', 'unsafe','.','Add','(','unsafe','.','Pointer','(','&']
    while i<len(tokens):
        if tokens[i:i+3]!=prefix or i+4>=len(tokens) or not re.fullmatch(r'u?int(?:8|16|32|64)|byte',tokens[i+3]) or tokens[i+4:i+15]!=middle:
            out.append(tokens[i]);i+=1;continue
        j=i+15;start=j
        if j>=len(tokens)or not re.fullmatch(ident,tokens[j]):out.append(tokens[i]);i+=1;continue
        j+=1
        while j<len(tokens):
            if tokens[j:j+1]==['.']and j+1<len(tokens)and re.fullmatch(ident,tokens[j+1]):j+=2
            elif tokens[j:j+1]==['[']and j+2<len(tokens)and re.fullmatch(number,tokens[j+1])and tokens[j+2]==']':j+=3
            else:break
        if tokens[j:j+2]!=[')',',']or j+4>=len(tokens)or not re.fullmatch(number,tokens[j+2])or tokens[j+3:j+5]!=[')',')']:
            out.append(tokens[i]);i+=1;continue
        ty=tokens[i+3];width=1 if ty=='byte'else int(re.search(r'[0-9]+',ty).group())//8;offset=int(tokens[j+2],0);p=''.join(tokens[start:j]);views.append((p,offset,width,ty))
        out.extend(tokens[start:j]+['.',f'_{offset}_{width}_']);i=j+5
    return out,views

def verify(before,after,name):
    a,old=reverse_views(lex(before));b,new=reverse_views(lex(after));old_counts=collections.Counter(old);new_counts=collections.Counter(new)
    assert not old_counts-new_counts,('existing typed view changed',name,old_counts-new_counts)
    conversions=[{'path':p,'offset':o,'size':s,'type':t}for (p,o,s,t),n in (new_counts-old_counts).items()for _ in range(n)];covers=[]
    arrays={m[0]:int(m[1])for m in re.findall(r'\bvar\s+([A-Za-z_]\w*)\s+\[([0-9]+)\](?:byte|uint8)\b',before)}
    i=j=0
    while i<len(a):
        whole=re.fullmatch(r'_0_([0-9]+)_',a[i+1])if a[i]=='.'and i+1<len(a)else None
        if whole is not None and b[j:j+2]!=a[i:i+2]:
            m=path.search(' '.join(a[:i]));assert m is not None,(name,a[max(0,i-15):i+4]);p=re.sub(r'\s+','',m[1]);n=int(whole[1]);assert n>1,(name,p,n)
            declared=arrays.get(p);assert declared is None or declared==n,(name,p,n,declared)
            covers.append({'path':p,'size':n,'direct_local_extent_verified':declared==n});i+=2;continue
        assert j<len(b)and a[i]==b[j],('non-view token change',name,a[max(0,i-10):i+15],b[max(0,j-10):j+15])
        i+=1;j+=1
    assert j==len(b),('extra output tokens',name,b[j:]);assert conversions or covers,name
    for c in conversions:
        if c['path']in arrays:assert c['offset']+c['size']<=arrays[c['path']],(name,c,arrays[c['path']])
        c['direct_local_extent_verified']=c['path']in arrays
    return {'integer_views':conversions,'whole_array_covers':covers}
