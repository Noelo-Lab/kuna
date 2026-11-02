from pathlib import Path
from concurrent.futures import ProcessPoolExecutor
import importlib.util,os,json,time,hashlib,subprocess,threading,multiprocessing,gzip,shutil
w=Path('/tmp/kuna-rust-io-ddd');old=Path('/tmp/kuna-rust-io-proposal')
spec=importlib.util.spec_from_file_location('io_timing_lib',w/'timing-lib.py');timing=importlib.util.module_from_spec(spec);spec.loader.exec_module(timing)
def worker(job):return timing.case(job)
def sha(path):return hashlib.sha256(Path(path).read_bytes()).hexdigest()
def main():
 plan=json.loads((w/'timing-plan.json').read_text());config=json.loads((w/'timing-cases.json').read_text());root=Path(config['root']);base=Path(config['base'])
 assert plan['samples_per_arm']==384 and plan['comparisons']==len(config['cases'])*4 and plan['maximum_looks']==1
 assert plan['cases']==[row[0]for row in config['cases']]
 reanchor=json.loads((w/'reanchor.json').read_text());assert config['baseline_commit']==reanchor['new_parent']
 assert json.loads(Path('/tmp/kuna-rust-fmt-write-ddd/breadth-current/record.json').read_text())['all_feature_changes_identical_to_reviewed_records']
 attempts=w/'timing-randomized-attempts';assert not attempts.exists(),'No unchanged-code repeat.'
 gates=json.loads((w/'gates-passed.json').read_text());assert gates['all_four_gates_passed']and gates['additional_tools_lint_catalog_counters_passed']
 breadth=json.loads((w/'breadth-current/record.json').read_text());assert not breadth['semantic_evidence_pending']and breadth['summary']['images']==29 and breadth['summary']['changed_functions']==11
 controls=json.loads((w/'post-integration-controls.json').read_text());assert controls['all_selected_outputs_identical']and controls['functions']==28
 assert json.loads((old/'native-execution.json').read_text())['total_comparisons']==63477
 assert json.loads((old/'promoted-native-execution.json').read_text())['total_comparisons']==35532
 assert not (root/'decompiler/target/debug').exists()
 current=json.loads((w/'production-source-before-validation.json').read_text());parent=json.loads((w/'parent-source-before-timing.json').read_text())
 def check_sources():
  assert all(sha(root/p)==h for p,h in current.items());assert all(sha(base/p)==h for p,h in parent['files'].items())
 check_sources();assert subprocess.check_output(['git','rev-parse','HEAD'],cwd=base,text=True).strip()==config['baseline_commit']==parent['commit']
 assert not subprocess.check_output(['git','status','--porcelain'],cwd=base)
 digests={'baseline':sha(base/'decompiler/target/release/kuna'),'patched':sha(root/'decompiler/target/release/kuna')}
 assert digests['baseline']==parent['binary_sha256']==breadth['baseline_kuna_sha256'];assert digests['patched']==gates['release_sha256']==breadth['new_kuna_sha256']==controls['new_kuna_sha256']
 unexpected=[]
 for path in Path('/proc').iterdir():
  if not path.name.isdigit()or int(path.name)==os.getpid():continue
  try:args=(path/'cmdline').read_bytes().decode(errors='replace').split('\0')
  except OSError:continue
  if args and Path(args[0]).name not in ['bash','timeout','rg']and any(prefix in ' '.join(args)for prefix in ['/home/mahaloz/kwt/angr-rust-','/tmp/kuna-rust-']):unexpected.append({'pid':int(path.name),'args':args})
 assert not unexpected,unexpected
 manifest=Path(plan['compiled_specs']['manifest']);assert sha(manifest)==plan['compiled_specs']['sha256'];specs=json.loads(manifest.read_text());assert len(specs)==154
 def check_specs():return {str(tree):all(sha(tree/'specs'/p)==h for p,h in specs.items())for tree in [base,root]}
 before_specs=check_specs();assert all(before_specs.values())
 allowed=os.sched_getaffinity(0);pairs=set()
 for cpu in allowed:
  pair=tuple(map(int,Path(f'/sys/devices/system/cpu/cpu{cpu}/topology/thread_siblings_list').read_text().strip().split(',')))
  if len(pair)==2 and set(pair)<=allowed and not set(pair)&set(plan['reserved_lightwork_cpus']):pairs.add(pair)
 a=timing.sample();time.sleep(5);b=timing.sample()
 idle={c:100*(b['cpus'][c][3]-a['cpus'][c][3])/sum(y-x for x,y in zip(a['cpus'][c],b['cpus'][c]))for pair in pairs for c in pair}
 selected=sorted(pairs,key=lambda p:(min(idle[c]for c in p),sum(idle[c]for c in p),p),reverse=True)[:len(config['cases'])];assert len(selected)==len(config['cases'])
 assignments={row[0]:{'cpu':max(pair),'sibling':min(pair)}for row,pair in zip(config['cases'],selected)}
 attempt=attempts/time.strftime('%Y%m%d-%H%M%S',time.gmtime());attempt.mkdir(parents=True)
 for name in ['timing.py','timing-lib.py','timing-plan.json','timing-cases.json']:shutil.copy2(w/name,attempt/name)
 (attempt/'cpu-selection.json').write_text(json.dumps({'assignments':assignments,'idle_pct':idle,'unexpected_owned_processes':unexpected},indent=2)+'\n')
 os.sched_setaffinity(0,allowed-{cpu for pair in selected for cpu in pair});telemetry=[timing.sample()];done=threading.Event()
 def monitor():
  while not done.wait(.25):telemetry.append(timing.sample())
 thread=threading.Thread(target=monitor);thread.start()
 try:
  jobs=[(config,plan,row,index,assignments[row[0]]['cpu'],digests,str(attempt))for index,row in enumerate(config['cases'])]
  with ProcessPoolExecutor(max_workers=len(jobs),mp_context=multiprocessing.get_context('spawn'))as pool:records=list(pool.map(worker,jobs))
 finally:
  done.set();thread.join();telemetry.append(timing.sample())
  with gzip.open(attempt/'cpu-samples.json.gz','wt')as output:json.dump(telemetry,output)
 check_sources();assert sha(base/'decompiler/target/release/kuna')==digests['baseline']and sha(root/'decompiler/target/release/kuna')==digests['patched']
 after_specs=check_specs();assert all(after_specs.values())
 for index,record in enumerate(records):
  timing.inference(record,plan,index);(attempt/f'timing-{record["fixture"]}.json').write_text(json.dumps(record,indent=2)+'\n')
  print('FINAL BOUNDS',record['fixture'],record['delta_pct'],record['confidence_upper_delta_pct'],record['speed_budget_passed'],flush=True)
 accepted=all(record['speed_budget_passed']for record in records)
 result={'plan':plan,'baseline_commit':config['baseline_commit'],'binaries_sha256':digests,'source_manifests':{'current':sha(w/'production-source-before-validation.json'),'parent':sha(w/'parent-source-before-timing.json')},'compiled_specs':{'count':154,'before':before_specs,'after':after_specs},'attempt':str(attempt),'accepted_for_speed_review':accepted,'results':{record['fixture']:{'median_delta_pct':record['delta_pct'],'confidence_upper_delta_pct':record['confidence_upper_delta_pct'],'speed_budget_passed':record['speed_budget_passed']}for record in records}}
 (attempt/'monitor.json').write_text(json.dumps(result,indent=2)+'\n');(w/'timing-result.json').write_text(json.dumps(result,indent=2)+'\n')
 assert accepted,'Default-on speed criterion unresolved; retain samples and keep the feature opt-in, with no unchanged-code repeat.'
 print('IO DEFAULT-ON SPEED STUDY PASSED',flush=True)
if __name__=='__main__':main()
