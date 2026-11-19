// No-return Rust paths under v0 mangling: core::option::unwrap_failed,
// core::slice::index::slice_index_fail and the panic_const! family.
#![no_std]
#![no_main]
use core::panic::PanicInfo;
#[no_mangle] pub extern "C" fn rust_eh_personality() {}
#[panic_handler]
fn on_panic(_: &PanicInfo) -> ! { loop {} }
#[inline(never)] #[no_mangle] pub extern "C" fn opt(x: u32) -> u32 { core::hint::black_box(if x > 3 { None } else { Some(x) }).unwrap() + 1 }
#[inline(never)] #[no_mangle] pub extern "C" fn sum(p: *const u8, len: usize, n: usize) -> u32 { let v = unsafe { core::slice::from_raw_parts(p, len) }; v[..n].iter().map(|&b| b as u32).sum() }
#[inline(never)] #[no_mangle] pub extern "C" fn add(a: u32, b: u32) -> u32 { a + b }
#[no_mangle] pub extern "C" fn _start() -> ! { let b = [1u8, 2, 3]; let r = opt(core::hint::black_box(2)) + sum(b.as_ptr(), 3, core::hint::black_box(2)) + add(core::hint::black_box(1), 2); unsafe { core::arch::asm!("mov edi, {0:e}", "mov eax, 60", "syscall", in(reg) r, options(noreturn)) } }
