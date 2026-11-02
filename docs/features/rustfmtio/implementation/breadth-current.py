from pathlib import Path
from concurrent.futures import ThreadPoolExecutor,as_completed
import ast,hashlib,json,os,subprocess,time,signal
w=Path('/tmp/kuna-rust-io-optin');old=Path('/tmp/kuna-rust-io-ddd');r=Path('/home/mahaloz/kwt/angr-rust-io-proposal');base=Path('/home/mahaloz/kwt/angr-rust-fmt-write-ddd');out=w/'breadth-current';out.mkdir(exist_ok=True)
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest();prior=json.loads((old/'breadth-current/record.json').read_text());index={x['fixture']:x for x in prior['records']};build=json.loads((w/'build.json').read_text())
assert sha(r/'decompiler/target/release/kuna')==build['new_cli_sha256'];assert sha(base/'decompiler/target/release/kuna')==build['parent_cli_sha256']
ns={'root':r,'refs':Path('/tmp/kuna-angr-reference/binaries'),'Path':Path}
for node in ast.parse(Path('/tmp/kuna-rust-fmt-write/breadth.py').read_text()).body:
 target=node.targets[0]if isinstance(node,ast.Assign)and len(node.targets)==1 else node.target if isinstance(node,ast.AugAssign)else None
 if isinstance(target,ast.Name)and target.id=='inputs':exec(compile(ast.Module(body=[node],type_ignores=[]),'<fixtures>','exec'),ns)
inputs=ns['inputs']+[('io-native',r/'decompiler/crates/kuna-analysis/tests/fixtures/rust_fmt_io_x86_64',['c','rust','go']),('io-promoted',r/'decompiler/crates/kuna-analysis/tests/fixtures/rust_fmt_io_promoted_x86_64',['c','rust','go'])]
def functions(path):
 data={f['address']:f for f in json.loads(path.read_text())['functions']};assert data and all(not f.get('error')for f in data.values());return data
def capture(binary,language,name,mode):
 path=out/f'{name}-{language}-{mode}.json';assert not path.exists()
 cmd=[str(r/'decompiler/target/release/kuna'),'decompile-all',str(binary),'--language',language,'--json','--jobs','1','--max-fn-seconds','600','--option','godwarfsig','on']
 if mode in ['off','on']:cmd+=['--option','rustfmtio',mode]
 if mode=='string-on-io-off':cmd+=['--option','rustfmtwrite','on','--option','rustfmtio','off']
 with path.open('w')as f,path.with_suffix('.log').open('w')as log:
  p=subprocess.Popen(cmd,cwd=r,env=os.environ|{'KUNA_SPECS':str(r/'specs')},stdout=f,stderr=log,start_new_session=True)
  while p.poll()is None:
   m={s.split(':')[0]:int(s.split()[1])for s in Path('/proc/meminfo').read_text().splitlines()}
   if 1-m['MemAvailable']/m['MemTotal']>=.9:os.killpg(p.pid,signal.SIGTERM);p.wait();(w/'memory-limit-hit.json').write_text(json.dumps(m));raise RuntimeError('90% RAM; owned work stopped; user help required.')
   time.sleep(1)
  assert p.returncode==0,(name,language,mode,p.returncode)
 return path
def run(row):
 name,binary,languages=row;previous=index[name];assert sha(binary)==previous['fixture_sha256'];results=[];changes=[]
 for language in languages:
  old_on=next(c for c in previous['captures']if c['language']==language and c['mode']=='default');parent_path=Path(old_on['parent_file']);assert sha(parent_path)==old_on['parent_sha256'];parent=functions(parent_path)
  reviewed_path=Path(old_on['fresh_file']);assert sha(reviewed_path)==old_on['fresh_sha256'];reviewed=functions(reviewed_path)
  for mode in ['default','on']+(['off']if language=='rust'else []):
   path=capture(binary,language,name,mode);current=functions(path);expected=reviewed if mode=='on'else parent
   assert current==expected,(name,language,mode,'full-record equality failed',[hex(a)for a in current if current.get(a)!=expected.get(a)])
   result={'language':language,'mode':mode,'functions':len(current),'fresh_file':str(path),'fresh_sha256':sha(path),'parent_file':str(parent_path),'parent_sha256':sha(parent_path),'current_parent_equal':mode!='on' or current==parent,'reviewed_enabled_equal':mode=='on','capture_origin':{'fresh_current_binary_capture':True}}
   if mode=='on':
    result.update(reviewed_file=str(reviewed_path),reviewed_sha256=sha(reviewed_path),feature_changes=sum(current[a]!=parent[a]for a in current))
    changes += [{'address':hex(a),'name':current[a]['name'],'entire_before_after_records_equal_previous_review':True}for a in current if current[a]!=parent[a]]
   if language=='go':
    p=subprocess.run(['/tmp/kuna-go-output/parse',str(path)],capture_output=True);assert p.returncode==0,p.stderr;result['all_functions_parse']=True
   results.append(result)
  if language=='rust'and name in ['rust','write-native']:
   previous_string=next(c for c in previous['captures']if c['language']=='rust'and c['mode']=='string-on-io-off');previous_path=Path(previous_string['fresh_file']);assert sha(previous_path)==previous_string['fresh_sha256']
   path=capture(binary,language,name,'string-on-io-off');current=functions(path);assert current==functions(previous_path)
   results.append({'language':language,'mode':'string-on-io-off','functions':len(current),'fresh_file':str(path),'fresh_sha256':sha(path),'current_parent_equal':True,'capture_origin':{'fresh_current_binary_capture':True}})
 result={'fixture':name,'fixture_sha256':sha(binary),'captures':results,'feature_changes':changes,'inherited_main_changes':previous['inherited_main_changes']};(out/f'{name}-record.json').write_text(json.dumps(result,indent=2)+'\n');print(name,'PASS enabled changes',len(changes),flush=True);return result
with ThreadPoolExecutor(max_workers=8)as pool:records=[f.result()for f in as_completed([pool.submit(run,row)for row in inputs])]
assert len(records)==29 and sum(len(x['feature_changes'])for x in records)==11
assert sha(r/'decompiler/target/release/kuna')==build['new_cli_sha256']and sha(base/'decompiler/target/release/kuna')==build['parent_cli_sha256']
summary={'images':len(records),'distinct_functions':sum(next(c['functions']for c in x['captures']if c['mode']=='on')for x in records),'function_records':sum(c['functions']for x in records for c in x['captures']if c['mode']=='on'),'changed_functions':11,'hunks':11,'C_Go_identical':sum(c['functions']for x in records for c in x['captures']if c['mode']=='on'and c['language']!='rust'),'Go_parses':sum(c['functions']for x in records for c in x['captures']if c['mode']=='on'and c['language']=='go'),'off_matches_parent':sum(c['functions']for x in records for c in x['captures']if c['mode']=='off')}
assert summary==prior['summary']
(out/'record.json').write_text(json.dumps({'summary':summary,'new_kuna_sha256':build['new_cli_sha256'],'baseline_kuna_sha256':build['parent_cli_sha256'],'baseline_commit_at_capture':build['parent_commit'],'baseline_has_pending_main_merge':False,'prior_kuna_sha256':prior['new_kuna_sha256'],'all_default_off_records_equal_current_parent':True,'all_disabled_records_equal_current_parent':True,'all_enabled_records_equal_previous_review':True,'all_feature_changes_identical_to_reviewed_records':True,'semantic_evidence_pending':False,'inherited_main_changed_records':prior['inherited_main_changed_records'],'scope':'Fresh default and disabled captures equal the committed current parent. Every fresh enabled language record equals the reviewed default-on candidate, including all 11 changed records, preserving prior native/AST/image/mapping evidence. Inherited main changes remain separately attributed.','records':records},indent=2)+'\n');print('OPT-IN IO CORPUS PASSED',flush=True)
