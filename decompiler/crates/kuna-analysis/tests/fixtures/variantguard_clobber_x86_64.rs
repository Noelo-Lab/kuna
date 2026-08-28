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
// The same rule has a GUARD side, added after a second review round: a guard
// proves what the object was AT THE GUARD, and does not survive a tag store or a
// call that may store through the pointer.  Functions (6)-(11) below are that
// half, including a control pair whose two loads must render identically.
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

// ---------------------------------------------------------------------------
// (6)-(10) The GUARD side of the same rule.  A guard proves what the object was
// AT THE GUARD; it does not survive a store over the tag bytes or a call that
// may store through the pointer.  A second revision of this pass had the guard
// fact beat a correctly-computed producer fact by precedence, so a write that
// built `Err` inside an `Ok`-guarded block was named `Ok` one line below its own
// `tag = 1`, and a read after a clobbering call was named with the variant the
// call had just destroyed.
// ---------------------------------------------------------------------------

// (6) A WRITE that builds `Err` inside a block the guard says is `Ok`. The two
// facts disagree; the store is what holds at that point, so the write is `Err`
// and the guard-proved read above it is still `Ok`.
#[no_mangle]
#[inline(never)]
pub unsafe extern "Rust" fn guarded_write(dst: *mut Result<u64, u64>) -> u64 {
    if let Ok(a) = *dst {
        *dst = Err(a + 1);
        return a;
    }
    0
}

// (7) The mirror, so neither direction can be right by accident.
#[no_mangle]
#[inline(never)]
pub unsafe extern "Rust" fn guarded_write_mirror(dst: *mut Result<u64, u64>) -> u64 {
    if let Err(a) = *dst {
        *dst = Ok(a + 1);
        return a;
    }
    0
}

// (8) A READ under a STALE guard, with no call involved at all: the tag was
// overwritten between the branch that proved `Ok` and this load.
#[no_mangle]
#[inline(never)]
pub unsafe extern "Rust" fn stale_read(dst: *mut Result<u64, u64>, x: u64) -> u64 {
    if let Ok(_) = *dst {
        *dst = Err(x);
        return core::ptr::read_volatile((dst as *const u64).add(1));
    }
    0
}

#[no_mangle]
#[inline(never)]
pub unsafe extern "Rust" fn set_err(dst: *mut Result<u64, u64>, x: u64) {
    core::ptr::write_volatile(dst, Err(x));
}

#[no_mangle]
#[inline(never)]
pub unsafe extern "Rust" fn set_ok(dst: *mut Result<u64, u64>, x: u64) {
    core::ptr::write_volatile(dst, Ok(x));
}

// (9)/(10) THE CONTROL PAIR for the guard side, the exact analogue of
// read_then_clobber / read_then_clobber_ok. Identical but for which arm the
// guard selects and which variant the call writes; the same
// `mov 0x8(%rdi),%rax` must not get opposite variant names, each naming the
// variant the preceding call had just destroyed.
#[no_mangle]
#[inline(never)]
pub unsafe extern "Rust" fn read_after_call_ok_guard(dst: *mut Result<u64, u64>, x: u64) -> u64 {
    if let Ok(_) = *dst {
        set_err(dst, x);
        return core::ptr::read_volatile((dst as *const u64).add(1));
    }
    0
}

#[no_mangle]
#[inline(never)]
pub unsafe extern "Rust" fn read_after_call_err_guard(dst: *mut Result<u64, u64>, x: u64) -> u64 {
    if let Err(_) = *dst {
        set_ok(dst, x);
        return core::ptr::read_volatile((dst as *const u64).add(1));
    }
    0
}

// (11) Aliasing: `q` may be `p`, so the `Ok` two lines up is not proved for the
// raw payload write below it.
#[no_mangle]
#[inline(never)]
pub unsafe extern "Rust" fn alias_clobber(
    p: *mut Result<u64, u64>,
    q: *mut Result<u64, u64>,
    x: u64,
) {
    *p = Ok(1);
    *q = Err(2);
    *(p as *mut u64).add(1) = x;
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
        acc += guarded_write(&mut r as *mut _);
        acc += guarded_write_mirror(&mut r2 as *mut _);
        acc += stale_read(&mut r as *mut _, s as u64);
        acc += read_after_call_ok_guard(&mut r as *mut _, s as u64);
        acc += read_after_call_err_guard(&mut r2 as *mut _, s as u64);
        alias_clobber(&mut r as *mut _, &mut r2 as *mut _, s as u64);
        core::ptr::write_volatile(core::ptr::addr_of_mut!(SINK), acc);
    }
    loop {}
}
