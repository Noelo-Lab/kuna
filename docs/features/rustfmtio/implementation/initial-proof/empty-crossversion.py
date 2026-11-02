from pathlib import Path
import json,socket,subprocess,hashlib
w=Path('/tmp/kuna-rust-io-proposal');rows=[]
for version in ['1.90.0','1.93.1','1.99.1']:
 for kind in ['macro','source']:
  source=w/f'empty-{kind}-crossversion.rs';binary=w/f'empty-{kind}-crossversion-{version}'
  p=subprocess.run(['rustc','+'+version,'--edition=2021','--crate-name','empty_io_probe',str(source),'-O','-o',str(binary)],capture_output=True)
  (w/f'empty-{kind}-crossversion-{version}.compile.log').write_bytes(p.stderr);assert p.returncode==0,p.stderr.decode()
  reader,writer=socket.socketpair();reader.close()
  with writer:p=subprocess.run([str(binary)],stdout=writer,stderr=subprocess.PIPE,check=True)
  results=p.stderr.decode().splitlines();assert len(results)==21 and all(x in ['true','false'] for x in results)
  rows.append({'compiler':version,'kind':kind,'error_results':results,'empty_call_error_results':[results[i]for i in [4,11,18]],'source_sha256':hashlib.sha256(source.read_bytes()).hexdigest()})
  print(version,kind,rows[-1]['empty_call_error_results'],flush=True)
(w/'empty-crossversion.json').write_text(json.dumps({'rows':rows,'scope':'Buffered broken-pipe sequence; original-source empty Arguments barrier versus unchanged initially emitted empty macro. This diagnoses refusal and compiler-version effects only.'},indent=2)+'\n')
