from common import *
import datetime
base=Path('/home/mahaloz/kwt/angr-base-db2d')
run(r,w,['cargo','build','--release','-p','kuna-cli','-p','kuna-console','-p','kuna-harness'],'build-final.log',r/'decompiler')
k=r/'decompiler/target/release/kuna'
run(r,w,[str(k),'test','--datatests','--datatests-dir','tests/stages','--name','ghangr-rust-empty-match'],'stage-focused.log')
run(r,w,[str(k),'test','--datatests','--datatests-dir','tests/stages','--save-baseline','docs/baseline-stages.json'],'record-stages.log')
stages=json.loads((r/'docs/baseline-stages.json').read_text())['data_footer'];assert stages[0]==stages[1]
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
record={'at':datetime.datetime.now(datetime.timezone.utc).isoformat(),'parent_commit':'db2ddd4d63413227bd4cab9756ed329ad561bf05','baseline_root':str(base),'parent_cli_sha256':sha(base/'decompiler/target/release/kuna'),'new_cli_sha256':sha(k),'stage_assertions':stages,'parent_artifact_provenance':'New independent build of clean main db2ddd4d; frozen154specs reused.'}
(w/'build.json').write_text(json.dumps(record,indent=2)+'\n');print(record,flush=True)
