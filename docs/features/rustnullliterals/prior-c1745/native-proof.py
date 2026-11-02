from pathlib import Path
import datetime,hashlib,json,os,subprocess
w=Path('/tmp/kuna-rust-null-literals-c1745');r=Path('/home/mahaloz/kwt/angr-rust-null-literals');base=Path('/home/mahaloz/kwt/angr-base-c1745');out=w/'native-proof';out.mkdir(exist_ok=False);sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();fixture=r/'decompiler/crates/kuna-analysis/tests/fixtures/rust_null_pointer_x86_64';facts=fixture.with_name('rust_null_pointer.kuna');names=['null_pointer','maybe_pointer','pointer_is_null','clear_pointer'];inputs=[0,1,2,41,42,43,127,255,0x7fffffff,0x80000000,0xffffffff,0x100000000,0x7fffffffffffffff,0x8000000000000000,0xffffffffffffffff]
harness='''
fn main() {
 let choice:u64=std::env::args().nth(1).unwrap().parse().unwrap();
 let mut byte=17u8; let pointer=&mut byte as *mut u8; let mut slot=pointer;
 unsafe { let null=null_pointer(); let selected=maybe_pointer(choice,pointer); clear_pointer(&mut slot);
 println!("{} {} {} {} {}",null.is_null(),selected==pointer,slot.is_null(),pointer_is_null(null),pointer_is_null(pointer)); }
}
'''
records={};builds=[]
for arm,root in [('parent',base),('candidate',r)]:
 for lang in ['c','rust','go']:
  cmd=[str(root/'decompiler/target/release/kuna'),'decompile-all',str(fixture),'--json','--language',lang,'--functions',','.join(names),'--assert','@'+str(facts)]
  p=subprocess.run(cmd,cwd=r,env=os.environ|{'KUNA_SPECS':str(r/'specs')},capture_output=True,text=True,check=True);(out/f'{arm}-{lang}.json').write_text(p.stdout);(out/f'{arm}-{lang}.log').write_text(p.stderr);doc=json.loads(p.stdout);assert len(doc['functions'])==4 and all(f['address']and not f['error']for f in doc['functions']);records[arm,lang]=doc['functions']
 for lang in ['rust']:
  bodies='\n'.join(f['code']for f in records[arm,lang]);token='core::ptr::null_mut__'if arm=='parent'else'core::ptr::null_mut()';assert bodies.count(token)==4;(out/f'{arm}.rs').write_text(bodies+harness)
  for opt in (['0']if arm=='parent'else['0','2']):
   cmd=['rustc','--edition=2021','--crate-name','recovered_null_pointers','-C',f'opt-level={opt}',str(out/f'{arm}.rs'),'-o',str(out/f'{arm}-O{opt}')];p=subprocess.run(cmd,capture_output=True,text=True);(out/f'{arm}-O{opt}-build.log').write_text(p.stdout+p.stderr);assert (p.returncode!=0 and 'E0425'in p.stderr)if arm=='parent'else p.returncode==0;builds.append({'arm':arm,'opt':opt,'exit':p.returncode})
for lang in ['c','go']:assert records['parent',lang]==records['candidate',lang],lang
for before,after in zip(records['parent','rust'],records['candidate','rust']):
 changed=dict(before);changed['code']=changed['code'].replace('core::ptr::null_mut__','core::ptr::null_mut()');assert changed==after,(before['name'],'unexpected whole record change')
observations=[]
for choice in inputs:
 original=subprocess.check_output([str(fixture),str(choice)],text=True).strip();assert original==f'true {str(choice==42).lower()} true 1 0'
 for opt in ['0','2']:
  actual=subprocess.check_output([str(out/f'candidate-O{opt}'),str(choice)],text=True).strip();assert actual==original;observations.append({'choice':choice,'optimization':opt,'original':original,'candidate':actual})
record={'at':datetime.datetime.now(datetime.timezone.utc).isoformat(),'cli_sha256':sha(r/'decompiler/target/release/kuna'),'parent_cli_sha256':sha(base/'decompiler/target/release/kuna'),'fixture_sha256':sha(fixture),'functions':4,'complete_bodies_unchanged':True,'native_values':150,'observations':observations,'builds':builds,'c_go_records_identical':True,'rust_only_constructor_spelling_changes':True};(out/'record.json').write_text(json.dumps(record,indent=2)+'\n');print('Rust null pointer proof PASS:150 native values;four complete bodies;parent E0425;C/Go identical.',flush=True)
