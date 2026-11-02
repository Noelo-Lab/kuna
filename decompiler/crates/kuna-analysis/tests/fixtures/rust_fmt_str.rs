use std::fmt;
use std::sync::atomic::{AtomicUsize, Ordering};

#[no_mangle]
#[inline(never)]
pub fn str_plain(text: &str) { print!("{}", text); }

#[no_mangle]
#[inline(never)]
pub fn str_line(text: &str) { println!("{{é🙂}} {}\n", text); }

#[no_mangle]
#[inline(never)]
pub fn str_stderr(text: &str) { eprintln!("stderr {}", text); }

#[no_mangle]
#[inline(never)]
pub fn str_stderr_pair(value: u32, text: &str) { eprintln!("{}: {}", value, text); }

#[no_mangle]
#[inline(never)]
pub fn str_flags(text: &str) { println!("|{0:·<8}|{0:🦀^10.3}|{0:>7.3}|", text); }

#[no_mangle]
#[inline(never)]
pub fn str_pair(left: &str, right: &str) { println!("{} / {}", left, right); }

#[no_mangle]
#[inline(never)]
pub fn str_integer(value: u32, text: &str) -> String { format!("{}-{}", value, text) }

#[no_mangle]
#[inline(never)]
pub fn str_result(text: &str) -> String { format!("[{}]", text) }

#[no_mangle]
#[inline(never)]
pub fn str_panic(text: &str) -> ! { panic!("bad {}", text); }

static CALLS: AtomicUsize = AtomicUsize::new(0);
struct Counted<'a>(&'a str);
impl fmt::Display for Counted<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        CALLS.fetch_add(1, Ordering::Relaxed);
        formatter.pad(self.0)
    }
}

#[no_mangle]
#[inline(never)]
pub fn str_custom(text: &str) { println!("{}", Counted(text)); }

#[no_mangle]
#[inline(never)]
pub fn str_debug(text: &str) { println!("{:?}", text); }

#[no_mangle]
#[inline(never)]
pub fn str_dynamic(text: &str, width: usize) { println!("{:width$}", text); }

#[no_mangle]
#[inline(never)]
pub fn str_reference(value: &u64) { println!("{}", value); }

#[no_mangle]
#[inline(never)]
pub fn str_owned(text: &String) { println!("{}", text); }

fn main() {
    if std::env::args().nth(1).as_deref() == Some("panic") {
        std::panic::set_hook(Box::new(|_| {}));
        for text in ["", "é🙂", "a\0b\n{}"] {
            let error = std::panic::catch_unwind(|| str_panic(text)).unwrap_err();
            println!("{:?}", error.downcast_ref::<String>().unwrap());
        }
        return;
    }
    for text in ["", "text", "é🙂🦀", "a\0b\n{}\\\""] {
        str_plain(text);
        str_line(text);
        str_stderr(text);
        str_stderr_pair(23, text);
        str_flags(text);
        str_pair(text, "right é");
        println!("{:?}", str_integer(u32::MAX, text));
        println!("{:?}", str_result(text));
    }
    if std::hint::black_box(false) {
        str_custom("custom");
        str_debug("debug");
        str_dynamic("dynamic", 8);
        str_reference(&7);
        str_owned(&String::from("owned"));
        str_panic("panic");
    }
}
