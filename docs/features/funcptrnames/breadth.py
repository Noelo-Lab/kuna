from pathlib import Path
from concurrent.futures import ThreadPoolExecutor,as_completed
import ast,hashlib,json,os,subprocess,time,signal,difflib
r=Path('/home/mahaloz/kwt/angr-fnptr-namespace');base=Path('/home/mahaloz/kwt/angr-base-4a9c');w=Path('/tmp/kuna-fnptr-namespace');out=w/'breadth';out.mkdir(exist_ok=False)
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest();build=json.loads((w/'build.json').read_text())
assert sha(r/'decompiler/target/release/kuna')==build['new_cli_sha256'];assert sha(base/'decompiler/target/release/kuna')==build['parent_cli_sha256']
ns={'root':r,'refs':Path('/tmp/kuna-angr-reference/binaries'),'Path':Path}
for node in ast.parse(Path('/tmp/kuna-rust-fmt-write/breadth.py').read_text()).body:
 target=node.targets[0]if isinstance(node,ast.Assign)and len(node.targets)==1 else node.target if isinstance(node,ast.AugAssign)else None
 if isinstance(target,ast.Name)and target.id=='inputs':exec(compile(ast.Module(body=[node],type_ignores=[]),'<fixtures>','exec'),ns)
f=r/'decompiler/crates/kuna-analysis/tests/fixtures'
inputs=ns['inputs']+[(name,path,languages)for name,path,languages in [
 ('io-native',f/'rust_fmt_io_x86_64',['c','rust','go']),('io-promoted',f/'rust_fmt_io_promoted_x86_64',['c','rust','go']),
 ('fnptr-native',f/'function_pointer_namespaces_x86_64',['c','rust','go']),
 ('hello',Path('/tmp/kuna-angr-reference/general/hello'),['c','rust','go']),
 ('fnptr-return',w/'address-return-no-debug',['c','rust','go']),
 ('fnptr-scopes',Path('/tmp/kuna-function-pointer-names/fixture-no-debug'),['c','rust','go']),
 ('namespace-rust',Path('/home/mahaloz/kwt/angr-dwarf-namespace/decompiler/crates/kuna-analysis/tests/fixtures/dwarf_namespaces_rust_x86_64'),['c','rust','go']),
 ('cppproto',f/'cppproto_x86_64',['c','rust','go'])]]
(w/'breadth-inputs.json').write_text(json.dumps([(n,str(p),ls)for n,p,ls in inputs],indent=2)+'\n')
def functions(path):
 doc=json.loads(path.read_text());assert doc.get('error') is None,(path,doc.get('error'));fs=doc['functions'];result={x['address']:x for x in fs};assert len(result)==len(fs)and result;return result

def capture(binary,language,name,arm,none=False):
 root=base if arm=='parent'else r;label=arm+('-none'if none else '')
 path=out/f'{name}-{language}-{label}.json';assert not path.exists()
 cmd=[str(root/'decompiler/target/release/kuna'),'decompile-all',str(binary),'--language',language,'--json','--jobs','1','--max-fn-seconds','600','--option','godwarfsig','on']
 if none:cmd+=['--option','namespacestrategy','none']
 with path.open('w')as stdout,path.with_suffix('.log').open('w')as stderr:
  p=subprocess.Popen(cmd,cwd=r,env=os.environ|{'KUNA_SPECS':str(r/'specs'),'SLEIGHHOME':str(r/'specs')},stdout=stdout,stderr=stderr,start_new_session=True)
  while p.poll()is None:
   m={s.split(':')[0]:int(s.split()[1])for s in Path('/proc/meminfo').read_text().splitlines()}
   if 1-m['MemAvailable']/m['MemTotal']>=.9:os.killpg(p.pid,signal.SIGTERM);p.wait();(w/'memory-limit-hit.json').write_text(json.dumps(m));raise RuntimeError('90% RAM; own process stopped; user help required')
   time.sleep(1)
  assert p.returncode==0,(name,language,label,p.returncode)
 return path

def run(row):
 name,binary,languages=row;digest=sha(binary);records=[];changes=[]
 for language in languages:
  before=capture(binary,language,name,'parent');after=capture(binary,language,name,'candidate');old=functions(before);new=functions(after)
  assert old.keys()==new.keys(),(name,language,'function addresses changed')
  new_errors=[hex(a)for a in new if new[a].get('error')and not old[a].get('error')];assert not new_errors,(name,language,new_errors)
  if language!='c':assert old==new,(name,language,'Non-C output changed')
  changed=[a for a in new if old[a]!=new[a]]
  for address in changed:
   a=old[address];b=new[address];fields=[k for k in set(a)|set(b)if a.get(k)!=b.get(k)];assert set(fields)<={'code','line_mappings','variables'},(name,language,hex(address),fields)
   changes.append({'language':language,'address':hex(address),'name':b['name'],'fields':fields,'diff':list(difflib.unified_diff(a.get('code','').splitlines(),b.get('code','').splitlines(),fromfile='parent',tofile='candidate',lineterm='')),'classification':'pending'})
  parse=None
  if language=='go':
   parsed=subprocess.run(['/tmp/kuna-go-output/parse',str(after)],capture_output=True,text=True);parse=parsed.returncode==0;(out/f'{name}-go-parse.log').write_text(parsed.stdout+parsed.stderr)
  records.append({'language':language,'parent_file':str(before),'parent_sha256':sha(before),'candidate_file':str(after),'candidate_sha256':sha(after),'fresh_parent_and_candidate':True,'functions':len(new),'changed':len(changed),'inherited_errors':sum(bool(x.get('error'))for x in old.values()),'new_errors':new_errors,'go_parse':parse,'non_C_exact_parent':language!='c'})
  if language=='c':
   off_before=capture(binary,language,name,'parent',True);off_after=capture(binary,language,name,'candidate',True)
   assert functions(off_before)==functions(off_after),(name,language,'namespaces none records changed')
   records[-1]['none_records_equal']=True;records[-1]['none_files']={str(p):sha(p)for p in [off_before,off_after]}
 assert sha(binary)==digest
 result={'fixture':name,'fixture_path':str(binary),'fixture_sha256':digest,'records':records,'changes':changes}
 (out/f'{name}-record.json').write_text(json.dumps(result,indent=2)+'\n');print(name,'COMPLETE',len(changes),'changed C records',flush=True);return result
with ThreadPoolExecutor(max_workers=8)as pool:records=[f.result()for f in as_completed([pool.submit(run,row)for row in inputs])]
assert sha(r/'decompiler/target/release/kuna')==build['new_cli_sha256'];assert sha(base/'decompiler/target/release/kuna')==build['parent_cli_sha256']
summary={'images':len(records),'distinct_functions':sum(row['records'][0]['functions']for row in records),'language_records':sum(x['functions']for row in records for x in row['records']),'changed_records':sum(len(row['changes'])for row in records),'semantic_review_pending':True,'new_cli_sha256':build['new_cli_sha256'],'parent_cli_sha256':build['parent_cli_sha256'],'parent_commit':build['parent_commit'],'records':records}
(out/'record.json').write_text(json.dumps(summary,indent=2)+'\n');print('FRESH BREADTH CAPTURES COMPLETE; SEMANTIC REVIEW PENDING',flush=True)
