#![allow(dead_code,unused_mut,unused_parens,unused_variables,non_snake_case)]
#[repr(C)] struct Fields{s8:i8,u8:u8,s16:i16,u16:u16,s32:i32,u32:u32,s64:i64,u64:u64}
#[repr(C)] struct Nested{values:[Fields;2]}
#[repr(C)] struct Envelope{before:[u64;2],value:Fields,after:[u64;2]}
#[repr(C)] struct NestedEnvelope{before:[u64;2],value:Nested,after:[u64;2]}
static mut pointer_calls:u32=0;
static mut value_calls:u32=0;
unsafe fn counted_destination(p:*mut Fields)->*mut Fields{pointer_calls+=1;p}
unsafe fn counted_value(a:u64,b:u64)->u64{value_calls+=1;a.wrapping_add(b)}
unsafe fn dump<T>(name:&str,value:*const T){print!("{} ",name);for byte in core::slice::from_raw_parts(value as *const u8,core::mem::size_of::<T>()){print!("{:02x}",byte);}let pc=pointer_calls;let vc=value_calls;println!(" {} {}",pc,vc);}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn signed8(mut p: *mut Fields, mut a: u8, mut b: u8)
{
  (*p).s8 = b + a;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn unsigned8(mut p: *mut Fields, mut a: i8, mut b: i8)
{
  (*p).u8 = b + a;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn signed_plain8(mut p: *mut i8, mut a: u8, mut b: u8)
{
  *p = b + a;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn unsigned_plain8(mut p: *mut u8, mut a: i8, mut b: i8)
{
  *p = b + a;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn signed_xor8(mut p: *mut Fields, mut a: i8)
{
  (*p).s8 = a as u8 ^ 1;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn signed_constant8(mut p: *mut Fields)
{
  (*p).s8 = -1;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn unsigned_constant8(mut p: *mut Fields)
{
  (*p).u8 = 0xff;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn signed16(mut p: *mut Fields, mut a: u16, mut b: u16)
{
  (*p).s16 = b + a;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn unsigned16(mut p: *mut Fields, mut a: i16, mut b: i16)
{
  (*p).u16 = b + a;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn signed_plain16(mut p: *mut i16, mut a: u16, mut b: u16)
{
  *p = b + a;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn unsigned_plain16(mut p: *mut u16, mut a: i16, mut b: i16)
{
  *p = b + a;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn signed_xor16(mut p: *mut Fields, mut a: i16)
{
  (*p).s16 = a as u16 ^ 1;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn signed_constant16(mut p: *mut Fields)
{
  (*p).s16 = -1;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn unsigned_constant16(mut p: *mut Fields)
{
  (*p).u16 = 0xffff;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn signed32(mut p: *mut Fields, mut a: u32, mut b: u32)
{
  (*p).s32 = a + b;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn unsigned32(mut p: *mut Fields, mut a: i32, mut b: i32)
{
  (*p).u32 = a + b;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn signed_plain32(mut p: *mut i32, mut a: u32, mut b: u32)
{
  *p = a + b;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn unsigned_plain32(mut p: *mut u32, mut a: i32, mut b: i32)
{
  *p = a + b;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn signed_xor32(mut p: *mut Fields, mut a: i32)
{
  (*p).s32 = a as u32 ^ 1;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn signed_constant32(mut p: *mut Fields)
{
  (*p).s32 = -1;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn unsigned_constant32(mut p: *mut Fields)
{
  (*p).u32 = 0xffffffff;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn signed64(mut p: *mut Fields, mut a: u64, mut b: u64)
{
  (*p).s64 = a + b;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn unsigned64(mut p: *mut Fields, mut a: i64, mut b: i64)
{
  (*p).u64 = a + b;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn signed_plain64(mut p: *mut i64, mut a: u64, mut b: u64)
{
  *p = a + b;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn unsigned_plain64(mut p: *mut u64, mut a: i64, mut b: i64)
{
  *p = a + b;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn signed_xor64(mut p: *mut Fields, mut a: i64)
{
  (*p).s64 = a as u64 ^ 1;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn signed_constant64(mut p: *mut Fields)
{
  (*p).s64 = -1;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn unsigned_constant64(mut p: *mut Fields)
{
  (*p).u64 = 0xffffffffffffffff;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn side_effect(mut p: *mut Fields, mut a: u64, mut b: u64)
{
  let mut v1: u64; // rax | rbx
  
  v1 = counted_value(a,b);
  (*counted_destination(p)).s64 = v1;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn nested_store(mut p: *mut Nested, mut a: u64)
{
  (*p).values[1].s32 = a as i32;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn float_store(mut p: *mut f64, mut a: f64)
{
  *p = a;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn pointer_store(mut p: *mut *mut i64, mut a: *mut i64)
{
  *p = a;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn bool_store(mut p: *mut bool, mut a: bool)
{
  *p = a;
}
fn main(){unsafe{let values:[u64;18]=[0,1,2,127,128,255,256,32767,32768,65535,65536,2147483647,2147483648,4294967295,4294967296,9223372036854775807,9223372036854775808,18446744073709551615];for a in values {for b in values {
{let mut storage=core::mem::MaybeUninit::<Envelope>::uninit();core::ptr::write_bytes(storage.as_mut_ptr(),0xa5,1);let p=core::ptr::addr_of_mut!((*storage.as_mut_ptr()).value);(*p).s8=-3;(*p).u8=5;(*p).s16=-7;(*p).u16=11;(*p).s32=-13;(*p).u32=17;(*p).s64=-19;(*p).u64=23;pointer_calls=0;value_calls=0;signed8(p,a as u8,b as u8);dump("signed8",storage.as_ptr());}
{let mut storage=core::mem::MaybeUninit::<Envelope>::uninit();core::ptr::write_bytes(storage.as_mut_ptr(),0xa5,1);let p=core::ptr::addr_of_mut!((*storage.as_mut_ptr()).value);(*p).s8=-3;(*p).u8=5;(*p).s16=-7;(*p).u16=11;(*p).s32=-13;(*p).u32=17;(*p).s64=-19;(*p).u64=23;pointer_calls=0;value_calls=0;unsigned8(p,a as i8,b as i8);dump("unsigned8",storage.as_ptr());}
{let mut storage=core::mem::MaybeUninit::<Envelope>::uninit();core::ptr::write_bytes(storage.as_mut_ptr(),0xa5,1);let p=core::ptr::addr_of_mut!((*storage.as_mut_ptr()).value);(*p).s8=-3;(*p).u8=5;(*p).s16=-7;(*p).u16=11;(*p).s32=-13;(*p).u32=17;(*p).s64=-19;(*p).u64=23;pointer_calls=0;value_calls=0;signed_plain8(core::ptr::addr_of_mut!((*p).s8),a as u8,b as u8);dump("signed_plain8",storage.as_ptr());}
{let mut storage=core::mem::MaybeUninit::<Envelope>::uninit();core::ptr::write_bytes(storage.as_mut_ptr(),0xa5,1);let p=core::ptr::addr_of_mut!((*storage.as_mut_ptr()).value);(*p).s8=-3;(*p).u8=5;(*p).s16=-7;(*p).u16=11;(*p).s32=-13;(*p).u32=17;(*p).s64=-19;(*p).u64=23;pointer_calls=0;value_calls=0;unsigned_plain8(core::ptr::addr_of_mut!((*p).u8),a as i8,b as i8);dump("unsigned_plain8",storage.as_ptr());}
{let mut storage=core::mem::MaybeUninit::<Envelope>::uninit();core::ptr::write_bytes(storage.as_mut_ptr(),0xa5,1);let p=core::ptr::addr_of_mut!((*storage.as_mut_ptr()).value);(*p).s8=-3;(*p).u8=5;(*p).s16=-7;(*p).u16=11;(*p).s32=-13;(*p).u32=17;(*p).s64=-19;(*p).u64=23;pointer_calls=0;value_calls=0;signed_xor8(p,a as i8);dump("signed_xor8",storage.as_ptr());}
{let mut storage=core::mem::MaybeUninit::<Envelope>::uninit();core::ptr::write_bytes(storage.as_mut_ptr(),0xa5,1);let p=core::ptr::addr_of_mut!((*storage.as_mut_ptr()).value);(*p).s8=-3;(*p).u8=5;(*p).s16=-7;(*p).u16=11;(*p).s32=-13;(*p).u32=17;(*p).s64=-19;(*p).u64=23;pointer_calls=0;value_calls=0;signed_constant8(p);dump("signed_constant8",storage.as_ptr());}
{let mut storage=core::mem::MaybeUninit::<Envelope>::uninit();core::ptr::write_bytes(storage.as_mut_ptr(),0xa5,1);let p=core::ptr::addr_of_mut!((*storage.as_mut_ptr()).value);(*p).s8=-3;(*p).u8=5;(*p).s16=-7;(*p).u16=11;(*p).s32=-13;(*p).u32=17;(*p).s64=-19;(*p).u64=23;pointer_calls=0;value_calls=0;unsigned_constant8(p);dump("unsigned_constant8",storage.as_ptr());}
{let mut storage=core::mem::MaybeUninit::<Envelope>::uninit();core::ptr::write_bytes(storage.as_mut_ptr(),0xa5,1);let p=core::ptr::addr_of_mut!((*storage.as_mut_ptr()).value);(*p).s8=-3;(*p).u8=5;(*p).s16=-7;(*p).u16=11;(*p).s32=-13;(*p).u32=17;(*p).s64=-19;(*p).u64=23;pointer_calls=0;value_calls=0;signed16(p,a as u16,b as u16);dump("signed16",storage.as_ptr());}
{let mut storage=core::mem::MaybeUninit::<Envelope>::uninit();core::ptr::write_bytes(storage.as_mut_ptr(),0xa5,1);let p=core::ptr::addr_of_mut!((*storage.as_mut_ptr()).value);(*p).s8=-3;(*p).u8=5;(*p).s16=-7;(*p).u16=11;(*p).s32=-13;(*p).u32=17;(*p).s64=-19;(*p).u64=23;pointer_calls=0;value_calls=0;unsigned16(p,a as i16,b as i16);dump("unsigned16",storage.as_ptr());}
{let mut storage=core::mem::MaybeUninit::<Envelope>::uninit();core::ptr::write_bytes(storage.as_mut_ptr(),0xa5,1);let p=core::ptr::addr_of_mut!((*storage.as_mut_ptr()).value);(*p).s8=-3;(*p).u8=5;(*p).s16=-7;(*p).u16=11;(*p).s32=-13;(*p).u32=17;(*p).s64=-19;(*p).u64=23;pointer_calls=0;value_calls=0;signed_plain16(core::ptr::addr_of_mut!((*p).s16),a as u16,b as u16);dump("signed_plain16",storage.as_ptr());}
{let mut storage=core::mem::MaybeUninit::<Envelope>::uninit();core::ptr::write_bytes(storage.as_mut_ptr(),0xa5,1);let p=core::ptr::addr_of_mut!((*storage.as_mut_ptr()).value);(*p).s8=-3;(*p).u8=5;(*p).s16=-7;(*p).u16=11;(*p).s32=-13;(*p).u32=17;(*p).s64=-19;(*p).u64=23;pointer_calls=0;value_calls=0;unsigned_plain16(core::ptr::addr_of_mut!((*p).u16),a as i16,b as i16);dump("unsigned_plain16",storage.as_ptr());}
{let mut storage=core::mem::MaybeUninit::<Envelope>::uninit();core::ptr::write_bytes(storage.as_mut_ptr(),0xa5,1);let p=core::ptr::addr_of_mut!((*storage.as_mut_ptr()).value);(*p).s8=-3;(*p).u8=5;(*p).s16=-7;(*p).u16=11;(*p).s32=-13;(*p).u32=17;(*p).s64=-19;(*p).u64=23;pointer_calls=0;value_calls=0;signed_xor16(p,a as i16);dump("signed_xor16",storage.as_ptr());}
{let mut storage=core::mem::MaybeUninit::<Envelope>::uninit();core::ptr::write_bytes(storage.as_mut_ptr(),0xa5,1);let p=core::ptr::addr_of_mut!((*storage.as_mut_ptr()).value);(*p).s8=-3;(*p).u8=5;(*p).s16=-7;(*p).u16=11;(*p).s32=-13;(*p).u32=17;(*p).s64=-19;(*p).u64=23;pointer_calls=0;value_calls=0;signed_constant16(p);dump("signed_constant16",storage.as_ptr());}
{let mut storage=core::mem::MaybeUninit::<Envelope>::uninit();core::ptr::write_bytes(storage.as_mut_ptr(),0xa5,1);let p=core::ptr::addr_of_mut!((*storage.as_mut_ptr()).value);(*p).s8=-3;(*p).u8=5;(*p).s16=-7;(*p).u16=11;(*p).s32=-13;(*p).u32=17;(*p).s64=-19;(*p).u64=23;pointer_calls=0;value_calls=0;unsigned_constant16(p);dump("unsigned_constant16",storage.as_ptr());}
{let mut storage=core::mem::MaybeUninit::<Envelope>::uninit();core::ptr::write_bytes(storage.as_mut_ptr(),0xa5,1);let p=core::ptr::addr_of_mut!((*storage.as_mut_ptr()).value);(*p).s8=-3;(*p).u8=5;(*p).s16=-7;(*p).u16=11;(*p).s32=-13;(*p).u32=17;(*p).s64=-19;(*p).u64=23;pointer_calls=0;value_calls=0;signed32(p,a as u32,b as u32);dump("signed32",storage.as_ptr());}
{let mut storage=core::mem::MaybeUninit::<Envelope>::uninit();core::ptr::write_bytes(storage.as_mut_ptr(),0xa5,1);let p=core::ptr::addr_of_mut!((*storage.as_mut_ptr()).value);(*p).s8=-3;(*p).u8=5;(*p).s16=-7;(*p).u16=11;(*p).s32=-13;(*p).u32=17;(*p).s64=-19;(*p).u64=23;pointer_calls=0;value_calls=0;unsigned32(p,a as i32,b as i32);dump("unsigned32",storage.as_ptr());}
{let mut storage=core::mem::MaybeUninit::<Envelope>::uninit();core::ptr::write_bytes(storage.as_mut_ptr(),0xa5,1);let p=core::ptr::addr_of_mut!((*storage.as_mut_ptr()).value);(*p).s8=-3;(*p).u8=5;(*p).s16=-7;(*p).u16=11;(*p).s32=-13;(*p).u32=17;(*p).s64=-19;(*p).u64=23;pointer_calls=0;value_calls=0;signed_plain32(core::ptr::addr_of_mut!((*p).s32),a as u32,b as u32);dump("signed_plain32",storage.as_ptr());}
{let mut storage=core::mem::MaybeUninit::<Envelope>::uninit();core::ptr::write_bytes(storage.as_mut_ptr(),0xa5,1);let p=core::ptr::addr_of_mut!((*storage.as_mut_ptr()).value);(*p).s8=-3;(*p).u8=5;(*p).s16=-7;(*p).u16=11;(*p).s32=-13;(*p).u32=17;(*p).s64=-19;(*p).u64=23;pointer_calls=0;value_calls=0;unsigned_plain32(core::ptr::addr_of_mut!((*p).u32),a as i32,b as i32);dump("unsigned_plain32",storage.as_ptr());}
{let mut storage=core::mem::MaybeUninit::<Envelope>::uninit();core::ptr::write_bytes(storage.as_mut_ptr(),0xa5,1);let p=core::ptr::addr_of_mut!((*storage.as_mut_ptr()).value);(*p).s8=-3;(*p).u8=5;(*p).s16=-7;(*p).u16=11;(*p).s32=-13;(*p).u32=17;(*p).s64=-19;(*p).u64=23;pointer_calls=0;value_calls=0;signed_xor32(p,a as i32);dump("signed_xor32",storage.as_ptr());}
{let mut storage=core::mem::MaybeUninit::<Envelope>::uninit();core::ptr::write_bytes(storage.as_mut_ptr(),0xa5,1);let p=core::ptr::addr_of_mut!((*storage.as_mut_ptr()).value);(*p).s8=-3;(*p).u8=5;(*p).s16=-7;(*p).u16=11;(*p).s32=-13;(*p).u32=17;(*p).s64=-19;(*p).u64=23;pointer_calls=0;value_calls=0;signed_constant32(p);dump("signed_constant32",storage.as_ptr());}
{let mut storage=core::mem::MaybeUninit::<Envelope>::uninit();core::ptr::write_bytes(storage.as_mut_ptr(),0xa5,1);let p=core::ptr::addr_of_mut!((*storage.as_mut_ptr()).value);(*p).s8=-3;(*p).u8=5;(*p).s16=-7;(*p).u16=11;(*p).s32=-13;(*p).u32=17;(*p).s64=-19;(*p).u64=23;pointer_calls=0;value_calls=0;unsigned_constant32(p);dump("unsigned_constant32",storage.as_ptr());}
{let mut storage=core::mem::MaybeUninit::<Envelope>::uninit();core::ptr::write_bytes(storage.as_mut_ptr(),0xa5,1);let p=core::ptr::addr_of_mut!((*storage.as_mut_ptr()).value);(*p).s8=-3;(*p).u8=5;(*p).s16=-7;(*p).u16=11;(*p).s32=-13;(*p).u32=17;(*p).s64=-19;(*p).u64=23;pointer_calls=0;value_calls=0;signed64(p,a as u64,b as u64);dump("signed64",storage.as_ptr());}
{let mut storage=core::mem::MaybeUninit::<Envelope>::uninit();core::ptr::write_bytes(storage.as_mut_ptr(),0xa5,1);let p=core::ptr::addr_of_mut!((*storage.as_mut_ptr()).value);(*p).s8=-3;(*p).u8=5;(*p).s16=-7;(*p).u16=11;(*p).s32=-13;(*p).u32=17;(*p).s64=-19;(*p).u64=23;pointer_calls=0;value_calls=0;unsigned64(p,a as i64,b as i64);dump("unsigned64",storage.as_ptr());}
{let mut storage=core::mem::MaybeUninit::<Envelope>::uninit();core::ptr::write_bytes(storage.as_mut_ptr(),0xa5,1);let p=core::ptr::addr_of_mut!((*storage.as_mut_ptr()).value);(*p).s8=-3;(*p).u8=5;(*p).s16=-7;(*p).u16=11;(*p).s32=-13;(*p).u32=17;(*p).s64=-19;(*p).u64=23;pointer_calls=0;value_calls=0;signed_plain64(core::ptr::addr_of_mut!((*p).s64),a as u64,b as u64);dump("signed_plain64",storage.as_ptr());}
{let mut storage=core::mem::MaybeUninit::<Envelope>::uninit();core::ptr::write_bytes(storage.as_mut_ptr(),0xa5,1);let p=core::ptr::addr_of_mut!((*storage.as_mut_ptr()).value);(*p).s8=-3;(*p).u8=5;(*p).s16=-7;(*p).u16=11;(*p).s32=-13;(*p).u32=17;(*p).s64=-19;(*p).u64=23;pointer_calls=0;value_calls=0;unsigned_plain64(core::ptr::addr_of_mut!((*p).u64),a as i64,b as i64);dump("unsigned_plain64",storage.as_ptr());}
{let mut storage=core::mem::MaybeUninit::<Envelope>::uninit();core::ptr::write_bytes(storage.as_mut_ptr(),0xa5,1);let p=core::ptr::addr_of_mut!((*storage.as_mut_ptr()).value);(*p).s8=-3;(*p).u8=5;(*p).s16=-7;(*p).u16=11;(*p).s32=-13;(*p).u32=17;(*p).s64=-19;(*p).u64=23;pointer_calls=0;value_calls=0;signed_xor64(p,a as i64);dump("signed_xor64",storage.as_ptr());}
{let mut storage=core::mem::MaybeUninit::<Envelope>::uninit();core::ptr::write_bytes(storage.as_mut_ptr(),0xa5,1);let p=core::ptr::addr_of_mut!((*storage.as_mut_ptr()).value);(*p).s8=-3;(*p).u8=5;(*p).s16=-7;(*p).u16=11;(*p).s32=-13;(*p).u32=17;(*p).s64=-19;(*p).u64=23;pointer_calls=0;value_calls=0;signed_constant64(p);dump("signed_constant64",storage.as_ptr());}
{let mut storage=core::mem::MaybeUninit::<Envelope>::uninit();core::ptr::write_bytes(storage.as_mut_ptr(),0xa5,1);let p=core::ptr::addr_of_mut!((*storage.as_mut_ptr()).value);(*p).s8=-3;(*p).u8=5;(*p).s16=-7;(*p).u16=11;(*p).s32=-13;(*p).u32=17;(*p).s64=-19;(*p).u64=23;pointer_calls=0;value_calls=0;unsigned_constant64(p);dump("unsigned_constant64",storage.as_ptr());}
{let mut storage=core::mem::MaybeUninit::<Envelope>::uninit();core::ptr::write_bytes(storage.as_mut_ptr(),0xa5,1);let p=core::ptr::addr_of_mut!((*storage.as_mut_ptr()).value);(*p).s8=-3;(*p).u8=5;(*p).s16=-7;(*p).u16=11;(*p).s32=-13;(*p).u32=17;(*p).s64=-19;(*p).u64=23;pointer_calls=0;value_calls=0;side_effect(p,a,b);dump("side_effect",storage.as_ptr());}
{let mut storage=core::mem::MaybeUninit::<NestedEnvelope>::uninit();core::ptr::write_bytes(storage.as_mut_ptr(),0xa5,1);pointer_calls=0;value_calls=0;nested_store(core::ptr::addr_of_mut!((*storage.as_mut_ptr()).value),a);dump("nested_store",storage.as_ptr());}
{#[repr(C)]struct E{before:[u64;2],value:f64,after:[u64;2]}let mut storage=core::mem::MaybeUninit::<E>::uninit();core::ptr::write_bytes(storage.as_mut_ptr(),0xa5,1);pointer_calls=0;value_calls=0;float_store(core::ptr::addr_of_mut!((*storage.as_mut_ptr()).value),f64::from_bits(a));dump("float_store",storage.as_ptr());}
{#[repr(C)]struct E{before:[u64;2],value:*mut i64,after:[u64;2]}let mut storage=core::mem::MaybeUninit::<E>::uninit();core::ptr::write_bytes(storage.as_mut_ptr(),0xa5,1);let mut input=a as i64;pointer_store(core::ptr::addr_of_mut!((*storage.as_mut_ptr()).value),&mut input);let s=&*storage.as_ptr();println!("pointer_store {} {} {} {} {}",u8::from(s.value==core::ptr::addr_of_mut!(input)),s.before[0],s.before[1],s.after[0],s.after[1]);}
{#[repr(C)]struct E{before:[u64;2],value:bool,after:[u64;2]}let mut storage=core::mem::MaybeUninit::<E>::uninit();core::ptr::write_bytes(storage.as_mut_ptr(),0xa5,1);core::ptr::addr_of_mut!((*storage.as_mut_ptr()).value).write(false);pointer_calls=0;value_calls=0;bool_store(core::ptr::addr_of_mut!((*storage.as_mut_ptr()).value),a&1!=0);dump("bool_store",storage.as_ptr());}
}}}}
