from pathlib import Path
import json,os,subprocess,hashlib
w=Path('/tmp/kuna-dwarf-namespace-p9');r=Path('/home/mahaloz/kwt/angr-dwarf-namespace');out=w/'public-repro';out.mkdir(exist_ok=False);build=json.loads((w/'build.json').read_text());source=out/'demo.cpp';source.write_text('''#include <cstdio>
namespace left { __attribute__((noinline)) unsigned target(unsigned x) { return x + 10; } }
__attribute__((noinline)) unsigned target(unsigned x) { return x + 40; }
extern "C" __attribute__((noinline)) unsigned direct(unsigned x) { return left::target(x); }
int main(int argc, char **) { std::printf("%u %u\\n", direct(argc), target(argc)); }
''');binary=out/'demo';cmd=['clang++','-g','-O1','-fno-inline','-fno-optimize-sibling-calls',str(source),'-o',str(binary)];p=subprocess.run(cmd,capture_output=True,text=True);(out/'build.log').write_text(p.stdout+p.stderr);assert p.returncode==0
captures={}
for arm,root in [('parent',Path(build['baseline_root'])),('candidate',r)]:
 cmd=[str(root/'decompiler/target/release/kuna'),'decompile',str(binary),'direct','--language','c','--json'];p=subprocess.run(cmd,cwd=r,env=os.environ|{'KUNA_SPECS':str(r/'specs')},capture_output=True,text=True);assert p.returncode==0;(out/(arm+'.json')).write_text(p.stdout);(out/(arm+'.log')).write_text(p.stderr);envelope=json.loads(p.stdout);assert not envelope.get('error')and len(envelope['functions'])==1;d=envelope['functions'][0];assert not d.get('error');captures[arm]=d;print(arm,d['code'])
assert 'return target(x);'in captures['parent']['code'];assert 'return left::target(x);'in captures['candidate']['code']
record={'build':build,'commands':{'compile':['clang++','-g','-O1','-fno-inline','-fno-optimize-sibling-calls','demo.cpp','-o','demo'],'decompile':['kuna','decompile','./demo','direct','--language','c']},'source_sha256':hashlib.sha256(source.read_bytes()).hexdigest(),'fixture_sha256':hashlib.sha256(binary.read_bytes()).hexdigest(),'before':captures['parent']['code'],'after':captures['candidate']['code'],'status':'PASS'};(out/'record.json').write_text(json.dumps(record,indent=2)+'\n')
