#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn read_port() -> u32
{
  return (r#in(0x31) as u8) as u32;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn read_variable(mut port: u32) -> u32
{
  return (r#in(port as i16) as u8) as u32;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn write_port(mut port: u32, mut data: u8)
{
  out(port as i16,data);
  return;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn avr_break()
{
  R1 = 0;
  r#break();
  return;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn cooperate(mut value: u32)
{
  r#yield(value);
  return;
}
