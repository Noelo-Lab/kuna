from pathlib import Path
import datetime,hashlib,json
w=Path('/tmp/kuna-highslice-optin');old=Path('/tmp/kuna-highslice-after-empty-match');r=Path('/home/mahaloz/kwt/angr-narrow-table-guard');read=lambda p:json.loads(Path(p).read_text());sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();build=read(w/'build.json');off=read(w/'breadth-off/record.json');on=read(w/'breadth-on/record.json');prior=read(old/'breadth/record.json');prior_review=read(old/'breadth-review.json');assert prior_review['complete'];assert off['new_cli_sha256']==on['new_cli_sha256']==build['new_cli_sha256']==sha(r/'decompiler/target/release/kuna');assert off['changed_records']==0 and on['changed_records']==33;previous={row['fixture']:row for row in prior['records']};assert len(off['records'])==len(on['records'])==len(previous)==43;inherited=[]
for breadth,enabled in [(off,False),(on,True)]:
 for row in breadth['records']:
  assert sha(row['fixture_path'])==row['fixture_sha256']==previous[row['fixture']]['fixture_sha256']
  for item in row['records']:
   for arm in ['parent','candidate']:assert sha(item[arm+'_file'])==item[arm+'_sha256']
   before=read(item['parent_file'])['functions'];after=read(item['candidate_file'])['functions'];prior_item=next(x for x in previous[row['fixture']]['records']if x['language']==item['language']);assert sha(prior_item['candidate_file'])==prior_item['candidate_sha256']
   if enabled:assert after==read(prior_item['candidate_file'])['functions'],(row['fixture'],item['language'],'enabled output drifted')
   else:assert after==before,(row['fixture'],item['language'],'default output changed')
   if item['language']=='go':
    assert item['go_parse']['parent']==item['go_parse']['candidate']
    if not enabled:inherited.extend((row['fixture'],a)for a in item['go_parse']['candidate']['failed_addresses'])
native=[read(w/name/'record.json')for name in ['native-proof','language-native','combined-native']];assert all(x['cli_sha256']==build['new_cli_sha256']for x in native);assert sum(x['native_values']for x in native)==26628
review={'at':datetime.datetime.now(datetime.timezone.utc).isoformat(),'complete':True,'semantic_review_pending':False,'new_cli_sha256':build['new_cli_sha256'],'parent_cli_sha256':build['parent_cli_sha256'],'images':43,'distinct_functions':off['distinct_functions'],'language_records_per_mode':off['language_records'],'default_changed_records':0,'explicit_on_changed_records':33,'all_default_records_identical_to_parent':True,'all_explicit_on_records_identical_to_previously_reviewed_candidate':True,'inherited_go_parser_failures':inherited,'native_values':26628,'previous_complete_review_sha256':sha(old/'breadth-review.json'),'enabled_study_sha256':sha(old/'timing-result.json'),'enabled_study_passed':False,'option':'highsliceguard','default':'off','disabled_in_all_presets':True,'full_angr_parity_claimed':False};(w/'breadth-review.json').write_text(json.dumps(review,indent=2)+'\n');print('OPT-IN REVIEW PASS:alldefault75krecords unchanged;explicitONexactprevious33changes;26,628nativevalues.',flush=True)
