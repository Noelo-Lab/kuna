use std::hint::black_box;
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn sign8(mut a: i64, mut b: u64) -> i64
{
  return a + (b as u8) as i64 * 8 + 0x40;
}

fn main() {
    let mut values: Vec<u64> = (0..256).collect();
    values.extend([0x7fff, 0x8000, 0xffff, 0x7fff_ffff, 0x8000_0000, 0xffff_ffff, u64::MAX]);
    for a in [i64::MIN, i64::MIN + 2048, -1, 0, 1, i64::MAX] {
        for b in &values {
            let a = black_box(a);
            let b = black_box(*b);
            unsafe { println!("{}",sign8(a,b)); }
        }
    }
}
