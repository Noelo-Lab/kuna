#![no_std]
pub mod rust_writer {
    #[inline(never)]
    pub fn write(a: u64, b: u64, c: u64, d: u64) -> u64 {
        a.wrapping_add(b).wrapping_add(c).wrapping_add(d)
    }
}
#[no_mangle]
#[inline(never)]
pub extern "C" fn call_write(a: u64, b: u64, c: u64, d: u64) -> u64 {
    rust_writer::write(a.wrapping_add(1), b.wrapping_add(2), c.wrapping_add(3), d.wrapping_add(4))
}
