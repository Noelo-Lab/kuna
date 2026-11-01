use std::hint::black_box;
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn sign8(mut a: i64, mut b: u64) -> i64
{
  return a + ((b as u8) as i8) as i64 * 8 + 0x40;
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
  return (((b as u8 + 0xb) as u8) as i8) as i64 + a;
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

fn main() {
    let mut values: Vec<u64> = (0..256).collect();
    values.extend([0x7fff, 0x8000, 0xffff, 0x7fff_ffff, 0x8000_0000, 0xffff_ffff, u64::MAX]);
    for a in [i64::MIN, i64::MIN + 2048, -1, 0, 1, i64::MAX] {
        for b in &values {
            let a = black_box(a);
            let b = black_box(*b);
            unsafe { println!("{} {} {} {} {} {}", sign8(a, b), sign16(a, b), sign32(a, b),
                narrow_sum(a, b), narrow_product(a, b), zero8(a as u64, b)); }
        }
    }
}
