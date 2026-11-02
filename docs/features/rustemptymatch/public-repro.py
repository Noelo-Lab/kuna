from pathlib import Path
import subprocess,json,hashlib,shutil,datetime
w=Path('/tmp/kuna-rust-empty-match-db2d');out=w/'public-repro';out.mkdir(exist_ok=True);old=Path('/tmp/kuna-rust-empty-match/public-repro');r=Path('/home/mahaloz/kwt/angr-rust-empty-match');base=Path('/home/mahaloz/kwt/angr-base-db2d');shutil.copy2(old/'empty.c',out/'empty.c')
subprocess.run(['cc','-O0','-fno-pie','-no-pie','-nostdlib','-Wl,-e,check',str(out/'empty.c'),'-o',str(out/'empty')],check=True)
record={}
for arm,root in [('parent',base),('candidate',r)]:
 cli=root/'decompiler/target/release/kuna';p=subprocess.run([str(cli),'decompile',str(out/'empty'),'check','--language','rust','--assert','prototype check int check(int x)','--json'],cwd=r,text=True,capture_output=True,check=True);(out/(arm+'.json')).write_text(p.stdout);(out/(arm+'.log')).write_text(p.stderr);doc=json.loads(p.stdout);assert doc['error'] is None and len(doc['functions'])==1;code=doc['functions'][0]['code'];source=code+'\nfn main(){unsafe{println!("{}",check(0));}}\n';(out/(arm+'.rs')).write_text(source);build=subprocess.run(['rustc','--edition=2021','--crate-name','empty_match_repro',str(out/(arm+'.rs')),'-o',str(out/arm)],text=True,capture_output=True);(out/(arm+'-build.log')).write_text(build.stdout+build.stderr)
 if arm=='parent':assert build.returncode and 'E0268' in build.stderr
 else:
  assert build.returncode==0,build.stderr
  assert subprocess.check_output([str(out/arm)],text=True).strip()=='7'
 record[arm]={'cli_sha256':hashlib.sha256(cli.read_bytes()).hexdigest(),'compile_exit':build.returncode,'result':'E0268'if arm=='parent'else'7','complete_body_unchanged':True}
record['at']=datetime.datetime.now(datetime.timezone.utc).isoformat();(out/'record.json').write_text(json.dumps(record,indent=2)+'\n');print(record)
