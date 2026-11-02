from pathlib import Path
import datetime,hashlib,json,os,subprocess
w=Path('/tmp/kuna-highslice-optin');r=Path('/home/mahaloz/kwt/angr-narrow-table-guard');base=Path('/home/mahaloz/kwt/angr-rust-empty-match');out=w/'combined-native';out.mkdir(exist_ok=False);f=r/'decompiler/crates/kuna-analysis/tests/fixtures';names=['empty_case','empty_default','empty_shared','empty_loop','loop_break_control'];build=json.loads((w/'build.json').read_text());sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();cli=r/'decompiler/target/release/kuna';assert sha(cli)==build['new_cli_sha256']
assembly=(f/'rust_empty_match.S').read_text();assert assembly.count('cmp $40,%rdi')==4;(out/'narrow.S').write_text(assembly.replace('cmp $40,%rdi','cmp $40,%edi'));command=['cc','-g','-no-pie',str(out/'narrow.S'),str(f/'rust_empty_match.c'),'-o',str(out/'original')];subprocess.run(command,check=True,capture_output=True)
captures={}
for arm,root in [('parent',base),('candidate',r)]:
 p=subprocess.run([str(root/'decompiler/target/release/kuna'),'decompile-all',str(out/'original'),'--functions',','.join(names),'--language','rust','--json','--assert','@'+str(f/'rust_empty_match.kuna')]+(['--option','highsliceguard','on']if arm=='candidate'else[]),env=os.environ|{'KUNA_SPECS':str(r/'specs')},capture_output=True,text=True,check=True);(out/(arm+'.json')).write_text(p.stdout);(out/(arm+'.log')).write_text(p.stderr);fs=json.loads(p.stdout)['functions'];assert len(fs)==5 and all(not v.get('error')for v in fs);captures['rust',arm]={v['name']:v for v in fs}
assert 'jump-as-call'in captures['rust','parent']['empty_case']['code']
assert sum(f['code'].count('match ')for f in captures['rust','candidate'].values())==4
for name,wanted in [('empty_case',0),('empty_default',0),('empty_shared',0),('empty_loop',1),('loop_break_control',1)]:
 code=captures['rust','candidate'][name]['code'];assert code.count('break;')==wanted and 'jump-as-call'not in code
source=out/'recovered.rs';source.write_text('\n'.join(captures['rust','candidate'][n]['code']for n in names)+'\nfn main(){let args:Vec<String>=std::env::args().collect();let x:u64=args[1].parse().unwrap();let n:i32=args[2].parse().unwrap();unsafe{println!("{} {} {} {} {}",empty_case(x),empty_default(x),empty_shared(x),empty_loop(x,n),loop_break_control(10,n));}}\n')
expected=[]
for high in [*range(43),0x7fffffff,0x80000000,0xffffffff]:
 for low in [0,3,0xffffffff]:
  for count in [-2,0,1,2,7]:
   x=(high<<32)|low;p=subprocess.run([str(out/'original'),str(x),str(count)],capture_output=True,text=True,check=True);values=list(map(int,p.stdout.split()));ordinary=high+7 if high<=40 else 48;default=high+8 if high<=40 else 7;shared=7 if high<=1 else ordinary;assert values==[ordinary,default,shared,ordinary*max(0,count),3*max(0,min(count,4))];expected.append({'input':x,'count':count,'values':values})
records=[]
for opt in ['0','2']:
 exe=out/f'rustc-O{opt}';cmd=['rustc','--edition=2021','-C','opt-level='+opt,str(source),'-o',str(exe)];p=subprocess.run(cmd,capture_output=True,text=True);(out/f'rustc-O{opt}-build.log').write_text(p.stdout+p.stderr);assert p.returncode==0,p.stderr
 for row in expected:
  p=subprocess.run([str(exe),str(row['input']),str(row['count'])],capture_output=True,text=True,check=True);assert list(map(int,p.stdout.split()))==row['values'],(opt,row,p.stdout,p.stderr)
 records.append({'optimization':opt,'checks':expected})
record={'at':datetime.datetime.now(datetime.timezone.utc).isoformat(),'cli_sha256':sha(cli),'parent_cli_sha256':sha(base/'decompiler/target/release/kuna'),'native_values':len(records)*len(expected)*len(names),'input_count':len(expected),'complete_emitted_bodies':5,'source_proven_prototypes':True,'function_bodies_unchanged':True,'narrow_guard_replacements':4,'parent_missing_recovery':True,'all_four_matches_recovered':True,'actual_loop_breaks_retained':2,'fixture_sha256':sha(out/'original'),'assembly_sha256':sha(out/'narrow.S'),'records':records};assert record['native_values']==6900 and sha(cli)==build['new_cli_sha256'];(out/'record.json').write_text(json.dumps(record,indent=2)+'\n');print('Combined high-slice/Rust empty-arm proof PASS:6,900 values from five unchanged bodies.',flush=True)
