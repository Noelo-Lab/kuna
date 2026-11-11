#![allow(dead_code,unused_mut,unused_parens)]
#[repr(C)]struct Arrays{prefix:i32,values:[i32;12],pointer:*mut i64}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn float_load(mut p: *mut f64, mut i: i64) -> f64
{
  return p[i];
}
fn main(){}
