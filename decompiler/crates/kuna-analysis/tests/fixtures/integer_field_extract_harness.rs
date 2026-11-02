#![allow(non_camel_case_types,dead_code,unused_mut,unused_parens)]
struct Pair { first:i64, second:i64 }
struct UPair { first:u64, second:u64 }
static mut CALLS:u64=0;
unsafe fn make_pair(a:i64,b:i64)->Pair { CALLS+=1; Pair{first:a.wrapping_add(b),second:a.wrapping_sub(b)} }
unsafe fn make_unsigned(a:i64,b:i64)->UPair { CALLS+=1; UPair{first:(a as u64).wrapping_add(b as u64),second:(a as u64).wrapping_sub(b as u64)} }
/* EMITTED_CALLERS */
fn main() { let values:[i64;28]=[-9223372036854775808,-9223372036854775807,-4294967297,-4294967296,-65537,-65536,-32769,-32768,-257,-256,-129,-128,-1,0,1,127,128,255,256,32767,32768,65535,65536,4294967295,4294967296,4294967297,9223372036854775806,9223372036854775807]; unsafe { for a in values { for b in values {
CALLS=0; println!("{:016x}",signed_int8(a,b) as u64); assert!(CALLS==1,"signed_int8");
CALLS=0; println!("{:016x}",signed_int8_wide(a,b) as u64); assert!(CALLS==1,"signed_int8_wide");
CALLS=0; println!("{:016x}",signed_uint8(a,b) as u64); assert!(CALLS==1,"signed_uint8");
CALLS=0; println!("{:016x}",signed_int16(a,b) as u64); assert!(CALLS==1,"signed_int16");
CALLS=0; println!("{:016x}",signed_int16_wide(a,b) as u64); assert!(CALLS==1,"signed_int16_wide");
CALLS=0; println!("{:016x}",signed_uint16(a,b) as u64); assert!(CALLS==1,"signed_uint16");
CALLS=0; println!("{:016x}",signed_int32(a,b) as u64); assert!(CALLS==1,"signed_int32");
CALLS=0; println!("{:016x}",signed_int32_wide(a,b) as u64); assert!(CALLS==1,"signed_int32_wide");
CALLS=0; println!("{:016x}",signed_uint32(a,b) as u64); assert!(CALLS==1,"signed_uint32");
CALLS=0; println!("{:016x}",unsigned_int8(a,b) as u64); assert!(CALLS==1,"unsigned_int8");
CALLS=0; println!("{:016x}",unsigned_uint8(a,b) as u64); assert!(CALLS==1,"unsigned_uint8");
CALLS=0; println!("{:016x}",unsigned_int16(a,b) as u64); assert!(CALLS==1,"unsigned_int16");
CALLS=0; println!("{:016x}",unsigned_uint16(a,b) as u64); assert!(CALLS==1,"unsigned_uint16");
CALLS=0; println!("{:016x}",unsigned_int32(a,b) as u64); assert!(CALLS==1,"unsigned_int32");
CALLS=0; println!("{:016x}",unsigned_uint32(a,b) as u64); assert!(CALLS==1,"unsigned_uint32");
CALLS=0; println!("{:016x}",first_narrow(a,b) as u64); assert!(CALLS==1,"first_narrow");
CALLS=0; println!("{:016x}",shift8(a,b) as u64); assert!(CALLS==1,"shift8");
CALLS=0; println!("{:016x}",shift32(a,b) as u64); assert!(CALLS==1,"shift32");
CALLS=0; println!("{:016x}",signed_high(a,b) as u64); assert!(CALLS==1,"signed_high");
CALLS=0; println!("{:016x}",exact_signed(a,b) as u64); assert!(CALLS==1,"exact_signed");
CALLS=0; println!("{:016x}",exact_unsigned(a,b) as u64); assert!(CALLS==1,"exact_unsigned");
}}}}
