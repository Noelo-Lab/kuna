from pathlib import Path
import subprocess,json,re,os,hashlib
w=Path('/tmp/kuna-rust-fmt-write');r=Path('/home/mahaloz/kwt/angr-rust-fmt-write')
records={x['name']:x for x in json.loads((w/'native-after.json').read_text())['functions']}
primitive=re.compile(r'&\*\(\((\w+) as \*const _\) as \*const (u32|i64)\)')
strings=re.compile(r'from_raw_parts\((\w+) as \*const u8,(\w+) as usize\)')
functions=[('write_unsigned','out:&mut String,value:u32'),('write_signed','out:&mut String,value:i64'),('write_flags','out:&mut String,value:u32'),('write_literal','out:&mut String'),('write_empty','out:&mut String'),('write_string','out:&mut String,text:&str'),('write_pair','out:&mut String,value:u32,text:&str')]
recovered='';sites=[]
for name,parameters in functions:
 body=records[name]['code'];lines=[line.strip()for line in body.splitlines()if'use std::fmt::Write as _;'in line];assert len(lines)==1
 line=lines[0];assert line.startswith('if ')and line.endswith(" == b'\\0' {");expression=line[3:-len(" == b'\\0' {")]
 writer_type=re.search(r'fn '+re.escape(name)+r'\(mut a0: (u64|\*mut \(\))',body)[1]
 assert writer_type in ['u64','*mut ()'],writer_type
 bindings='let a0=out as *mut String as '+writer_type+';'
 for pointer,ty in primitive.findall(expression):bindings+=f'let {pointer}=&value as *const {ty};'
 for data,length in strings.findall(expression):bindings+=f'let {data}=text.as_ptr() as u64;let {length}=text.len() as u64;'
 recovered+=f'fn {name}({parameters})->u8{{unsafe{{{bindings}{expression}}}}}\n'
 sites.append({'name':name,'expression':expression,'bindings':bindings,'unchanged_emitted_expression':True})
texts=[]
for text in ['', 'é🙂🦀', 'a\0b\n{}', '中', 'e\u0301']+[chr(i)for i in range(128)]+[a+b for a in ['é','🙂','🦀','中','\0','\n','{','}']for b in ['a','é','🙂','\0','\n']]:
 if text not in texts:texts.append(text)
texts=texts[:156];assert len(texts)==156
unsigned=list(range(256))+[256,257,65535,65536,2**31-1,2**31,2**32-1]
signed=list(range(-128,128))+[-2**63,2**63-1,-257,256,-65537,65536,2**32,-2**32]
values=[0,1,2,23,255,256,65535,2**31,2**32-1]
main='''fn emit(name:&str,code:u8,out:String){assert_eq!(code,0);println!("{} {} {:?}",name,code,out);}
fn main(){let texts:Vec<String>=vec![TEXTS];let prefixes=["","prefix:é🙂\\0","long-long-long-existing:"];
for prefix in prefixes {for value in [UNSIGNED]{for kind in 0..2{let mut out=String::with_capacity(1024);out.push_str(prefix);let code=if kind==0{write_unsigned(&mut out,value)STATUS}else{write_flags(&mut out,value)STATUS};emit(if kind==0{"unsigned"}else{"flags"},code,out);}}
for value in [SIGNED]{let mut out=String::from(prefix);let code=write_signed(&mut out,value)STATUS;emit("signed",code,out);}
for text in &texts{let mut out=String::from(prefix);let code=write_string(&mut out,text)STATUS;emit("string",code,out);for value in [VALUES]{let mut out=String::from(prefix);let code=write_pair(&mut out,value,text)STATUS;emit("pair",code,out);}}
for kind in 0..2{let mut out=String::from(prefix);let code=if kind==0{write_literal(&mut out)STATUS}else{write_empty(&mut out)STATUS};emit(if kind==0{"literal"}else{"empty"},code,out);}}}
'''
main=main.replace('TEXTS',','.join('String::from_utf8(vec!['+','.join(map(str,t.encode()))+']).unwrap()'for t in texts)).replace('UNSIGNED',','.join(str(x)+'u32'for x in unsigned)).replace('SIGNED',','.join(str(x)+'i64'for x in signed)).replace('VALUES',','.join(str(x)+'u32'for x in values))
original=(r/'decompiler/crates/kuna-analysis/tests/fixtures/rust_fmt_write.rs').read_text();original=original[:original.index('fn main() {')]
(w/'native-oracle.rs').write_text(original+main.replace('STATUS','.is_err() as u8'))
(w/'recovered-execution.rs').write_text(recovered+main.replace('STATUS',''))
def build(source,version,name):
 exe=w/name
 p=subprocess.run(['rustc','+'+version,'--edition=2021','--crate-name','write_oracle',str(w/source),'-O','-o',str(exe)],capture_output=True)
 (w/(name+'.compile.log')).write_bytes(p.stderr);assert p.returncode==0,p.stderr.decode()
 return exe
native=build('native-oracle.rs','1.93.1','native-exhaustive-1931');expected=subprocess.run([str(native)],capture_output=True,check=True)
(w/'native-exhaustive.stdout').write_bytes(expected.stdout);assert not expected.stderr
count=len(expected.stdout.splitlines());assert count==3*(len(unsigned)*2+len(signed)+len(texts)*(len(values)+1)+2)
results=[]
for version in ['1.90.0','1.93.1','1.99.1']:
 exe=build('recovered-execution.rs',version,'recovered-'+version);actual=subprocess.run([str(exe)],capture_output=True,check=True)
 assert actual.stdout==expected.stdout and actual.stderr==expected.stderr,version
 results.append({'version':version,'calls':count,'stdout_sha256':hashlib.sha256(actual.stdout).hexdigest(),'original_string_bytes_and_result_bytes_match':True})
 print(version,count,'native write comparisons PASS',flush=True)
(w/'native-execution.json').write_text(json.dumps({'native_producer':'1.93.1','texts':len(texts),'u32_values':len(unsigned),'i64_values':len(signed),'mixed_u32_values':len(values),'prefixes':3,'total_comparisons':count*len(results),'sites':sites,'results':results,'complete_function_recompilation_claimed':False,'explicit_typed_input_bindings':True},indent=2)+'\n')
