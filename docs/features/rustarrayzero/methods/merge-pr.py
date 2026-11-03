from common import *
import datetime
number=int((w/'published.txt').read_text().strip().rsplit('/',1)[-1])
commit=json.loads((w/'commit.json').read_text());build=json.loads((w/'build.json').read_text());merge=json.loads((w/'ci-merge.json').read_text());ci=json.loads((w/'ci-completed-checkouts.json').read_text())
assert len(ci['runs'])==2 and all(run['status']=='completed'and run['conclusion']=='success'and run['head_sha']==commit['head']for run in ci['runs'])
assert len(ci['records'])==8
assert {row['name']for row in ci['records']}=={'parity gates','cargo workspace suite','Analyze (actions)','Analyze (c-cpp)','Analyze (java-kotlin)','Analyze (javascript-typescript)','Analyze (python)','Analyze (rust)'}
for row in ci['records']:
 assert row['checkout']==(merge['merge'] if row['name']in ['parity gates','cargo workspace suite']else commit['head'])
 assert hashlib.sha256((w/f'ci-job-{row["job"]}.log').read_bytes()).hexdigest()==row['log_sha256']
subprocess.run(['python3',str(w/'check-pr.py')],cwd=r,check=True)
pr=json.loads((w/'pr-fresh.json').read_text());threads=json.loads((w/'review-threads.json').read_text())
assert pr['state']=='OPEN'and not pr['isDraft']and pr['mergeStateStatus']=='CLEAN'
assert not pr['comments']and not pr['reviews']and not threads['data']['repository']['pullRequest']['reviewThreads']['nodes']
checks=[check for check in pr['statusCheckRollup']if '/runs/'+str(merge['superseded_tests_run'])+'/'not in check.get('detailsUrl','')]
assert len(checks)==9 and all(check['status']=='COMPLETED'and check['conclusion']=='SUCCESS'for check in checks)
remote=subprocess.check_output(['git','ls-remote','origin','refs/heads/main'],cwd=r,text=True).split()[0];assert remote==commit['parent_main']
assert not subprocess.check_output(['git','status','--porcelain'],cwd=r)
assert subprocess.check_output(['git','rev-parse','HEAD^{tree}'],cwd=r,text=True).strip()==commit['tree']==merge['tree']
assert hashlib.sha256((r/'decompiler/target/release/kuna').read_bytes()).hexdigest()==build['new_cli_sha256']
specs=json.loads(Path('/tmp/kuna-rust-io-proposal/built-specs.json').read_text());assert all(hashlib.sha256((r/'specs'/p).read_bytes()).hexdigest()==digest for p,digest in specs.items())
(w/'ready-to-merge.json').write_text(json.dumps({'at':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':commit['head'],'tree':commit['tree'],'base':remote,'ci_merge':merge['merge'],'checks':checks,'checkout_records':ci['records'],'candidate_cli_sha256':build['new_cli_sha256'],'all_current_checks_passed':True,'superseded_tests_run':merge['superseded_tests_run']},indent=2)+'\n')
(w/'merge-pr-request.json').write_text(json.dumps({'repository_full_name':'Noelo-Lab/kuna','pr_number':number,'merge_method':'merge','expected_head_sha':commit['head'],'commit_title':f'[AUTOMATED] Add opt-in zero values for Rust byte-array stores (#{number})'},indent=2)+'\n');print('Ready to merge through GitHub plugin',number,commit['head'],flush=True)
