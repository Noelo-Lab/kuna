from pathlib import Path
import subprocess,time,os,json,hashlib,shutil,signal
w=Path('/tmp/kuna-rust-fmt-write');nw=Path('/tmp/kuna-rust-fmt-write-69fa');parent=Path('/tmp/kuna-rust-fmt-str')
while not(nw/'gates-passed.json').exists()or not(parent/'main-69fa-equivalence.json').exists()or not(w/'breadth/write-native-record.json').exists():time.sleep(5)
for name in ['production-source-before-validation.json','original.log','stages.log','workspace-final.log','tools.log','lint.log','spec.log','catalog.log','counters.log','gates-passed.json']:
 shutil.copy2(nw/name,w/name)
def run(script,log,args=[]):
 with(w/log).open('w')as f:
  p=subprocess.Popen(['python3','-u',str(w/script),*args],stdout=f,stderr=subprocess.STDOUT,start_new_session=True)
  while p.poll()is None:
   m={line.split(':')[0]:int(line.split()[1])for line in Path('/proc/meminfo').read_text().splitlines()}
   if 1-m['MemAvailable']/m['MemTotal']>=.9:
    os.killpg(p.pid,signal.SIGTERM);p.wait();(w/'memory-limit-hit.json').write_text(json.dumps(m));raise RuntimeError('90% RAM; own capture stopped; user help required.')
   time.sleep(1)
  assert p.returncode==0,(script,p.returncode)
 print(script,'PASS',flush=True)
run('breadth.py','breadth-69fa-capture.log',['--reuse-existing'])
run('type-audit.py','type-audit.log')
run('postreview.py','postreview.log')
print('WRITE RECOVERY READY FOR DECLARED TIMINGS',flush=True)
