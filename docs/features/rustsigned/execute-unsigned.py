from pathlib import Path
import subprocess,ctypes,mmap,json,hashlib
w=Path('/tmp/kuna-rust-signed-bytes');r=Path('/home/mahaloz/kwt/angr-rust-signed-bytes');base=Path('/home/mahaloz/kwt/angr-rust-userop');fixture=r/'tests/fixtures/rust-signed-bytes';functions=json.loads((w/'unsigned-functions.json').read_text())
values=sorted(set(list(range(256))+[v for bits in [16,32,64] for v in [(1<<(bits-1))-1,1<<(bits-1),(1<<(bits-1))+1,(1<<bits)-1]]));assert len(values)==268
maps=[];native=[]
for name,hexcode,proto in functions:
 code=bytes.fromhex(hexcode);mem=mmap.mmap(-1,len(code),prot=mmap.PROT_READ|mmap.PROT_WRITE|mmap.PROT_EXEC);mem.write(code);maps.append(mem)
 if name=='unsigned_loop':signature=[ctypes.c_uint8,ctypes.c_int8,ctypes.c_uint32]
 else:
  width=int(''.join(c for c in name if c.isdigit()));ty={8:ctypes.c_uint8,16:ctypes.c_uint16,32:ctypes.c_uint32,64:ctypes.c_uint64}[width];signature=[ctypes.c_uint32,ty,ty] if 'less' in name else [ty,ty,ctypes.c_uint32]
 native.append(ctypes.CFUNCTYPE(*signature)(ctypes.addressof(ctypes.c_char.from_buffer(mem))))
rows=[]
for a in values:
 for b in values:
  rows.append([f(a,b&31) if name=='unsigned_loop' else f(a,b) if 'less' in name else f(a,2) for f,(name,_,_) in zip(native,functions)])
expected=''.join(' '.join(map(str,row))+'\n' for row in rows).encode();assert len(rows)==71824
source=(fixture/'unsigned.rs').read_text();versions=['1.90.0','1.93.1','1.99.1'];records=[]
def compile_run(version,source,binary,allow_failure=False):
 comp=['rustc']+([]if version=='1.90.0'else['+'+version]);p=subprocess.run(comp+['--crate-name','signed_unsigned','-O',str(source),'-o',str(binary)],text=True,capture_output=True);(binary.with_suffix('.compile.log')).write_text(p.stdout+p.stderr);
 if p.returncode and allow_failure:return None
 assert p.returncode==0,p.stderr
 return subprocess.check_output([str(binary)])
for version in versions:
 oracle=compile_run(version,fixture/'unsigned.rs',w/f'unsigned-oracle-{version}');assert oracle==expected,version
for side,tree in [('before',base),('after',r)]:
 bodies=[]
 for name,hexcode,proto in functions:
  cmd=f'load file {fixture}/unsigned.xml\noption outlang rust\nparse line extern {proto};\nlo fu {name}\ndecompile\nprint C\nquit\n';p=subprocess.run([str(tree/'decompiler/target/release/decomp_dbg'),'-s',str(r/'specs')],input=cmd,text=True,capture_output=True,check=True)
  body='#[allow'+p.stdout.split('#[allow',1)[1].split('[decomp]>',1)[0].strip()+'\n';bodies.append(body)
 code=w/f'unsigned-{side}-execution.rs';(w/f'unsigned-{side}.rs').write_text('\n'.join(bodies));code.write_text('\n'.join(bodies)+source[source.index('fn main()'):])
 for version in versions:
  actual=compile_run(version,code,w/f'unsigned-{side}-{version}',side=='before')
  if actual is None:
   records.append({'side':side,'compiler':version,'compile_succeeded':False,'untouched_body':True});continue
  got=[list(map(int,line.split())) for line in actual.splitlines()];assert len(got)==len(rows)
  counts={name:sum(a[i]!=b[i] for a,b in zip(rows,got)) for i,(name,_,_) in enumerate(functions)}
  if side=='after':assert actual==expected,(version,counts)
  else:
   assert all(counts[name]>0 for name in ['unsigned_less8','unsigned_shift8','unsigned_shift16','unsigned_shift32','unsigned_shift64','unsigned_loop']),counts
   assert all(counts[name]==0 for name in ['unsigned_less16','unsigned_less32','unsigned_less64']),counts
  records.append({'side':side,'compiler':version,'compile_succeeded':True,'input_rows':len(rows),'results':len(rows)*len(functions),'mismatches':counts,'untouched_body':True})
record={'oracle':'Exact original x86-64 fixture bytes executed via ctypes, cross-checked against Rust source on all three compilers','fixture_sha256':hashlib.sha256((fixture/'unsigned.xml').read_bytes()).hexdigest(),'values_per_operand':268,'widths':[8,16,32,64],'all_byte_pairs':True,'includes_each_width_sign_boundary_and_maximum':True,'functions':[x[0] for x in functions],'records':records}
(w/'unsigned-execution.json').write_text(json.dumps(record,indent=2)+'\n');print(json.dumps(record,indent=2))
