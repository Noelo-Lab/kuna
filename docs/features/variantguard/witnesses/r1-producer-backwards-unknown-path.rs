#![no_std]
#![no_main]
#![allow(dead_code)]
#[panic_handler]
fn ph(_: &core::panic::PanicInfo) -> ! { loop {} }
static mut SEED: u32 = 3;
static mut SINK: u64 = 0;

// UNKNOWN test: one forward path from the read reaches an Err store, the other
// RETURNS with no store at all.  Header claims that path contributes UNKNOWN and
// blocks the conclusion.
#[no_mangle]
#[inline(never)]
pub unsafe extern "Rust" fn cex_unknown_path(dst: *mut Result<u64, u64>, x: u32) -> u64 {
    let out = match *dst { Ok(v) => v, Err(e) => e.wrapping_add(100) };
    if x != 0 { *dst = Err(x as u64); }
    out
}

// DIVERGENCE test: the non-storing path is an infinite loop (size_out != 0, so
// it is never seeded UNKNOWN).
#[no_mangle]
#[inline(never)]
pub unsafe extern "Rust" fn cex_diverge(dst: *mut Result<u64, u64>, x: u32) -> u64 {
    let out = match *dst { Ok(v) => v, Err(e) => e.wrapping_add(100) };
    if x == 0 { loop { core::hint::spin_loop(); } }
    *dst = Err(x as u64);
    out
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    unsafe {
        let s = core::ptr::read_volatile(core::ptr::addr_of!(SEED));
        let mut r: Result<u64, u64> = Ok(0);
        let mut acc = cex_unknown_path(&mut r as *mut _, s);
        acc += cex_diverge(&mut r as *mut _, s);
        core::ptr::write_volatile(core::ptr::addr_of_mut!(SINK), acc);
    }
    loop {}
}
