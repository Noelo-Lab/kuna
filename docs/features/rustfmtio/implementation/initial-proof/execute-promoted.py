from pathlib import Path
import json,re,subprocess,socket,hashlib
w=Path('/tmp/kuna-rust-io-proposal');r=Path('/home/mahaloz/kwt/angr-rust-io-proposal')
primitive=re.compile(r'&\*\(\((\w+) as \*const _\) as \*const (u32|i64)\)')
proof=json.loads((w/'initial-image-proofs.json').read_text());assert proof['all_exact_writer_profiles_verified']
sites=[];recovered=''
for label in ['promoted','reference']:
 records=json.loads((w/f'{label}-after.json').read_text())['functions']
 ast=json.loads((w/f'{label}-ast.json').read_text())
 for address,record in ast.items():
  body=next(x for x in records if x['address']==int(address,16));assert len(record['changes'])==1
  after=record['changes'][0]['after'];assert after['receiver']=='PromotedInner' and len(after['parameters'])==1
  name=body['name'].split('::')[-1];name='reference_'+name if label=='reference'else name
  line=next(line.strip()for line in body['code'].splitlines()if 'use std::io::Write as _;'in line)
  expression=line.removeprefix('return ').split('; //',1)[0].removesuffix(';')
  writer=after['writer'];assert writer in ['a0','v1']
  ty=re.search(r'(?:mut )'+writer+r': ([^;,\n]+)',body['code'])[1].strip();assert ty=='*mut i64',ty
  bindings=f'let {writer}=std::mem::transmute_copy::<std::io::Stdout,*mut ()>(&**out) as {ty};'
  args=primitive.findall(expression);assert len(args)==1
  pointer,primitive_type=args[0];bindings+=f'let {pointer}=&value as *const {primitive_type};'
  recovered+=f'fn {name}(out:&mut &std::io::Stdout,value:{primitive_type})->std::io::Result<()>{{unsafe{{{bindings}let raw={expression};if raw==0{{Ok(())}}else{{Err(std::mem::transmute::<*mut (),std::io::Error>(raw as usize as *mut ()))}}}}}}\n'
  sites.append({'fixture':label,'function':name,'expression':expression,'bindings':bindings,'unchanged_emitted_expression':True,'writer_binding_basis':'Original Stdout inner pointer from the independently verified promoted-receiver profile.'})
assert len(sites)==5
original=(r/'decompiler/crates/kuna-analysis/tests/fixtures/rust_fmt_io_promoted.rs').read_text().split('fn main() {',1)[0]
original+='fn reference_write_out(out:&mut &std::io::Stdout,value:i64)->std::io::Result<()>{out.write_fmt(format_args!("v: {}\\n",value))}\n'.replace('\\n','\n')
# Keep the newline as a Rust string escape, rather than a line break in Python's generated source.
original=original.replace('format_args!("v: {}\n",value)', 'format_args!("v: {}\\n",value)')
observe=(w/'native-oracle.rs').read_text().split('fn observe(',1)[1].split('fn main() {',1)[0]
observe='fn observe('+observe
unsigned=list(range(256))+[256,257,65535,65536,2**31-1,2**31,2**32-1]
signed=list(range(-128,128))+[-2**63,2**63-1,-257,256,-65537,65536,2**32,-2**32]
main=r'''fn main(){let stdout=std::io::stdout();let mut out=&stdout;
 for pass in 0..3 {
  if pass==1{let _=std::io::Write::write_all(&mut out,b"prefix:\xc3\xa9");}
  for value in [UNSIGNED] {
   if pass==2{let _=std::io::Write::flush(&mut out);}
   observe("value",write_value(&mut out,value));observe("hex",write_hex(&mut out,value));
   observe("binary",write_binary(&mut out,value));observe("debug",write_debug(&mut out,value));
  }
  for value in [SIGNED] {observe("reference",reference_write_out(&mut out,value));}
 }
}'''
main=main.replace('UNSIGNED',','.join(str(x)+'u32'for x in unsigned)).replace('SIGNED',','.join(str(x)+'i64'for x in signed))
(w/'promoted-native-oracle.rs').write_text(original+observe+main)
(w/'promoted-recovered-execution.rs').write_text(recovered+observe+main)
def build(source,version,name):
 binary=w/name;p=subprocess.run(['rustc','+'+version,'--edition=2021','--crate-name','promoted_io_probe',str(w/source),'-O','-o',str(binary)],capture_output=True)
 (w/(name+'.compile.log')).write_bytes(p.stderr);assert p.returncode==0,p.stderr.decode();return binary
def run(binary,mode):
 if mode=='pipe':return subprocess.run([str(binary)],capture_output=True,check=True)
 if mode=='broken-pipe':
  reader,writer=socket.socketpair();reader.close()
  with writer:return subprocess.run([str(binary)],stdout=writer,stderr=subprocess.PIPE,check=True)
 with open('/dev/full','wb',buffering=0)as output:return subprocess.run([str(binary)],stdout=output,stderr=subprocess.PIPE,check=True)
count=3*(len(unsigned)*4+len(signed));assert count==3948
native=build('promoted-native-oracle.rs','1.93.1','promoted-native-execution');expected={}
for mode in ['pipe','broken-pipe','device-full']:
 p=run(native,mode);assert len(p.stderr.splitlines())==count
 expected[mode]=p;(w/f'promoted-native-{mode}.stdout').write_bytes(p.stdout or b'');(w/f'promoted-native-{mode}.stderr').write_bytes(p.stderr)
results=[]
for version in ['1.90.0','1.93.1','1.99.1']:
 binary=build('promoted-recovered-execution.rs',version,'promoted-recovered-'+version)
 for mode,p in expected.items():
  actual=run(binary,mode);assert actual.stdout==p.stdout and actual.stderr==p.stderr,(version,mode)
  results.append({'compiler':version,'mode':mode,'calls':count,'output_bytes_and_owned_result_words_match':True,'stdout_sha256':hashlib.sha256(actual.stdout or b'').hexdigest(),'stderr_sha256':hashlib.sha256(actual.stderr).hexdigest()})
  print(version,mode,count,'PROMOTED IO COMPARISONS PASS',flush=True)
(w/'promoted-native-execution.json').write_text(json.dumps({'sites':sites,'results':results,'total_comparisons':count*len(results),'original_fixture_function_comparisons':3*len(unsigned)*4*len(results),'reference_oracle_comparisons':3*len(signed)*len(results),'reference_oracle_scope':'The reference oracle is reconstructed from the independently decoded i64 Display descriptor, newline and verified Stdout method; it is not execution of the original reference executable. Four promoted-fixture oracles use their original Rust function source.','complete_function_recompilation_claimed':False},indent=2)+'\n')
