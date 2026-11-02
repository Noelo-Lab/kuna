from pathlib import Path
import subprocess,json,hashlib,sys
slug='str';config={'pieces':('pieces-f874','angr-rust-panic-main',['rustfmtpanic']),'str':('str','angr-rust-fmt-pieces',['rustfmtpanic','rustfmtpieces']),'write':('write','angr-rust-fmt-str',['rustfmtpanic','rustfmtpieces','rustfmtstr'])}[slug]
r=Path('/home/mahaloz/kwt/angr-rust-fmt-'+slug);base=Path('/home/mahaloz/kwt')/config[1];w=Path('/tmp/kuna-rust-fmt-'+config[0]);excluded=tuple('docs/features/'+name+'/'for name in config[2])
def git(tree,*args):return subprocess.check_output(['git',*args],cwd=tree)
def digest(path):return hashlib.sha256(path.read_bytes()).hexdigest()
def files(tree):return set(git(tree,'ls-files','--cached','--others','--exclude-standard').decode().splitlines())
sha=git(base,'rev-parse','HEAD').decode().strip();assert not git(base,'status','--porcelain')
parent=json.loads((w/'parent-source-before-commit.json').read_text());assert all(digest(base/p)==h for p,h in parent['files'].items());assert digest(base/'decompiler/target/release/kuna')==parent['binary_sha256']
paths={p for p in files(r)|files(base)if not p.startswith(excluded)}
before={p:digest(r/p)if(r/p).is_file()else None for p in paths};parent_files={p:digest(base/p)if(base/p).is_file()else None for p in paths};feature={p:h for p,h in before.items()if parent_files[p]!=h};assert feature
old=git(r,'rev-parse','HEAD').decode().strip();release=digest(r/'decompiler/target/release/kuna')
(w/'reanchor-before-feature.json').write_text(json.dumps({'previous_head':old,'new_parent':sha,'files':feature,'parent_files':{p:parent_files[p]for p in feature}},indent=2)+'\n')
assert subprocess.run(['git','rev-parse','--verify','MERGE_HEAD'],cwd=r,capture_output=True).returncode!=0,'Resolve the existing merge first.'
subprocess.run(['git','reset','--mixed',sha],cwd=r,check=True,stdout=subprocess.DEVNULL)
subprocess.run(['git','restore','--source=HEAD','--worktree','--',*[p.rstrip('/')for p in excluded]],cwd=r,check=True)
after={p:digest(r/p)if(r/p).is_file()else None for p in paths};assert before==after;assert digest(r/'decompiler/target/release/kuna')==release
(w/'own-feature-after-reanchor.diff').write_bytes(git(r,'diff','--binary'))
proof={'previous_head':old,'new_baseline_commit':sha,'parent_source_files_verified':len(parent['files']),'parent_cli_sha256':parent['binary_sha256'],'unchanged_working_files_verified':len(before),'feature_files_unchanged':feature,'release_binary_sha256':release,'release_binary_unchanged':True,'inherited_evidence_restored':list(excluded)}
def bind(path,archive):
 record=json.loads(path.read_text());pending=record.get('baseline_has_pending_feature',False)or record.get('baseline_pending_merge_commit')is not None
 if record['baseline_commit']!=sha or pending:
  archive.parent.mkdir(parents=True,exist_ok=True);archive.write_bytes(path.read_bytes());record['captured_baseline_commit']=record['baseline_commit']
  if 'baseline_pending_merge_commit'in record:record['captured_baseline_pending_merge_commit']=record['baseline_pending_merge_commit'];record['baseline_pending_merge_commit']=None
 record['baseline_commit']=sha;record['baseline_has_pending_feature']=False;record['parent_source_identity_verified']=True;record['parent_source_files_verified']=len(parent['files']);path.write_text(json.dumps(record,indent=2)+'\n')
if(w/'breadth/record.json').exists():bind(w/'breadth/record.json',w/'breadth-before-parent-commit.json')
preserved=w/'timing-before-parent-commit';preserved.mkdir(exist_ok=True)
if(w/'timing-monitor.json').exists():
 config=json.loads((w/'timing-cases.json').read_text())
 for name in [f'timing-{row[0]}.json'for row in config['cases']]+['timing-monitor.json']:bind(w/name,preserved/name)
(w/'reanchor-proof.json').write_text(json.dumps(proof,indent=2)+'\n');(w/'baseline-commit.txt').write_text(sha+'\n');print(sha)
