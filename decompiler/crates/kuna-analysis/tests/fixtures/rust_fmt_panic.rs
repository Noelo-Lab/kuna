use std::hint::black_box;

#[no_mangle]
#[inline(never)]
pub fn panic_literal() -> ! {
    panic!("literal {{é🙂}}\n\n");
}
#[no_mangle]
#[inline(never)]
pub fn panic_empty() -> ! {
    panic!("");
}
#[no_mangle]
#[inline(never)]
pub fn panic_decimal(a: u64) -> ! {
    panic!("value={a}\n");
}
#[no_mangle]
#[inline(never)]
pub fn panic_signed(a: i64) -> ! {
    panic!("signed={a:+}");
}
#[no_mangle]
#[inline(never)]
pub fn panic_hex(a: u32) -> ! {
    panic!("hex={a:#010x}");
}
#[no_mangle]
#[inline(never)]
pub fn panic_reorder(a: u32, b: u16) -> ! {
    panic!("{1:#08x} {0:+06} {1:>5o} {0:+06}", a, b);
}
#[no_mangle]
#[inline(never)]
pub fn panic_narrow(a: i8) -> ! {
    panic!("{a:+} {a:#x} {a:08b}");
}
#[no_mangle]
#[inline(never)]
pub fn panic_escaped(a: u8) -> ! {
    panic!("{{|{a:\"<6}|{a:\\>6}|{a:\n^7}|{a:\0<3}|}}\n");
}
#[no_mangle]
#[inline(never)]
pub fn panic_dynamic(a: u64, width: usize) -> ! {
    panic!("{a:0width$}");
}
#[no_mangle]
#[inline(never)]
pub fn panic_generic(a: &str) -> ! {
    panic!("{a:>8}");
}
#[no_mangle]
#[inline(never)]
pub fn panic_float(a: f64) -> ! {
    panic!("{a:.3e}");
}
#[no_mangle]
#[inline(never)]
#[track_caller]
pub fn panic_forwarded(a: u64) -> ! {
    panic!("forwarded={a}");
}

fn capture(f: impl FnOnce() + std::panic::UnwindSafe) {
    let payload = std::panic::catch_unwind(f).unwrap_err();
    let text = payload
        .downcast_ref::<&str>()
        .copied()
        .unwrap_or_else(|| payload.downcast_ref::<String>().unwrap().as_str());
    println!("{text:?}");
}

fn main() {
    std::panic::set_hook(Box::new(|_| {}));
    capture(|| panic_literal());
    capture(|| panic_empty());
    capture(|| panic_decimal(black_box(u64::MAX)));
    capture(|| panic_signed(black_box(i64::MIN + 1)));
    capture(|| panic_hex(black_box(42)));
    capture(|| panic_reorder(black_box(7), black_box(99)));
    capture(|| panic_narrow(black_box(-127)));
    capture(|| panic_escaped(black_box(7)));
    if black_box(false) {
        capture(|| panic_dynamic(black_box(7), black_box(8)));
        capture(|| panic_generic(black_box("text")));
        capture(|| panic_float(black_box(1.25)));
        capture(|| panic_forwarded(black_box(7)));
    }
}
