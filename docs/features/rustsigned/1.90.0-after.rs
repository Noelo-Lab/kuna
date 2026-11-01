#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn byte_less(mut a: u64, mut b: u64) -> u32
{
  return (((a as u8) as i8) < ((b as u8) as i8)) as u32;
}

#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn byte_lesseq(mut a: u64, mut b: u64) -> u32
{
  return (((a as u8) as i8) <= ((b as u8) as i8)) as u32;
}

#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn byte_shift(mut a: u64) -> u32
{
  return (((((a as u8) as i8) >> 2) as u8) as u8) as u32;
}

#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn byte_unsigned(mut a: u64, mut b: u64) -> u32
{
  return ((a as u8) < (b as u8)) as u32;
}

#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn byte_logical(mut a: u64) -> u32
{
  return (a >> 2) as u32 & 0x3f;
}

#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn byte_store(mut p: *mut u8)
{
  *p = ((*p as i8) >> 2) as u8;
  return;
}

#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn byte_call(mut a: u64) -> u32
{
  return (((((byte_source(a) as u8) as i8) >> 2) as u8) as u8) as u32;
}

#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn byte_loop(mut a: u64, mut n: u32) -> u32
{
  let mut v1: u32; // eax | ecx | esi
  let mut v2: u8; // al | dil
  
  v2 = a as u8;
  if n != 0 {
    v1 = n;
    if n & 7 != 0 {
      v1 = 0;
      loop {
        v2 = (((v2 as u8) as i8) >> 1) as u8;
        v1 += 1;
        if !(n & 7 != v1) {
          break;
        }
      }
      v1 = n - v1;
    }
    if 8 <= n {
      v2 = (((v2 as u8) as i8) >> 6) as u8;
      loop {
        v2 = (((v2 as u8) as i8) >> 2) as u8;
        v1 -= 8;
        if !(v1 != 0) {
          break;
        }
      }
    }
  }
  return v2 as u32;
}
