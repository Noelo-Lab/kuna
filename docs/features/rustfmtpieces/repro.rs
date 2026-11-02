#[no_mangle]
#[inline(never)]
pub fn pieces_example(value: u32) { println!("value {} end", value); }
fn main() { pieces_example(std::hint::black_box(23)); }
