from common import *
f=w/'native';parent=Path('/home/mahaloz/kwt/angr-base-0b7117');names=json.loads((f/'cases.json').read_text());sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest();captures={};records=[]
for arm,root in [('parent',parent),('candidate',r)]:
 for language in ['c','go','rust']:
  cmd=[str(root/'decompiler/target/release/kuna'),'decompile-all',str(f/'pointer_index_x86_64'),'--language',language,'--json','--functions',','.join(names),'--assert','@'+str(f/'pointer_index.kuna'),'--option','ctypes','off','--option','realtypes','off'];p=subprocess.run(cmd,env=env|{'KUNA_SPECS':str(root/'specs')},capture_output=True);(f/(arm+'-'+language+'.json')).write_bytes(p.stdout);(f/(arm+'-'+language+'.log')).write_bytes(p.stderr);assert p.returncode==0
  d=json.loads(p.stdout);assert len(d['functions'])==len(names)and all(not x.get('error')for x in d['functions']);assert all(x['status']=='applied'for x in d['assertions']);captures[arm,language]=d;records.append({'arm':arm,'language':language,'command':cmd,'capture_sha256':sha(f/(arm+'-'+language+'.json'))})
for language in ['c','go']:assert captures['parent',language]==captures['candidate',language]
for arm in ['parent','candidate']:
 bodies={x['name']:x['code']for x in captures[arm,'rust']['functions']};source=(f/'harness.rs').read_text().replace('/* EMITTED_BODIES */','\n'.join(bodies[n]for n in names));(f/(arm+'.rs')).write_text(source)
 for opt in ['0','2']:
  cmd=['rustc','--edition=2021','--error-format=json','-C','opt-level='+opt,'-C','overflow-checks=off',str(f/(arm+'.rs')),'-o',str(f/(arm+'-'+opt))];p=subprocess.run(cmd,capture_output=True);(f/(arm+'-'+opt+'-build.jsonl')).write_bytes(p.stdout+p.stderr)
  errors=[json.loads(line)for line in p.stderr.splitlines()if line.startswith(b'{')];errors=[e for e in errors if e.get('level')=='error'and e.get('code')];record={'arm':arm,'optimization':opt,'command':cmd,'compile_exit':p.returncode,'errors':[{'code':e['code']['code'],'message':e['message']}for e in errors]};records.append(record)
  if arm=='parent':assert p.returncode!=0 and any(e['code']['code']=='E0608'for e in errors);continue
  assert p.returncode==0,record
  p=subprocess.run([str(f/(arm+'-'+opt))],capture_output=True);(f/(arm+'-'+opt+'.stdout')).write_bytes(p.stdout);(f/(arm+'-'+opt+'.stderr')).write_bytes(p.stderr);assert p.returncode==0
  expected=(f/('reference-'+opt+'.stdout')).read_bytes()
  if p.stdout!=expected:
   actual=p.stdout.splitlines();expected=expected.splitlines();bad=[{'line':i,'actual':a.decode(),'expected':b.decode()}for i,(a,b)in enumerate(zip(actual,expected))if a!=b];(f/(arm+'-'+opt+'-mismatches.json')).write_text(json.dumps(bad,indent=2)+'\n');raise AssertionError(('native mismatches',len(bad),bad[:5]))
  assert len(p.stdout.splitlines())==54*18*15+3*64;record.update(output_sha256=hashlib.sha256(p.stdout).hexdigest(),function_comparisons=54*18*15+3*64)
summary={'status':'PASS','parent_cli_sha256':sha(parent/'decompiler/target/release/kuna'),'candidate_cli_sha256':sha(r/'decompiler/target/release/kuna'),'fixture_sha256':sha(f/'pointer_index_x86_64'),'complete_bodies':len(names),'native_function_comparisons':(54*18*15+3*64)*2,'body_rewrites':False,'c_go_unchanged':True,'scope':'1/2/4/8-byte signed/unsigned loads/stores, signed negative and unsigned valid indices, uintptr address wrapping, record fields, callback trace order, full array bytes and guards; x86_64 only.','records':records,'source_sha256':{p.name:sha(p)for p in f.iterdir()if p.suffix in ['.rs','.c','.kuna']}}
(w/'native-record.json').write_text(json.dumps(summary,indent=2)+'\n');print('Native',len(names),'bodies',summary['native_function_comparisons'],'comparisons PASS',flush=True)
