#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn shift_address(mut p: *mut i32, mut n: u32) -> *mut i32
{
  return ((4 << (n as u8 & 0x3f)) + p as i64) as *mut i32;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn logical_address(mut p: *mut i32, mut n: u32) -> *mut i32
{
  return p.wrapping_byte_offset(((0xffffffffffffffff >> (n as u8 & 0x3f)) as isize).wrapping_mul(4));
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn signed_address(mut p: *mut i32, mut n: u32) -> *mut i32
{
  return p.wrapping_byte_offset(((-0x8000000000000000 >> (n as u8 & 0x3f)) as isize).wrapping_mul(4));
}
fn main(){}
