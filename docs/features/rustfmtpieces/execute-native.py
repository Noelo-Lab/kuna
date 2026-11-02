from pathlib import Path
import subprocess,os,json,hashlib,re
r=Path('/home/mahaloz/kwt/angr-rust-fmt-pieces');w=Path('/tmp/kuna-rust-fmt-pieces-f874');env=os.environ|{'KUNA_SPECS':str(r/'specs')}
values=list(range(256))+[0x7fff,0x8000,0xffff,0x7fffffff,0x80000000,0xffffffff,0x7fffffffffffffff,0x8000000000000000,0xffffffffffffffff]
names=['pieces_literal','pieces_plain','pieces_stderr','pieces_empty','pieces_u32','pieces_i64','pieces_pair','pieces_narrow','pieces_adjacent']
bindings={
 'pieces_u32':('a:u32','let v2=&a as *const u32;'),
 'pieces_i64':('a:i64','let v2=&a as *const i64;'),
 'pieces_pair':('a:i8,b:u64','let v3=&a as *const i8;let v6=&b as *const u64;'),
 'pieces_narrow':('a:u8,b:i16,c:u16,d:i32','let v5=&a as *const u8;let v8=&b as *const i16;let v10=&c as *const u16;let v12=&d as *const i32;'),
 'pieces_adjacent':('a:u64,b:i64','let v3=&a as *const u64;let v6=&b as *const i64;'),
}
calls=['pieces_u32(x as u32)','pieces_i64(x as i64)','pieces_pair(x as i8,y)','pieces_narrow(x as u8,y as i16,x as u16,y as i32)','pieces_adjacent(x,y as i64)']
body=''.join(name+'();\n'for name in names[:4])+'let values:[u64;'+str(len(values))+']=['+','.join(hex(v)for v in values)+'];\nfor (i,x) in values.into_iter().enumerate(){let y=x.rotate_left(17)^0xa5a55a5a8000ffff;\n'
for index,call in enumerate(calls):body+='println!("<'+str(index)+' {}>",i);eprintln!("<'+str(index)+' {}>",i);'+call+';println!("</'+str(index)+'>");eprintln!("</'+str(index)+'>");\n'
body+='}\n'
source=(r/'decompiler/crates/kuna-analysis/tests/fixtures/rust_fmt_pieces.rs').read_text().split('fn main()')[0]+'fn main(){'+body+'}\n'
(w/'native-oracle.rs').write_text(source)
def compile(tc,source,destination):
 p=subprocess.run(['rustc','+'+tc,'--edition=2021','--crate-name','pieces_execution','-C','opt-level=2','-C','debuginfo=1','-C','strip=debuginfo',str(source),'-o',str(destination)],capture_output=True,text=True)
 destination.with_suffix('.compile.log').write_text(p.stderr);assert p.returncode==0,p.stderr
def decompile(binary,name,off=False):
 cmd=[str(r/'decompiler/target/release/kuna'),'decompile',str(binary),name,'--language','rust']
 if off:cmd+=['--option','rustfmtpieces','off']
 return subprocess.check_output(cmd,cwd=r,env=env,text=True)
records=[]
for producer in ['1.89.0','1.90.0','nightly-2025-05-22']:
 native=w/f'native-{producer}';compile(producer,w/'native-oracle.rs',native)
 expected=subprocess.run([str(native)],capture_output=True,check=True)
 native.with_suffix('.stdout').write_bytes(expected.stdout);native.with_suffix('.stderr').write_bytes(expected.stderr)
 recovered=''
 for name in names:
  code=decompile(native,name);(w/f'{producer}-{name}-execution-body.rs').write_text(code)
  if name not in bindings:recovered+=code
  else:
   macros=[line.strip()for line in code.splitlines()if re.search(r'\b(?:e?print(?:ln)?!)\(',line)]
   assert len(macros)==1,(name,code)
   parameters,locals=bindings[name];recovered+=f'unsafe fn {name}({parameters}){{{locals}{macros[0]}}}\n'
 recovered+='fn main(){unsafe{'+body+'}}\n';path=w/f'recovered-{producer}-execution.rs';path.write_text(recovered)
 executors=[]
 for executor in ['1.90.0','1.93.1','1.99.1']:
  binary=w/f'recovered-{producer}-{executor}';compile(executor,path,binary)
  actual=subprocess.run([str(binary)],capture_output=True,check=True)
  assert actual.stdout==expected.stdout and actual.stderr==expected.stderr,(producer,executor)
  executors.append({'compiler':executor,'stdout_sha256':hashlib.sha256(actual.stdout).hexdigest(),'stderr_sha256':hashlib.sha256(actual.stderr).hexdigest(),'native_equal':True})
 records.append({'producer':producer,'native_sha256':hashlib.sha256(native.read_bytes()).hexdigest(),'values':len(values),'primitive_calls':len(values)*len(calls),'literal_calls':4,'executors':executors})
 print('native matched',producer,flush=True)
controls=[]
for producer in ['nightly-2025-05-21','1.93.1','1.99.1']:
 binary=w/f'control-{producer}';compile(producer,r/'decompiler/crates/kuna-analysis/tests/fixtures/rust_fmt_pieces.rs',binary)
 for name in ['pieces_literal','pieces_u32','pieces_flags']:
  on,off=decompile(binary,name),decompile(binary,name,True);assert on==off,(producer,name)
  controls.append({'producer':producer,'function':name,'unchanged':True})
print('unsupported producer controls unchanged',len(controls),flush=True)
(w/'native-execution.json').write_text(json.dumps({'producers':records,'controls':controls,'scope':'Unchanged entire literal bodies; unchanged primitive macro expressions with explicit typed input bindings; complete primitive descriptor bodies are not claimed recompilable.'},indent=2)+'\n')
