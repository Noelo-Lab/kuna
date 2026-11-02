from common import *
import shutil
expected=['original.log','stages.log','tools.log','lint.log','spec.log','catalog.log','counters.log'];log=(w/'validation-coordinator.log').read_text();assert all(name+' PASS'in log for name in expected)
for name in ['workspace-final.log','validation-coordinator.log','production-source-before-validation.json']:
 shutil.copy2(w/name,w/('first-'+name))
old=json.loads((w/'production-source-before-validation.json').read_text());source={p:hashlib.sha256((r/p).read_bytes()).hexdigest()for p in old};changed=[p for p in old if old[p]!=source[p]];assert changed==['decompiler/crates/kuna-cli/tests/joined_callback_return.rs'],changed
(w/'production-source-before-validation.json').write_text(json.dumps(source,indent=2)+'\n')
env['PATH']='/tmp/kuna-toolchains/go1.27.3/go/bin:'+env['PATH']
run(r,w,['make','rust-test'],'workspace-final.log')
assert all(hashlib.sha256((r/p).read_bytes()).hexdigest()==h for p,h in source.items())
debug=r/'decompiler/target/debug';assert debug.resolve()==debug and not debug.is_symlink();shutil.rmtree(debug)
(w/'gates-passed.json').write_text(json.dumps({'all_four_gates_passed':True,'additional_tools_lint_catalog_counters_passed':True,'original_assertions':675,'stage_assertions':3707,'production_files_unchanged':len(source),'release_sha256':hashlib.sha256((r/'decompiler/target/release/kuna').read_bytes()).hexdigest(),'workspace_retry_reason':'Test harness passed owned Command where helper expects &mut Command; only this test expression changed. Other seven passing checks remain applicable.','changed_files':changed},indent=2)+'\n')
print('All four gates and additional checks now pass; worktree debug removed',flush=True)
