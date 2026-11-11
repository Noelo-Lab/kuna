#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn panic_generic(mut a0: u64, mut a1: u64)
{
  let mut v1: u64; // rdi | stack - 0x20
  let mut v2: *mut u64; // rax | stack - 0x10
  let mut v3: u64; // rsi | stack - 0x18
  let mut v4: *mut *const (); // stack - 0x8
  
  v2 = &mut v1;
  v4 = fmt;
  v1 = a0;
  v3 = a1;
  core::panicking::panic_fmt(0x6918,&mut v2,0x5bd60); // no-return
}
