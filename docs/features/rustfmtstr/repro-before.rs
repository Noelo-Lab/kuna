#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn make_string(mut a0: u64, mut a1: u32, mut a2: u64, mut a3: u64) -> u64
{
  let mut v1: u32; // esi | stack - 0x3c
  let mut v2: u64; // rdx | stack - 0x38
  let mut v3: *mut u32; // rax | stack - 0x28
  let mut v4: u64; // rcx | stack - 0x30
  let mut v5: *mut *const (); // stack - 0x20
  let mut v6: *mut u64; // rax | stack - 0x18
  let mut v7: *mut *const (); // stack - 0x10
  
  v3 = &mut v1;
  v5 = fmt;
  v6 = &mut v2;
  v7 = fmt;
  v1 = a1;
  v2 = a2;
  v4 = a3;
  alloc::fmt::format::format_inner(a0,0x5edd,&mut v3);
  return a0;
}
