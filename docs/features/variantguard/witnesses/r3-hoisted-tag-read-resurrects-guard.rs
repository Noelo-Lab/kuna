#![no_std]
#![no_main]
#![allow(dead_code)]
#[panic_handler]
fn ph(_: &core::panic::PanicInfo) -> ! { loop {} }
static mut SEED: u32 = 3;
static mut SINK: u64 = 0;

#[no_mangle]
#[inline(never)]
pub unsafe extern "Rust" fn wipe(dst: *mut Result<u64, u64>) {
    core::ptr::write_volatile(dst, Err(99));
}

// R4-1: the tag is READ above the clobber; the BRANCH on it is below.  The
// event raises the block's fact to TOP, and then the same block's edge
// constraint -- derived from the stale read -- resurrects it.
#[no_mangle]
#[inline(never)]
pub unsafe extern "Rust" fn hoist_read(dst: *mut Result<u64, u64>) -> u64 {
    let t = core::ptr::read_volatile(dst as *const u64);
    wipe(dst);
    if t & 1 != 0 { return 0; }
    core::ptr::read_volatile((dst as *const u64).add(1))
}

// R4-2: the clobber is in a DIFFERENT block from the guard, so a positional
// intra-block kill cannot see it at all.
#[no_mangle]
#[inline(never)]
pub unsafe extern "Rust" fn hoist_xblock(dst: *mut Result<u64, u64>, c: u32) -> u64 {
    let t = core::ptr::read_volatile(dst as *const u64);
    if c != 0 { wipe(dst); }
    if t & 1 != 0 { return 0; }
    core::ptr::read_volatile((dst as *const u64).add(1))
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    unsafe {
        let s = core::ptr::read_volatile(core::ptr::addr_of!(SEED));
        let mut r: Result<u64, u64> = Ok(s as u64);
        let mut acc = hoist_read(&mut r as *mut _);
        acc += hoist_xblock(&mut r as *mut _, s);
        core::ptr::write_volatile(core::ptr::addr_of_mut!(SINK), acc);
    }
    loop {}
}
