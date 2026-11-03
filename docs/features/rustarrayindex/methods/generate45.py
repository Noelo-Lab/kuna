from pathlib import Path
import json,subprocess
w=Path('/tmp/kuna-rust-array-index');f=w/'native45';f.mkdir(exist_ok=True)
types=[(f'{sign}{size*8}',('int'if sign=='i'else'uint')+str(size*8)+'_t',('int'if sign=='i'else'uint')+str(size))for size in [1,2,4,8]for sign in ['i','u']]
c=['#include <stdint.h>','#define NOINLINE __attribute__((noinline))','struct ArrayFields {uint64_t before;'+''.join(f'{ct} {rt}[9];'for rt,ct,kt in types)+'uint64_t after;};'];rust_struct='#[repr(C)]struct ArrayFields{before:u64,'+''.join(f'{rt}:[{rt};9],'for rt,ct,kt in types)+'after:u64}'
ch=[];rh=[];decl=[];cases=[];values=['0','1','2','0x7f','0x80','0xff','0x7fff','0x8000','0xffff','0x7fffffff','0x80000000','0xffffffff','0x7fffffffffffffff','0x8000000000000000','0xffffffffffffffff','0x0123456789abcdef','0xfedcba9876543210','0xaaaaaaaa55555555']
ci='struct ArrayFields a;memset(&a,0xa5,sizeof(a));struct ArrayFields*p=&a;'+''.join(f'for(unsigned k=0;k<9;k++)p->{rt}[k]=({ct})(UINT64_C(0x9e3779b97f4a7c15)*k+v);'for rt,ct,kt in types)
ri='let mut buffer=Buffer([0xa5;std::mem::size_of::<ArrayFields>()]);let p=buffer.0.as_mut_ptr() as *mut ArrayFields;'+''.join(f'for k in 0..9{{(*p).{rt}[k]=0x9e3779b97f4a7c15u64.wrapping_mul(k as u64).wrapping_add(v) as {rt};}}'for rt,ct,kt in types)
for rt,ct,kt in types:
 for unsigned in [False,True]:
  it='uint64_t'if unsigned else'int64_t';ir='u64'if unsigned else'i64';ik='uint8'if unsigned else'int8'
  for kind in ['load','store']:
   name=kind+'_'+rt+('_u'if unsigned else'_s');cases.append(name);ret=ct if kind=='load'else'void';kr=kt if kind=='load'else'void';params=f'struct ArrayFields*p,{it} i'+(f',{ct} value'if kind=='store'else'');kparams=f'struct ArrayFields*p,{ik} i'+(f',{kt} value'if kind=='store'else'');body=f'return p->{rt}[i];'if kind=='load'else f'p->{rt}[i]=value;';c.append(f'NOINLINE {ret} {name}({params}){{{body}}}');decl.append(f'prototype {name} {kr} {name}({kparams})')
   if kind=='load':ch.append('{'+ci+f'printf("{name} %016" PRIx64 "\\n",(uint64_t){name}(p,({it})i));'+'}');rh.append('{'+ri+f'println!("{name} {{:016x}}",{name}(p,i as {ir}) as u64);'+'}')
   else:ch.append('{'+ci+f'{name}(p,({it})i,({ct})v);printf("{name} ");bytes(&a,sizeof(a));'+'}');rh.append('{'+ri+f'{name}(p,i as {ir},v as {rt});print!("{name} ");bytes(&buffer.0);'+'}')
for name,ret,kr,body in [('constant_load','int32_t','int4','return p->i32[3];'),('zero_load','uint64_t','uint8','return p->u64[0];')]:
 cases.append(name);c.append(f'NOINLINE {ret} {name}(struct ArrayFields*p){{{body}}}');decl.append(f'prototype {name} {kr} {name}(struct ArrayFields*p)');ch.append('{'+ci+f'printf("{name} %016" PRIx64 "\\n",(uint64_t){name}(p));'+'}');rh.append('{'+ri+f'println!("{name} {{:016x}}",{name}(p) as u64);'+'}')
for unsigned in [False,True]:
 it='uint64_t'if unsigned else'int64_t';ir='u64'if unsigned else'i64';ik='uint8'if unsigned else'int8';name='array_address_u'if unsigned else'array_address_s'
 cases.append(name);c.append(f'NOINLINE int32_t*{name}(struct ArrayFields*p,{it} i){{return &p->i32[i];}}');decl.append(f'prototype {name} int4*{name}(struct ArrayFields*p,{ik} i)')
 ch.append('{'+ci+f'printf("{name} %016" PRIx64 "\\n",(uint64_t)((uintptr_t){name}(p,({it})i)-(uintptr_t)p));'+'}')
 rh.append('{'+ri+f'println!("{name} {{:016x}}",({name}(p,i as {ir})as usize).wrapping_sub(p as usize)as u64);'+'}')
c+=['int64_t counted_index(int64_t);struct ArrayFields*counted_base(struct ArrayFields*);int32_t counted_value(int32_t);','NOINLINE int32_t callback_load(struct ArrayFields*p,int64_t i){i=counted_index(i);p=counted_base(p);return p->i32[i];}','NOINLINE void callback_store(struct ArrayFields*p,int64_t i,int32_t value){value=counted_value(value);i=counted_index(i);p=counted_base(p);p->i32[i]=value;}'];decl+=['prototype callback_load int4 callback_load(struct ArrayFields*p,int8 i)','prototype callback_store void callback_store(struct ArrayFields*p,int8 i,int4 value)'];cases+=['callback_load','callback_store']
ch+=['{'+ci+'trace=0;uint64_t result=(uint64_t)callback_load(p,i);printf("callback_load %016" PRIx64 " %lu\\n",result,trace);}', '{'+ci+'trace=0;callback_store(p,i,(int32_t)v);printf("callback_store %lu ",trace);bytes(&a,sizeof(a));}']
rh+=['{'+ri+'TRACE=0;let result=callback_load(p,i as i64)as u64;let trace=TRACE;println!("callback_load {:016x} {}",result,trace);}', '{'+ri+'TRACE=0;callback_store(p,i as i64,v as i32);let trace=TRACE;print!("callback_store {} ",trace);bytes(&buffer.0);}']
nch=[];nrh=[]
for unsigned in [False,True]:
 it='uint64_t'if unsigned else'int64_t';ik='uint8'if unsigned else'int8';ir='u64'if unsigned else'i64'
 for kind in ['load','store']:
  name='nested_'+kind+('_u'if unsigned else'_s');cases.append(name);ret='int32_t'if kind=='load'else'void';kr='int4'if kind=='load'else'void';params=f'int32_t(*p)[9],int64_t i,{it} j'+(',int32_t value'if kind=='store'else'');kp=f'int4(*p)[9],int8 i,{ik} j'+(',int4 value'if kind=='store'else'');body='return p[i][j];'if kind=='load'else'p[i][j]=value;';c.append(f'NOINLINE {ret} {name}({params}){{{body}}}');decl.append(f'prototype {name} {kr} {name}({kp})')
  init='int32_t a[5][9];for(unsigned row=0;row<5;row++)for(unsigned col=0;col<9;col++)a[row][col]=(int32_t)(v+row*31+col);int32_t(*p)[9]=a+2;'
  rinit='let mut a=[[0i32;9];5];for row in 0..5{for col in 0..9{a[row][col]=v.wrapping_add((row*31+col)as u64)as i32;}}let p=a.as_mut_ptr().add(2);'
  if kind=='load':nch.append('{'+init+f'printf("{name} %016" PRIx64 "\\n",(uint64_t){name}(p,i,({it})j));'+'}');nrh.append('{'+rinit+f'println!("{name} {{:016x}}",{name}(p,i,j as {ir})as u64);'+'}')
  else:nch.append('{'+init+f'{name}(p,i,({it})j,(int32_t)v);printf("{name} ");bytes(a,sizeof(a));'+'}');nrh.append('{'+rinit+f'{name}(p,i,j as {ir},v as i32);print!("{name} ");bytes(std::slice::from_raw_parts(a.as_ptr()as *const u8,180));'+'}')
context_c=[];context_r=[]
for name,index in [('right_context','UINT64_C(7)>>shift'),('signed_context','(INT64_C(-8)>>shift)+8'),('left_context','UINT64_C(1)<<(shift&3)')]:
 cases.append(name);c.append(f'NOINLINE int32_t {name}(struct ArrayFields*p,uint64_t shift){{return p->i32[{index}];}}');decl.append(f'prototype {name} int4 {name}(struct ArrayFields*p,uint8 shift)')
 context_c.append('{'+ci+f'printf("{name} %016" PRIx64 "\\n",(uint64_t){name}(p,shift));'+'}')
 context_r.append('{'+ri+f'println!("{name} {{:016x}}",{name}(p,shift)as u64);'+'}')
decl+=['prototype counted_index int8 counted_index(int8 i)','prototype counted_base struct ArrayFields*counted_base(struct ArrayFields*p)','prototype counted_value int4 counted_value(int4 value)'];c+=['#ifndef OMIT_MAIN','int main(void){return 0;}','#endif'];(f/'array_index.c').write_text('\n'.join(c)+'\n');(f/'array_index.kuna').write_text('\n'.join(decl)+'\n');(f/'cases.json').write_text(json.dumps(cases,indent=2)+'\n')
(f/'array_index_helpers.c').write_text('#include <stdint.h>\nstruct ArrayFields;unsigned long trace=0;\n__attribute__((noinline)) int64_t counted_index(int64_t i){trace=trace*10+1;return i;}\n__attribute__((noinline)) struct ArrayFields*counted_base(struct ArrayFields*p){trace=trace*10+2;return p;}\n__attribute__((noinline)) int32_t counted_value(int32_t value){trace=trace*10+3;return value;}\n')
(f/'harness.c').write_text('#include <stdio.h>\n#include <inttypes.h>\n#include <string.h>\n#define OMIT_MAIN\n#include "array_index.c"\nextern unsigned long trace;\nstatic void bytes(const void*p,size_t n){const unsigned char*b=p;for(size_t k=0;k<n;k++)printf("%02x",b[k]);putchar(\'\\n\');}\nint main(void){uint64_t values[]={'+','.join('UINT64_C('+v+')'for v in values)+'};for(unsigned n=0;n<18;n++){uint64_t v=values[n];for(int64_t i=0;i<9;i++){\n'+'\n'.join(ch)+'\n}for(int64_t i=-1;i<=1;i++){for(int64_t j=0;j<9;j++){\n'+'\n'.join(nch)+'\n}}for(uint64_t shift=0;shift<64;shift++){\n'+'\n'.join(context_c)+'\n}}return 0;}\n')
(f/'harness.rs').write_text('#![allow(dead_code,unused_mut,unused_variables,unused_parens,non_snake_case)]\n'+rust_struct+'\n#[repr(align(8))]struct Buffer([u8;std::mem::size_of::<ArrayFields>()]);\nstatic mut TRACE:u64=0;\nunsafe fn counted_index(i:i64)->i64{TRACE=TRACE*10+1;i}\nunsafe fn counted_base(p:*mut ArrayFields)->*mut ArrayFields{TRACE=TRACE*10+2;p}\nunsafe fn counted_value(v:i32)->i32{TRACE=TRACE*10+3;v}\nfn bytes(v:&[u8]){for x in v{print!("{:02x}",x);}println!();}\n/* EMITTED_BODIES */\nfn main(){unsafe{for v in ['+','.join(v+'u64'for v in values)+']{for i in 0..9i64{\n'+'\n'.join(rh)+'\n}for i in -1i64..=1{for j in 0..9i64{\n'+'\n'.join(nrh)+'\n}}for shift in 0u64..64{\n'+'\n'.join(context_r)+'\n}}}}\n')
print('Generated',len(cases),'array bodies; C/native comparison pending')
