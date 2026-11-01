#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn panic_hex(mut a0: u32)
{
  let mut v1: u32; // edi | stack - 0x14
  let mut v2: *mut u32; // rax | stack - 0x10
  let mut v3: *mut *const (); // stack - 0x8
  
  v2 = &mut v1;
  v3 = fmt;
  v1 = a0;
  core::panicking::panic_fmt(0x69e8,&mut v2,0x5bd78); // no-return
}
