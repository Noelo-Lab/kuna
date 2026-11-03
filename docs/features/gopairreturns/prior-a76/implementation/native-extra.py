from common import *
import itertools
out=w/'native-extra';out.mkdir(exist_ok=False);k=r/'decompiler/target/release/kuna';sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();digest=sha(k)
names=['pair','apply','indirectFirst','indirectSecond','identityPair','signedUnsigned','indirectNarrow'];pair='struct __kuna_go_pair_i64_i64';assertions=[f'prototype main.{name} {ret} {name}({pair} (*fn)(int8,int8),int8 r0,int8 r1)' for name,ret in [('apply',pair),('indirectFirst','int8'),('indirectSecond','int8'),('indirectNarrow','int4')]]
for language in ['go','c']:
 cmd=[str(k),'decompile-all',str(r/'tests/fixtures/go-pair-results/contracts'),'--functions',','.join('main.'+n for n in names),'--language',language,'--option','godwarfsig','on','--option','gopairreturns','on','--option','ctypes','off','--option','realtypes','off','--json']
 for a in assertions:cmd+=['--assert',a]
 if language=='c':
  listing=subprocess.check_output(['nm','-n',str(r/'tests/fixtures/go-pair-results/contracts')],text=True)
  for line in listing.splitlines():
   fields=line.split()
   if len(fields)==3 and fields[2]in ['main.'+n for n in names]:cmd+=['--assert','function '+fields[0]+'='+fields[2][5:]]
 p=subprocess.run(cmd,cwd=r,env=env|{'KUNA_SPECS':str(r/'specs')},capture_output=True,text=True,check=True);(out/(language+'.json')).write_text(p.stdout);(out/(language+'.log')).write_text(p.stderr)
 document=json.loads(p.stdout);assert all(a['status']=='applied'for a in document['assertions']);fs=document['functions'];assert len(fs)==7 and all(not f.get('error')for f in fs),fs;assert not p.stderr,p.stderr
 (out/(language+'.bodies')).write_text('\n'.join(f['code']for f in fs))
print('Captured seven complete indirect/identity/mixed-sign bodies per language',flush=True)
