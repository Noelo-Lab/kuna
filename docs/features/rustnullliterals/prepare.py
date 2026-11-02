from common import *
import datetime
base=Path('/home/mahaloz/kwt/angr-base-8db14');setup=json.loads((w/'setup.json').read_text());sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
run(r,w,['cargo','build','--release','-p','kuna-cli','-p','kuna-console','-p','kuna-harness'],'build-final.log',r/'decompiler')
k=r/'decompiler/target/release/kuna';run(r,w,[str(k),'test','--datatests','--datatests-dir','tests/stages','--name','ghangr-rust-null-pointer'],'stage-focused.log');run(r,w,[str(k),'test','--datatests','--datatests-dir','tests/stages','--save-baseline','docs/baseline-stages.json'],'record-stages.log')
stages=json.loads((r/'docs/baseline-stages.json').read_text())['data_footer'];assert stages==[3699,3699]
record={'at':datetime.datetime.now(datetime.timezone.utc).isoformat(),'parent_commit':setup['parent_commit'],'baseline_root':str(base),'parent_cli_sha256':sha(base/'decompiler/target/release/kuna'),'new_cli_sha256':sha(k),'stage_assertions':stages,'strict_correctness_fix':True,'option':None};assert record['parent_cli_sha256']==json.loads((w/'parent-source-before-timing.json').read_text())['release_sha256'];(w/'build.json').write_text(json.dumps(record,indent=2)+'\n');print(record,flush=True)
