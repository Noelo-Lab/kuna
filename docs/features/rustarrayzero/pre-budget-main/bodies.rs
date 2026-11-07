#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn zero_unsigned(mut p: *mut [u8; 16])
{
  *p = 0;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn zero_signed(mut p: *mut [u8; 16])
{
  *p = 0;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn zero_field(mut p: *mut WideFields)
{
  *(&raw mut (*p).value as *mut [u8; 16]) = 0;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn zero_second(mut p: *mut [u8; 16])
{
  *p.wrapping_byte_offset(((1 as i64) as isize).wrapping_mul(16)) = 0;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn zero_index(mut p: *mut [u8; 16], mut i: u64)
{
  *p.wrapping_byte_offset((i as isize).wrapping_mul(16)) = 0;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn zero_callback(mut p: *mut [u8; 16])
{
  *counted_pointer(p) = 0;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn zero_bytes(mut p: *mut Byte16)
{
  *(&raw mut (*p).bytes as *mut [u8; 16]) = 0;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn one_unsigned(mut p: *mut [u8; 16])
{
  *(&raw mut *p as *mut u64) = 1;
  *(((&raw mut *p).wrapping_byte_offset(((8 as i64) as isize).wrapping_mul(1)) as *mut u8) as *mut u64) = 0;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn signed_input(mut p: *mut [u8; 16], mut x: i64)
{
  *(&raw mut *p as *mut i64) = x;
  *(((&raw mut *p).wrapping_byte_offset(((8 as i64) as isize).wrapping_mul(1)) as *mut u8) as *mut i64) = x >> 0x3f;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn unsigned_input(mut p: *mut [u8; 16], mut x: u64)
{
  *(&raw mut *p as *mut u64) = x;
  *(((&raw mut *p).wrapping_byte_offset(((8 as i64) as isize).wrapping_mul(1)) as *mut u8) as *mut u64) = 0;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn two_halves(mut p: *mut [u8; 16], mut lo: u64, mut hi: u64)
{
  *(&raw mut *p as *mut u64) = lo;
  *(((&raw mut *p).wrapping_byte_offset(((8 as i64) as isize).wrapping_mul(1)) as *mut u8) as *mut u64) = hi;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn copy_wide(mut p: *mut [u8; 16], mut q: *mut [u8; 16])
{
  let mut v1: u64; // xmm0_qb
  
  v1 = *(((&raw mut *q).wrapping_byte_offset(((8 as i64) as isize).wrapping_mul(1)) as *mut u8) as *mut u64);
  *(&raw mut *p as *mut u64) = *(&raw mut *q as *mut u64);
  *(((&raw mut *p).wrapping_byte_offset(((8 as i64) as isize).wrapping_mul(1)) as *mut u8) as *mut u64) = v1;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn zero_narrow(mut p: *mut u64)
{
  *p = 0;
}
