//! Formatting through the exact Write implementation for a Stdout reference.

use std::fmt;
use std::io::{self, Write};

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
pub fn write_unsigned(out: &mut &io::Stdout, value: u32) -> io::Result<()> {
    out.write_fmt(format_args!("[{}]", value))
}

#[no_mangle]
#[inline(never)]
pub fn write_signed(out: &mut &io::Stdout, value: i64) -> io::Result<()> {
    out.write_fmt(format_args!("{}\n", value))
}

#[no_mangle]
#[inline(never)]
pub fn write_flags(out: &mut &io::Stdout, value: u32) -> io::Result<()> {
    out.write_fmt(format_args!("{0:#010x}/{0:·>12}\n", value))
}

#[no_mangle]
#[inline(never)]
pub fn write_literal(out: &mut &io::Stdout) -> io::Result<()> {
    out.write_fmt(runtime_arguments(format_args!("{{é🙂}}\0\n\n")))
}

#[no_mangle]
#[inline(never)]
pub fn write_empty(out: &mut &io::Stdout) -> io::Result<()> {
    out.write_fmt(runtime_arguments(format_args!("")))
}

#[no_mangle]
#[inline(never)]
pub fn write_string(out: &mut &io::Stdout, text: &str) -> io::Result<()> {
    out.write_fmt(format_args!("|{0:🦀^10.3}|", text))
}

#[no_mangle]
#[inline(never)]
pub fn write_pair(out: &mut &io::Stdout, value: u32, text: &str) -> io::Result<()> {
    out.write_fmt(format_args!("{}: {}", value, text))
}

#[no_mangle]
#[inline(never)]
pub fn write_dynamic(out: &mut &io::Stdout, value: u32, width: usize) -> io::Result<()> {
    out.write_fmt(format_args!("{:width$}", value))
}
#[no_mangle]
#[inline(never)]
pub fn write_unknown(out: &mut dyn io::Write, value: u32) -> io::Result<()> {
    out.write_fmt(format_args!("{}", value))
}
#[no_mangle]
#[inline(never)]
pub fn write_vec(out: &mut Vec<u8>, value: u32) -> io::Result<()> {
    out.write_fmt(format_args!("{}", value))
}
fn main() {
    let stdout = io::stdout();
    let mut out = &stdout;
    for text in ["", "é🙂🦀", "a\0b\n{}"] {
        for result in [write_unsigned(&mut out,u32::MAX),write_signed(&mut out,i64::MIN),write_flags(&mut out,42),write_literal(&mut out),write_empty(&mut out),write_string(&mut out,text),write_pair(&mut out,23,text)] {
            eprintln!("{}",result.is_err());
        }
    }
    if std::hint::black_box(false) {
        let _ = write_dynamic(&mut out,7,8);
        let _ = write_unknown(&mut out,7);
        let _ = write_vec(&mut Vec::new(),7);
    }
}
