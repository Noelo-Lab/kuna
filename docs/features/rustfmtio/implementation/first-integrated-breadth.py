from pathlib import Path
from concurrent.futures import ThreadPoolExecutor,as_completed
import ast,hashlib,json,os,subprocess,time,signal,difflib,re
import capstone
w=Path('/tmp/kuna-rust-io-proposal');root=Path('/home/mahaloz/kwt/angr-rust-io-proposal');base=Path('/home/mahaloz/kwt/angr-rust-fmt-write-main');out=w/'breadth-final';out.mkdir(exist_ok=True)
refs=Path('/tmp/kuna-angr-reference/binaries');old=Path('/tmp/kuna-rust-fmt-write-optin')
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
new=root/'decompiler/target/release/kuna';parent=base/'decompiler/target/release/kuna'
build=json.loads((w/'current-main-build.json').read_text());assert sha(new)==build['cli_sha256']
parent_head=subprocess.check_output(['git','rev-parse','HEAD'],cwd=base,text=True).strip();assert parent_head=='b7745023f25688558babc621414b0d8f30332616'
assert not subprocess.check_output(['git','status','--porcelain'],cwd=base)
parent_sha=sha(parent)
receipt=json.loads((old/'breadth/record.json').read_text());prior={row['fixture']:row for row in receipt['fixtures']}
source_tree=ast.parse(Path('/tmp/kuna-rust-fmt-write/breadth.py').read_text());namespace={'root':root,'refs':refs,'Path':Path}
for node in source_tree.body:
 target=node.targets[0]if isinstance(node,ast.Assign)and len(node.targets)==1 else node.target if isinstance(node,ast.AugAssign)else None
 if isinstance(target,ast.Name)and target.id=='inputs':exec(compile(ast.Module(body=[node],type_ignores=[]),'<fixture-list>','exec'),namespace)
inputs=namespace['inputs'];assert len(inputs)==27
inputs += [('io-native',root/'decompiler/crates/kuna-analysis/tests/fixtures/rust_fmt_io_x86_64',['c','rust','go']),('io-promoted',root/'decompiler/crates/kuna-analysis/tests/fixtures/rust_fmt_io_promoted_x86_64',['c','rust','go'])]
exec((w/'evidence.py').read_text())
raw_descriptors=json.loads((w/'parent-descriptors-complete.json').read_text())['records']
initial_labels={'rust':'reference','io-native':'native','io-promoted':'promoted'}

def capture(executable,binary,language,path,options=[]):
 assert not path.exists(),path
 cmd=[str(executable),'decompile-all',str(binary),'--language',language,'--json','--jobs','1','--max-fn-seconds','600','--option','godwarfsig','on']+options
 with path.open('w')as output,path.with_suffix('.log').open('w')as log:
  p=subprocess.Popen(cmd,cwd=root,env=os.environ|{'KUNA_SPECS':str(root/'specs')},stdout=output,stderr=log,start_new_session=True)
  while p.poll()is None:
   memory={s.split(':')[0]:int(s.split()[1])for s in Path('/proc/meminfo').read_text().splitlines()}
   if 1-memory['MemAvailable']/memory['MemTotal']>=.9:
    os.killpg(p.pid,signal.SIGTERM);p.wait();(w/'memory-limit-hit.json').write_text(json.dumps(memory));raise RuntimeError('90% RAM; own work stopped; user help required.')
   time.sleep(1)
  assert p.returncode==0,(path,p.returncode)

def records(path):return {x['address']:x for x in json.loads(path.read_text())['functions']}

def run(row):
 name,binary,languages=row;digest=sha(binary);result={'fixture':name,'fixture_sha256':digest,'languages':[]};image=None;symbols=None
 if name in prior:assert digest==prior[name]['fixture_sha256']
 for language in languages:
  if name in prior:
   old_capture=next(x for x in prior[name]['captures']if x['language']==language and x['mode']=='default')
   before=Path(old_capture['fresh_file']);assert sha(before)==old_capture['fresh_sha256'];baseline_scope='Previously captured committed String-writer parent; emission/analysis/console source unchanged by the subsequent CLI-only integration.'
  elif language=='rust':
   before=w/f'{initial_labels[name]}-before.json';p=json.loads((w/'parent-captures.json').read_text());entry=next(x for x in p['captures']if x['fixture']==initial_labels[name]);assert sha(before)==entry['sha256'];baseline_scope='Independently captured committed String-writer parent before the CLI-only integration.'
  else:
   before=out/f'{name}-{language}-before.json';capture(parent,binary,language,before);baseline_scope='Fresh current-main String-writer parent.'
  path=out/f'{name}-{language}.json';capture(new,binary,language,path)
  a,b=records(before),records(path);assert a.keys()==b.keys()
  assert all(not x.get('error')for group in [a,b]for x in group.values())
  language_record={'language':language,'functions':len(b),'before_sha256':sha(before),'after_sha256':sha(path),'baseline_scope':baseline_scope,'changes':[]}
  if language!='rust':
   assert a==b,(name,language)
   if language=='go':
    parse=subprocess.run(['/tmp/kuna-go-output/parse',str(path)],capture_output=True,text=True);assert parse.returncode==0,parse.stdout+parse.stderr
    language_record['all_functions_parse']=True
  else:
   off=out/f'{name}-rust-off.json';capture(new,binary,language,off,['--option','rustfmtio','off']);assert records(off)==a,('off mismatch',name)
   language_record['off_sha256']=sha(off);language_record['off_equals_parent']=True
   proofs={}
   if a!=b:
    p=subprocess.run([str(w/'verifier/target/release/rust-io-verifier'),str(before),str(path)],capture_output=True)
    (out/f'{name}-ast.log').write_bytes(p.stderr);assert p.returncode==0,(name,p.stderr.decode());proofs=json.loads(p.stdout)
   for address,x in a.items():
    y=b[address]
    if x==y:continue
    proof=proofs[hex(address)]
    assert {k:v for k,v in x.items()if k not in ['code','line_mappings','variables']}=={k:v for k,v in y.items()if k not in ['code','line_mappings','variables']}
    normalized=lambda v:{k:value for k,value in v.items()if k not in ['addresses','line_numbers']}
    assert list(map(normalized,x['variables']))==list(map(normalized,y['variables']))
    lines={line for site in proof['changes']for line in range(site['before']['source_line_start'],site['before']['source_line_end']+1)}
    call_addresses={addr for line in x['line_mappings']if line['line_number']in lines for addr in line['addresses']}
    addresses=lambda d:{addr for line in d['line_mappings']for addr in line['addresses']}
    lost=addresses(x)-addresses(y);assert not addresses(y)-addresses(x);assert lost<=call_addresses
    if image is None:image=Image(binary.read_bytes());symbols=demangled_symbols(binary)
    instructions=[]
    for addr in sorted(lost):
     decoder=capstone.Cs(capstone.CS_ARCH_X86,capstone.CS_MODE_64);instruction=next(decoder.disasm(image.read(addr,15),addr))
     assert instruction.mnemonic=='lea'and instruction.op_str.split(',')[0]in ['rdx','edx'],(name,x['name'],hex(addr),instruction.mnemonic,instruction.op_str)
     instructions.append({'address':hex(addr),'bytes':instruction.bytes.hex(),'mnemonic':instruction.mnemonic,'operands':instruction.op_str,'classification':'Descriptor-address LEA into RDX on the replaced call line; no memory read/write or flag effect.'})
    variable_changes=[]
    for old_var,new_var in zip(x['variables'],y['variables']):
     if old_var==new_var:continue
     assert set(old_var['addresses'])^set(new_var['addresses'])<=call_addresses
     assert set(old_var['line_numbers'])^set(new_var['line_numbers'])<=lines
     variable_changes.append({'name':old_var['name'],'added':sorted(set(new_var['addresses'])-set(old_var['addresses'])),'lost':sorted(set(old_var['addresses'])-set(new_var['addresses']))})
    assert name in initial_labels,('New changed image requires operand extraction',name,x['name'])
    calls=[p['call']for p in raw_descriptors if p['fixture']==initial_labels[name]and p['address']==address];assert len(calls)==1
    for site in proof['changes']:verify_site(site,x,calls,image,symbols)
    initial=records(w/f'{initial_labels[name]}-after.json');assert initial[address]==y,('Fresh complete record differs from initial native evidence',name,x['name'])
    diff=''.join(difflib.unified_diff(x['code'].splitlines(True),y['code'].splitlines(True),fromfile='before/'+x['name'],tofile='after/'+y['name']))
    (out/f'{name}-{address:x}.diff').write_text(diff)
    language_record['changes'].append({'name':x['name'],'address':hex(address),'proof':proof,'lost_instruction_mappings':instructions,'variable_mapping_changes':variable_changes,'hunks':sum(s.startswith('@@')for s in diff.splitlines()),'complete_record_matches_initial_native_evidence':True})
   assert len(proofs)==len(language_record['changes'])
   if name in ['rust','write-native']:
    string_prior=next(x for x in prior[name]['captures']if x['language']=='rust'and x['mode']=='on');old_on=Path(string_prior['fresh_file']);assert sha(old_on)==string_prior['fresh_sha256']
    string_on=out/f'{name}-rust-string-on-io-off.json';capture(new,binary,'rust',string_on,['--option','rustfmtwrite','on','--option','rustfmtio','off'])
    assert records(string_on)==records(old_on);language_record['opt_in_string_recovery_unchanged']=True
  result['languages'].append(language_record)
 (out/f'{name}-record.json').write_text(json.dumps(result,indent=2)+'\n')
 print(name,[(x['language'],x['functions'],len(x['changes']))for x in result['languages']],flush=True)
 return result
results=[]
with ThreadPoolExecutor(max_workers=8)as pool:
 for future in as_completed([pool.submit(run,row)for row in inputs]):results.append(future.result())
assert sha(new)==build['cli_sha256']and sha(parent)==parent_sha
changed=sum(len(lang['changes'])for row in results for lang in row['languages']);assert changed==11
summary={'images':len(results),'distinct_functions':sum(row['languages'][0]['functions']for row in results),'function_records':sum(lang['functions']for row in results for lang in row['languages']),'changed_functions':changed,'hunks':sum(change['hunks']for row in results for lang in row['languages']for change in lang['changes']),'C_Go_identical':sum(lang['functions']for row in results for lang in row['languages']if lang['language']!='rust'),'Go_parses':sum(lang['functions']for row in results for lang in row['languages']if lang['language']=='go'),'off_matches_parent':sum(lang['functions']for row in results for lang in row['languages']if lang['language']=='rust')}
(out/'record.json').write_text(json.dumps({'new_kuna_sha256':build['cli_sha256'],'baseline_kuna_sha256':parent_sha,'baseline_commit':parent_head,'earlier_cached_parent_commit':'a874bff7a12391fe05867088692c1a23d8d3c4fe','cache_scope':'The current parent integrates CLI byte-read provenance only; decompiler/analysis/console sources are unchanged. Fresh feature-off records must exactly match the earlier cached parent for every function. Newly introduced fixture C/Go captures use the current parent directly.','semantic_evidence_pending':False,'native_reuse_basis':'Every changed complete function record is freshly checked equal to the initial generation whose unchanged expressions executed in the native comparison.','summary':summary,'records':results},indent=2)+'\n')
print(json.dumps(summary,indent=2),flush=True)
