from common import *
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
base=Path('/home/mahaloz/kwt/angr-go-pair-results');f=r/'decompiler/crates/kuna-analysis/tests/fixtures';meta=json.loads((w/'fixture.json').read_text());names=','.join(row['name']for row in meta['cases']);assert len(names.split(','))==34;build=json.loads((w/'build.json').read_text());records=[]
for arm,root in [('parent',base),('candidate',r)]:
 assert sha(root/'decompiler/target/release/kuna')==build['parent_cli_sha256'if arm=='parent'else'new_cli_sha256']
 for lang in ['c','go','rust']:
  file=w/(arm+'-'+lang+'.json');cmd=[str(root/'decompiler/target/release/kuna'),'decompile-all',str(f/'go_store_value_x86_64'),'--json','--functions',names,'--language',lang,'--assert','@'+str(f/'go_store_value.kuna'),'--option','ctypes','off','--option','realtypes','off']
  with file.open('w')as out,(w/(arm+'-'+lang+'.log')).open('w')as err:subprocess.run(cmd,cwd=r,env=env|{'KUNA_SPECS':str(root/'specs')},stdout=out,stderr=err,check=True)
  d=json.loads(file.read_text());assert len(d['functions'])==34 and all(not fn.get('error')for fn in d['functions']);assert all(x['status']=='applied'for x in d['assertions'])
  prior=json.loads((w.parent/(arm+'-'+lang+'.json')).read_text());assert d['functions']==prior['functions'],(arm,lang,'capture differs from pre-integration proof; review before proceeding')
  records.append({'arm':arm,'language':lang,'file':str(file),'sha256':sha(file),'command':cmd,'functions':34,'matches_pre_integration_records':True})
(w/'probe-capture.json').write_text(json.dumps({'parent_cli_sha256':build['parent_cli_sha256'],'candidate_cli_sha256':build['new_cli_sha256'],'signedness_guard_output_unchanged':True,'records':records},indent=2)+'\n');print('Fresh STORE capture preserves all prior complete function records',flush=True)
