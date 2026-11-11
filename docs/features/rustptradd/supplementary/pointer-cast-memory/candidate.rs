#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn byte_load(mut p: *mut i64, mut i: i64) -> u8
{
  return *(p.wrapping_byte_offset((i as isize).wrapping_mul(8)) as *mut u8);
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn unsigned_load(mut p: *mut i64, mut i: i64) -> u64
{
  return *p.wrapping_byte_offset((i as isize).wrapping_mul(8)) as u64;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn byte_store(mut p: *mut i64, mut i: i64, mut v: u8)
{
  *(p.wrapping_byte_offset((i as isize).wrapping_mul(8)) as *mut u8) = v;
}
fn main(){unsafe{for v in [0u64,1u64,2u64,0x7fu64,0x80u64,0xffu64,0x7fffu64,0x8000u64,0xffffu64,0x7fffffffu64,0x80000000u64,0xffffffffu64,0x7fffffffffffffffu64,0x8000000000000000u64,0xffffffffffffffffu64,0x0123456789abcdefu64,0xfedcba9876543210u64,0xaaaaaaaa55555555u64]{for i in -7i64..=7{let mut a=[0i64;17];for k in 0..17{a[k]=0x9e3779b97f4a7c15u64.wrapping_mul(k as u64).wrapping_add(v) as i64;}let p=a.as_mut_ptr().add(8);print!("{:02x} {:016x} ",byte_load(p,i),unsigned_load(p,i));byte_store(p,i,v as u8);let b=std::slice::from_raw_parts(a.as_ptr() as *const u8,136);for x in b{print!("{:02x}",x);}println!();}}}}
