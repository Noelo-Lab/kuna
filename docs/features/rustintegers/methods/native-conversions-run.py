from common import *
import shutil
sha=lambda path:hashlib.sha256(Path(path).read_bytes()).hexdigest()
assert any((Path('/tmp/kuna-go-pair-results-implementation/integration-a324')/name).exists() for name in ['timing-result.json','timing-cancelled.json']), 'Do not compete with the isolated Go timing study.'
source=w.parent/'planned-fixtures';out=w/'native-conversions';out.mkdir(exist_ok=False)
base=Path('/home/mahaloz/kwt/angr-go-store-conversions')
candidate_sha=sha(r/'decompiler/target/release/kuna')
assert candidate_sha=='f846b2b4f01492a8c26ae22eac3b8563368d756996290ee42cfa76ef29c2f352'
assert sha(base/'decompiler/target/release/kuna')=='b4557e50aa5b06baa5252307700b652dbecd651137ea873aa504ea364d8b82fd'
cases=json.loads((source/'cases.json').read_text())['cases'];names=[c['name']for c in cases]
assert len(names)==49 and len(set(names))==49
for p in source.iterdir():
 if p.is_file():shutil.copy2(p,out/p.name)
fixture=out/'fixture'
run(r,out,['cc','-O2','-g','-fwrapv','-fno-pie','-no-pie','-fno-inline','-fno-optimize-sibling-calls',str(out/'rust_integer_conversions.c'),str(out/'rust_integer_conversions_helpers.c'),'-o',str(fixture)],'fixture-build.log')
assertions=(out/'rust_integer_conversions.kuna').read_text().splitlines()
captures={};commands=[]
for arm,root in [('parent',base),('candidate',r)]:
 for language in ['c','go','rust']:
  cmd=[str(root/'decompiler/target/release/kuna'),'decompile-all',str(fixture),'--functions',','.join(names),'--language',language,'--json']
  for assertion in assertions:
   if assertion.strip() and not assertion.startswith('#'):cmd+=['--assert',assertion]
  commands.append(cmd)
  p=subprocess.run(cmd,cwd=root,env=env|{'KUNA_SPECS':str(root/'specs')},capture_output=True,text=True)
  (out/(arm+'-'+language+'.json')).write_text(p.stdout);(out/(arm+'-'+language+'.log')).write_text(p.stderr)
  assert p.returncode==0,(arm,language,p.stderr)
  capture=json.loads(p.stdout);assert len(capture['functions'])==len(names)
  assert all(not f.get('error')for f in capture['functions'])
  assert all(a['status']=='applied'for a in capture['assertions'])
  assert {f['name']for f in capture['functions']}==set(names)
  captures[(arm,language)]=capture
  print('CAPTURE',arm,language,flush=True)
for language in ['c','go']:assert captures[('parent',language)]['functions']==captures[('candidate',language)]['functions']
parent_by_name={f['name']:f for f in captures[('parent','rust')]['functions']}
changed=[f['name']for f in captures[('candidate','rust')]['functions']if f!=parent_by_name[f['name']]]
for name in ['float_identity','pointer_identity','bool_identity']:assert name not in changed
c_source=(out/'rust_integer_conversions.c').read_text().split('int main(void)')[0]+(out/'rust_integer_conversions_harness.c').read_text()
(out/'native-reference.c').write_text(c_source)
expected=None;records=[]
for opt in ['0','2']:
 exe=out/('reference-O'+opt);cmd=['cc','-O'+opt,'-fwrapv','-fno-inline','-std=c11','-Wall','-Wextra','-Werror',str(out/'native-reference.c'),str(out/'rust_integer_conversions_helpers.c'),'-o',str(exe)];commands.append(cmd)
 run(r,out,cmd,'reference-O'+opt+'-build.log')
 actual=subprocess.check_output([str(exe)]);(out/('reference-O'+opt+'.stdout')).write_bytes(actual)
 assert len(actual.splitlines())==49*324
 if expected is None:expected=actual
 else:assert expected==actual
 records.append({'arm':'reference','optimization':opt,'binary_sha256':sha(exe),'stdout_sha256':hashlib.sha256(actual).hexdigest()})
failures=[]
for arm in ['parent','candidate']:
 bodies='\n'.join(f['code']for f in captures[(arm,'rust')]['functions'])
 file=out/(arm+'.rs');template=(out/'rust_integer_conversions_harness.rs').read_text();assert template.count('/* EMITTED_BODIES */')==1
 file.write_text(template.replace('/* EMITTED_BODIES */',bodies))
 for opt in ['0','2']:
  exe=out/(arm+'-O'+opt);cmd=['rustc','--edition=2021','-C','opt-level='+opt,'-C','overflow-checks=off',str(file),'-o',str(exe)];commands.append(cmd)
  p=subprocess.run(cmd,capture_output=True,text=True);(out/(arm+'-O'+opt+'-build.log')).write_text(p.stdout+p.stderr)
  record={'arm':arm,'optimization':opt,'overflow_checks':'off','source_sha256':sha(file),'compile_exit':p.returncode}
  if p.returncode!=0:
   if arm=='candidate':failures.append({'profile':opt,'reason':'compile failure','log':arm+'-O'+opt+'-build.log'})
  else:
   actual=subprocess.check_output([str(exe)]);(out/(arm+'-O'+opt+'.stdout')).write_bytes(actual);record.update(binary_sha256=sha(exe),stdout_sha256=hashlib.sha256(actual).hexdigest(),matches_reference=actual==expected)
   if arm=='candidate'and actual!=expected:failures.append({'profile':opt,'reason':'native mismatch'})
  records.append(record)
assert sha(r/'decompiler/target/release/kuna')==candidate_sha
record={'status':'PASS'if not failures else 'FAIL; retained complete output for diagnosis','candidate_cli_sha256':candidate_sha,'parent_cli_sha256':sha(base/'decompiler/target/release/kuna'),'fixture_sha256':sha(fixture),'planned_functions':49,'input_pairs_per_run':324,'native_rust_input_comparisons':648 if not failures else None,'function_comparisons':31752 if not failures else None,'body_rewrites':False,'c_go_output_unchanged':True,'changed_rust_functions':changed,'failures':failures,'records':records,'commands':commands,'source_hashes':{p.name:sha(p)for p in source.iterdir()if p.is_file()}}
(out/'record.json').write_text(json.dumps(record,indent=2)+'\n')
print(record['status'],len(changed),'changed functions',flush=True)
assert not failures,failures
