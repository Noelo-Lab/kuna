from pathlib import Path
import datetime,hashlib,json
w=Path('/tmp/kuna-rust-probestack');fresh=json.loads((w/('breadth-final.json'if(w/'breadth-final.json').exists()else'breadth-mutable/record.json')).read_text());old=json.loads((w/'breadth/record.json').read_text());previous={row['fixture']:row for row in old['records']}
details=[];parse=[];metadata=[];controls=[]
for row in fresh['records']:
 oldrow=previous[row['fixture']];oldlangs={x['language']:x for x in oldrow['records']}
 for item in row['records']:
  lang=item['language'];before=json.loads(Path(item['parent_file']).read_text())['functions'];after=json.loads(Path(item['candidate_file']).read_text())['functions']
  assert before==json.loads(Path(oldlangs[lang]['parent_file']).read_text())['functions']
  assert after==json.loads(Path(oldlangs[lang]['candidate_file']).read_text())['functions']
  assert hashlib.sha256(Path(item['parent_file']).read_bytes()).hexdigest()==item['parent_sha256']
  assert hashlib.sha256(Path(item['candidate_file']).read_bytes()).hexdigest()==item['candidate_sha256']
  oldfuncs={f['address']:f for f in before};newfuncs={f['address']:f for f in after}
  if item.get('go_parse'):
   a=item['go_parse']['parent']['failed_addresses'];b=item['go_parse']['candidate']['failed_addresses'];assert b==a
   inherited=[a for a in a if oldfuncs[int(a,16)]!=newfuncs[int(a,16)]]
   parse.append({'fixture':row['fixture'],'inherited_failure_addresses':b,'changed_failure_records':inherited})
  for a,x in oldfuncs.items():
   y=newfuncs[a]
   if x==y:continue
   assert row['fixture']in ['probe-native','probe-buffer','probe-upstream']
   assert '__rust_probestack()'in x['code'] and '__rust_probestack()'not in y['code']
   assert not y.get('error')
   for z in [x,y]:
    lines=len(z['code'].splitlines())
    assert all(0<v['line_number']<=lines for v in z.get('line_mappings',[]))
    assert all(0<line<=lines for v in z.get('variables',[])for line in v['line_numbers'])
   slim=lambda z:[{k:v for k,v in x.items()if k not in ['line_numbers','addresses']}for x in z.get('variables',[])]
   vx=slim(x);vy=slim(y)
   metadata.append({'fixture':row['fixture'],'language':lang,'address':hex(a),'name':x['name'],'removed':[v for v in vx if v not in vy],'added':[v for v in vy if v not in vx]})
   details.append({'fixture':row['fixture'],'language':lang,'address':hex(a),'name':x['name'],'fields':sorted(k for k in set(x)|set(y)if x.get(k)!=y.get(k))})
  if row['fixture']not in ['probe-native','probe-buffer','probe-upstream']:controls.append({'fixture':row['fixture'],'language':lang,'records_equal':True,'functions':len(after)})
assert len(details)==84 and fresh['images']==41
report={'at':datetime.datetime.now(datetime.timezone.utc).isoformat(),'fresh_captures':True,'pre_guard_captures_not_reused':True,'all_function_records_equal_already_inspected_pre_guard_diff':True,'images':41,'distinct_functions':fresh['distinct_functions'],'language_records':fresh['language_records'],'changed_records':details,'controls':controls,'go_parse':parse,'semantic_review_pending':True}
(w/'final-breadth-structural.json').write_text(json.dumps(report,indent=2)+'\n');(w/'final-metadata-brief.json').write_text(json.dumps(metadata,indent=2)+'\n')
print('All fresh records equal previously inspected code changes;84 changed records, metadata review pending')
