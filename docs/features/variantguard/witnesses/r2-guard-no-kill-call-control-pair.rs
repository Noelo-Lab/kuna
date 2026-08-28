#![no_std]
#![no_main]
#![allow(dead_code)]
#[panic_handler]
fn ph(_: &core::panic::PanicInfo) -> ! { loop {} }
static mut SEED: u32 = 3;
static mut SINK: u64 = 0;

#[no_mangle]
#[inline(never)]
pub unsafe extern "Rust" fn set_err(dst: *mut Result<u64, u64>, x: u64) {
    core::ptr::write_volatile(dst, Err(x));
}
#[no_mangle]
#[inline(never)]
pub unsafe extern "Rust" fn set_ok(dst: *mut Result<u64, u64>, x: u64) {
    core::ptr::write_volatile(dst, Ok(x));
}

// The CONTROL PAIR: byte-identical but for which arm the guard selects. The
// same `mov 0x8(%rdi),%rax` must not get opposite variant names.
#[no_mangle]
#[inline(never)]
pub unsafe extern "Rust" fn read_after_call_ok_guard(dst: *mut Result<u64, u64>, x: u64) -> u64 {
    if let Ok(_) = *dst {
        set_err(dst, x);
        return core::ptr::read_volatile((dst as *const u64).add(1));
    }
    0
}
#[no_mangle]
#[inline(never)]
pub unsafe extern "Rust" fn read_after_call_err_guard(dst: *mut Result<u64, u64>, x: u64) -> u64 {
    if let Err(_) = *dst {
        set_ok(dst, x);
        return core::ptr::read_volatile((dst as *const u64).add(1));
    }
    0
}

// The explicit DISAGREE shape: tag=0; payload=v; tag=1; in one block.
#[no_mangle]
#[inline(never)]
pub unsafe extern "Rust" fn disagree_same_block(p: *mut Result<u64, u64>, x: u64) {
    *(p as *mut u32) = 0;
    *(p as *mut u64).add(1) = x;
    *(p as *mut u32) = 1;
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    unsafe {
        let s = core::ptr::read_volatile(core::ptr::addr_of!(SEED));
        let mut r: Result<u64, u64> = if s > 3 { Ok(s as u64) } else { Err(s as u64) };
        let mut acc = read_after_call_ok_guard(&mut r as *mut _, s as u64);
        acc += read_after_call_err_guard(&mut r as *mut _, s as u64);
        disagree_same_block(&mut r as *mut _, s as u64);
        core::ptr::write_volatile(core::ptr::addr_of_mut!(SINK), acc);
    }
    loop {}
}
