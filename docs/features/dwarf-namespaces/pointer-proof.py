from pathlib import Path
import os,hashlib,json,subprocess,time

w=Path('/tmp/kuna-dwarf-namespace-p9');r=Path('/home/mahaloz/kwt/angr-dwarf-namespace');base=Path('/home/mahaloz/kwt/angr-base-1d5f')
while not(w/'build.json').exists():time.sleep(1)
out=w/'pointer-proof';out.mkdir(exist_ok=False);build=json.loads((w/'build.json').read_text());sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
env=os.environ|{'KUNA_SPECS':str(r/'specs'),'SLEIGHHOME':str(r/'specs')};fixture=r/'decompiler/crates/kuna-analysis/tests/fixtures/dwarf_function_pointer_x86_64';inputs=[0,1,7,12345,0x7fffffff,0x80000000,0xffffffd7,0xfffffff5,0xffffffff];records=[]
for arm,root in [('parent',base),('candidate',r)]:
 cli=root/'decompiler/target/release/kuna';assert sha(cli)==build['parent_cli_sha256'if arm=='parent'else'new_cli_sha256']
 cmd=[str(cli),'decompile-all',str(fixture),'--functions','target,left::target,get_target,main','--language','c','--json'];p=subprocess.run(cmd,cwd=r,env=env,capture_output=True,text=True,check=True);(out/f'{arm}.json').write_text(p.stdout);(out/f'{arm}.log').write_text(p.stderr);doc=json.loads(p.stdout);assert len(doc['functions'])==3 and all(not f.get('error')for f in doc['functions'])
 bodies='\n'.join(f['code']for f in doc['functions']);target='left::target'if arm=='candidate'else'target';assert f'return (unsigned long){target};'in bodies
 prefix='#include <cstdio>\n#include <cstdlib>\n#include <cstdint>\nnamespace left { unsigned int target(unsigned int); }\n#define main recovered_main\n'
 driver=f'\n#undef main\nint main(int argc, char **argv) {{ if (get_target() != reinterpret_cast<uintptr_t>(&{target})) return 91; return recovered_main(argc, argv); }}\n'
 source=out/f'{arm}.cpp';source.write_text(prefix+bodies+driver)
 for compiler in ['clang++','g++']:
  for level in ['0','2']:
   binary=out/f'{arm}-{compiler}-O{level}';p=subprocess.run([compiler,'-std=c++17',f'-O{level}',str(source),'-o',str(binary)],capture_output=True,text=True);(out/f'{arm}-{compiler}-O{level}.compile.log').write_text(p.stdout+p.stderr);assert p.returncode==0
   samples=[]
   for value in inputs:
    native=subprocess.check_output([str(fixture),str(value)],text=True);actual=subprocess.check_output([str(binary),str(value)],text=True);assert native==actual;samples.append({'input':value,'native_and_recompiled_stdout':actual,'returned_address_matches_emitted_definition':True})
   records.append({'arm':arm,'compiler':compiler,'optimization':level,'source_sha256':sha(source),'binary_sha256':sha(binary),'samples':samples})
record={'build':build,'fixture_sha256':sha(fixture),'unchanged_whole_bodies_per_arm':3,'candidate_native_value_comparisons':36,'candidate_returned_address_comparisons':36,'parent_also_compiles_and_matches_native':True,'pre_dependency_regression':'/tmp/kuna-dwarf-namespace/promoted-pointer-pre-dependency/record.json','source_rewrites':'No emitted body edits; main is renamed by a preprocessor macro so the added driver can verify the pointer before invoking it.','records':records}
(out/'record.json').write_text(json.dumps(record,indent=2)+'\n');print({k:v for k,v in record.items()if k!='records'})
