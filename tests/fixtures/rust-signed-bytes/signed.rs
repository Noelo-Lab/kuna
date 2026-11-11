use std::hint::black_box;
use std::sync::atomic::{AtomicU32, Ordering::SeqCst};

static CALLS: AtomicU32 = AtomicU32::new(0);

#[no_mangle]
#[inline(never)]
pub extern "C" fn byte_source(a: u64) -> u64 {
    CALLS.fetch_add(1, SeqCst);
    black_box(a)
}

#[no_mangle]
#[inline(never)]
pub extern "C" fn byte_less(a: u64, b: u64) -> u32 {
    ((a as i8) < (b as i8)) as u32
}

#[no_mangle]
#[inline(never)]
pub extern "C" fn byte_lesseq(a: u64, b: u64) -> u32 {
    ((a as i8) <= (b as i8)) as u32
}

#[no_mangle]
#[inline(never)]
pub extern "C" fn byte_shift(a: u64) -> u32 {
    ((a as i8) >> 2) as u8 as u32
}

#[no_mangle]
#[inline(never)]
pub extern "C" fn byte_unsigned(a: u64, b: u64) -> u32 {
    ((a as u8) < (b as u8)) as u32
}

#[no_mangle]
#[inline(never)]
pub extern "C" fn byte_logical(a: u64) -> u32 {
    ((a as u8) >> 2) as u32
}

#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn byte_store(p: *mut i8) {
    unsafe { *p >>= 2; }
}

#[no_mangle]
#[inline(never)]
pub extern "C" fn byte_call(a: u64) -> u32 {
    ((byte_source(a) as i8) >> 2) as u8 as u32
}

#[no_mangle]
#[inline(never)]
pub extern "C" fn byte_loop(a: u64, mut n: u32) -> u32 {
    let mut byte = a as i8;
    while n != 0 {
        byte >>= 1;
        n -= 1;
    }
    byte as u8 as u32
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
