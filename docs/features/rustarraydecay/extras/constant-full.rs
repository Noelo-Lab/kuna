#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn integer_eight(mut p: *mut [i32; 9]) -> *mut i32
{
  return (&raw mut *p).wrapping_byte_offset(((8 as i64) as isize).wrapping_mul(4)) as *mut i32;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn integer_end(mut p: *mut [i32; 9]) -> *mut i32
{
  return &raw mut *p.wrapping_byte_offset(((1 as i64) as isize).wrapping_mul(36)) as *mut i32;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn pointer_eight(mut p: *mut [*mut u8; 9]) -> *mut *mut u8
{
  return (&raw mut *p).wrapping_byte_offset(((8 as i64) as isize).wrapping_mul(8)) as *mut *mut u8;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn pointer_end(mut p: *mut [*mut u8; 9]) -> *mut *mut u8
{
  return &raw mut *p.wrapping_byte_offset(((1 as i64) as isize).wrapping_mul(72)) as *mut *mut u8;
}
fn main(){unsafe{let mut a=[0i32;9];let mut b=[std::ptr::null_mut::<u8>();9];let ap=&raw mut a;let bp=&raw mut b;println!("{} {} {} {}",(integer_eight(ap) as usize).wrapping_sub(ap as usize),(integer_end(ap) as usize).wrapping_sub(ap as usize),(pointer_eight(bp) as usize).wrapping_sub(bp as usize),(pointer_end(bp) as usize).wrapping_sub(bp as usize));}}
