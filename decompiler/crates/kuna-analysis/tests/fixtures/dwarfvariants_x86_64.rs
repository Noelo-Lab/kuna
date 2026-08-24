// Fixture source for `option dwarfvariants` (kuna-analysis DWARF variant-part import).
//
// Built with:
//   rustc -C opt-level=1 -C debuginfo=2 -C relocation-model=static \
//         -C panic=abort -C target-feature=+crt-static \
//         -o dwarfvariants_x86_64 dwarfvariants_x86_64.rs
//
// `#![no_std]`/`#![no_main]` keeps the image tiny; `relocation-model=static`
// fixes the VMAs so a stage testcase can pin addresses.  Every enum below is a
// distinct DWARF `DW_TAG_variant_part` shape the importer has to handle.
#![no_std]
#![no_main]
#![allow(dead_code)]

#[panic_handler]
fn ph(_: &core::panic::PanicInfo) -> ! { loop {} }

static mut SEED: u32 = 3;
static mut SINK: u64 = 0;
static ANCHOR: u32 = 41;

// (1) Result<u32,u32>: tag u32 @0, payload @4, discr 0 = Ok, 1 = Err.
#[no_mangle]
#[inline(never)]
pub extern "Rust" fn ret_result(x: u32) -> Result<u32, u32> {
    if x > 10 { return Ok(x * 3); }
    Err(x + 1)
}

// (2) Option<u32>: tag u32 @0, payload @4, discr 0 = None (no members), 1 = Some.
#[no_mangle]
#[inline(never)]
pub extern "Rust" fn ret_option(x: u32) -> Option<u32> {
    if x > 10 { return Some(x * 3); }
    None
}

// (3) Option<&u32>: NICHE-encoded.  `Some` is the DEFAULT variant (no
// DW_AT_discr_value) and its payload overlaps the discriminant read.
#[no_mangle]
#[inline(never)]
pub extern "Rust" fn ret_niche(x: u32, p: &'static u32) -> Option<&'static u32> {
    if x > 10 { return Some(p); }
    None
}

// (4) three variants, one of them fieldless.
pub enum Three { A(u32), B(u64), C }

#[no_mangle]
#[inline(never)]
pub extern "Rust" fn ret_three(x: u32) -> Three {
    if x > 10 { return Three::A(x); }
    if x > 5 { return Three::B(x as u64); }
    Three::C
}

// (5) a variant with MULTIPLE fields (rustc flattens it into the payload struct,
// it does NOT nest a second variant_part).
pub enum Multi { P { a: u32, b: u32 }, Q(u64) }

#[no_mangle]
#[inline(never)]
pub extern "Rust" fn ret_multi(x: u32) -> Multi {
    if x > 10 { return Multi::P { a: x, b: x + 1 }; }
    Multi::Q(x as u64)
}

// (6) RECURSIVE: a variant payload references the enclosing enum.
pub enum List { Cons(u32, *const List), Nil }

#[no_mangle]
#[inline(never)]
pub extern "Rust" fn list_len(l: *const List) -> u32 {
    let mut n = 0u32;
    let mut cur = l;
    loop {
        unsafe {
            match &*cur {
                List::Cons(_, next) => { n += 1; cur = *next; }
                List::Nil => return n,
            }
        }
    }
}

// (7) a FIELDLESS enum: rustc emits DW_TAG_enumeration_type, not a variant_part,
// so the importer must never see it.
#[derive(Clone, Copy)]
pub enum Plain { R = 0, S = 1, T = 2 }

#[no_mangle]
#[inline(never)]
pub extern "Rust" fn ret_plain(x: u32) -> Plain {
    match x { 0 => Plain::R, 1 => Plain::S, _ => Plain::T }
}

// (8) a plain C-shaped struct in the same image: nothing about it may change.
pub struct Pair { pub lo: u32, pub hi: u32 }

#[no_mangle]
#[inline(never)]
pub extern "Rust" fn ret_pair(x: u32) -> Pair {
    Pair { lo: x, hi: x + 1 }
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    unsafe {
        let s = core::ptr::read_volatile(core::ptr::addr_of!(SEED));
        let mut acc = 0u64;
        acc += match ret_result(s) { Ok(v) => v as u64, Err(e) => e as u64 };
        acc += match ret_option(s) { Some(v) => v as u64, None => 1 };
        acc += match ret_niche(s, &ANCHOR) { Some(v) => *v as u64, None => 2 };
        acc += match ret_three(s) { Three::A(v) => v as u64, Three::B(v) => v, Three::C => 3 };
        acc += match ret_multi(s) { Multi::P { a, b } => (a + b) as u64, Multi::Q(v) => v };
        let nil = List::Nil;
        let one = List::Cons(7, &nil as *const List);
        acc += list_len(&one as *const List) as u64;
        acc += ret_plain(s) as u64;
        let p = ret_pair(s);
        acc += (p.lo + p.hi) as u64;
        core::ptr::write_volatile(core::ptr::addr_of_mut!(SINK), acc);
    }
    loop {}
}
