#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn array_address(mut p: *mut Fields, mut i: i64) -> *mut i32
{
  return (&raw mut (*p).values).wrapping_byte_offset((i as isize).wrapping_mul(4)) as *mut i32;
}
