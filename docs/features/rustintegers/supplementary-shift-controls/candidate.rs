#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn logical_min(mut n: u64) -> i64
{
  return ::core::convert::identity::<u64>(0x8000000000000000 >> (n as u8 & 0x3f)) as i64;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn logical_max(mut n: u64) -> i64
{
  return ::core::convert::identity::<u64>(0xffffffffffffffff >> (n as u8 & 0x3f)) as i64;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn arithmetic_min(mut n: u64) -> u64
{
  return ::core::convert::identity::<i64>(-0x8000000000000000 >> (n as u8 & 0x3f)) as u64;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn arithmetic_negative(mut n: u64) -> u64
{
  return ::core::convert::identity::<i64>(-3 >> (n as u8 & 0x3f)) as u64;
}
fn main(){for n in 0..64u64 {unsafe {println!("{:016x} {:016x} {:016x} {:016x}",logical_min(n) as u64,logical_max(n) as u64,arithmetic_min(n),arithmetic_negative(n));}}}
