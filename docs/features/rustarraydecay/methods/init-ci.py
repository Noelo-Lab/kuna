from common import *
commit=json.loads((w/'commit.json').read_text());number=int((w/'published.txt').read_text().strip().rsplit('/',1)[-1]);repo='Noelo-Lab/kuna'
runs=json.loads(subprocess.check_output(['gh','api',f'repos/{repo}/actions/runs?branch=fix/angr-rust-array-decay&per_page=30'],cwd=r,text=True))['workflow_runs']
tests=sorted([run for run in runs if run['head_sha']==commit['head']and run['name']=='Tests'],key=lambda run:run['id'],reverse=True);assert tests,'Tests workflow has not started yet'
active=tests[0];superseded=[run for run in tests[1:]];assert len(superseded)<=1 and all(run['status']=='completed'and run['conclusion']=='cancelled'for run in superseded),superseded
ref=f'refs/kuna/angr/rust-array-decay-ci-merge';subprocess.run(['git','fetch','origin',f'refs/pull/{number}/merge:{ref}'],cwd=r,check=True)
merge=subprocess.check_output(['git','rev-parse',ref],cwd=r,text=True).strip();tree=subprocess.check_output(['git','rev-parse',merge+'^{tree}'],cwd=r,text=True).strip();assert tree==commit['tree']
parents=subprocess.check_output(['git','rev-list','--parents','-n','1',merge],cwd=r,text=True).split()[1:];assert parents==[commit['parent_main'],commit['head']],parents
(w/'ci-merge.json').write_text(json.dumps({'merge':merge,'tree':tree,'head':commit['head'],'base':commit['parent_main'],'active_tests_run':active['id'],'superseded_tests_run':superseded[0]['id']if superseded else None},indent=2)+'\n');print('CI merge',merge,tree,'Tests',active['id'])
