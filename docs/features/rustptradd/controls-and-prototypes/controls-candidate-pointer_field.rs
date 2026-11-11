#![allow(dead_code,unused_mut,unused_parens)]
#[repr(C)]struct Arrays{prefix:i32,values:[i32;12],pointer:*mut i64}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn pointer_field(mut p: *mut Arrays, mut i: i64) -> i64
{
  return *(*p).pointer.wrapping_byte_offset((i as isize).wrapping_mul(8));
}
fn main(){}
