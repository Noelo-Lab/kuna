from common import *
import shutil,datetime
base=Path('/home/mahaloz/kwt/angr-base-83a6f');initial=w.parent
assert (initial/'breadth/record.json').exists()
f=r/'decompiler/crates/kuna-analysis/tests/fixtures'
for src,dst in [('callback.c','joined_callback_return.c'),('callback','joined_callback_return_x86_64'),('callback.kuna','joined_callback_return.kuna'),('native-harness.c','joined_callback_return_harness.c')]:shutil.copy2(w/src,f/dst)
(f/'joined_callback_return.build.txt').write_text('Authored C callbacks returning two signed 64-bit fields, plus scalar and void controls.\nOriginal build:\ncc -O2 -g -fwrapv -fno-pie -no-pie -o /tmp/kuna-joined-callback-return/callback /tmp/kuna-joined-callback-return/callback.c\nThe original callback.c is byte-identical to joined_callback_return.c.\nEquivalent rebuild from this directory:\ncc -O2 -g -fwrapv -fno-pie -no-pie -o joined_callback_return_x86_64 joined_callback_return.c\n\n'+subprocess.check_output(['cc','--version'],text=True).splitlines()[0]+'\nSource SHA256 '+hashlib.sha256((f/'joined_callback_return.c').read_bytes()).hexdigest()+'\nBinary SHA256 '+hashlib.sha256((f/'joined_callback_return_x86_64').read_bytes()).hexdigest()+'\n')
run(r,w,['cargo','build','--release','-p','kuna-cli','-p','kuna-console','-p','kuna-harness'],'build-final.log',r/'decompiler')
b={'at':datetime.datetime.now(datetime.timezone.utc).isoformat(),'parent_commit':'83a6fd8483bf4d8a6dcc5d2bb1335f8724584636','parent_cli_sha256':hashlib.sha256((base/'decompiler/target/release/kuna').read_bytes()).hexdigest(),'new_cli_sha256':hashlib.sha256((r/'decompiler/target/release/kuna').read_bytes()).hexdigest(),'strict_correctness_fix':True,'option':None};(w/'build.json').write_text(json.dumps(b,indent=2)+'\n')
run(r,w,['decompiler/target/release/kuna','test','--datatests','--datatests-dir','tests/stages','--name','ghangr-joined-callback-return'],'stage-focused.log')
run(r,w,['decompiler/target/release/kuna','test','--datatests','--datatests-dir','tests/stages','--name','kuna-indirect-value-arguments','--name','kuna-stacklifetimes-cfg','--name','kuna-stackviews'],'void-regressions.log')
run(r,w,['decompiler/target/release/kuna','test','--datatests','--datatests-dir','tests/stages','--save-baseline','docs/baseline-stages.json'],'record-stages.log')
old=json.loads(subprocess.check_output(['git','show','HEAD:docs/baseline-stages.json'],cwd=r,text=True));new=json.loads((r/'docs/baseline-stages.json').read_text());before=set(old['passing']);after=set(new['passing']);assert after>=before and len(after-before)==8,(len(before),len(after));assert new['data_footer']==[3707,3707],new['data_footer'];b['stage_assertions']=new['data_footer'];b['stage_keys_added']=sorted(after-before);(w/'build.json').write_text(json.dumps(b,indent=2)+'\n')
run(r,w,['python3',str(w/'captures.py')],'captures.log')
run(r,w,['python3',str(w/'native-proof.py')],'native-coordinator.log')
run(r,w,['python3',str(w/'public-repro.py')],'public-repro-coordinator.log')
print('Round2 prepared and native proof passed',flush=True)
