from pathlib import Path
import hashlib,json,os,subprocess

r=Path('/home/mahaloz/kwt/angr-fnptr-namespace');base=Path('/home/mahaloz/kwt/angr-base-4a9c');w=Path('/tmp/kuna-fnptr-namespace');out=w/'public-repro';out.mkdir(exist_ok=False)
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest();build=json.loads((w/'build.json').read_text())
env=os.environ|{'KUNA_SPECS':str(r/'specs'),'SLEIGHHOME':str(r/'specs')}
source='''namespace left { __attribute__((noinline)) unsigned target(unsigned x) { return x + 10; } }
__attribute__((noinline)) unsigned target(unsigned x) { return x + 40; }
extern "C" unsigned long get_target() { return reinterpret_cast<unsigned long>(&left::target); }
int main() { return 0; }
'''
src=out/'callback.cpp';src.write_text(source);fixture=out/'callback'
cmd=['clang++','-O1','-fno-inline',str(src),'-o',str(fixture)];result=subprocess.run(cmd,capture_output=True,text=True,check=True);(out/'compile.log').write_text(result.stdout+result.stderr)
captures={};hello={}
for arm,root in [('parent',base),('candidate',r)]:
 assert sha(root/'decompiler/target/release/kuna')==build['parent_cli_sha256'if arm=='parent'else 'new_cli_sha256']
 cmd=[str(root/'decompiler/target/release/kuna'),'decompile',str(fixture),'get_target','--assert-strict','--assert','prototype get_target unsigned long get_target(void)']
 result=subprocess.run(cmd,cwd=r,env=env,capture_output=True,text=True,check=True);(out/f'{arm}.c').write_text(result.stdout);(out/f'{arm}.log').write_text(result.stderr);captures[arm]=result.stdout
 cmd=[str(root/'decompiler/target/release/kuna'),'decompile-all','/tmp/kuna-angr-reference/general/hello','--functions','main','--language','c','--json']
 result=subprocess.run(cmd,cwd=r,env=env,capture_output=True,text=True,check=True);(out/f'hello-{arm}.json').write_text(result.stdout);(out/f'hello-{arm}.log').write_text(result.stderr);doc=json.loads(result.stdout);assert len(doc['functions'])==1 and not doc['error'];hello[arm]=doc['functions'][0]
assert 'return (unsigned long)target;'in captures['parent'];assert 'return (unsigned long)left::target;'in captures['candidate']
assert hello['parent']['address']==0x8a50 and hello['candidate']['address']==0x8a50
assert 'std::rt::lang_start(main,'in hello['parent']['code']
assert 'std::rt::lang_start(rust_hello_world::main,'in hello['candidate']['code']
assert 'extern'not in hello['candidate']['code']
assert {k:v for k,v in hello['parent'].items()if k!='code'}=={k:v for k,v in hello['candidate'].items()if k!='code'}
record={'build':build,'source':source,'source_sha256':sha(src),'fixture_sha256':sha(fixture),'before':captures['parent'],'after':captures['candidate'],'hello_fixture_sha256':sha(Path('/tmp/kuna-angr-reference/general/hello')),'hello_assertions':{'qualified_function_pointer':True,'no_extern_declaration':True,'all_non_code_metadata_equal':True},'hello':{arm:row['code']for arm,row in hello.items()}}
(out/'record.json').write_text(json.dumps(record,indent=2)+'\n');print(json.dumps(record,indent=2))
