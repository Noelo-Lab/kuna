from pathlib import Path
import runpy,time,json,sys,os
w=Path('/tmp/kuna-rust-panic-d850');history=w/'idle-waits';history.mkdir(exist_ok=True)
allowed=os.sched_getaffinity(0)
pairs=set()
for cpu in sorted(allowed):
 siblings=tuple(map(int,Path(f'/sys/devices/system/cpu/cpu{cpu}/topology/thread_siblings_list').read_text().strip().split(',')))
 if len(siblings)==2 and set(siblings)<=allowed:pairs.add(siblings)
assert pairs

def sample():
 return {int(p[0][3:]):list(map(int,p[1:])) for line in Path('/proc/stat').read_text().splitlines() if (p:=line.split()) and p[0].startswith('cpu') and p[0][3:].isdigit()}
for index in range(40):
 a=sample();time.sleep(45);b=sample()
 idle={cpu:100*(b[cpu][3]-a[cpu][3])/sum(y-x for x,y in zip(a[cpu],b[cpu])) for pair in pairs for cpu in pair}
 ranked=sorted(pairs,key=lambda pair:(min(idle[c]for c in pair),sum(idle[c]for c in pair)),reverse=True)
 chosen=ranked[0];stamp=time.strftime('%Y%m%d-%H%M%S',time.gmtime())
 record={'time':stamp,'selection_basis':'highest minimum physical-core sibling idle percentage over forty-five seconds; independent of benchmark results','selected_pair':chosen,'all_pairs':[{'cpus':pair,'idle_pct':[idle[c]for c in pair]}for pair in ranked]}
 (history/(stamp+'-selection.json')).write_text(json.dumps(record,indent=2)+'\n')
 if min(idle[c]for c in chosen)<95:
  print('No idle pair',stamp,chosen,[idle[c]for c in chosen],flush=True);time.sleep(30);continue
 os.environ['KUNA_TIMING_CPU']=str(max(chosen));sys.argv=[str(w/'timing-monitored.py'),'rust-large-control','go-control']
 before=set((w/'timing-attempts').glob('*'))
 try:
  print('Selected idle pair',stamp,chosen,[idle[c]for c in chosen],flush=True)
  runpy.run_path(str(w/'timing-monitored.py'),run_name='__main__')
  print('MONITORED TIMING FINISHED',flush=True);break
 except AssertionError as error:
  after=set((w/'timing-attempts').glob('*'))
  if after!=before:raise
  record['reason']=str(error)
  for name in ['cpu-audit.json','owned-process-audit.json']:
   if (w/name).exists():record[name]=json.loads((w/name).read_text())
  (history/(stamp+'-preflight.json')).write_text(json.dumps(record,indent=2)+'\n')
  print('Preflight still busy',stamp,flush=True);time.sleep(30)
else:raise RuntimeError('No idle physical-core window in the waiting period; no measurement accepted.')
