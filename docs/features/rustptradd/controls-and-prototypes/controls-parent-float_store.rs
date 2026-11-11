#![allow(dead_code,unused_mut,unused_parens)]
#[repr(C)]struct Arrays{prefix:i32,values:[i32;12],pointer:*mut i64}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn float_store(mut p: *mut f64, mut i: i64, mut value: f64)
{
  p[i] = value;
}
fn main(){}
