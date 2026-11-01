#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn panic_signed(mut a0: u64)
{
  let mut v1: u64; // rdi | stack - 0x18
  let mut v2: *mut u8; // rax | rsp | stack - 0x10
  let mut v3: *mut *const (); // stack - 0x8
  
  v2 = &mut v1;
  v3 = fmt;
  v1 = a0;
  panic!("signed={:+}",&*((v2 as *const _) as *const i64)); // no-return
}
