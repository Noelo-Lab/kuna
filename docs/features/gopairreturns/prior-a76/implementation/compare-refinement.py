from common import *
import difflib
old=w.parent/'integration-350/breadth';out=w/'breadth';changes=[]
for path in sorted(out.glob('*-record.json')):
 row=json.loads(path.read_text());before=json.loads((old/path.name).read_text())
 for rec in row['records']:
  previous=next(x for x in before['records']if x['language']==rec['language'])
  for arm in ['off','on']:
   a={f['address']:f for f in json.loads(Path(previous['candidate_files'][arm]).read_text())['functions']};b={f['address']:f for f in json.loads(Path(rec['candidate_files'][arm]).read_text())['functions']};assert a.keys()==b.keys()
   for addr in a:
    if a[addr]!=b[addr]:changes.append({'fixture':row['fixture'],'language':rec['language'],'arm':arm,'address':hex(addr),'name':b[addr]['name'],'fields':[k for k in a[addr]if a[addr][k]!=b[addr][k]],'diff':list(difflib.unified_diff(a[addr]['code'].splitlines(),b[addr]['code'].splitlines(),fromfile='before-unlocked-adapter',tofile='after',lineterm=''))})
(w/'refinement-diff.json').write_text(json.dumps({'completed_images':len(list(out.glob('*-record.json'))),'changes':changes},indent=2)+'\n');print('Completed images',len(list(out.glob('*-record.json'))),'refined records',len(changes))
for c in changes:print(c['fixture'],c['language'],c['name'],'\n'+'\n'.join(c['diff']))
