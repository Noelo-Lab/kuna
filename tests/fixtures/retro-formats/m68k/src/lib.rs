#![no_std]
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub static mut counter: i32 = 7;
#[no_mangle]
pub static mut table: [i32; 4] = [1, 2, 3, 4];
#[no_mangle]
pub static mut scratch: i32 = 0;
#[no_mangle]
pub static mut handler: unsafe extern "C" fn(i32) -> i32 = scale;

#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn checksum(text: *const u8, count: i32) -> i32 {
    let mut total = 0i32;
    let mut index = 0;
    while index < count {
        total = total.wrapping_mul(31).wrapping_add(*text.offset(index as isize) as i32);
        index += 1;
    }
    total
}

#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn scale(x: i32) -> i32 {
    x.wrapping_mul(3).wrapping_add(counter)
}

#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn sum(a: i32, b: i32) -> i32 {
    scratch = a;
    handler(a).wrapping_add(table[(b & 3) as usize])
}

#[no_mangle]
pub unsafe extern "C" fn main() -> i32 {
    sum(counter, checksum(b"retro".as_ptr(), 5)).wrapping_add(scratch)
}
