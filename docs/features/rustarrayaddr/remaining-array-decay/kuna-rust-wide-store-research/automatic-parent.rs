#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn wide_copy(mut p: *mut [u8; 16], mut q: *mut [u8; 16])
{
  let mut v1: u64; // xmm0_qb
  
  v1 = *(&mut (*q)[8] as *mut u64);
  *(*p as *mut u64) = *(*q as *mut u64);
  *(&mut (*p)[8] as *mut u64) = v1;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn wide_zero(mut p: *mut [u8; 16])
{
  *p = 0;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn wide_unsigned(mut p: *mut [u8; 16], mut x: u64)
{
  *(*p as *mut u64) = x;
  *(&mut (*p)[8] as *mut u64) = 0;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn wide_signed(mut p: *mut [u8; 16], mut x: i64)
{
  *(*p as *mut i64) = x;
  *(&mut (*p)[8] as *mut i64) = x >> 0x3f;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn wide_halves(mut p: *mut [u8; 16], mut lo: u64, mut hi: u64)
{
  *(*p as *mut u64) = lo;
  *(&mut (*p)[8] as *mut u64) = hi;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn wide_add(mut p: *mut [u8; 16], mut q: *mut [u8; 16])
{
  let mut v1: u64; // rax
  let mut v2: i64; // rdx
  
  v1 = *(*q as *mut u64);
  v2 = *(&mut (*q)[8] as *mut i64);
  *(*p as *mut u64) = v1 + 1;
  *(&mut (*p)[8] as *mut u64) = v2 as u64 + (0xffffffffffffffff <= v1) as u64;
}
fn main(){}
