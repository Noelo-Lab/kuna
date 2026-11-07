#![allow(dead_code,unused_variables,unused_parens,unused_mut,non_snake_case)]
#[repr(C,align(16))]struct WideFields{before:u64,padding:[u8;8],value:[u8;16],after:u64}
#[repr(C)]struct Byte16{bytes:[u8;16]}
#[repr(C,align(16))]struct Memory([u8;128]);
static mut TRACE:u64=0;
unsafe fn counted_pointer(p:*mut[u8;16])->*mut[u8;16]{TRACE=TRACE*100+1;p}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn zero_unsigned(mut p: *mut [u8; 16])
{
  *p = 0;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn zero_signed(mut p: *mut [u8; 16])
{
  *p = 0;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn zero_field(mut p: *mut WideFields)
{
  *(&raw mut (*p).value as *mut [u8; 16]) = 0;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn zero_second(mut p: *mut [u8; 16])
{
  *p.wrapping_byte_offset(((1 as i64) as isize).wrapping_mul(16)) = 0;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn zero_index(mut p: *mut [u8; 16], mut i: u64)
{
  *p.wrapping_byte_offset((i as isize).wrapping_mul(16)) = 0;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn zero_callback(mut p: *mut [u8; 16])
{
  *counted_pointer(p) = 0;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn zero_bytes(mut p: *mut Byte16)
{
  *(&raw mut (*p).bytes as *mut [u8; 16]) = 0;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn one_unsigned(mut p: *mut [u8; 16])
{
  *(&raw mut *p as *mut u64) = 1;
  *(((&raw mut *p).wrapping_byte_offset(((8 as i64) as isize).wrapping_mul(1)) as *mut u8) as *mut u64) = 0;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn signed_input(mut p: *mut [u8; 16], mut x: i64)
{
  *(&raw mut *p as *mut i64) = x;
  *(((&raw mut *p).wrapping_byte_offset(((8 as i64) as isize).wrapping_mul(1)) as *mut u8) as *mut i64) = x >> 0x3f;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn unsigned_input(mut p: *mut [u8; 16], mut x: u64)
{
  *(&raw mut *p as *mut u64) = x;
  *(((&raw mut *p).wrapping_byte_offset(((8 as i64) as isize).wrapping_mul(1)) as *mut u8) as *mut u64) = 0;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn two_halves(mut p: *mut [u8; 16], mut lo: u64, mut hi: u64)
{
  *(&raw mut *p as *mut u64) = lo;
  *(((&raw mut *p).wrapping_byte_offset(((8 as i64) as isize).wrapping_mul(1)) as *mut u8) as *mut u64) = hi;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn copy_wide(mut p: *mut [u8; 16], mut q: *mut [u8; 16])
{
  let mut v1: u64; // xmm0_qb
  
  v1 = *(((&raw mut *q).wrapping_byte_offset(((8 as i64) as isize).wrapping_mul(1)) as *mut u8) as *mut u64);
  *(&raw mut *p as *mut u64) = *(&raw mut *q as *mut u64);
  *(((&raw mut *p).wrapping_byte_offset(((8 as i64) as isize).wrapping_mul(1)) as *mut u8) as *mut u64) = v1;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn zero_narrow(mut p: *mut u64)
{
  *p = 0;
}

fn main(){unsafe{assert_eq!(std::mem::offset_of!(WideFields,value),16);assert_eq!(std::mem::size_of::<WideFields>(),48);let values:[u64;8]=[0,1,2,9223372036854775807,9223372036854775808,18446744073709551615,81985529216486895,18364758544493064720];for sample in 0..8{let lo=values[sample];let hi=values[7-sample];for location in 0..2{
{let mut memory=Memory([0xa5;128]);memory.0[96..104].copy_from_slice(&lo.to_le_bytes());memory.0[104..112].copy_from_slice(&hi.to_le_bytes());let p=memory.0.as_mut_ptr().add(16+location*16+0);TRACE=0;zero_unsigned(p.cast());let trace=TRACE;print!("zero_unsigned {} {} {} ",sample,location,trace);for b in memory.0{print!("{:02x}",b);}println!();}
{let mut memory=Memory([0xa5;128]);memory.0[96..104].copy_from_slice(&lo.to_le_bytes());memory.0[104..112].copy_from_slice(&hi.to_le_bytes());let p=memory.0.as_mut_ptr().add(16+location*16+0);TRACE=0;zero_signed(p.cast());let trace=TRACE;print!("zero_signed {} {} {} ",sample,location,trace);for b in memory.0{print!("{:02x}",b);}println!();}
{let mut memory=Memory([0xa5;128]);memory.0[96..104].copy_from_slice(&lo.to_le_bytes());memory.0[104..112].copy_from_slice(&hi.to_le_bytes());let p=memory.0.as_mut_ptr().add(16+location*16+0);TRACE=0;zero_field(p.cast());let trace=TRACE;print!("zero_field {} {} {} ",sample,location,trace);for b in memory.0{print!("{:02x}",b);}println!();}
{let mut memory=Memory([0xa5;128]);memory.0[96..104].copy_from_slice(&lo.to_le_bytes());memory.0[104..112].copy_from_slice(&hi.to_le_bytes());let p=memory.0.as_mut_ptr().add(16+location*16+0);TRACE=0;zero_second(p.cast());let trace=TRACE;print!("zero_second {} {} {} ",sample,location,trace);for b in memory.0{print!("{:02x}",b);}println!();}
{let mut memory=Memory([0xa5;128]);memory.0[96..104].copy_from_slice(&lo.to_le_bytes());memory.0[104..112].copy_from_slice(&hi.to_le_bytes());let p=memory.0.as_mut_ptr().add(16+location*16+0);TRACE=0;zero_index(p.cast(),1);let trace=TRACE;print!("zero_index {} {} {} ",sample,location,trace);for b in memory.0{print!("{:02x}",b);}println!();}
{let mut memory=Memory([0xa5;128]);memory.0[96..104].copy_from_slice(&lo.to_le_bytes());memory.0[104..112].copy_from_slice(&hi.to_le_bytes());let p=memory.0.as_mut_ptr().add(16+location*16+0);TRACE=0;zero_callback(p.cast());let trace=TRACE;print!("zero_callback {} {} {} ",sample,location,trace);for b in memory.0{print!("{:02x}",b);}println!();}
{let mut memory=Memory([0xa5;128]);memory.0[96..104].copy_from_slice(&lo.to_le_bytes());memory.0[104..112].copy_from_slice(&hi.to_le_bytes());let p=memory.0.as_mut_ptr().add(16+location*16+1);TRACE=0;zero_bytes(p.cast());let trace=TRACE;print!("zero_bytes {} {} {} ",sample,location,trace);for b in memory.0{print!("{:02x}",b);}println!();}
{let mut memory=Memory([0xa5;128]);memory.0[96..104].copy_from_slice(&lo.to_le_bytes());memory.0[104..112].copy_from_slice(&hi.to_le_bytes());let p=memory.0.as_mut_ptr().add(16+location*16+0);TRACE=0;one_unsigned(p.cast());let trace=TRACE;print!("one_unsigned {} {} {} ",sample,location,trace);for b in memory.0{print!("{:02x}",b);}println!();}
{let mut memory=Memory([0xa5;128]);memory.0[96..104].copy_from_slice(&lo.to_le_bytes());memory.0[104..112].copy_from_slice(&hi.to_le_bytes());let p=memory.0.as_mut_ptr().add(16+location*16+0);TRACE=0;signed_input(p.cast(),lo as i64);let trace=TRACE;print!("signed_input {} {} {} ",sample,location,trace);for b in memory.0{print!("{:02x}",b);}println!();}
{let mut memory=Memory([0xa5;128]);memory.0[96..104].copy_from_slice(&lo.to_le_bytes());memory.0[104..112].copy_from_slice(&hi.to_le_bytes());let p=memory.0.as_mut_ptr().add(16+location*16+0);TRACE=0;unsigned_input(p.cast(),lo);let trace=TRACE;print!("unsigned_input {} {} {} ",sample,location,trace);for b in memory.0{print!("{:02x}",b);}println!();}
{let mut memory=Memory([0xa5;128]);memory.0[96..104].copy_from_slice(&lo.to_le_bytes());memory.0[104..112].copy_from_slice(&hi.to_le_bytes());let p=memory.0.as_mut_ptr().add(16+location*16+0);TRACE=0;two_halves(p.cast(),lo,hi);let trace=TRACE;print!("two_halves {} {} {} ",sample,location,trace);for b in memory.0{print!("{:02x}",b);}println!();}
{let mut memory=Memory([0xa5;128]);memory.0[96..104].copy_from_slice(&lo.to_le_bytes());memory.0[104..112].copy_from_slice(&hi.to_le_bytes());let p=memory.0.as_mut_ptr().add(16+location*16+0);TRACE=0;copy_wide(p.cast(),memory.0.as_mut_ptr().add(96).cast());let trace=TRACE;print!("copy_wide {} {} {} ",sample,location,trace);for b in memory.0{print!("{:02x}",b);}println!();}
{let mut memory=Memory([0xa5;128]);memory.0[96..104].copy_from_slice(&lo.to_le_bytes());memory.0[104..112].copy_from_slice(&hi.to_le_bytes());let p=memory.0.as_mut_ptr().add(16+location*16+0);TRACE=0;zero_narrow(p.cast());let trace=TRACE;print!("zero_narrow {} {} {} ",sample,location,trace);for b in memory.0{print!("{:02x}",b);}println!();}
}}}}
