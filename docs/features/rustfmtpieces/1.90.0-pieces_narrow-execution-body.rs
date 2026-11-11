#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn pieces_narrow(mut a0: u8, mut a1: u16, mut a2: u16, mut a3: u32)
{
  let mut v1: u8; // dil | stack - 0x79
  let mut v2: u16; // si | stack - 0x78
  let mut v3: u16; // dx | stack - 0x76
  let mut v4: u32; // ecx | stack - 0x74
  let mut v5: *mut u8; // rax | stack - 0x70
  let mut v6: u64; // stack - 0x30
  let mut v7: *mut *const (); // stack - 0x68
  let mut v8: *mut u16; // rax | stack - 0x60
  let mut v9: *mut *const (); // stack - 0x58
  let mut v10: *mut u16; // rax | stack - 0x50
  let mut v11: *mut *const (); // stack - 0x48
  let mut v12: *mut u32; // rax | stack - 0x40
  let mut v13: *mut *const (); // stack - 0x38
  let mut v14: u64; // stack - 0x28
  let mut v15: *mut *mut u8; // rax | stack - 0x20
  let mut v16: u64; // stack - 0x18
  let mut v17: u64; // stack - 0x10
  
  v5 = &mut v1;
  v7 = fmt;
  v8 = &mut v2;
  v9 = fmt;
  v10 = &mut v3;
  v11 = fmt;
  v12 = &mut v4;
  v13 = fmt;
  v6 = 0x58a38;
  v14 = 5;
  v17 = 0;
  v15 = &mut v5;
  v16 = 4;
  v1 = a0;
  v2 = a1;
  v3 = a2;
  v4 = a3;
  println!("[{}|{}|{}|{}]",&*((v5 as *const _) as *const u8),&*((v8 as *const _) as *const i16),&*((v10 as *const _) as *const u16),&*((v12 as *const _) as *const i32));
}
