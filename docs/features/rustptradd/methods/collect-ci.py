from common import *
from concurrent.futures import ThreadPoolExecutor
import re
commit=json.loads((w/'commit.json').read_text());merge=json.loads((w/'ci-merge.json').read_text());pr=json.loads((w/'pr-fresh.json').read_text());repo='Noelo-Lab/kuna'
run_ids={int(re.search(r'/actions/runs/(\d+)',c['detailsUrl']).group(1))for c in pr['statusCheckRollup'] if c.get('detailsUrl') and '/actions/runs/'in c['detailsUrl']}
run_ids.discard(merge['superseded_tests_run']);jobs=[];runs=[]
for run_id in sorted(run_ids):
 run=json.loads(subprocess.check_output(['gh','api',f'repos/{repo}/actions/runs/{run_id}'],text=True));assert run['head_sha']==commit['head'];runs.append({key:run[key]for key in ['id','name','head_sha','status','conclusion','event']})
 data=json.loads(subprocess.check_output(['gh','api',f'repos/{repo}/actions/runs/{run_id}/jobs?per_page=100'],text=True))
 for job in data['jobs']:
  if job['status']=='completed' and job['conclusion']=='success':jobs.append((run,job))
def collect(pair):
 run,job=pair;expected=merge['merge'] if run['name']=='Tests' else commit['head'];path=w/f'ci-job-{job["id"]}.log'
 if not path.exists():path.write_bytes(subprocess.check_output(['gh','api',f'repos/{repo}/actions/jobs/{job["id"]}/logs']))
 text=path.read_text();lines=[line for line in text.splitlines()if line.rstrip().endswith(expected)];assert lines,(job['name'],expected)
 return {'run':run['id'],'job':job['id'],'name':job['name'],'conclusion':job['conclusion'],'checkout':expected,'checkout_lines':lines,'log_sha256':hashlib.sha256(path.read_bytes()).hexdigest()}
with ThreadPoolExecutor(max_workers=4)as pool:records=list(pool.map(collect,jobs))
(w/'ci-completed-checkouts.json').write_text(json.dumps({'runs':runs,'records':records,'superseded_tests_run':merge['superseded_tests_run']},indent=2)+'\n')
print('CI runs:',runs);print('Verified completed checkouts:',[(r['name'],r['checkout'][:12])for r in records])
