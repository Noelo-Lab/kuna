from pathlib import Path
import subprocess,json,hashlib
w=Path(__file__).resolve().parent;r=Path('/home/mahaloz/kwt/angr-narrow-table-guard');base=Path('/home/mahaloz/kwt/angr-rust-empty-match')
subprocess.run(['python3',str(w/'public-repro-source.py')],check=True)
x=w/'public-repro';subprocess.run(['cc','-no-pie','-nostdlib','-Wl,-e,pick',str(x/'guard.S'),'-o',str(x/'guard')],check=True)
records={}
for label,root,opts in [('parent',base,[]),('default',r,[]),('enabled',r,['--option','highsliceguard','on'])]:
 p=subprocess.run([str(root/'decompiler/target/release/kuna'),'decompile',str(x/'guard'),'pick',*opts],cwd=root,capture_output=True,text=True,check=True);(x/(label+'.c')).write_text(p.stdout);(x/(label+'.log')).write_text(p.stderr);records[label]={'switches':p.stdout.count('switch('),'cases':p.stdout.count('case '),'jumps':p.stdout.count('jump-as-call'),'cli_sha256':hashlib.sha256((root/'decompiler/target/release/kuna').read_bytes()).hexdigest()}
assert (x/'default.c').read_text()==(x/'parent.c').read_text()
assert records['default']['jumps']==1 and records['default']['switches']==0
assert records['enabled']['jumps']==0 and records['enabled']['switches']==1 and records['enabled']['cases']==41
(x/'record.json').write_text(json.dumps(records,indent=2)+'\n');print(records,flush=True)
