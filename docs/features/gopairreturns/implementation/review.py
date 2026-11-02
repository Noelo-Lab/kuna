from common import *
import collections,difflib
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
d=json.loads((w/'breadth/record.json').read_text());old=w.parent/'integration-a324';prior=json.loads((old/'changed-functions.json').read_text());reviewed={(x['fixture'],x['language'],x['address']):x for x in prior};assert d['images']==72 and d['changed_off']==0;assert d['candidate_cli_sha256']==sha(r/'decompiler/target/release/kuna');complete=[];pending=[];parses=0;langcounts=collections.Counter();categorycounts=collections.Counter();seen=set()
for row in d['records']:
 changes={(x['language'],x['address']):x for x in row['changes']}
 for item in row['records']:
  lang=item['language'];assert item['off_unchanged']and not any(item['new_errors'].values())
  if item['go_parse']:
   p=item['go_parse'];assert p['on']==p['off']==p['parent'];parses+=len(p['parent'])
  baseline={f['address']:f for f in json.loads(Path(item['parent_file']).read_text())['functions']};now={f['address']:f for f in json.loads(Path(item['candidate_files']['on']).read_text())['functions']};assert baseline.keys()==now.keys()
  for addr,current in now.items():
   if(lang,hex(addr))not in changes:continue
   change=changes[(lang,hex(addr))];assert change['arm']=='on'and set(change['fields'])<={'code','variables','line_mappings'};key=(row['fixture'],lang,hex(addr));seen.add(key);earlier=reviewed.get(key)
   entry={'fixture':row['fixture'],'language':lang,'address':hex(addr),'name':current['name'],'parent':baseline[addr],'candidate':current}
   if earlier and baseline[addr]==earlier['parent']and current==earlier['candidate']:
    entry.update(classification=earlier['classification'],limitations=earlier['limitations'],review_basis='Both complete before/after function records equal the previously reviewed a324 comparison byte for byte.');complete.append(entry);langcounts[lang]+=1;categorycounts[earlier['classification']]+=1
   else:entry.update(previous=earlier,diff=change['diff']);pending.append(entry)
result={'complete':not pending,'semantic_review_pending':bool(pending),'review_scope':'Every complete changed before/after record must equal its prior reviewed pair or remain explicitly pending. No whole-runtime native equivalence claim.','whole_corpus_native_equivalence':False,'images':d['images'],'distinct_functions':d['distinct_functions'],'language_records':d['language_records_per_arm'],'changed_records':len(complete)+len(pending),'default_off_changed_records':0,'changed_by_language':dict(langcounts),'classification_counts':dict(categorycounts),'inherited_go_parse_failures':parses,'new_errors':0,'new_go_parse_failures':0,'parent_cli_sha256':d['parent_cli_sha256'],'candidate_cli_sha256':d['candidate_cli_sha256'],'first_capture_review':json.loads((w/'review-first-capture.json').read_text()),'remaining_gaps':json.loads((w/'review-first-capture.json').read_text())['remaining_gaps'],'prior_changes_no_longer_present':[list(key)for key in sorted(set(reviewed)-seen)],'pending':pending,'records':d['records']}
(w/'changed-functions.json').write_text(json.dumps(complete+pending,indent=2)+'\n');(w/'breadth-review.json').write_text(json.dumps(result,indent=2)+'\n');print('Reviewed',len(complete),'complete identical change pairs;',len(pending),'pending; parser failures',parses,flush=True);assert not pending,'Current-main changes require review.'
