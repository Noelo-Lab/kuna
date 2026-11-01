#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn sign8(mut a: i64, mut b: u64) -> i64
{
  return a + (b as u8) as i64 * 8 + 0x40;
}

#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn sign16(mut a: i64, mut b: u64) -> i64
{
  return a + (b as i16) as i64 * 8 + 0x40;
}

#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn sign32(mut a: i64, mut b: u64) -> i64
{
  return a + (b as i32) as i64 * 8 + 0x40;
}

#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn narrow_sum(mut a: i64, mut b: u64) -> i64
{
  return (b as u8 + 0xb) as u8 + a;
}

#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn narrow_product(mut a: i64, mut b: u64) -> i64
{
  return (((b * 0x700000000000000) as i64) >> 0x38) + a;
}

#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn zero8(mut a: u64, mut b: u64) -> u64
{
  return a + (b & 0xff) * 8 + 0x40;
}
