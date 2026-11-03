from common import *
x=w/'pointer-cast-native';x.mkdir(exist_ok=True);sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest();names=['unsigned_address','byte_address']
(x/'probe.c').write_text('''#include <stdint.h>
__attribute__((noinline)) uint64_t* unsigned_address(int64_t*p,uint64_t i){return(uint64_t*)((uintptr_t)p+i*8);}
__attribute__((noinline)) uint8_t* byte_address(int64_t*p,uint64_t i){return(uint8_t*)((uintptr_t)p+i*8);}
#ifndef OMIT_MAIN
int main(void){return 0;}
#endif
''');(x/'probe.kuna').write_text('prototype unsigned_address uint8*unsigned_address(int8*p,uint8 i)\nprototype byte_address uint1*byte_address(int8*p,uint8 i)\n')
values=['0','1','2','7','0x7fffffffffffffff','0x8000000000000000','0xffffffffffffffff','0x2000000000000000','0x4000000000000000','0xaaaaaaaa55555555']
(x/'harness.c').write_text('#include <stdio.h>\n#include <inttypes.h>\n#define OMIT_MAIN\n#include "probe.c"\nint main(void){int64_t a[4]={0};uint64_t values[]={'+','.join('UINT64_C('+v+')'for v in values)+'};for(unsigned i=0;i<10;i++){printf("%016" PRIx64 " %016" PRIx64 "\\n",(uint64_t)((uintptr_t)unsigned_address(a,values[i])-(uintptr_t)a),(uint64_t)((uintptr_t)byte_address(a,values[i])-(uintptr_t)a));}return 0;}\n')
cmd=['cc','-O2','-g','-fno-pie','-no-pie',str(x/'probe.c'),'-o',str(x/'probe')];subprocess.run(cmd,capture_output=True,check=True);records=[{'kind':'fixture','command':cmd}]
for opt in ['0','2']:
 cmd=['cc','-O'+opt,'-fwrapv',str(x/'harness.c'),'-o',str(x/('reference-'+opt))];subprocess.run(cmd,capture_output=True,check=True);p=subprocess.run([str(x/('reference-'+opt))],capture_output=True,check=True);(x/('reference-'+opt+'.stdout')).write_bytes(p.stdout);records.append({'kind':'reference','optimization':opt,'command':cmd,'output_sha256':sha(x/('reference-'+opt+'.stdout'))})
assert(x/'reference-0.stdout').read_bytes()==(x/'reference-2.stdout').read_bytes()
for arm,root in [('parent',Path('/home/mahaloz/kwt/angr-rust-store-conversions')),('candidate',r)]:
 cmd=[str(root/'decompiler/target/release/kuna'),'decompile-all',str(x/'probe'),'--language','rust','--json','--functions',','.join(names),'--assert','@'+str(x/'probe.kuna')];p=subprocess.run(cmd,capture_output=True,check=True,env=env|{'KUNA_SPECS':str(root/'specs')});(x/(arm+'.json')).write_bytes(p.stdout);d=json.loads(p.stdout);assert len(d['functions'])==2 and all(not z.get('error')for z in d['functions']);body='\n'.join(z['code']for z in d['functions']);(x/(arm+'.rs')).write_text(body+'\nfn main(){let mut a=[0i64;4];for i in ['+','.join(v+'u64'for v in values)+']{unsafe{let p=a.as_mut_ptr();println!("{:016x} {:016x}",(unsigned_address(p,i) as u64).wrapping_sub(p as u64),(byte_address(p,i) as u64).wrapping_sub(p as u64));}}}\n');print(arm,body,flush=True)
 for opt in ['0','2']:
  compile=['rustc','--edition=2021','--error-format=json','-C','opt-level='+opt,'-C','overflow-checks=off',str(x/(arm+'.rs')),'-o',str(x/(arm+'-'+opt))];p=subprocess.run(compile,capture_output=True);(x/(arm+'-'+opt+'-build.jsonl')).write_bytes(p.stderr);rec={'arm':arm,'optimization':opt,'capture_command':cmd,'compile_command':compile,'compile_exit':p.returncode,'cli_sha256':sha(root/'decompiler/target/release/kuna')};records.append(rec)
  if arm=='parent':assert p.returncode!=0;continue
  assert p.returncode==0,p.stderr.decode();p=subprocess.run([str(x/(arm+'-'+opt))],capture_output=True,check=True);(x/(arm+'-'+opt+'.stdout')).write_bytes(p.stdout);assert p.stdout==(x/('reference-'+opt+'.stdout')).read_bytes();rec.update(matches_reference=True,comparisons=20)
record={'status':'PASS','complete_bodies':2,'comparisons':40,'body_rewrites':False,'registered':False,'scope':'Supplementary pointer-result conversions and modular address bits, no invalid dereferences; C/Rust O0/O2.','records':records,'files':{p.name:sha(p)for p in x.iterdir()if p.suffix in ['.rs','.c','.kuna','.json','.jsonl','.stdout']}};(x/'record.json').write_text(json.dumps(record,indent=2)+'\n');print('Supplementary pointer cast native40 PASS')
