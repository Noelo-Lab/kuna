from common import *
fixture=w/'native';fixture.mkdir(exist_ok=True)
c=['#include <stdint.h>','#include <stddef.h>','#define NOINLINE __attribute__((noinline))'];decl=[];cases=[];ch=[];rh=[]
cintro=['#include <stdio.h>','#include <inttypes.h>','#include <string.h>','#define OMIT_MAIN','#include "pointer_index.c"','extern unsigned long trace;','static void bytes(const void*p,size_t n){const unsigned char*b=p;for(size_t k=0;k<n;k++)printf("%02x",b[k]);putchar(\'\\n\');}']
rintro=['#![allow(dead_code,unused_mut,unused_variables,unused_parens,non_snake_case)]','static mut TRACE:u64=0;','unsafe fn counted_index(i:i64)->i64{TRACE=TRACE*10+1;i}','unsafe fn counted_base(p:*mut i64)->*mut i64{TRACE=TRACE*10+2;p}','unsafe fn counted_value(v:i64)->i64{TRACE=TRACE*10+3;v}','fn bytes<T>(v:&[T]){let b=unsafe{std::slice::from_raw_parts(v.as_ptr() as *const u8,std::mem::size_of_val(v))};for x in b{print!("{:02x}",x);}println!();}','/* EMITTED_BODIES */']
values=['0','1','2','0x7f','0x80','0xff','0x7fff','0x8000','0xffff','0x7fffffff','0x80000000','0xffffffff','0x7fffffffffffffff','0x8000000000000000','0xffffffffffffffff','0x0123456789abcdef','0xfedcba9876543210','0xaaaaaaaa55555555']
for size in [1,2,4,8]:
 for signed in [True,False]:
  bits=size*8;ct=('int' if signed else 'uint')+str(bits)+'_t';rt=('i' if signed else 'u')+str(bits);kt=('int' if signed else 'uint')+str(size)
  for unsigned_index in [False,True]:
   it='uint64_t' if unsigned_index else 'int64_t';ir='u64' if unsigned_index else 'i64';ik='uint8' if unsigned_index else 'int8';suffix=rt+('_u'if unsigned_index else '_s')
   for kind in ['load','store','address']:
    name=kind+'_'+suffix;cases.append(name)
    ret=ct if kind=='load'else'void'if kind=='store'else ct+'*';kret=kt if kind=='load'else'void'if kind=='store'else kt+'*'
    params=f'{ct}*p,{it} i'+(f',{ct} value'if kind=='store'else'');kparams=f'{kt}*p,{ik} i'+(f',{kt} value'if kind=='store'else'')
    body='return p[i];'if kind=='load'else'p[i]=value;'if kind=='store'else f'return ({ct}*)((uintptr_t)p+(uint64_t)i*{size});'
    c.append(f'NOINLINE {ret} {name}({params}){{{body}}}');decl.append(f'prototype {name} {kret} {name}({kparams})')
    init=f'{ct} a[17];for(unsigned k=0;k<17;k++)a[k]=({ct})(UINT64_C(0x9e3779b97f4a7c15)*k+v);{ct}*p=a+8;'
    rinit=f'let mut a:[{rt};17]=[0;17];for k in 0..17{{a[k]=(0x9e3779b97f4a7c15u64.wrapping_mul(k as u64).wrapping_add(v)) as {rt};}}let p=a.as_mut_ptr().add(8);'
    ciarg='(i<0?-i:i)' if unsigned_index else 'i'
    riarg='i.unsigned_abs()' if unsigned_index else 'i'
    if kind=='load':
     ch.append('{'+init+f'printf("{name} %016" PRIx64 "\\n",(uint64_t){name}(p,({it}){ciarg}));'+'}')
     rh.append('{'+rinit+f'println!("{name} {{:016x}}",{name}(p,{riarg} as {ir}) as u64);'+'}')
    elif kind=='store':
     ch.append('{'+init+f'{name}(p,({it}){ciarg},({ct})v);printf("{name} ");bytes(a,sizeof(a));'+'}')
     rh.append('{'+rinit+f'{name}(p,{riarg} as {ir},v as {rt});print!("{name} ");bytes(&a);'+'}')
    else:
     ch.append('{'+init+f'printf("{name} %016" PRIx64 "\\n",(uint64_t)((uintptr_t){name}(p,({it})v)-(uintptr_t)p));'+'}')
     rh.append('{'+rinit+f'println!("{name} {{:016x}}",({name}(p,v as {ir}) as u64).wrapping_sub(p as u64));'+'}')
c+=['struct Record{int64_t before;int32_t value;uint32_t tag;};','int64_t counted_index(int64_t);int64_t*counted_base(int64_t*);int64_t counted_value(int64_t);']
rintro.insert(1,'#[repr(C)]#[derive(Clone,Copy)]struct Record{before:i64,value:i32,tag:u32}')
extras=[('field_load','int32_t','int4','struct Record*p,int64_t i','struct Record*p,int8 i','return p[i].value;'),('field_store','void','void','struct Record*p,int64_t i,int32_t value','struct Record*p,int8 i,int4 value','p[i].value=value;'),('callback_load','int64_t','int8','int64_t*p,int64_t i','int8*p,int8 i','i=counted_index(i);p=counted_base(p);return p[i];'),('callback_store','void','void','int64_t*p,int64_t i,int64_t value','int8*p,int8 i,int8 value','value=counted_value(value);i=counted_index(i);p=counted_base(p);p[i]=value;'),('callback_address','int64_t*','int8*','int64_t*p,int64_t i','int8*p,int8 i','i=counted_index(i);p=counted_base(p);return (int64_t*)((uintptr_t)p+(uint64_t)i*8);'),('zero_load','int32_t','int4','int32_t*p','int4*p','return *p;')]
for name,ret,kret,params,kparams,body in extras:
 c.append(f'NOINLINE {ret} {name}({params}){{{body}}}');decl.append(f'prototype {name} {kret} {name}({kparams})');cases.append(name)
record_init='struct Record a[17];for(unsigned k=0;k<17;k++)a[k]=(struct Record){(int64_t)(v+k),(int32_t)(v+k),0xdeadbeef+k};struct Record*p=a+8;'
rrecord_init='let mut a=[Record{before:0,value:0,tag:0};17];for k in 0..17{a[k]=Record{before:v.wrapping_add(k as u64) as i64,value:v.wrapping_add(k as u64) as i32,tag:0xdeadbeef+k as u32};}let p=a.as_mut_ptr().add(8);'
ch+=['{'+record_init+'printf("field_load %016" PRIx64 "\\n",(uint64_t)field_load(p,i));}', '{'+record_init+'field_store(p,i,(int32_t)v);printf("field_store ");bytes(a,sizeof(a));}']
rh+=['{'+rrecord_init+'println!("field_load {:016x}",field_load(p,i) as u64);}', '{'+rrecord_init+'field_store(p,i,v as i32);print!("field_store ");bytes(&a);}']
ci='int64_t a[17];for(unsigned k=0;k<17;k++)a[k]=(int64_t)(v+k);int64_t*p=a+8;trace=0;'
ri='let mut a=[0i64;17];for k in 0..17{a[k]=v.wrapping_add(k as u64) as i64;}let p=a.as_mut_ptr().add(8);TRACE=0;'
ch+=['{'+ci+'uint64_t result=(uint64_t)callback_load(p,i);printf("callback_load %016" PRIx64 " %lu\\n",result,trace);}', '{'+ci+'callback_store(p,i,(int64_t)v);printf("callback_store %lu ",trace);bytes(a,sizeof(a));}', '{'+ci+'uint64_t result=(uintptr_t)callback_address(p,(int64_t)v)-(uintptr_t)p;printf("callback_address %016" PRIx64 " %lu\\n",result,trace);}']
rh+=['{'+ri+'let result=callback_load(p,i) as u64;let trace=TRACE;println!("callback_load {:016x} {}",result,trace);}', '{'+ri+'callback_store(p,i,v as i64);let trace=TRACE;print!("callback_store {} ",trace);bytes(&a);}', '{'+ri+'let result=(callback_address(p,v as i64) as u64).wrapping_sub(p as u64);let trace=TRACE;println!("callback_address {:016x} {}",result,trace);}']
ch+=['{int32_t x=(int32_t)v;printf("zero_load %016" PRIx64 "\\n",(uint64_t)zero_load(&x));}'];rh+=['{let mut x=v as i32;println!("zero_load {:016x}",zero_load(&mut x) as u64);}']
context_c=(w/'index-context.c').read_text().splitlines()[1:-1]
c+=context_c
decl+=(w/'index-context.kuna').read_text().splitlines()
cases+=['shift_address','logical_address','signed_address']
ctail='int32_t anchor=0;for(unsigned n=0;n<64;n++){'+''.join('printf("'+name+' %016" PRIx64 "\\n",(uint64_t)((uintptr_t)'+name+'(&anchor,n)-(uintptr_t)&anchor));'for name in cases[-3:])+'}'
rtail='let mut anchor=0i32;for n in 0..64u32{'+''.join('println!("'+name+' {:016x}",('+name+'(&mut anchor,n) as u64).wrapping_sub(&mut anchor as *mut i32 as u64));'for name in cases[-3:])+'}'
decl+=['prototype counted_index int8 counted_index(int8 i)','prototype counted_base int8*counted_base(int8*p)','prototype counted_value int8 counted_value(int8 value)']
c+=['#ifndef OMIT_MAIN','int main(void){return 0;}','#endif']
(fixture/'pointer_index.c').write_text('\n'.join(c)+'\n');(fixture/'pointer_index.kuna').write_text('\n'.join(decl)+'\n');(fixture/'cases.json').write_text(json.dumps(cases,indent=2)+'\n')
(fixture/'pointer_index_helpers.c').write_text('#include <stdint.h>\nunsigned long trace=0;\n__attribute__((noinline)) int64_t counted_index(int64_t i){trace=trace*10+1;return i;}\n__attribute__((noinline)) int64_t*counted_base(int64_t*p){trace=trace*10+2;return p;}\n__attribute__((noinline)) int64_t counted_value(int64_t value){trace=trace*10+3;return value;}\n')
(fixture/'harness.c').write_text('\n'.join(cintro)+'\nint main(void){uint64_t values[]={'+','.join('UINT64_C('+v+')'for v in values)+'};for(unsigned k=0;k<18;k++){uint64_t v=values[k];for(int64_t i=-7;i<=7;i++){\n'+'\n'.join(ch)+'\n}}'+ctail+'return 0;}\n')
(fixture/'harness.rs').write_text('\n'.join(rintro)+'\nfn main(){let values:[u64;18]=['+','.join(values)+'];unsafe{for v in values{for i in -7i64..=7{\n'+'\n'.join(rh)+'\n}}'+rtail+'}}\n')
cmd=['cc','-O2','-g','-fno-inline','-fno-pie','-no-pie',str(fixture/'pointer_index.c'),str(fixture/'pointer_index_helpers.c'),'-o',str(fixture/'pointer_index_x86_64')];subprocess.run(cmd,check=True,capture_output=True)
for opt in ['0','2']:
 cmd=['cc','-O'+opt,'-fwrapv','-fno-inline','-std=c11','-Wall','-Wextra','-Werror',str(fixture/'harness.c'),str(fixture/'pointer_index_helpers.c'),'-o',str(fixture/('reference-'+opt))];p=subprocess.run(cmd,capture_output=True);(fixture/('reference-'+opt+'-build.log')).write_bytes(p.stdout+p.stderr);assert p.returncode==0,p.stderr.decode();p=subprocess.run([str(fixture/('reference-'+opt))],capture_output=True,check=True);(fixture/('reference-'+opt+'.stdout')).write_bytes(p.stdout);assert len(p.stdout.splitlines())==54*18*15+3*64
assert(fixture/'reference-0.stdout').read_bytes()==(fixture/'reference-2.stdout').read_bytes()
print('Generated',len(cases),'complete-body cases;',54*18*15+3*64,'comparisons/profile;C O0/O2 agree',flush=True)
