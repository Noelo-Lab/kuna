from common import *
import datetime
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest();base=Path('/home/mahaloz/kwt/angr-base-0b7117');integration=json.loads((w/'integration.json').read_text());parent=integration['parent_main']
assert subprocess.check_output(['git','rev-parse','HEAD'],cwd=base,text=True).strip()==parent
assert subprocess.check_output(['git','rev-parse','HEAD'],cwd=r,text=True).strip()==integration['milestone_head']
assert subprocess.check_output(['git','rev-parse','MERGE_HEAD'],cwd=r,text=True).strip()==parent
assert not subprocess.check_output(['git','status','--porcelain'],cwd=base)
kuna=r/'decompiler/target/release/kuna';rows=json.loads(subprocess.check_output([str(kuna),'catalog','--json'],text=True));assert rows==json.loads(subprocess.check_output([str(base/'decompiler/target/release/kuna'),'catalog','--json'],text=True))
run(r,w,[str(kuna),'catalog','--check'],'catalog.log');run(r,w,['python3','-m','scripts.repipe.counters','--check'],'counters.log')
command=[str(kuna),'test','--datatests','--datatests-dir','tests/stages']
for name in ['ghangr-rust-pointer-index','ghangr-rust-store-value','ghangr-rust-integer-conversions','kuna-persistcalls']:command+=['--name',name]
run(r,w,command,'stage-focused.log')
run(r,w,[str(kuna),'test','--datatests','--datatests-dir','tests/stages','--save-baseline','docs/baseline-stages.json'],'record-stages.log')
old=json.loads(subprocess.check_output(['git','show',parent+':docs/baseline-stages.json'],cwd=r,text=True));new=json.loads((r/'docs/baseline-stages.json').read_text());before=set(old['passing']);after=set(new['passing']);assert not before-after;assert len(after-before)==8;assert new['data_footer']==[old['data_footer'][0]+8]*2;assert not subprocess.check_output(['git','diff',parent,'--','docs/baseline.json'],cwd=r)
record={'at':datetime.datetime.now(datetime.timezone.utc).isoformat(),'parent_commit':parent,'parent_worktree_head':parent,'parent_tree':integration['parent_tree'],'milestone_head':integration['milestone_head'],'parent_cli_sha256':sha(base/'decompiler/target/release/kuna'),'new_cli_sha256':sha(kuna),'catalog_count':len(rows),'corpus_files':sum(1 for folder in ['tests/datatests','tests/stages']for _ in(r/folder).glob('*.xml')),'stage_assertions':new['data_footer'],'stage_keys_added':sorted(after-before)};(w/'build.json').write_text(json.dumps(record,indent=2)+'\n');(w/'parent-main.json').write_text(json.dumps({'merged_commit':parent,'merged_tree':integration['parent_tree'],'scope':'Exact current-main parent, including persistcalls PR1459; detached parent worktree is clean.'},indent=2)+'\n');print('Prepared integrated pointer build',record,flush=True)
