#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn panic_narrow(mut a0: u8)
{
  let mut v1: u8; // dil | stack - 0x31
  let mut v2: *mut u8; // rax | stack - 0x10 | stack - 0x20 | stack - 0x30
  let mut v3: *mut *const (); // stack - 0x28
  let mut v4: *mut u8; // rax | stack - 0x10 | stack - 0x20 | stack - 0x30
  let mut v5: *mut *const (); // stack - 0x18
  let mut v6: *mut u8; // rax | stack - 0x10 | stack - 0x20 | stack - 0x30
  let mut v7: *mut *const (); // stack - 0x8
  
  v2 = &mut v1;
  v3 = fmt;
  v5 = fmt;
  v7 = fmt;
  v1 = a0;
  v4 = v2;
  v6 = v2;
  core::panicking::panic_fmt(0x6a07,&mut v2,0x5bda8); // no-return
}
