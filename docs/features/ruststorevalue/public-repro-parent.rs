#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn store(mut p: *mut i64, mut value: u64)
{
  *p = value;
}
