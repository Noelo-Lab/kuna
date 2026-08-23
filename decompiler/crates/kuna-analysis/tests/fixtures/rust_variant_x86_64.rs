#![no_std]
#![no_main]

#[panic_handler]
fn ph(_: &core::panic::PanicInfo) -> ! { loop {} }

static mut SEED: u32 = 3;
static mut SINK: u32 = 0;

// Three return paths, the tag a literal on each: the two-variant tagged case
// `option rustadt` recognizes.  `branchy` returns Ok on two of them and Err on
// the third; `samevariant` has the same two-register return shape with ONE tag
// value, so the rule must refuse it.
#[no_mangle]
#[inline(never)]
pub extern "Rust" fn branchy(x: u32, y: u32) -> Result<u32, u32> {
    if x < 11 {
        return Ok(x.wrapping_add(y).wrapping_mul(3));
    }
    if y > 100 {
        return Ok(y - 1);
    }
    Err(x ^ y)
}

#[no_mangle]
#[inline(never)]
pub extern "Rust" fn samevariant(x: u32, y: u32) -> Result<u32, u32> {
    if x < 11 {
        return Ok(x.wrapping_add(y));
    }
    Ok(y.wrapping_sub(1))
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    unsafe {
        let s = core::ptr::read_volatile(core::ptr::addr_of!(SEED));
        let a = match branchy(s, s + 1) { Ok(v) => v, Err(e) => e };
        let b = match samevariant(s, s + 2) { Ok(v) => v, Err(e) => e };
        core::ptr::write_volatile(core::ptr::addr_of_mut!(SINK), a ^ b);
    }
    loop {}
}
