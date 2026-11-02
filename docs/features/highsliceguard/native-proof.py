from pathlib import Path
import os,json,subprocess,hashlib,datetime
r=Path('/home/mahaloz/kwt/angr-narrow-table-guard');base=Path('/home/mahaloz/kwt/angr-rust-empty-match');w=Path('/tmp/kuna-highslice-optin');out=w/'native-proof';out.mkdir(exist_ok=False);f=r/'decompiler/crates/kuna-analysis/tests/fixtures';names=json.loads((w/'fixture-names.json').read_text());cli=r/'decompiler/target/release/kuna';build=json.loads((w/'build.json').read_text());sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();assert sha(cli)==build['new_cli_sha256']
def capture(cli,binary,functions,label,facts=None):
 cmd=([str(cli),'decompile',str(binary),functions]if functions.startswith('0x')else[str(cli),'decompile-all',str(binary),'--functions',functions])+['--language','c','--json']
 if cli==r/'decompiler/target/release/kuna':cmd+=['--option','highsliceguard','on']
 if facts:cmd+=['--assert','@'+str(facts)]
 p=subprocess.run(cmd,cwd=r,env=os.environ|{'KUNA_SPECS':str(r/'specs')},capture_output=True,text=True,check=True);(out/(label+'.json')).write_text(p.stdout);(out/(label+'.log')).write_text(p.stderr);fs={v['name']:v for v in json.loads(p.stdout)['functions']};assert fs and not any(v.get('error')for v in fs.values());return fs
fs=capture(cli,f/'highslice_guard_x86_64',','.join(names),'capture',f/'highslice_guard.kuna');assert set(fs)==set(names)and all(v['code'].count('switch(')==1 and v['code'].count('case ')==41 and'jump-as-call'not in v['code']for v in fs.values())
source=out/'recovered.c';source.write_text((f/'highslice_guard.c').read_text()+'\n'+'\n'.join(fs[n]['code']for n in names));inputs=set()
for shift in [32,48,56]:
 max_value=(1<<(64-shift))-1
 for high in [*range(44),max_value>>1,(max_value>>1)+1,max_value]:
  for low in [0,3,(1<<shift)-1]:inputs.add((high<<shift)|low)
expected=[]
for x in sorted(inputs):
 p=subprocess.run([str(f/'highslice_guard_x86_64'),str(x)],capture_output=True,text=True,check=True);values=list(map(int,p.stdout.split()));wanted=[(x>>shift)*7+3 if(x>>shift)<=40 else 1000 for shift in [32,32,32,56,48,32]];assert values==wanted;expected.append({'input':x,'values':values})
records=[]
for compiler in ['gcc','clang']:
 for opt in ['0','2']:
  exe=out/f'{compiler}-O{opt}';cmd=[compiler,'-std=c11','-Wall','-Wextra','-O'+opt,'-fsanitize=undefined','-fno-sanitize-recover=all',str(source),'-o',str(exe)];p=subprocess.run(cmd,capture_output=True,text=True);(out/f'{compiler}-O{opt}-build.log').write_text(p.stdout+p.stderr);assert p.returncode==0
  for row in expected:
   p=subprocess.run([str(exe),str(row['input'])],capture_output=True,text=True,check=True);assert not p.stderr and list(map(int,p.stdout.split()))==row['values'],(compiler,opt,row,p.stdout,p.stderr)
  records.append({'compiler':compiler,'optimization':opt,'ubsan':True,'checks':expected})
upstream=[]
for arm,root in [('parent',base),('candidate',r)]:
 result=capture(root/'decompiler/target/release/kuna',Path('/tmp/kuna-angr-reference/general/probestack'),'0xbcf0','upstream-'+arm);assert len(result)==1;function=next(iter(result.values()));count=function['code'].count('switch(');assert count==(1 if arm=='parent'else 2);upstream.append({'arm':arm,'switches':count,'cli_sha256':sha(root/'decompiler/target/release/kuna'),'capture_file':'upstream-'+arm+'.json'})
record={'at':datetime.datetime.now(datetime.timezone.utc).isoformat(),'cli_sha256':sha(cli),'fixture_sha256':sha(f/'highslice_guard_x86_64'),'native_values':len(records)*len(expected)*len(names),'input_count':len(inputs),'complete_emitted_bodies':6,'source_proven_prototypes':True,'function_bodies_unchanged':True,'supplied_upstream_executable_executed':False,'upstream_static_test':{'angr_commit':'6afa0c79a83d9f677a46b0275cdb63bf16c1979e','binary_commit':'76ad9e296dde8f82b8ede93ce06a578cd1933639','name':'test_decompiling_amd64_single_block_jumptable','records':upstream},'records':records};assert sha(cli)==build['new_cli_sha256'];(out/'record.json').write_text(json.dumps(record,indent=2)+'\n');print('Native proof PASS',record['native_values'],'values;unchanged6bodies;GCC/Clang O0/O2 UBSan;upstream1->2switches',flush=True)
