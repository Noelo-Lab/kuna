#![no_std]
#![no_main]
#![allow(dead_code)]
#[panic_handler]
fn ph(_: &core::panic::PanicInfo) -> ! { loop {} }
static mut SEED: u32 = 3;
static mut SINK: u64 = 0;

// NZ-1: no calls at all.  Guard proves Ok; INSIDE the Ok arm the object is
// overwritten with Err(7) and the function returns.  The store is in the
// GUARDED block, not the join block -- which is the only structural difference
// from the shipped `guard_then_clobber` fixture.
#[no_mangle]
#[inline(never)]
pub unsafe extern "Rust" fn overwrite_in_arm(dst: *mut Result<u64,u64>) -> u64 {
    match *dst {
        Ok(a) => { *dst = Err(7); a }
        Err(_) => 0,
    }
}

// NZ-2: same, and then READ the payload back inside the same arm.
#[no_mangle]
#[inline(never)]
pub unsafe extern "Rust" fn overwrite_then_read_in_arm(dst: *mut Result<u64,u64>, x: u32) -> u64 {
    match *dst {
        Ok(_) => {
            *dst = Err(x as u64);
            let v = core::ptr::read_volatile(dst);
            match v { Ok(a) => a, Err(b) => b }
        }
        Err(_) => 0,
    }
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    unsafe {
        let s = core::ptr::read_volatile(core::ptr::addr_of!(SEED));
        let mut r: Result<u64,u64> = Ok(s as u64);
        let mut acc = overwrite_in_arm(&mut r as *mut _);
        acc += overwrite_then_read_in_arm(&mut r as *mut _, s);
        core::ptr::write_volatile(core::ptr::addr_of_mut!(SINK), acc);
    }
    loop {}
}
