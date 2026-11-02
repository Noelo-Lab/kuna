from common import *
base=Path('/home/mahaloz/kwt/angr-base-83a6f');out=w/'public-repro';out.mkdir(exist_ok=False)
(out/'callback.c').write_text('struct Pair { long first,second; };\nlong second(struct Pair (*fn)(long,long),long a,long b) { return fn(a,b).second; }\n')
subprocess.run(['cc','-O2','-fno-pie','-no-pie','-nostdlib','-Wl,-e,second',str(out/'callback.c'),'-o',str(out/'callback')],check=True)
record={}
for arm,root in [('parent',base),('candidate',r)]:
 cmd=[str(root/'decompiler/target/release/kuna'),'decompile',str(out/'callback'),'second','--assert','typedef struct Pair { long first; long second; };','--assert','prototype second long second(struct Pair (*fn)(long,long),long a,long b)']
 p=subprocess.run(cmd,env=env|{'KUNA_SPECS':str(r/'specs')},text=True,capture_output=True,check=True);(out/f'{arm}.c').write_text(p.stdout);(out/f'{arm}.log').write_text(p.stderr)
 if arm=='parent':assert '(*fn)(a,b);'in p.stdout and 'return v1;'in p.stdout
 else:assert '(*fn)(a,b).second;'in p.stdout
 record[arm]={'cli_sha256':hashlib.sha256((root/'decompiler/target/release/kuna').read_bytes()).hexdigest(),'output':p.stdout}
(out/'record.json').write_text(json.dumps(record,indent=2)+'\n');print('PUBLIC REPRO PASS',flush=True)
