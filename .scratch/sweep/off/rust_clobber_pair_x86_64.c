// Function: scalar_callee @ 0x201240
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn scalar_callee(mut a0: i64) -> i64
{
  return a0 + 7;
}

// Function: pair_shaped_reader @ 0x201250
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn pair_shaped_reader() -> u64
{
  let mut v1: u64; // rax
  let mut v2: i64;
  let mut v3: i64; // rdx
  
  scalar_callee();
  v2 = v3 + 1;
  if v1 & 1 == 0 {
    v2 = v3 + 100;
  }
  v1 = v2;
  return v1;
}

// Function: _start @ 0x201270
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn _start()
{
  pair_shaped_reader(3);
  loop {
  }
}

