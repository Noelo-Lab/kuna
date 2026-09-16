// Function: r16 @ 0x201220
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn r16(mut x: u32) -> core::result::Result<u64, u64>
{
  let mut v1: core::result::Result<u64, u64>;
  let mut v2: core::result::Result<u64, u64>::payload;
  
  v2._0_4_ = x + 1;
  v2.field_0x8.__0._4_4_ = 0;
  v1.tag = (x < 0xb) as u64;
  if x >= 0xb {
    v2.field_0x8.__0 = ((x as u64 * 3) as core::result::Result<u64, u64>::field_0x8) as core::result::Result<u64, u64>::payload;
  }
  v1.payload.field_0x8.__0 = v2.field_0x8.__0;
  return v1;
}

// Function: use16 @ 0x201240
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn use16(mut x: u32) -> u64
{
  let mut v1: core::result::Result<u64, u64>::payload;
  let mut v2: core::result::Result<u64, u64>;
  
  v2 = r16(x);
  v1 = ((v2.payload.field_0x8 as i64 + 100) as core::result::Result<u64, u64>::field_0x8) as core::result::Result<u64, u64>::payload;
  if v2.tag & 1 == 0 {
    v1 = v2.payload.field_0x8;
  }
  return v1 as u64;
}

// Function: put_res @ 0x201260
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn put_res(mut dst: *mut core::result::Result<u64, u64>, mut x: u32)
{
  let mut v1: u64;
  
  v1 = (x + 1) as u64;
  if x >= 0xb {
    v1 = x as u64 * 3;
  }
  (*dst).tag = (x < 0xb) as u64;
  (*dst).payload.field_0x8.__0 = v1;
}

// Function: put_opt @ 0x201280
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn put_opt(mut dst: *mut core::option::Option<u64>, mut x: u32)
{
  (*dst).tag = (0xb <= x) as u64;
  (*dst).payload.Some.__0 = x as u64 * 3;
}

// Function: dwarfvariants_overlay_x86_64::_start @ 0x2012a0
// dwarfvariants_overlay_x86_64::_start
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn _start()
{
  let mut v1: core::option::Option<u64>; // stack - 0x28
  let mut v2: core::result::Result<u64, u64>; // stack - 0x18
  let mut x: u32;
  
  x = _ZN28dwarfvariants_overlay_x86_644SEED17hd53e45deb5329e92E;
  v2.tag = 0;
  v2.payload.field_0x8.__0 = 0 as core::result::Result<u64, u64>::payload;
  v1.tag = 0;
  put_res(&mut v2,_ZN28dwarfvariants_overlay_x86_644SEED17hd53e45deb5329e92E);
  put_opt(&mut v1,x);
  use16(x);
  loop {
  }
}

