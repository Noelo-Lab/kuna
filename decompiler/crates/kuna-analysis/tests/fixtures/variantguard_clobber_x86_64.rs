// Fixture source for the `option variantguard` PRODUCER-ORDERING rule.
//
// Built with:
//   rustc -C opt-level=1 -C debuginfo=2 -C relocation-model=static \
//         -C panic=abort -C target-feature=+crt-static -C link-arg=-nostartfiles \
//         -o variantguard_clobber_x86_64 variantguard_clobber_x86_64.rs
//
// A constant `tag = K` store is evidence about what the object BECOMES.  It says
// nothing about a value that was read BEFORE it.  The first revision of
// `variantguard` propagated the store backwards over whole blocks with no
// ordering test and no kill, so the READ below was labelled with the variant the
// store was about to write:
//
//     v1 = (*dst).payload.Err.__0;      // reads whatever the CALLER passed
//     (*dst).tag = 1;
//
// `read_then_clobber` and `read_then_clobber_ok` are byte-identical up to the
// clobber constant, so that bug gave the SAME `mov 0x8(%rdi),%rax` opposite
// variant names.  At most one could be right; by construction neither was --
// the source reads both arms.  Every read here must therefore be spelled by
// OFFSET unless a GUARD proves it, while the writes below keep their names.
//
// Read-then-overwrite of an enum in place is not a corner case.  It is what
// `mem::replace`, `Option::take`, and any state machine that consumes its old
// state and writes the new one in one block compile to, and it is the shape drop
// glue and `?`-desugaring produce.  The two fixtures this feature shipped with
// never read an object they later clobber, which is exactly why the defect
// survived the author's own testing -- so the shape is committed here.
#![no_std]
#![no_main]
#![allow(dead_code)]

#[panic_handler]
fn ph(_: &core::panic::PanicInfo) -> ! { loop {} }

static mut SEED: u32 = 3;
static mut SINK: u64 = 0;

// (1) An UNGUARDED read of the caller's value, above an `Err` clobber.
#[no_mangle]
#[inline(never)]
pub unsafe extern "Rust" fn read_then_clobber(dst: *mut Result<u64, u64>) -> u64 {
    let v = match *dst { Ok(a) => a, Err(b) => b };
    *dst = Err(7);
    v
}

// (2) The CONTROL: identical but for the clobber constant. The read must render
// exactly as (1) does; if the two differ, a store is deciding a read's name.
#[no_mangle]
#[inline(never)]
pub unsafe extern "Rust" fn read_then_clobber_ok(dst: *mut Result<u64, u64>) -> u64 {
    let v = match *dst { Ok(a) => a, Err(b) => b };
    *dst = Ok(9);
    v
}

// (3) The read is preserved as an expression across the clobber, which is what
// made the old output contradict itself three lines apart: named `Err`, then
// tested for `Ok` and returned unchanged.
#[no_mangle]
#[inline(never)]
pub unsafe extern "Rust" fn read_then_store(dst: *mut Result<u64, u64>, x: u32) -> u64 {
    let out = match *dst { Ok(v) => v, Err(e) => e.wrapping_add(100) };
    *dst = Err(x as u64);
    out
}

// (4) A read the GUARD does prove, above a clobber of the OTHER variant. The
// read is `Ok` because the tag was tested; the store below is `Err`. The two
// coexist, and neither is allowed to decide the other.
#[no_mangle]
#[inline(never)]
pub unsafe extern "Rust" fn guard_then_clobber(dst: *mut Result<u64, u64>) -> u64 {
    let v = match *dst { Ok(a) => a, Err(_) => 0 };
    *dst = Err(v + 1);
    v
}

// (5) One forward path stores, the other returns with no store at all.
#[no_mangle]
#[inline(never)]
pub unsafe extern "Rust" fn maybe_store(dst: *mut Result<u64, u64>, x: u32) -> u64 {
    let out = match *dst { Ok(v) => v, Err(e) => e.wrapping_add(100) };
    if x != 0 { *dst = Err(x as u64); }
    out
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    unsafe {
        let s = core::ptr::read_volatile(core::ptr::addr_of!(SEED));
        let mut r: Result<u64, u64> = if s > 3 { Ok(s as u64) } else { Err(s as u64) };
        let mut acc = read_then_clobber(&mut r as *mut _);
        let mut r2: Result<u64, u64> = if s > 4 { Ok(1) } else { Err(2) };
        acc += read_then_clobber_ok(&mut r2 as *mut _);
        acc += read_then_store(&mut r as *mut _, s);
        acc += guard_then_clobber(&mut r2 as *mut _);
        acc += maybe_store(&mut r as *mut _, s);
        core::ptr::write_volatile(core::ptr::addr_of_mut!(SINK), acc);
    }
    loop {}
}
