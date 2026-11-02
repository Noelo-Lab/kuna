import subprocess,re,json
import struct

class Image:
    def __init__(self,data):
        assert data[:6]==b'\x7fELF\x02\x01'
        self.data=data;self.loads=[];self.protected=[];self.relocations={}
        phoff=struct.unpack_from('<Q',data,32)[0];width,count=struct.unpack_from('<HH',data,54)
        for i in range(count):
            kind,flags,offset,vaddr,paddr,filesz,memsz,align=struct.unpack_from('<IIQQQQQQ',data,phoff+i*width)
            if kind==1:
                self.loads.append((vaddr,offset,filesz))
                if flags&2==0:self.protected.append((vaddr,vaddr+memsz))
            elif kind==0x6474e552:self.protected.append((vaddr,(vaddr+memsz)&~4095))
        shoff=struct.unpack_from('<Q',data,40)[0];width,count=struct.unpack_from('<HH',data,58)
        sections=[struct.unpack_from('<IIQQQQIIQQ',data,shoff+i*width)for i in range(count)]
        for section in sections:
            _,kind,_,_,offset,size,link,_,_,entry=section
            if kind!=4:continue
            assert entry==24
            for p in range(offset,offset+size,entry):
                address,info,addend=struct.unpack_from('<QQq',data,p);kind=info&0xffffffff;index=info>>32
                if kind==8:value=addend
                elif kind in [1,6,7]:
                    sym=sections[link];at=sym[4]+index*sym[9]
                    _,_,_,defined,value,_=struct.unpack_from('<IBBHQQ',data,at)
                    if defined==0:continue
                    if kind==1:value+=addend
                else:continue
                self.relocations[address]=(value&0xffffffffffffffff).to_bytes(8,'little')
    def read(self,address,size):
        assert size<=4096 and 0<address<=address+size<=0xffffffffffffffff
        if size==0:return b''
        assert any(lo<=address and address+size<=hi for lo,hi in self.protected),('not protected',hex(address),size)
        for vaddr,offset,filesz in self.loads:
            if vaddr<=address and address+size<=vaddr+filesz:
                result=bytearray(self.data[offset+address-vaddr:offset+address-vaddr+size])
                for place,value in self.relocations.items():
                    lo=max(place,address);hi=min(place+8,address+size)
                    if lo<hi:result[lo-address:hi-address]=value[lo-place:hi-place]
                return bytes(result)
        raise AssertionError(('not file-backed',hex(address),size))

def demangled_symbols(binary):
    symbols={}
    for line in subprocess.check_output(['nm','-C',str(binary)],text=True,stderr=subprocess.DEVNULL).splitlines():
        match=re.fullmatch(r'([0-9a-f]+) ([TtWw]) (.*)',line)
        if match:symbols.setdefault(int(match[1],16),set()).add(match[3])
    return symbols

def pointer_offset(pointer,body):
    value=re.sub(r'\s+','',pointer)
    while value.startswith('(') and value.endswith(')'):value=value[1:-1]
    seen=set()
    while re.fullmatch(r'[A-Za-z_][A-Za-z_0-9]*',value):
        assert value not in seen and len(seen)<8,('cyclic source pointer',pointer,value)
        seen.add(value)
        assignments=re.findall(r'^\s*'+re.escape(value)+r'\s*=\s*([^;]+);',body['code'],re.M)
        assert assignments and len(set(assignments))==1,('ambiguous source pointer',pointer,assignments)
        value=re.sub(r'\s+','',assignments[0])
    match=re.fullmatch(r'&(?:mut)?([A-Za-z_][A-Za-z_0-9]*)',value)
    assert match,('unsupported source pointer spelling',pointer,value)
    candidates=[v for v in body['variables']if v['name']==match[1]and v['stack_offset']is not None]
    assert len(candidates)==1,(pointer,value,candidates)
    return candidates[0]['stack_offset']

def template(memory,address,traits,sink):
    start=address;parts=[];next_arg=0;used=set();indexed=False
    def take(n):
        nonlocal address
        assert address+n-start<=4096
        value=memory.read(address,n);address+=n;return value
    for _ in range(128):
        head=take(1)[0]
        if head==0:break
        if head<=0x80:
            size=int.from_bytes(take(2),'little')if head==0x80 else head
            assert size>0;parts.append(take(size).decode('utf8'));continue
        assert head&0xf0==0xc0
        flags=int.from_bytes(take(4),'little')if head&1 else 0x60000020
        width=int.from_bytes(take(2),'little')if head&2 else 0
        precision=int.from_bytes(take(2),'little')if head&4 else 0
        if head&8:next_arg=int.from_bytes(take(2),'little');indexed=True
        assert next_arg<len(traits)
        fill=chr(flags&0x1fffff);align=flags>>29&3;spec=''
        if align!=3:spec+=(fill if fill!=' 'else'')+'<>^'[align]
        if flags&(1<<21):spec+='+'
        elif flags&(1<<22):spec+='-'
        if flags&(1<<23):spec+='#'
        if flags&(1<<24):spec+='0'
        if flags&(1<<27):spec+=str(width)
        if flags&(1<<28):spec+='.'+str(precision)
        trait=traits[next_arg]
        suffix={'Display':'','LowerHex':'x','UpperHex':'X','Octal':'o','Binary':'b','LowerExp':'e','UpperExp':'E'}
        if trait=='Debug':spec+=['?','x?','X?'][flags>>25&3]
        else:assert flags&(3<<25)==0;spec+=suffix[trait]
        parts.append((next_arg,spec));used.add(next_arg);next_arg+=1
    else:raise AssertionError('unterminated template')
    assert used==set(range(len(traits)))
    newline=sink in ['stdout','stderr','write']and isinstance(parts[-1],str)and parts[-1].endswith('\n')
    if newline:parts[-1]=parts[-1][:-1]
    literal=''.join(p.replace('{','{{').replace('}','}}')if isinstance(p,str)else'{'+(str(p[0])if indexed else'')+(':'+p[1]if p[1]else'')+'}'for p in parts)
    name={'stdout':'print','stderr':'eprint','panic':'panic','format':'format','write':'write'}[sink]+('ln'if newline else'')
    return name,literal,{'address':hex(start),'bytes':memory.read(start,address-start).hex(),'used_arguments':sorted(used)}

def source_word(expression,body,word,stack_offset):
    assert word and word['size']==8
    try:constant=int(expression,0)
    except ValueError:constant=None
    if constant is not None:
        assert constant==word['constant'];return
    candidates=[v for v in body['variables']if v['name']==expression]
    assert len(candidates)==1,('source word name',expression,candidates)
    variable=candidates[0]
    assert variable['stack_offset']==stack_offset,(expression,variable,word,stack_offset)

def verify_arguments(site,body,calls,memory,symbols):
    before,after=site['before'],site['after'];params=after['parameters']
    lines=set(range(before['source_line_start'],before['source_line_end']+1))
    addresses={address for line in body['line_mappings']if line['line_number']in lines for address in line['addresses']}
    candidates=[call for call in calls if call['call_instruction']in addresses]
    assert len(candidates)==1,('ambiguous IR call',before,addresses,calls)
    call=candidates[0];descriptor=call['descriptor'];assert descriptor and descriptor['argument_count']==len(params)
    traits=[];forwarders=[]
    for source,argument in zip(descriptor['arguments'],params):
        names=symbols.get(source['formatter'],set());assert names,source
        if argument['kind']=='string':
            assert names=={'<&T as core::fmt::Display>::fmt'},names
            raw=memory.read(source['formatter'],18)
            assert raw[:14]==bytes.fromhex('4889f0488b37488b57084889c7e9')
            target=source['formatter']+18+int.from_bytes(raw[14:],'little',signed=True)
            assert symbols.get(target)=={'core::fmt::Formatter::pad'},symbols.get(target)
            source_word(argument['data'],body,source['data_word'],source['pointer_stack_offset']);source_word(argument['length'],body,source['length_word'],source['pointer_stack_offset']+8)
            traits.append('Display');forwarders.append({'address':hex(source['formatter']),'bytes':raw.hex(),'target':hex(target),'target_name':'core::fmt::Formatter::pad','data_word':source['data_word'],'length_word':source['length_word']})
        else:
            identities=set()
            for name in names:
                match=re.fullmatch(r'core::fmt::num::(?:imp::)?<impl core::fmt::(Display|Debug|LowerHex|UpperHex|Octal|Binary|LowerExp|UpperExp) for (u8|i8|u16|i16|u32|i32|u64|i64|usize|isize)>::fmt',name)
                assert match,('unknown formatter',name)
                identities.add(({'usize':'u64','isize':'i64'}.get(match[2],match[2]),match[1]))
            assert len(identities)==1;ty,trait=identities.pop();assert ty==argument['primitive'];traits.append(trait)
            assert pointer_offset(argument['pointer'],body)==source['pointer_stack_offset']
    name,literal,decoded=template(memory,descriptor['template'],traits,before['sink'])
    assert name==after['name']and literal==after['literal'],(name,literal,after)
    site['independent_descriptor']=call;site['template_bytes']=decoded;site['verified_string_forwarders']=forwarders
    site['all_template_bytes_and_argument_bindings_verified']=True


def verify_site(site,body,calls,memory,symbols):
    before,after=site['before'],site['after'];lines=set(range(before['source_line_start'],before['source_line_end']+1))
    addresses={address for line in body['line_mappings']if line['line_number']in lines for address in line['addresses']}
    candidates=[call for call in calls if call['call_instruction']in addresses];assert len(candidates)==1
    call=candidates[0];assert symbols.get(call['callee'])=={'core::fmt::write'}
    assert call['runtime_argument_count']==4 and call['result_size']==1 and call['result_type']==after['result_type']
    pointer=call['vtable'];assert pointer and pointer%8==0
    raw=memory.read(pointer,48);words=struct.unpack('<6Q',raw);assert words[1:3]==(24,8)
    methods={0:'core::ptr::drop_in_place<alloc::string::String>',3:'<alloc::string::String as core::fmt::Write>::write_str',4:'<alloc::string::String as core::fmt::Write>::write_char',5:'core::fmt::Write::write_fmt'}
    assert all(symbols.get(words[index])=={name}for index,name in methods.items())
    descriptor=call['descriptor'];assert descriptor and descriptor['argument_count']==len(after['parameters'])
    if 'literal_tag'in descriptor:
        assert not after['parameters'];tag=descriptor['literal_tag'];assert tag&1==1
        message=memory.read(descriptor['template'],tag>>1).decode('utf8');newline=message.endswith('\n')
        if newline:message=message[:-1]
        literal=message.replace('{','{{').replace('}','}}')
        assert after['name']==('writeln'if newline else'write')and after['literal']==literal
        site['template_bytes']={'address':hex(descriptor['template']),'literal_tag':tag,'bytes':memory.read(descriptor['template'],tag>>1).hex()}
        site['independent_descriptor']=call;site['all_template_bytes_and_argument_bindings_verified']=True
    else:verify_arguments(site,body,calls,memory,symbols)
    site['verified_writer_vtable']={'address':hex(pointer),'bytes':raw.hex(),'words':[hex(x)for x in words],'method_identities':methods}
    site['result_bridge_type_verified']=True
