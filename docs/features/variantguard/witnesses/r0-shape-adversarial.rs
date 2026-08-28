// The adversarial witness set for `variantguard`: every shape whose CODEGEN
// looks like a tagged enum but which is not one, plus the tagged enums the pass
// must leave alone.  Built with FULL debug info, so the DWARF gate is exercised
// rather than trivially empty.
#![no_std]
#![no_main]
#![allow(dead_code)]

#[panic_handler]
fn ph(_: &core::panic::PanicInfo) -> ! { loop {} }

static mut SEED: u32 = 3;
static mut SINK: u64 = 0;

// (W1) a plain repr(C) two-field struct: two return paths storing different
// constants at offset 0 -- the exact shape #333 classified as an enum.
#[repr(C)]
pub struct Pair { pub kind: u64, pub val: u64 }

#[no_mangle]
#[inline(never)]
pub extern "C" fn w1_pair(x: u32) -> Pair {
    if x > 10 { return Pair { kind: 0, val: x as u64 * 3 }; }
    Pair { kind: 1, val: x as u64 + 1 }
}

// (W2) a (u64,u64) tuple whose first element is 7 or 9.
#[no_mangle]
#[inline(never)]
pub extern "Rust" fn w2_tuple(x: u32) -> (u64, u64) {
    if x > 10 { return (7, x as u64 * 3); }
    (9, x as u64 + 1)
}

// (W3) a (u64,u64) bitmask pair.
#[no_mangle]
#[inline(never)]
pub extern "Rust" fn w3_bits(x: u32) -> (u64, u64) {
    if x > 10 { return (0x8000_0000_0000_0001, x as u64); }
    (0x4000_0000_0000_0002, !(x as u64))
}

// (W4) a plain fat pointer: the "discriminant" would be a .rodata address.
#[no_mangle]
#[inline(never)]
pub extern "Rust" fn w4_str(x: u32) -> &'static str {
    if x > 10 { "alpha" } else { "bravo!" }
}

// (W5) a NICHE-encoded Option over a fat pointer.
#[no_mangle]
#[inline(never)]
pub extern "Rust" fn w5_optstr(x: u32) -> Option<&'static str> {
    if x > 10 { Some("alpha") } else { None }
}

#[no_mangle]
#[inline(never)]
pub extern "Rust" fn w5_use(x: u32) -> u64 {
    match w5_optstr(x) { Some(s) => s.len() as u64, None => 7 }
}

// (W6) a 3-variant enum of which only two tags are ever returned.
pub enum Trio { A(u64), B(u64), C(u64) }

#[no_mangle]
#[inline(never)]
pub extern "Rust" fn w6_trio(x: u32) -> Trio {
    if x > 10 { return Trio::A(x as u64); }
    Trio::C(x as u64 + 1)
}

#[no_mangle]
#[inline(never)]
pub extern "Rust" fn w6_use(x: u32) -> u64 {
    match w6_trio(x) { Trio::A(v) => v, Trio::B(v) => v + 1, Trio::C(v) => v + 2 }
}

// --- the true positives, and the block that must stay UNPINNED --------------

#[no_mangle]
#[inline(never)]
pub extern "Rust" fn p_mk(x: u32) -> Result<u64, u64> {
    if x > 10 { return Ok(x as u64 * 3); }
    Err(x as u64 + 1)
}

// TRUE POSITIVE: the two arms differ, so each payload read sits under a guard.
#[no_mangle]
#[inline(never)]
pub extern "Rust" fn p_split(x: u32) -> u64 {
    match p_mk(x) { Ok(v) => v, Err(e) => e + 100 }
}

// MUST STAY UNPINNED: both arms do the same thing with the payload, so the read
// is common to both and its block can still reach either variant.
#[no_mangle]
#[inline(never)]
pub extern "Rust" fn p_either(x: u32) -> u64 {
    match p_mk(x) { Ok(v) => v, Err(e) => e }
}

extern "C" {
    fn c_kindval(x: u32) -> u64;
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    unsafe {
        let s = core::ptr::read_volatile(core::ptr::addr_of!(SEED));
        let mut acc = 0u64;
        let p = w1_pair(s);
        acc += p.kind + p.val;
        let t = w2_tuple(s); acc += t.0 + t.1;
        let b = w3_bits(s); acc += b.0 ^ b.1;
        acc += w4_str(s).len() as u64;
        acc += w5_use(s);
        acc += w6_use(s);
        acc += p_split(s);
        acc += p_either(s);
        acc += c_kindval(s);
        core::ptr::write_volatile(core::ptr::addr_of_mut!(SINK), acc);
    }
    loop {}
}
