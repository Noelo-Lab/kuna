#![no_std]
#![no_main]
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
#[inline(never)]
pub extern "C" fn sink(p: *const u8, n: usize) {
    unsafe { core::ptr::write_volatile(0x1000 as *mut usize, p as usize + n) };
}

#[no_mangle]
#[inline(never)]
pub extern "C" fn greet() {
    sink("hello there".as_ptr(), 11);
    sink("general kenobi".as_ptr(), 14);
}

#[no_mangle]
#[inline(never)]
pub extern "C" fn sink_ptr(p: *const u8) {
    unsafe { core::ptr::write_volatile(0x1008 as *mut usize, p as usize) };
}

#[no_mangle]
#[inline(never)]
pub extern "C" fn keep() {
    sink_ptr("hello there".as_ptr());
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    greet();
    keep();
    loop {}
}
