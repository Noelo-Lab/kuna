#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn panic_reorder(mut a0: u32, mut a1: u16)
{
  let mut v1: u16; // si | stack - 0x36
  let mut v2: u32; // edi | stack - 0x34
  let mut v3: *mut u16; // rax | stack - 0x10 | stack - 0x30
  let mut v4: *mut *const (); // stack - 0x28
  let mut v5: *mut u32; // rcx | stack - 0x20
  let mut v6: *mut *const (); // stack - 0x18
  let mut v7: *mut u16; // rax | stack - 0x10 | stack - 0x30
  let mut v8: *mut *const (); // stack - 0x8
  
  v3 = &mut v1;
  v4 = fmt;
  v5 = &mut v2;
  v6 = fmt;
  v8 = fmt;
  v1 = a1;
  v2 = a0;
  v7 = v3;
  panic!("{0:#08x} {1:+06} {2:>5o} {1:+06}",&*((v3 as *const _) as *const u16),&*((v5 as *const _) as *const u32),&*((v7 as *const _) as *const u16)); // no-return
}
