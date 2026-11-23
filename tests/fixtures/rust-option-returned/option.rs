#![no_std]
#![no_main]

#[panic_handler]
fn panic(_: &core::panic::PanicInfo<'_>) -> ! { loop {} }

static mut OBSERVED: u64 = 0;

#[no_mangle]
#[inline(never)]
pub fn option_present(value: u64) -> u64 {
    unsafe { core::ptr::write_volatile(core::ptr::addr_of_mut!(OBSERVED), value); }
    value
}

#[no_mangle]
#[inline(never)]
pub fn option_absent(value: u64) -> u64 {
    unsafe { core::ptr::write_volatile(core::ptr::addr_of_mut!(OBSERVED), value); }
    value
}

#[no_mangle]
#[inline(never)]
pub fn inspect_niche(value: Option<&u64>) -> u64 {
    if value.is_some() { option_present(17) } else { option_absent(29) }
}

#[no_mangle]
#[inline(never)]
pub fn inspect_tagged(value: Option<u64>) -> u64 {
    if value.is_some() { option_present(31) } else { option_absent(43) }
}

#[no_mangle]
#[inline(never)]
pub fn inspect_none(value: Option<&u64>) -> u64 {
    if value.is_none() { option_present(47) } else { option_absent(59) }
}

#[no_mangle]
#[inline(never)]
pub fn provide_tagged(value: u64) -> Option<u64> {
    if value > 11 { Some(value) } else { None }
}

#[no_mangle]
#[inline(never)]
pub fn inspect_returned(value: u64) -> u64 {
    let result = provide_tagged(value);
    if result.is_some() { option_present(61) } else { option_absent(73) }
}

#[no_mangle]
pub fn _start() -> ! { loop {} }
