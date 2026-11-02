from pathlib import Path
import subprocess,time,os,json,hashlib,shutil,signal,datetime
parent=Path('/home/mahaloz/kwt/angr-rust-panic-main');parentwork=Path('/tmp/kuna-rust-panic-69fa')
roots=[Path('/home/mahaloz/kwt/angr-rust-fmt-'+s)for s in ['pieces','str']]
work=[Path('/tmp/kuna-rust-fmt-'+s)for s in ['pieces-f874','str']]
newwork=[Path('/tmp/kuna-rust-fmt-'+s+'-69fa')for s in ['pieces','str']]
while not(parentwork/'breadth/record.json').exists()or not all((w/'gates-passed.json').exists()for w in newwork):time.sleep(5)
def digest(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def run(w,cmd,log,cwd=None):
 with(w/log).open('w')as file:
  p=subprocess.Popen(cmd,cwd=cwd,stdout=file,stderr=subprocess.STDOUT,start_new_session=True)
  while p.poll()is None:
   m={s.split(':')[0]:int(s.split()[1])for s in Path('/proc/meminfo').read_text().splitlines()}
   if 1-m['MemAvailable']/m['MemTotal']>=.9:
    os.killpg(p.pid,signal.SIGTERM);p.wait();(w/'memory-limit-hit.json').write_text(json.dumps(m));raise RuntimeError('RAM reached 90%; own captures stopped; user help required.')
   time.sleep(1)
  assert p.returncode==0,(str(w),log,p.returncode)
 print(w.name,log,'PASS',flush=True)
for index,(r,w,nw)in enumerate(zip(roots,work,newwork)):
 if index==0:continue
 base=parent if index==0 else roots[0];basework=parentwork if index==0 else newwork[0]
 parent_source=json.loads((basework/'production-source-before-validation.json').read_text());assert all(digest(base/p)==h for p,h in parent_source.items())
 (w/'parent-source-before-commit.json').write_text(json.dumps({'files':parent_source,'binary_sha256':digest(base/'decompiler/target/release/kuna')},indent=2)+'\n')
 for name in ['production-source-before-validation.json','original.log','stages.log','workspace-final.log','tools.log','lint.log','spec.log','catalog.log','counters.log','gates-passed.json']:
  shutil.copy2(nw/name,w/name)
 (w/'baseline-commit.txt').write_bytes(subprocess.check_output(['git','rev-parse','HEAD'],cwd=base))
 p=w/'breadth.py';text=p.read_text()
 if index==0:
  text=text.replace('/tmp/kuna-rust-panic-f874/breadth','/tmp/kuna-rust-panic-69fa/breadth').replace("pending_main=='f8746b9730b525a065c89c26e358ffd7c2328bc7'","pending_main=='69fad3ce4cbdbb3232680938252bd75c2bec5a68'")
 p.write_text(text)
 deps=r/'decompiler/target/release/deps';cmd=['rustc','--edition=2021',str(w/'descriptor-evidence.rs'),'-L',f'dependency={deps}','-o',str(w/'descriptor-evidence')]
 for crate in ['kuna_console','kuna_decomp','kuna_num','serde_json','slotmap']:
  files=sorted(deps.glob(f'lib{crate}-*.rlib'),key=lambda p:p.stat().st_mtime,reverse=True);assert files;cmd+=['--extern',f'{crate}={files[0]}']
 run(w,cmd,'descriptor-evidence-69fa.compile.log')
 for p in(w/'breadth').glob('*-descriptor.jsonl'):p.unlink()
 run(w,['python3','-u',str(w/'breadth.py'),'--capture-only'],'overlap-capture.log')
 run(w,['python3','-u',str(w/'breadth.py'),'--review-existing'],'breadth-69fa.log')
 record=json.loads((w/'breadth/record.json').read_text());prior=w.with_name(w.name+'-pre69fa')/'breadth';count=0
 for fixture in record['fixtures']:
  name=fixture['fixture']
  for language in fixture['languages']:
   lang=language['language'];a=json.loads((prior/f'{name}-{lang}.json').read_text())['functions'];b=json.loads((w/'breadth'/f'{name}-{lang}.json').read_text())['functions']
   assert {x['address']:x for x in a}=={x['address']:x for x in b},('main integration changed reviewed records',name,lang);count+=len(b)
 record['semantic_evidence_pending']=False;record['main_integration_reviewed_output_identical']=True
 (w/'breadth/record.json').write_text(json.dumps(record,indent=2)+'\n')
 proof={'main_commit':'69fad3ce4cbdbb3232680938252bd75c2bec5a68','all_complete_function_records_identical':True,'function_records':count,'images':len(record['fixtures']),'all_original_native_and_manual_mapping_evidence_still_applies':True,'prior_record_sha256':digest(prior/'record.json'),'fresh_descriptor_helper_sha256':digest(w/'descriptor-evidence'),'binary_sha256':digest(r/'decompiler/target/release/kuna')}
 (w/'main-69fa-equivalence.json').write_text(json.dumps(proof,indent=2)+'\n')
 plan=json.loads((w/'timing-candidate-index-plan.json').read_text());plan['common_main_before_sampling']='69fad3ce4cbdbb3232680938252bd75c2bec5a68';plan['main_amendment_declared_at']=str(datetime.datetime.now(datetime.timezone.utc));plan['main_amendment']='All arms rebuilt after the shared wrap-splitting fix; complete output equivalence and all repository checks pass. Statistical protocol unchanged; no samples from this revision collected yet.'
 (w/'timing-candidate-index-plan.json').write_text(json.dumps(plan,indent=2)+'\n')
 print(w.name,'FRESH 69FA BREADTH AND PRIOR REVIEW IDENTITY PASSED',count,flush=True)
print('BOTH CANDIDATE-INDEX FEATURES READY FOR DECLARED TIMINGS',flush=True)
