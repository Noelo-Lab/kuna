from common import *
parent=Path('/home/mahaloz/kwt/angr-field-extract-parent-c5')
run(r,w,['cargo','build','--release','-p','kuna-harness'],'harness-build.log',r/'decompiler')
run(r,w,['decompiler/target/release/kuna','test','--datatests','--datatests-dir','tests/stages','--save-baseline','docs/baseline-stages.json'],'record-stages.log')
old=json.loads(subprocess.check_output(['git','show','c5ea0fa96b83ac92b140e83017f57cff2d298e4c:docs/baseline-stages.json'],cwd=r,text=True));new=json.loads((r/'docs/baseline-stages.json').read_text())
assert set(old['passing'])<=set(new['passing']) and len(set(new['passing'])-set(old['passing']))==14
build={'parent_commit':'c5ea0fa96b83ac92b140e83017f57cff2d298e4c','parent_cli_sha256':hashlib.sha256((parent/'decompiler/target/release/kuna').read_bytes()).hexdigest(),'new_cli_sha256':hashlib.sha256((r/'decompiler/target/release/kuna').read_bytes()).hexdigest(),'stage_assertions':new['data_footer'],'catalog_count':374}
(w/'build.json').write_text(json.dumps(build,indent=2)+'\n')
print(json.dumps(build),flush=True)
