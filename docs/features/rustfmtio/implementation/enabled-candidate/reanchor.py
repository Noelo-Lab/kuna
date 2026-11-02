from pathlib import Path
import json,hashlib,subprocess,shutil
r=Path('/home/mahaloz/kwt/angr-rust-io-proposal');base=Path('/home/mahaloz/kwt/angr-rust-fmt-write-ddd');w=Path('/tmp/kuna-rust-io-ddd');sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
head=subprocess.check_output(['git','rev-parse','HEAD'],cwd=base,text=True).strip();assert head!='b7745023f25688558babc621414b0d8f30332616';assert not subprocess.check_output(['git','status','--porcelain'],cwd=base)
assert json.loads(Path('/tmp/kuna-rust-fmt-write-ddd/finalized.json').read_text())['fresh_default_off_speed_passed']
manifest=json.loads((w/'production-source-before-validation.json').read_text());assert all(sha(r/p)==h for p,h in manifest.items());gates=json.loads((w/'gates-passed.json').read_text());assert gates['all_four_gates_passed'];assert sha(r/'decompiler/target/release/kuna')==gates['release_sha256']
assert json.loads((w/'breadth-current/record.json').read_text())['all_feature_changes_identical_to_reviewed_records']
paths=set(manifest)|{'docs/baseline-stages.json'}
own={p:sha(r/p)for p in paths if(r/p).is_file()and(not(base/p).is_file()or sha(base/p)!=sha(r/p))}
assert len(own)==27,(len(own),own.keys())
backup=w/'before-reanchor';backup.mkdir()
for rel in own:
 dest=backup/rel;dest.parent.mkdir(parents=True,exist_ok=True);shutil.copy2(r/rel,dest)
(backup/'own-files.json').write_text(json.dumps(own,indent=2)+'\n');(backup/'working.patch').write_bytes(subprocess.check_output(['git','diff','--binary'],cwd=r));(backup/'index.patch').write_bytes(subprocess.check_output(['git','diff','--cached','--binary'],cwd=r))
tracked=subprocess.check_output(['git','diff','--name-only'],cwd=r,text=True).splitlines()
if tracked:subprocess.run(['git','restore','--worktree','--',*tracked],cwd=r,check=True)
parent_tracked=set(subprocess.check_output(['git','ls-files'],cwd=base,text=True).splitlines())
common_untracked=[]
for rel in subprocess.check_output(['git','ls-files','--others','--exclude-standard'],cwd=r,text=True).splitlines():
 if rel in parent_tracked:
  assert sha(r/rel)==sha(base/rel),rel
  dest=backup/'common-untracked'/rel;dest.parent.mkdir(parents=True,exist_ok=True);shutil.copy2(r/rel,dest);(r/rel).unlink();common_untracked.append(rel)
subprocess.run(['git','merge','--abort'],cwd=r,check=True)
subprocess.run(['git','merge','--no-commit','--no-ff',head],cwd=r,check=True)
for rel,h in own.items():
 dest=r/rel;dest.parent.mkdir(parents=True,exist_ok=True);shutil.copy2(backup/rel,dest);assert sha(dest)==h
assert all(sha(r/p)==h for p,h in manifest.items());assert sha(r/'decompiler/target/release/kuna')==gates['release_sha256']
parent_paths=subprocess.check_output(['git','ls-files','decompiler','tests','specs','VERSION','Makefile','docs/spec','docs/options.md'],cwd=base,text=True).splitlines()
parent={'commit':head,'binary_sha256':sha(base/'decompiler/target/release/kuna'),'files':{p:sha(base/p)for p in parent_paths if(base/p).is_file()}}
(w/'parent-source-before-timing.json').write_text(json.dumps(parent,indent=2)+'\n')
receipt={'old_pending_parent':'b7745023f25688558babc621414b0d8f30332616','new_parent':head,'own_files_preserved':len(own),'common_untracked_replaced_by_parent':common_untracked,'all_validated_source_files_identical':len(manifest),'new_kuna_sha256':gates['release_sha256'],'parent_kuna_sha256':parent['binary_sha256']}
(w/'reanchor.json').write_text(json.dumps(receipt,indent=2)+'\n')
print('IO REANCHORED WITHOUT SOURCE/BINARY CHANGE',head,flush=True)
