from pathlib import Path
from concurrent.futures import ThreadPoolExecutor
import subprocess,os,time,json,hashlib,signal,re,shutil
r=Path("/home/mahaloz/kwt/angr-narrow-table-guard");w=Path("/tmp/kuna-highslice-optin")
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
assert (w/'build.json').exists()
stages=json.loads((w/'build.json').read_text())['stage_assertions'];assert stages[0]==stages[1]
assert json.loads((r/'docs/baseline-stages.json').read_text())['data_footer']==stages
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
debug=r/'decompiler/target/debug';assert debug.resolve()==debug and not debug.is_symlink();shutil.rmtree(debug)
(w/'gates-passed.json').write_text(json.dumps({'all_four_gates_passed':True,'additional_tools_lint_catalog_counters_passed':True,'original_assertions':675,'stage_assertions':stages[0],'production_files_unchanged':len(source),'release_sha256':hashlib.sha256((r/'decompiler/target/release/kuna').read_bytes()).hexdigest()},indent=2)+'\n')
print(r.name,'ALL HIGH SLICE GUARD VALIDATION GATES PASSED',flush=True)
print("HIGH SLICE GUARD VALIDATION ALL GATES PASSED",flush=True)
