from common import *
out=w/'native-matrix';out.mkdir(exist_ok=False)
cases=json.loads((w/'matrix-cases.json').read_text());values=[-(1<<63),-(1<<63)+1,-(1<<32)-1,-(1<<32),-65537,-65536,-32769,-32768,-257,-256,-129,-128,-1,0,1,127,128,255,256,32767,32768,65535,65536,(1<<32)-1,1<<32,(1<<32)+1,(1<<63)-2,(1<<63)-1]
chead='#include <stdio.h>\n#include <inttypes.h>\n#include <assert.h>\n'
charness='int main(void) { int64_t values[]={'+','.join('INT64_MIN' if v==-(1<<63)else str(v)+'LL' for v in values)+'}; for (unsigned i=0;i<sizeof(values)/sizeof(*values);++i) { for (unsigned j=0;j<sizeof(values)/sizeof(*values);++j) { int64_t a=values[i],b=values[j];\n'+''.join(f'calls=0; printf("%016" PRIx64 "\\n",(uint64_t){n}(a,b)); assert(calls==1);\n'for n,_,_ in cases)+'}}}\n'
source=chead+(w/'matrix.c').read_text().split('int main(void)')[0]+charness
ctypes=''.join(f'typedef {s}{bits}_t {"int" if s=="int" else "uint"}{bits//8};\n'for s in ['int','uint']for bits in [8,16,32,64])
chelpers=(w/'matrix.c').read_text().split('__attribute__((noinline,noclone)) int8_t signed_int8(')[0]
ghead='''package main
import "fmt"
type Pair struct { first int64; second int64 }
type UPair struct { first uint64; second uint64 }
var calls uint64
func make_pair(a,b int64) Pair { calls++; return Pair{a+b,a-b} }
func make_unsigned(a,b int64) UPair { calls++; return UPair{uint64(a)+uint64(b),uint64(a)-uint64(b)} }
'''
gharness='func main() { values:=[]int64{'+','.join(map(str,values))+'}; for _,a:=range values { for _,b:=range values {\n'+''.join(f'calls=0; fmt.Printf("%016x\\n",uint64({n}(a,b))); if calls!=1 {{panic("{n}")}}\n'for n,_,_ in cases)+'}}}\n'
rhead='''#![allow(non_camel_case_types,dead_code,unused_mut,unused_parens)]
struct Pair { first:i64, second:i64 }
struct UPair { first:u64, second:u64 }
static mut CALLS:u64=0;
unsafe fn make_pair(a:i64,b:i64)->Pair { CALLS+=1; Pair{first:a.wrapping_add(b),second:a.wrapping_sub(b)} }
unsafe fn make_unsigned(a:i64,b:i64)->UPair { CALLS+=1; UPair{first:(a as u64).wrapping_add(b as u64),second:(a as u64).wrapping_sub(b as u64)} }
'''
rharness='fn main() { let values:[i64;'+str(len(values))+']=['+','.join(map(str,values))+']; unsafe { for a in values { for b in values {\n'+''.join(f'CALLS=0; println!("{{:016x}}",{n}(a,b) as u64); assert!(CALLS==1,"{n}");\n'for n,_,_ in cases)+'}}}}\n'
record={'complete_callers':len(cases),'input_pairs':len(values)**2,'comparisons_per_run':len(cases)*len(values)**2,'source_equivalent_helpers':True,'unchanged_emitted_callers':True,'results':[]};expected=None
for arm in ['source','parent','candidate']:
 for lang in (['c']if arm=='source'else ['c','go','rust']):
  if arm=='source':body=source
  else:
   fs=json.loads((w/f'matrix-{arm}-{lang}.json').read_text())['functions'];body='\n'.join(f['code']for f in fs)
   body={'c':chead+ctypes+chelpers,'go':ghead,'rust':rhead}[lang]+body+'\n'+{'c':charness,'go':gharness,'rust':rharness}[lang]
  path=out/f'{arm}.{dict(c="c",go="go",rust="rs")[lang]}';path.write_text(body)
  commands=([['cc','-O0','-fwrapv','-Werror'],['cc','-O2','-fwrapv','-Werror']]if lang=='c'else [[str(Path('/tmp/kuna-toolchains')/v/'go/bin/go'),'build']for v in ['go1.22.5','go1.27.3']]if lang=='go'else [['rustc','--edition=2021','-C','opt-level=0'],['rustc','--edition=2021','-C','opt-level=2']])
  for i,cmd in enumerate(commands):
   exe=out/f'{arm}-{lang}-{i}';build=cmd+['-o',str(exe),str(path)];p=subprocess.run(build,text=True,capture_output=True);(out/f'{exe.name}-build.log').write_text(p.stdout+p.stderr);item={'arm':arm,'language':lang,'command':build,'build_exit':p.returncode}
   if p.returncode==0:
    p=subprocess.run([str(exe)],text=True,capture_output=True);(out/f'{exe.name}-run.log').write_text(p.stderr);(out/f'{exe.name}.txt').write_text(p.stdout);item.update(run_exit=p.returncode,output_sha256=hashlib.sha256(p.stdout.encode()).hexdigest(),lines=len(p.stdout.splitlines()))
    if expected is None:assert arm=='source' and p.returncode==0;expected=p.stdout
    item['matches_source']=p.returncode==0 and p.stdout==expected
   else:item['matches_source']=False
   record['results'].append(item);(out/'record.json').write_text(json.dumps(record,indent=2)+'\n');print(arm,lang,i,item['matches_source'],flush=True)
   if arm in ['source','candidate']:assert item['matches_source'],(arm,lang,i,p.stderr[:4000])
print('COMPLETE CALLER MATRIX PASS',flush=True)
