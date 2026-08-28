#![no_std]
#![no_main]
#![allow(dead_code)]
#[panic_handler]
fn ph(_: &core::panic::PanicInfo) -> ! { loop {} }
static mut SEED: u32 = 3;
static mut SINK: u64 = 0;

#[no_mangle]
#[inline(never)]
pub unsafe extern "Rust" fn opaque(v: u64) -> u64 { core::ptr::read_volatile(core::ptr::addr_of!(SEED)) as u64 ^ v }

// NY-1: the MIRROR of the fixed bug, on the GUARD side.  The guard proves the
// object was Ok AT THE GUARD.  We then OVERWRITE it with Err and read the
// payload again.  The second read reads an Err payload, but it sits on the
// Ok-guarded edge.  If the guard colours it Ok, that is the same class of lie.
#[no_mangle]
#[inline(never)]
pub unsafe extern "Rust" fn guard_overwrite_read(dst: *mut Result<u64,u64>) -> u64 {
    match *dst {
        Ok(_) => {
            *dst = Err(opaque(1));
            let v = match *dst { Ok(a) => a, Err(b) => b };
            opaque(v)
        }
        Err(_) => 0,
    }
}

// NY-2: guard proves Ok, then a CALL that may mutate through the same pointer,
// then read.
#[no_mangle]
#[inline(never)]
pub unsafe extern "Rust" fn mutate(p: *mut Result<u64,u64>) { *p = Err(opaque(2)); }

#[no_mangle]
#[inline(never)]
pub unsafe extern "Rust" fn guard_call_read(dst: *mut Result<u64,u64>) -> u64 {
    match *dst {
        Ok(_) => { mutate(dst); let v = match *dst { Ok(a) => a, Err(b) => b }; opaque(v) }
        Err(_) => 0,
    }
}

// NY-3: producer-side layout mismatch: write the P-only field (offset 4) and
// then tag the object Q.
#[no_mangle]
#[inline(never)]
pub unsafe extern "Rust" fn misfit(dst: *mut u32, x: u32) {
    *dst.add(1) = x;          // offset 4: only P has a field here
    *(dst as *mut u64).add(1) = x as u64;  // offset 8
    *dst = 1;                 // tag = 1 => Q
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    unsafe {
        let s = core::ptr::read_volatile(core::ptr::addr_of!(SEED));
        let mut r: Result<u64,u64> = Ok(s as u64);
        let mut acc = guard_overwrite_read(&mut r as *mut _);
        acc += guard_call_read(&mut r as *mut _);
        core::ptr::write_volatile(core::ptr::addr_of_mut!(SINK), acc);
    }
    loop {}
}
