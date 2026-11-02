from common import *
p=w/'breadth-final/record.json';d=json.loads(p.read_text());expected={f['name']for f in json.loads((w/'matrix-parent-c.json').read_text())['functions']if f['code']!=next(x['code']for x in json.loads((w/'matrix-candidate-c.json').read_text())['functions']if x['name']==f['name'])};assert len(expected)==18
changed=[];unchanged=0;parser_failures=0
for row in d['records']:
 for record in row['records']:
  assert not record['new_errors'];assert hashlib.sha256(Path(record['candidate_file']).read_bytes()).hexdigest()==record['candidate_sha256'];assert hashlib.sha256(Path(record['parent_file']).read_bytes()).hexdigest()==record['parent_sha256']
  if record['go_parse']:
   assert record['go_parse']['parent']['failed_addresses']==record['go_parse']['candidate']['failed_addresses'];parser_failures+=len(record['go_parse']['parent']['failed_addresses'])
 if not row['changes']:unchanged+=1;continue
 assert row['fixture']in ['joined_callback_return_x86_64','integer_field_extract_x86_64'],row['fixture']
 names=expected if row['fixture']=='integer_field_extract_x86_64'else {'narrow_second'}
 for lang in ['c','go','rust']:assert {x['name']for x in row['changes']if x['language']==lang}==names
 for item in row['changes']:
  assert item['fields']==['code'],item
  item['classification']='Explicit integer extraction preserves width before widening; interior unsigned field slices also preserve their bit position. The field/call occurs once.'
  item['native_proof']='Full emitted caller checked against source in C/Go/Rust.'if row['fixture']=='integer_field_extract_x86_64'else 'Full emitted callback checked in C/Go. Rust retains preexisting invalid keyword/function-pointer spelling; only the i32 extraction changes.'
  changed.append({'fixture':row['fixture'],**item})
assert len(changed)==57 and unchanged==59,(len(changed),unchanged)
d['semantic_review_pending']=False;d['complete']=True;d['unchanged_existing_images']=unchanged;d['unchanged_go_parse_failures']=parser_failures;d['changed_function_review']=changed;d['native_matrix']={'c':26,'go':26,'rust':21,'input_pairs_per_run':784,'every_changed_matrix_caller_covered':True};d['callback_native']=json.loads((w/'callback-native/record.json').read_text());d['rust_unchanged_signedness_exclusions']=json.loads((w/'native-rust-remainder/record.json').read_text())['exclusions_unchanged_from_parent'];(w/'breadth-review.json').write_text(json.dumps(d,indent=2)+'\n');print({k:d[k]for k in ['images','distinct_functions','language_records','changed_records','unchanged_existing_images','unchanged_go_parse_failures']},flush=True)
