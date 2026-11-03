from pathlib import Path
import json,subprocess,hashlib,os
w=Path('/tmp/kuna-rust-store-research/width-native');w.mkdir(exist_ok=False)
roots={'parent':Path('/home/mahaloz/kwt/angr-rust-integer-conversions'),'candidate':Path('/home/mahaloz/kwt/angr-rust-store-conversions')};sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
ctypes={(8,True):'signed char',(8,False):'unsigned char',(16,True):'short',(16,False):'unsigned short',(32,True):'int',(32,False):'unsigned int',(64,True):'long',(64,False):'unsigned long'}
values=[0,1,2,127,128,255,256,32767,32768,65535,65536,2147483647,2147483648,4294967295,4294967296,9223372036854775807,9223372036854775808,18446744073709551615]
source=[];decl=[];cases=[]
for sb,ss in ctypes:
 for tb,ts in ctypes:
  if sb==tb:continue
  name=('i'if ss else'u')+str(sb)+'_to_'+('i'if ts else'u')+str(tb);st=ctypes[sb,ss];tt=ctypes[tb,ts];source.append('__attribute__((noinline,noclone)) void '+name+'('+tt+'*p,'+st+' value){*p=('+tt+')value;}');decl.append('prototype '+name+' void '+name+'('+('int'if ts else'uint')+str(tb//8)+' *p,'+('int'if ss else'uint')+str(sb//8)+' value)');cases.append((name,sb,ss,tb,ts))
assert len(cases)==48
(w/'fixture.c').write_text('\n'.join(source)+'\nint main(void){return 0;}\n');(w/'assertions.kuna').write_text('\n'.join(decl)+'\n')
c='#include <stdio.h>\n#include <string.h>\n#define main fixture_main\n#include "fixture.c"\n#undef main\nvoid dump(const char*n,void*v,size_t z){printf("%s ",n);for(size_t k=0;k<z;k++)printf("%02x",((unsigned char*)v)[k]);puts("");}\nint main(void){unsigned long values[]={'+','.join(str(v)+'UL'for v in values)+'};for(unsigned i=0;i<18;i++){unsigned long a=values[i];\n'
r='#![allow(dead_code,unused_mut,unused_parens,unused_variables)]\n#[repr(C)]struct E<T>{before:[u64;2],value:T,after:[u64;2]}\nunsafe fn dump<T>(n:&str,p:*const T){print!("{} ",n);for v in core::slice::from_raw_parts(p as*const u8,core::mem::size_of::<T>()){print!("{:02x}",v);}println!();}\n/* BODIES */\nfn main(){unsafe{for a in ['+','.join(str(v)+'u64'for v in values)+']{\n'
for name,sb,ss,tb,ts in cases:
 ct=ctypes[tb,ts];cs=ctypes[sb,ss];rt=('i'if ts else'u')+str(tb);rs=('i'if ss else'u')+str(sb)
 c+='{struct {unsigned long before[2];'+ct+' value;unsigned long after[2];}s;memset(&s,0xa5,sizeof s);'+name+'(&s.value,('+cs+')a);dump("'+name+'",&s,sizeof s);}\n'
 r+='{let mut s=core::mem::MaybeUninit::<E<'+rt+'>>::uninit();core::ptr::write_bytes(s.as_mut_ptr(),0xa5,1);'+name+'(core::ptr::addr_of_mut!((*s.as_mut_ptr()).value),a as '+rs+');dump("'+name+'",s.as_ptr());}\n'
c+='}return 0;}\n';r+='}}}\n';(w/'reference.c').write_text(c);(w/'harness.rs').write_text(r)
records=[]
def cmd(args,stem,required=True):
 p=subprocess.run(args,capture_output=True);(w/(stem+'.stdout')).write_bytes(p.stdout);(w/(stem+'.stderr')).write_bytes(p.stderr);records.append({'command':args,'stem':stem,'exit':p.returncode});assert not required or p.returncode==0,(stem,p.stderr.decode()[:2000]);return p
cmd(['cc','-O2','-g','-fno-inline','-fno-pie','-no-pie',str(w/'fixture.c'),'-o',str(w/'fixture')],'fixture-build')
for opt in ['0','2']:
 cmd(['cc','-O'+opt,'-fwrapv',str(w/'reference.c'),'-o',str(w/('reference-'+opt))],'reference-'+opt+'-build');cmd([str(w/('reference-'+opt))],'reference-'+opt)
assert (w/'reference-0.stdout').read_bytes()==(w/'reference-2.stdout').read_bytes()
summary=[]
for arm,root in roots.items():
 cli=root/'decompiler/target/release/kuna';digest=sha(cli);p=cmd([str(cli),'decompile-all',str(w/'fixture'),'--language','rust','--json','--functions',','.join(row[0]for row in cases),'--assert','@'+str(w/'assertions.kuna')],arm+'-capture');d=json.loads(p.stdout);assert len(d['functions'])==48 and not any(f.get('error')for f in d['functions']);(w/(arm+'.rs')).write_text(r.replace('/* BODIES */','\n'.join(f['code']for f in d['functions'])));outcomes=[]
 for opt in ['0','2']:
  p=cmd(['rustc','--edition=2021','-C','opt-level='+opt,'-C','overflow-checks=off',str(w/(arm+'.rs')),'-o',str(w/(arm+'-'+opt))],arm+'-'+opt+'-build',False)
  row={'optimization':opt,'compile_exit':p.returncode}
  if p.returncode==0:
   p=cmd([str(w/(arm+'-'+opt))],arm+'-'+opt);row['matches_reference']=p.stdout==(w/('reference-'+opt+'.stdout')).read_bytes();assert row['matches_reference']
  outcomes.append(row)
 assert sha(cli)==digest;summary.append({'arm':arm,'cli_sha256':digest,'outcomes':outcomes})
(w/'record.json').write_text(json.dumps({'scope':'48 ordinary authored widening/narrowing signed/unsigned stores, complete emitted bodies, byte guards and 18 values at O0/O2. Does not prove arbitrary inconsistent inferred types or unknown128-bit stores.','complete_bodies':48,'body_rewrites':False,'records':summary,'commands':records},indent=2)+'\n');print(summary,flush=True)
