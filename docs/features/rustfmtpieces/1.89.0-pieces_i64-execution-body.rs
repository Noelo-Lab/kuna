#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn pieces_i64(mut a0: u64)
{
  let mut v1: u64; // rdi | stack - 0x48
  let mut v2: *mut u8; // rax | rsp | stack - 0x40
  let mut v3: u64; // stack - 0x30
  let mut v4: *mut *const (); // stack - 0x38
  let mut v5: u64; // stack - 0x28
  let mut v6: *mut *mut u8; // rax | stack - 0x20
  let mut v7: u64; // stack - 0x18
  let mut v8: u64; // stack - 0x10
  
  v2 = &mut v1;
  v4 = fmt;
  v3 = 0x49010;
  v5 = 1;
  v8 = 0;
  v6 = &mut v2;
  v7 = 1;
  v1 = a0;
  print!("{}",&*((v2 as *const _) as *const i64));
}
