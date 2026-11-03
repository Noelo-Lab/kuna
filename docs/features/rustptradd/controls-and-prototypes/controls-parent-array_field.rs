#![allow(dead_code,unused_mut,unused_parens)]
#[repr(C)]struct Arrays{prefix:i32,values:[i32;12],pointer:*mut i64}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn array_field(mut p: *mut Arrays, mut i: i64) -> i32
{
  return (*p).values[i];
}
fn main(){}
