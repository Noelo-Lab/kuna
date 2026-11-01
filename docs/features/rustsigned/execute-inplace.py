from pathlib import Path
import subprocess,ctypes,mmap,json,os,hashlib
w=Path('/tmp/kuna-rust-signed-bytes');r=Path('/home/mahaloz/kwt/angr-rust-signed-bytes');base=Path('/home/mahaloz/kwt/angr-rust-userop')
code=bytes.fromhex('4088f885f67406d0f8ffce75fac3')
mem=mmap.mmap(-1,len(code),prot=mmap.PROT_READ|mmap.PROT_WRITE|mmap.PROT_EXEC);mem.write(code)
fn=ctypes.CFUNCTYPE(ctypes.c_uint8,ctypes.c_int8,ctypes.c_uint32)(ctypes.addressof(ctypes.c_char.from_buffer(mem)))
expected=[fn(a,n) for a in range(256) for n in range(32)]
records=[]
for proto,label in [('char','signed-text'),('unsigned char','unsigned-type-conflict')]:
 for side,tree in [('before',base),('after',r)]:
  cmd=f'load file {w}/inplace.xml\noption outlang rust\nparse line extern {proto} byte_loop_direct(char a, unsigned int n);\nlo fu byte_loop_direct\ndecompile\nprint C\nquit\n'
  p=subprocess.run([str(tree/'decompiler/target/release/decomp_dbg'),'-s',str(r/'specs')],input=cmd,text=True,capture_output=True,check=True)
  body=p.stdout.split('#[allow',1)[1].split('[decomp]>',1)[0].strip();body='#[allow'+body+'\n'
  stem=f'inplace-{label}-{side}';(w/(stem+'.rs')).write_text(body)
  source=w/(stem+'-execution.rs');source.write_text(body+'fn main() { for a in 0..256u16 { for n in 0..32u32 { println!("{}", unsafe { byte_loop_direct(a as _, n) }); } } }\n')
  for version in ['1.90.0','1.93.1','1.99.1']:
   binary=w/(stem+'-'+version);comp=['rustc']+([] if version=='1.90.0' else ['+'+version])
   c=subprocess.run(comp+['--crate-name','signed_inplace','-C','opt-level=3',str(source),'-o',str(binary)],capture_output=True,text=True);assert c.returncode==0,c.stderr
   actual=list(map(int,subprocess.check_output([str(binary)],text=True).splitlines()));assert len(actual)==8192
   wrong=sum(a!=b for a,b in zip(actual,expected));assert wrong==(0 if side=='after' else 3968),(label,side,version,wrong)
   records.append({'prototype':proto,'scope':label,'side':side,'compiler':version,'rows':8192,'mismatches':wrong,'untouched_body':True})
  assert ('a >>= 1;' in body)==(side=='before'),body
record={'native_machine_code_hex':code.hex(),'architecture':'x86-64 System V','native_oracle':'the exact fixture bytes executed via ctypes','inputs':'all 256 byte values and loop counts 0 through 31','records':records,'coverage':'Both signed-text and unsigned recovered inputs now preserve the arithmetic shift.'}
(w/'inplace-execution.json').write_text(json.dumps(record,indent=2)+'\n');print(json.dumps(record,indent=2))
