from common import *
import datetime
plan=json.loads((w/'main-budget-integration-plan.json').read_text());base=Path(plan['parent_root'])
run(base,w,['cargo','build','--release','-p','kuna-cli','-p','kuna-console','-p','kuna-harness'],'budget-base-release.log',cwd=base/'decompiler')
assert subprocess.check_output(['git','rev-parse','HEAD'],cwd=base,text=True).strip()==plan['base_commit']
assert not subprocess.check_output(['git','status','--porcelain'],cwd=base)
record={'at':datetime.datetime.now(datetime.timezone.utc).isoformat(),'source':'Actual main after independent PR1466, PR1468 and PR1465','published_commit':plan['base_commit'],'merged_commit':plan['base_commit'],'published_tree':plan['base_tree'],'merged_tree':plan['base_tree'],'candidate_cli_sha256':hashlib.sha256((base/'decompiler/target/release/kuna').read_bytes()).hexdigest(),'prior_feature_parent':'https://github.com/Noelo-Lab/kuna/pull/1471','upstream_changes':['https://github.com/Noelo-Lab/kuna/pull/'+str(n)for n in [1466,1468,1465]]}
(w/'parent-budget-main.json').write_text(json.dumps(record,indent=2)+'\n');print('Budget main parent ready',record,flush=True)
