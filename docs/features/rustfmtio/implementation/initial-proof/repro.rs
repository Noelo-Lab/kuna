use std::io::{self, Write};
macro_rules! writer { ($name:ident, $fmt:literal) => {
    #[no_mangle] #[inline(never)]
    pub fn $name(out: &mut &io::Stdout, n: u32) -> io::Result<()> {
        out.write_fmt(format_args!($fmt, n))
    }
}; }
writer!(write_value, "v: {}\n"); writer!(write_hex, "{:#x}");
writer!(write_binary, "{:#b}"); writer!(write_debug, "{:?}");
fn main() {
    let stdout = io::stdout();
    for write in [write_value, write_hex, write_binary, write_debug] {
        write(&mut &stdout, std::hint::black_box(42)).unwrap();
    }
}
