from pathlib import Path
import datetime,hashlib,json,re
w=Path('/tmp/kuna-rust-empty-match-db2d');r=Path('/home/mahaloz/kwt/angr-rust-empty-match');read=lambda n:json.loads((w/n).read_text());sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();build=read('build.json');breadth=read('breadth/record.json');native=read('native-proof/record.json');assert breadth['new_cli_sha256']==native['cli_sha256']==build['new_cli_sha256']==sha(r/'decompiler/target/release/kuna')
pattern=re.compile(r'(=>\s*\{)\s*break;\s*(\})');normalize=lambda text:re.sub(r'\s+','',text);changes=[];unchanged=0;go_failures=[]
for row in breadth['records']:
 assert sha(row['fixture_path'])==row['fixture_sha256']
 for item in row['records']:
  for arm in ['parent','candidate']:assert sha(item[arm+'_file'])==item[arm+'_sha256']
  before={f['address']:f for f in json.loads(Path(item['parent_file']).read_text())['functions']};after={f['address']:f for f in json.loads(Path(item['candidate_file']).read_text())['functions']};assert before.keys()==after.keys()
  if item['language']=='go':
   assert item['go_parse']['parent']==item['go_parse']['candidate'];go_failures.extend({'fixture':row['fixture'],'address':a}for a in item['go_parse']['candidate']['failed_addresses'])
  for address in before:
   a,b=before[address],after[address]
   if a==b:unchanged+=1;continue
   assert item['language']=='rust';fields=[k for k in set(a)|set(b)if a.get(k)!=b.get(k)];assert set(fields)<={'code','line_mappings','variables'},(row['fixture'],hex(address),fields)
   transformed,count=pattern.subn(r'\1\2',a['code']);assert count>0;assert normalize(transformed)==normalize(b['code']),(row['fixture'],hex(address),'change beyond empty switch-exit arm')
   assert a['code'].count('break;')-b['code'].count('break;')==count
   changes.append({'fixture':row['fixture'],'language':'rust','address':hex(address),'name':b['name'],'empty_exit_arms':count,'fields':fields,'classification':'Only break in a direct empty match arm disappears; remaining text is identical except whitespace.','before_sha256':hashlib.sha256(a['code'].encode()).hexdigest(),'after_sha256':hashlib.sha256(b['code'].encode()).hexdigest()})
assert len(changes)==breadth['changed_records'];assert native['native_values']==6900
review={'at':datetime.datetime.now(datetime.timezone.utc).isoformat(),'complete':True,'semantic_review_pending':False,'new_cli_sha256':breadth['new_cli_sha256'],'parent_cli_sha256':breadth['parent_cli_sha256'],'images':breadth['images'],'distinct_functions':breadth['distinct_functions'],'language_records':breadth['language_records'],'changed_records':len(changes),'unchanged_records':unchanged,'all_C_and_Go_records_identical':True,'all_Rust_changes_only_empty_direct_exit_arm_break_removal':True,'inherited_go_parser_failures':go_failures,'changed':changes,'native_values':6900,'full_angr_parity_claimed':False,'limitations':['This does not fix nonempty switch-arm transfers, C fallthrough, Rust residual gotos, or unrelated inferred-type/string rendering errors.','The unchanged full emitted native proof covers the authored fixture. Upstream corpus changes are checked as exact empty-arm token edits; supplied upstream executables are not run.']};(w/'breadth-review.json').write_text(json.dumps(review,indent=2)+'\n');print('BREADTH REVIEW COMPLETE',len(changes),'Rust functions;',sum(c['empty_exit_arms']for c in changes),'empty exit arms;C/Go complete records identical.',flush=True)
