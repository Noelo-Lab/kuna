// Fixture source for the `option dwarfvariants` NAMING RULE (kuna-analysis DWARF
// variant-part import).  The sibling `dwarfvariants_x86_64` covers the shapes the
// importer has to read; this one covers what it is allowed to NAME.
//
// Built with:
//   rustc -C opt-level=1 -C debuginfo=2 -C relocation-model=static \
//         -C panic=abort -C target-feature=+crt-static -C link-arg=-nostartfiles \
//         -o dwarfvariants_overlay_x86_64 dwarfvariants_overlay_x86_64.rs
//
// A union member selects itself by OFFSET and the discriminant is never
// consulted, so a variant name is only sound where exactly one variant claims
// the bytes being accessed.  `Result<u64,u64>` is the case where it is NOT:
// `Ok.__0` and `Err.__0` are both at offset 8, always.  `Option<u64>` is the
// case where it IS: `Some` is the only payload-carrying variant.
#![no_std]
#![no_main]
#![allow(dead_code)]

#[panic_handler]
fn ph(_: &core::panic::PanicInfo) -> ! { loop {} }

static mut SEED: u32 = 3;
static mut SINK: u64 = 0;

// (1) Result<u64,u64>: size 16, tag u64 @0, `Ok` (discr 0) and `Err` (discr 1)
// BOTH with `__0` at offset 8.  Two variants over one byte range: neither may be
// named, and both arms must render through the offset spelling.
#[no_mangle]
#[inline(never)]
pub extern "Rust" fn r16(x: u32) -> Result<u64, u64> {
    if x > 10 { return Ok(x as u64 * 3); }
    Err(x as u64 + 1)
}

// (2) the CONSUMER side of the same type: the `Err` arm is the one the old
// labelling rendered as `Ok`.
#[no_mangle]
#[inline(never)]
pub extern "Rust" fn use16(x: u32) -> u64 {
    match r16(x) { Ok(v) => v, Err(e) => e + 100 }
}

// (3) the same payload written THROUGH A POINTER, so the store is a field path
// on the recovered struct rather than a register pair.
#[no_mangle]
#[inline(never)]
pub unsafe extern "Rust" fn put_res(dst: *mut Result<u64, u64>, x: u32) {
    *dst = if x > 10 { Ok(x as u64 * 3) } else { Err(x as u64 + 1) };
}

// (4) Option<u64>: exactly ONE payload-carrying variant, so `Some` is forced and
// must survive.  Written through a pointer for the same reason as (3).
#[no_mangle]
#[inline(never)]
pub unsafe extern "Rust" fn put_opt(dst: *mut Option<u64>, x: u32) {
    *dst = if x > 10 { Some(x as u64 * 3) } else { None };
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    unsafe {
        let s = core::ptr::read_volatile(core::ptr::addr_of!(SEED));
        let mut r: Result<u64, u64> = Ok(0);
        let mut o: Option<u64> = None;
        put_res(&mut r as *mut Result<u64, u64>, s);
        put_opt(&mut o as *mut Option<u64>, s);
        let mut acc = use16(s);
        acc += match r { Ok(v) => v, Err(e) => e };
        acc += match o { Some(v) => v, None => 5 };
        core::ptr::write_volatile(core::ptr::addr_of_mut!(SINK), acc);
    }
    loop {}
}
