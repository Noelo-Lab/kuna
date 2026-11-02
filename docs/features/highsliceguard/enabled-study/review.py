from pathlib import Path
import datetime,hashlib,json,re
w=Path('/tmp/kuna-highslice-after-empty-match');old=Path('/tmp/kuna-narrow-table-guard');r=Path('/home/mahaloz/kwt/angr-narrow-table-guard');read=lambda p:json.loads(Path(p).read_text());sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();build=read(w/'build.json');current=read(w/'breadth/record.json');prior=read(old/'breadth/record.json');c_native=read(w/'native-proof/record.json');languages=read(w/'language-native/record.json');combined=read(w/'combined-native/record.json');static=read(old/'static-replay.json');assert current['new_cli_sha256']==build['new_cli_sha256']==c_native['cli_sha256']==languages['cli_sha256']==combined['cli_sha256']==sha(r/'decompiler/target/release/kuna');assert c_native['native_values']==languages['native_values']==9864 and combined['native_values']==6900;assert static['tables']==6 and static['rows']==246
for name,digest in static['inputs'].items():assert sha(old/name)==digest
previous={row['fixture']:row for row in prior['records']};old_changes={(row['fixture'],c['language'],c['address'])for row in prior['records']for c in row['changes']};changes=[];inherited_parser=[];dependency_edits=[];normalize=lambda s:re.sub(r'\s+','',s);empty=re.compile(r'(=>\s*\{)\s*break;\s*(\})')
def fs(path,digest):
 assert sha(path)==digest
 return {f['address']:f for f in read(path)['functions']}
for row in current['records']:
 assert sha(row['fixture_path'])==row['fixture_sha256']
 for item in row['records']:
  before=fs(item['parent_file'],item['parent_sha256']);after=fs(item['candidate_file'],item['candidate_sha256']);assert before.keys()==after.keys();lang=item['language']
  if lang=='go':assert item['go_parse']['parent']==item['go_parse']['candidate'];inherited_parser.extend((row['fixture'],a)for a in item['go_parse']['candidate']['failed_addresses'])
  if row['fixture']in previous:
   old_item=next(x for x in previous[row['fixture']]['records']if x['language']==lang);old_after=fs(old_item['candidate_file'],old_item['candidate_sha256']);assert after.keys()==old_after.keys()
   for address,f in after.items():
    prior_f=old_after[address]
    if lang in ['c','go']:assert f==prior_f,(row['fixture'],lang,hex(address),'changed beyond previously reviewed table recovery')
    elif f!=prior_f:
     fields={k for k in set(f)|set(prior_f)if f.get(k)!=prior_f.get(k)};assert fields<={'code','variables','line_mappings'};text,count=empty.subn(r'\1\2',prior_f['code']);assert count and normalize(text)==normalize(f['code']),(row['fixture'],hex(address),'Rust change beyond dependency');dependency_edits.append({'fixture':row['fixture'],'address':hex(address),'empty_arm_breaks':count})
  for address,a in before.items():
   b=after[address]
   if a==b:continue
   key=(row['fixture'],lang,hex(address));assert key in old_changes,key;fields={k for k in set(a)|set(b)if a.get(k)!=b.get(k)};assert fields<={'code','variables','line_mappings'};ac=a['code'];bc=b['code'];assert ac.count('jump-as-call')-bc.count('jump-as-call')==1,key;pattern=r'\bmatch\s'if lang=='rust'else r'\bswitch\s*\(?';assert len(re.findall(pattern,bc))-len(re.findall(pattern,ac))==1,key;assert len(re.findall(r'\bgoto\s',ac))==len(re.findall(r'\bgoto\s',bc)),key
   if lang=='rust':assert not empty.search(bc),key
   changes.append({'fixture':row['fixture'],'language':lang,'address':hex(address),'name':b['name'],'fields':sorted(fields),'classification':'Exact prior C/Go table recovery; Rust differs only by the separately validated empty-arm correction. Authored execution or archived static table/ABI proof covers the recovered transfer.'})
assert {(x['fixture'],x['language'],x['address'])for x in changes}==old_changes and len(changes)==33;assert len(current['records'])==43
review={'at':datetime.datetime.now(datetime.timezone.utc).isoformat(),'complete':True,'semantic_review_pending':False,'new_cli_sha256':build['new_cli_sha256'],'parent_cli_sha256':build['parent_cli_sha256'],'images':current['images'],'distinct_functions':current['distinct_functions'],'language_records':current['language_records'],'changed_records':len(changes),'changes':changes,'all_C_Go_candidate_records_equal_predependency_review':True,'all_Rust_dependency_changes_exact_empty_arm_removals':True,'dependency_edits':dependency_edits,'inherited_go_parser_failures':inherited_parser,'native_values':9864+9864+6900,'static_tables':6,'static_rows':246,'full_angr_parity_claimed':False,'supplied_upstream_executable_executed':False,'limitations':['The supplied upstream executable was inspected statically, never run. Whole upstream Rust/Go functions can retain unrelated type, string and residual-goto issues.','The fmt return width changes from64 to32 after recovery; archived Ghidra listing shows all exits pass through MOV EAX,EBX. The thunk target is independently bound by its pointer slot.','The decode_error_kind argument becomes signed32 after removing the misleading table guard; the same32 bits are passed to the existing errno decoder.','Variable renumbering and cleanup-branch ordering follow the recovered CFG. Authored controls exercise all high-byte widths admitted by the proof and representative signed/wrapping range behavior.']};(w/'breadth-review.json').write_text(json.dumps(review,indent=2)+'\n');print('HIGH-SLICE REVIEW COMPLETE',len(changes),'records;',review['native_values'],'native values;6static tables/246rows;Rust empty arms repaired by dependency.',flush=True)
