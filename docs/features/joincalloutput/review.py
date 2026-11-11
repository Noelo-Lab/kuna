from common import *
record=json.loads((w/'breadth/record.json').read_text());changes=[];inherited_go=[]
expected={'first','second','narrow_second'}
for row in record['records']:
 if row['fixture']!='joined_callback_return_x86_64':assert not row['changes'],row['fixture']
 else:
  for language in ['c','go','rust']:
   rows=[c for c in row['changes']if c['language']==language];assert {c['name']for c in rows}==expected,rows
   for c in rows:
    assert set(c['fields'])<={'code','line_mappings','variables'},c
    field='first'if c['name']=='first'else'second'
    c['classification']='Restore the surviving ABI return component from one typed callback invocation.'
    if c['name']=='narrow_second':c['classification']+=' C return conversion narrows the restored member; Go/Rust still require an explicit conversion (preexisting P9 gap with independent main83 witness).'
    c['native_c_complete_body_verified']=True;c['native_go_complete_body_verified']=language=='go'and c['name']!='narrow_second';c['rust_native_execution_claimed']=False
    changes.append({'fixture':row['fixture'],**c})
 for item in row['records']:
  assert not item['new_errors']
  if item.get('go_parse'):
   old=set(item['go_parse']['parent']['failed_addresses']);new=set(item['go_parse']['candidate']['failed_addresses']);assert old==new
   if old:inherited_go.append({'fixture':row['fixture'],'addresses':sorted(old)})
assert record['images']==60 and len(changes)==9
assert json.loads((w/'native-final/record.json').read_text())['candidate_input_comparisons']==338
native_go=json.loads((w/'native-go/record.json').read_text());assert all(row['seven_complete_bodies_pass']for row in native_go['records']['candidate'].values())
summary={'complete':True,'semantic_review_pending':False,'images':record['images'],'distinct_functions':record['distinct_functions'],'language_records':record['language_records'],'changed_records':len(changes),'unchanged_existing_images':59,'native_c_full_bodies':8,'native_c_input_comparisons':338,'native_go_full_bodies':7,'native_go_input_comparisons':338,'go_narrowed_callback_compiles':False,'rust_native_execution_claimed':False,'inherited_go_parse_failures':inherited_go,'followup':'P9 special-printing SUBPIECE of an implied struct field omits the narrowing conversion; independently reproduced on unmodified main83 for a direct call in Go and Rust.','changes':changes};(w/'breadth-review.json').write_text(json.dumps(summary,indent=2)+'\n');print('Review complete: 9 restored-component records; existing59 images unchanged; Go/Rust narrowing limitation recorded.',flush=True)
