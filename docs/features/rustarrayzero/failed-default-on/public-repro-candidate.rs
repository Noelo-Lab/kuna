#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn zero_array(mut p: *mut [u8; 16])
{
  *p = [0; 16];
}
