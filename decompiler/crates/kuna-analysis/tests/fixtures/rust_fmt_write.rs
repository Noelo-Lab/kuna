//! Formatting calls with an observable error branch and a String writer.

use std::fmt;
use std::sync::atomic::{AtomicUsize, Ordering};

/// Keep literal calls visible while preserving both native argument words.
#[inline(always)]
fn runtime_arguments(mut arguments: fmt::Arguments<'_>) -> fmt::Arguments<'_> {
    assert!(std::mem::size_of_val(&arguments) >= 16);
    unsafe {
        let words = (&mut arguments as *mut fmt::Arguments<'_>).cast::<usize>();
        let mut first = words.read();
        let mut second = words.add(1).read();
        std::arch::asm!("/* {first} {second} */", first = inout(reg) first, second = inout(reg) second,
            options(nomem, nostack, preserves_flags));
        words.write(first);
        words.add(1).write(second);
    }
    arguments
}

#[no_mangle]
#[inline(never)]
pub fn write_unsigned(out: &mut String, value: u32) -> fmt::Result {
    fmt::write(out, format_args!("[{}]", value)).unwrap();
    Ok(())
}

#[no_mangle]
#[inline(never)]
pub fn write_signed(out: &mut String, value: i64) -> fmt::Result {
    fmt::write(out, format_args!("{}\n", value)).unwrap();
    Ok(())
}

#[no_mangle]
#[inline(never)]
pub fn write_flags(out: &mut String, value: u32) -> fmt::Result {
    fmt::write(out, format_args!("{0:#010x}/{0:·>12}\n", value)).unwrap();
    Ok(())
}

#[no_mangle]
#[inline(never)]
pub fn write_literal(out: &mut String) -> fmt::Result {
    fmt::write(out, runtime_arguments(format_args!("{{é🙂}}\0\n\n"))).unwrap();
    Ok(())
}

#[no_mangle]
#[inline(never)]
pub fn write_empty(out: &mut String) -> fmt::Result {
    fmt::write(out, runtime_arguments(format_args!(""))).unwrap();
    Ok(())
}

#[no_mangle]
#[inline(never)]
pub fn write_string(out: &mut String, text: &str) -> fmt::Result {
    fmt::write(out, format_args!("|{0:🦀^10.3}|", text)).unwrap();
    Ok(())
}

#[no_mangle]
#[inline(never)]
pub fn write_pair(out: &mut String, value: u32, text: &str) -> fmt::Result {
    fmt::write(out, format_args!("{}: {}", value, text)).unwrap();
    Ok(())
}

static CALLS: AtomicUsize = AtomicUsize::new(0);
struct Counted(u32);
impl fmt::Display for Counted {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        CALLS.fetch_add(1, Ordering::Relaxed);
        write!(formatter, "{}", self.0)
    }
}

struct Reject;
impl fmt::Write for Reject {
    fn write_str(&mut self, _: &str) -> fmt::Result {
        Err(fmt::Error)
    }
}

#[no_mangle]
#[inline(never)]
pub fn write_custom(out: &mut String, value: u32) -> fmt::Result {
    fmt::write(out, format_args!("{}", Counted(value)))
}

#[no_mangle]
#[inline(never)]
pub fn write_dynamic(out: &mut String, value: u32, width: usize) -> fmt::Result {
    fmt::write(out, format_args!("{:width$}", value))
}

#[no_mangle]
#[inline(never)]
pub fn write_unknown(out: &mut dyn fmt::Write, value: u32) -> fmt::Result {
    fmt::write(out, format_args!("{}", value))
}

#[no_mangle]
#[inline(never)]
pub fn write_reject(value: u32) -> fmt::Result {
    fmt::write(&mut Reject, format_args!("{}", value))
}

fn main() {
    for text in ["", "é🙂🦀", "a\0b\n{}"] {
        let mut out = String::from("prefix:");
        let result = write_unsigned(&mut out, u32::MAX);
        println!("{} {out:?}", result.is_err() as u8);
        let result = write_signed(&mut out, i64::MIN);
        println!("{} {out:?}", result.is_err() as u8);
        let result = write_flags(&mut out, 42);
        println!("{} {out:?}", result.is_err() as u8);
        let result = write_literal(&mut out);
        println!("{} {out:?}", result.is_err() as u8);
        let result = write_empty(&mut out);
        println!("{} {out:?}", result.is_err() as u8);
        let result = write_string(&mut out, text);
        println!("{} {out:?}", result.is_err() as u8);
        let result = write_pair(&mut out, 23, text);
        println!("{} {out:?}", result.is_err() as u8);
    }
    if std::hint::black_box(false) {
        let mut out = String::new();
        let _ = write_custom(&mut out, 7);
        let _ = write_dynamic(&mut out, 7, 8);
        let _ = write_unknown(&mut out, 7);
        let _ = write_reject(7);
    }
}
