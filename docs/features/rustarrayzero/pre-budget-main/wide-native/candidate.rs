#![allow(dead_code,unused_variables,unused_parens,unused_mut,non_snake_case)]
#[repr(C,align(16))]struct Memory([u8;128]);
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn wide_copy(mut p: *mut [u8; 16], mut q: *mut [u8; 16])
{
  let mut v1: u64; // xmm0_qb
  
  v1 = *(((&raw mut *q).wrapping_byte_offset(((8 as i64) as isize).wrapping_mul(1)) as *mut u8) as *mut u64);
  *(&raw mut *p as *mut u64) = *(&raw mut *q as *mut u64);
  *(((&raw mut *p).wrapping_byte_offset(((8 as i64) as isize).wrapping_mul(1)) as *mut u8) as *mut u64) = v1;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn wide_zero(mut p: *mut [u8; 16])
{
  *p = [0; 16];
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn wide_unsigned(mut p: *mut [u8; 16], mut x: u64)
{
  *(&raw mut *p as *mut u64) = x;
  *(((&raw mut *p).wrapping_byte_offset(((8 as i64) as isize).wrapping_mul(1)) as *mut u8) as *mut u64) = 0;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn wide_signed(mut p: *mut [u8; 16], mut x: i64)
{
  *(&raw mut *p as *mut i64) = x;
  *(((&raw mut *p).wrapping_byte_offset(((8 as i64) as isize).wrapping_mul(1)) as *mut u8) as *mut i64) = x >> 0x3f;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn wide_halves(mut p: *mut [u8; 16], mut lo: u64, mut hi: u64)
{
  *(&raw mut *p as *mut u64) = lo;
  *(((&raw mut *p).wrapping_byte_offset(((8 as i64) as isize).wrapping_mul(1)) as *mut u8) as *mut u64) = hi;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn wide_add(mut p: *mut [u8; 16], mut q: *mut [u8; 16])
{
  let mut v1: u64; // rax
  let mut v2: i64; // rdx
  
  v1 = *(&raw mut *q as *mut u64);
  v2 = *(((&raw mut *q).wrapping_byte_offset(((8 as i64) as isize).wrapping_mul(1)) as *mut u8) as *mut i64);
  *(&raw mut *p as *mut u64) = v1 + 1;
  *(((&raw mut *p).wrapping_byte_offset(((8 as i64) as isize).wrapping_mul(1)) as *mut u8) as *mut u64) = v2 as u64 + (0xffffffffffffffff <= v1) as u64;
}
fn main(){unsafe{let values:[u64;8]=[0,1,2,9223372036854775807,9223372036854775808,18446744073709551615,81985529216486895,18364758544493064720];let positions=[16,32,96];for a in 0..8{for b in 0..8{let lo=values[a];let hi=values[b];for location in 0..3{
{let mut memory=Memory([0xa5;128]);memory.0[96..104].copy_from_slice(&lo.to_le_bytes());memory.0[104..112].copy_from_slice(&hi.to_le_bytes());let p=memory.0.as_mut_ptr().add(positions[location]);wide_copy(p.cast(),memory.0.as_mut_ptr().add(96).cast());print!("wide_copy {} {} {} ",a,b,location);for byte in memory.0{print!("{:02x}",byte);}println!();}
{let mut memory=Memory([0xa5;128]);memory.0[96..104].copy_from_slice(&lo.to_le_bytes());memory.0[104..112].copy_from_slice(&hi.to_le_bytes());let p=memory.0.as_mut_ptr().add(positions[location]);wide_zero(p.cast());print!("wide_zero {} {} {} ",a,b,location);for byte in memory.0{print!("{:02x}",byte);}println!();}
{let mut memory=Memory([0xa5;128]);memory.0[96..104].copy_from_slice(&lo.to_le_bytes());memory.0[104..112].copy_from_slice(&hi.to_le_bytes());let p=memory.0.as_mut_ptr().add(positions[location]);wide_unsigned(p.cast(),lo);print!("wide_unsigned {} {} {} ",a,b,location);for byte in memory.0{print!("{:02x}",byte);}println!();}
{let mut memory=Memory([0xa5;128]);memory.0[96..104].copy_from_slice(&lo.to_le_bytes());memory.0[104..112].copy_from_slice(&hi.to_le_bytes());let p=memory.0.as_mut_ptr().add(positions[location]);wide_signed(p.cast(),lo as i64);print!("wide_signed {} {} {} ",a,b,location);for byte in memory.0{print!("{:02x}",byte);}println!();}
{let mut memory=Memory([0xa5;128]);memory.0[96..104].copy_from_slice(&lo.to_le_bytes());memory.0[104..112].copy_from_slice(&hi.to_le_bytes());let p=memory.0.as_mut_ptr().add(positions[location]);wide_halves(p.cast(),lo,hi);print!("wide_halves {} {} {} ",a,b,location);for byte in memory.0{print!("{:02x}",byte);}println!();}
{let mut memory=Memory([0xa5;128]);memory.0[96..104].copy_from_slice(&lo.to_le_bytes());memory.0[104..112].copy_from_slice(&hi.to_le_bytes());let p=memory.0.as_mut_ptr().add(positions[location]);wide_add(p.cast(),memory.0.as_mut_ptr().add(96).cast());print!("wide_add {} {} {} ",a,b,location);for byte in memory.0{print!("{:02x}",byte);}println!();}
}}}}}
