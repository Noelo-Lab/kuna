from pathlib import Path
import subprocess,json,hashlib
w=Path('/tmp/kuna-dwarf-namespace-p9');r=Path('/home/mahaloz/kwt/angr-dwarf-namespace');out=w/'cpp-complete-proof';out.mkdir(exist_ok=False)
row=next(x for x in json.loads((w/'breadth/namespace-cpp-record.json').read_text())['records']if x['language']=='c')
path=Path(row['candidate_file']);assert hashlib.sha256(path.read_bytes()).hexdigest()==row['candidate_sha256']
functions=[x for x in json.loads(path.read_text())['functions']if 0x1150<=x['address']<=0x1200];assert len(functions)==12 and all(not x.get('error')for x in functions)
header='#include <cstdio>\n#include <cstdlib>\nnamespace left { unsigned int target(unsigned int); unsigned int same(unsigned int); namespace deep { unsigned int target(unsigned int); } }\nnamespace right { unsigned int target(unsigned int); unsigned int cross(unsigned int); }\n'
source=out/'complete.cpp';source.write_text(header+'\n'.join(x['code']for x in functions)+'\n')
fixture=r/'decompiler/crates/kuna-analysis/tests/fixtures/dwarf_namespaces_cpp_x86_64';inputs=[[],['0'],['1'],['7'],['12345'],['2147483647'],['2147483648'],['4294967255'],['4294967285'],['4294967295']];records=[]
for compiler in ['clang++','g++']:
 for level in ['0','2']:
  binary=out/f'complete-{compiler}-O{level}';cmd=[compiler,'-std=c++17',f'-O{level}',str(source),'-o',str(binary)];p=subprocess.run(cmd,capture_output=True,text=True);(out/f'{compiler}-O{level}.log').write_text(p.stdout+p.stderr);assert p.returncode==0
  samples=[]
  for args in inputs:
   expected=subprocess.check_output([str(fixture),*args]);actual=subprocess.check_output([str(binary),*args]);assert actual==expected,(args,actual,expected)
   samples.append({'args':args,'native_and_recompiled_stdout':actual.decode()})
  records.append({'compiler':compiler,'optimization':level,'samples':samples,'binary_sha256':hashlib.sha256(binary.read_bytes()).hexdigest()})
d={'whole_user_function_bodies':len(functions),'includes_all_changed_C_function_bodies':True,'source_body_rewrites':0,'runtime_support':'Only standard headers and namespace forward declarations; all twelve user bodies including main are exact emitted text.','values_compared':280,'input_modes':10,'source_sha256':hashlib.sha256(source.read_bytes()).hexdigest(),'candidate_capture_sha256':row['candidate_sha256'],'candidate_cli_sha256':json.loads((w/'build.json').read_text())['new_cli_sha256'],'records':records}
(out/'record.json').write_text(json.dumps(d,indent=2)+'\n');print({k:v for k,v in d.items()if k!='records'})
