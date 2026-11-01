#![allow(unused_mut)]
unsafe fn unsigned_less8(a: u8, b: u8) -> u32 { ((a as i8) < (b as i8)) as u32 }
unsafe fn unsigned_shift8(a: u8, n: u32) -> u8 { let mut value = a as i8; for _ in 0..n { value >>= 1; } value as u8 }
unsafe fn unsigned_less16(a: u16, b: u16) -> u32 { ((a as i16) < (b as i16)) as u32 }
unsafe fn unsigned_shift16(a: u16, n: u32) -> u16 { let mut value = a as i16; for _ in 0..n { value >>= 1; } value as u16 }
unsafe fn unsigned_less32(a: u32, b: u32) -> u32 { ((a as i32) < (b as i32)) as u32 }
unsafe fn unsigned_shift32(a: u32, n: u32) -> u32 { let mut value = a as i32; for _ in 0..n { value >>= 1; } value as u32 }
unsafe fn unsigned_less64(a: u64, b: u64) -> u32 { ((a as i64) < (b as i64)) as u32 }
unsafe fn unsigned_shift64(a: u64, n: u32) -> u64 { let mut value = a as i64; for _ in 0..n { value >>= 1; } value as u64 }
unsafe fn unsigned_loop(a: u8, n: u32) -> u8 { let mut a = a as i8; for _ in 0..n { a >>= 1; } a as u8 }
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
