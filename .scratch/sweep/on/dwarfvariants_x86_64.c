// Function: ret_result @ 0x201220
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn ret_result(mut x: u32) -> core::result::Result<u32, u32>
{
  let mut v1: core::result::Result<u32, u32>; // rax
  
  v1._1_7_ = 0;
  v1.tag._0_1_ = x < 0xb;
  return v1;
}

// Function: ret_option @ 0x201240
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn ret_option(mut x: u32) -> core::option::Option<u32>
{
  let mut v1: core::option::Option<u32>; // rax
  
  v1._1_7_ = 0;
  v1.tag._0_1_ = 0xb <= x;
  return v1;
}

// Function: ret_niche @ 0x201250
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn ret_niche(mut x: u32, mut p: *mut u32) -> core::option::Option<&u32> // early-return
{
  if x <= 10 {
    return 0 as core::option::Option<&u32>;
  }
  return (p as core::option::Option<&u32>::Some) as core::option::Option<&u32>;
}

// Function: ret_three @ 0x201260
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn ret_three(mut rethidden: *mut fx::Three, mut x: u32) -> *mut fx::Three
{
  let mut v1: u32; // ecx
  
  if 0xb <= x {
    (*rethidden).payload.A.__0 = x;
    (*rethidden).tag = 0;
    return rethidden;
  }
  v1 = 2;
  if 6 <= x {
    (*rethidden).payload.B.__0 = x as u64;
    v1 = 1;
  }
  (*rethidden).tag = v1;
  return rethidden;
}

// Function: ret_multi @ 0x201290
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn ret_multi(mut rethidden: *mut fx::Multi, mut x: u32) -> *mut fx::Multi
{
  if 0xb <= x {
    (*rethidden).payload.field_0x4.a = x;
    (*rethidden).payload.field_0x4.field_0x8 = x + 1;
    (*rethidden).tag = 0;
    return rethidden;
  }
  (*rethidden).payload.field_0x8.field_0x8 = x as u64;
  (*rethidden).tag = 1;
  return rethidden;
}

// Function: list_len @ 0x2012c0
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn list_len(mut l: *mut fx::List) -> u32
{
  let mut v1: u32; // eax
  
  v1 = 0;
  if (*l).tag & 1 == 0 {
    v1 = 0;
    loop {
      v1 += 1;
      l = (*l).payload.Cons.__1 as *mut fx::List;
      if !((*l).tag & 1 == 0) {
        break;
      }
    }
  }
  return v1;
}

// Function: ret_plain @ 0x2012e0
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn ret_plain(mut x: u32) -> u8
{
  let mut v1: u8;
  
  v1 = 2 - (x == 1);
  if x == 0 {
    v1 = R;
  }
  return v1;
}

// Function: ret_pair @ 0x201300
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn ret_pair(mut x: u32) -> fx::Pair
{
  let mut v1: fx::Pair; // rax
  
  v1.hi = 0;
  v1.lo = x;
  return v1;
}

// Function: fx::_start @ 0x201310
// fx::_start
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn _start()
{
  let mut v1: fx::Three; // stack - 0x38
  let mut v2: [u32; 4]; // stack - 0x28
  let mut x: u32;
  
  x = _ZN2fx4SEED17h1ebf63af60d40248E;
  ret_result(_ZN2fx4SEED17h1ebf63af60d40248E);
  ret_option(x);
  ret_niche(x,___ as *mut u32);
  ret_three(&mut v1,x);
  ret_multi(&mut v1 as *mut fx::Multi,x);
  v2[0] = 1;
  v1.payload.B.__0 = v2 as u64;
  v1._0_8_ = 0x700000000;
  list_len(&mut v1 as *mut fx::List);
  ret_plain(x);
  ret_pair(x);
  loop {
  }
}

