#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn indexed(mut p: *mut i32, mut i: i64) -> i32
{
  return p[i];
}
