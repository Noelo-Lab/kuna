from pathlib import Path
import json,subprocess,hashlib
w=Path('/tmp/kuna-rust-pointer-index');base=Path('/home/mahaloz/kwt/angr-rust-store-conversions');cli=base/'decompiler/target/release/kuna';sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
source='''#include <stdint.h>
__attribute__((noinline)) int load32(int*p,long i){return p[i];}
__attribute__((noinline)) void store32(int*p,long i,int value){p[i]=value;}
__attribute__((noinline)) int*address32(int*p,unsigned long i){return(int*)((uintptr_t)p+i*4);}
__attribute__((noinline)) int*address_signed32(int*p,long i){return(int*)((uintptr_t)p+(unsigned long)i*4);}
__attribute__((noinline)) long load64(long*p,long i){return p[i];}
__attribute__((noinline)) void store64(long*p,long i,long value){p[i]=value;}
struct Record{long before;int value;unsigned tag;};
__attribute__((noinline)) int field_load(struct Record*p,long i){return p[i].value;}
__attribute__((noinline)) void field_store(struct Record*p,long i,int value){p[i].value=value;}
__attribute__((noinline)) int zero_load(int*p){return *p;}
int main(void){return 0;}
'''
decl='''prototype load32 int4 load32(int4*p,int8 i)
prototype store32 void store32(int4*p,int8 i,int4 value)
prototype address32 int4*address32(int4*p,uint8 i)
prototype address_signed32 int4*address_signed32(int4*p,int8 i)
prototype load64 int8 load64(int8*p,int8 i)
prototype store64 void store64(int8*p,int8 i,int8 value)
prototype field_load int4 field_load(struct Record*p,int8 i)
prototype field_store void field_store(struct Record*p,int8 i,int4 value)
prototype zero_load int4 zero_load(int4*p)
'''
(w/'probe.c').write_text(source);(w/'probe.kuna').write_text(decl)
cmd=['cc','-O2','-g','-fno-inline','-fno-pie','-no-pie',str(w/'probe.c'),'-o',str(w/'probe')];subprocess.run(cmd,check=True,capture_output=True)
names=[line.split()[1]for line in decl.splitlines()];capture=[str(cli),'decompile-all',str(w/'probe'),'--language','rust','--json','--functions',','.join(names),'--assert','@'+str(w/'probe.kuna')];p=subprocess.run(capture,capture_output=True,check=True);(w/'parent.json').write_bytes(p.stdout);(w/'parent.log').write_bytes(p.stderr);d=json.loads(p.stdout);assert len(d['functions'])==len(names)and not any(f.get('error')for f in d['functions'])
body='#![allow(dead_code,unused_mut,unused_parens)]\n#[repr(C)]struct Record{before:i64,value:i32,tag:u32}\n'+'\n'.join(f['code']for f in d['functions'])+'\nfn main(){}\n';(w/'parent.rs').write_text(body)
compile=['rustc','--edition=2021','--error-format=json',str(w/'parent.rs'),'-o',str(w/'parent')];p=subprocess.run(compile,capture_output=True);(w/'parent-compile.jsonl').write_bytes(p.stderr);errors=[json.loads(line)for line in p.stderr.splitlines()if line.startswith(b'{')];errors=[e for e in errors if e.get('level')=='error'and e.get('code')]
assert p.returncode!=0 and errors
record={'scope':'Nine complete authored pointer indexing bodies, compile-only parent reproduction; no native or candidate proof yet.','parent_commit':subprocess.check_output(['git','rev-parse','HEAD'],cwd=base,text=True).strip(),'parent_cli_sha256':sha(cli),'fixture_sha256':sha(w/'probe'),'compile_exit':p.returncode,'errors':[{'code':e['code']['code'],'message':e['message']}for e in errors],'commands':[cmd,capture,compile]};(w/'parent-record.json').write_text(json.dumps(record,indent=2)+'\n');print(record['errors'])
