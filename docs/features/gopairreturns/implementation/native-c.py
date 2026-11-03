from pathlib import Path
import hashlib,itertools,json,os,re,subprocess
r=Path('/home/mahaloz/kwt/angr-go-pair-results');w=Path('/tmp/kuna-go-pair-results-implementation/integration-460/native-c');w.mkdir(parents=True,exist_ok=False);k=r/'decompiler/target/release/kuna';sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();digest=sha(k);env=os.environ|{'KUNA_SPECS':str(r/'specs')};names=['pair','divmod','unsignedPair','branchPair','consume','first','second','forward','effect','consumeEffect','zeroPair','lowPair','signedLowPair','highPair','onesPair','choosePair'];inputs=list(itertools.product([-(1<<63),-(1<<63)+1,-65536,-17,-2,-1,0,1,2,17,65536,(1<<31)-1,(1<<32)-1,(1<<63)-2,(1<<63)-1],[-(1<<63),-17,-1,0,1,17,(1<<63)-1]));records=[]
prelude='''#include <stdio.h>
#include <stdlib.h>
typedef unsigned char xunknown1;
typedef unsigned long long xunknown8;
typedef long long int8;
typedef unsigned long long uint8;
typedef struct {int8 result0,result1;} __kuna_go_pair_i64_i64;
typedef struct {uint8 result0,result1;} __kuna_go_pair_u64_u64;
typedef struct {int8 result0;uint8 result1;} __kuna_go_pair_i64_u64;
static uint8 calls;
'''
harness='''int main(int argc,char **argv){int8 a=strtoll(argv[1],0,10),b=strtoll(argv[2],0,10);__kuna_go_pair_i64_i64 v=pair(a,b);printf("%lld %lld\\n",v.result0,v.result1);v=forward(a,b);printf("%lld %lld\\n",v.result0,v.result1);v=branchPair(a,b);printf("%lld %lld\\n",v.result0,v.result1);__kuna_go_pair_u64_u64 u=unsignedPair((uint8)a,(uint8)b);printf("%llu %llu\\n",u.result0,u.result1);printf("%lld %lld %lld\\n",consume(a,b),first(a,b),second(a,b));calls=0;int8 e=consumeEffect(a);printf("%lld %llu\\n",e,calls);u=zeroPair();printf("%llu %llu\\n",u.result0,u.result1);u=lowPair((uint8)a);printf("%llu %llu\\n",u.result0,u.result1);v=signedLowPair(a);printf("%lld %lld\\n",v.result0,v.result1);u=highPair((uint8)a);printf("%llu %llu\\n",u.result0,u.result1);v=onesPair();printf("%lld %lld\\n",v.result0,v.result1);u=choosePair((uint8)a,(uint8)b);printf("%llu %llu\\n",u.result0,u.result1);if(b!=0){v=divmod(a,b);printf("%lld %lld\\n",v.result0,v.result1);}return 0;}
'''
for version in ['go1.22.5','go1.27.3']:
 binary=r/'tests/fixtures/go-pair-results'/(version+'-pair');symbols={parts[2]:int(parts[0],16)for line in subprocess.check_output(['nm','-n',str(binary)],text=True).splitlines()if len(parts:=line.split())==3};assertions=[f'function {symbols["main."+name]:x}={name}'for name in names]+[f'data {symbols["main.calls"]:x} uint8 calls'];cmd=[str(k),'decompile-all',str(binary),'--functions',','.join(names),'--language','c','--option','godwarfsig','on','--option','gopairreturns','on','--option','ctypes','off','--option','realtypes','off','--json']
 for assertion in assertions:cmd+=['--assert',assertion]
 p=subprocess.run(cmd,cwd=r,env=env,text=True,capture_output=True,check=True);(w/(version+'.json')).write_text(p.stdout);(w/(version+'.log')).write_text(p.stderr);document=json.loads(p.stdout);assert all(a['status']=='applied'for a in document['assertions']);fs=document['functions'];assert len(fs)==16 and all(not f['error']for f in fs);bodies='\n'.join(f['code']for f in fs);source=w/(version+'.c');source.write_text(prelude+bodies+'\n'+harness)
 for opt in ['0','2']:
  exe=w/(version+'-O'+opt);p=subprocess.run(['cc','-std=c11','-O'+opt,'-fwrapv',str(source),'-o',str(exe)],text=True,capture_output=True);(w/(exe.name+'-build.log')).write_text(p.stdout+p.stderr);assert p.returncode==0,p.stderr;observations=[]
  for a,b in inputs:
   reference=Path('/tmp/kuna-go-pair-results-implementation/integration-460/native-go')/(version+'-source');expected=subprocess.check_output([str(reference),str(a),str(b)],text=True)
   if b==0:expected='\n'.join(expected.splitlines()[:-1])+'\n'
   actual=subprocess.check_output([str(exe),str(a),str(b)],text=True);assert actual==expected,(version,opt,a,b,expected,actual);observations.append({'a':a,'b':b,'division_executed':b!=0,'stdout':actual})
  records.append({'version':version,'optimization':'O'+opt,'complete_bodies':16,'body_rewrites':False,'source_sha256':sha(source),'binary_sha256':sha(exe),'assertions':assertions,'inputs':105,'nonzero_divisions':90,'observations':observations})
assert sha(k)==digest
(w/'record.json').write_text(json.dumps({'cli_sha256':digest,'scope':'Ordinary C struct representation of the automatically recovered Go pair signatures. Function/global names are asserted for C identifier legality; ctypes/realtypes off preserve explicit widths. Complete bodies unchanged. -fwrapv preserves machine integer wrap. C division by zero is not executed or claimed.','native_comparisons':420,'records':records},indent=2)+'\n');print('C PAIR PROOF PASS:420 inputs,16 full bodies/version atO0/O2,360 nonzero division cases.',flush=True)
