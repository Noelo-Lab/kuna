#![no_std]
#![no_main]
#![allow(dead_code)]

#[panic_handler]
fn ph(_: &core::panic::PanicInfo) -> ! { loop {} }

static mut SEED: u32 = 3;
static mut SINK: u64 = 0;

// Two Result<u64,u64> objects. Guard is on `a` only; `b` is read in the SAME
// region (and, after inlining of the read, potentially at the same instruction).
#[no_mangle]
#[inline(never)]
pub unsafe extern "Rust" fn two_objs(a: *mut Result<u64, u64>, b: *mut Result<u64, u64>) -> u64 {
    if let Ok(v) = *a {
        // `b` is NOT guarded: it may be Ok or Err here.
        let w = match *b { Ok(t) => t, Err(t) => t };
        return v + w;
    }
    0
}

// Guard on `a`, write through `b` in the guarded block.
#[no_mangle]
#[inline(never)]
pub unsafe extern "Rust" fn guard_a_store_b(a: *mut Result<u64, u64>, b: *mut Result<u64, u64>, x: u64) {
    if let Ok(v) = *a {
        (*b) = Ok(v + x);
        let _ = core::ptr::read_volatile(b);
    }
}

// The SAME pointer read twice: guard on the tag, then read payload in a loop
// whose back edge re-enters with a different tag.
#[no_mangle]
#[inline(never)]
pub unsafe extern "Rust" fn loopy(a: *mut Result<u64, u64>, n: u32) -> u64 {
    let mut acc = 0u64;
    let mut i = 0u32;
    while i < n {
        if let Ok(v) = *a { acc += v; } else { acc += 1; }
        (*a) = if i & 1 == 0 { Err(i as u64) } else { Ok(i as u64) };
        i += 1;
    }
    acc
}

// Aliasing: the callee is handed the same object twice.
#[no_mangle]
#[inline(never)]
pub unsafe extern "Rust" fn alias(a: *mut Result<u64, u64>) -> u64 {
    two_objs(a, a)
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    unsafe {
        let s = core::ptr::read_volatile(core::ptr::addr_of!(SEED));
        let mut r1: Result<u64, u64> = Ok(s as u64);
        let mut r2: Result<u64, u64> = Err(1);
        let mut acc = two_objs(&mut r1, &mut r2);
        guard_a_store_b(&mut r1, &mut r2, s as u64);
        acc += loopy(&mut r1, s);
        acc += alias(&mut r2);
        core::ptr::write_volatile(core::ptr::addr_of_mut!(SINK), acc);
    }
    loop {}
}
