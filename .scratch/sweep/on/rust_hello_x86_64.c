// Function: black_box @ 0x1320
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn black_box(mut a0: u64) -> u64
{
  return a0;
}

// Function: nostd::m::rusty_helper @ 0x1330
// nostd::m::rusty_helper
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn rusty_helper(mut a0: i64)
{
  black_box(a0 * 3); // tail-call
}

// Function: _start @ 0x1340
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn _start()
{
  nostd::m::rusty_helper(black_box(0x29));
  loop {
  }
}

