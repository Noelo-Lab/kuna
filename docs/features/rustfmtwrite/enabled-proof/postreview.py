from pathlib import Path
import json,re,subprocess,hashlib,collections
w=Path('/tmp/kuna-rust-fmt-write');b=w/'breadth';record=json.loads((b/'record.json').read_text())
prior_mappings=json.loads((w/'mapping-review.json').read_text());assert not prior_mappings['review_pending']
known={(('write-native' if f['fixture']=='native' else 'rust'),f['address']):f for f in prior_mappings['functions']}
sites=[];changed=[];source='fn main(){\n';mappings=[]
for fixture in record['fixtures']:
 name=fixture['fixture']
 for language in fixture['languages']:
  for item in language['changed']:
   assert language['language']=='rust'
   key=(name,item['address']);assert key in known,('New function needs manual review',key)
   prior=known[key];mapping=item['source_mapping']
   assert prior['lost_addresses']==mapping['lost_instruction_addresses'] and not mapping['added_instruction_addresses']
   mappings.append({'fixture':name,'address':item['address'],'prior_manual_review_preserved':True,**mapping,'disassembly':prior['disassembly']})
   functions={x['address']:x for x in json.loads((b/f'{name}-rust.json').read_text())['functions']}
   function=functions[int(item['address'],16)];body=function['code']
   original_name='native' if name=='write-native' else 'reference-write'
   previous={x['address']:x for x in json.loads((w/f'{original_name}-after.json').read_text())['functions']}
   if function!=previous[int(item['address'],16)]:
    assert key==('rust','0x11230'),('New function needs context review',key)
    context=json.loads((w/'reference-context-review.json').read_text())
    assert context['context_difference_preexists_feature']and context['all_other_complete_function_fields_identical']
    assert context['source_instruction_mappings_identical_across_contexts']and context['total_comparisons']==7128
    assert function==context['functions']['full_after']and previous[int(item['address'],16)]==context['functions']['single_after']

   changed.append(key)
   assert len(item['proof']['changes'])==1
   for site in item['proof']['changes']:
    assert site['all_template_bytes_and_argument_bindings_verified'] and site['result_bridge_type_verified']
    after=site['after'];assert after['result_type']=='u8'
    lines=[line.strip()for line in body.splitlines()if 'use std::fmt::Write as _;'in line];assert len(lines)==1
    line=lines[0];suffix=" == b'\\0' {";assert line.startswith('if ')and line.endswith(suffix)
    expression=line[3:-len(suffix)];writer=after['writer'];assert re.fullmatch(r'a\d+',writer)
    writer_type=re.search(r'\b'+writer+r': (u64|\*mut \(\))',body)[1]
    source+='{let text="é🦀\\0{}\\n";let mut out=String::from("prefix:é\\0");'
    source+=f'let {writer}=&mut out as *mut String as {writer_type};'
    bindings={}
    def bind(name,value):
     assert re.fullmatch(r'[A-Za-z_][A-Za-z_0-9]*',name),name
     if name in bindings:assert bindings[name]==value
     bindings[name]=value
    for arg in after['parameters']:
     if arg['kind']=='string':
      bind(arg['data'],f'let {arg["data"]}=text.as_ptr() as u64;');bind(arg['length'],f'let {arg["length"]}=text.len() as u64;')
     else:
      pointer=arg['pointer'];ty=arg['primitive'];value=ty+'::MIN+1'if ty[0]=='i'else ty+'::MAX'
      bind(pointer,f'let {pointer}_value:{ty}={value};let {pointer}=&{pointer}_value as *const {ty};')
    source+=''.join(bindings.values())+'let result=unsafe{'+expression+'};assert_eq!(result,0);println!("{:?}",out);}\n'
    sites.append({'fixture':name,'address':item['address'],'expression':expression,'bindings':bindings,'unchanged_emitted_expression':True})
source+='}\n';assert len(changed)==len(known)==8 and len(sites)==8
(w/'all-macros-execution.rs').write_text(source);outputs=[]
for version in ['1.90.0','1.93.1','1.99.1']:
 exe=w/f'all-macros-{version}'
 p=subprocess.run(['rustc','+'+version,'--edition=2021','--crate-name','all_writes',str(w/'all-macros-execution.rs'),'-o',str(exe)],capture_output=True,text=True)
 (w/f'all-macros-{version}.compile.log').write_text(p.stderr);assert p.returncode==0,p.stderr
 p=subprocess.run([str(exe)],capture_output=True,check=True);assert not p.stderr;outputs.append(p.stdout)
 (w/f'all-macros-{version}.stdout').write_bytes(p.stdout)
assert outputs[0]==outputs[1]==outputs[2]
audit=json.loads((w/'type-audit.json').read_text())
for item in audit['records']:
 before=collections.Counter((d['code'],d['message'])for d in item['before']['diagnostics']);after=collections.Counter((d['code'],d['message'])for d in item['after']['diagnostics'])
 assert not(after-before),(item['name'],after-before)
assert not audit['summary']['previously_compiling_now_failing']
summary={'images':len(record['fixtures']),'distinct_functions':sum(max(l['functions']for l in f['languages'])for f in record['fixtures']),'function_records':sum(l['functions']for f in record['fixtures']for l in f['languages']),'changed_functions':len(changed),'hunks':sum(len(c['hunks'])for f in record['fixtures']for l in f['languages']for c in l['changed']),'C_Go_identical':sum(l['functions']for f in record['fixtures']for l in f['languages']if l['language']in ['c','go']),'Go_parses':sum(l['parsed']for f in record['fixtures']for l in f['languages']),'off_matches_parent_rust_records':sum(l['functions']for f in record['fixtures']for l in f['languages']if l['language']=='rust')}
(w/'breadth-summary.json').write_text(json.dumps(summary,indent=2)+'\n')
(w/'all-macros.json').write_text(json.dumps({'changed_functions':len(changed),'recovered_calls':len(sites),'sites':sites,'execution_compilers':['1.90.0','1.93.1','1.99.1'],'stdout_sha256':hashlib.sha256(outputs[0]).hexdigest(),'all_template_bytes_and_argument_bindings_verified':True,'all_writer_vtables_and_result_bytes_verified':True,'every_macro_expression_unchanged':True,'execution_uses_explicit_input_bindings':True,'source_mapping_classification_pending':False,'type_diagnostics':audit['summary'],'complete_function_recompilation_claimed':False},indent=2)+'\n')
(w/'mapping-review-breadth.json').write_text(json.dumps({'review_pending':False,'functions':mappings,'native_complete_function_records_equal_prior_review':True,'reference_context_difference_preexists_feature_and_is_independently_verified':True},indent=2)+'\n')
record['semantic_evidence_pending']=False;(b/'record.json').write_text(json.dumps(record,indent=2)+'\n')
print('ALL 8 CHANGED MACROS AND SOURCE MAPPINGS REVIEWED',json.dumps(summary),flush=True)
