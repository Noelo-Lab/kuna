#![no_std]
#![no_main]
#![allow(dead_code)]

#[panic_handler]
fn ph(_: &core::panic::PanicInfo) -> ! { loop {} }

static mut SEED: u32 = 3;
static mut SINK: u64 = 0;

// CEX-1: an ambiguous payload read that HAPPENS ABOVE an unconditional
// `*dst = Err(..)` store.  The store says nothing about the value that was read
// BEFORE it, but the backward producer analysis has no ordering / kill.
#[no_mangle]
#[inline(never)]
pub unsafe extern "Rust" fn cex_read_then_store(dst: *mut Result<u64, u64>, x: u32) -> u64 {
    let out = match *dst { Ok(v) => v, Err(e) => e.wrapping_add(100) };
    *dst = Err(x as u64);
    out
}

// CEX-2: a read the SOURCE proves is the Ok payload, above an Err store.
#[no_mangle]
#[inline(never)]
pub unsafe extern "Rust" fn cex_ok_then_err(dst: *mut Result<u64, u64>, x: u32) -> u64 {
    let v = match *dst { Ok(v) => v, Err(_) => 0 };
    *dst = Err(x as u64);
    v
}

// CEX-3: the same, other way round: read Err payload, then store Ok.
#[no_mangle]
#[inline(never)]
pub unsafe extern "Rust" fn cex_err_then_ok(dst: *mut Result<u64, u64>, x: u32) -> u64 {
    let v = match *dst { Err(e) => e, Ok(_) => 0 };
    *dst = Ok(x as u64);
    v
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    unsafe {
        let s = core::ptr::read_volatile(core::ptr::addr_of!(SEED));
        let mut r: Result<u64, u64> = Ok(0);
        let mut acc = cex_read_then_store(&mut r as *mut _, s);
        acc += cex_ok_then_err(&mut r as *mut _, s);
        acc += cex_err_then_ok(&mut r as *mut _, s);
        core::ptr::write_volatile(core::ptr::addr_of_mut!(SINK), acc);
    }
    loop {}
}
