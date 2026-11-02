from pathlib import Path
import hashlib,json,os,subprocess,time,signal
w=Path('/tmp/kuna-rust-io-proposal');r=Path('/home/mahaloz/kwt/angr-rust-io-proposal');out=w/'unsupported-producers';out.mkdir(exist_ok=True)
exe=r/'decompiler/target/release/kuna';digest=hashlib.sha256(exe.read_bytes()).hexdigest();rows=[]
cases=[('native','rust_fmt_io.rs',['write_unsigned','write_signed','write_flags','write_literal','write_empty','write_string','write_pair','write_dynamic','write_unknown','write_vec']),('promoted','rust_fmt_io_promoted.rs',['write_value','write_hex','write_binary','write_debug'])]
for version in ['1.90.0','1.99.1']:
 for label,source,names in cases:
  binary=out/f'{label}-{version}';source=r/'decompiler/crates/kuna-analysis/tests/fixtures'/source
  with (out/f'{label}-{version}.compile.log').open('w')as log:
   p=subprocess.Popen(['rustc','+'+version,'--edition=2021','-C','opt-level=3','-C','lto=fat','-C','codegen-units=1','-C','debuginfo=1','-C','strip=debuginfo',str(source),'-o',str(binary)],stdout=log,stderr=subprocess.STDOUT,start_new_session=True)
   while p.poll()is None:
    m={s.split(':')[0]:int(s.split()[1])for s in Path('/proc/meminfo').read_text().splitlines()}
    if 1-m['MemAvailable']/m['MemTotal']>=.9:
     os.killpg(p.pid,signal.SIGTERM);p.wait();(w/'memory-limit-hit.json').write_text(json.dumps(m));raise RuntimeError('90% RAM; own work stopped; user help required.')
    time.sleep(1)
   assert p.returncode==0,(version,label,p.returncode)
  for name in names:
   outputs=[]
   for mode in ['off','on']:
    p=subprocess.run([str(exe),'decompile',str(binary),name,'--language','rust','--option','rustfmtio',mode],cwd=r,env=os.environ|{'KUNA_SPECS':str(r/'specs')},capture_output=True,check=True)
    (out/f'{label}-{version}-{name}-{mode}.rs').write_bytes(p.stdout);(out/f'{label}-{version}-{name}-{mode}.log').write_bytes(p.stderr);outputs.append(p.stdout)
   assert outputs[0]==outputs[1] and b'use std::io::Write as _;'not in outputs[1],(version,label,name)
  rows.append({'compiler':version,'fixture':label,'fixture_sha256':hashlib.sha256(binary.read_bytes()).hexdigest(),'source_sha256':hashlib.sha256(source.read_bytes()).hexdigest(),'functions':names,'all_on_off_outputs_identical':True})
  print(version,label,len(names),'UNSUPPORTED PRODUCER CONTROLS PASS',flush=True)
assert hashlib.sha256(exe.read_bytes()).hexdigest()==digest
(w/'unsupported-producer-controls.json').write_text(json.dumps({'kuna_sha256':digest,'rows':rows,'functions':sum(len(x['functions'])for x in rows)},indent=2)+'\n')
