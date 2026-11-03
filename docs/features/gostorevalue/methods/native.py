from common import *
import itertools
f=r/'decompiler/crates/kuna-analysis/tests/fixtures';meta=json.loads((w/'fixture.json').read_text());cases=[row for row in meta['cases']if row['name']!='nested_store'];values=[0,1,2,127,128,255,256,32767,32768,65535,65536,2147483647,2147483648,4294967295,4294967296,9223372036854775807,9223372036854775808,18446744073709551615]
gprefix='''package main
import("fmt";"math";"unsafe")
type Fields struct{s8 int8;u8 byte;s16 int16;u16 uint16;s32 int32;u32 uint32;s64 int64;u64 uint64}
var pointer_calls,value_calls uint32
func counted_destination(p *Fields)*Fields{pointer_calls++;return p}
func counted_value(a,b uint64)uint64{value_calls++;return a+b}
/* EMITTED_BODIES */
func main(){
values:=[]uint64{'''+','.join(str(v)for v in values)+'''}
for _,a:=range values {for _,b:=range values {
'''
glines=[];clines=[]
for case in cases:
 name=case['name'];n=case['width'];kind=case['kind'];s=f'int{n}';u=f'uint{n}';cs={8:'signed char',16:'short',32:'int',64:'long'}[n];cu='unsigned '+{'signed char':'char','short':'short','int':'int','long':'long'}[cs]
 init='p:=Fields{-3,5,-7,11,-13,17,-19,23};pointer_calls=0;value_calls=0;'
 ci='struct Fields p={-3,5,-7,11,-13,17,-19,23};pointer_calls=0;value_calls=0;'
 if kind in ['signed','unsigned','signed_plain','unsigned_plain','signed_xor','signed_constant','unsigned_constant','side_effect']:
  pointer='&p'+(f'.s{n}'if kind=='signed_plain'else f'.u{n}'if kind=='unsigned_plain'else'')
  suffix=''if kind.endswith('constant')else f',{s}(a)'if kind=='signed_xor'else f',{u}(a),{u}(b)'if kind in ['signed','signed_plain','side_effect']else f',{s}(a),{s}(b)'
  csuffix=''if kind.endswith('constant')else f',({cs})a'if kind=='signed_xor'else f',({cu})a,({cu})b'if kind in ['signed','signed_plain','side_effect']else f',({cs})a,({cs})b'
  g=init+f'{name}({pointer}{suffix});fmt.Println("{name}",uint64(p.s8),p.u8,uint64(p.s16),p.u16,uint64(p.s32),p.u32,uint64(p.s64),p.u64,pointer_calls,value_calls)'
  c=ci+f'{name}({pointer}{csuffix});printf("{name} %lu %lu %lu %lu %lu %lu %lu %lu %u %u\\n",(unsigned long)p.s8,(unsigned long)p.u8,(unsigned long)p.s16,(unsigned long)p.u16,(unsigned long)p.s32,(unsigned long)p.u32,(unsigned long)p.s64,(unsigned long)p.u64,pointer_calls,value_calls);'
 elif kind=='array_store':
  g='arr:=[3]int64{9,10,11};array_store(&arr[0],a%3,b);fmt.Println("array_store",uint64(arr[0]),uint64(arr[1]),uint64(arr[2]))'
  c='long arr[3]={9,10,11};array_store(arr,a%3,b);printf("array_store %lu %lu %lu\\n",(unsigned long)arr[0],(unsigned long)arr[1],(unsigned long)arr[2]);'
 elif kind=='float_store':
  g='var dest float64;float_store(&dest,math.Float64frombits(a));fmt.Println("float_store",math.Float64bits(dest))'
  c='double dest,input;memcpy(&input,&a,8);float_store(&dest,input);unsigned long bits;memcpy(&bits,&dest,8);printf("float_store %lu\\n",bits);'
 elif kind=='pointer_store':
  g='value:=int64(a);var dest *int64;pointer_store(&dest,&value);fmt.Println("pointer_store",uint64(*dest))'
  c='long value=(long)a,*dest;pointer_store(&dest,&value);printf("pointer_store %lu\\n",(unsigned long)*dest);'
 else:
  assert kind=='bool_store';g='var dest bool;bool_store(&dest,a&1!=0);v:=0;if dest{v=1};fmt.Println("bool_store",v)';c='_Bool dest;bool_store(&dest,(a&1)!=0);printf("bool_store %u\\n",(unsigned int)dest);'
 glines.append('{'+g+'}');clines.append('{'+c+'}')
go=gprefix+'\n'.join(glines)+'\n}}}\n';ch='''#include <stdio.h>
#include <string.h>
int main(void){unsigned long values[]={'''+','.join(str(v)+'UL'for v in values)+'''};for(unsigned i=0;i<sizeof(values)/sizeof(*values);i++){for(unsigned j=0;j<sizeof(values)/sizeof(*values);j++){unsigned long a=values[i],b=values[j];
'''+ '\n'.join(clines)+'\n}}return 0;}\n'
(f/'go_store_value_harness.go').write_text(go);(f/'go_store_value_harness.c').write_text(ch)
source=(f/'go_store_value.c').read_text().split('int main(void)')[0]+'\n'+'\n'.join((f/'go_store_value_helpers.c').read_text().splitlines()[1:])+'\n'+ch
out=w/'native';out.mkdir(exist_ok=False);(out/'source.c').write_text(source);expected=None;records=[];sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
for opt in ['O0','O2']:
 exe=out/('source-'+opt);cmd=['cc','-'+opt,'-std=c11','-fwrapv','-Wall','-Wextra','-Werror',str(out/'source.c'),'-o',str(exe)];run(r,out,cmd,'source-'+opt+'-build.log');output=subprocess.check_output([str(exe)]);(out/('source-'+opt+'.stdout')).write_bytes(output)
 if expected is None:expected=output
 else:assert output==expected
 records.append({'arm':'source','optimization':opt,'source_sha256':sha(out/'source.c'),'executable_sha256':sha(exe),'stdout_sha256':hashlib.sha256(output).hexdigest()})
assert len(expected.splitlines())==len(cases)*len(values)**2
for arm in ['parent','candidate']:
 d=json.loads((w/(arm+'-go.json')).read_text());bodies='\n'.join(fn['code']for fn in d['functions']if fn['name']!='nested_store');assert len([fn for fn in d['functions']if fn['name']!='nested_store'])==33;file=out/(arm+'.go');file.write_text(go.replace('/* EMITTED_BODIES */',bodies))
 for version in ['go1.22.5','go1.27.3']:
  exe=out/(arm+'-'+version);cmd=['/tmp/kuna-toolchains/'+version+'/go/bin/go','build','-p','4','-trimpath','-o',str(exe),str(file)];p=subprocess.run(cmd,capture_output=True,text=True,env=env|{'GOTOOLCHAIN':'local','GOMAXPROCS':'4','CGO_ENABLED':'0'});(out/(arm+'-'+version+'-build.log')).write_text(p.stdout+p.stderr)
  if arm=='parent':assert p.returncode!=0;continue
  assert p.returncode==0,p.stderr;actual=subprocess.check_output([str(exe)]);(out/(arm+'-'+version+'.stdout')).write_bytes(actual);assert actual==expected,version;records.append({'arm':arm,'compiler':version,'source_sha256':sha(file),'executable_sha256':sha(exe),'stdout_sha256':hashlib.sha256(actual).hexdigest()})
record={'complete_bodies':33,'changed_bodies':22,'inputs_per_run':len(values)**2,'native_go_input_comparisons':2*len(values)**2,'function_comparisons':2*len(values)**2*len(cases),'output_lines_per_run':len(expected.splitlines()),'body_rewrites':False,'exclusions':{'nested_store':'Unchanged preexisting nested-array to unsafe.Pointer rendering error; retained parent/candidate whole bodies and excluded from native proof.'},'candidate_cli_sha256':sha(r/'decompiler/target/release/kuna'),'records':records};(out/'record.json').write_text(json.dumps(record,indent=2)+'\n');print('PASS native complete STORE bodies',json.dumps(record),flush=True)
