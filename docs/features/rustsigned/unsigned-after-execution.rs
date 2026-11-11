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
fn main() {
    let mut values: Vec<u64> = (0..256).collect();
    for bits in [16, 32, 64] {
        let sign = 1u64 << (bits - 1);
        values.extend([sign - 1, sign, sign + 1, u64::MAX >> (64 - bits)]);
    }
    values.sort_unstable(); values.dedup();
    for &a in &values { for &b in &values {
        unsafe { println!("{} {} {} {} {} {} {} {} {}",
            unsigned_less8(a as _, b as _), unsigned_shift8(a as _, 2),
            unsigned_less16(a as _, b as _), unsigned_shift16(a as _, 2),
            unsigned_less32(a as _, b as _), unsigned_shift32(a as _, 2),
            unsigned_less64(a, b), unsigned_shift64(a, 2), unsigned_loop(a as _, b as u32 & 31)); }
    } }
}
