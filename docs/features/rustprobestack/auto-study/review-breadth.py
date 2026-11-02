from pathlib import Path
import json,hashlib,difflib,re,datetime
w=Path('/tmp/kuna-rust-probestack');out=w/'breadth';summary=json.loads((w/('breadth-final.json'if(w/'breadth-final.json').exists()else'breadth-mutable/record.json')).read_text());changes=[];text=[]
for row in summary['records']:
 for record in row['records']:
  a=json.loads(Path(record['parent_file']).read_text())['functions'];b=json.loads(Path(record['candidate_file']).read_text())['functions'];old={f['address']:f for f in a};new={f['address']:f for f in b};assert old.keys()==new.keys()
  for address in old:
   x,y=old[address],new[address]
   if x==y:continue
   fields=sorted(k for k in set(x)|set(y)if x.get(k)!=y.get(k));assert set(fields)<={'code','line_mappings','variables'},(row['fixture'],record['language'],hex(address),fields)
   changes.append({'fixture':row['fixture'],'language':record['language'],'address':hex(address),'name':x['name'],'fields':fields,'before_lines':len(x.get('code','').splitlines()),'after_lines':len(y.get('code','').splitlines()),'before_variables':x.get('variables'),'after_variables':y.get('variables'),'before_line_mappings':x.get('line_mappings'),'after_line_mappings':y.get('line_mappings'),'classification':'pending'})
   text += [f"===== {row['fixture']} {record['language']} {hex(address)} {x['name']} =====",*difflib.unified_diff(x.get('code','').splitlines(),y.get('code','').splitlines(),fromfile='parent',tofile='candidate',lineterm=''),'']
(out/'all-changes.txt').write_text('\n'.join(text))
(out/'metadata-review-pending.json').write_text(json.dumps(changes,indent=2)+'\n')
print('Full diff',len(changes),'changed records',len(text),'diff lines; personal review still required')
