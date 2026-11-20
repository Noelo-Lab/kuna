#![no_std]
#![no_main]
#[panic_handler] fn panic(_: &core::panic::PanicInfo<'_>) -> ! { loop {} }
static mut MARK: [u64; 3] = [0; 3];
#[no_mangle] #[inline(never)] pub fn on_some(x: u64) -> u64 {
 unsafe { core::ptr::write_volatile(core::ptr::addr_of_mut!(MARK[0]), x); } x
}
#[no_mangle] #[inline(never)] pub fn on_none(x: u64) -> u64 {
 unsafe { core::ptr::write_volatile(core::ptr::addr_of_mut!(MARK[1]), x); } x
}
#[no_mangle] #[inline(never)] pub fn named_some(value: Option<&u64>) -> u64 {
 if value.is_some() { on_some(17) } else { on_none(29) }
}
#[no_mangle] #[inline(never)] pub fn named_none(value: Option<&u64>) -> u64 {
 if value.is_none() { on_some(31) } else { on_none(43) }
}
#[no_mangle] #[inline(never)] pub fn named_payload(value: Option<&u64>) -> u64 {
 match value { Some(payload) => on_some(*payload), None => on_none(53) }
}
#[no_mangle] #[inline(never)] pub fn make_niche<'a>(present: bool, payload: &'a u64) -> Option<&'a u64> {
 unsafe { core::ptr::write_volatile(core::ptr::addr_of_mut!(MARK[2]), present as u64); }
 if present { Some(payload) } else { None }
}
#[no_mangle] #[inline(never)] pub fn returned_some(present: bool, payload: &u64) -> u64 {
 if make_niche(present, payload).is_some() { on_some(61) } else { on_none(73) }
}
#[no_mangle] #[inline(never)] pub fn raw_null(value: *const u64) -> u64 {
 if !value.is_null() { on_some(79) } else { on_none(83) }
}
#[no_mangle] #[inline(never)] pub fn raw_word(value: usize) -> u64 {
 if value != 0 { on_some(89) } else { on_none(97) }
}
#[no_mangle] pub fn _start() -> ! { loop {} }
