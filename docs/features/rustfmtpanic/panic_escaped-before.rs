#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn panic_escaped(mut a0: u8)
{
  let mut v1: u8; // dil | stack - 0x11
  let mut v2: *mut u8; // rax | stack - 0x10
  let mut v3: *mut *const (); // stack - 0x8
  
  v2 = &mut v1;
  v3 = fmt;
  v1 = a0;
  core::panicking::panic_fmt(0x69b8,&mut v2,0x5bd30); // no-return
}
