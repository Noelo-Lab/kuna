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
  if core::fmt::write(a0,0x4afc8,23000,&mut v2) == b'\0' {
    return;
  }
  core::result::unwrap_failed(_called__Result::unwrap____on_an__Err__value_,0x2b,&mut v3,0x4afa8,0x4aff8); // no-return
}
