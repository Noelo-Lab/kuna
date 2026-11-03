#![allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
static mut value_calls:u32=0;
unsafe fn counted_unsigned(value:u64)->u64 {value_calls+=1;value}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn insert_bit(mut a: u64, mut n: u64) -> u64
{
  return a | ::core::convert::identity::<i64>(1 << (n & 0x3f)) as u64;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn isolated_bit(mut n: u64) -> u64
{
  return ::core::convert::identity::<i64>(1 << (n as u8 & 0x3f)) as u64;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn invert_low(mut n: u64) -> u64
{
  return ::core::convert::identity::<i64>(!(-1 << (n as u8 & 0x3f))) as u64;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn shift_again(mut n: u64, mut m: u64) -> u64
{
  return (::core::convert::identity::<i64>(1 << (n as u8 & 0x3f)) as u64) >> (m as u8 & 0x3f);
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn literal_right(mut n: u64) -> u64
{
  return 0x8000000000000000 >> (n as u8 & 0x3f);
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn signed_right(mut n: u64) -> i64
{
  return -0x8000000000000000 >> (n as u8 & 0x3f);
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn named_shift(mut a: u64, mut n: u64) -> u64
{
  return a << (n as u8 & 0x3f);
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn signed_shift(mut a: i64, mut n: u64) -> i64
{
  return a << (n as u8 & 0x3f);
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn mixed_shift(mut a: u64, mut n: u64) -> u64
{
  return ::core::convert::identity::<i64>(1 << (n as u8 & 0x3f)) as u64 + a;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn accumulate_bits(mut n: u64) -> u64
{
  let mut v1: u64; // rax
  let mut v2: u32; // ecx | edx
  
  if n & 0x3f != 0 {
    v2 = 0;
    v1 = 0;
    loop {
      v1 = v1 | ::core::convert::identity::<i64>(1 << (v2 as u8 & 0x3f)) as u64;
      v2 += 1;
      if !(v2 < n as u32 & 0x3f) {
        break;
      }
    }
    return v1;
  }
  return 0;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn accumulate_calls(mut acc: i64, mut step: u64, mut count: u32) -> i64
{
  while count != 0 {
    count -= 1;
    acc = acc + counted_unsigned(step) as i64;
  }
  return acc;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn mixed_less(mut a: u64, mut b: i64, mut c: u64) -> u64
{
  return (b as u64 + a < c) as u64;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn mixed_left(mut a: u64, mut b: i64, mut n: u64) -> u64
{
  return b as u64 + a << (n as u8 & 0x3f);
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn mixed_sub_less(mut a: u64, mut b: i64, mut c: u64) -> u64
{
  return (a - (b as u64) < c) as u64;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn mixed_sub_left(mut a: u64, mut b: i64, mut n: u64) -> u64
{
  return a - (b as u64) << (n as u8 & 0x3f);
}
fn main(){for a in [0u64,1,0x8000000000000000,0xffffffffffffffff]{for n in 0..64u64{for m in [0u64,1,31,32,63]{unsafe{value_calls=0;let accumulated=accumulate_calls(a as i64,a^m,(n&7)as u32)as u64;let calls=value_calls;println!("{:016x} {:016x} {:016x} {:016x} {:016x} {:016x} {:016x} {:016x} {:016x} {:016x} {:016x} {:016x} {:016x} {:016x} {:016x} {}",insert_bit(a,n),isolated_bit(n),invert_low(n),shift_again(n,m),literal_right(n),signed_right(n)as u64,named_shift(a,n),signed_shift(a as i64,n)as u64,mixed_shift(a,n),accumulate_bits(n),accumulated,mixed_less(a,m as i64,n),mixed_left(a,m as i64,n),mixed_sub_less(a,m as i64,n),mixed_sub_left(a,m as i64,n),calls);}}}}}
