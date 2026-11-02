from pathlib import Path
from concurrent.futures import ThreadPoolExecutor
import subprocess,os,time,json,hashlib,signal,re
main=Path('/home/mahaloz/kwt/angr-rust-panic-main');rev='69fad3ce4cbdbb3232680938252bd75c2bec5a68';prior='f8746b9730b525a065c89c26e358ffd7c2328bc7'
roots=[Path('/home/mahaloz/kwt/angr-rust-fmt-'+slug)for slug in ['pieces','str','write']]
oldwork=[Path('/tmp/kuna-rust-fmt-'+slug)for slug in ['pieces-f874','str','write']]
work=[Path('/tmp/kuna-rust-fmt-'+slug+'-69fa')for slug in ['pieces','str','write']]
while 'ALL CANDIDATE-INDEX CORRECTNESS AND COMPLETE OUTPUT EQUIVALENCE PASSED'not in Path('/tmp/kuna-angr-reference/validate-candidate-index.log').read_text():time.sleep(5)
files=subprocess.check_output(['git','diff','--name-only',prior,rev],cwd=main,text=True).splitlines()
for index,(r,old,w)in enumerate(zip(roots,oldwork,work)):
 w.mkdir(exist_ok=True)
 backup=old.with_name(old.name+'-pre69fa');assert not backup.exists();subprocess.run(['cp','--reflink=auto','-a',str(old),str(backup)],check=True)
 for name in files:
  if name in ['decompiler/crates/kuna-base/src/xml.rs','docs/baseline-stages.json']:continue
  previous=subprocess.run(['git','show',prior+':'+name],cwd=main,capture_output=True)
  if previous.returncode==0:assert (r/name).read_bytes()==previous.stdout,('unexpected local edit',str(r),name)
  else:assert not(r/name).exists(),('unexpected local file',str(r),name)
  (r/name).parent.mkdir(parents=True,exist_ok=True);(r/name).write_bytes(subprocess.check_output(['git','show',rev+':'+name],cwd=main))
 p=r/'decompiler/crates/kuna-base/src/xml.rs';s=p.read_text();count=644+index
 s,n=re.subn(r'        assert_eq!\(count, \d+, "corpus file count drifted"\);','        // and kuna-wrapsplit + kuna-wrapsplit-be / accesses that run past the top of their space\n        assert_eq!(count, '+str(count)+', "corpus file count drifted");',s);assert n==1;p.write_text(s)
 (w/'integration-source.json').write_text(json.dumps({'main_commit':rev,'previous_main':prior,'changed_main_paths':files,'corpus_files':count,'previous_evidence':str(backup)},indent=2)+'\n')
 print(r.name,'main source integrated',flush=True)
env=os.environ|{'PATH':'/tmp/kuna-toolchains/go1.27.3/go/bin:'+os.environ['PATH'],'CARGO_INCREMENTAL':'0','CARGO_PROFILE_DEV_DEBUG':'0','CARGO_PROFILE_TEST_DEBUG':'0','CARGO_BUILD_JOBS':'8','RUST_TEST_THREADS':'8'}
def run(r,w,cmd,log,cwd=None):
 with(w/log).open('w')as f:
  p=subprocess.Popen(cmd,cwd=cwd or r,env=env|{'KUNA_SPECS':str(r/'specs'),'SLEIGHHOME':str(r/'specs'),'PYTHONPATH':str(r)},stdout=f,stderr=subprocess.STDOUT,start_new_session=True)
  while p.poll()is None:
   m={s.split(':')[0]:int(s.split()[1])for s in Path('/proc/meminfo').read_text().splitlines()}
   if 1-m['MemAvailable']/m['MemTotal']>=.9:
    os.killpg(p.pid,signal.SIGTERM);p.wait();(w/'memory-limit-hit.json').write_text(json.dumps(m));raise RuntimeError('90% RAM; own process stopped; user help required.')
   time.sleep(1)
  assert p.returncode==0,(r.name,log,p.returncode)
 print(r.name,log,'PASS',flush=True)
def feature(index):
 r,w=roots[index],work[index]
 run(r,w,['cargo','build','--release','-p','kuna-cli','-p','kuna-console','-p','kuna-harness'],'build.log',r/'decompiler')
 run(r,w,['decompiler/target/release/kuna','test','--datatests','--datatests-dir','tests/stages','--save-baseline','docs/baseline-stages.json'],'record-stages.log')
 count=[3523,3531,3541][index];assert json.loads((r/'docs/baseline-stages.json').read_text())['data_footer']==[count,count]
 files=subprocess.check_output(['git','ls-files','--cached','--others','--exclude-standard','decompiler','tests','specs','VERSION','Makefile','docs/spec','docs/options.md'],cwd=r,text=True).splitlines()
 source={p:hashlib.sha256((r/p).read_bytes()).hexdigest()for p in files if(r/p).is_file()}
 (w/'production-source-before-validation.json').write_text(json.dumps(source,indent=2)+'\n')
 def gates():
  for target,log in [('test','original.log'),('test-stages','stages.log'),('test-tools','tools.log'),('lint','lint.log'),('check-spec','spec.log')]:run(r,w,['make',target],log)
  run(r,w,['decompiler/target/release/kuna','catalog','--check'],'catalog.log')
  run(r,w,['python3','-m','scripts.repipe.counters','--check'],'counters.log')
 with ThreadPoolExecutor(max_workers=2)as pool:
  fs=[pool.submit(run,r,w,['make','rust-test'],'workspace-final.log'),pool.submit(gates)]
  for f in fs:f.result()
 assert all(hashlib.sha256((r/p).read_bytes()).hexdigest()==h for p,h in source.items())
 subprocess.run(['rm','-rf',str(r/'decompiler/target/debug')],check=True)
 (w/'gates-passed.json').write_text(json.dumps({'all_four_gates_passed':True,'additional_tools_lint_catalog_counters_passed':True,'original_assertions':675,'stage_assertions':count,'production_files_unchanged':len(source),'release_sha256':hashlib.sha256((r/'decompiler/target/release/kuna').read_bytes()).hexdigest()},indent=2)+'\n')
 print(r.name,'ALL 69FA GATES PASSED',flush=True)
with ThreadPoolExecutor(max_workers=3)as pool:
 fs=[pool.submit(feature,index)for index in range(3)]
 for f in fs:f.result()
print('ALL THREE FEATURE 69FA INTEGRATION GATES PASSED',flush=True)
