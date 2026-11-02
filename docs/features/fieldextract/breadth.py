from pathlib import Path
from concurrent.futures import ThreadPoolExecutor,as_completed
import difflib,hashlib,json,os,re,signal,subprocess,time
r=Path('/home/mahaloz/kwt/angr-integer-field-extract');base=Path('/home/mahaloz/kwt/angr-joined-callback-return');w=Path('/tmp/kuna-integer-field-extract');out=w/'breadth';out.mkdir(exist_ok=False)
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();build=json.loads((w/'build.json').read_text());prior=json.loads(Path('/tmp/kuna-joined-callback-return/round2/breadth/record.json').read_text())
assert sha(base/'decompiler/target/release/kuna')==build['parent_cli_sha256'];assert sha(r/'decompiler/target/release/kuna')==build['new_cli_sha256']
inputs=[(row['fixture'],Path(row['fixture_path'].replace(str(base),str(r))),[item['language']for item in row['records']])for row in prior['records']]
inputs.append(('integer_field_extract_x86_64',r/'decompiler/crates/kuna-analysis/tests/fixtures/integer_field_extract_x86_64',['c','go','rust']))
previous={old['fixture']:old for old in prior['records']}
assert prior['new_cli_sha256']==build['parent_cli_sha256']
(w/'breadth-inputs.json').write_text(json.dumps([(n,str(p),ls)for n,p,ls in inputs],indent=2)+'\n')
def functions(path):
 d=json.loads(path.read_text());assert not d.get('error');fs=d['functions'];result={f['address']:f for f in fs};assert len(result)==len(fs)and result;return result
def capture(binary,language,name,arm):
 root=base if arm=='parent'else r;path=out/f'{name}-{language}-{arm}.json';assert not path.exists();cmd=[str(root/'decompiler/target/release/kuna'),'decompile-all',str(binary),'--language',language,'--json','--jobs','1','--max-fn-seconds','600','--option','godwarfsig','on']
 if name in ['joined_callback_return_x86_64','integer_field_extract_x86_64']:cmd+=['--assert','@'+str(r/('decompiler/crates/kuna-analysis/tests/fixtures/'+name.removesuffix('_x86_64')+'.kuna'))]
 if name in {'mipsstubs_noreturn_n64el': 'MIPS:LE:64:default', 'mipsgot_gcc_be64': 'MIPS:BE:64:default'}:cmd+=['--target',{'mipsstubs_noreturn_n64el': 'MIPS:LE:64:default', 'mipsgot_gcc_be64': 'MIPS:BE:64:default'}[name]]
 with path.open('w')as stdout,path.with_suffix('.log').open('w')as stderr:
  p=subprocess.Popen(cmd,cwd=r,env=os.environ|{'KUNA_SPECS':str(r/'specs'),'SLEIGHHOME':str(r/'specs')},stdout=stdout,stderr=stderr,start_new_session=True)
  while p.poll()is None:
   m={line.split(':')[0]:int(line.split()[1])for line in Path('/proc/meminfo').read_text().splitlines()}
   if 1-m['MemAvailable']/m['MemTotal']>=.9:
    os.killpg(p.pid,signal.SIGTERM);p.wait();(w/'memory-limit-hit.json').write_text(json.dumps(m));raise RuntimeError('90% RAM; own capture stopped; user help required')
   time.sleep(1)
  assert p.returncode==0,(name,language,arm,p.returncode)
 return path
def parse_go(name,arm,fs):
 path=out/f'{name}-go-{arm}-parse-input.json';path.write_text(json.dumps({'functions':[{'name':hex(addr),'code':f.get('code','')}for addr,f in fs.items()]}));p=subprocess.run(['/tmp/kuna-go-output/parse',str(path)],capture_output=True,text=True);assert p.returncode in [0,1];output=p.stdout+p.stderr;(out/f'{name}-go-{arm}-parse.log').write_text(output);failed=sorted(set(re.findall(r'^0x[0-9a-f]+',output,re.MULTILINE)));assert(p.returncode==0)==(not failed);return {'all_parse':p.returncode==0,'failed_addresses':failed,'whole_emitted_bodies_unchanged':True}
def run(row):
 name,binary,languages=row;digest=sha(binary);changes=[]
 if name in previous:assert digest==previous[name]['fixture_sha256']
 def language_capture(language):
  cached=name in previous
  if cached:
   p=next(item for item in previous[name]['records']if item['language']==language);before=Path(p['candidate_file']);assert sha(before)==p['candidate_sha256']
  else:before=capture(binary,language,name,'parent')
  after=capture(binary,language,name,'candidate');old=functions(before);new=functions(after);assert old.keys()==new.keys(),(name,language,'function addresses changed');errors=[hex(a)for a in new if new[a].get('error')and not old[a].get('error')];assert not errors,(name,language,errors)
  for a in old:
   for key in ['name','aliases','address','address_hex','object_location','size']:assert old[a][key]==new[a][key],(name,language,hex(a),key)
   if old[a]!=new[a]:changes.append({'language':language,'address':hex(a),'name':new[a]['name'],'fields':[k for k in set(old[a])|set(new[a])if old[a].get(k)!=new[a].get(k)],'diff':list(difflib.unified_diff(old[a].get('code','').splitlines(),new[a].get('code','').splitlines(),fromfile='parent',tofile='candidate',lineterm='')),'classification':'pending'})
  parse=None
  if language=='go':
   parse={'parent':parse_go(name,'parent',old),'candidate':parse_go(name,'candidate',new)};assert set(parse['candidate']['failed_addresses'])<=set(parse['parent']['failed_addresses']),(name,'new Go syntax failures',parse)
  return {'language':language,'parent_file':str(before),'parent_sha256':sha(before),'candidate_file':str(after),'candidate_sha256':sha(after),'cached_parent_same_binary_and_fixture':cached,'fresh_candidate':True,'functions':len(new),'changed':sum(old[a]!=new[a]for a in old),'inherited_errors':sum(bool(f.get('error'))for f in old.values()),'new_errors':errors,'go_parse':parse}
 with ThreadPoolExecutor(max_workers=len(languages))as pool:records=[f.result()for f in as_completed([pool.submit(language_capture,language)for language in languages])]
 assert sha(binary)==digest
 result={'fixture':name,'fixture_path':str(binary),'fixture_sha256':digest,'capture_target':{'mipsstubs_noreturn_n64el': 'MIPS:LE:64:default', 'mipsgot_gcc_be64': 'MIPS:BE:64:default'}.get(name),'records':records,'changes':changes};(out/f'{name}-record.json').write_text(json.dumps(result,indent=2)+'\n');print(name,'COMPLETE',len(changes),'changed records',flush=True);return result
with ThreadPoolExecutor(max_workers=4)as pool:records=[f.result()for f in as_completed([pool.submit(run,row)for row in inputs])]
assert sha(r/'decompiler/target/release/kuna')==build['new_cli_sha256']and sha(base/'decompiler/target/release/kuna')==build['parent_cli_sha256']
record={'images':len(records),'distinct_functions':sum(row['records'][0]['functions']for row in records),'language_records':sum(item['functions']for row in records for item in row['records']),'changed_records':sum(len(row['changes'])for row in records),'semantic_review_pending':True,'new_cli_sha256':build['new_cli_sha256'],'parent_cli_sha256':build['parent_cli_sha256'],'parent_commit':build['parent_commit'],'records':records};(out/'record.json').write_text(json.dumps(record,indent=2)+'\n');print('BREADTH COMPLETE; REVIEW PENDING',flush=True)
