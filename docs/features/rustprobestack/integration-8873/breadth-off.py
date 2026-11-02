from pathlib import Path
from concurrent.futures import ThreadPoolExecutor,as_completed
import difflib,hashlib,json,os,re,signal,subprocess,time

r=Path('/home/mahaloz/kwt/angr-rust-probestack');base=Path('/home/mahaloz/kwt/angr-base-8873');w=Path('/tmp/kuna-rust-probestack-8873');out=w/'breadth-off';out.mkdir(exist_ok=False)
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest();build=json.loads((w/'build.json').read_text())
assert sha(r/'decompiler/target/release/kuna')==build['new_cli_sha256']and sha(base/'decompiler/target/release/kuna')==build['parent_cli_sha256']
inputs=[(name,Path(path.replace('/home/mahaloz/kwt/angr-dwarf-namespace',str(r))),languages)for name,path,languages in json.loads(Path('/tmp/kuna-dwarf-namespace-p9/breadth-inputs.json').read_text())]
fixtures=r/'decompiler/crates/kuna-analysis/tests/fixtures'
inputs += [('probe-native',fixtures/'rust_probestack_x86_64',['c','rust','go']),('probe-buffer',fixtures/'rust_probestack_buffer_x86_64',['c','rust','go']),('probe-upstream',Path('/tmp/kuna-angr-reference/general/probestack'),['c','rust','go'])]
off_controls={'probe-native','probe-buffer','probe-upstream'}
previous={row['fixture']:row for row in json.loads((w/'auto-study/breadth-final.json').read_text())['records']}
(w/'breadth-inputs.json').write_text(json.dumps([(n,str(p),ls)for n,p,ls in inputs],indent=2)+'\n')
def functions(path):
 d=json.loads(path.read_text());assert not d.get('error');fs=d['functions'];result={f['address']:f for f in fs};assert len(result)==len(fs)and result;return result
def capture(binary,language,name,arm,off=False):
 root=base if arm=='parent'else r;label=arm+('-auto'if off else '');path=out/f'{name}-{language}-{label}.json';assert not path.exists()
 cmd=[str(root/'decompiler/target/release/kuna'),'decompile-all',str(binary),'--language',language,'--json','--jobs','1','--max-fn-seconds','600','--option','godwarfsig','on']
 if off and arm=='candidate':cmd+=['--option','rustprobestack','auto']
 with path.open('w')as stdout,path.with_suffix('.log').open('w')as stderr:
  p=subprocess.Popen(cmd,cwd=r,env=os.environ|{'KUNA_SPECS':str(r/'specs'),'SLEIGHHOME':str(r/'specs')},stdout=stdout,stderr=stderr,start_new_session=True)
  while p.poll()is None:
   m={line.split(':')[0]:int(line.split()[1])for line in Path('/proc/meminfo').read_text().splitlines()}
   if 1-m['MemAvailable']/m['MemTotal']>=.9:
    os.killpg(p.pid,signal.SIGTERM);p.wait();(w/'memory-limit-hit.json').write_text(json.dumps(m));raise RuntimeError('90% RAM; own captures stopped; user help required')
   time.sleep(1)
  assert p.returncode==0,(name,language,label,p.returncode)
 return path
def parse_go(name,arm,fs):
 path=out/f'{name}-go-{arm}-parse-input.json';path.write_text(json.dumps({'functions':[{'name':hex(addr),'code':f.get('code','')}for addr,f in fs.items()]}))
 p=subprocess.run(['/tmp/kuna-go-output/parse',str(path)],capture_output=True,text=True);assert p.returncode in [0,1]
 output=p.stdout+p.stderr;(out/f'{name}-go-{arm}-parse.log').write_text(output)
 failed=sorted(set(re.findall(r'^0x[0-9a-f]+',output,re.MULTILINE)));assert (p.returncode==0)==(not failed),(name,arm,output)
 return {'all_parse':p.returncode==0,'failed_addresses':failed,'code_unchanged_in_address_labeled_parse_input':True}
def run(row):
 name,binary,languages=row;digest=sha(binary);records=[];changes=[]
 def language_capture(language):
  before=capture(binary,language,name,'parent');after=capture(binary,language,name,'candidate');old=functions(before);new=functions(after);assert old.keys()==new.keys(),(name,language,'function addresses changed')
  new_errors=[hex(a)for a in new if new[a].get('error')and not old[a].get('error')];assert not new_errors,(name,language,new_errors)
  changed=[a for a in new if old[a]!=new[a]];assert not changed,(name,language,'OFF-default records differ from parent')
  for address in changed:
   a=old[address];b=new[address];fields=[k for k in set(a)|set(b)if a.get(k)!=b.get(k)];assert set(fields)<={'code','line_mappings','variables'},(name,language,hex(address),fields)
   old_names={a['name'],*a.get('aliases',[])};new_names={b['name'],*b.get('aliases',[])}
   changes.append({'language':language,'address':hex(address),'before_name':a['name'],'name':b['name'],'fields':fields,'added_names':sorted(new_names-old_names),'removed_names':sorted(old_names-new_names),'diff':list(difflib.unified_diff(a.get('code','').splitlines(),b.get('code','').splitlines(),fromfile='parent',tofile='candidate',lineterm='')),'classification':'pending'})
  parse=None
  if language=='go':
   parse={'parent':parse_go(name,'parent',old),'candidate':parse_go(name,'candidate',new)};assert set(parse['candidate']['failed_addresses'])<=set(parse['parent']['failed_addresses']),(name,'new Go syntax failures',parse)
  result={'language':language,'parent_file':str(before),'parent_sha256':sha(before),'candidate_file':str(after),'candidate_sha256':sha(after),'fresh_parent_and_candidate':True,'functions':len(new),'changed':len(changed),'inherited_errors':sum(bool(f.get('error'))for f in old.values()),'new_errors':new_errors,'go_parse':parse}
  if name in off_controls:
   enabled=capture(binary,language,name,'candidate',True)
   old_enabled=next(x for x in previous[name]['records']if x['language']==language)
   assert functions(enabled)==functions(Path(old_enabled['candidate_file'])),(name,language,'enabled records changed')
   result['enabled_records_equal_previous_verified_auto']=True;result['enabled_file']=str(enabled);result['enabled_sha256']=sha(enabled)
  return result
 with ThreadPoolExecutor(max_workers=len(languages))as pool:
  records=[f.result()for f in as_completed([pool.submit(language_capture,language)for language in languages])]
 assert sha(binary)==digest
 result={'fixture':name,'fixture_path':str(binary),'fixture_sha256':digest,'records':records,'changes':changes};(out/f'{name}-record.json').write_text(json.dumps(result,indent=2)+'\n');print(name,'COMPLETE',len(changes),'changed records',flush=True);return result
with ThreadPoolExecutor(max_workers=8)as pool:records=[f.result()for f in as_completed([pool.submit(run,row)for row in inputs])]
assert sha(r/'decompiler/target/release/kuna')==build['new_cli_sha256']and sha(base/'decompiler/target/release/kuna')==build['parent_cli_sha256']
summary={'images':len(records),'distinct_functions':sum(row['records'][0]['functions']for row in records),'language_records':sum(item['functions']for row in records for item in row['records']),'changed_records':sum(len(row['changes'])for row in records),'semantic_review_pending':True,'new_cli_sha256':build['new_cli_sha256'],'parent_cli_sha256':build['parent_cli_sha256'],'parent_commit':build['parent_commit'],'records':records};(out/'record.json').write_text(json.dumps(summary,indent=2)+'\n');print('FRESH RUST PROBESTACK BREADTH CAPTURES COMPLETE; REVIEW PENDING',flush=True)
