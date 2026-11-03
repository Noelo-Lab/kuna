from common import *
from concurrent.futures import ThreadPoolExecutor,as_completed
import difflib
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest();base=Path('/home/mahaloz/kwt/angr-rust-array-address');prior=json.loads(Path('/tmp/kuna-rust-array-address/breadth/record.json').read_text());build=json.loads((w/'build.json').read_text());out=w/'breadth';out.mkdir(exist_ok=False)
assert sha(r/'decompiler/target/release/kuna')==build['new_cli_sha256'] and sha(base/'decompiler/target/release/kuna')==build['parent_cli_sha256']
assert prior['candidate_cli_sha256']==build['parent_cli_sha256'];assert not subprocess.check_output(['git','status','--porcelain'],cwd=base);assert subprocess.check_output(['git','rev-parse','HEAD'],cwd=base,text=True).strip()==build['parent_commit']
def functions(path):
 d=json.loads(path.read_text());assert not d.get('error');result={f['address']:f for f in d['functions']};assert len(result)==len(d['functions'])and result;return result
def capture(root,command,path):
 command=[str(root/'decompiler/target/release/kuna')]+command[1:]
 with path.open('w')as stdout,path.with_suffix('.log').open('w')as stderr:
  p=subprocess.Popen(command,cwd=root,env=env|{'KUNA_SPECS':str(root/'specs')},stdout=stdout,stderr=stderr,start_new_session=True)
  while p.poll()is None:
   memory={line.split(':')[0]:int(line.split()[1])for line in Path('/proc/meminfo').read_text().splitlines()}
   if 1-memory['MemAvailable']/memory['MemTotal']>=.9:
    os.killpg(p.pid,signal.SIGTERM);p.wait();(w/'memory-limit-hit.json').write_text(json.dumps(memory));raise RuntimeError('90% RAM; own capture stopped; user help required')
   time.sleep(1)
  assert p.returncode==0,(path,p.returncode)
 return command

def metadata(before,after,name,pair,address):
 assert len(before['variables'])==len(after['variables']);changes=[]
 for a,b in zip(before['variables'],after['variables']):
  assert {k:v for k,v in a.items()if k not in ['line_numbers','addresses']}=={k:v for k,v in b.items()if k not in ['line_numbers','addresses']},(name,pair,hex(address),'variable identity/type/storage changed',a,b)
  if a==b:continue
  for variable,function in [(a,before),(b,after)]:
   mappings={item['line_number']:item['addresses']for item in function['line_mappings']};assert variable['addresses']==sorted({v for line in variable['line_numbers']for v in mappings.get(line,[])});assert all(0<line<=len(function['code'].splitlines())for line in variable['line_numbers'])
  changes.append({'fixture':name,'gopairreturns':pair,'address':hex(address),'function':after['name'],'name':a['name'],'parent_lines':a['line_numbers'],'candidate_lines':b['line_numbers'],'parent_addresses':a['addresses'],'candidate_addresses':b['addresses'],'only_provenance_changed':True,'addresses_equal_exact_line_mapping_union':True})
 return changes

def row_job(row):
 name=row['fixture'];binary=Path(row['fixture_path']);assert sha(binary)==row['fixture_sha256'];records=[];changes=[];provenance=[]
 def language_job(item):
  language=item['language'];pair=item['gopairreturns'];command=list(item['command']);command[command.index('gopairreturns')+1]=pair
  before=Path(item['candidate_file']);assert sha(before)==item['candidate_sha256'];after=out/f'{name}-{language}-{pair}-candidate.json';parent_command=list(item['command']);candidate_command=capture(r,command,after);old=functions(before);new=functions(after);assert sha(before)==item['candidate_sha256'];assert old.keys()==new.keys();errors=[hex(a)for a in old if new[a].get('error')and not old[a].get('error')];assert not errors,(name,language,pair,errors);local=[];local_meta=[]
  for address in old:
   if old[address]==new[address]:continue
   fields=sorted(k for k in set(old[address])|set(new[address])if old[address].get(k)!=new[address].get(k));assert language=='rust'and set(fields)<={'code','line_mappings','variables'},(name,language,pair,hex(address),fields)
   if 'variables'in fields:local_meta+=metadata(old[address],new[address],name,pair,address)
   local.append({'language':language,'gopairreturns':pair,'address':hex(address),'name':new[address]['name'],'fields':fields,'parent':old[address],'candidate':new[address],'diff':list(difflib.unified_diff(old[address].get('code','').splitlines(),new[address].get('code','').splitlines(),fromfile='parent',tofile='candidate',lineterm='')),'review_pending':True})
  return {'language':language,'gopairreturns':pair,'functions':len(old),'parent_file':str(before),'parent_sha256':sha(before),'candidate_file':str(after),'candidate_sha256':sha(after),'command':candidate_command,'parent_command':parent_command,'new_errors':errors,'fresh_parent_capture':False,'frozen_parent_capture_from_pr':1467},local,local_meta
 with ThreadPoolExecutor(max_workers=len(row['records']))as pool:
  for future in as_completed([pool.submit(language_job,item)for item in row['records']]):
   record,local,meta=future.result();records.append(record);changes+=local;provenance+=meta
 assert sha(binary)==row['fixture_sha256'];result={'fixture':name,'fixture_path':str(binary),'fixture_sha256':row['fixture_sha256'],'records':records,'changes':changes,'variable_provenance':provenance};path=out/(name+'-record.json');temp=path.with_suffix('.json.tmp');temp.write_text(json.dumps(result,indent=2)+'\n');temp.replace(path);print(name,'complete',len(changes),'Rust changes',len(provenance),'variable provenance changes',flush=True);return result
with ThreadPoolExecutor(max_workers=4)as pool:records=[future.result()for future in as_completed([pool.submit(row_job,row)for row in prior['records']])]
assert sha(r/'decompiler/target/release/kuna')==build['new_cli_sha256'] and sha(base/'decompiler/target/release/kuna')==build['parent_cli_sha256']
provenance=[item for row in records for item in row['variable_provenance']];(out/'metadata-review.json').write_text(json.dumps({'complete':True,'scope':'Variable semantic metadata is identical; each changed address list equals the exact union of its source-line mappings.','changed_variable_records':len(provenance),'records':provenance},indent=2)+'\n')
record={'status':'captured; structural review pending','scope':'72 fresh candidate captures compared to exact frozen parent captures from published raw-array-address PR1467. C/Go records unchanged; Rust code/line mappings/variable provenance may change. Probestack static only and never executed.','images':len(records),'distinct_functions':sum(row['records'][0]['functions']for row in records),'language_function_records':sum(item['functions']for row in records for item in row['records']),'changed_rust_records':sum(len(row['changes'])for row in records),'parent_cli_sha256':build['parent_cli_sha256'],'candidate_cli_sha256':build['new_cli_sha256'],'records':records};temp=out/'record.json.tmp';temp.write_text(json.dumps(record,indent=2)+'\n');temp.replace(out/'record.json');print('Array breadth complete',record['images'],record['changed_rust_records'],'Rust changes',flush=True)
