// Function: prod @ 0x201270
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn prod(mut a0: u32) -> bool
{
  return a0 < 0xb;
}

// Function: cons @ 0x201290
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn cons() -> i32
{
  let mut v1: i32;
  let mut v2: [u8; 12];
  
  v2 = prod();
  v1 = SUB124(v2,8) + 100;
  if SUB128(v2,0) & 1 == 0 {
    v1 = SUB124(v2,8) + 1;
  }
  return v1;
}

// Function: _start @ 0x2012b0
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn _start()
{
  cons(_ZN2w44SEED17he1f640fc9a4aed45E);
  loop {
  }
}

