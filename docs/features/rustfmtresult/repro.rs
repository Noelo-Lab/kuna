#[no_mangle]
#[inline(never)]
pub fn hex_width(x: u32) -> String { format!("0x{:08x} {:>5}", x, x) }
#[no_mangle]
#[inline(never)]
pub fn decimal(x: i64) -> String { format!("value {}\n", x) }
fn main() { println!("{}", hex_width(31)); print!("{}", decimal(-123)); }
