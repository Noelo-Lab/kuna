#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn array_address(mut p: *mut Fields, mut i: i64) -> *mut i32
{
  return &mut (*p).values[i as usize];
}
