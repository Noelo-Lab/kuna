#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn panic_dynamic(mut a0: u64, mut a1: u64)
{
  let mut v1: u64; // rdi | stack - 0x28
  let mut v2: *mut u8; // rax | rsp | stack - 0x20
  let mut v3: *mut *const (); // stack - 0x18
  let mut v4: u64; // stack - 0x10
  let mut v5: u16; // si | stack - 0x8
  
  v2 = &mut v1;
  v1 = a0;
  if 0x10000 <= a1 {
    panic!("Formatting argument out of range"); // no-return
  }
  v3 = fmt;
  v4 = 0;
  v5 = a1 as u16;
  core::panicking::panic_fmt(0x6940,&mut v2,0x5bd00); // no-return
}
