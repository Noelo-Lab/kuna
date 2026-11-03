from pathlib import Path
import json
w=Path('/tmp/kuna-rust-store-conversions')
values=[0,1,2,127,128,255,256,32767,32768,65535,65536,2147483647,2147483648,4294967295,4294967296,9223372036854775807,9223372036854775808,18446744073709551615]
cases=[]
for bits,cs,cu in [(8,'signed char','unsigned char'),(16,'short','unsigned short'),(32,'int','unsigned int'),(64,'long','unsigned long')]:
 for kind in ['signed','unsigned','signed_plain','unsigned_plain','signed_xor','signed_constant','unsigned_constant']:
  name=kind+str(bits);plain='_plain' in kind;signed=kind.startswith('signed');ptype='s' if signed else 'u';argtype='u' if signed else 'i'
  cargs=['&p->'+ptype+str(bits) if plain else 'p'];rargs=['core::ptr::addr_of_mut!((*p).'+ptype+str(bits)+')' if plain else 'p']
  if '_constant' not in kind:
   if '_xor' in kind:argtype='i'
   ctype=cu if argtype=='u' else cs
   cargs.append('('+ctype+')a');rargs.append('a as '+argtype+str(bits))
   if '_xor' not in kind:cargs.append('('+ctype+')b');rargs.append('b as '+argtype+str(bits))
  cases.append((name,name+'('+','.join(cargs)+');',name+'('+','.join(rargs)+');'))
cases.append(('side_effect','side_effect(p,a,b);','side_effect(p,a,b);'))
initc='p->s8=-3;p->u8=5;p->s16=-7;p->u16=11;p->s32=-13;p->u32=17;p->s64=-19;p->u64=23;'
initr='(*p).s8=-3;(*p).u8=5;(*p).s16=-7;(*p).u16=11;(*p).s32=-13;(*p).u32=17;(*p).s64=-19;(*p).u64=23;'
c='''#include <stdio.h>
#include <string.h>
#define main fixture_main
#include "go_store_value.c"
#undef main
extern unsigned int pointer_calls,value_calls;
struct Envelope { unsigned long before[2]; struct Fields value; unsigned long after[2]; };
struct NestedEnvelope { unsigned long before[2]; struct Nested value; unsigned long after[2]; };
static void dump(const char *name,const void *value,size_t size){const unsigned char *p=value;printf("%s ",name);for(size_t i=0;i<size;i++)printf("%02x",p[i]);printf(" %u %u\\n",pointer_calls,value_calls);}
int main(void){unsigned long values[]={'''+','.join(str(v)+'UL'for v in values)+'''};for(unsigned i=0;i<18;i++){for(unsigned j=0;j<18;j++){unsigned long a=values[i],b=values[j];
'''
r='''#![allow(dead_code,unused_mut,unused_parens,unused_variables,non_snake_case)]
#[repr(C)] struct Fields{s8:i8,u8:u8,s16:i16,u16:u16,s32:i32,u32:u32,s64:i64,u64:u64}
#[repr(C)] struct Nested{values:[Fields;2]}
#[repr(C)] struct Envelope{before:[u64;2],value:Fields,after:[u64;2]}
#[repr(C)] struct NestedEnvelope{before:[u64;2],value:Nested,after:[u64;2]}
static mut pointer_calls:u32=0;
static mut value_calls:u32=0;
unsafe fn counted_destination(p:*mut Fields)->*mut Fields{pointer_calls+=1;p}
unsafe fn counted_value(a:u64,b:u64)->u64{value_calls+=1;a.wrapping_add(b)}
unsafe fn dump<T>(name:&str,value:*const T){print!("{} ",name);for byte in core::slice::from_raw_parts(value as *const u8,core::mem::size_of::<T>()){print!("{:02x}",byte);}let pc=pointer_calls;let vc=value_calls;println!(" {} {}",pc,vc);}
/* EMITTED_BODIES */
fn main(){unsafe{let values:[u64;18]=['''+','.join(str(v)for v in values)+'''];for a in values {for b in values {
'''
for name,ccall,rcall in cases:
 c+='{struct Envelope storage;memset(&storage,0xa5,sizeof storage);struct Fields *p=&storage.value;'+initc+'pointer_calls=0;value_calls=0;'+ccall+'dump("'+name+'",&storage,sizeof storage);}\n'
 r+='{let mut storage=core::mem::MaybeUninit::<Envelope>::uninit();core::ptr::write_bytes(storage.as_mut_ptr(),0xa5,1);let p=core::ptr::addr_of_mut!((*storage.as_mut_ptr()).value);'+initr+'pointer_calls=0;value_calls=0;'+rcall+'dump("'+name+'",storage.as_ptr());}\n'
c+='''{struct NestedEnvelope storage;memset(&storage,0xa5,sizeof storage);pointer_calls=0;value_calls=0;nested_store(&storage.value,a);dump("nested_store",&storage,sizeof storage);}
{struct {unsigned long before[2];double value;unsigned long after[2];}storage;memset(&storage,0xa5,sizeof storage);double input;memcpy(&input,&a,8);pointer_calls=0;value_calls=0;float_store(&storage.value,input);dump("float_store",&storage,sizeof storage);}
{struct {unsigned long before[2];long *value;unsigned long after[2];}storage;memset(&storage,0xa5,sizeof storage);long input=(long)a;pointer_store(&storage.value,&input);printf("pointer_store %u %lu %lu %lu %lu\\n",storage.value==&input,storage.before[0],storage.before[1],storage.after[0],storage.after[1]);}
{struct {unsigned long before[2];_Bool value;unsigned long after[2];}storage;memset(&storage,0xa5,sizeof storage);storage.value=0;pointer_calls=0;value_calls=0;bool_store(&storage.value,(a&1)!=0);dump("bool_store",&storage,sizeof storage);}
}}return 0;}
'''
r+='''{let mut storage=core::mem::MaybeUninit::<NestedEnvelope>::uninit();core::ptr::write_bytes(storage.as_mut_ptr(),0xa5,1);pointer_calls=0;value_calls=0;nested_store(core::ptr::addr_of_mut!((*storage.as_mut_ptr()).value),a);dump("nested_store",storage.as_ptr());}
{#[repr(C)]struct E{before:[u64;2],value:f64,after:[u64;2]}let mut storage=core::mem::MaybeUninit::<E>::uninit();core::ptr::write_bytes(storage.as_mut_ptr(),0xa5,1);pointer_calls=0;value_calls=0;float_store(core::ptr::addr_of_mut!((*storage.as_mut_ptr()).value),f64::from_bits(a));dump("float_store",storage.as_ptr());}
{#[repr(C)]struct E{before:[u64;2],value:*mut i64,after:[u64;2]}let mut storage=core::mem::MaybeUninit::<E>::uninit();core::ptr::write_bytes(storage.as_mut_ptr(),0xa5,1);let mut input=a as i64;pointer_store(core::ptr::addr_of_mut!((*storage.as_mut_ptr()).value),&mut input);let s=&*storage.as_ptr();println!("pointer_store {} {} {} {} {}",u8::from(s.value==core::ptr::addr_of_mut!(input)),s.before[0],s.before[1],s.after[0],s.after[1]);}
{#[repr(C)]struct E{before:[u64;2],value:bool,after:[u64;2]}let mut storage=core::mem::MaybeUninit::<E>::uninit();core::ptr::write_bytes(storage.as_mut_ptr(),0xa5,1);core::ptr::addr_of_mut!((*storage.as_mut_ptr()).value).write(false);pointer_calls=0;value_calls=0;bool_store(core::ptr::addr_of_mut!((*storage.as_mut_ptr()).value),a&1!=0);dump("bool_store",storage.as_ptr());}
}}}}
'''
(w/'harness.c').write_text(c);(w/'harness.rs').write_text(r)
(w/'cases.json').write_text(json.dumps([row[0]for row in cases]+['nested_store','float_store','pointer_store','bool_store'],indent=2)+'\n')
print('Prepared',len(cases)+4,'complete-body harness cases; not yet run')
