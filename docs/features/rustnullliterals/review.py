from pathlib import Path
import datetime,hashlib,json
w=Path('/tmp/kuna-rust-null-literals-8db14');r=Path('/home/mahaloz/kwt/angr-rust-null-literals');read=lambda p:json.loads(Path(p).read_text());sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();build=read(w/'build.json');breadth=read(w/'breadth/record.json');assert sha(r/'decompiler/target/release/kuna')==build['new_cli_sha256']==breadth['new_cli_sha256'];changed=[];inherited=[];occurrences=0
for row in breadth['records']:
 assert sha(row['fixture_path'])==row['fixture_sha256']
 for item in row['records']:
  for arm in ['parent','candidate']:assert sha(item[arm+'_file'])==item[arm+'_sha256']
  before={f['address']:f for f in read(item['parent_file'])['functions']};after={f['address']:f for f in read(item['candidate_file'])['functions']};assert before.keys()==after.keys()
  for address,old in before.items():
   new=after[address]
   if old==new:continue
   assert item['language']=='rust',(row['fixture'],item['language'],hex(address));expected=dict(old);expected['code']=old['code'].replace('core::ptr::null_mut__','core::ptr::null_mut()');assert expected==new,(row['fixture'],hex(address),'unexpected complete record change');n=old['code'].count('core::ptr::null_mut__');assert n>0;occurrences+=n;changed.append({'fixture':row['fixture'],'address':hex(address),'constructors':n,'before':old,'after':new})
  if item['language']=='go':
   assert item['go_parse']['parent']==item['go_parse']['candidate'];inherited.extend((row['fixture'],a)for a in item['go_parse']['candidate']['failed_addresses'])
assert len(changed)==breadth['changed_records'] and changed;native=read(w/'native-proof/record.json');assert native['cli_sha256']==build['new_cli_sha256'] and native['native_values']==150 and native['complete_bodies_unchanged']
(w/'changed-functions.json').write_text(json.dumps(changed,indent=2)+'\n')
review={'at':datetime.datetime.now(datetime.timezone.utc).isoformat(),'complete':True,'semantic_review_pending':False,'new_cli_sha256':build['new_cli_sha256'],'parent_cli_sha256':build['parent_cli_sha256'],'images':breadth['images'],'distinct_functions':breadth['distinct_functions'],'language_records':breadth['language_records'],'changed_records':len(changed),'constructor_occurrences_fixed':occurrences,'all_c_go_records_identical':True,'all_other_rust_fields_and_records_identical':True,'rust_code_changes_exactly_null_constructor_spelling':True,'inherited_go_parser_failures':inherited,'native_values':150,'full_angr_parity_claimed':False};(w/'breadth-review.json').write_text(json.dumps(review,indent=2)+'\n');print('RUST NULL POINTER WHOLE RECORD REVIEW PASSED',review,flush=True)
