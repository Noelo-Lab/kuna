#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn panic_signed(mut a0: u64)
{
  let mut v1: u64; // rdi | stack - 0x18
  let mut v2: *mut u8; // rax | rsp | stack - 0x10
  let mut v3: *mut *const (); // stack - 0x8
  
  v2 = &mut v1;
  v3 = fmt;
  v1 = a0;
  core::panicking::panic_fmt(0x6a42,&mut v2,0x5bdd8); // no-return
}
