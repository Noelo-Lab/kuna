#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn write_value(mut a0: u64, mut a1: u64)
{
  let mut v1: u64; // rsi | stack - 0x18
  let mut v2: *mut u64; // rax | stack - 0x10
  let mut v3: u8; // stack - 0x19
  let mut v4: *mut *const (); // stack - 0x8
  
  v2 = &mut v1;
  v4 = fmt;
  v1 = a1;
  if ({ use std::fmt::Write as _; write!(&mut *(a0 as *mut std::string::String),"[{}]",&*((v2 as *const _) as *const i64)) }).is_err() as u8 == b'\0' {
    return;
  }
  core::result::unwrap_failed(_called__Result::unwrap____on_an__Err__value_,0x2b,&mut v3,0x4afa8,0x4aff8); // no-return
}
