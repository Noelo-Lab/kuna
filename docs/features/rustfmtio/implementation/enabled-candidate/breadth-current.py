from pathlib import Path
from concurrent.futures import ThreadPoolExecutor,as_completed
import ast,hashlib,json,os,subprocess,time,signal,shutil
w=Path('/tmp/kuna-rust-io-ddd');old=Path('/tmp/kuna-rust-io-proposal');r=Path('/home/mahaloz/kwt/angr-rust-io-proposal');base=Path('/home/mahaloz/kwt/angr-rust-fmt-write-ddd');write=Path('/tmp/kuna-rust-fmt-write-ddd');out=w/'breadth-current';out.mkdir(exist_ok=True);sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
prior=json.loads((old/'breadth-final/record.json').read_text());index={x['fixture']:x for x in prior['records']};build=json.loads((w/'build.json').read_text());assert sha(r/'decompiler/target/release/kuna')==build['new_cli_sha256'];assert sha(base/'decompiler/target/release/kuna')==build['parent_cli_sha256']
ns={'root':r,'refs':Path('/tmp/kuna-angr-reference/binaries'),'Path':Path}
for node in ast.parse(Path('/tmp/kuna-rust-fmt-write/breadth.py').read_text()).body:
 target=node.targets[0]if isinstance(node,ast.Assign)and len(node.targets)==1 else node.target if isinstance(node,ast.AugAssign)else None
 if isinstance(target,ast.Name)and target.id=='inputs':exec(compile(ast.Module(body=[node],type_ignores=[]),'<fixtures>','exec'),ns)
inputs=ns['inputs']+[('io-native',r/'decompiler/crates/kuna-analysis/tests/fixtures/rust_fmt_io_x86_64',['c','rust','go']),('io-promoted',r/'decompiler/crates/kuna-analysis/tests/fixtures/rust_fmt_io_promoted_x86_64',['c','rust','go'])]
def functions(path):return{f['address']:f for f in json.loads(path.read_text())['functions']}
def capture(binary,language,name,mode):
 path=out/f'{name}-{language}-{mode}.json';assert not path.exists();previous=w/'breadth'/path.name;reuse=False
 if previous.exists():
  try:parsed=functions(previous);reuse=bool(parsed)and all(not f.get('error')for f in parsed.values())
  except(json.JSONDecodeError,KeyError):pass
 if reuse:shutil.copy2(previous,path);return path,{'reused_from':str(previous),'sha256':sha(previous)}
 root=base if mode=='parent'else r;cmd=[str(root/'decompiler/target/release/kuna'),'decompile-all',str(binary),'--language',language,'--json','--jobs','1','--max-fn-seconds','600','--option','godwarfsig','on']
 if mode=='off':cmd+=['--option','rustfmtio','off']
 if mode=='string-on-io-off':cmd+=['--option','rustfmtwrite','on','--option','rustfmtio','off']
 with path.open('w')as f,path.with_suffix('.log').open('w')as log:
  p=subprocess.Popen(cmd,cwd=r,env=os.environ|{'KUNA_SPECS':str(r/'specs')},stdout=f,stderr=log,start_new_session=True)
  while p.poll()is None:
   m={s.split(':')[0]:int(s.split()[1])for s in Path('/proc/meminfo').read_text().splitlines()}
   if 1-m['MemAvailable']/m['MemTotal']>=.9:os.killpg(p.pid,signal.SIGTERM);p.wait();(w/'memory-limit-hit.json').write_text(json.dumps(m));raise RuntimeError('90% RAM; owned work stopped; user help required.')
   time.sleep(1)
  assert p.returncode==0,(name,language,mode,p.returncode)
 return path,{'fresh_current_binary_capture':True}
def run(row):
 name,binary,languages=row;previous=index[name];assert sha(binary)==previous['fixture_sha256'];results=[];changes=[];inherited=[]
 if name not in ['io-native','io-promoted']:
  ready=write/'breadth-current'/f'{name}-record.json'
  while not ready.exists():
   assert 'Traceback'not in(write/'breadth-current.log').read_text();time.sleep(3)
  parent_receipt=json.loads(ready.read_text());assert parent_receipt['fixture_sha256']==sha(binary)
 for language in languages:
  if name in ['io-native','io-promoted']:parent_path,parent_origin=capture(binary,language,name,'parent')
  else:
   row=next(x for x in parent_receipt['captures']if x['language']==language and x['mode']=='default');parent_path=Path(row['fresh_file']);assert sha(parent_path)==row['fresh_sha256'];parent_origin={'fresh_parent_from_writer_corpus':str(parent_path)}
  parent=functions(parent_path);path,origin=capture(binary,language,name,'default');current=functions(path);assert current.keys()==parent.keys();assert all(not f.get('error')for group in [parent,current]for f in group.values())
  old_lang=next(x for x in previous['languages']if x['language']==language);old_after=old/'breadth-final'/f'{name}-{language}.json';assert sha(old_after)==old_lang['after_sha256'];historical_after=functions(old_after)
  if language=='rust':old_before=old/'breadth-final'/f'{name}-rust-off.json';assert sha(old_before)==old_lang['off_sha256'];historical_before=functions(old_before)
  else:historical_before=historical_after
  assert historical_before.keys()==parent.keys();inherited+=[{'language':language,'address':hex(a),'name':parent[a]['name']}for a in parent if parent[a]!=historical_before[a]]
  changed={a for a in current if current[a]!=parent[a]};expected={int(x['address'],16)for x in old_lang['changes']};assert changed==expected,(name,language,'unreviewed feature differences',changed,expected)
  for a in changed:
   assert current[a]==historical_after[a]and parent[a]==historical_before[a],(name,hex(a),'feature record needs renewed native/AST review');changes.append({'address':hex(a),'name':current[a]['name'],'entire_before_after_records_equal_previous_review':True})
  results.append({'language':language,'mode':'default','functions':len(current),'fresh_file':str(path),'fresh_sha256':sha(path),'parent_file':str(parent_path),'parent_sha256':sha(parent_path),'feature_changes':len(changed),'capture_origin':origin,'parent_origin':parent_origin})
  if language=='go':
   p=subprocess.run(['/tmp/kuna-go-output/parse',str(path)],capture_output=True);assert p.returncode==0,p.stderr;results[-1]['all_functions_parse']=True
  if language=='rust':
   off_path,origin=capture(binary,language,name,'off');assert functions(off_path)==parent;results.append({'language':language,'mode':'off','functions':len(parent),'fresh_file':str(off_path),'fresh_sha256':sha(off_path),'current_parent_equal':True,'capture_origin':origin})
   if name in ['rust','write-native']:
    string_path,origin=capture(binary,language,name,'string-on-io-off');row=next(x for x in parent_receipt['captures']if x['language']=='rust'and x['mode']=='on');parent_on=Path(row['fresh_file']);assert sha(parent_on)==row['fresh_sha256'];assert functions(string_path)==functions(parent_on);results.append({'language':language,'mode':'string-on-io-off','functions':len(parent),'fresh_file':str(string_path),'fresh_sha256':sha(string_path),'current_parent_equal':True,'capture_origin':origin})
 result={'fixture':name,'fixture_sha256':sha(binary),'captures':results,'feature_changes':changes,'inherited_main_changes':inherited};(out/f'{name}-record.json').write_text(json.dumps(result,indent=2)+'\n');print(name,'PASS changes',len(changes),'inherited records',len(inherited),flush=True);return result
with ThreadPoolExecutor(max_workers=8)as pool:records=[f.result()for f in as_completed([pool.submit(run,row)for row in inputs])]
assert len(records)==29 and sum(len(x['feature_changes'])for x in records)==11
assert sha(r/'decompiler/target/release/kuna')==build['new_cli_sha256']and sha(base/'decompiler/target/release/kuna')==build['parent_cli_sha256']
summary={'images':len(records),'distinct_functions':sum(next(c['functions']for c in x['captures']if c['mode']=='default')for x in records),'function_records':sum(c['functions']for x in records for c in x['captures']if c['mode']=='default'),'changed_functions':11,'hunks':11,'C_Go_identical':sum(c['functions']for x in records for c in x['captures']if c['mode']=='default'and c['language']!='rust'),'Go_parses':sum(c['functions']for x in records for c in x['captures']if c['mode']=='default'and c['language']=='go'),'off_matches_parent':sum(c['functions']for x in records for c in x['captures']if c['mode']=='off')}
assert summary==prior['summary']
(out/'record.json').write_text(json.dumps({'summary':summary,'new_kuna_sha256':build['new_cli_sha256'],'baseline_kuna_sha256':build['parent_cli_sha256'],'baseline_commit_at_capture':build['parent_commit'],'baseline_has_pending_main_merge':True,'prior_kuna_sha256':prior['new_kuna_sha256'],'all_disabled_records_equal_current_parent':True,'all_feature_changes_identical_to_reviewed_records':True,'semantic_evidence_pending':False,'inherited_main_changed_records':sum(len(x['inherited_main_changes'])for x in records),'scope':'Fresh current-main parent records attribute shared switchrunonguard changes separately. All disabled Rust and all C/Go records equal the current parent. All 11 enabled-only changes retain exact previously reviewed before/after whole records, preserving the basis of prior native/AST/image/mapping evidence. Old whole-corpus equality is not claimed.','records':records},indent=2)+'\n');print('CURRENT MAIN IO CORPUS PASSED',flush=True)
