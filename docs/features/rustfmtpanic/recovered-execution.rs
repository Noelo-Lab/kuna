use std::hint::black_box;
fn capture(f: impl FnOnce() + std::panic::UnwindSafe) {
    let payload = std::panic::catch_unwind(f).unwrap_err();
    let text = payload
        .downcast_ref::<&str>()
        .copied()
        .unwrap_or_else(|| payload.downcast_ref::<String>().unwrap().as_str());
    println!("{text:?}");
}

#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn panic_literal()
{
  panic!("literal {{é🙂}}\n\n"); // no-return
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn panic_empty()
{
  panic!(""); // no-return
}
fn main() {std::panic::set_hook(Box::new(|_| {}));capture(|| unsafe {panic_literal()});capture(|| unsafe {panic_empty()});
for x in (0u64..256).chain([u64::MAX, u64::MAX-1, 1<<63, (1<<63)-1, 0xffff_ffff, 0xffff, 0x123456789abcdef0]) { let x=black_box(x);
capture(|| unsafe {
let mut v2_value=x as u64;let v2=std::ptr::addr_of_mut!(v2_value);
panic!("value={}\n",&*((v2 as *const _) as *const u64));
});
capture(|| unsafe {
let mut v2_value=x as i64;let v2=std::ptr::addr_of_mut!(v2_value);
panic!("signed={:+}",&*((v2 as *const _) as *const i64));
});
capture(|| unsafe {
let mut v2_value=x as u32;let v2=std::ptr::addr_of_mut!(v2_value);
panic!("hex={:#010x}",&*((v2 as *const _) as *const u32));
});
capture(|| unsafe {
let mut v3_value=(x.wrapping_mul(37) ^ (x >> 16)) as u16;let v3=std::ptr::addr_of_mut!(v3_value);
let mut v5_value=x as u32;let v5=std::ptr::addr_of_mut!(v5_value);
let mut v7_value=(x.wrapping_mul(37) ^ (x >> 16)) as u16;let v7=std::ptr::addr_of_mut!(v7_value);
panic!("{0:#08x} {1:+06} {2:>5o} {1:+06}",&*((v3 as *const _) as *const u16),&*((v5 as *const _) as *const u32),&*((v7 as *const _) as *const u16));
});
capture(|| unsafe {
let mut v2_value=x as i8;let v2=std::ptr::addr_of_mut!(v2_value);
let mut v4_value=x as i8;let v4=std::ptr::addr_of_mut!(v4_value);
let mut v6_value=x as i8;let v6=std::ptr::addr_of_mut!(v6_value);
panic!("{:+} {:#x} {:08b}",&*((v2 as *const _) as *const i8),&*((v4 as *const _) as *const i8),&*((v6 as *const _) as *const i8));
});
capture(|| unsafe {
let mut v2_value=x as u8;let v2=std::ptr::addr_of_mut!(v2_value);
panic!("{{|{0:\"<6}|{0:\\>6}|{0:\n^7}|{0:\0<3}|}}\n",&*((v2 as *const _) as *const u8));
});
} }
