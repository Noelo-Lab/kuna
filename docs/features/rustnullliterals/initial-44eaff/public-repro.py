from pathlib import Path
import hashlib,json,os,subprocess
w=Path(__file__).resolve().parent;r=Path('/home/mahaloz/kwt/angr-rust-null-literals');base=Path('/home/mahaloz/kwt/angr-narrow-table-guard');out=w/'public-repro';out.mkdir(exist_ok=False);(out/'null.c').write_text('unsigned char *zero(void) { return 0; }\n');subprocess.run(['cc','-O0','-fno-pie','-no-pie','-nostdlib','-Wl,-e,zero',str(out/'null.c'),'-o',str(out/'null')],check=True)
record={}
for arm,root in [('parent',base),('candidate',r)]:
 p=subprocess.run([str(root/'decompiler/target/release/kuna'),'decompile',str(out/'null'),'zero','--language','rust','--assert','prototype zero unsigned char *zero(void)'],cwd=r,env=os.environ|{'KUNA_SPECS':str(r/'specs')},capture_output=True,text=True,check=True);(out/f'{arm}.rs').write_text(p.stdout+'\nfn main() { assert!(unsafe { zero() }.is_null()); }\n');(out/f'{arm}.log').write_text(p.stderr)
 q=subprocess.run(['rustc','--edition=2021','--crate-name','null_repro',str(out/f'{arm}.rs'),'-o',str(out/arm)],capture_output=True,text=True);(out/f'{arm}-build.log').write_text(q.stdout+q.stderr)
 if arm=='parent':assert q.returncode and 'E0425'in q.stderr
 else:assert q.returncode==0;subprocess.run([str(out/arm)],check=True)
 record[arm]={'cli_sha256':hashlib.sha256((root/'decompiler/target/release/kuna').read_bytes()).hexdigest(),'compile_exit':q.returncode}
(out/'record.json').write_text(json.dumps(record,indent=2)+'\n');print('PUBLIC NULL REPRO PASS:parent E0425,candidate compiles and returns null.',flush=True)
