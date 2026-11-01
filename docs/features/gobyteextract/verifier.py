import re
token=re.compile(r'//[^\n]*|/\*[\s\S]*?\*/|"(?:\\.|[^"\\])*"|\x27(?:\\.|[^\x27\\])*\x27|`[^`]*`|[A-Za-z_][A-Za-z_0-9]*|[0-9][A-Za-z_0-9.]*|\s+|<<=|>>=|&\^=|\.\.\.|\+\+|--|:=|==|!=|<=|>=|&&|\|\||<<|>>|&\^|<-|[+*/%&|^\-]=|.',re.DOTALL)
def lex(code): return [t for t in token.findall(code) if not t.isspace()]

def view(tokens,i):
    if tokens[i:i+3]!=['*','(','*']or i+20>=len(tokens):return None
    ty=tokens[i+3]
    if not re.fullmatch(r'u?int(?:8|16|32|64)|byte',ty):return None
    if tokens[i+4:i+15]!=[')', '(', 'unsafe','.','Add','(','unsafe','.','Pointer','(','&']:return None
    name=tokens[i+15]
    if not re.fullmatch(r'[A-Za-z_]\w*',name):return None
    if tokens[i+16:i+18]!=[')',',']or tokens[i+19:i+21]!=[')',')']:return None
    if not re.fullmatch(r'0x[0-9a-f]+|[0-9]+',tokens[i+18]):return None
    return name,ty,1 if ty=='byte'else int(re.search(r'[0-9]+',ty)[0])//8,int(tokens[i+18],0)

def verify(before,after,name,big=False):
    arrays={n:int(k)for n,k in re.findall(r'\b([A-Za-z_]\w*)\s+\[([0-9]+)\](?:byte|uint8)\b',before)}
    a,b=lex(before),lex(after);i=j=0;conversions=[]
    while i<len(a):
        candidate=view(b,j) if re.fullmatch(r'SUB[0-9]+',a[i]) else None
        if candidate:
            local,ty,width,offset=candidate
            assert local in arrays,(name,local,arrays)
            size=arrays[local];lsb=size-width-offset if big else offset
            expected=[f'SUB{size}{width}','(',local,',',a[i+4],')']
            assert a[i:i+6]==expected,(name,a[i:i+8],expected)
            assert re.fullmatch(r'0x[0-9a-f]+|[0-9]+',a[i+4]),(name,a[i:i+6])
            assert int(a[i+4],0)==lsb and offset>=0 and offset+width<=size,(name,candidate,a[i:i+6],size,big)
            conversions.append({'local':local,'array_size':size,'memory_offset':offset,'value_offset':lsb,'width':width,'type':ty,'big_endian':big})
            i+=6;j+=21;continue
        assert j<len(b)and a[i]==b[j],('non-extraction token change',name,a[max(0,i-8):i+12],b[max(0,j-8):j+12])
        i+=1;j+=1
    assert j==len(b),('extra tokens',name,b[j:]);assert conversions,name
    return {'extractions':conversions}
