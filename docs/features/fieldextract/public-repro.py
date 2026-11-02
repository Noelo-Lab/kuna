from common import *
out=w/'public-repro';out.mkdir(exist_ok=False)
source='''struct Pair { long first,second; };
__attribute__((noinline,noclone)) struct Pair pair(long a,long b) { return (struct Pair){a+b,a-b}; }
long widened_narrow(long a,long b) { return (int)pair(a,b).second; }
'''
(out/'field.c').write_text(source)
subprocess.run(['cc','-O2','-g','-fwrapv','-fno-pie','-no-pie','-nostdlib','-Wl,-e,widened_narrow',str(out/'field.c'),'-o',str(out/'field')],check=True)
records={};base=Path('/home/mahaloz/kwt/angr-joined-callback-return')
for arm,root in [('parent',base),('candidate',r)]:
 cmd=[str(root/'decompiler/target/release/kuna'),'decompile',str(out/'field'),'widened_narrow','--assert','prototype pair struct Pair pair(long a,long b)','--assert','prototype widened_narrow long widened_narrow(long a,long b)'];p=subprocess.run(cmd,env=env|{'KUNA_SPECS':str(r/'specs')},capture_output=True,text=True,check=True);(out/f'{arm}.c').write_text(p.stdout);(out/f'{arm}.log').write_text(p.stderr);records[arm]={'command':cmd,'stdout':p.stdout,'stderr':p.stderr,'cli_sha256':hashlib.sha256((root/'decompiler/target/release/kuna').read_bytes()).hexdigest()};print(arm,p.stdout)
assert 'return pair(a,b).second;'in records['parent']['stdout'];assert '(int)pair(a,b).second'in records['candidate']['stdout']
(out/'record.json').write_text(json.dumps(records,indent=2)+'\n')
