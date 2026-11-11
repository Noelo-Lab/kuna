#[no_mangle]
#[inline(never)]
pub fn write_value(out: &mut String, value: i64) {
    std::fmt::write(out, format_args!("[{}]", value)).unwrap();
}
fn main() {
    let mut out = String::from("prefix:");
    write_value(&mut out, std::hint::black_box(-42));
    println!("{out}");
}
