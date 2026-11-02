#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn pieces_plain()
{
  let mut v1: u64; // stack - 0x30
  let mut v2: u64; // stack - 0x28
  let mut v3: u64; // stack - 0x20
  let mut v4: u64; // stack - 0x18
  let mut v5: u64; // stack - 0x10
  
  v1 = 0x59878;
  v2 = 1;
  v3 = 8;
  v4 = 0;
  v5 = 0;
  print!("quote=\" slash=\\ nul=\0 {{}} ");
}
