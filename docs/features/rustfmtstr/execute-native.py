from pathlib import Path
import subprocess,os,json,hashlib,re
w=Path('/tmp/kuna-rust-fmt-str');r=Path('/home/mahaloz/kwt/angr-rust-fmt-str');base=Path('/home/mahaloz/kwt/angr-rust-fmt-pieces')
fixture=r/'decompiler/crates/kuna-analysis/tests/fixtures/rust_fmt_str.rs'
prefix=fixture.read_text().split('fn main()')[0]
main=r'''fn main() {
    std::panic::set_hook(Box::new(|_|{}));
    let mut strings:Vec<String>=(0u8..=127).map(|c|String::from(c as char)).collect();
    strings.extend(["", "text", "é🙂🦀", "a\0b\n{}\\\"", "\r\t", "e\u{301}", "👩\u{200d}💻", "مرحبا", "日本語"].into_iter().map(String::from));
    for c in [0x80,0x7ff,0x800,0xd7ff,0xe000,0xffff,0x10000,0x10ffff] {strings.push(char::from_u32(c).unwrap().to_string());}
    for size in [2,3,7,8,9,16,31,32,255,256,4096] {strings.push("é{}\0🦀".repeat(size));}
    let values=[0u32,1,7,127,255,256,0x7fffffff,0x80000000,u32::MAX];
    if std::env::args().nth(1).as_deref()==Some("panic") {
        for text in &strings {
            let error=std::panic::catch_unwind(||str_panic(text)).unwrap_err();
            println!("{:?}",error.downcast_ref::<String>().unwrap());
        }
        return;
    }
    for text in &strings {
        str_plain(text);str_line(text);str_stderr(text);str_flags(text);str_pair(text,"right é🦀\0\n");
        println!("{:?}",str_result(text));
        for &value in &values {str_stderr_pair(value,text);println!("{:?}",str_integer(value,text));}
    }
    if std::hint::black_box(false) {
        str_custom("custom");str_debug("debug");str_dynamic("dynamic",8);str_reference(&7);str_owned(&String::from("owned"));
    }
}
'''
source=w/'native-oracle.rs';source.write_text(prefix+main)
def build(version,source,binary,producer=False):
 command=['taskset','-c','0-78','rustc']+([]if version=='1.90.0'else['+'+version])+['--edition=2021','--crate-name','string_oracle']
 if producer:command+=['-C','opt-level=3','-C','lto=fat','-C','codegen-units=1','-C','debuginfo=1','-C','strip=debuginfo']
 p=subprocess.run(command+[str(source),'-o',str(binary)],capture_output=True,text=True)
 binary.with_suffix('.compile.log').write_text(p.stdout+p.stderr);assert p.returncode==0,p.stderr
native=w/'native-exhaustive-1931';build('1.93.1',source,native,True)
oracles={}
for mode in ['streams','panic']:
 result=subprocess.run([str(native)]+(['panic']if mode=='panic'else[]),capture_output=True,check=True)
 oracles[mode]=result
 for stream in ['stdout','stderr']:(w/f'native-{mode}.{stream}').write_bytes(getattr(result,stream))
string='let v1=text.as_ptr() as u64;let v3=text.len() as u64;'
mixed='let v3=&value as *const u32;let v2=text.as_ptr() as u64;let v4=text.len() as u64;'
rows=[('str_plain','text:&str',string,False),('str_line','text:&str',string,False),('str_stderr','text:&str',string,False),('str_stderr_pair','value:u32,text:&str',mixed,False),('str_flags','text:&str',string,False),('str_pair','left:&str,right:&str','let v1=left.as_ptr() as u64;let v4=left.len() as u64;let v2=right.as_ptr() as u64;let v5=right.len() as u64;',False),('str_integer','value:u32,text:&str',mixed,True),('str_result','text:&str',string,True),('str_panic','text:&str',string,False)]
code='';captures=[]
for name,params,bindings,result in rows:
 outputs={}
 for side,tree in [('before',base),('after',r)]:
  body=subprocess.check_output(['taskset','-c','0-78',str(tree/'decompiler/target/release/kuna'),'decompile',str(native),name,'--language','rust'],env=os.environ|{'KUNA_SPECS':str(r/'specs')},text=True)
  outputs[side]=body;(w/f'native-{name}-{side}.rs').write_text(body)
 lines=[line for line in outputs['after'].splitlines()if 'from_utf8_unchecked'in line];assert len(lines)==1,(name,outputs)
 assert 'from_utf8_unchecked'not in outputs['before']
 returns=' -> String'if result else''
 before='let mut destination=std::mem::MaybeUninit::<String>::uninit();let a0=destination.as_mut_ptr() as u64;'if result else''
 after='destination.assume_init()'if result else''
 code+=f'fn {name}({params}){returns}{{unsafe{{{bindings}{before}\n{lines[0]}\n{after}\n}}}}\n'
 captures.append({'name':name,'macro_expression_unchanged':True,'explicit_input_bindings':bindings,'hidden_destination':result})
code+=main[:main.index('    if std::hint::black_box(false)')]+'}\n'
recovered=w/'recovered-execution.rs';recovered.write_text(code)
records=[]
for version in ['1.90.0','1.93.1','1.99.1']:
 binary=w/f'recovered-{version}';build(version,recovered,binary)
 for mode in ['streams','panic']:
  actual=subprocess.run([str(binary)]+(['panic']if mode=='panic'else[]),capture_output=True,check=True)
  assert actual.stdout==oracles[mode].stdout and actual.stderr==oracles[mode].stderr,(version,mode)
 records.append({'executor':version,'stream_calls':156*24,'panic_payloads':156,'stdout_stderr_exact':True})
 print(version,'all native streams and panic payloads exact',flush=True)
controls=[]
for name in ['str_custom','str_debug','str_dynamic','str_reference','str_owned']:
 outputs=[subprocess.check_output(['taskset','-c','0-78',str(tree/'decompiler/target/release/kuna'),'decompile',str(native),name,'--language','rust'],env=os.environ|{'KUNA_SPECS':str(r/'specs')})for tree in [base,r]]
 assert outputs[0]==outputs[1] and b'from_utf8_unchecked'not in outputs[1];controls.append(name)
for version in ['1.90.0','1.99.1']:
 binary=w/f'native-control-{version}';build(version,source,binary,True)
 for name in ['str_plain','str_integer','str_panic']:
  outputs=[subprocess.check_output(['taskset','-c','0-78',str(tree/'decompiler/target/release/kuna'),'decompile',str(binary),name,'--language','rust'],env=os.environ|{'KUNA_SPECS':str(r/'specs')})for tree in [base,r]]
  assert outputs[0]==outputs[1] and b'from_utf8_unchecked'not in outputs[1];controls.append(version+':'+name)
(w/'native-execution.json').write_text(json.dumps({'producer':'1.93.1','input_sha256':hashlib.sha256(native.read_bytes()).hexdigest(),'strings':156,'integer_values':9,'calls_per_string':24,'stream_calls_per_executor':3744,'panic_payloads_per_executor':156,'executions':records,'unchanged_macros':captures,'full_body_compilation_claimed':False,'original_panic_location_equivalence_claimed':False,'refusal_controls':controls,'streams':{mode:{stream:{'bytes':len(getattr(value,stream)),'sha256':hashlib.sha256(getattr(value,stream)).hexdigest()}for stream in ['stdout','stderr']}for mode,value in oracles.items()}},indent=2)+'\n')
