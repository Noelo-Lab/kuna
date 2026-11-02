from common import *
import re,collections
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
d=json.loads((w/'breadth/record.json').read_text());old=json.loads((w.parent/'integration-350/breadth/record.json').read_text());assert d['images']==67 and d['changed_off']==0;assert d['candidate_cli_sha256']==sha(r/'decompiler/target/release/kuna');before={x['fixture']:x for x in old['records']};complete=[];refinements=[];parses=0;langcounts=collections.Counter();categorycounts=collections.Counter()
go_type='struct { result0 uint64; result1 uint64 }';go_prefix='func(v [16]byte) '+go_type+' { return '+go_type+'{*(*uint64)(unsafe.Pointer(&v[0])), *(*uint64)(unsafe.Pointer(&v[8]))} }('
known=json.loads((w/'review-first-capture.json').read_text())['remaining_gaps']
for row in d['records']:
 previous=before[row['fixture']];changed={(x['language'],int(x['address'],16)):x for x in row['changes']}
 for item in row['records']:
  lang=item['language'];assert item['off_unchanged']and not any(item['new_errors'].values());olditem=next(x for x in previous['records']if x['language']==lang)
  if item['go_parse']:
   p=item['go_parse'];assert p['on']==p['off']==p['parent'];parses+=len(p['parent'])
  baseline={f['address']:f for f in json.loads(Path(item['parent_file']).read_text())['functions']};earlier={f['address']:f for f in json.loads(Path(olditem['candidate_files']['on']).read_text())['functions']};now={f['address']:f for f in json.loads(Path(item['candidate_files']['on']).read_text())['functions']};assert baseline.keys()==earlier.keys()==now.keys()
  for addr,current in now.items():
   if current!=earlier[addr]:
    assert current['name']=='runtime.(*mheap).allocSpan'and lang in ['c','go'];assert {k for k in current if current[k]!=earlier[addr][k]}<= {'code','line_mappings'}
    code=current['code']
    if lang=='c':code=re.sub(r'\(\(union \{ unsigned __int128 carrier; (__kuna_go_pair_u64_u64) value; \}\)\{\.carrier = (runtime\.\(\*pageAlloc\)\.alloc\([^\n]+\))\}\)\.value',r'(\1)\2',code)
    else:
     lines=[]
     for line in code.splitlines():
      if go_prefix in line:
       start=line.index(go_prefix);prefix=line[:start];suffix=line[start+len(go_prefix):];assert suffix.endswith(')');line=prefix+go_type+'('+suffix
      lines.append(line)
     code='\n'.join(lines)+ ('\n'if code.endswith('\n')else'')
    assert code==earlier[addr]['code'],(row['fixture'],lang,current['name'],'unexpected refinement')
    refinements.append({'fixture':row['fixture'],'language':lang,'address':hex(addr),'name':current['name'],'proof':'Only the invalid opaque-to-pair cast is replaced by the single-evaluation adapter; reversing that substitution yields the previous complete function byte for byte.'})
   if(lang,addr)not in changed:continue
   delta=changed[(lang,addr)];assert delta['arm']=='on';assert set(delta['fields'])<= {'code','variables','line_mappings'};name=current['name'];langcounts[lang]+=1
   if name.startswith('main.')and name!='main.main':category='authored-pair-or-projection'
   elif any('result0' in line or '__kuna_go_pair_' in line for line in delta['diff']):category='imported-pair-or-consumer'
   else:category='downstream-type-propagation'
   limits=[]
   if name=='internal/poll.init.func1':limits.append('go-untyped-bool-return')
   if name in ['runtime.heapBitsSlice','runtime.(*mspan).heapBits']:limits.append('go-untyped-slice-return')
   if not name.startswith('main.'):limits.append('go-untyped-call-arguments')
   if lang=='go'and name in ['runtime.blockAlignSummaryRange','runtime.(*mheap).allocSpan']:limits.append('go-signed-field-assignment')
   if lang=='rust':limits.append('rust-aggregate-carriers')
   delta['classification']=category;delta['review']={'basis':'Full per-function diff retained; all language/name groups structurally reviewed using the recorded normalized views, with native proof for the authored mechanisms.','limitations':limits,'whole_function_native_proof':False};categorycounts[category]+=1
   complete.append({'fixture':row['fixture'],'language':lang,'address':hex(addr),'name':name,'classification':category,'limitations':limits,'parent':baseline[addr],'candidate':current})
assert len(complete)==d['changed_on']==439 and parses==15
(w/'changed-functions.json').write_text(json.dumps(complete,indent=2)+'\n')
result={'complete':True,'semantic_review_pending':False,'review_scope':'Structural review of every changed function group, preservation of all full changed records and explicit remaining limits. Not whole-corpus native equivalence.','whole_corpus_native_equivalence':False,'images':67,'distinct_functions':d['distinct_functions'],'language_records':d['language_records_per_arm'],'changed_records':len(complete),'default_off_changed_records':0,'changed_by_language':dict(langcounts),'classification_counts':dict(categorycounts),'inherited_go_parse_failures':parses,'new_errors':0,'new_go_parse_failures':0,'parent_cli_sha256':d['parent_cli_sha256'],'candidate_cli_sha256':d['candidate_cli_sha256'],'first_capture_review':json.loads((w/'review-first-capture.json').read_text()),'refinements':refinements,'remaining_gaps':known,'records':d['records']};(w/'breadth-review.json').write_text(json.dumps(result,indent=2)+'\n');print('ALL67IMAGESREVIEWED',len(complete),'changes',len(refinements),'opaque refinements',dict(langcounts),'with explicit limitations',flush=True)

prior=json.loads((w.parent/'integration-350-round2/breadth/record.json').read_text());prior_rows={row['fixture']:row for row in prior['records']};matched=0
for row in d['records']:
 for item in row['records']:
  previous=next(x for x in prior_rows[row['fixture']]['records']if x['language']==item['language'])
  for arm in ['off','on']:
   oldfs={f['address']:f for f in json.loads(Path(previous['candidate_files'][arm]).read_text())['functions']};newfs={f['address']:f for f in json.loads(Path(item['candidate_files'][arm]).read_text())['functions']};assert oldfs==newfs,(row['fixture'],item['language'],arm);matched+=len(newfs)
(w/'allocation-refinement-review.json').write_text(json.dumps({'previous_cli_sha256':prior['candidate_cli_sha256'],'candidate_cli_sha256':d['candidate_cli_sha256'],'images':67,'matched_function_records':matched,'changed_function_records':0,'claim':'Every complete enabled and disabled function record equals the previously reviewed candidate after replacing result lists with fixed slots and bypassing disabled call dispatch.'},indent=2)+'\n');print('Allocation refinement preserves all',matched,'complete function records',flush=True)
