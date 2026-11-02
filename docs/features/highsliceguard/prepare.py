from common import *
import datetime,subprocess
base=Path('/home/mahaloz/kwt/angr-rust-empty-match');sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
run(r,w,['cargo','build','--release','-p','kuna-cli','-p','kuna-console','-p','kuna-harness'],'build-final.log',r/'decompiler')
k=r/'decompiler/target/release/kuna'
with(r/'docs/options.md').open('w')as f:subprocess.run([str(k),'catalog','--markdown'],cwd=r,stdout=f,check=True)
fixture=r/'decompiler/crates/kuna-decomp/tests/fixtures/phase_catalog.json';script=f'openfile write {fixture}\nphase catalog\nclosefile\nquit\n';p=subprocess.run([str(r/'decompiler/target/release/decomp_dbg'),'-s',str(r/'specs')],input=script,capture_output=True,text=True,cwd=r,env=env);(w/'catalog-fixture.log').write_text(p.stdout+p.stderr);assert p.returncode==0;assert len(json.loads(fixture.read_text()))==372
run(r,w,[str(k),'catalog','--check'],'catalog-focused.log')
run(r,w,[str(k),'test','--datatests','--datatests-dir','tests/stages','--name','ghangr-highslice-guard'],'stage-focused.log')
run(r,w,[str(k),'test','--datatests','--datatests-dir','tests/stages','--save-baseline','docs/baseline-stages.json'],'record-stages.log')
stages=json.loads((r/'docs/baseline-stages.json').read_text())['data_footer'];assert stages==[3670,3670]
record={'at':datetime.datetime.now(datetime.timezone.utc).isoformat(),'parent_commit':'af49462d8c34c3047d3b6a54943e02658f0ca11b','baseline_root':str(base),'parent_cli_sha256':sha(base/'decompiler/target/release/kuna'),'new_cli_sha256':sha(k),'stage_assertions':stages,'option':'highsliceguard','default':'off','disabled_in_all_presets':True,'enabled_study_passed':False};(w/'build.json').write_text(json.dumps(record,indent=2)+'\n');print(record,flush=True)
