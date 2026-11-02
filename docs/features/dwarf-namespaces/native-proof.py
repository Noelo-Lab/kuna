from pathlib import Path
import subprocess,json,hashlib,os
r=Path('/home/mahaloz/kwt/angr-dwarf-namespace');base=Path('/home/mahaloz/kwt/angr-base-1d5f');w=Path('/tmp/kuna-dwarf-namespace-p9');out=w/'native-proof';out.mkdir(exist_ok=False)
build=json.loads((w/'build.json').read_text());sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
assert sha(r/'decompiler/target/release/kuna')==build['new_cli_sha256'];assert sha(base/'decompiler/target/release/kuna')==build['parent_cli_sha256']
inputs=[0,1,7,12345,0x7fffffff,0x80000000,0xffffffd7,0xfffffff5,0xffffffff]
records=[]
for lang,output,count,compilers in [('cpp','c',5,['clang++','g++']),('rust','rust',4,['rustc'])]:
 fixture=r/f'decompiler/crates/kuna-analysis/tests/fixtures/dwarf_namespaces_{lang}_x86_64'
 names='direct_left,direct_nested,direct_right,direct_global'+(',direct_shadow'if lang=='cpp'else '')
 source=(r/f'decompiler/crates/kuna-analysis/tests/fixtures/dwarf_namespaces.{"cpp"if lang=="cpp"else "rs"}').read_text()
 helpers=source.split('extern "C"'if lang=='cpp'else '#[no_mangle]')[0]
 main=('int main(int argc, char **argv) { uint32_t x = static_cast<uint32_t>(std::strtoul(argv[1], nullptr, 0)); std::printf("%u %u %u %u %u\\n", direct_left(x), direct_nested(x), direct_right(x), direct_global(x), direct_shadow(x)); }'if lang=='cpp'else 'fn main() { let x = std::env::args().nth(1).unwrap().parse::<u32>().unwrap(); unsafe { println!("{} {} {} {}", direct_left(x), direct_nested(x), direct_right(x), direct_global(x)); } }')
 for arm,root in [('parent',base),('candidate',r)]:
  cmd=[str(root/'decompiler/target/release/kuna'),'decompile-all',str(fixture),'--json','--language',output,'--option','cppproto','on','--functions',names]
  env=os.environ|{'KUNA_SPECS':str(r/'specs'),'SLEIGHHOME':str(r/'specs')}
  capture=subprocess.run(cmd,cwd=r,env=env,capture_output=True,text=True,check=True);doc=json.loads(capture.stdout);assert len(doc['functions'])==count
  (out/f'{lang}-{arm}.json').write_text(capture.stdout);(out/f'{lang}-{arm}.log').write_text(capture.stderr)
  bodies='\n'.join(f['code']for f in doc['functions']);assert all(not f.get('error')for f in doc['functions'])
  sourcefile=out/f'{lang}-{arm}.{ "cpp"if lang=="cpp"else "rs" }'
  sourcefile.write_text(('extern crate self as dwarf_namespaces;\n'if lang=='rust'else '')+helpers+'\n'+bodies+'\n'+main+'\n')
  for compiler in compilers:
   for level in ['0','2']:
    binary=sourcefile.with_suffix(f'.{compiler}-O{level}')
    flags=['--edition=2021','--crate-name','namespace_proof','-C',f'opt-level={level}']if lang=='rust'else ['-std=c++17',f'-O{level}']
    compiled=subprocess.run([compiler,*flags,str(sourcefile),'-o',str(binary)],capture_output=True,text=True);(out/f'{lang}-{arm}-{compiler}-O{level}.log').write_text(compiled.stdout+compiled.stderr);assert compiled.returncode==0
    samples=[]
    for value in inputs:
     native=list(map(int,subprocess.check_output([str(fixture),str(value)],text=True).split()))[:count]
     actual=list(map(int,subprocess.check_output([str(binary),str(value)],text=True).split()))
     if arm=='candidate':assert actual==native,(lang,value,actual,native)
     else:assert actual!=native and all(x==(value+40)&0xffffffff for x in actual)
     samples.append({'input':value,'native':native,'recompiled':actual,'equal':actual==native})
    records.append({'language':lang,'arm':arm,'compiler':compiler,'optimization':level,'source':sourcefile.name,'source_sha256':sha(sourcefile),'binary_sha256':sha(binary),'complete_bodies':count,'samples':samples})
record={'new_cli_sha256':build['new_cli_sha256'],'parent_cli_sha256':build['parent_cli_sha256'],'correct_candidate_value_comparisons':sum(len(x['samples'])*x['complete_bodies']for x in records if x['arm']=='candidate'),'parent_mismatch_sequences':sum(len(x['samples'])for x in records if x['arm']=='parent'),'original_helpers_used_without_rewriting':True,'emitted_bodies_used_without_rewriting':True,'records':records}
(out/'record.json').write_text(json.dumps(record,indent=2)+'\n');print({k:v for k,v in record.items()if k!='records'})
