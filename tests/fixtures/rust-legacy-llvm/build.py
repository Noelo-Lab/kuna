"""Write harmless symbol-only ELF fixtures; never execute them."""
from pathlib import Path
import struct
import json
import hashlib
root = Path(__file__).resolve().parent
def elf(path,names):
 text=b''.join(bytes.fromhex('31c0c3')+b'\x90'*13 for _ in names)
 namestr=bytearray(b'\0');syms=bytearray(bytes(24))
 for i,name in enumerate(names):
  idx=len(namestr);namestr.extend(name.encode()+b'\0')
  syms.extend(struct.pack('<IBBHQQ',idx,0x12,0,1,0x401000+16*i,3))
 data=bytearray(4096)+text
 def add(b,alignment=1):
  data.extend(bytes((-len(data))%alignment));off=len(data);data.extend(b);return off
 symoff=add(syms,8);stroff=add(namestr)
 shstr=b'\0.text\0.symtab\0.strtab\0.shstrtab\0';shstroff=add(shstr)
 data.extend(bytes((-len(data))%8));shoff=len(data)
 headers=[bytes(64),struct.pack('<IIQQQQIIQQ',1,1,6,0x401000,4096,len(text),0,0,16,0),struct.pack('<IIQQQQIIQQ',7,2,0,0,symoff,len(syms),3,1,8,24),struct.pack('<IIQQQQIIQQ',15,3,0,0,stroff,len(namestr),0,0,1,0),struct.pack('<IIQQQQIIQQ',23,3,0,0,shstroff,len(shstr),0,0,1,0)]
 data.extend(b''.join(headers))
 data[:64]=struct.pack('<16sHHIQQQIHHHHHH',b'\x7fELF\x02\x01\x01'+bytes(9),2,62,1,0x401000,64,shoff,0,64,56,1,64,5,4)
 data[64:120]=struct.pack('<IIQQQQQQ',1,5,0,0x400000,0x400000,4096+len(text),4096+len(text),4096)
 path.write_bytes(data);path.chmod(0o644)
 return {'path':str(path),'sha256':hashlib.sha256(data).hexdigest(),'symbols':[{'name':s,'address':hex(0x401000+i*16)} for i,s in enumerate(names)],'executed':False}
closure = "_ZN3std2rt19lang_start_internal28_$u7b$$u7b$closure$u7d$$u7d$17hf421b6f6b8a4a2f4E"
cases = {
    "llvm": closure + ".llvm.9325873435131735662",
    "ordinary": closure,
    "cpp": "_ZN4demo3fooE.llvm.9325873435131735662",
    "uppercase": "_ZN4demo17hABCDEF0123456789E.llvm.9325873435131735662",
    "uppercase-bare": "_ZN4demo17hABCDEF0123456789E",
    "malformed": "_ZN999bad17h0123456789abcdefE.llvm.9325873435131735662",
    "invalid-suffix": "_ZN4demo4keep17h0123456789abcdefE.llvm.bad",
}
manifest = []
for case, symbol in cases.items():
    row = elf(root / (case + ".elf"), [symbol])
    row["path"] = case + ".elf"
    manifest.append(row)
(root / "manifest.json").write_text(json.dumps(manifest, indent=2) + "\n")
