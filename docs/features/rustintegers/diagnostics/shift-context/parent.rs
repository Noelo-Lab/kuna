#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn insert_bit(mut a: u64, mut n: u64) -> u64
{
  return a | 1 << (n & 0x3f);
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn isolated_bit(mut n: u64) -> u64
{
  return 1 << (n as u8 & 0x3f);
}
fn main(){for a in [0u64,1,0x8000000000000000,0xffffffffffffffff]{for n in 0..64u64{unsafe{println!("{:016x} {:016x}",insert_bit(a,n),isolated_bit(n));}}}}
