#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn array_decay(mut p: *mut [i32; 9]) -> *mut i32
{
  return &raw mut *p as *mut i32;
}
