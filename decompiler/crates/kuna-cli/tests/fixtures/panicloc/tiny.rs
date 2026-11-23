#![no_std]
#![no_main]
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
#[inline(never)]
pub extern "C" fn pick(v: *const u32, n: usize, i: usize) -> u32 {
    let s = unsafe { core::slice::from_raw_parts(v, n) };
    if i == 7 {
        panic!("seven is not allowed");
    }
    s[i]
}

#[no_mangle]
pub extern "C" fn rust_eh_personality() {}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let a = [1u32, 2, 3];
    let i = unsafe { core::ptr::read_volatile(&5usize) };
    let r = pick(a.as_ptr(), 3, i);
    unsafe { core::ptr::write_volatile(0x1000 as *mut u32, r) };
    loop {}
}
