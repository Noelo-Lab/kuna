#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn hex_width(mut a0: u64, mut a1: u32) -> u64
{
  let mut v1: u32; // esi | stack - 0x2c
  let mut v2: *mut u32; // rax | stack - 0x18 | stack - 0x28
  let mut v3: *mut *const (); // stack - 0x20
  let mut v4: *mut u32; // rax | stack - 0x18 | stack - 0x28
  let mut v5: *mut *const (); // stack - 0x10

  v2 = &mut v1;
  v3 = fmt;
  v5 = fmt;
  v1 = a1;
  v4 = v2;
  std::ptr::write(a0 as *mut std::string::String,format!("0x{:08x} {:>5}",&*((v2 as *const _) as *const u32),&*((v4 as *const _) as *const u32)));
  return a0;
}
