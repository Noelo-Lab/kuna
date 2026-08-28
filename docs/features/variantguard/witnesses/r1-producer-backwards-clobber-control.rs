#![no_std]
#![no_main]
#![allow(dead_code)]

#[panic_handler]
fn ph(_: &core::panic::PanicInfo) -> ! { loop {} }

static mut SEED: u32 = 3;
static mut SINK: u64 = 0;

// ADVERSARIAL 1: read the payload of an UNKNOWN Result, then clobber the object
// with Err(7).  The read is dominated by nothing and its object is later
// stored a constant tag 1 -- but the value read may well have been an `Ok`.
#[no_mangle]
#[inline(never)]
pub unsafe extern "Rust" fn read_then_clobber(dst: *mut Result<u64, u64>) -> u64 {
    let v = match *dst { Ok(a) => a, Err(b) => b };
    *dst = Err(7);
    v
}

// ADVERSARIAL 2: same, but the clobber is Ok(9).
#[no_mangle]
#[inline(never)]
pub unsafe extern "Rust" fn read_then_clobber_ok(dst: *mut Result<u64, u64>) -> u64 {
    let v = match *dst { Ok(a) => a, Err(b) => b };
    *dst = Ok(9);
    v
}

// ADVERSARIAL 3: guard on the tag, then INSIDE the Ok arm read the payload and
// also clobber to Err.  Region intersection must be empty (contradiction).
#[no_mangle]
#[inline(never)]
pub unsafe extern "Rust" fn guard_then_clobber(dst: *mut Result<u64, u64>) -> u64 {
    let v = match *dst { Ok(a) => a, Err(_) => 0 };
    *dst = Err(v + 1);
    v
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    unsafe {
        let s = core::ptr::read_volatile(core::ptr::addr_of!(SEED));
        let mut r: Result<u64, u64> = if s > 3 { Ok(s as u64) } else { Err(s as u64) };
        let mut acc = read_then_clobber(&mut r as *mut Result<u64, u64>);
        let mut r2: Result<u64, u64> = if s > 4 { Ok(1) } else { Err(2) };
        acc += read_then_clobber_ok(&mut r2 as *mut Result<u64, u64>);
        let mut r3: Result<u64, u64> = if s > 5 { Ok(3) } else { Err(4) };
        acc += guard_then_clobber(&mut r3 as *mut Result<u64, u64>);
        core::ptr::write_volatile(core::ptr::addr_of_mut!(SINK), acc);
    }
    loop {}
}
