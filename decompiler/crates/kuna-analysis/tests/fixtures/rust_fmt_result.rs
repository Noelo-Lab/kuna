#[no_mangle]
#[inline(never)]
pub fn hex_width(x: u32) -> String {
    format!("0x{:08x} {:>5}", x, x)
}
#[no_mangle]
#[inline(never)]
pub fn decimal(x: i64) -> String {
    format!("value {}\n", x)
}
#[no_mangle]
#[inline(never)]
pub fn reordered(x: u16, y: i16) -> String {
    format!("{0:#06x}/{1:+}/{0:#06x}", x, y)
}
#[no_mangle]
#[inline(never)]
pub fn constant_value(x: u16) -> String {
    format!("{0:#06x}/{1:+}/{0:#06x}", x, -7i16)
}
#[no_mangle]
#[inline(never)]
pub fn local(x: u32) -> usize {
    let text = format!("n={}", x);
    text.len()
}
#[no_mangle]
#[inline(never)]
pub fn generic(x: &str) -> String {
    format!("text {}", x)
}
#[no_mangle]
#[inline(never)]
pub fn dynamic(x: u64, width: usize) -> String {
    format!("{:0width$}", x)
}
#[no_mangle]
#[inline(never)]
pub fn floating(x: f64) -> String {
    format!("{:.2}", x)
}
fn main() {
    println!("{:?}", hex_width(31));
    println!("{:?}", decimal(-123));
    println!("{:?}", reordered(99, -7));
    if std::hint::black_box(false) {
        let _ = local(1234);
        drop(constant_value(99));
        drop(generic("abc"));
        drop(dynamic(7, 8));
        drop(floating(1.25));
    }
}
