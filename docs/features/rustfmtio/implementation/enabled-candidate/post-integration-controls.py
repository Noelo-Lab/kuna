from pathlib import Path
from concurrent.futures import ThreadPoolExecutor
import hashlib,json,os,subprocess
w=Path('/tmp/kuna-rust-io-ddd');prior=Path('/tmp/kuna-rust-io-proposal');r=Path('/home/mahaloz/kwt/angr-rust-io-proposal');old=prior/'unsupported-producers';out=w/'post-integration-controls';out.mkdir(exist_ok=True)
receipt=json.loads((prior/'unsupported-producer-controls.json').read_text());exe=r/'decompiler/target/release/kuna';digest=hashlib.sha256(exe.read_bytes()).hexdigest()
def run(row):
 version,label=row['compiler'],row['fixture'];binary=old/f'{label}-{version}';assert hashlib.sha256(binary.read_bytes()).hexdigest()==row['fixture_sha256']
 hashes=[]
 for name in row['functions']:
  for mode in ['off','on']:
   p=subprocess.run([str(exe),'decompile',str(binary),name,'--language','rust','--option','rustfmtio',mode],cwd=r,env=os.environ|{'KUNA_SPECS':str(r/'specs')},capture_output=True,check=True)
   path=out/f'{label}-{version}-{name}-{mode}.rs';path.write_bytes(p.stdout);path.with_suffix('.log').write_bytes(p.stderr)
   prior=old/path.name;assert p.stdout==prior.read_bytes(),(version,label,name,mode)
   hashes.append({'function':name,'mode':mode,'sha256':hashlib.sha256(p.stdout).hexdigest()})
 print(version,label,len(row['functions']),'FRESH UNSUPPORTED OUTPUTS IDENTICAL',flush=True)
 return row|{'captures':hashes}
with ThreadPoolExecutor(max_workers=4)as pool:records=list(pool.map(run,receipt['rows']))
assert hashlib.sha256(exe.read_bytes()).hexdigest()==digest
(w/'post-integration-controls.json').write_text(json.dumps({'new_kuna_sha256':digest,'prior_kuna_sha256':receipt['kuna_sha256'],'all_selected_outputs_identical':True,'functions':sum(len(row['functions'])for row in records),'records':records},indent=2)+'\n')
