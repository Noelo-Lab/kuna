#![allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
static mut value_calls:u32=0;
unsafe fn counted_unsigned(value:u64)->u64 { value_calls+=1;value }
unsafe fn counted_signed(value:i64)->i64 { value_calls+=1;value }
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn as_unsigned8(mut value: i8) -> u8
{
  return value as u8;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn as_signed8(mut value: u8) -> i8
{
  return value as i8;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn masked8(mut value: i8) -> u8
{
  return value as u8 & 0x55;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn mixed_add8(mut left: u8, mut right: i8) -> u8
{
  return right as u8 + left;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn mixed_sub8(mut left: u8, mut right: i8) -> u8
{
  return left - right as u8;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn mixed_and8(mut left: u8, mut right: i8) -> u8
{
  return right as u8 & left;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn mixed_or8(mut left: u8, mut right: i8) -> u8
{
  return right as u8 | left;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn mixed_xor8(mut left: u8, mut right: i8) -> u8
{
  return right as u8 ^ left;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn mixed_multiply8(mut left: u8, mut right: i8) -> u8
{
  return right as u8 * left;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn inplace8(mut left: i8, mut right: u8) -> i8
{
  return right as i8 + left;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn as_unsigned16(mut value: i16) -> u16
{
  return value as u16;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn as_signed16(mut value: u16) -> i16
{
  return value as i16;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn masked16(mut value: i16) -> u16
{
  return value as u16 & 0x55;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn mixed_add16(mut left: u16, mut right: i16) -> u16
{
  return right as u16 + left;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn mixed_sub16(mut left: u16, mut right: i16) -> u16
{
  return left - right as u16;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn mixed_and16(mut left: u16, mut right: i16) -> u16
{
  return right as u16 & left;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn mixed_or16(mut left: u16, mut right: i16) -> u16
{
  return right as u16 | left;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn mixed_xor16(mut left: u16, mut right: i16) -> u16
{
  return right as u16 ^ left;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn mixed_multiply16(mut left: u16, mut right: i16) -> u16
{
  return right as u16 * left;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn inplace16(mut left: i16, mut right: u16) -> i16
{
  return right as i16 + left;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn as_unsigned32(mut value: i32) -> u32
{
  return value as u32;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn as_signed32(mut value: u32) -> i32
{
  return value as i32;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn masked32(mut value: i32) -> u32
{
  return value as u32 & 0x55;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn mixed_add32(mut left: u32, mut right: i32) -> u32
{
  return right as u32 + left;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn mixed_sub32(mut left: u32, mut right: i32) -> u32
{
  return left - right as u32;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn mixed_and32(mut left: u32, mut right: i32) -> u32
{
  return right as u32 & left;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn mixed_or32(mut left: u32, mut right: i32) -> u32
{
  return right as u32 | left;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn mixed_xor32(mut left: u32, mut right: i32) -> u32
{
  return right as u32 ^ left;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn mixed_multiply32(mut left: u32, mut right: i32) -> u32
{
  return right as u32 * left;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn inplace32(mut left: i32, mut right: u32) -> i32
{
  return (left as u32 + right) as i32;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn as_unsigned64(mut value: i64) -> u64
{
  return value as u64;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn as_signed64(mut value: u64) -> i64
{
  return value as i64;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn masked64(mut value: i64) -> u64
{
  return (value as u32 & 0x55) as u64;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn mixed_add64(mut left: u64, mut right: i64) -> u64
{
  return right as u64 + left;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn mixed_sub64(mut left: u64, mut right: i64) -> u64
{
  return left - right as u64;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn mixed_and64(mut left: u64, mut right: i64) -> u64
{
  return right as u64 & left;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn mixed_or64(mut left: u64, mut right: i64) -> u64
{
  return right as u64 | left;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn mixed_xor64(mut left: u64, mut right: i64) -> u64
{
  return right as u64 ^ left;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn mixed_multiply64(mut left: u64, mut right: i64) -> u64
{
  return right as u64 * left;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn inplace64(mut left: i64, mut right: u64) -> i64
{
  return (left as u64 + right) as i64;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn branch_value(mut left: u64, mut right: u64, mut choose_left: bool) -> i64
{
  let mut v1: u64; // rax | rdi | rsi
  
  v1 = left;
  if !choose_left {
    v1 = right;
  }
  return v1 as i64;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn call_unsigned(mut value: u64) -> i64
{
  return counted_unsigned(value) as i64;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn call_signed(mut value: i64) -> u64
{
  return counted_signed(value) as u64;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn call_add(mut left: u64, mut right: i64) -> u64
{
  return counted_unsigned(left) + right as u64;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn high_mask(mut value: u64) -> i64
{
  return (value & 0x7fffffffffffffff) as i64;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn negative_mask(mut value: i64) -> u64
{
  return value as u64 & 0xffffffffffffff7f;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn float_identity(mut value: f64) -> f64
{
  return value;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn pointer_identity(mut value: *mut u64) -> *mut u64
{
  return value;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn bool_identity(mut value: bool) -> bool
{
  return value;
}
fn main(){let values:[u64;18]=[0,1,2,127,128,255,256,32767,32768,65535,65536,2147483647,2147483648,4294967295,4294967296,9223372036854775807,9223372036854775808,18446744073709551615];for a in values{for b in values{unsafe{
{value_calls=0;let value=as_unsigned8(a as i8);let calls=value_calls;println!("as_unsigned8 {} {}",value as u64,calls);}
{value_calls=0;let value=as_signed8(a as u8);let calls=value_calls;println!("as_signed8 {} {}",value as u64,calls);}
{value_calls=0;let value=masked8(a as i8);let calls=value_calls;println!("masked8 {} {}",value as u64,calls);}
{value_calls=0;let value=mixed_add8(a as u8,b as i8);let calls=value_calls;println!("mixed_add8 {} {}",value as u64,calls);}
{value_calls=0;let value=mixed_sub8(a as u8,b as i8);let calls=value_calls;println!("mixed_sub8 {} {}",value as u64,calls);}
{value_calls=0;let value=mixed_and8(a as u8,b as i8);let calls=value_calls;println!("mixed_and8 {} {}",value as u64,calls);}
{value_calls=0;let value=mixed_or8(a as u8,b as i8);let calls=value_calls;println!("mixed_or8 {} {}",value as u64,calls);}
{value_calls=0;let value=mixed_xor8(a as u8,b as i8);let calls=value_calls;println!("mixed_xor8 {} {}",value as u64,calls);}
{value_calls=0;let value=mixed_multiply8(a as u8,b as i8);let calls=value_calls;println!("mixed_multiply8 {} {}",value as u64,calls);}
{value_calls=0;let value=inplace8(a as i8,b as u8);let calls=value_calls;println!("inplace8 {} {}",value as u64,calls);}
{value_calls=0;let value=as_unsigned16(a as i16);let calls=value_calls;println!("as_unsigned16 {} {}",value as u64,calls);}
{value_calls=0;let value=as_signed16(a as u16);let calls=value_calls;println!("as_signed16 {} {}",value as u64,calls);}
{value_calls=0;let value=masked16(a as i16);let calls=value_calls;println!("masked16 {} {}",value as u64,calls);}
{value_calls=0;let value=mixed_add16(a as u16,b as i16);let calls=value_calls;println!("mixed_add16 {} {}",value as u64,calls);}
{value_calls=0;let value=mixed_sub16(a as u16,b as i16);let calls=value_calls;println!("mixed_sub16 {} {}",value as u64,calls);}
{value_calls=0;let value=mixed_and16(a as u16,b as i16);let calls=value_calls;println!("mixed_and16 {} {}",value as u64,calls);}
{value_calls=0;let value=mixed_or16(a as u16,b as i16);let calls=value_calls;println!("mixed_or16 {} {}",value as u64,calls);}
{value_calls=0;let value=mixed_xor16(a as u16,b as i16);let calls=value_calls;println!("mixed_xor16 {} {}",value as u64,calls);}
{value_calls=0;let value=mixed_multiply16(a as u16,b as i16);let calls=value_calls;println!("mixed_multiply16 {} {}",value as u64,calls);}
{value_calls=0;let value=inplace16(a as i16,b as u16);let calls=value_calls;println!("inplace16 {} {}",value as u64,calls);}
{value_calls=0;let value=as_unsigned32(a as i32);let calls=value_calls;println!("as_unsigned32 {} {}",value as u64,calls);}
{value_calls=0;let value=as_signed32(a as u32);let calls=value_calls;println!("as_signed32 {} {}",value as u64,calls);}
{value_calls=0;let value=masked32(a as i32);let calls=value_calls;println!("masked32 {} {}",value as u64,calls);}
{value_calls=0;let value=mixed_add32(a as u32,b as i32);let calls=value_calls;println!("mixed_add32 {} {}",value as u64,calls);}
{value_calls=0;let value=mixed_sub32(a as u32,b as i32);let calls=value_calls;println!("mixed_sub32 {} {}",value as u64,calls);}
{value_calls=0;let value=mixed_and32(a as u32,b as i32);let calls=value_calls;println!("mixed_and32 {} {}",value as u64,calls);}
{value_calls=0;let value=mixed_or32(a as u32,b as i32);let calls=value_calls;println!("mixed_or32 {} {}",value as u64,calls);}
{value_calls=0;let value=mixed_xor32(a as u32,b as i32);let calls=value_calls;println!("mixed_xor32 {} {}",value as u64,calls);}
{value_calls=0;let value=mixed_multiply32(a as u32,b as i32);let calls=value_calls;println!("mixed_multiply32 {} {}",value as u64,calls);}
{value_calls=0;let value=inplace32(a as i32,b as u32);let calls=value_calls;println!("inplace32 {} {}",value as u64,calls);}
{value_calls=0;let value=as_unsigned64(a as i64);let calls=value_calls;println!("as_unsigned64 {} {}",value as u64,calls);}
{value_calls=0;let value=as_signed64(a as u64);let calls=value_calls;println!("as_signed64 {} {}",value as u64,calls);}
{value_calls=0;let value=masked64(a as i64);let calls=value_calls;println!("masked64 {} {}",value as u64,calls);}
{value_calls=0;let value=mixed_add64(a as u64,b as i64);let calls=value_calls;println!("mixed_add64 {} {}",value as u64,calls);}
{value_calls=0;let value=mixed_sub64(a as u64,b as i64);let calls=value_calls;println!("mixed_sub64 {} {}",value as u64,calls);}
{value_calls=0;let value=mixed_and64(a as u64,b as i64);let calls=value_calls;println!("mixed_and64 {} {}",value as u64,calls);}
{value_calls=0;let value=mixed_or64(a as u64,b as i64);let calls=value_calls;println!("mixed_or64 {} {}",value as u64,calls);}
{value_calls=0;let value=mixed_xor64(a as u64,b as i64);let calls=value_calls;println!("mixed_xor64 {} {}",value as u64,calls);}
{value_calls=0;let value=mixed_multiply64(a as u64,b as i64);let calls=value_calls;println!("mixed_multiply64 {} {}",value as u64,calls);}
{value_calls=0;let value=inplace64(a as i64,b as u64);let calls=value_calls;println!("inplace64 {} {}",value as u64,calls);}
{value_calls=0;let value=branch_value(a,b,a&1!=0);let calls=value_calls;println!("branch_value {} {}",value as u64,calls);}
{value_calls=0;let value=call_unsigned(a as u64);let calls=value_calls;println!("call_unsigned {} {}",value as u64,calls);}
{value_calls=0;let value=call_signed(a as i64);let calls=value_calls;println!("call_signed {} {}",value as u64,calls);}
{value_calls=0;let value=call_add(a as u64,b as i64);let calls=value_calls;println!("call_add {} {}",value as u64,calls);}
{value_calls=0;let value=high_mask(a as u64);let calls=value_calls;println!("high_mask {} {}",value as u64,calls);}
{value_calls=0;let value=negative_mask(a as i64);let calls=value_calls;println!("negative_mask {} {}",value as u64,calls);}
{value_calls=0;let value=float_identity(f64::from_bits(a));let calls=value_calls;println!("float_identity {} {}",value.to_bits(),calls);}
{value_calls=0;let mut cell=a;let pointer=&mut cell as *mut u64;let value=pointer_identity(pointer);let calls=value_calls;println!("pointer_identity {} {} {}",*value,u8::from(value==pointer),calls);}
{value_calls=0;let value=bool_identity(a&1!=0);let calls=value_calls;println!("bool_identity {} {}",u8::from(value),calls);}
}}}}
