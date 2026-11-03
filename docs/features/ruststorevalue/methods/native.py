from common import *
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
parent=Path('/home/mahaloz/kwt/angr-rust-integer-conversions');fixture=r/'decompiler/crates/kuna-analysis/tests/fixtures/go_store_value_x86_64';assertions=fixture.parent/'go_store_value.kuna'
names=json.loads((w/'cases.json').read_text());selected=names+['array_store'];records=[];captures={}
assert sha(parent/'decompiler/target/release/kuna')=='f846b2b4f01492a8c26ae22eac3b8563368d756996290ee42cfa76ef29c2f352'
for arm,root in [('parent',parent),('candidate',r)]:
 for language in ['c','go','rust']:
  cmd=[str(root/'decompiler/target/release/kuna'),'decompile-all',str(fixture),'--language',language,'--json','--functions',','.join(selected),'--assert','@'+str(assertions),'--option','ctypes','off','--option','realtypes','off']
  p=subprocess.run(cmd,cwd=root,env=env|{'KUNA_SPECS':str(root/'specs')},capture_output=True);(w/(arm+'-'+language+'.json')).write_bytes(p.stdout);(w/(arm+'-'+language+'.log')).write_bytes(p.stderr);assert p.returncode==0
  d=json.loads(p.stdout);assert len(d['functions'])==34 and all(not f.get('error')for f in d['functions']);captures[arm,language]=d
  records.append({'arm':arm,'language':language,'command':cmd,'capture_sha256':hashlib.sha256(p.stdout).hexdigest()})
for language in ['c','go']:assert captures['parent',language]==captures['candidate',language]
functions={f['name']:f for f in captures['candidate','rust']['functions']};old={f['name']:f for f in captures['parent','rust']['functions']}
for name in ['nested_store','float_store','pointer_store','bool_store']:assert functions[name]==old[name]
code=(w/'harness.rs').read_text().replace('/* EMITTED_BODIES */','\n'.join(functions[name]['code']for name in names));(w/'candidate.rs').write_text(code)
for opt in ['0','2']:
 cmd=['rustc','--edition=2021','-C','opt-level='+opt,'-C','overflow-checks=off',str(w/'candidate.rs'),'-o',str(w/('candidate-'+opt))]
 p=subprocess.run(cmd,capture_output=True);(w/('candidate-'+opt+'-build.log')).write_bytes(p.stdout+p.stderr);assert p.returncode==0,(opt,p.stderr.decode()[:3000])
 result=subprocess.run([str(w/('candidate-'+opt))],capture_output=True);(w/('candidate-'+opt+'.stdout')).write_bytes(result.stdout);(w/('candidate-'+opt+'.stderr')).write_bytes(result.stderr)
 assert result.returncode==0 and result.stdout==(w/('reference-'+opt+'.stdout')).read_bytes(),('native mismatch',opt)
 assert len(result.stdout.splitlines())==324*33
 records.append({'kind':'native','optimization':opt,'command':cmd,'function_comparisons':324*33,'output_sha256':hashlib.sha256(result.stdout).hexdigest()})
exclusions=[]
for arm,source in [('parent',old),('candidate',functions)]:
 path=w/(arm+'-array.rs');path.write_text(source['array_store']['code']+'\nfn main(){}\n');cmd=['rustc','--edition=2021','--error-format=json',str(path),'-o',str(w/(arm+'-array'))];p=subprocess.run(cmd,capture_output=True);(w/(arm+'-array-build.jsonl')).write_bytes(p.stderr)
 errors=[json.loads(line)for line in p.stderr.splitlines()if line.startswith(b'{')];errors=[e for e in errors if e.get('level')=='error'and e.get('code')];assert p.returncode!=0 and any(e['code']['code']=='E0608'for e in errors)
 exclusions.append({'arm':arm,'name':'array_store','command':cmd,'compile_exit':p.returncode,'errors':[{'code':e['code']['code'],'message':e['message']}for e in errors]})
summary={'status':'PASS','parent_cli_sha256':sha(parent/'decompiler/target/release/kuna'),'candidate_cli_sha256':sha(r/'decompiler/target/release/kuna'),'parent_commit':subprocess.check_output(['git','rev-parse','HEAD'],cwd=parent,text=True).strip(),'fixture_sha256':sha(fixture),'complete_bodies':33,'input_pairs_per_run':324,'native_function_comparisons':21384,'body_rewrites':False,'c_go_unchanged':True,'unchanged_rust_controls':['nested_store','float_store','pointer_store','bool_store'],'changed_rust_functions':[name for name in selected if functions[name]!=old[name]],'guard_bytes':True,'pointer_and_value_callback_counts':True,'records':records,'exclusions':exclusions,'source_sha256':{name:sha(w/name)for name in ['harness.c','harness.rs','native.py','candidate.rs','parent.rs','cases.json']}}
(w/'native-record.json').write_text(json.dumps(summary,indent=2)+'\n');print('Rust STORE native33 bodies/21384 comparisons PASS',flush=True)
