from pathlib import Path
import hashlib,json,subprocess,os
w=Path('/tmp/kuna-dwarf-namespace-p9');r=Path('/home/mahaloz/kwt/angr-dwarf-namespace');out=w/'raw-namespace-proof';out.mkdir(exist_ok=False)
build=json.loads((w/'build.json').read_text());fixture=r/'decompiler/crates/kuna-analysis/tests/fixtures/dwarf_raw_namespace_x86_64';inputs=[0,1,7,12345,0x7fffffff,0x80000000,0xffffffd7,0xfffffff5,0xffffffff];records={}
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
arms={'parent':Path(build['baseline_root'])/'decompiler/target/release/kuna','rejected':w/'pre-scope-guard/kuna','candidate':r/'decompiler/target/release/kuna'}
addresses=None
for arm,kuna in arms.items():
 selector='raw_scope::union::target,direct_target'if arm=='rejected'else'target,direct_target'
 cmd=[str(kuna),'decompile-all',str(fixture),'--json','--language','c','--functions',selector];p=subprocess.run(cmd,cwd=r,env=os.environ|{'KUNA_SPECS':str(r/'specs'),'SLEIGHHOME':str(r/'specs')},capture_output=True,text=True);assert p.returncode==0
 (out/(arm+'.json')).write_text(p.stdout);(out/(arm+'.log')).write_text(p.stderr);rows=json.loads(p.stdout)['functions'];assert len(rows)==2 and all(x['address']!=0 and not x.get('error')for x in rows)
 current_addresses=[x['address']for x in rows]
 if addresses is not None:assert addresses==current_addresses
 addresses=current_addresses
 body='\n'.join(x['code']for x in rows);source=out/(arm+'.cpp');source.write_text('#include <cstdio>\n#include <cstdlib>\nnamespace raw_scope {}\n'+body+'\nint main(int argc,char **argv) { unsigned int x=static_cast<unsigned int>(std::strtoul(argv[1],nullptr,0)); std::printf("%u\\n",direct_target(x)); }\n')
 runs=[]
 for compiler in ['g++','clang++']:
  for level in ['0','2']:
   binary=out/f'{arm}-{compiler}-O{level}';q=subprocess.run([compiler,'-std=c++17',f'-O{level}',str(source),'-o',str(binary)],capture_output=True,text=True);log=binary.name+'.log';(out/log).write_text(q.stdout+q.stderr);samples=[]
   if arm=='rejected':assert q.returncode!=0 and 'union' in q.stderr
   else:
    assert q.returncode==0,(arm,compiler,q.stderr)
    for x in inputs:
     expected=subprocess.check_output([str(fixture),str(x)],text=True);actual=subprocess.check_output([str(binary),str(x)],text=True);assert expected==actual
     samples.append({'input':x,'native_and_recompiled_stdout':actual})
   runs.append({'compiler':compiler,'optimization':level,'compile_returncode':q.returncode,'log':log,'samples':samples})
 records[arm]={'cli_sha256':sha(kuna),'capture_sha256':sha(out/(arm+'.json')),'source_sha256':sha(source),'body_rewrites':0,'whole_bodies':2,'runs':runs}
parent=json.loads((out/'parent.json').read_text())['functions'];candidate=json.loads((out/'candidate.json').read_text())['functions'];assert parent==candidate
record={'fixture_sha256':sha(fixture),'records':records,'candidate_native_values_compared':36,'parent_records_equal_candidate':True,'rejected_candidate_compile_failures':4,'status':'PASS'};(out/'record.json').write_text(json.dumps(record,indent=2)+'\n');print('Raw namespace: parent/final candidate exact records, native execution PASS36, rejected42b compile fails4')
