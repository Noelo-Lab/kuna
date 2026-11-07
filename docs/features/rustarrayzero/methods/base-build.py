from pathlib import Path
import sys,json,hashlib,subprocess,datetime
sys.path.insert(0,'/tmp/kuna-rust-array-constant')
from common import run
w=Path('/tmp/kuna-rust-array-zero-rebase');plan=json.loads((w/'rebase-plan.json').read_text());r=Path(plan['new_base_root'])
run(r,w,['cargo','build','--release','-p','kuna-cli','-p','kuna-console','-p','kuna-harness'],'base-release.log',cwd=r/'decompiler')
head=subprocess.check_output(['git','rev-parse','HEAD'],cwd=r,text=True).strip();assert head==plan['new_base_commit'];assert not subprocess.check_output(['git','status','--porcelain'],cwd=r)
record={'at':datetime.datetime.now(datetime.timezone.utc).isoformat(),'source':'Actual fetched main branch tip after merged PR1466, PR1468 and PR1465','published_commit':head,'merged_commit':head,'published_tree':plan['new_base_tree'],'merged_tree':plan['new_base_tree'],'candidate_cli_sha256':hashlib.sha256((r/'decompiler/target/release/kuna').read_bytes()).hexdigest(),'prior_feature_parent':'https://github.com/Noelo-Lab/kuna/pull/1471','upstream_changes':['https://github.com/Noelo-Lab/kuna/pull/1466','https://github.com/Noelo-Lab/kuna/pull/1468']};(w/'parent-main.json').write_text(json.dumps(record,indent=2)+'\n');print('Fresh main parent built',record,flush=True)
