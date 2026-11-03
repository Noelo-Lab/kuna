#![allow(dead_code,unused_mut,unused_parens)]
#[repr(C)]struct Arrays{prefix:i32,values:[i32;12],pointer:*mut i64}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn pointer_load(mut p: *mut *mut i32, mut i: i64) -> *mut i32
{
  return *p.wrapping_byte_offset((i as isize).wrapping_mul(8));
}
fn main(){}
