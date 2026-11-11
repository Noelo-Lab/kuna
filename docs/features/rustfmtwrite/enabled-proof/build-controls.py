from pathlib import Path
from concurrent.futures import ThreadPoolExecutor
import subprocess,json,os
w=Path('/tmp/kuna-rust-fmt-write');r=Path('/home/mahaloz/kwt/angr-rust-fmt-write');source=r/'decompiler/crates/kuna-analysis/tests/fixtures/rust_fmt_write.rs'
def build(version):
 binary=w/('native-control-'+version)
 result=subprocess.run(['rustc','+'+version,'--edition=2021','-C','opt-level=3','-C','lto=fat','-C','codegen-units=1','-C','debuginfo=1','-C','strip=debuginfo',str(source),'-o',str(binary)],capture_output=True)
 (w/('native-control-'+version+'.compile.log')).write_bytes(result.stderr);assert result.returncode==0,(version,result.stderr.decode())
 names=['write_unsigned','write_signed','write_flags','write_literal','write_empty','write_string','write_pair','write_custom','write_dynamic','write_unknown','write_reject']
 for name in names:
  captures=[]
  for flag in ['off','on']:
   result=subprocess.run([str(r/'decompiler/target/release/kuna'),'decompile',str(binary),name,'--language','rust','--option','rustfmtwrite',flag],cwd=r,env=os.environ|{'KUNA_SPECS':str(r/'specs')},capture_output=True,check=True)
   (w/f'{version}-{name}-{flag}.rs').write_bytes(result.stdout);captures.append(result.stdout)
  assert captures[0]==captures[1],(version,name)
 print(version,'11 unsupported-producer calls unchanged',flush=True)
 return {'version':version,'functions':names,'all_on_off_outputs_identical':True}
with ThreadPoolExecutor(max_workers=2)as pool:records=list(pool.map(build,['1.90.0','1.99.1']))
(w/'unsupported-producer-controls.json').write_text(json.dumps(records,indent=2)+'\n')
