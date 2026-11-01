#[no_mangle]
#[inline(never)]
pub fn panic_literal() -> ! { panic!("bad {{value}}\n"); }
fn main() { if std::hint::black_box(false) { panic_literal(); } }
