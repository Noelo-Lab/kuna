#![allow(dead_code,unused_mut,unused_parens)]
#[repr(C)]struct Arrays{prefix:i32,values:[i32;12],pointer:*mut i64}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn nested_array(mut p: *mut [i32; 12], mut i: i64, mut j: i64) -> i32
{
  return (*p.wrapping_byte_offset((i as isize).wrapping_mul(48)))[j];
}
fn main(){}
