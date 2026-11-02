from pathlib import Path
from concurrent.futures import ProcessPoolExecutor
import datetime,gzip,hashlib,itertools,json,multiprocessing,os,random,resource,statistics,subprocess,threading,time
w=Path('/tmp/kuna-dwarf-namespace-p9')
def sha(path):return hashlib.sha256(Path(path).read_bytes()).hexdigest()
def sample():
 return {'time':time.monotonic(),'cpus':{int(p[0][3:]):list(map(int,p[1:]))for line in Path('/proc/stat').read_text().splitlines()if(p:=line.split())and p[0].startswith('cpu')and p[0][3:].isdigit()}}
def case(job):
 plan,row,index,cpu=job;name,binary,selector,language=row;os.sched_setaffinity(0,{cpu});rng=random.Random(plan['seed']+index)
 raw={arm:[]for arm in plan['arms']};orders=[];fixture_sha=sha(binary)
 def run(arm):
  root=Path(plan['base']if arm=='parent'else plan['root']);cmd=[str(root/'decompiler/target/release/kuna'),'decompile',binary,selector,'--language',language]
  if language=='go':cmd+=['--option','godwarfsig','on']
  before=resource.getrusage(resource.RUSAGE_CHILDREN);start=time.perf_counter()
  subprocess.run(cmd,cwd=plan['root'],env=os.environ|{'KUNA_SPECS':str(Path(plan['root'])/'specs')},stdout=subprocess.DEVNULL,stderr=subprocess.DEVNULL,check=True)
  elapsed=time.perf_counter()-start;after=resource.getrusage(resource.RUSAGE_CHILDREN)
  return {'cpu_ms':1000*(after.ru_utime+after.ru_stime-before.ru_utime-before.ru_stime),'wall_ms':1000*elapsed}
 for _ in range(plan['warmups_per_arm']):
  order=list(raw);rng.shuffle(order)
  for arm in order:run(arm)
 for block in range(plan['samples_per_arm']//2):
  permutations=list(itertools.permutations(raw));rng.shuffle(permutations)
  for order in permutations:
   orders.append(order)
   for arm in order:raw[arm].append(run(arm))
  if(block+1)%24==0:print(name,'completed rounds',(block+1)*2,flush=True)
 assert sha(binary)==fixture_sha
 return {'case':name,'fixture_sha256':fixture_sha,'cpu':cpu,'raw':raw,'orders':orders}
def main():
 plan=json.loads((w/'timing-plan.json').read_text());assert not(w/'timing-result.json').exists()
 assert json.loads((w/'gates-passed.json').read_text())['all_four_gates_passed']
 assert not json.loads((w/'breadth-review.json').read_text())['semantic_review_pending']
 unexpected=[]
 for p in Path('/proc').iterdir():
  if not p.name.isdigit()or int(p.name)==os.getpid():continue
  try:args=(p/'cmdline').read_bytes().decode(errors='replace').split('\0')
  except OSError:continue
  if args and Path(args[0]).name not in ['bash','timeout','rg']and any(prefix in ' '.join(args)for prefix in ['/home/mahaloz/kwt/angr-rust-','/tmp/kuna-rust-','/home/mahaloz/kwt/angr-fnptr-namespace','/tmp/kuna-fnptr-namespace','/home/mahaloz/kwt/angr-base-1d5f','/tmp/kuna-dwarf-namespace','/home/mahaloz/kwt/angr-dwarf-namespace']):unexpected.append({'pid':int(p.name),'args':args})
 assert not unexpected,unexpected
 manifests={}
 for label,name in [('current',w/'production-source-before-validation.json'),('parent',w/'parent-source-before-timing.json')]:
  data=json.loads(name.read_text());files=data if label=='current'else data['files'];root=Path(plan['root']if label=='current'else plan['base'])
  assert all(sha(root/path)==digest for path,digest in files.items());manifests[label]=sha(name)
 binaries={arm:Path(plan['base']if arm=='parent'else plan['root'])/'decompiler/target/release/kuna'for arm in plan['arms']};digests={arm:sha(path)for arm,path in binaries.items()}
 assert digests['candidate']==plan['build']['new_cli_sha256'] and digests['parent']==plan['build']['parent_cli_sha256']
 spec_path=Path(plan['compiled_spec_verification']['manifest']);specs=json.loads(spec_path.read_text())
 assert sha(spec_path)==plan['compiled_spec_verification']['sha256']
 def check_specs():
  return {str(root):all(sha(root/'specs'/name)==digest for name,digest in specs.items())for root in [Path(plan['base']),Path(plan['root'])]}
 specs_before=check_specs();assert all(specs_before.values())
 assert not subprocess.check_output(['git','status','--porcelain'],cwd=plan['base'])
 allowed=os.sched_getaffinity(0);pairs=set()
 for cpu in allowed:
  pair=tuple(map(int,Path(f'/sys/devices/system/cpu/cpu{cpu}/topology/thread_siblings_list').read_text().strip().split(',')))
  if len(pair)==2 and set(pair)<=allowed and not(set(pair)&set(plan['lightwork_reservation_amendment']['reserved_cpus'])):pairs.add(pair)
 before=sample();time.sleep(5);after=sample()
 idle={c:100*(after['cpus'][c][3]-before['cpus'][c][3])/sum(b-a for a,b in zip(before['cpus'][c],after['cpus'][c]))for pair in pairs for c in pair}
 selected=sorted(pairs,key=lambda pair:(min(idle[c]for c in pair),sum(idle[c]for c in pair),pair),reverse=True)[:len(plan['cases'])]
 assignments={row[0]:{'cpu':max(pair),'sibling':min(pair)}for row,pair in zip(plan['cases'],selected)}
 (w/'timing-cpus.json').write_text(json.dumps({'assignments':assignments,'idle_pct':idle,'unexpected_owned_processes':unexpected},indent=2)+'\n')
 os.sched_setaffinity(0,allowed-{cpu for pair in selected for cpu in pair});telemetry=[sample()];done=threading.Event()
 def monitor():
  while not done.wait(.25):telemetry.append(sample())
 thread=threading.Thread(target=monitor);thread.start()
 try:
  with ProcessPoolExecutor(max_workers=len(selected),mp_context=multiprocessing.get_context('spawn'))as pool:
   records=list(pool.map(case,[(plan,row,index,assignments[row[0]]['cpu'])for index,row in enumerate(plan['cases'])]))
 finally:
  done.set();thread.join();telemetry.append(sample())
  with gzip.open(w/'timing-cpu-samples.json.gz','wt')as out:json.dump(telemetry,out)
 assert all(sha(path)==digests[arm]for arm,path in binaries.items())
 specs_after=check_specs();assert all(specs_after.values())
 for label,name in [('current',w/'production-source-before-validation.json'),('parent',w/'parent-source-before-timing.json')]:
  data=json.loads(name.read_text());files=data if label=='current'else data['files'];root=Path(plan['root']if label=='current'else plan['base'])
  assert sha(name)==manifests[label]and all(sha(root/path)==digest for path,digest in files.items())
 for index,record in enumerate(records):
  raw=record['raw'];metrics=['cpu_ms','wall_ms'];medians={arm:{metric:statistics.median(x[metric]for x in rows)for metric in metrics}for arm,rows in raw.items()}
  delta={m:100*(medians['candidate'][m]/medians['parent'][m]-1)for m in metrics}
  keys=list(itertools.product(plan['arms'],metrics));blocks=[[sum(raw[arm][i][metric]for i in range(start,start+2))for arm,metric in keys]for start in range(0,plan['samples_per_arm'],2)]
  rng=random.Random(plan['seed']+1000+index);distributions={m:[]for m in metrics}
  for _ in range(plan['bootstrap_resamples']):
   totals=[0.0]*4
   for _ in blocks:
    chosen=rng.choice(blocks)
    for j,value in enumerate(chosen):totals[j]+=value
   for j,metric in enumerate(metrics):distributions[metric].append(100*(totals[2+j]/totals[j]-1))
  upper={}
  for metric,values in distributions.items():
   values.sort();position=plan['upper_quantile']*(len(values)-1);lo=int(position);hi=min(lo+1,len(values)-1);upper[metric]=values[lo]+(position-lo)*(values[hi]-values[lo])
  record.update(medians=medians,median_delta_pct=delta,confidence_upper_mean_delta_pct=upper,speed_budget_passed=all(value<=plan['budget_pct']for value in list(delta.values())+list(upper.values())))
  print('FINAL DEFAULT BOUNDS',record['case'],delta,upper,record['speed_budget_passed'],flush=True)
 result={'plan':plan,'completed_at':datetime.datetime.now(datetime.timezone.utc).isoformat(),'binary_sha256':digests,'source_manifest_sha256':manifests,'compiled_specs':{'count':len(specs),'before':specs_before,'after':specs_after},'parent_commit':subprocess.check_output(['git','rev-parse','HEAD'],cwd=plan['base'],text=True).strip(),'all_default_comparisons_passed':all(record['speed_budget_passed']for record in records),'default_on_accepted':all(record['speed_budget_passed']for record in records),'records':records}
 (w/'timing-result.json').write_text(json.dumps(result,indent=2)+'\n')
 assert result['all_default_comparisons_passed'],'Default DWARF namespace speed comparison unresolved; no unchanged-code repeat.'
 print('DEFAULT DWARF NAMESPACE SPEED COMPARISON PASSED',flush=True)
if __name__=='__main__':main()
