#![no_std]
#![no_main]

#[panic_handler]
fn ph(_: &core::panic::PanicInfo) -> ! { loop {} }

static mut SEED: u32 = 3;
static mut SINK: u32 = 0;

#[no_mangle]
#[inline(never)]
pub extern "Rust" fn prod(x: u32) -> Result<u32, u32> {
    if x > 10 { Ok(x * 2) } else { Err(x + 7) }
}

#[no_mangle]
#[inline(never)]
pub extern "C" fn cons(x: u32) -> u32 {
    match prod(x) {
        Ok(v) => v + 1,
        Err(e) => e + 100,
    }
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    unsafe {
        let s = core::ptr::read_volatile(core::ptr::addr_of!(SEED));
        core::ptr::write_volatile(core::ptr::addr_of_mut!(SINK), cons(s));
    }
    loop {}
}
