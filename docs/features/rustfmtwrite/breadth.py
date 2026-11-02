from pathlib import Path
from concurrent.futures import ThreadPoolExecutor,as_completed
import ast,fcntl,hashlib,json,os,signal,subprocess,time
w=Path('/tmp/kuna-rust-fmt-write-optin');old=Path('/tmp/kuna-rust-fmt-write/breadth');r=Path('/home/mahaloz/kwt/angr-rust-fmt-write');base=Path('/home/mahaloz/kwt/angr-rust-fmt-str')
lock=(w/'breadth-exclusive.lock').open('w');fcntl.flock(lock,fcntl.LOCK_EX|fcntl.LOCK_NB)
out=w/'breadth';out.mkdir(exist_ok=True)
ns={'Path':Path,'root':r,'refs':Path('/tmp/kuna-angr-reference/binaries')}
for node in ast.parse(Path('/tmp/kuna-rust-fmt-write/breadth.py').read_text()).body:
 selected=isinstance(node,ast.Assign) and any(isinstance(t,ast.Name) and t.id=='inputs'for t in node.targets)
 selected=selected or isinstance(node,ast.AugAssign) and isinstance(node.target,ast.Name) and node.target.id=='inputs'
 if selected:exec(compile(ast.Module(body=[node],type_ignores=[]),'<fixed fixture inventory>','exec'),ns)
inputs=ns['inputs'];assert len(inputs)==27
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
previous=json.loads((old/'record.json').read_text());assert not previous['semantic_evidence_pending']
old_rows={row['fixture']:row for row in previous['fixtures']}
cli=r/'decompiler/target/release/kuna';binary_sha=sha(cli)
assert sha(base/'decompiler/target/release/kuna')==previous['baseline_kuna_sha256']
manifest=json.loads(Path('/tmp/kuna-rust-fmt-write/parent-source-before-commit.json').read_text());assert all(sha(base/name)==digest for name,digest in manifest['files'].items())
def capture(binary,language,name,mode):
 target=out/f'{name}-{language}-{mode}.json';temporary=target.with_name(target.name+'.partial')
 cmd=[str(cli),'decompile-all',str(binary),'--language',language,'--json','--jobs','1','--max-fn-seconds','600','--option','godwarfsig','on']
 if mode in ['on','off']:cmd+=['--option','rustfmtwrite',mode]
 with temporary.open('w')as stdout,target.with_suffix('.log').open('w')as stderr:
  process=subprocess.Popen(cmd,cwd=r,env=os.environ|{'KUNA_SPECS':str(r/'specs')},stdout=stdout,stderr=stderr,start_new_session=True)
  while process.poll()is None:
   memory={line.split(':')[0]:int(line.split()[1])for line in Path('/proc/meminfo').read_text().splitlines()}
   if 1-memory['MemAvailable']/memory['MemTotal']>=.9:
    os.killpg(process.pid,signal.SIGTERM);process.wait();(w/'memory-limit-hit.json').write_text(json.dumps(memory));raise RuntimeError('90 percent RAM: owned capture stopped; request user help.')
   time.sleep(1)
  assert process.returncode==0,(name,language,mode,process.returncode)
 temporary.replace(target);return target

def run(row):
 name,binary,languages=row;assert sha(binary)==old_rows[name]['sha256']
 record={'fixture':name,'fixture_sha256':sha(binary),'captures':[]}
 for language in languages:
  for mode in (['on','off','default']if language=='rust'else['default']):
   prior=old/f'{name}-{language}{"-off"if language=="rust"and mode!="on"else""}.json'
   fresh=capture(binary,language,name,mode)
   a=json.loads(prior.read_text());b=json.loads(fresh.read_text())
   assert a['functions']==b['functions'],('complete function record changed',name,language,mode)
   assert not b.get('error') and all(not f.get('error')for f in b['functions'])
   record['captures'].append({'language':language,'mode':mode,'functions':len(b['functions']),'fresh_file':str(fresh),'fresh_sha256':sha(fresh),'prior_file':str(prior),'prior_sha256':sha(prior),'all_complete_function_records_identical':True})
 (out/f'{name}-record.json').write_text(json.dumps(record,indent=2)+'\n')
 print('OPT-IN CAPTURE EQUIVALENT',name,len(record['captures']),flush=True);return record
with ThreadPoolExecutor(max_workers=3)as pool:rows=list(pool.map(run,inputs))
assert sha(cli)==binary_sha
record={'new_kuna_sha256':binary_sha,'previous_enabled_kuna_sha256':previous['patched_kuna_sha256'],'baseline_kuna_sha256':previous['baseline_kuna_sha256'],'baseline_commit':subprocess.check_output(['git','rev-parse','HEAD'],cwd=base,text=True).strip(),'baseline_has_pending_feature':bool(subprocess.check_output(['git','status','--porcelain'],cwd=base)),'images':27,'all_complete_function_records_identical':True,'scope':'Fresh enabled Rust equals the previously reviewed enabled output; fresh default and explicit-off Rust equal the previously reviewed off/parent output; fresh C and Go equal previously reviewed output. Every function field, instruction mapping and variable record is compared. Prior native execution applies only to these byte-identical enabled expressions.','fixtures':rows}
(out/'record.json').write_text(json.dumps(record,indent=2)+'\n');print('ALL OPT-IN COMPLETE OUTPUT EQUIVALENCE PASSED',flush=True)
