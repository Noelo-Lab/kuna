from pathlib import Path
import subprocess,os,time,json,hashlib,signal
r=Path("/home/mahaloz/kwt/angr-rust-store-conversions");w=Path("/tmp/kuna-rust-store-conversions")
env=os.environ|{'PATH':'/tmp/kuna-toolchains/go1.27.3/go/bin:'+os.environ['PATH'],'CARGO_INCREMENTAL': '0', 'CARGO_PROFILE_DEV_DEBUG': '0', 'CARGO_PROFILE_TEST_DEBUG': '0', 'CARGO_BUILD_JOBS': '8', 'RUST_TEST_THREADS': '8'}
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
