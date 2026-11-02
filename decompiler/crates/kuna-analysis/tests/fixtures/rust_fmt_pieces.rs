use std::hint::black_box;

#[no_mangle]
#[inline(never)]
pub fn pieces_literal() {
    println!("literal {{é🙂}}\n");
}

#[no_mangle]
#[inline(never)]
pub fn pieces_plain() {
    print!("quote=\" slash=\\ nul=\0 {{}} ");
}

#[no_mangle]
#[inline(never)]
pub fn pieces_stderr() {
    eprintln!("stderr\r");
}

#[no_mangle]
#[inline(never)]
pub fn pieces_empty() {
    print!("");
}

#[no_mangle]
#[inline(never)]
pub fn pieces_u32(a: u32) {
    println!("value {} end", a);
}

#[no_mangle]
#[inline(never)]
pub fn pieces_i64(a: i64) {
    print!("{}", a);
}

#[no_mangle]
#[inline(never)]
pub fn pieces_pair(a: i8, b: u64) {
    eprint!("{} = {}", a, b);
}

#[no_mangle]
#[inline(never)]
pub fn pieces_narrow(a: u8, b: i16, c: u16, d: i32) {
    println!("[{}|{}|{}|{}]", a, b, c, d);
}

#[no_mangle]
#[inline(never)]
pub fn pieces_repeat(a: u32, b: i32) {
    println!("{1} {0} {1}", a, b);
}

#[no_mangle]
#[inline(never)]
pub fn pieces_adjacent(a: u64, b: i64) {
    println!("{}{}", a, b);
}

#[no_mangle]
#[inline(never)]
pub fn pieces_flags(a: u32) {
    println!("{:08x}", a);
}

#[no_mangle]
#[inline(never)]
pub fn pieces_dynamic(a: u32, width: usize) {
    println!("{:width$}", a, width = width);
}

#[no_mangle]
#[inline(never)]
pub fn pieces_generic(a: &str) {
    println!("{}", a);
}

#[no_mangle]
#[inline(never)]
pub fn pieces_float(a: f64) {
    println!("{}", a);
}

#[no_mangle]
#[inline(never)]
pub fn pieces_debug(a: u32) {
    println!("{:?}", a);
}

fn main() {
    pieces_literal();
    pieces_plain();
    pieces_stderr();
    pieces_empty();
    pieces_u32(black_box(23));
    pieces_i64(black_box(i64::MIN));
    pieces_pair(black_box(-7), black_box(u64::MAX));
    pieces_narrow(
        black_box(255),
        black_box(-32768),
        black_box(65535),
        black_box(i32::MIN),
    );
    pieces_adjacent(black_box(11), black_box(-22));
    if black_box(false) {
        pieces_repeat(23, -7);
        pieces_flags(3);
        pieces_dynamic(3, 5);
        pieces_generic("value");
        pieces_float(1.25);
        pieces_debug(9);
    }
}
