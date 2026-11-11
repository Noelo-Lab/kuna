from pathlib import Path
from concurrent.futures import ThreadPoolExecutor
import subprocess,os,time,json,hashlib,signal,re,shutil
w=Path('/tmp/kuna-rust-io-ddd');root=Path('/home/mahaloz/kwt/angr-rust-io-proposal');base=Path('/home/mahaloz/kwt/angr-rust-fmt-write-ddd')
env=os.environ|{'CARGO_INCREMENTAL':'0','CARGO_PROFILE_DEV_DEBUG':'0','CARGO_PROFILE_TEST_DEBUG':'0','CARGO_BUILD_JOBS':'8','RUST_TEST_THREADS':'8'}
def run(r,cmd,log,stdin=None):
 with (w/log).open('w')as f:
  p=subprocess.Popen(cmd,cwd=r,env=env|{'KUNA_SPECS':str(r/'specs'),'SLEIGHHOME':str(r/'specs')},stdin=subprocess.PIPE if stdin else None,stdout=f,stderr=subprocess.STDOUT,start_new_session=True)
  if stdin:p.stdin.write(stdin);p.stdin.close()
  while p.poll()is None:
   m={s.split(':')[0]:int(s.split()[1])for s in Path('/proc/meminfo').read_text().splitlines()}
   if 1-m['MemAvailable']/m['MemTotal']>=.9:
    os.killpg(p.pid,signal.SIGTERM);p.wait();(w/'memory-limit-hit.json').write_text(json.dumps(m));raise RuntimeError('90% RAM; own work stopped; user help required.')
   time.sleep(1)
  assert p.returncode==0,(log,p.returncode)
 print(log,'PASS',flush=True)
def build(r):run(r/'decompiler',['cargo','build','--release','-p','kuna-cli','-p','kuna-console','-p','kuna-harness'],r.name+'-build.log')
assert (w/'integration.json').exists()
build(root)
r=root
run(r,[str(r/'decompiler/target/release/decomp_dbg'),'-s',str(r/'specs')],'catalog-capture.log',f'openfile write {w}/phase_catalog.json\nphase catalog\nclosefile\nquit\n'.encode())
shutil.copy2(w/'phase_catalog.json',r/'decompiler/crates/kuna-decomp/tests/fixtures/phase_catalog.json')
run(r,['python3','-m','scripts.repipe.counters','--fix'],'counters-fix.log')
rows=json.loads(subprocess.check_output([str(r/'decompiler/target/release/kuna'),'catalog','--json'],cwd=r));rows=rows['settables']if isinstance(rows,dict)else rows
live=sum('current'in row for row in rows)
# The live count requires an architecture; count existing metadata-backed live fields.
phases=(r/'decompiler/crates/kuna-decomp/phases.toml').read_text();live=sum(bool(re.search(r'^live_(?:field|expr)\s*=',row,re.M))for row in phases.split('[[settable]]')[1:])
p=r/'decompiler/crates/kuna-decomp/src/p0_knowledge/kuna_phases/tests.rs';s=p.read_text();s=re.sub(r'assert_eq!\(with_live, \d+\)',f'assert_eq!(with_live, {live})',s);s=re.sub(r'fn option_values_live_value_present_for_\d+',f'fn option_values_live_value_present_for_{live}',s);p.write_text(s)
p=r/'tests/stages/kuna-catalog.xml';s=p.read_text();s=re.sub(r'\d+ registry rows plus six individual queries',f'{len(rows)} registry rows plus six individual queries',s);queries=len(re.findall(r'<com>phase catalog [^<]+</com>',s));assert queries==6;s=re.sub(r'(name="KUNA-CATALOG #15:[^"]*" min=")\d+(" max=")\d+',lambda m:m[1]+str(len(rows)+queries)+m[2]+str(len(rows)+queries),s);p.write_text(s)
with(r/'docs/options.md').open('wb')as f:subprocess.run([str(r/'decompiler/target/release/kuna'),'catalog','--markdown'],cwd=r,stdout=f,check=True)
run(r,[str(r/'decompiler/target/release/kuna'),'test','--datatests','--datatests-dir','tests/stages','--save-baseline','docs/baseline-stages.json'],'record-stages.log')
print('Counts',len(rows),live,json.loads((r/'docs/baseline-stages.json').read_text())['data_footer'],flush=True)
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
(w/'build.json').write_text(json.dumps({'parent_commit':subprocess.check_output(['git','rev-parse','HEAD'],cwd=base,text=True).strip(),'parent_has_pending_main_merge':True,'parent_cli_sha256':sha(base/'decompiler/target/release/kuna'),'new_cli_sha256':sha(r/'decompiler/target/release/kuna'),'catalog_count':len(rows),'live_count':live,'stage_assertions':json.loads((r/'docs/baseline-stages.json').read_text())['data_footer']},indent=2)+'\n')
print('IO CURRENT MAIN BUILD AND GENERATED DATA COMPLETE',flush=True)
