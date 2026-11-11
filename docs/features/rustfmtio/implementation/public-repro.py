from pathlib import Path
from concurrent.futures import ThreadPoolExecutor
import subprocess,json,hashlib,os
w=Path('/tmp/kuna-rust-io-optin');old=Path('/tmp/kuna-rust-io-proposal');fixture=Path('/tmp/kuna-rust-stdout-repro/repro');sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest();prior=json.loads((old/'public-repro.json').read_text());assert sha(fixture)==prior['fixture_sha256']
def capture(row):
 side,root=row;r=Path(root);cmd=[str(r/'decompiler/target/release/kuna'),'decompile',str(fixture),'write_value','--language','rust'];cmd+=['--option','rustfmtio','on']if side=='after'else [];p=subprocess.run(cmd,cwd=r,env=os.environ|{'KUNA_SPECS':str(r/'specs')},capture_output=True);(w/f'public-repro-{side}.rs').write_bytes(p.stdout);(w/f'public-repro-{side}.log').write_bytes(p.stderr);assert p.returncode==0,p.stderr.decode();assert p.stdout==(old/f'public-repro-{side}.rs').read_bytes();return{'side':side,'cli_sha256':sha(r/'decompiler/target/release/kuna'),'complete_output_unchanged':True,'output_sha256':hashlib.sha256(p.stdout).hexdigest()}
with ThreadPoolExecutor(max_workers=2)as pool:records=list(pool.map(capture,[('before','/home/mahaloz/kwt/angr-rust-fmt-write-ddd'),('after','/home/mahaloz/kwt/angr-rust-io-proposal')]))
(w/'public-repro.json').write_text(json.dumps({'fixture_sha256':sha(fixture),'records':records,'all_outputs_unchanged':True},indent=2)+'\n');print('PUBLIC REPRO CURRENT MAIN BEFORE/AFTER IDENTICAL')
