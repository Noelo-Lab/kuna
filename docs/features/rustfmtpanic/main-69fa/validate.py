from pathlib import Path
from concurrent.futures import ThreadPoolExecutor
import os,subprocess,json,hashlib,time,signal
w=Path('/tmp/kuna-rust-panic-69fa');r=Path('/home/mahaloz/kwt/angr-rust-panic-main')
env=os.environ|{'PATH':'/tmp/kuna-toolchains/go1.27.3/go/bin:'+os.environ['PATH'],'CARGO_INCREMENTAL':'0','CARGO_PROFILE_DEV_DEBUG':'0','CARGO_PROFILE_TEST_DEBUG':'0','CARGO_BUILD_JOBS':'8','RUST_TEST_THREADS':'8','KUNA_SPECS':str(r/'specs'),'SLEIGHHOME':str(r/'specs'),'PYTHONPATH':str(r)}
def run(command,log,cwd=r):
 with(w/log).open('w')as f:
  p=subprocess.Popen(command,cwd=cwd,env=env,stdout=f,stderr=subprocess.STDOUT,start_new_session=True)
  while p.poll()is None:
   m={s.split(':')[0]:int(s.split()[1])for s in Path('/proc/meminfo').read_text().splitlines()}
   if 1-m['MemAvailable']/m['MemTotal']>=.9:
    os.killpg(p.pid,signal.SIGTERM);p.wait();(w/'memory-limit-hit.json').write_text(json.dumps(m));raise RuntimeError('RAM reached 90%; own process group stopped; user help required.')
   time.sleep(1)
  assert p.returncode==0,(log,p.returncode)
 print(log,'PASS',flush=True)
run(['cargo','build','--release','-p','kuna-cli','-p','kuna-console','-p','kuna-harness'],'build.log',r/'decompiler')
run(['decompiler/target/release/kuna','test','--datatests','--datatests-dir','tests/stages','--save-baseline','docs/baseline-stages.json'],'record-stages.log')
assert json.loads((r/'docs/baseline-stages.json').read_text())['data_footer']==[3516,3516]
subprocess.run(['git','add','docs/baseline-stages.json'],cwd=r,check=True)
files=subprocess.check_output(['git','ls-files','--cached','--others','--exclude-standard','decompiler','tests','specs','VERSION','Makefile','docs/spec','docs/options.md'],cwd=r,text=True).splitlines()
source={p:hashlib.sha256((r/p).read_bytes()).hexdigest()for p in files if(r/p).is_file()}
(w/'production-source-before-validation.json').write_text(json.dumps(source,indent=2)+'\n')
def gates():
 for target,log in [('test','original.log'),('test-stages','stages.log'),('test-tools','tools.log'),('lint','lint.log'),('check-spec','spec.log')]:run(['make',target],log)
 run(['decompiler/target/release/kuna','catalog','--check'],'catalog.log')
 run(['python3','-m','scripts.repipe.counters','--check'],'counters.log')
with ThreadPoolExecutor(max_workers=2)as pool:
 fs=[pool.submit(run,['make','rust-test'],'workspace-final.log'),pool.submit(gates)]
 for f in fs:f.result()
assert all(hashlib.sha256((r/p).read_bytes()).hexdigest()==h for p,h in source.items())
subprocess.run(['rm','-rf',str(r/'decompiler/target/debug')],check=True)
print('ALL PARENT 69FA INTEGRATION CHECKS PASSED',flush=True)
