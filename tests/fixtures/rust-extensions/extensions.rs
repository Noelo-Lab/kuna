#![allow(unused_unsafe)]
use std::hint::black_box;
#[no_mangle] #[inline(never)] pub extern "C" fn sign8(a:i64,b:u64)->i64{a.wrapping_add((b as i8 as i64).wrapping_mul(8)).wrapping_add(64)}
#[no_mangle] #[inline(never)] pub extern "C" fn sign16(a:i64,b:u64)->i64{a.wrapping_add((b as i16 as i64).wrapping_mul(8)).wrapping_add(64)}
#[no_mangle] #[inline(never)] pub extern "C" fn sign32(a:i64,b:u64)->i64{a.wrapping_add((b as i32 as i64).wrapping_mul(8)).wrapping_add(64)}
#[no_mangle] #[inline(never)] pub extern "C" fn narrow_sum(a:i64,b:u64)->i64{a.wrapping_add((b as i8).wrapping_add(11) as i64)}
#[no_mangle] #[inline(never)] pub extern "C" fn narrow_product(a:i64,b:u64)->i64{a.wrapping_add((b as i8).wrapping_mul(7) as i64)}
#[no_mangle] #[inline(never)] pub extern "C" fn zero8(a:u64,b:u64)->u64{a.wrapping_add((b as u8 as u64).wrapping_mul(8)).wrapping_add(64)}
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
