from common import *
import itertools
out=w/'native-extra';sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();k=r/'decompiler/target/release/kuna';digest=sha(k);inputs=list(itertools.product([-(1<<63),-4294967297,-17,-1,0,1,17,4294967297,(1<<63)-1],[-(1<<63),-1,0,1,(1<<63)-1]));records=[]
go_harness='''
var calls int
func counted(a,b int64)(int64,int64){calls++;return pair(a,b)}
func main(){a,_:=strconv.ParseInt(os.Args[1],10,64);b,_:=strconv.ParseInt(os.Args[2],10,64);x,y:=pair(a,b);fmt.Println(x,y);calls=0;x,y=apply(counted,a,b);fmt.Println(x,y,calls);calls=0;x=indirectFirst(counted,a,b);fmt.Println(x,calls);calls=0;x=indirectSecond(counted,a,b);fmt.Println(x,calls);calls=0;n:=indirectNarrow(counted,a,b);fmt.Println(n,calls);x,y=identityPair(a,b);fmt.Println(x,y);x,u:=signedUnsigned(a,b);fmt.Println(x,u)}
'''
c_prefix='''#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
typedef int64_t int8;typedef uint64_t uint8;typedef int32_t int4;
typedef struct { int8 result0,result1; } __kuna_go_pair_i64_i64;
typedef struct { int8 result0;uint8 result1; } __kuna_go_pair_i64_u64;
'''
c_harness='''
static int calls;
static __kuna_go_pair_i64_i64 counted(int8 a,int8 b){calls++;return pair(a,b);}
int main(int argc,char**argv){(void)argc;int8 a=strtoll(argv[1],0,10),b=strtoll(argv[2],0,10);__kuna_go_pair_i64_i64 p=pair(a,b);printf("%ld %ld\\n",p.result0,p.result1);calls=0;p=apply(counted,a,b);printf("%ld %ld %d\\n",p.result0,p.result1,calls);calls=0;int8 x=indirectFirst(counted,a,b);printf("%ld %d\\n",x,calls);calls=0;x=indirectSecond(counted,a,b);printf("%ld %d\\n",x,calls);calls=0;int4 n=indirectNarrow(counted,a,b);printf("%d %d\\n",n,calls);p=identityPair(a,b);printf("%ld %ld\\n",p.result0,p.result1);__kuna_go_pair_i64_u64 m=signedUnsigned(a,b);printf("%ld %lu\\n",m.result0,m.result1);}
'''
(out/'candidate.go').write_text('package main\nimport("fmt";"os";"strconv")\n'+(out/'go.bodies').read_text()+go_harness)
(out/'candidate.c').write_text(c_prefix+(out/'c.bodies').read_text()+c_harness)
wrap=lambda a,bits=64:((a+(1<<(bits-1)))%(1<<bits))-(1<<(bits-1))
for name,cmd,source in [(v,['/tmp/kuna-toolchains/'+v+'/go/bin/go','build','-p','4','-trimpath'],out/'candidate.go')for v in ['go1.22.5','go1.27.3']]+[(o,['cc','-'+o,'-std=c11','-fwrapv','-Wall','-Wextra','-Werror'],out/'candidate.c')for o in ['O0','O2']]:
 exe=out/name;p=subprocess.run(cmd+['-o',str(exe),str(source)],capture_output=True,text=True,env=env|{'GOTOOLCHAIN':'local','GOMAXPROCS':'4','CGO_ENABLED':'0'});(out/(name+'-build.log')).write_text(p.stdout+p.stderr);assert p.returncode==0,p.stderr
 observations=[]
 for a,b in inputs:
  x,y=wrap(a+b),wrap(a-b);expected=f'{x} {y}\n{x} {y} 1\n{x} 1\n{y} 1\n{wrap(y,32)} 1\n{a} {b}\n{x} {y%(1<<64)}\n';actual=subprocess.check_output([str(exe),str(a),str(b)],text=True);assert actual==expected,(name,a,b,actual,expected);observations.append({'a':a,'b':b,'stdout':actual})
 records.append({'name':name,'command':cmd,'complete_bodies':7,'body_rewrites':False,'source_sha256':sha(source),'executable_sha256':sha(exe),'observations':observations,'inputs':len(inputs)})
assert digest==sha(k);(out/'record.json').write_text(json.dumps({'cli_sha256':digest,'scope':'Complete recovered direct/indirect/identity/mixed-sign/narrowed bodies, source-equivalent modular arithmetic oracle, one callback invocation. Asserted callback types; function renames only for C.','native_input_comparisons':180,'records':records},indent=2)+'\n');print('PASS 7 complete bodies, 45 inputs each, 2 Go compilers and C O0/O2')
