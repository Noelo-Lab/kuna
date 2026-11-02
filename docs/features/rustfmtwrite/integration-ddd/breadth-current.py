from pathlib import Path
from concurrent.futures import ThreadPoolExecutor,as_completed
import ast,hashlib,json,os,subprocess,time,signal,shutil
r=Path('/home/mahaloz/kwt/angr-rust-fmt-write-ddd');base=Path('/home/mahaloz/kwt/angr-rust-base-ddd');w=Path('/tmp/kuna-rust-fmt-write-ddd');out=w/'breadth-current';out.mkdir(exist_ok=True);old=Path('/tmp/kuna-rust-fmt-write-optin');sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
prior=json.loads((old/'breadth/record.json').read_text());index={x['fixture']:x for x in prior['fixtures']};build=json.loads((w/'build.json').read_text());assert sha(r/'decompiler/target/release/kuna')==build['new_cli_sha256'];assert sha(base/'decompiler/target/release/kuna')==build['parent_cli_sha256']
ns={'root':r,'refs':Path('/tmp/kuna-angr-reference/binaries'),'Path':Path}
for node in ast.parse(Path('/tmp/kuna-rust-fmt-write/breadth.py').read_text()).body:
 target=node.targets[0]if isinstance(node,ast.Assign)and len(node.targets)==1 else node.target if isinstance(node,ast.AugAssign)else None
 if isinstance(target,ast.Name)and target.id=='inputs':exec(compile(ast.Module(body=[node],type_ignores=[]),'<fixtures>','exec'),ns)
def functions(path):return{f['address']:f for f in json.loads(path.read_text())['functions']}
def capture(binary,language,name,mode):
 path=out/f'{name}-{language}-{mode}.json';assert not path.exists()
 previous=w/'breadth'/path.name;reuse=False
 if mode!='parent'and previous.exists():
  try:parsed=functions(previous);reuse=bool(parsed)and all(not f.get('error')for f in parsed.values())
  except(json.JSONDecodeError,KeyError):pass
 if reuse:shutil.copy2(previous,path);return path,{'reused_from':str(previous),'sha256':sha(previous)}
 root=base if mode=='parent'else r;cmd=[str(root/'decompiler/target/release/kuna'),'decompile-all',str(binary),'--language',language,'--json','--jobs','1','--max-fn-seconds','600','--option','godwarfsig','on']
 if mode in ['on','off']:cmd+=['--option','rustfmtwrite',mode]
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
 for language in languages:
  parent_path,origin=capture(binary,language,name,'parent');parent=functions(parent_path)
  path,reuse=capture(binary,language,name,'default');current=functions(path);assert current==parent,(name,language,'current parent mismatch');assert all(not f.get('error')for f in current.values())
  old_default=next(x for x in previous['captures']if x['language']==language and x['mode']=='default');older=Path(old_default['fresh_file']);assert sha(older)==old_default['fresh_sha256'];historical=functions(older);assert historical.keys()==current.keys()
  inherited+=[{'language':language,'address':hex(a),'name':current[a]['name'],'changed_fields':[k for k in current[a]if current[a][k]!=historical[a].get(k)]}for a in current if current[a]!=historical[a]]
  results.append({'language':language,'mode':'default','functions':len(current),'fresh_file':str(path),'fresh_sha256':sha(path),'parent_file':str(parent_path),'parent_sha256':sha(parent_path),'current_parent_equal':True,'capture_origin':reuse})
  if language=='go':
   p=subprocess.run(['/tmp/kuna-go-output/parse',str(path)],capture_output=True);assert p.returncode==0,p.stderr;results[-1]['all_functions_parse']=True
  if language!='rust':continue
  off_path,reuse=capture(binary,language,name,'off');assert functions(off_path)==parent;results.append({'language':language,'mode':'off','functions':len(parent),'fresh_file':str(off_path),'fresh_sha256':sha(off_path),'current_parent_equal':True,'capture_origin':reuse})
  on_path,reuse=capture(binary,language,name,'on');on=functions(on_path);assert on.keys()==parent.keys();old_on=next(x for x in previous['captures']if x['language']=='rust'and x['mode']=='on');older_on=Path(old_on['fresh_file']);assert sha(older_on)==old_on['fresh_sha256'];historical_on=functions(older_on)
  changed={a for a in on if on[a]!=parent[a]};expected={a for a in historical_on if historical_on[a]!=historical[a]};assert changed==expected,(name,'unexpected feature changes',changed,expected)
  for a in changed:
   assert on[a]==historical_on[a]and parent[a]==historical[a],(name,hex(a),'changed feature record needs new semantic review');changes.append({'address':hex(a),'name':on[a]['name'],'entire_on_off_records_equal_previous_review':True})
  results.append({'language':language,'mode':'on','functions':len(on),'fresh_file':str(on_path),'fresh_sha256':sha(on_path),'feature_changes_only':True,'capture_origin':reuse})
 result={'fixture':name,'fixture_sha256':sha(binary),'captures':results,'feature_changes':changes,'inherited_main_changes':inherited};(out/f'{name}-record.json').write_text(json.dumps(result,indent=2)+'\n');print(name,'PASS changes',len(changes),'inherited records',len(inherited),flush=True);return result
with ThreadPoolExecutor(max_workers=8)as pool:records=[f.result()for f in as_completed([pool.submit(run,row)for row in ns['inputs']])]
assert len(records)==27 and sum(len(x['feature_changes'])for x in records)==8
assert sha(r/'decompiler/target/release/kuna')==build['new_cli_sha256']and sha(base/'decompiler/target/release/kuna')==build['parent_cli_sha256']
(out/'record.json').write_text(json.dumps({'images':27,'new_kuna_sha256':build['new_cli_sha256'],'baseline_kuna_sha256':build['parent_cli_sha256'],'baseline_commit':build['parent_commit'],'all_default_off_records_equal_current_parent':True,'all_feature_changes_identical_to_reviewed_records':True,'feature_changed_functions':8,'inherited_main_changed_records':sum(len(x['inherited_main_changes'])for x in records),'scope':'The fresh main-only baseline attributes switchrunonguard effects separately. Every default/off function matches current main exactly. All eight enabled-only changes retain exact prior before/after records, which permits reuse of the native/AST/mapping evidence. Old whole-corpus equality is not claimed.','fixtures':records},indent=2)+'\n');print('CURRENT MAIN WRITE CORPUS PASSED',flush=True)
