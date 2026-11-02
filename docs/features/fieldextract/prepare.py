from common import *
import datetime
base=Path('/home/mahaloz/kwt/angr-joined-callback-return')
run(r,w,['cargo','build','--release','-p','kuna-cli','-p','kuna-console','-p','kuna-harness'],'build-final.log',r/'decompiler')
b={'at':datetime.datetime.now(datetime.timezone.utc).isoformat(),'parent_commit':'8f403caeb8115aeb3f2eba8bef9840f1cdfbacef','parent_cli_sha256':hashlib.sha256((base/'decompiler/target/release/kuna').read_bytes()).hexdigest(),'new_cli_sha256':hashlib.sha256((r/'decompiler/target/release/kuna').read_bytes()).hexdigest(),'strict_correctness_fix':True,'option':None}
(w/'build.json').write_text(json.dumps(b,indent=2)+'\n')
run(r,w,['decompiler/target/release/kuna','test','--datatests','--datatests-dir','tests/stages','--name','ghangr-integer-field-extract','--name','ghangr-joined-callback-return'],'stage-focused.log')
run(r,w,['decompiler/target/release/kuna','test','--datatests','--datatests-dir','tests/stages','--save-baseline','docs/baseline-stages.json'],'record-stages.log')
old=json.loads(subprocess.check_output(['git','show','HEAD:docs/baseline-stages.json'],cwd=r,text=True));new=json.loads((r/'docs/baseline-stages.json').read_text());before=set(old['passing']);after=set(new['passing']);assert after>=before and len(after-before)==14,(len(before),len(after));assert new['data_footer']==[3721,3721],new['data_footer'];b['stage_assertions']=new['data_footer'];b['stage_keys_added']=sorted(after-before);(w/'build.json').write_text(json.dumps(b,indent=2)+'\n')
print('PREPARE PASS',flush=True)
