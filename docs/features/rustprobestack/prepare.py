from common import *
import datetime
base=Path('/home/mahaloz/kwt/angr-base-c5fc')
run(r,w,['cargo','build','--release','-p','kuna-cli','-p','kuna-console','-p','kuna-harness'],'build-final.log',r/'decompiler')
k=r/'decompiler/target/release/kuna'
p=subprocess.run([str(k),'catalog','--markdown'],cwd=r,env=env,capture_output=True,text=True);assert p.returncode==0;(r/'docs/options.md').write_text(p.stdout)
cap=w/'phase_catalog.json';commands=f'openfile write {cap}\nphase catalog\nclosefile\nquit\n'
p=subprocess.run([str(r/'decompiler/target/release/decomp_dbg'),'-s',str(r/'specs')],cwd=r,env=env,input=commands,capture_output=True,text=True);(w/'catalog-generation.log').write_text(p.stdout+p.stderr);assert p.returncode==0
s=cap.read_text();assert len(json.loads(s))==370 and '"current"'not in s
(r/'decompiler/crates/kuna-decomp/tests/fixtures/phase_catalog.json').write_text(s)
run(r,w,[str(k),'test','--datatests','--datatests-dir','tests/stages','--name','ghangr-rust-probestack'],'stage-focused.log')
run(r,w,[str(k),'test','--datatests','--datatests-dir','tests/stages','--save-baseline','docs/baseline-stages.json'],'record-stages.log')
stages=json.loads((r/'docs/baseline-stages.json').read_text())['data_footer'];assert stages[0]==stages[1]
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
record={'at':datetime.datetime.now(datetime.timezone.utc).isoformat(),'parent_commit':'c5fcb0331dbdbbe9498237d0afe0bfaa372f9246','baseline_root':str(base),'parent_cli_sha256':sha(base/'decompiler/target/release/kuna'),'new_cli_sha256':sha(k),'stage_assertions':stages,'parent_artifact_provenance':'base-update.json: new independent release build from clean c5fcb033 main with frozen compiled specs.'}
(w/'build.json').write_text(json.dumps(record,indent=2)+'\n');print(record,flush=True)
