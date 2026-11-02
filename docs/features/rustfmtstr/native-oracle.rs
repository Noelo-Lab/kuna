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
    std::panic::set_hook(Box::new(|_|{}));
    let mut strings:Vec<String>=(0u8..=127).map(|c|String::from(c as char)).collect();
    strings.extend(["", "text", "é🙂🦀", "a\0b\n{}\\\"", "\r\t", "e\u{301}", "👩\u{200d}💻", "مرحبا", "日本語"].into_iter().map(String::from));
    for c in [0x80,0x7ff,0x800,0xd7ff,0xe000,0xffff,0x10000,0x10ffff] {strings.push(char::from_u32(c).unwrap().to_string());}
    for size in [2,3,7,8,9,16,31,32,255,256,4096] {strings.push("é{}\0🦀".repeat(size));}
    let values=[0u32,1,7,127,255,256,0x7fffffff,0x80000000,u32::MAX];
    if std::env::args().nth(1).as_deref()==Some("panic") {
        for text in &strings {
            let error=std::panic::catch_unwind(||str_panic(text)).unwrap_err();
            println!("{:?}",error.downcast_ref::<String>().unwrap());
        }
        return;
    }
    for text in &strings {
        str_plain(text);str_line(text);str_stderr(text);str_flags(text);str_pair(text,"right é🦀\0\n");
        println!("{:?}",str_result(text));
        for &value in &values {str_stderr_pair(value,text);println!("{:?}",str_integer(value,text));}
    }
    if std::hint::black_box(false) {
        str_custom("custom");str_debug("debug");str_dynamic("dynamic",8);str_reference(&7);str_owned(&String::from("owned"));
    }
}
