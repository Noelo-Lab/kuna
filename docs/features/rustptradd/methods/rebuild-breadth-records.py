from common import *
import difflib,datetime
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest();base=Path('/home/mahaloz/kwt/angr-rust-store-conversions');prior=json.loads(Path('/tmp/kuna-rust-store-conversions/breadth/record.json').read_text());build=json.loads((w/'build.json').read_text());out=w/'breadth';records=[];metadata=[];all_captures=[]
assert sha(r/'decompiler/target/release/kuna')==build['new_cli_sha256'];assert sha(base/'decompiler/target/release/kuna')==build['parent_cli_sha256']
sources=json.loads((w/'production-source-before-validation.json').read_text());assert all(sha(r/p)==digest for p,digest in sources.items())
def functions(path):
 d=json.loads(path.read_text());assert not d.get('error');result={f['address']:f for f in d['functions']};assert len(result)==len(d['functions'])and result;return result
def variable_metadata(old,new,fixture,pair,address):
 assert len(old['variables'])==len(new['variables'])
 for a,b in zip(old['variables'],new['variables']):
  assert {k:v for k,v in a.items()if k not in ['line_numbers','addresses']}=={k:v for k,v in b.items()if k not in ['line_numbers','addresses']},(fixture,pair,hex(address),'semantic variable field',a,b)
  if a==b:continue
  for var,fn in [(a,old),(b,new)]:
   maps={x['line_number']:x['addresses']for x in fn['line_mappings']};assert var['addresses']==sorted({v for line in var['line_numbers']for v in maps.get(line,[])}),(fixture,hex(address),'variable provenance union mismatch',var)
   assert all(0<line<=len(fn['code'].splitlines())for line in var['line_numbers'])
  metadata.append({'fixture':fixture,'gopairreturns':pair,'address':hex(address),'function':new['name'],'name':a['name'],'parent_lines':a['line_numbers'],'candidate_lines':b['line_numbers'],'parent_addresses':a['addresses'],'candidate_addresses':b['addresses'],'only_provenance_changed':True,'addresses_equal_exact_line_mapping_union':True})
for row in prior['records']:
 name=row['fixture'];binary=Path(row['fixture_path']);assert sha(binary)==row['fixture_sha256'];items=[];changes=[]
 for item in row['records']:
  language=item['language'];pair=item['gopairreturns'];before=Path(item['candidate_file']);after=out/f'{name}-{language}-{pair}-candidate.json';assert sha(before)==item['candidate_sha256'];old=functions(before);new=functions(after);assert old.keys()==new.keys()
  errors=[hex(a)for a in old if new[a].get('error')and not old[a].get('error')];assert not errors,(name,language,pair,errors)
  for address in old:
   if old[address]==new[address]:continue
   fields=sorted(k for k in set(old[address])|set(new[address])if old[address].get(k)!=new[address].get(k));assert language=='rust'and set(fields)<={'code','line_mappings','variables'},(name,language,pair,hex(address),fields)
   if 'variables'in fields:variable_metadata(old[address],new[address],name,pair,address)
   changes.append({'language':language,'gopairreturns':pair,'address':hex(address),'name':new[address]['name'],'fields':fields,'parent':old[address],'candidate':new[address],'diff':list(difflib.unified_diff(old[address].get('code','').splitlines(),new[address].get('code','').splitlines(),fromfile='parent',tofile='candidate',lineterm='')),'review_pending':True})
  command=[arg.replace(str(base),str(r))for arg in item['command']];command[command.index('gopairreturns')+1]=pair
  entry={'language':language,'gopairreturns':pair,'functions':len(old),'parent_file':str(before),'parent_sha256':sha(before),'candidate_file':str(after),'candidate_sha256':sha(after),'command':command,'new_errors':errors,'fresh_parent_capture':False};items.append(entry);all_captures.append({'path':str(after),'sha256':entry['candidate_sha256']})
 result={'fixture':name,'fixture_path':str(binary),'fixture_sha256':row['fixture_sha256'],'records':items,'changes':changes};record_path=out/(name+'-record.json')
 if record_path.exists():
  existing=json.loads(record_path.read_text());assert sorted(existing['records'],key=lambda x:(x['language'],x['gopairreturns']))==sorted(items,key=lambda x:(x['language'],x['gopairreturns']));assert sorted(existing['changes'],key=lambda x:(x['gopairreturns'],x['address']))==sorted(changes,key=lambda x:(x['gopairreturns'],x['address']));result=existing
 else:
  temp=record_path.with_suffix('.json.tmp');temp.write_text(json.dumps(result,indent=2)+'\n');temp.replace(record_path)
 records.append(result);print(name,len(changes),'changed records recovered',flush=True)
record={'status':'captured; semantic review pending','scope':'72-image complete function records; C/Go unchanged. Rust code, line mappings, and variable provenance may change. Variable identity/type/storage are exact; changed addresses equal the union of the recorded variable lines. Upstream probestack static only, never executed.','images':len(records),'distinct_functions':sum(row['records'][0]['functions']for row in records),'language_function_records':sum(item['functions']for row in records for item in row['records']),'changed_rust_records':sum(len(row['changes'])for row in records),'parent_cli_sha256':build['parent_cli_sha256'],'candidate_cli_sha256':build['new_cli_sha256'],'records':records}
(out/'metadata-review.json').write_text(json.dumps({'scope':'Variable semantic metadata is identical. Changed variable addresses are independently recomputed as the exact union of source-line mappings on that variable occurrence list, matching resolve_markup_provenance_with_addresses.','changed_variable_records':len(metadata),'records':metadata,'complete':True},indent=2)+'\n')
(w/'complete-capture-manifest.json').write_text(json.dumps({'at':datetime.datetime.now(datetime.timezone.utc).isoformat(),'captures':all_captures,'reused_existing_captures_without_decompilation':True,'candidate_cli_sha256':build['new_cli_sha256']},indent=2)+'\n')
assert all(sha(r/p)==digest for p,digest in sources.items());assert sha(r/'decompiler/target/release/kuna')==build['new_cli_sha256']and sha(base/'decompiler/target/release/kuna')==build['parent_cli_sha256']
temp=out/'record.json.tmp';temp.write_text(json.dumps(record,indent=2)+'\n');temp.replace(out/'record.json');print('Recovered',len(records),'images;',record['changed_rust_records'],'Rust changes;',len(metadata),'variable provenance records',flush=True)
