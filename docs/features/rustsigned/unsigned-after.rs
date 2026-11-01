#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn unsigned_less8(mut a: u8, mut b: u8) -> u32
{
  return (((a as u8) as i8) < ((b as u8) as i8)) as u32;
}

#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn unsigned_shift8(mut a: u8, mut n: u32) -> u8
{
  while n != 0 {
    a = (((a as u8) as i8) >> 1) as u8;
    n -= 1;
  }
  return a;
}

#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn unsigned_less16(mut a: u16, mut b: u16) -> u32
{
  return ((a as i16) < (b as i16)) as u32;
}

#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn unsigned_shift16(mut a: u16, mut n: u32) -> u16
{
  while n != 0 {
    a = ((a as i16) >> 1) as u16;
    n -= 1;
  }
  return a;
}

#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn unsigned_less32(mut a: u32, mut b: u32) -> u32
{
  return ((a as i32) < (b as i32)) as u32;
}

#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn unsigned_shift32(mut a: u32, mut n: u32) -> u32
{
  while n != 0 {
    a = ((a as i32) >> 1) as u32;
    n -= 1;
  }
  return a;
}

#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn unsigned_less64(mut a: u64, mut b: u64) -> u32
{
  return ((a as i64) < (b as i64)) as u32;
}

#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn unsigned_shift64(mut a: u64, mut n: u32) -> u64
{
  while n != 0 {
    a = ((a as i64) >> 1) as u64;
    n -= 1;
  }
  return a;
}

#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn unsigned_loop(mut a: u8, mut n: u32) -> u8
{
  while n != 0 {
    a = ((a as i8) >> 1) as u8;
    n -= 1;
  }
  return a;
}
