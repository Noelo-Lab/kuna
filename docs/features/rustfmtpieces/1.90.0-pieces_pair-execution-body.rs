#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn pieces_pair(mut a0: u8, mut a1: u64)
{
  let mut v1: u8; // dil | stack - 0x59
  let mut v2: u64; // rsi | stack - 0x58
  let mut v3: *mut u8; // rax | stack - 0x50
  let mut v4: u64; // stack - 0x30
  let mut v5: *mut *const (); // stack - 0x48
  let mut v6: *mut u64; // rax | stack - 0x40
  let mut v7: *mut *const (); // stack - 0x38
  let mut v8: u64; // stack - 0x28
  let mut v9: *mut *mut u8; // rax | stack - 0x20
  let mut v10: u64; // stack - 0x18
  let mut v11: u64; // stack - 0x10
  
  v3 = &mut v1;
  v5 = fmt;
  v6 = &mut v2;
  v7 = fmt;
  v4 = 0x58a18;
  v8 = 2;
  v11 = 0;
  v9 = &mut v3;
  v10 = 2;
  v1 = a0;
  v2 = a1;
  eprint!("{} = {}",&*((v3 as *const _) as *const i8),&*((v6 as *const _) as *const u64));
}
