from pathlib import Path
from concurrent.futures import ThreadPoolExecutor,as_completed
import ast,hashlib,json,os,subprocess,time,signal
r=Path('/home/mahaloz/kwt/angr-rust-fmt-write-ddd');w=Path('/tmp/kuna-rust-fmt-write-ddd');old=Path('/tmp/kuna-rust-fmt-write-optin');out=w/'breadth';out.mkdir(exist_ok=True)
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
prior=json.loads((old/'breadth/record.json').read_text());index={x['fixture']:x for x in prior['fixtures']}
ns={'root':r,'refs':Path('/tmp/kuna-angr-reference/binaries'),'Path':Path}
for node in ast.parse(Path('/tmp/kuna-rust-fmt-write/breadth.py').read_text()).body:
 target=node.targets[0]if isinstance(node,ast.Assign)and len(node.targets)==1 else node.target if isinstance(node,ast.AugAssign)else None
 if isinstance(target,ast.Name)and target.id=='inputs':exec(compile(ast.Module(body=[node],type_ignores=[]),'<fixtures>','exec'),ns)
assert len(ns['inputs'])==27
build=json.loads((w/'build.json').read_text());cli=r/'decompiler/target/release/kuna';assert sha(cli)==build['new_cli_sha256']
def run(row):
 name,binary,languages=row;previous=index[name];assert sha(binary)==previous['fixture_sha256'];results=[]
 for capture in previous['captures']:
  language=capture['language'];mode=capture['mode'];path=out/f'{name}-{language}-{mode}.json';assert not path.exists()
  command=[str(cli),'decompile-all',str(binary),'--language',language,'--json','--jobs','1','--max-fn-seconds','600','--option','godwarfsig','on']
  if mode in ['on','off']:command+=['--option','rustfmtwrite',mode]
  with path.open('w')as f,path.with_suffix('.log').open('w')as log:
   p=subprocess.Popen(command,cwd=r,env=os.environ|{'KUNA_SPECS':str(r/'specs')},stdout=f,stderr=log,start_new_session=True)
   while p.poll()is None:
    m={s.split(':')[0]:int(s.split()[1])for s in Path('/proc/meminfo').read_text().splitlines()}
    if 1-m['MemAvailable']/m['MemTotal']>=.9:
     os.killpg(p.pid,signal.SIGTERM);p.wait();(w/'memory-limit-hit.json').write_text(json.dumps(m));raise RuntimeError('90% RAM; own work stopped; user help required.')
    time.sleep(1)
   assert p.returncode==0,(name,language,mode,p.returncode)
  before=Path(capture['fresh_file']);assert sha(before)==capture['fresh_sha256'];a=json.loads(before.read_text());b=json.loads(path.read_text());assert a['functions']==b['functions'],(name,language,mode,'complete records changed')
  assert all(not f.get('error')for f in b['functions'])
  results.append({'language':language,'mode':mode,'functions':len(b['functions']),'prior_file':str(before),'prior_sha256':sha(before),'fresh_file':str(path),'fresh_sha256':sha(path),'all_complete_function_records_identical':True})
 record={'fixture':name,'fixture_sha256':sha(binary),'captures':results};(out/f'{name}-record.json').write_text(json.dumps(record,indent=2)+'\n');print(name,'PASS',len(results),flush=True);return record
with ThreadPoolExecutor(max_workers=8)as pool:records=[f.result()for f in as_completed([pool.submit(run,row)for row in ns['inputs']])]
assert sha(cli)==build['new_cli_sha256']
(out/'record.json').write_text(json.dumps({'images':27,'new_kuna_sha256':build['new_cli_sha256'],'previous_kuna_sha256':prior['new_kuna_sha256'],'integrated_main_commit':build['parent_commit'],'all_complete_function_records_identical':True,'scope':'Every function record in all 27 images remains byte-identical to the previously reviewed outputs: Rust on, off and default; C and Go default. Native expression evidence is reused only through that exact equality. Previous speed measurements retain their old binary identities.','fixtures':records},indent=2)+'\n');print('ALL27INTEGRATEDCAPTURESPASSED',flush=True)
