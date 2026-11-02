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
    if re.fullmatch(r'[A-Za-z_][A-Za-z_0-9]*',value):
        assignments=re.findall(r'^\s*'+re.escape(value)+r'\s*=\s*([^;]+);',body['code'],re.M)
        assert assignments and len(set(assignments))==1,('ambiguous source pointer',pointer,assignments)
        value=re.sub(r'\s+','',assignments[0])
    match=re.fullmatch(r'&(?:mut)?([A-Za-z_][A-Za-z_0-9]*)',value)
    assert match,('unsupported source pointer spelling',pointer,value)
    candidates=[v for v in body['variables']if v['name']==match[1]and v['stack_offset']is not None]
    assert len(candidates)==1,(pointer,value,candidates)
    return candidates[0]['stack_offset']

def verify_site(site,body,calls,memory,symbols):
    before,after=site['before'],site['after']
    lines=set(range(before['source_line_start'],before['source_line_end']+1))
    addresses={address for line in body['line_mappings']if line['line_number']in lines for address in line['addresses']}
    candidates=[call for call in calls if call['call_instruction']in addresses]
    assert len(candidates)==1,('ambiguous IR call',before,addresses,calls)
    call=candidates[0];descriptor=call['descriptor'];assert descriptor and descriptor['format_pointer']==0
    count=descriptor['argument_count'];pieces=descriptor['piece_count'];assert count==len(after['arguments'])and count<=16 and count<=pieces<=count+1
    raw=memory.read(descriptor['table'],pieces*16);literals=[];length=0
    for i in range(pieces):
        pointer,size=struct.unpack_from('<QQ',raw,i*16);length+=size;assert length<=4096
        literals.append(memory.read(pointer,size).decode('utf8'))
    newline=bool(pieces>count and literals[-1].endswith('\n'))
    assert after['newline']==newline
    if newline:literals[-1]=literals[-1][:-1]
    fmt=''.join(text.replace('{','{{').replace('}','}}')+('{}'if i<count else '')for i,text in enumerate(literals))
    assert fmt==after['literal'],(fmt,after['literal'])
    for source,argument in zip(descriptor['arguments'],after['arguments']):
        names=symbols.get(source['formatter'],set());assert names,source
        types=set()
        for name in names:
            match=re.fullmatch(r'core::fmt::num::imp::<impl core::fmt::Display for (u8|i8|u16|i16|u32|i32|u64|i64|usize|isize)>::fmt',name)
            assert match,('unknown or conflicting formatter alias',name)
            types.add({'usize':'u64','isize':'i64'}.get(match[1],match[1]))
        assert types=={argument['primitive']},(types,argument)
        assert pointer_offset(argument['pointer'],body)==source['pointer_stack_offset'],(argument,source)
    site['independent_descriptor']=call
    site['all_piece_bytes_and_formatter_identities_verified']=True
