from common import *
import re,datetime
base=Path('/home/mahaloz/kwt/angr-integer-field-extract-main');k=r/'decompiler/target/release/kuna'
p=subprocess.run([str(k),'catalog','--markdown'],capture_output=True,text=True,check=True);(r/'docs/options.md').write_text(p.stdout)
rows=json.loads(subprocess.check_output([str(k),'catalog','--json'],text=True));assert len(rows)==375
run(r,w,[str(k),'catalog','--check'],'catalog.log')
run(r,w,['python3','-m','scripts.repipe.counters','--check'],'counters.log')
run(r,w,[str(k),'test','--datatests','--datatests-dir','tests/stages','--name','ghangr-go-pair-results','--name','ghangr-integer-field-extract','--name','ghangr-joined-callback-return','--name','kuna-catalog'],'stage-focused.log')
run(r,w,[str(k),'test','--datatests','--datatests-dir','tests/stages','--save-baseline','docs/baseline-stages.json'],'record-stages.log')
old=json.loads(subprocess.check_output(['git','show','35005170a58026809d2ec82b975d2660cc7e2003:docs/baseline-stages.json'],cwd=r,text=True));new=json.loads((r/'docs/baseline-stages.json').read_text());before=set(old['passing']);after=set(new['passing']);old_label='data:KUNA-CATALOG #15: 374 registry rows plus six individual queries';new_label=old_label.replace('374','375');assert before-after=={old_label};added=after-before-{new_label};assert len(added)==8,(added,before-after);assert new['data_footer']==[3733,3733],new['data_footer']
record={'at':datetime.datetime.now(datetime.timezone.utc).isoformat(),'parent_commit':'35005170a58026809d2ec82b975d2660cc7e2003','parent_cli_sha256':hashlib.sha256((base/'decompiler/target/release/kuna').read_bytes()).hexdigest(),'new_cli_sha256':hashlib.sha256(k.read_bytes()).hexdigest(),'option':'gopairreturns','default':'off','requires':'godwarfsig on','catalog_count':375,'corpus_files':682,'stage_assertions':new['data_footer'],'stage_keys_added':sorted(added),'catalog_label_rename':[old_label,new_label],'publication_requires_parent_pr_merged':True};(w/'build.json').write_text(json.dumps(record,indent=2)+'\n');print('GO PREPARE PASS',json.dumps(record),flush=True)

plan=json.loads((w/'timing-plan.json').read_text());assert 'declared_binary_sha256'not in plan;plan['declared_binary_sha256']={'parent':record['parent_cli_sha256'],'candidate':record['new_cli_sha256']};(w/'timing-plan.json').write_text(json.dumps(plan,indent=2)+'\n')
