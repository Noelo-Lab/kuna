#[no_mangle]
#[inline(never)]
pub fn traits(x: u32) {
    println!(
        "{} {:?} {:x} {:X} {:o} {:b} {:e} {:E}",
        x, x, x, x, x, x, x, x
    );
}
#[no_mangle]
#[inline(never)]
pub fn signed(x: i16) {
    eprintln!("{:+07} {:-5} {:#08x} {:#010b}", x, x, x, x);
}
#[no_mangle]
#[inline(never)]
pub fn align(x: i32) {
    println!("|{:·<8}|{:🦀^10}|{:>7.3}|", x, x, x);
}
#[no_mangle]
#[inline(never)]
pub fn debug_hex(x: u32) {
    println!("{:#x?} {:#X?}", x, x);
}
#[no_mangle]
#[inline(never)]
pub fn dynamic(x: u64, width: usize) {
    println!("{:0width$}", x);
}
#[no_mangle]
#[inline(never)]
pub fn generic(x: &str) {
    println!("{:>8}", x);
}
#[no_mangle]
#[inline(never)]
pub fn floating(x: f64) {
    println!("{:.3e}", x);
}
#[no_mangle]
#[inline(never)]
pub fn pointer(x: *const u8) {
    println!("{:p}", x);
}
#[no_mangle]
#[inline(never)]
pub fn escaped(x: u8) {
    println!("|{:\"<6}|{:\\>6}|{:\n^7}|{:\0<3}|", x, x, x, x);
}
#[no_mangle]
#[inline(never)]
pub fn reorder(x: u32, y: u16) {
    println!("{1:#08x} {0:+06} {1:>5o} {0:+06}", x, y);
}
#[no_mangle]
#[inline(never)]
pub fn narrow_signed(x: i8) {
    println!("{:+} {:#x} {:08b}", x, x, x);
}
#[no_mangle]
#[inline(never)]
pub fn narrow_unsigned(x: u16) {
    println!("{:#x} {:o}", x, x);
}
#[no_mangle]
#[inline(never)]
pub fn wide_signed(x: i64) {
    println!("{:+} {:#x}", x, x);
}
#[no_mangle]
#[inline(never)]
pub fn wide_unsigned(x: u64) {
    println!("{:#x} {:e}", x, x);
}
fn main() {
    traits(123456789);
    signed(-317);
    align(-9);
    debug_hex(0xabcdef);
    escaped(7);
    reorder(7, 99);
    narrow_signed(-127);
    narrow_unsigned(65535);
    wide_signed(i64::MIN + 1);
    wide_unsigned(u64::MAX);
    if std::hint::black_box(false) {
        dynamic(7, 8);
        generic("text");
        floating(1.25);
        pointer(std::ptr::null());
    }
}
