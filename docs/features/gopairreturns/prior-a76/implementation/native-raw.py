from common import *
old=w.parent/'integration-350';deps=r/'decompiler/target/release/deps';lib=max(deps.glob('libkuna_console-*.rlib'),key=lambda p:p.stat().st_mtime);sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();digest=sha(r/'decompiler/target/release/kuna')
for variant,source in [('explicit',old/'native-raw/capture.rs'),('inferred',old/'inferred-raw-probe/unlocked.rs')]:
 out=w/('native-raw-'+variant);out.mkdir(exist_ok=False);(out/'capture.rs').write_text(source.read_text());p=subprocess.run(['rustc','--edition=2021','-L','dependency='+str(deps),'--extern','kuna_console='+str(lib),str(out/'capture.rs'),'-o',str(out/'capture')],capture_output=True,text=True);(out/'capture-build.log').write_text(p.stdout+p.stderr);assert p.returncode==0,p.stderr
 for version in ['go1.22.5','go1.27.3']:
  for lang in ['c','go']:
   p=subprocess.run([str(out/'capture'),str(r/'tests/fixtures/go-pair-results'/(version+'-pair')),str(r/'specs'),lang],capture_output=True,text=True,check=True);(out/(version+'-'+lang+'.body')).write_text(p.stdout);(out/(version+'-'+lang+'.log')).write_text(p.stderr)
 (out/'capture-build.json').write_text(json.dumps({'variant':variant,'library':str(lib),'library_sha256':sha(lib),'capture_source_sha256':sha(out/'capture.rs'),'capture_binary_sha256':sha(out/'capture'),'cli_sha256':digest},indent=2)+'\n');(out/'execute.py').write_text((old/'native-raw/execute.py').read_text());run(r,out,['python3',str(out/'execute.py')],'execution.log')
assert digest==sha(r/'decompiler/target/release/kuna');print('PASS explicit and inferred opaque complete callers: 560 comparisons',flush=True)
