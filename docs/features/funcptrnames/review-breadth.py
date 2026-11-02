from pathlib import Path
import collections,hashlib,json,re

w=Path('/tmp/kuna-fnptr-namespace');out=w/'breadth';record=json.loads((out/'record.json').read_text());build=json.loads((w/'build.json').read_text())
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
assert record['new_cli_sha256']==build['new_cli_sha256'] and record['parent_cli_sha256']==build['parent_cli_sha256']
lex=re.compile(r'''"(?:\\.|[^"\\])*"|'(?:\\.|[^'\\])*'|//[^\n]*|[A-Za-z_][A-Za-z_0-9]*|.''')
identifier=re.compile(r'[A-Za-z_][A-Za-z_0-9]*\Z')
prefix=r'(?:(?:::)?(?:[A-Za-z_][A-Za-z_0-9]*::)+|::)'
keywords=set(re.findall(r'"([a-z][a-z_0-9]*)"',Path('/home/mahaloz/kwt/angr-fnptr-namespace/decompiler/crates/kuna-decomp/src/p9_emit/kuna_funcptrnames.rs').read_text().split('fn plain_scope')[1]))
changes=[];totals=collections.Counter();parse_failures=[]
for image in record['records']:
 assert sha(Path(image['fixture_path']))==image['fixture_sha256']
 for capture in image['records']:
  before=Path(capture['parent_file']);after=Path(capture['candidate_file']);assert sha(before)==capture['parent_sha256']and sha(after)==capture['candidate_sha256']
  old={f['address']:f for f in json.loads(before.read_text())['functions']};new={f['address']:f for f in json.loads(after.read_text())['functions']};assert old.keys()==new.keys()
  if capture['language']!='c':assert old==new;totals['non_C_exact_records']+=len(old)
  if capture['language']=='go'and not capture['go_parse']:parse_failures.append({'fixture':image['fixture'],'all_records_exact_parent':True,'log':str(out/f'{image["fixture"]}-go-parse.log')})
  for addr,a in old.items():
   b=new[addr]
   if a==b:continue
   assert {k:v for k,v in a.items()if k!='code'}=={k:v for k,v in b.items()if k!='code'},(image['fixture'],hex(addr),'metadata changed')
   lines_a=a['code'].splitlines();lines_b=b['code'].splitlines();assert len(lines_a)==len(lines_b)
   insertions=[]
   for number,(left,right)in enumerate(zip(lines_a,lines_b),1):
    if left==right:continue
    tokens=lex.findall(left);assert ''.join(tokens)==left
    pattern='';groups=[]
    for token in tokens:
     if identifier.fullmatch(token):
      label=f'p{len(groups)}';pattern+=f'(?P<{label}>{prefix})?'+re.escape(token);groups.append((label,token))
     else:pattern+=re.escape(token)
    match=re.fullmatch(pattern,right);assert match,(image['fixture'],hex(addr),number,left,right)
    additions=[{'prefix':match[label],'leaf':leaf}for label,leaf in groups if match[label]];assert additions
    assert all(not(set(item['prefix'].split('::'))&keywords)for item in additions)
    insertions.append({'line':number,'before':left,'after':right,'qualifications':additions})
   assert insertions
   changes.append({'fixture':image['fixture'],'language':'c','address':hex(addr),'name':a['name'],'classification':'Exact whole-function address gains a representable namespace prefix; leaf, expression operators, statements, instruction mappings, variable metadata, aliases and types remain unchanged.','changed_lines':insertions})
   totals['changed_functions']+=1;totals['changed_lines']+=len(insertions);totals['qualified_function_values']+=sum(len(line['qualifications'])for line in insertions)
assert totals['changed_functions']==record['changed_records']
review={'semantic_review_pending':False,'build':build,'images':record['images'],'distinct_functions':record['distinct_functions'],'language_records':record['language_records'],'totals':dict(totals),'all_changed_records_only_namespace_insertions':True,'all_non_code_fields_exact_parent':True,'all_none_records_exact_parent':True,'all_non_C_records_exact_parent':True,'inherited_go_parse_failures':parse_failures,'limits':'Broad review verifies every changed expression and its metadata; native execution covers the authored namespace fixture. It does not claim that every C rendering of Rust code is compilable.','changes':changes}
(w/'breadth-review.json').write_text(json.dumps(review,indent=2)+'\n');print({k:v for k,v in review.items()if k not in ['changes','build']})
