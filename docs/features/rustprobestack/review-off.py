from pathlib import Path
import datetime,hashlib,json
w=Path('/tmp/kuna-rust-probestack')
read=lambda n:json.loads((w/n).read_text())
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
build=read('build.json');fresh=read('breadth-off/record.json');old=read('auto-study/breadth-final.json');manual=read('auto-study/manual-review.json')
assert fresh['new_cli_sha256']==build['new_cli_sha256']
assert fresh['parent_cli_sha256']==build['parent_cli_sha256']
assert fresh['images']==41 and fresh['distinct_functions']==33695 and fresh['language_records']==75801 and fresh['changed_records']==0
old_by={row['fixture']:row for row in old['records']};enabled_changes=[];go_failures=[]
def records(path):
 return {f['address']:f for f in json.loads(Path(path).read_text())['functions']}
for row in fresh['records']:
 previous=old_by[row['fixture']];assert row['fixture_sha256']==previous['fixture_sha256']==sha(row['fixture_path'])
 for item in row['records']:
  prev=next(x for x in previous['records']if x['language']==item['language'])
  for arm in ['parent','candidate']:assert sha(item[arm+'_file'])==item[arm+'_sha256']
  parent=records(item['parent_file']);candidate=records(item['candidate_file'])
  assert parent==candidate
  if item['language']=='go':
   p=item['go_parse'];assert p['parent']==p['candidate']
   go_failures.extend({'fixture':row['fixture'],'address':a,'whole_record_equal':True}for a in p['candidate']['failed_addresses'])
  if 'enabled_file'in item:
   assert item['enabled_records_equal_previous_verified_auto']and sha(item['enabled_file'])==item['enabled_sha256']
   enabled=records(item['enabled_file']);assert parent==records(prev['parent_file']);assert enabled==records(prev['candidate_file'])
   enabled_changes.extend({'fixture':row['fixture'],'language':item['language'],'address':hex(a)}for a in parent if parent[a]!=enabled[a])
assert len(enabled_changes)==84 and len(go_failures)==12
assert manual['all_hunks_reviewed']and manual['changed_records']==84
review={'at':datetime.datetime.now(datetime.timezone.utc).isoformat(),'complete':True,'semantic_review_pending':False,'new_cli_sha256':fresh['new_cli_sha256'],'parent_cli_sha256':fresh['parent_cli_sha256'],'images':41,'distinct_functions':33695,'language_records':75801,'default_changed_records':0,'enabled_changed_records':84,'fresh_parent_default_candidate_default_and_selected_candidate_auto':True,'enabled_full_records_equal_previously_reviewed_AUTO_build':True,'affected_parent_records_equal_previous_parent':True,'new_parent_commit':build['parent_commit'],'inherited_go_parser_failures':go_failures,'enabled_changes':enabled_changes,'enabled_manual_review_sha256':sha(w/'auto-study/manual-review.json'),'enabled_manual_review_scope':manual,'limitations':manual['limitations']}
(w/'breadth-off-review.json').write_text(json.dumps(review,indent=2)+'\n')
print('OFF DEFAULT41 images/75801records UNCHANGED; enabled84records exact previous reviewed records;12inherited Go parser failures unchanged')
