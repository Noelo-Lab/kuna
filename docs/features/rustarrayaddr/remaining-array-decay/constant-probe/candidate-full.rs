#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn integer_eight(mut p: *mut [i32; 9]) -> *mut i32
{
  return (&raw mut *p).wrapping_byte_offset(((8 as i64) as isize).wrapping_mul(4)) as *mut i32;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn integer_end(mut p: *mut [i32; 9]) -> *mut i32
{
  return *p.wrapping_byte_offset(((1 as i64) as isize).wrapping_mul(36));
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn pointer_eight(mut p: *mut [*mut u8; 9]) -> *mut *mut u8
{
  return (&raw mut *p).wrapping_byte_offset(((8 as i64) as isize).wrapping_mul(8)) as *mut *mut u8;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn pointer_end(mut p: *mut [*mut u8; 9]) -> *mut *mut u8
{
  return *p.wrapping_byte_offset(((1 as i64) as isize).wrapping_mul(72));
}
fn main(){}
