use std::hint::black_box;
use std::sync::atomic::{AtomicU32, Ordering::SeqCst};

static CALLS: AtomicU32 = AtomicU32::new(0);

#[no_mangle]
#[inline(never)]
pub extern "C" fn byte_source(a: u64) -> u64 {
    CALLS.fetch_add(1, SeqCst);
    black_box(a)
}

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
fn main() {
    for low_a in 0..256 {
        for low_b in 0..256 {
            let a = black_box(0x1234567800000000 | low_a);
            let b = black_box(0xfedcba9800000000 | low_b);
            let mut cell = a as u8;
            let calls = CALLS.load(SeqCst);
            unsafe {
                byte_store((&mut cell as *mut u8).cast());
                let from_call = byte_call(a);
                assert_eq!(CALLS.load(SeqCst), calls + 1);
                println!("{} {} {} {} {} {} {} {}", byte_less(a, b), byte_lesseq(a, b),
                    byte_shift(a), byte_unsigned(a, b), byte_logical(a), cell,
                    from_call, byte_loop(a, b as u32 & 31));
            }
        }
    }
}
