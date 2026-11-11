from pathlib import Path
import hashlib,json,os,subprocess,shutil

r=Path('/home/mahaloz/kwt/angr-fnptr-namespace');base=Path('/home/mahaloz/kwt/angr-base-4a9c');w=Path('/tmp/kuna-fnptr-namespace');out=w/'native-proof';out.mkdir(exist_ok=False)
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
build=json.loads((w/'build.json').read_text());env=os.environ|{'KUNA_SPECS':str(r/'specs'),'SLEIGHHOME':str(r/'specs')}
assert sha(r/'decompiler/target/release/kuna')==build['new_cli_sha256'];assert sha(base/'decompiler/target/release/kuna')==build['parent_cli_sha256']
fixtures=r/'decompiler/crates/kuna-analysis/tests/fixtures';fixture=fixtures/'function_pointer_namespaces_x86_64'
shutil.copy2(fixtures/'function_pointer_namespaces.cpp',out/'function_pointer_namespaces.cpp')
names=['left::same_address','right::cross_address','foreign_address','nested_address','right_address','global_shadow','scope_shadow']
inputs=[0,1,7,12345,0x7fffffff,0x80000000,0xffffffd7,0xfffffff5,0xffffffff]
renamed=[name.split('::')[-1]for name in names]+['main']
prefix=''.join(f'#define {name} original_{name}\n'for name in renamed)+'#include "function_pointer_namespaces.cpp"\n'+''.join(f'#undef {name}\n'for name in renamed)+'namespace left { uintptr_t same_address(); }\nnamespace right { uintptr_t cross_address(); }\n'
driver=r'''
int main() {
    unsigned inputs[] = {0, 1, 7, 12345, 0x7fffffffU, 0x80000000U, 0xffffffd7U, 0xfffffff5U, 0xffffffffU};
    uintptr_t valid[] = {reinterpret_cast<uintptr_t>(&left::target), reinterpret_cast<uintptr_t>(&left::deep::target), reinterpret_cast<uintptr_t>(&right::target), reinterpret_cast<uintptr_t>(&::target)};
    for (unsigned x : inputs) {
        uintptr_t actual[] = {left::same_address(), right::cross_address(), foreign_address(), nested_address(), right_address(), global_shadow(x), scope_shadow(x)};
        uintptr_t expected[] = {left::original_same_address(), right::original_cross_address(), original_foreign_address(), original_nested_address(), original_right_address(), original_global_shadow(x), original_scope_shadow(x)};
        for (unsigned i = 0; i < 7; ++i) {
            bool known = false;
            for (uintptr_t address : valid) known |= actual[i] == address;
            unsigned actual_value = known ? reinterpret_cast<Callback>(actual[i])(x) : 0;
            unsigned expected_value = reinterpret_cast<Callback>(expected[i])(x);
            std::printf("%u %u %u %u %u %u\n", x, i, actual[i] == expected[i], known, actual_value, expected_value);
        }
    }
}
'''
records=[];captures={}
for arm,root in [('parent',base),('candidate',r)]:
 for strategy in ['none','minimal','all']:
  cmd=[str(root/'decompiler/target/release/kuna'),'decompile-all',str(fixture),'--json','--language','c','--assert-strict','--assert',f'@{fixtures / "function_pointer_namespaces.kuna"}','--option','namespacestrategy',strategy,'--functions',','.join(names+['interior_address','data_address'])]
  result=subprocess.run(cmd,cwd=r,env=env,capture_output=True,text=True,check=True);doc=json.loads(result.stdout);assert len(doc['functions'])==9 and all(not f.get('error')for f in doc['functions'])
  (out/f'{arm}-{strategy}.json').write_text(result.stdout);(out/f'{arm}-{strategy}.log').write_text(result.stderr);captures[(arm,strategy)]={f['name']:f for f in doc['functions']}
  if strategy=='none':continue
  source=out/f'{arm}-{strategy}.cpp';source.write_text(prefix+'\n'+'\n'.join(captures[(arm,strategy)][name]['code']for name in names)+'\n'+driver)
  for compiler in ['clang++','g++']:
   for level in ['0','2']:
    binary=source.with_suffix(f'.{compiler}-O{level}');cmd=[compiler,'-std=c++17',f'-O{level}',str(source),'-o',str(binary)]
    result=subprocess.run(cmd,capture_output=True,text=True);(out/f'{arm}-{strategy}-{compiler}-O{level}.compile.log').write_text(result.stdout+result.stderr);assert result.returncode==0,(arm,strategy,compiler,level)
    actual=subprocess.check_output([str(binary)],text=True);(out/f'{arm}-{strategy}-{compiler}-O{level}.stdout').write_text(actual)
    samples=[dict(zip(['input','getter_index','address_equal','known_address','actual_value','expected_value'],map(int,line.split())))for line in actual.splitlines()];assert len(samples)==63
    if arm=='candidate':assert all(s['address_equal']and s['known_address']and s['actual_value']==s['expected_value']for s in samples)
    else:assert any(not s['address_equal']for s in samples)
    records.append({'arm':arm,'strategy':strategy,'compiler':compiler,'optimization':level,'source':source.name,'source_sha256':sha(source),'binary_sha256':sha(binary),'samples':samples})
assert captures[('parent','none')]==captures[('candidate','none')]
for strategy in ['minimal','all']:
 for name in ['interior_address','data_address']:assert captures[('parent',strategy)][name]==captures[('candidate',strategy)][name]
 for name in names:
  before=captures[('parent',strategy)][name];after=captures[('candidate',strategy)][name]
  assert {k:v for k,v in before.items()if k!='code'}=={k:v for k,v in after.items()if k!='code'},(strategy,name,'metadata changed')
record={'build':build,'fixture_sha256':sha(fixture),'original_source_sha256':sha(fixtures/'function_pointer_namespaces.cpp'),'original_helper_bodies_unchanged':True,'whole_emitted_getter_bodies_unchanged':True,'prototype_assertions':'Source-derived return/parameter types and shadow parameter names; no function-address names supplied.','candidate_address_comparisons':504,'candidate_callback_value_comparisons':504,'parent_address_mismatches':sum(not s['address_equal']for row in records if row['arm']=='parent'for s in row['samples']),'none_records_equal':True,'all_non_code_metadata_equal':True,'records':records}
(out/'record.json').write_text(json.dumps(record,indent=2)+'\n');print({k:v for k,v in record.items()if k!='records'})
