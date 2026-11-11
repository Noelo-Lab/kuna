#[inline(never)]
#[no_mangle]
pub fn print_one(value: u64) {
    println!("value = {}", value);
}
#[inline(never)]
#[no_mangle]
pub fn reorder(first: u32, second: u32) {
    println!("{1} {0} {1}", first, second);
}
#[inline(never)]
#[no_mangle]
pub fn signed(value: i64) {
    eprintln!("signed {}", value);
}
#[inline(never)]
#[no_mangle]
pub fn exponent(value: u64) {
    println!("scientific {:e}", value);
}
#[inline(never)]
#[no_mangle]
pub fn debug(value: u64) {
    println!("debug {:?}", value);
}
#[inline(never)]
#[no_mangle]
pub fn width(value: u64) {
    println!("width {:08}", value);
}
fn main() {
    let controls = std::env::args().any(|arg| arg == "--controls");
    print_one(123456789);
    reorder(99, 7);
    signed(-9223372036854775807);
    if controls {
        exponent(123456789);
        debug(123456789);
        width(23);
    }
}
