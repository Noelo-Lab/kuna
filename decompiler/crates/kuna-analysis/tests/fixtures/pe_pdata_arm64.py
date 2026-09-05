#!/usr/bin/env python3
"""Probe: a minimal ARM64 PE32+ whose .pdata holds 8-byte ARM RUNTIME_FUNCTION
records ({BeginAddress, UnwindData}), not the 12-byte x64 form.  Used to check
whether kuna's fixed 12-byte `pdata_begins` stride misreads an ARM64 image."""
import os, struct
IMAGE_BASE=0x140000000; SECT_ALIGN=0x1000; FILE_ALIGN=0x200
TEXT_RVA=0x1000; PDATA_RVA=0x2000
FUNCS=[0x1000,0x1010,0x1020,0x1030]
def build_text():
    t=bytearray(0x40)
    NOP=0xD503201F; RET=0xD65F03C0
    for f in FUNCS:
        o=f-TEXT_RVA
        struct.pack_into('<IIII',t,o,NOP,NOP,NOP,RET)
    return bytes(t)
def build_pdata():
    e=bytearray()
    for f in FUNCS:
        # packed unwind data: Flag=1 (bits 0-1), FunctionLength=4 (bits 2-12)
        packed = 1 | (4 << 2)
        e += struct.pack('<II', f, packed)
    return bytes(e)
def build():
    text=build_text(); pdata=build_pdata()
    dos=bytearray(0x40); dos[0:2]=b'MZ'; struct.pack_into('<I',dos,0x3C,0x40)
    nsec=2; opt_size=240
    hdr=0x40+4+20+opt_size+40*nsec
    headers_sz=(hdr+FILE_ALIGN-1)//FILE_ALIGN*FILE_ALIGN
    text_off=headers_sz; text_sz=(len(text)+FILE_ALIGN-1)//FILE_ALIGN*FILE_ALIGN
    pdata_off=text_off+text_sz; pdata_sz=(len(pdata)+FILE_ALIGN-1)//FILE_ALIGN*FILE_ALIGN
    image_sz=PDATA_RVA+(len(pdata)+SECT_ALIGN-1)//SECT_ALIGN*SECT_ALIGN
    b=bytearray(dos); b+=b'PE\0\0'
    b+=struct.pack('<HHIIIHH',0xAA64,nsec,0,0,0,opt_size,0x0022)
    opt=bytearray()
    opt+=struct.pack('<HBBIIIII',0x20B,14,0,len(text),len(pdata),0,FUNCS[3],TEXT_RVA)
    opt+=struct.pack('<Q',IMAGE_BASE)
    opt+=struct.pack('<IIHHHHHHIIIIHHQQQQII',SECT_ALIGN,FILE_ALIGN,6,0,0,0,6,0,0,
                     image_sz,headers_sz,0,3,0x8160,0x100000,0x1000,0x100000,0x1000,0,16)
    dirs=[(0,0)]*16; dirs[3]=(PDATA_RVA,len(pdata))
    for rva,sz in dirs: opt+=struct.pack('<II',rva,sz)
    assert len(opt)==opt_size
    b+=opt
    def sect(n,vsz,rva,rsz,roff,ch):
        return n.encode().ljust(8,b'\0')+struct.pack('<IIIIIIHHI',vsz,rva,rsz,roff,0,0,0,0,ch)
    b+=sect('.text',len(text),TEXT_RVA,text_sz,text_off,0x60000020)
    b+=sect('.pdata',len(pdata),PDATA_RVA,pdata_sz,pdata_off,0x40000040)
    b+=bytes(headers_sz-len(b)); b+=text.ljust(text_sz,b'\0'); b+=pdata.ljust(pdata_sz,b'\0')
    return bytes(b)
if __name__=='__main__':
    out=os.path.join(os.path.dirname(os.path.abspath(__file__)),'pe_pdata_arm64.exe')
    open(out,'wb').write(build()); print(f'wrote {out} ({os.path.getsize(out)} bytes)')
