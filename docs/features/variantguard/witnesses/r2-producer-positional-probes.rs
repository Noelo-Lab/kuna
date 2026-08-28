#![no_std]
#![no_main]
#![allow(dead_code)]
#[panic_handler]
fn ph(_: &core::panic::PanicInfo) -> ! { loop {} }
static mut SEED: u32 = 3;
static mut SINK: u64 = 0;

// NX-1: TWO DISTINCT objects interleaved in ONE block.  a's payload write sits
// between b's tag store and a's own tag store.  If the positional rule takes the
// "nearest constant tag store" without checking the object root, a's payload
// write gets b's variant.
#[no_mangle]
#[inline(never)]
pub unsafe extern "Rust" fn two_objects(a: *mut Result<u64,u64>, b: *mut Result<u64,u64>, x: u32) {
    *b = Err(x as u64);
    *a = Ok(x as u64 + 1);
}

// NX-2: the same address expression feeds a READ and a WRITE.  If "pointer
// arithmetic feeding a store and nothing else" is mis-decided, the read is
// coloured by the store's tag.
#[no_mangle]
#[inline(never)]
pub unsafe extern "Rust" fn alias_ptr(dst: *mut Result<u64,u64>) -> u64 {
    let p = &mut *dst;
    let v = match *p { Ok(a) => a, Err(b) => b };
    *p = Err(v.wrapping_mul(3));
    v
}

// NX-3: a payload write ABOVE the tag store of the OTHER variant, with the
// object's real variant fixed by a store further up.  Tests the disagreement
// refusal in a straight line: tag=0; payload=1; tag=1; payload=2.
#[no_mangle]
#[inline(never)]
pub unsafe extern "Rust" fn retag(dst: *mut Result<u64,u64>, x: u32) {
    *dst = Ok(x as u64);
    *dst = Err(x as u64 + 1);
}

// NX-4: write the payload, then CALL, then store the tag.  The call is supposed
// to kill.  If it does not, the write is named across an opaque callee.
#[no_mangle]
#[inline(never)]
pub unsafe extern "Rust" fn opaque(_v: u64) -> u64 { core::ptr::read_volatile(core::ptr::addr_of!(SEED)) as u64 }

#[no_mangle]
#[inline(never)]
pub unsafe extern "Rust" fn write_call_tag(dst: *mut Result<u64,u64>, x: u32) {
    let q = dst as *mut u64;
    *q.add(1) = x as u64;
    let k = opaque(x as u64);
    *q = k & 1;
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    unsafe {
        let s = core::ptr::read_volatile(core::ptr::addr_of!(SEED));
        let mut r: Result<u64,u64> = Ok(0);
        let mut r2: Result<u64,u64> = Err(1);
        two_objects(&mut r as *mut _, &mut r2 as *mut _, s);
        let mut acc = alias_ptr(&mut r as *mut _);
        retag(&mut r2 as *mut _, s);
        write_call_tag(&mut r as *mut _, s);
        acc += match r2 { Ok(v) => v, Err(e) => e };
        core::ptr::write_volatile(core::ptr::addr_of_mut!(SINK), acc);
    }
    loop {}
}
