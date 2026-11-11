from pathlib import Path
import os,sys,time,json,subprocess,resource,statistics,random,itertools,hashlib,threading,runpy,shutil
w=Path('/tmp/kuna-rust-panic-f874');root=Path('/home/mahaloz/kwt/angr-rust-panic-main');base=Path('/home/mahaloz/kwt/angr-rust-panic-base-f874')
plan=json.loads((w/'timing-randomized-plan.json').read_text());assert plan['samples_per_arm']==192 and plan['comparisons']==20
allowed=os.sched_getaffinity(0);pairs=set()
for cpu in sorted(allowed):
 pair=tuple(map(int,Path(f'/sys/devices/system/cpu/cpu{cpu}/topology/thread_siblings_list').read_text().strip().split(',')))
 if len(pair)==2 and set(pair)<=allowed:pairs.add(pair)
def sample():
 return {'time':time.monotonic(),'cpus':{int(p[0][3:]):list(map(int,p[1:]))for line in Path('/proc/stat').read_text().splitlines()if (p:=line.split())and p[0].startswith('cpu')and p[0][3:].isdigit()}}
a=sample();time.sleep(5);b=sample();idle={c:100*(b['cpus'][c][3]-a['cpus'][c][3])/sum(y-x for x,y in zip(a['cpus'][c],b['cpus'][c]))for pair in pairs for c in pair}
pair=max(sorted(pairs),key=lambda p:(min(idle[c]for c in p),sum(idle[c]for c in p)));cpu=max(pair)
os.environ['KUNA_TIMING_CPU']=str(cpu);os.environ['KUNA_TIMING_MIN_IDLE']='0'
runpy.run_path(str(w/'audit-idle.py'),run_name='__main__')
attempt=w/'timing-randomized-attempts'/time.strftime('%Y%m%d-%H%M%S',time.gmtime());attempt.mkdir(parents=True)
shutil.copy2(w/'timing-randomized-plan.json',attempt/'plan.json')
shutil.copy2(w/'cpu-audit.json',attempt/'cpu-audit.json');shutil.copy2(w/'owned-process-audit.json',attempt/'owned-process-audit.json')
(attempt/'cpu-selection.json').write_text(json.dumps({'selected_pair':pair,'idle_pct':idle,'method':plan['cpu_selection']},indent=2)+'\n')
binaries={side:checkout/'decompiler/target/release/kuna'for side,checkout in [('baseline',base),('patched',root)]}
digests={side:hashlib.sha256(p.read_bytes()).hexdigest()for side,p in binaries.items()}
fixture=root/'decompiler/crates/kuna-analysis/tests/fixtures/rust_fmt_panic_real_x86_64'
rows=[('scalar-c',root/'tests/fixtures/go-output/output-c','choose','c'),('panic-literal',fixture,'panic_literal','rust'),('panic-integers',fixture,'panic_reorder','rust'),('rust-large-control',Path('/tmp/kuna-angr-reference/binaries/rust-fmtdemo-1931'),'0x13df0','rust'),('go-control',Path('/tmp/kuna-go-dwarf-signatures/native1273'),'main.main','go')]
telemetry=[sample()];done=threading.Event()
def monitor():
 while not done.wait(0.25):telemetry.append(sample())
thread=threading.Thread(target=monitor);thread.start();records=[]
try:
 for case_index,(name,binary,selector,language)in enumerate(rows):
  raw={side:[]for side in ['baseline','off','patched']};orders=[];rng=random.Random(plan['seed']+case_index)
  fixture_digest=hashlib.sha256(binary.read_bytes()).hexdigest()
  def run(side):
   executable=binaries['baseline'if side=='baseline'else'patched'];options=['--option','godwarfsig','on']if language=='go'else[]
   if side=='off':options+=['--option','rustfmtpanic','off']
   cmd=['taskset','-c',str(cpu),str(executable),'decompile',str(binary),selector,'--language',language]+options
   before=resource.getrusage(resource.RUSAGE_CHILDREN);started=time.perf_counter()
   subprocess.run(cmd,cwd=root,env=os.environ|{'KUNA_SPECS':str(root/'specs')},stdout=subprocess.DEVNULL,stderr=subprocess.DEVNULL,check=True)
   wall=time.perf_counter()-started;after=resource.getrusage(resource.RUSAGE_CHILDREN)
   return {'wall_ms':wall*1000,'cpu_ms':1000*((after.ru_utime+after.ru_stime)-(before.ru_utime+before.ru_stime))}
  for _ in range(plan['warmups_per_arm']):
   order=list(raw);rng.shuffle(order)
   for side in order:run(side)
  for block in range(plan['samples_per_arm']//6):
   permutations=list(itertools.permutations(raw));rng.shuffle(permutations)
   for order in permutations:
    orders.append(order)
    for side in order:raw[side].append(run(side))
   if (block+1)%8==0:print(name,'completed rounds',(block+1)*6,flush=True)
  assert hashlib.sha256(binary.read_bytes()).hexdigest()==fixture_digest
  medians={s:{m:statistics.median(x[m]for x in xs)for m in ['cpu_ms','wall_ms']}for s,xs in raw.items()}
  means={s:{m:statistics.mean(x[m]for x in xs)for m in ['cpu_ms','wall_ms']}for s,xs in raw.items()}
  deltas={a+'_to_patched':{m:100*(medians['patched'][m]/medians[a][m]-1)for m in ['cpu_ms','wall_ms']}for a in ['baseline','off']}
  record={'method':'balanced_randomized_block_bootstrap','plan':plan,'baseline_commit':'f8746b9730b525a065c89c26e358ffd7c2328bc7','fixture':name,'sha256':fixture_digest,'selector':selector,'language':language,'samples':plan['samples_per_arm'],'warmups':plan['warmups_per_arm'],'affinity_cpu':cpu,'binaries_sha256':digests,'medians':medians,'means':means,'delta_pct':deltas,'raw':raw,'orders':orders}
  records.append(record);(attempt/f'timing-{name}.json').write_text(json.dumps(record,indent=2)+'\n');print(name,'medians',deltas,flush=True)
finally:
 done.set();thread.join();telemetry.append(sample());(attempt/'cpu-samples.json').write_text(json.dumps(telemetry,indent=2)+'\n')
assert all(hashlib.sha256(p.read_bytes()).hexdigest()==digests[side]for side,p in binaries.items())
def quantile(values,q):
 values=sorted(values);position=q*(len(values)-1);lo=int(position);hi=min(lo+1,len(values)-1)
 return values[lo]+(position-lo)*(values[hi]-values[lo])
for case_index,record in enumerate(records):
 raw=record['raw'];keys=[(s,m)for s in ['baseline','off','patched']for m in ['cpu_ms','wall_ms']]
 blocks=[[sum(raw[s][i][m]for i in range(start,start+6))for s,m in keys]for start in range(0,plan['samples_per_arm'],6)]
 distributions={a+'_to_patched':{m:[]for m in ['cpu_ms','wall_ms']}for a in ['baseline','off']};rng=random.Random(plan['seed']+1000+case_index)
 for _ in range(plan['bootstrap_resamples']):
  totals=[0.0]*6
  for _ in blocks:
   block=rng.choice(blocks)
   for j,x in enumerate(block):totals[j]+=x
  for side,index in [('baseline',0),('off',2)]:
   for m,j in [('cpu_ms',0),('wall_ms',1)]:distributions[side+'_to_patched'][m].append(100*(totals[4+j]/totals[index+j]-1))
 upper={a:{m:quantile(values,plan['upper_quantile'])for m,values in metrics.items()}for a,metrics in distributions.items()}
 record['confidence_upper_delta_pct']=upper;record['confidence_method']='Percentile bootstrap of 32 complete six-round blocks; simultaneous family alpha 0.05 across twenty CPU/wall comparisons.'
 record['speed_budget_passed']=all(v<=plan['budget_pct']for group in [record['delta_pct'],upper]for metrics in group.values()for v in metrics.values())
 (attempt/f'timing-{record["fixture"]}.json').write_text(json.dumps(record,indent=2)+'\n');print(record['fixture'],'upper bounds',upper,'passed',record['speed_budget_passed'],flush=True)
accepted=len(records)==5 and all(r['speed_budget_passed']for r in records)
monitor_record={'method':'balanced_randomized_block_bootstrap','attempt':str(attempt),'target_cpu':cpu,'sibling_cpu':min(pair),'plan':plan,'accepted_for_speed_review':accepted,'results':{r['fixture']:{'median_delta_pct':r['delta_pct'],'confidence_upper_delta_pct':r['confidence_upper_delta_pct'],'speed_budget_passed':r['speed_budget_passed']}for r in records}}
(attempt/'monitor.json').write_text(json.dumps(monitor_record,indent=2)+'\n');print(json.dumps(monitor_record,indent=2),flush=True)
assert accepted,'Speed bound unresolved or failed; all samples retained without replacing canonical results.'
for p in attempt.glob('timing-*.json'):shutil.copy2(p,w/p.name)
shutil.copy2(attempt/'monitor.json',w/'timing-monitor.json')
print('RANDOMIZED CONTROL TIMINGS PASSED',flush=True)
