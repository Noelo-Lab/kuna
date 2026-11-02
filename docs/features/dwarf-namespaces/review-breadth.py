from pathlib import Path
import copy,hashlib,json,re
w=Path('/tmp/kuna-dwarf-namespace-p9');out=w/'breadth';sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest();complete=(out/'record.json').exists();rows=json.loads((out/'record.json').read_text())['records']if complete else [json.loads(p.read_text())for p in sorted(out.glob('*-record.json'))];reviews=[];inventory=[];counts={'changed_records':0,'qualified_comments':0,'removed_aliases':0,'unchanged_records':0,'changed_code_records':0};failures=[]
for row in rows:
 classified=[]
 for item in row['records']:
  oldfile=Path(item['parent_file']);newfile=Path(item['candidate_file']);assert sha(oldfile)==item['parent_sha256']and sha(newfile)==item['candidate_sha256'];old={f['address']:f for f in json.loads(oldfile.read_text())['functions']};new={f['address']:f for f in json.loads(newfile.read_text())['functions']};assert old.keys()==new.keys();language=item['language'];substitutions={}
  for a in old:
   x,y=old[a],new[a];before={x['name'],*x['aliases']};after={y['name'],*y['aliases']}
   if before==after and x['name']==y['name']:continue
   name=y['name'];parts=name.split('::');assert len(parts)>1 and all(re.fullmatch('[A-Za-z_][A-Za-z_0-9]*',p)for p in parts),(row['fixture'],name)
   leaf=parts[-1]
   for i in range(len(parts)-1):
    qualified=('::'if language!='go'else '.').join(parts[i:]);substitutions[qualified]=leaf
   for removed in before-after:
    assert '::'not in removed,(row['fixture'],removed)
    spelling=removed
    if language=='rust':spelling=re.sub('[^A-Za-z_0-9]','_',removed)
    elif language=='go'and not re.fullmatch('[A-Za-z_][A-Za-z_0-9]*',removed):spelling='_kuna_ident_'+removed.encode().hex()
    substitutions[spelling]=leaf
  pattern=re.compile(r'(?<![A-Za-z_0-9])(?:'+'|'.join(re.escape(x)for x in sorted(substitutions,key=len,reverse=True))+r')(?![A-Za-z_0-9])')if substitutions else None
  def normalize(code):return pattern.sub(lambda m:substitutions[m[0]],code)if pattern else code
  for a in old:
   x,y=old[a],new[a]
   if x==y:counts['unchanged_records']+=1;continue
   change=next(c for c in row['changes']if int(c['address'],16)==a and c['language']==language);counts['changed_records']+=1
   before={x['name'],*x['aliases']};after={y['name'],*y['aliases']};assert after<=before,(row['fixture'],hex(a),'new unproven identity',after-before)
   removed=sorted(before-after);counts['removed_aliases']+=len(removed);shift=0;xc=x['code'];yc=y['code']
   if language in ['rust','go']and yc.startswith('// '+y['name']+'\n')and not xc.startswith('// '+y['name']+'\n'):
    yc=yc.split('\n',1)[1];shift=1;counts['qualified_comments']+=1
   yp=copy.deepcopy(y)
   for m in yp['line_mappings']:m['line_number']-=shift
   for v in yp['variables']:v['line_numbers']=[n-shift for n in v['line_numbers']]
   for key in x.keys()|yp.keys():
    if key not in ['name','aliases','code']:assert x.get(key)==yp.get(key),(row['fixture'],language,hex(a),key)
   structural=normalize(xc)==normalize(yc)
   if not structural:failures.append({'fixture':row['fixture'],'language':language,'address':hex(a),'normalized_before':normalize(xc),'normalized_after':normalize(yc)})
   if xc!=yc:counts['changed_code_records']+=1
   classification=[]
   if x['name']!=y['name']:classification.append('existing same-address qualified alias becomes the primary name')
   if removed:classification.append('redundant generic global alias removed')
   if shift:classification.append('leading qualified-name comment; variable and instruction lines shift by one')
   if xc!=yc:classification.append('function definition, direct call or C whole-function value uses the existing qualified target identity')
   entry=change|{'fixture':row['fixture'],'classification':classification,'normalized_code_identical':structural,'all_nonname_metadata_unchanged_except_comment_line_shift':True,'line_shift':shift,'added_unproven_names':[]};classified.append(entry);inventory.append(entry)
  if item['go_parse']and item['go_parse']['candidate']['failed_addresses']:
   failures_old=item['go_parse']['parent']['failed_addresses'];assert set(item['go_parse']['candidate']['failed_addresses'])<=set(failures_old)
 reviews.append({'fixture':row['fixture'],'changes':classified})
record={'complete':complete,'semantic_review_pending':True,'images':len(rows),'distinct_functions':sum(row['records'][0]['functions']for row in rows),'language_records':sum(item['functions']for row in rows for item in row['records']),'counts':counts,'structural_review_failures':failures,'changes':inventory,'go_parse_failures':[{'fixture':row['fixture'],'parse':item['go_parse']}for row in rows for item in row['records']if item['go_parse']and item['go_parse']['candidate']['failed_addresses']]}
name='breadth-review.json'if complete else 'breadth-review-partial.json';(w/name).write_text(json.dumps(record,indent=2)+'\n')
(w/('hunks.txt'if complete else'hunks-partial.txt')).write_text('\n\n'.join(f"{x['fixture']} {x['language']} {x['address']} {x['classification']}\n"+'\n'.join(x['diff'])for x in inventory))
print({k:v for k,v in record.items()if k not in ['changes','structural_review_failures']});assert not failures,[(x['fixture'],x['language'],x['address'])for x in failures]
