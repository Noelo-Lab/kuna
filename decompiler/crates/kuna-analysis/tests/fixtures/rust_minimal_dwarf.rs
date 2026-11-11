#![no_std]
#[no_mangle]
#[inline(never)]
pub extern "C" fn add_input(value: u64) -> u64 { value.wrapping_add(7) }
#[no_mangle]
#[inline(never)]
pub extern "C" fn call_input(value: u64) -> u64 { add_input(value.wrapping_mul(3)) }
