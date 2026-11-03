#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn unsigned_address(mut p: *mut i64, mut i: u64) -> *mut u64
{
  return &mut p[i] as *mut u64;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn byte_address(mut p: *mut i64, mut i: u64) -> *mut u8
{
  return &mut p[i] as *mut u8;
}
fn main(){let mut a=[0i64;4];for i in [0u64,1u64,2u64,7u64,0x7fffffffffffffffu64,0x8000000000000000u64,0xffffffffffffffffu64,0x2000000000000000u64,0x4000000000000000u64,0xaaaaaaaa55555555u64]{unsafe{let p=a.as_mut_ptr();println!("{:016x} {:016x}",(unsigned_address(p,i) as u64).wrapping_sub(p as u64),(byte_address(p,i) as u64).wrapping_sub(p as u64));}}}
