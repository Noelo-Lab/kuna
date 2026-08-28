#![no_std]
#![no_main]
#![allow(dead_code)]

#[panic_handler]
fn ph(_: &core::panic::PanicInfo) -> ! { loop {} }

static mut SEED: u32 = 3;
static mut SINK: u64 = 0;

// A1: a WRITE that builds Err, sitting inside a block the guard says is Ok.
#[no_mangle]
#[inline(never)]
pub unsafe extern "Rust" fn guarded_write(dst: *mut Result<u64, u64>) -> u64 {
    if let Ok(a) = *dst {
        *dst = Err(a + 1);
        return a;
    }
    0
}

// A1b: the mirror. Same instruction shape, guard says Err, write builds Ok.
#[no_mangle]
#[inline(never)]
pub unsafe extern "Rust" fn guarded_write_mirror(dst: *mut Result<u64, u64>) -> u64 {
    if let Err(a) = *dst {
        *dst = Ok(a + 1);
        return a;
    }
    0
}

// A3: an aliasing store the kill rule cannot see (q may equal p).
#[no_mangle]
#[inline(never)]
pub unsafe extern "Rust" fn alias_clobber(
    p: *mut Result<u64, u64>,
    q: *mut Result<u64, u64>,
    x: u64,
) {
    *p = Ok(1);
    *q = Err(2);
    // raw payload write into p, no tag store of its own
    *(p as *mut u64).add(1) = x;
}

// A4: the write is in a block dominated by two different tag stores.
#[no_mangle]
#[inline(never)]
pub unsafe extern "Rust" fn split_tag(p: *mut Result<u64, u64>, c: u32, x: u64) {
    if c != 0 {
        *p = Ok(0);
    } else {
        *p = Err(0);
    }
    *(p as *mut u64).add(1) = x;
}

// A5: the disagreeing second store is in a SUCCESSOR block.
#[no_mangle]
#[inline(never)]
pub unsafe extern "Rust" fn disagree_succ(p: *mut Result<u64, u64>, c: u32, x: u64) {
    *p = Ok(x);
    if c != 0 {
        *(p as *mut u32) = 1;
    }
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    unsafe {
        let s = core::ptr::read_volatile(core::ptr::addr_of!(SEED));
        let mut r: Result<u64, u64> = if s > 3 { Ok(s as u64) } else { Err(s as u64) };
        let mut r2: Result<u64, u64> = if s > 4 { Ok(1) } else { Err(2) };
        let mut acc = guarded_write(&mut r as *mut _);
        acc += guarded_write_mirror(&mut r2 as *mut _);
        alias_clobber(&mut r as *mut _, &mut r2 as *mut _, s as u64);
        split_tag(&mut r as *mut _, s, s as u64);
        disagree_succ(&mut r2 as *mut _, s, s as u64);
        core::ptr::write_volatile(core::ptr::addr_of_mut!(SINK), acc);
    }
    loop {}
}
