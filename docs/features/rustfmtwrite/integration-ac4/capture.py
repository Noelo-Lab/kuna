from pathlib import Path
from concurrent.futures import ThreadPoolExecutor
import hashlib,json,os,subprocess,time,signal
r=Path('/home/mahaloz/kwt/angr-rust-fmt-write-main');base=Path('/home/mahaloz/kwt/angr-rust-fmt-write');w=Path('/tmp/kuna-rust-fmt-write-ac4');old=Path('/tmp/kuna-rust-fmt-write-optin');out=w/'captures';out.mkdir(exist_ok=True)
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
assert 'build.log PASS' in (w/'validate.log').read_text()
exe=r/'decompiler/target/release/kuna';digest=sha(exe)
assert sha(base/'decompiler/target/release/kuna')=='ebff674185bf38266bea0adc10afda33a0bba10a699f09be32a1ab9b09e9a923'
expected={'decompiler/crates/kuna-cli/src/disassemble.rs','decompiler/crates/kuna-cli/src/disassemble/provenance.rs','decompiler/crates/kuna-cli/tests/read_provenance_cli.rs','docs/cli.md','docs/spec/01-program-prep.md'}
changed=set(subprocess.check_output(['git','diff','--name-only','a874bff7a12391fe05867088692c1a23d8d3c4fe'],cwd=r,text=True).splitlines());assert changed==expected,changed
receipt=json.loads((old/'breadth/record.json').read_text());rows=[]
fixtures={'rust':Path('/tmp/kuna-angr-reference/binaries/rust-fmtdemo-1931'),'write-native':r/'decompiler/crates/kuna-analysis/tests/fixtures/rust_fmt_write_real_x86_64'}
for label,binary in fixtures.items():
 record=next(f for f in receipt['fixtures']if f['fixture']==label)
 assert sha(binary)==record['fixture_sha256']
 for capture in record['captures']:
  if capture['mode']=='off':continue
  rows.append((label,binary,capture))
def run(row):
 label,binary,prior=row;language=prior['language'];mode=prior['mode'];previous=Path(prior['fresh_file']);assert sha(previous)==prior['fresh_sha256']
 path=out/f'{label}-{language}-{mode}.json';assert not path.exists()
 cmd=[str(exe),'decompile-all',str(binary),'--language',language,'--json','--jobs','1','--max-fn-seconds','600','--option','godwarfsig','on']
 if mode=='on':cmd+=['--option','rustfmtwrite','on']
 with path.open('w')as output,path.with_suffix('.log').open('w')as log:
  p=subprocess.Popen(cmd,cwd=r,env=os.environ|{'KUNA_SPECS':str(r/'specs')},stdout=output,stderr=log,start_new_session=True)
  while p.poll()is None:
   m={s.split(':')[0]:int(s.split()[1])for s in Path('/proc/meminfo').read_text().splitlines()}
   if 1-m['MemAvailable']/m['MemTotal']>=.9:
    os.killpg(p.pid,signal.SIGTERM);p.wait();(w/'memory-limit-hit.json').write_text(json.dumps(m));raise RuntimeError('90% RAM; own work stopped; user help required.')
   time.sleep(1)
  assert p.returncode==0,(label,language,mode,p.returncode)
 a=json.loads(previous.read_text())['functions'];b=json.loads(path.read_text())['functions']
 assert a==b,(label,language,mode)
 print(label,language,mode,len(b),'FULL RECORDS IDENTICAL',flush=True)
 return {'fixture':label,'language':language,'mode':mode,'functions':len(b),'all_complete_function_records_identical':True,'fresh_sha256':sha(path),'prior_sha256':sha(previous)}
with ThreadPoolExecutor(max_workers=7)as pool:result=list(pool.map(run,rows))
assert sha(exe)==digest
(w/'capture-equivalence.json').write_text(json.dumps({'kuna_sha256':digest,'prior_kuna_sha256':receipt['new_kuna_sha256'],'main_commit':'ac4cd3aeef7565684fb1547101d61ee09e75cd6f','changed_paths':sorted(changed),'images':len(fixtures),'records':result,'all_complete_function_records_identical':True,'scope':'Fresh complete records for the modern reference image and native String-writer fixture. Engine, analysis and console source paths remain identical. The earlier 27-image and speed studies retain their original binary identities; no claim they were rerun on this integration build.'},indent=2)+'\n')
