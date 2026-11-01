#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn byte_loop_direct(mut a: u8, mut n: u32) -> u8
{
  while n != 0 {
    a = ((a as i8) >> 1) as u8;
    n -= 1;
  }
  return a;
}
fn main() { for a in 0..256u16 { for n in 0..32u32 { println!("{}", unsafe { byte_loop_direct(a as _, n) }); } } }
