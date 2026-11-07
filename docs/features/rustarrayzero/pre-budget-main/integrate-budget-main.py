from common import *
import datetime,shutil
assert (w/'build.json').exists()
plan=json.loads((w/'main-budget-integration-plan.json').read_text())
old=w/'pre-budget-main';old.mkdir()
for p in sorted(w.iterdir()):
 if p.is_file() and p.suffix in ['.json','.jsonl','.log','.stdout','.stderr','.rs','.c','.patch','.py','.toml','.txt']:
  shutil.copy2(p,old/p.name)
shutil.copytree(w/'wide-native',old/'wide-native')
shutil.copy2(r/'decompiler/target/release/kuna',old/'candidate-kuna')
(old/'pending.patch').write_bytes(subprocess.check_output(['git','diff','HEAD','--binary'],cwd=r))
(old/'status.json').write_text(json.dumps({'status':'Superseded before full validation or timing by upstream watchdog integration','prior_candidate_cli_sha256':hashlib.sha256((old/'candidate-kuna').read_bytes()).hexdigest(),'prior_build':json.loads((w/'build.json').read_text()),'timing_sampled':False},indent=2)+'\n')
subprocess.run(['git','merge','--ff-only',plan['base_commit']],cwd=r,check=True)
assert subprocess.check_output(['git','rev-parse','HEAD'],cwd=r,text=True).strip()==plan['base_commit']
for p in w.glob('*.py'):
 text=p.read_text().replace('/home/mahaloz/kwt/angr-array-zero-base-8ced',plan['parent_root']).replace('main8ced','main d5a1042b').replace('PR1466 and PR1468','PR1466, PR1468 and PR1465')
 if p.name=='bootstrap-parent.py':text=text.replace("shutil.copy2(w/'plan.json',w/'original-plan.json');",'')
 if p.name=='finalize.py':
  text=text.replace("workspace['passed']>=10375", "workspace['passed']>=10377").replace("workspace['suites']==72", "workspace['suites']==73")
  text=text.replace("'pending-before-rebase.patch']", "'pending-before-rebase.patch','main-budget-integration-plan.json']")
  text=text.replace("'failed-default-on','pre-main-validation']", "'failed-default-on','pre-main-validation','pre-budget-main']")
  text=text.replace('including two independent P6 performance changes', 'including independent P6 performance and watchdog changes')
 p.write_text(text)
timing=json.loads((w/'timing-plan.json').read_text());timing.update(base=plan['parent_root'],seed=202611075,declared_at=datetime.datetime.now(datetime.timezone.utc).isoformat(),status='Actual main d5a1042b includes independent performance and watchdog changes. Prior seed074 was never sampled. Fresh default-off/on comparison required; option remains off.')
timing.pop('build',None);timing.pop('bound_at',None);timing.pop('candidate_cli_sha256',None)
(w/'timing-plan.json').write_text(json.dumps(timing,indent=2)+'\n')
(w/'build.json').unlink()
print('Pending feature integrated on latest main; previous native proof retained',flush=True)
