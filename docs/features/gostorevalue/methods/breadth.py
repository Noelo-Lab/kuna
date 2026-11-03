from common import *
from concurrent.futures import ThreadPoolExecutor,as_completed
import difflib,re
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
prior=json.loads(Path('/tmp/kuna-go-pair-results-implementation/integration-460/breadth/record.json').read_text())
build=json.loads((w/'build.json').read_text());assert build['parent_cli_sha256']==prior['candidate_cli_sha256']
out=w/'breadth';out.mkdir(exist_ok=False);digest=sha(r/'decompiler/target/release/kuna');assert digest==build['new_cli_sha256']
def functions(path):
 d=json.loads(Path(path).read_text());assert not d.get('error');fs=d['functions'];result={f['address']:f for f in fs};assert len(result)==len(fs)and result;return result

def parse(name,arm,fs):
 p=out/f'{name}-{arm}-parse.json';p.write_text(json.dumps({'functions':[{'name':hex(a),'code':f.get('code','')}for a,f in fs.items()]}));q=subprocess.run(['/tmp/kuna-go-output/parse',str(p)],capture_output=True,text=True);assert q.returncode in [0,1];output=q.stdout+q.stderr;(out/f'{name}-{arm}-parse.log').write_text(output);failed=sorted(set(re.findall(r'^0x[0-9a-f]+',output,re.MULTILINE)));assert(q.returncode==0)==(not failed);return failed

def row_job(row):
 name=row['fixture'];binary=Path(row['fixture_path']);assert sha(binary)==row['fixture_sha256'];records=[];changes=[]
 def language_job(item):
  lang=item['language'];arms=['off','on']if lang=='go'else['off'];local=[]
  for arm in arms:
   before=Path(item['candidate_files'][arm]);assert sha(before)==item['candidate_sha256'][arm];old=functions(before);path=out/f'{name}-{lang}-{arm}.json';cmd=[str(r/'decompiler/target/release/kuna'),'decompile-all',str(binary),'--language',lang,'--json','--jobs','1','--max-fn-seconds','600','--option','godwarfsig','on','--option','gopairreturns',arm]
   if name in ['joined_callback_return_x86_64','integer_field_extract_x86_64']:cmd+=['--assert','@'+str(r/('decompiler/crates/kuna-analysis/tests/fixtures/'+name.removesuffix('_x86_64')+'.kuna'))]
   targets={'mipsstubs_noreturn_n64el':'MIPS:LE:64:default','mipsgot_gcc_be64':'MIPS:BE:64:default'}
   if name in targets:cmd+=['--target',targets[name]]
   with path.open('w')as stdout,path.with_suffix('.log').open('w')as stderr:
    p=subprocess.Popen(cmd,cwd=r,env=env|{'KUNA_SPECS':str(r/'specs')},stdout=stdout,stderr=stderr,start_new_session=True)
    while p.poll()is None:
     m={line.split(':')[0]:int(line.split()[1])for line in Path('/proc/meminfo').read_text().splitlines()}
     if 1-m['MemAvailable']/m['MemTotal']>=.9:os.killpg(p.pid,signal.SIGTERM);p.wait();(w/'memory-limit-hit.json').write_text(json.dumps(m));raise RuntimeError('90% RAM; own capture stopped; user help required')
     time.sleep(1)
    assert p.returncode==0,(name,lang,arm,p.returncode)
   new=functions(path);assert old.keys()==new.keys();errors=[hex(a)for a in old if new[a].get('error')and not old[a].get('error')];assert not errors,(name,lang,arm,errors)
   for a in old:
    if old[a]!=new[a]:
     fields=sorted(k for k in set(old[a])|set(new[a])if old[a].get(k)!=new[a].get(k));assert lang=='go'and set(fields)<={'code','line_mappings'},(name,lang,arm,hex(a),fields)
     changes.append({'language':lang,'gopairreturns':arm,'address':hex(a),'name':new[a]['name'],'fields':fields,'parent':old[a],'candidate':new[a],'diff':list(difflib.unified_diff(old[a].get('code','').splitlines(),new[a].get('code','').splitlines(),fromfile='parent',tofile='candidate',lineterm='')),'review_pending':True})
   parses=None
   if lang=='go':
    parses={'parent':parse(name+'-'+arm,'parent',old),'candidate':parse(name+'-'+arm,'candidate',new)};assert not set(parses['candidate'])-set(parses['parent'])
   local.append({'language':lang,'gopairreturns':arm,'functions':len(old),'parent_file':str(before),'parent_sha256':sha(before),'candidate_file':str(path),'candidate_sha256':sha(path),'command':cmd,'new_errors':errors,'go_parse':parses})
  return local
 with ThreadPoolExecutor(max_workers=len(row['records']))as pool:
  for f in as_completed([pool.submit(language_job,item)for item in row['records']]):records.extend(f.result())
 assert sha(binary)==row['fixture_sha256'];result={'fixture':name,'fixture_path':str(binary),'fixture_sha256':row['fixture_sha256'],'records':records,'changes':changes};(out/(name+'-record.json')).write_text(json.dumps(result,indent=2)+'\n');print(name,'complete',len(changes),'changed Go records',flush=True);return result
with ThreadPoolExecutor(max_workers=4)as pool:records=[f.result()for f in as_completed([pool.submit(row_job,row)for row in prior['records']])]
assert sha(r/'decompiler/target/release/kuna')==digest
result={'status':'captured; semantic review pending','images':len(records),'distinct_functions':sum(row['records'][0]['functions']for row in records),'language_function_records':sum(item['functions']for row in records for item in row['records']),'changed_go_records':sum(len(row['changes'])for row in records),'parent_cli_sha256':prior['candidate_cli_sha256'],'candidate_cli_sha256':digest,'records':records};(out/'record.json').write_text(json.dumps(result,indent=2)+'\n');print('Complete STORE breadth with pair OFF/ON captured',flush=True)
