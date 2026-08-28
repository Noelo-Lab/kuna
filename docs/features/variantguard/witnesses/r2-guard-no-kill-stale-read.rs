#![no_std]
#![no_main]
#![allow(dead_code)]
#[panic_handler]
fn ph(_: &core::panic::PanicInfo) -> ! { loop {} }
static mut SEED: u32 = 3;
static mut SINK: u64 = 0;

// C1: a READ under a STALE guard. The volatile payload read cannot be folded,
// so the load survives; the tag was overwritten to Err above it.
#[no_mangle]
#[inline(never)]
pub unsafe extern "Rust" fn stale_read(dst: *mut Result<u64, u64>, x: u64) -> u64 {
    if let Ok(_) = *dst {
        *dst = Err(x);
        return core::ptr::read_volatile((dst as *const u64).add(1));
    }
    0
}

// C2: same, but the clobber is done by a call the analysis CAN see, then a read.
#[no_mangle]
#[inline(never)]
pub unsafe extern "Rust" fn set_err(dst: *mut Result<u64, u64>, x: u64) {
    core::ptr::write_volatile(dst, Err(x));
}

#[no_mangle]
#[inline(never)]
pub unsafe extern "Rust" fn stale_read_call(dst: *mut Result<u64, u64>, x: u64) -> u64 {
    if let Ok(_) = *dst {
        set_err(dst, x);
        return core::ptr::read_volatile((dst as *const u64).add(1));
    }
    0
}
#[no_mangle]
pub extern "C" fn _start() -> ! {
    unsafe {
        let s = core::ptr::read_volatile(core::ptr::addr_of!(SEED));
        let mut r: Result<u64, u64> = if s > 3 { Ok(s as u64) } else { Err(s as u64) };
        let mut acc = stale_read(&mut r as *mut _, s as u64);
        acc += stale_read_call(&mut r as *mut _, s as u64);
        core::ptr::write_volatile(core::ptr::addr_of_mut!(SINK), acc);
    }
    loop {}
}
