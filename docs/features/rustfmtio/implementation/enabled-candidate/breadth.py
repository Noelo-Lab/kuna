from pathlib import Path
from concurrent.futures import ThreadPoolExecutor,as_completed
import hashlib,json,os,subprocess,time,signal,ast
w=Path('/tmp/kuna-rust-io-ddd');old=Path('/tmp/kuna-rust-io-proposal');r=Path('/home/mahaloz/kwt/angr-rust-io-proposal');base=Path('/home/mahaloz/kwt/angr-rust-fmt-write-ddd');out=w/'breadth';out.mkdir(exist_ok=True)
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
prior=json.loads((old/'breadth-final/record.json').read_text());assert not prior['semantic_evidence_pending'];build=json.loads((w/'build.json').read_text());assert sha(r/'decompiler/target/release/kuna')==build['new_cli_sha256'];assert sha(base/'decompiler/target/release/kuna')==build['parent_cli_sha256']
ns={'root':r,'refs':Path('/tmp/kuna-angr-reference/binaries'),'Path':Path}
for node in ast.parse(Path('/tmp/kuna-rust-fmt-write/breadth.py').read_text()).body:
 target=node.targets[0]if isinstance(node,ast.Assign)and len(node.targets)==1 else node.target if isinstance(node,ast.AugAssign)else None
 if isinstance(target,ast.Name)and target.id=='inputs':exec(compile(ast.Module(body=[node],type_ignores=[]),'<fixtures>','exec'),ns)
inputs=ns['inputs']+[('io-native',r/'decompiler/crates/kuna-analysis/tests/fixtures/rust_fmt_io_x86_64',['c','rust','go']),('io-promoted',r/'decompiler/crates/kuna-analysis/tests/fixtures/rust_fmt_io_promoted_x86_64',['c','rust','go'])];assert len(inputs)==29
prior_rows={row['fixture']:row for row in prior['records']}
def capture(binary,language,name,mode,expected,parent=False):
 root=base if parent else r;path=out/f'{name}-{language}-{mode}.json';assert not path.exists()
 cmd=[str(root/'decompiler/target/release/kuna'),'decompile-all',str(binary),'--language',language,'--json','--jobs','1','--max-fn-seconds','600','--option','godwarfsig','on']
 if mode=='off':cmd+=['--option','rustfmtio','off']
 if mode=='string-on-io-off':cmd+=['--option','rustfmtwrite','on','--option','rustfmtio','off']
 with path.open('w')as f,path.with_suffix('.log').open('w')as log:
  p=subprocess.Popen(cmd,cwd=r,env=os.environ|{'KUNA_SPECS':str(r/'specs')},stdout=f,stderr=log,start_new_session=True)
  while p.poll()is None:
   m={s.split(':')[0]:int(s.split()[1])for s in Path('/proc/meminfo').read_text().splitlines()}
   if 1-m['MemAvailable']/m['MemTotal']>=.9:
    os.killpg(p.pid,signal.SIGTERM);p.wait();(w/'memory-limit-hit.json').write_text(json.dumps(m));raise RuntimeError('90% RAM; own work stopped; user help required.')
   time.sleep(1)
  assert p.returncode==0,(name,language,mode,p.returncode)
 a=json.loads(expected.read_text())['functions'];b=json.loads(path.read_text())['functions'];assert a==b,(name,language,mode,'complete record differs')
 assert all(not f.get('error')for f in b)
 return {'language':language,'mode':mode,'functions':len(b),'prior_file':str(expected),'prior_sha256':sha(expected),'fresh_file':str(path),'fresh_sha256':sha(path),'all_complete_function_records_identical':True}
def run(row):
 name,binary,languages=row;record=prior_rows[name];assert sha(binary)==record['fixture_sha256'];results=[]
 for language in languages:
  expected=old/'breadth-final'/f'{name}-{language}.json';old_lang=next(x for x in record['languages']if x['language']==language);assert sha(expected)==old_lang['after_sha256']
  results.append(capture(binary,language,name,'default',expected))
  if language=='rust':
   expected=old/'breadth-final'/f'{name}-rust-off.json';assert sha(expected)==old_lang['off_sha256'];results.append(capture(binary,language,name,'off',expected))
   if name in ['rust','write-native']:results.append(capture(binary,language,name,'string-on-io-off',old/'breadth-final'/f'{name}-rust-string-on-io-off.json'))
  if name in ['io-native','io-promoted']:
   expected=old/f'{name[3:]}-before.json'if language=='rust'else old/'breadth-final'/f'{name}-{language}-before.json'
   assert sha(expected)==old_lang['before_sha256'];results.append(capture(binary,language,name,'parent',expected,parent=True))
 result={'fixture':name,'fixture_sha256':sha(binary),'captures':results};(out/f'{name}-record.json').write_text(json.dumps(result,indent=2)+'\n');print(name,'PASS',len(results),flush=True);return result
with ThreadPoolExecutor(max_workers=8)as pool:results=[f.result()for f in as_completed([pool.submit(run,row)for row in inputs])]
assert sha(r/'decompiler/target/release/kuna')==build['new_cli_sha256']and sha(base/'decompiler/target/release/kuna')==build['parent_cli_sha256']
(out/'record.json').write_text(json.dumps({'new_kuna_sha256':build['new_cli_sha256'],'baseline_kuna_sha256':build['parent_cli_sha256'],'baseline_has_pending_main_merge':True,'baseline_commit':build['parent_commit'],'prior_kuna_sha256':prior['new_kuna_sha256'],'all_complete_function_records_identical':True,'semantic_evidence_pending':False,'summary':prior['summary'],'native_reuse_basis':'Every whole function record is identical to the previous 29-image review, including all 11 independently verified changes. Native expression, AST, image-byte, mapping, type and Go parser evidence applies only through that exact equality. Fresh current parent records for the two new I/O fixtures also match the old parent. The other 27 current parent images are independently compared in the String feature corpus run, required before finalization.','records':results},indent=2)+'\n');print('ALL29INTEGRATEDIOCAPTURESPASSED',flush=True)
