from common import *
import shutil
assert Path('/tmp/kuna-go-pair-results-implementation/integration-460/timing-result.json').exists(), 'Wait for Go timing to close before running.'
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
out=w/'native-context';out.mkdir(exist_ok=False);source=w.parent/'planned-angle-subtraction';base=Path('/home/mahaloz/kwt/angr-go-store-conversions');digest=sha(r/'decompiler/target/release/kuna');assert digest=='f846b2b4f01492a8c26ae22eac3b8563368d756996290ee42cfa76ef29c2f352'
for p in source.iterdir():
 if p.is_file():shutil.copy2(p,out/p.name)
shutil.copy2(r/'decompiler/crates/kuna-analysis/tests/fixtures/rust_integer_conversions_helpers.c',out/'helpers.c')
commands=[]
def compile_c(opt,target,reference=False):
 cmd=['cc','-O'+opt,'-g','-fwrapv','-fno-inline','-fno-optimize-sibling-calls','-fno-pie','-no-pie','-Wall','-Wextra','-Werror']
 if reference:cmd+=['-DKUNA_NATIVE_MAIN']
 cmd += [str(out/'context.c'),str(out/'helpers.c'),'-o',str(target)];commands.append(cmd);run(r,out,cmd,target.name+'-build.log')
fixture=out/'fixture';compile_c('2',fixture)
assertions=(out/'context.kuna').read_text().splitlines();names=[line.split()[1]for line in assertions if not line.startswith('prototype counted_')];assert len(names)==15
captures={}
for arm,root in [('parent',base),('candidate',r)]:
 for lang in ['c','go','rust']:
  cmd=[str(root/'decompiler/target/release/kuna'),'decompile-all',str(fixture),'--functions',','.join(names),'--language',lang,'--json','--assert','@'+str(out/'context.kuna')];commands.append(cmd);p=subprocess.run(cmd,cwd=root,env=env|{'KUNA_SPECS':str(root/'specs')},capture_output=True,text=True);(out/(arm+'-'+lang+'.json')).write_text(p.stdout);(out/(arm+'-'+lang+'.log')).write_text(p.stderr);assert p.returncode==0;p=json.loads(p.stdout);assert all(a['status']=='applied'for a in p['assertions']);assert len(p['functions'])==15 and all(not f.get('error')for f in p['functions']);captures[(arm,lang)]=p
for lang in ['c','go']:assert captures[('parent',lang)]['functions']==captures[('candidate',lang)]['functions']
expected=None;records=[]
for opt in ['0','2']:
 exe=out/('reference-'+opt);compile_c(opt,exe,True);actual=subprocess.check_output([str(exe)]);(out/('reference-'+opt+'.stdout')).write_bytes(actual);assert len(actual.splitlines())==1280
 if expected is None:expected=actual
 else:assert actual==expected
 records.append({'arm':'reference','opt':opt,'stdout_sha256':hashlib.sha256(actual).hexdigest()})
failures=[]
for arm in ['parent','candidate']:
 file=out/(arm+'.rs');template=(out/'harness.rs').read_text();assert template.count('/* EMITTED_BODIES */')==1;file.write_text(template.replace('/* EMITTED_BODIES */','\n'.join(f['code']for f in captures[(arm,'rust')]['functions'])))
 for opt in ['0','2']:
  exe=out/(arm+'-'+opt);cmd=['rustc','--edition=2021','-C','opt-level='+opt,'-C','overflow-checks=off',str(file),'-o',str(exe)];commands.append(cmd);p=subprocess.run(cmd,capture_output=True,text=True);(out/(arm+'-'+opt+'-build.log')).write_text(p.stdout+p.stderr);record={'arm':arm,'opt':opt,'compile_exit':p.returncode}
  if p.returncode:
   if arm=='candidate':failures.append({'opt':opt,'reason':'compile'})
  else:
   p=subprocess.run([str(exe)],capture_output=True);(out/(arm+'-'+opt+'.stdout')).write_bytes(p.stdout);(out/(arm+'-'+opt+'.stderr')).write_bytes(p.stderr);record.update(execute_exit=p.returncode,matches_reference=p.stdout==expected,stdout_sha256=hashlib.sha256(p.stdout).hexdigest());different=[{'line':i,'reference':a.decode(),'candidate':b.decode()}for i,(a,b)in enumerate(zip(expected.splitlines(),p.stdout.splitlines()))if a!=b];record['first_difference']=different[:1]
   if arm=='candidate'and(p.returncode or p.stdout!=expected):failures.append({'opt':opt,'reason':'native output','first_difference':different[:1]})
  records.append(record)
record={'status':'PASS'if not failures else'FAIL; retain every probe for diagnosis','candidate_cli_sha256':digest,'parent_cli_sha256':sha(base/'decompiler/target/release/kuna'),'fixture_sha256':sha(fixture),'complete_functions':15,'input_vectors_per_run':1280,'native_rust_input_comparisons':2560 if not failures else None,'function_comparisons':38400 if not failures else None,'body_rewrites':False,'c_go_unchanged':True,'records':records,'failures':failures,'commands':commands};(out/'record.json').write_text(json.dumps(record,indent=2)+'\n');print(record['status'],json.dumps(failures),flush=True);assert not failures
