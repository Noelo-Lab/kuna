#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn array_store(mut p: *mut i64, mut i: u64, mut a: u64)
{
  p[i] = a;
}
fn main(){}
