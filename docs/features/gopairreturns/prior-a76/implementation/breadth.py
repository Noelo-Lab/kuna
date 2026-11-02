from common import *
from concurrent.futures import ThreadPoolExecutor,as_completed
import difflib,re
while not(w/'build.json').exists():time.sleep(2)
out=w/'breadth';out.mkdir(exist_ok=False);base=Path('/home/mahaloz/kwt/angr-integer-field-extract-main');prior=json.loads(Path('/tmp/kuna-integer-field-extract/integration-c5ea/breadth-final/record.json').read_text());sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();candidate_sha=sha(r/'decompiler/target/release/kuna');parent_sha=sha(base/'decompiler/target/release/kuna');assert parent_sha==prior['new_cli_sha256'];previous={row['fixture']:row for row in prior['records']};inputs=[(row['fixture'],Path(row['fixture_path']),[item['language']for item in row['records']])for row in previous.values()]
inputs.extend((name,r/'tests/fixtures/go-pair-results'/filename,['c','go','rust'])for name,filename in [('gopair122','go1.22.5-pair'),('gopair127','go1.27.3-pair'),('gopair-contracts','contracts'),('gopair-contracts-plain','contracts-plain')]);assert len(inputs)==67

def functions(path):
 d=json.loads(path.read_text());assert not d.get('error');fs=d['functions'];result={f['address']:f for f in fs};assert len(result)==len(fs)and result;return result

def capture(binary,lang,name,arm):
 root=base if arm=='parent'else r;path=out/f'{name}-{lang}-{arm}.json';cmd=[str(root/'decompiler/target/release/kuna'),'decompile-all',str(binary),'--language',lang,'--json','--jobs','1','--max-fn-seconds','600','--option','godwarfsig','on']
 if arm!='parent':cmd+=['--option','gopairreturns',arm]
 if name in ['joined_callback_return_x86_64','integer_field_extract_x86_64']:cmd+=['--assert','@'+str(r/('decompiler/crates/kuna-analysis/tests/fixtures/'+name.removesuffix('_x86_64')+'.kuna'))]
 targets={'mipsstubs_noreturn_n64el':'MIPS:LE:64:default','mipsgot_gcc_be64':'MIPS:BE:64:default'}
 if name in targets:cmd+=['--target',targets[name]]
 with path.open('w')as stdout,path.with_suffix('.log').open('w')as stderr:
  p=subprocess.Popen(cmd,cwd=r,env=env|{'KUNA_SPECS':str(r/'specs')},stdout=stdout,stderr=stderr,start_new_session=True)
  while p.poll()is None:
   m={line.split(':')[0]:int(line.split()[1])for line in Path('/proc/meminfo').read_text().splitlines()}
   if 1-m['MemAvailable']/m['MemTotal']>=.9:
    os.killpg(p.pid,signal.SIGTERM);p.wait();(w/'memory-limit-hit.json').write_text(json.dumps(m));raise RuntimeError('90% RAM; own capture stopped; user help required')
   time.sleep(1)
  assert p.returncode==0,(name,lang,arm,p.returncode)
 return path

def parse(name,arm,fs):
 p=out/f'{name}-{arm}-parse.json';p.write_text(json.dumps({'functions':[{'name':hex(a),'code':f.get('code','')}for a,f in fs.items()]}));q=subprocess.run(['/tmp/kuna-go-output/parse',str(p)],capture_output=True,text=True);assert q.returncode in [0,1];output=q.stdout+q.stderr;(out/f'{name}-{arm}-parse.log').write_text(output);failed=sorted(set(re.findall(r'^0x[0-9a-f]+',output,re.MULTILINE)));assert(q.returncode==0)==(not failed);return failed

def run_row(row):
 name,binary,langs=row;digest=sha(binary)
 if name in previous:assert digest==previous[name]['fixture_sha256']
 records=[];changes=[]
 def one_language(lang):
  if name in previous:
   cache=next(item for item in previous[name]['records']if item['language']==lang);before=Path(cache['candidate_file']);assert sha(before)==cache['candidate_sha256']
  else:before=capture(binary,lang,name,'parent')
  old=functions(before);paths={};data={}
  for arm in ['off','on']:
   path=capture(binary,lang,name,arm);new=functions(path);assert old.keys()==new.keys();paths[arm]=str(path);data[arm]=new
   for address in old:
    if old[address]!=new[address]:changes.append({'language':lang,'arm':arm,'address':hex(address),'name':new[address]['name'],'fields':sorted(k for k in set(old[address])|set(new[address])if old[address].get(k)!=new[address].get(k)),'diff':list(difflib.unified_diff(old[address].get('code','').splitlines(),new[address].get('code','').splitlines(),fromfile='parent',tofile=arm,lineterm='')),'classification':'pending'})
  errors={arm:[hex(a)for a in old if data[arm][a].get('error')and not old[a].get('error')]for arm in data};parses=None
  if lang=='go':parses={'parent':parse(name,'parent',old)}|{arm:parse(name,arm,new)for arm,new in data.items()}
  records.append({'language':lang,'parent_file':str(before),'parent_sha256':sha(before),'candidate_files':paths,'candidate_sha256':{arm:sha(path)for arm,path in paths.items()},'functions':len(old),'new_errors':errors,'go_parse':parses,'off_unchanged':old==data['off']})
 with ThreadPoolExecutor(max_workers=len(langs))as pool:
  for f in as_completed([pool.submit(one_language,lang)for lang in langs]):f.result()
 assert sha(binary)==digest;result={'fixture':name,'fixture_path':str(binary),'fixture_sha256':digest,'records':records,'changes':changes};(out/(name+'-record.json')).write_text(json.dumps(result,indent=2)+'\n');print(name,'complete',len(changes),'changed records',flush=True);return result
with ThreadPoolExecutor(max_workers=4)as pool:records=[f.result()for f in as_completed([pool.submit(run_row,row)for row in inputs])]
assert sha(r/'decompiler/target/release/kuna')==candidate_sha and sha(base/'decompiler/target/release/kuna')==parent_sha
result={'distinct_functions':sum(row['records'][0]['functions']for row in records),'language_records_per_arm':sum(item['functions']for row in records for item in row['records']),'changed_on':sum(x['arm']=='on'for row in records for x in row['changes']),'changed_off':sum(x['arm']=='off'for row in records for x in row['changes']),'scope':'Complete 67-image parent/default-off/opt-in comparison on the updated field extraction parent','images':len(records),'parent_cli_sha256':parent_sha,'candidate_cli_sha256':candidate_sha,'records':records,'semantic_review_pending':True};(out/'record.json').write_text(json.dumps(result,indent=2)+'\n');print('Complete Go pair breadth captured; semantic review pending',flush=True)
