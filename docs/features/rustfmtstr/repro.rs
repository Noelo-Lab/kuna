#[no_mangle]
#[inline(never)]
pub fn make_string(value: u32, text: &str) -> String { format!("{}-{}", value, text) }
fn main() {
    println!("{}", make_string(7, "text"));
    println!("{}", format!("[{}]", std::hint::black_box("x")));
}
