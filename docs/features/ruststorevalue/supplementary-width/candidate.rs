#![allow(dead_code,unused_mut,unused_parens,unused_variables)]
#[repr(C)]struct E<T>{before:[u64;2],value:T,after:[u64;2]}
unsafe fn dump<T>(n:&str,p:*const T){print!("{} ",n);for v in core::slice::from_raw_parts(p as*const u8,core::mem::size_of::<T>()){print!("{:02x}",v);}println!();}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn i8_to_i16(mut p: *mut i16, mut value: i8)
{
  *p = value as i16;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn i8_to_u16(mut p: *mut u16, mut value: i8)
{
  *p = (value as i16) as u16;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn i8_to_i32(mut p: *mut i32, mut value: i8)
{
  *p = value as i32;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn i8_to_u32(mut p: *mut u32, mut value: i8)
{
  *p = (value as i32) as u32;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn i8_to_i64(mut p: *mut i64, mut value: i8)
{
  *p = value as i64;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn i8_to_u64(mut p: *mut u64, mut value: i8)
{
  *p = (value as i64) as u64;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn u8_to_i16(mut p: *mut i16, mut value: u8)
{
  *p = (value as u16) as i16;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn u8_to_u16(mut p: *mut u16, mut value: u8)
{
  *p = value as u16;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn u8_to_i32(mut p: *mut i32, mut value: u8)
{
  *p = (value as u32) as i32;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn u8_to_u32(mut p: *mut u32, mut value: u8)
{
  *p = value as u32;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn u8_to_i64(mut p: *mut i64, mut value: u8)
{
  *p = (value as u64) as i64;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn u8_to_u64(mut p: *mut u64, mut value: u8)
{
  *p = value as u64;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn i16_to_i8(mut p: *mut i8, mut value: i16)
{
  *p = value as i8;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn i16_to_u8(mut p: *mut u8, mut value: i16)
{
  *p = value as u8;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn i16_to_i32(mut p: *mut i32, mut value: i16)
{
  *p = value as i32;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn i16_to_u32(mut p: *mut u32, mut value: i16)
{
  *p = (value as i32) as u32;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn i16_to_i64(mut p: *mut i64, mut value: i16)
{
  *p = value as i64;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn i16_to_u64(mut p: *mut u64, mut value: i16)
{
  *p = (value as i64) as u64;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn u16_to_i8(mut p: *mut i8, mut value: u16)
{
  *p = value as i8;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn u16_to_u8(mut p: *mut u8, mut value: u16)
{
  *p = value as u8;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn u16_to_i32(mut p: *mut i32, mut value: u16)
{
  *p = (value as u32) as i32;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn u16_to_u32(mut p: *mut u32, mut value: u16)
{
  *p = value as u32;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn u16_to_i64(mut p: *mut i64, mut value: u16)
{
  *p = (value as u64) as i64;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn u16_to_u64(mut p: *mut u64, mut value: u16)
{
  *p = value as u64;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn i32_to_i8(mut p: *mut i8, mut value: i32)
{
  *p = value as i8;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn i32_to_u8(mut p: *mut u8, mut value: i32)
{
  *p = value as u8;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn i32_to_i16(mut p: *mut i16, mut value: i32)
{
  *p = value as i16;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn i32_to_u16(mut p: *mut u16, mut value: i32)
{
  *p = value as u16;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn i32_to_i64(mut p: *mut i64, mut value: i32)
{
  *p = value as i64;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn i32_to_u64(mut p: *mut u64, mut value: i32)
{
  *p = (value as i64) as u64;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn u32_to_i8(mut p: *mut i8, mut value: u32)
{
  *p = value as i8;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn u32_to_u8(mut p: *mut u8, mut value: u32)
{
  *p = value as u8;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn u32_to_i16(mut p: *mut i16, mut value: u32)
{
  *p = value as i16;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn u32_to_u16(mut p: *mut u16, mut value: u32)
{
  *p = value as u16;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn u32_to_i64(mut p: *mut i64, mut value: u32)
{
  *p = (value as u64) as i64;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn u32_to_u64(mut p: *mut u64, mut value: u32)
{
  *p = value as u64;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn i64_to_i8(mut p: *mut i8, mut value: i64)
{
  *p = value as i8;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn i64_to_u8(mut p: *mut u8, mut value: i64)
{
  *p = value as u8;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn i64_to_i16(mut p: *mut i16, mut value: i64)
{
  *p = value as i16;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn i64_to_u16(mut p: *mut u16, mut value: i64)
{
  *p = value as u16;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn i64_to_i32(mut p: *mut i32, mut value: i64)
{
  *p = value as i32;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn i64_to_u32(mut p: *mut u32, mut value: i64)
{
  *p = value as u32;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn u64_to_i8(mut p: *mut i8, mut value: u64)
{
  *p = value as i8;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn u64_to_u8(mut p: *mut u8, mut value: u64)
{
  *p = value as u8;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn u64_to_i16(mut p: *mut i16, mut value: u64)
{
  *p = value as i16;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn u64_to_u16(mut p: *mut u16, mut value: u64)
{
  *p = value as u16;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn u64_to_i32(mut p: *mut i32, mut value: u64)
{
  *p = value as i32;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn u64_to_u32(mut p: *mut u32, mut value: u64)
{
  *p = value as u32;
}
fn main(){unsafe{for a in [0u64,1u64,2u64,127u64,128u64,255u64,256u64,32767u64,32768u64,65535u64,65536u64,2147483647u64,2147483648u64,4294967295u64,4294967296u64,9223372036854775807u64,9223372036854775808u64,18446744073709551615u64]{
{let mut s=core::mem::MaybeUninit::<E<i16>>::uninit();core::ptr::write_bytes(s.as_mut_ptr(),0xa5,1);i8_to_i16(core::ptr::addr_of_mut!((*s.as_mut_ptr()).value),a as i8);dump("i8_to_i16",s.as_ptr());}
{let mut s=core::mem::MaybeUninit::<E<u16>>::uninit();core::ptr::write_bytes(s.as_mut_ptr(),0xa5,1);i8_to_u16(core::ptr::addr_of_mut!((*s.as_mut_ptr()).value),a as i8);dump("i8_to_u16",s.as_ptr());}
{let mut s=core::mem::MaybeUninit::<E<i32>>::uninit();core::ptr::write_bytes(s.as_mut_ptr(),0xa5,1);i8_to_i32(core::ptr::addr_of_mut!((*s.as_mut_ptr()).value),a as i8);dump("i8_to_i32",s.as_ptr());}
{let mut s=core::mem::MaybeUninit::<E<u32>>::uninit();core::ptr::write_bytes(s.as_mut_ptr(),0xa5,1);i8_to_u32(core::ptr::addr_of_mut!((*s.as_mut_ptr()).value),a as i8);dump("i8_to_u32",s.as_ptr());}
{let mut s=core::mem::MaybeUninit::<E<i64>>::uninit();core::ptr::write_bytes(s.as_mut_ptr(),0xa5,1);i8_to_i64(core::ptr::addr_of_mut!((*s.as_mut_ptr()).value),a as i8);dump("i8_to_i64",s.as_ptr());}
{let mut s=core::mem::MaybeUninit::<E<u64>>::uninit();core::ptr::write_bytes(s.as_mut_ptr(),0xa5,1);i8_to_u64(core::ptr::addr_of_mut!((*s.as_mut_ptr()).value),a as i8);dump("i8_to_u64",s.as_ptr());}
{let mut s=core::mem::MaybeUninit::<E<i16>>::uninit();core::ptr::write_bytes(s.as_mut_ptr(),0xa5,1);u8_to_i16(core::ptr::addr_of_mut!((*s.as_mut_ptr()).value),a as u8);dump("u8_to_i16",s.as_ptr());}
{let mut s=core::mem::MaybeUninit::<E<u16>>::uninit();core::ptr::write_bytes(s.as_mut_ptr(),0xa5,1);u8_to_u16(core::ptr::addr_of_mut!((*s.as_mut_ptr()).value),a as u8);dump("u8_to_u16",s.as_ptr());}
{let mut s=core::mem::MaybeUninit::<E<i32>>::uninit();core::ptr::write_bytes(s.as_mut_ptr(),0xa5,1);u8_to_i32(core::ptr::addr_of_mut!((*s.as_mut_ptr()).value),a as u8);dump("u8_to_i32",s.as_ptr());}
{let mut s=core::mem::MaybeUninit::<E<u32>>::uninit();core::ptr::write_bytes(s.as_mut_ptr(),0xa5,1);u8_to_u32(core::ptr::addr_of_mut!((*s.as_mut_ptr()).value),a as u8);dump("u8_to_u32",s.as_ptr());}
{let mut s=core::mem::MaybeUninit::<E<i64>>::uninit();core::ptr::write_bytes(s.as_mut_ptr(),0xa5,1);u8_to_i64(core::ptr::addr_of_mut!((*s.as_mut_ptr()).value),a as u8);dump("u8_to_i64",s.as_ptr());}
{let mut s=core::mem::MaybeUninit::<E<u64>>::uninit();core::ptr::write_bytes(s.as_mut_ptr(),0xa5,1);u8_to_u64(core::ptr::addr_of_mut!((*s.as_mut_ptr()).value),a as u8);dump("u8_to_u64",s.as_ptr());}
{let mut s=core::mem::MaybeUninit::<E<i8>>::uninit();core::ptr::write_bytes(s.as_mut_ptr(),0xa5,1);i16_to_i8(core::ptr::addr_of_mut!((*s.as_mut_ptr()).value),a as i16);dump("i16_to_i8",s.as_ptr());}
{let mut s=core::mem::MaybeUninit::<E<u8>>::uninit();core::ptr::write_bytes(s.as_mut_ptr(),0xa5,1);i16_to_u8(core::ptr::addr_of_mut!((*s.as_mut_ptr()).value),a as i16);dump("i16_to_u8",s.as_ptr());}
{let mut s=core::mem::MaybeUninit::<E<i32>>::uninit();core::ptr::write_bytes(s.as_mut_ptr(),0xa5,1);i16_to_i32(core::ptr::addr_of_mut!((*s.as_mut_ptr()).value),a as i16);dump("i16_to_i32",s.as_ptr());}
{let mut s=core::mem::MaybeUninit::<E<u32>>::uninit();core::ptr::write_bytes(s.as_mut_ptr(),0xa5,1);i16_to_u32(core::ptr::addr_of_mut!((*s.as_mut_ptr()).value),a as i16);dump("i16_to_u32",s.as_ptr());}
{let mut s=core::mem::MaybeUninit::<E<i64>>::uninit();core::ptr::write_bytes(s.as_mut_ptr(),0xa5,1);i16_to_i64(core::ptr::addr_of_mut!((*s.as_mut_ptr()).value),a as i16);dump("i16_to_i64",s.as_ptr());}
{let mut s=core::mem::MaybeUninit::<E<u64>>::uninit();core::ptr::write_bytes(s.as_mut_ptr(),0xa5,1);i16_to_u64(core::ptr::addr_of_mut!((*s.as_mut_ptr()).value),a as i16);dump("i16_to_u64",s.as_ptr());}
{let mut s=core::mem::MaybeUninit::<E<i8>>::uninit();core::ptr::write_bytes(s.as_mut_ptr(),0xa5,1);u16_to_i8(core::ptr::addr_of_mut!((*s.as_mut_ptr()).value),a as u16);dump("u16_to_i8",s.as_ptr());}
{let mut s=core::mem::MaybeUninit::<E<u8>>::uninit();core::ptr::write_bytes(s.as_mut_ptr(),0xa5,1);u16_to_u8(core::ptr::addr_of_mut!((*s.as_mut_ptr()).value),a as u16);dump("u16_to_u8",s.as_ptr());}
{let mut s=core::mem::MaybeUninit::<E<i32>>::uninit();core::ptr::write_bytes(s.as_mut_ptr(),0xa5,1);u16_to_i32(core::ptr::addr_of_mut!((*s.as_mut_ptr()).value),a as u16);dump("u16_to_i32",s.as_ptr());}
{let mut s=core::mem::MaybeUninit::<E<u32>>::uninit();core::ptr::write_bytes(s.as_mut_ptr(),0xa5,1);u16_to_u32(core::ptr::addr_of_mut!((*s.as_mut_ptr()).value),a as u16);dump("u16_to_u32",s.as_ptr());}
{let mut s=core::mem::MaybeUninit::<E<i64>>::uninit();core::ptr::write_bytes(s.as_mut_ptr(),0xa5,1);u16_to_i64(core::ptr::addr_of_mut!((*s.as_mut_ptr()).value),a as u16);dump("u16_to_i64",s.as_ptr());}
{let mut s=core::mem::MaybeUninit::<E<u64>>::uninit();core::ptr::write_bytes(s.as_mut_ptr(),0xa5,1);u16_to_u64(core::ptr::addr_of_mut!((*s.as_mut_ptr()).value),a as u16);dump("u16_to_u64",s.as_ptr());}
{let mut s=core::mem::MaybeUninit::<E<i8>>::uninit();core::ptr::write_bytes(s.as_mut_ptr(),0xa5,1);i32_to_i8(core::ptr::addr_of_mut!((*s.as_mut_ptr()).value),a as i32);dump("i32_to_i8",s.as_ptr());}
{let mut s=core::mem::MaybeUninit::<E<u8>>::uninit();core::ptr::write_bytes(s.as_mut_ptr(),0xa5,1);i32_to_u8(core::ptr::addr_of_mut!((*s.as_mut_ptr()).value),a as i32);dump("i32_to_u8",s.as_ptr());}
{let mut s=core::mem::MaybeUninit::<E<i16>>::uninit();core::ptr::write_bytes(s.as_mut_ptr(),0xa5,1);i32_to_i16(core::ptr::addr_of_mut!((*s.as_mut_ptr()).value),a as i32);dump("i32_to_i16",s.as_ptr());}
{let mut s=core::mem::MaybeUninit::<E<u16>>::uninit();core::ptr::write_bytes(s.as_mut_ptr(),0xa5,1);i32_to_u16(core::ptr::addr_of_mut!((*s.as_mut_ptr()).value),a as i32);dump("i32_to_u16",s.as_ptr());}
{let mut s=core::mem::MaybeUninit::<E<i64>>::uninit();core::ptr::write_bytes(s.as_mut_ptr(),0xa5,1);i32_to_i64(core::ptr::addr_of_mut!((*s.as_mut_ptr()).value),a as i32);dump("i32_to_i64",s.as_ptr());}
{let mut s=core::mem::MaybeUninit::<E<u64>>::uninit();core::ptr::write_bytes(s.as_mut_ptr(),0xa5,1);i32_to_u64(core::ptr::addr_of_mut!((*s.as_mut_ptr()).value),a as i32);dump("i32_to_u64",s.as_ptr());}
{let mut s=core::mem::MaybeUninit::<E<i8>>::uninit();core::ptr::write_bytes(s.as_mut_ptr(),0xa5,1);u32_to_i8(core::ptr::addr_of_mut!((*s.as_mut_ptr()).value),a as u32);dump("u32_to_i8",s.as_ptr());}
{let mut s=core::mem::MaybeUninit::<E<u8>>::uninit();core::ptr::write_bytes(s.as_mut_ptr(),0xa5,1);u32_to_u8(core::ptr::addr_of_mut!((*s.as_mut_ptr()).value),a as u32);dump("u32_to_u8",s.as_ptr());}
{let mut s=core::mem::MaybeUninit::<E<i16>>::uninit();core::ptr::write_bytes(s.as_mut_ptr(),0xa5,1);u32_to_i16(core::ptr::addr_of_mut!((*s.as_mut_ptr()).value),a as u32);dump("u32_to_i16",s.as_ptr());}
{let mut s=core::mem::MaybeUninit::<E<u16>>::uninit();core::ptr::write_bytes(s.as_mut_ptr(),0xa5,1);u32_to_u16(core::ptr::addr_of_mut!((*s.as_mut_ptr()).value),a as u32);dump("u32_to_u16",s.as_ptr());}
{let mut s=core::mem::MaybeUninit::<E<i64>>::uninit();core::ptr::write_bytes(s.as_mut_ptr(),0xa5,1);u32_to_i64(core::ptr::addr_of_mut!((*s.as_mut_ptr()).value),a as u32);dump("u32_to_i64",s.as_ptr());}
{let mut s=core::mem::MaybeUninit::<E<u64>>::uninit();core::ptr::write_bytes(s.as_mut_ptr(),0xa5,1);u32_to_u64(core::ptr::addr_of_mut!((*s.as_mut_ptr()).value),a as u32);dump("u32_to_u64",s.as_ptr());}
{let mut s=core::mem::MaybeUninit::<E<i8>>::uninit();core::ptr::write_bytes(s.as_mut_ptr(),0xa5,1);i64_to_i8(core::ptr::addr_of_mut!((*s.as_mut_ptr()).value),a as i64);dump("i64_to_i8",s.as_ptr());}
{let mut s=core::mem::MaybeUninit::<E<u8>>::uninit();core::ptr::write_bytes(s.as_mut_ptr(),0xa5,1);i64_to_u8(core::ptr::addr_of_mut!((*s.as_mut_ptr()).value),a as i64);dump("i64_to_u8",s.as_ptr());}
{let mut s=core::mem::MaybeUninit::<E<i16>>::uninit();core::ptr::write_bytes(s.as_mut_ptr(),0xa5,1);i64_to_i16(core::ptr::addr_of_mut!((*s.as_mut_ptr()).value),a as i64);dump("i64_to_i16",s.as_ptr());}
{let mut s=core::mem::MaybeUninit::<E<u16>>::uninit();core::ptr::write_bytes(s.as_mut_ptr(),0xa5,1);i64_to_u16(core::ptr::addr_of_mut!((*s.as_mut_ptr()).value),a as i64);dump("i64_to_u16",s.as_ptr());}
{let mut s=core::mem::MaybeUninit::<E<i32>>::uninit();core::ptr::write_bytes(s.as_mut_ptr(),0xa5,1);i64_to_i32(core::ptr::addr_of_mut!((*s.as_mut_ptr()).value),a as i64);dump("i64_to_i32",s.as_ptr());}
{let mut s=core::mem::MaybeUninit::<E<u32>>::uninit();core::ptr::write_bytes(s.as_mut_ptr(),0xa5,1);i64_to_u32(core::ptr::addr_of_mut!((*s.as_mut_ptr()).value),a as i64);dump("i64_to_u32",s.as_ptr());}
{let mut s=core::mem::MaybeUninit::<E<i8>>::uninit();core::ptr::write_bytes(s.as_mut_ptr(),0xa5,1);u64_to_i8(core::ptr::addr_of_mut!((*s.as_mut_ptr()).value),a as u64);dump("u64_to_i8",s.as_ptr());}
{let mut s=core::mem::MaybeUninit::<E<u8>>::uninit();core::ptr::write_bytes(s.as_mut_ptr(),0xa5,1);u64_to_u8(core::ptr::addr_of_mut!((*s.as_mut_ptr()).value),a as u64);dump("u64_to_u8",s.as_ptr());}
{let mut s=core::mem::MaybeUninit::<E<i16>>::uninit();core::ptr::write_bytes(s.as_mut_ptr(),0xa5,1);u64_to_i16(core::ptr::addr_of_mut!((*s.as_mut_ptr()).value),a as u64);dump("u64_to_i16",s.as_ptr());}
{let mut s=core::mem::MaybeUninit::<E<u16>>::uninit();core::ptr::write_bytes(s.as_mut_ptr(),0xa5,1);u64_to_u16(core::ptr::addr_of_mut!((*s.as_mut_ptr()).value),a as u64);dump("u64_to_u16",s.as_ptr());}
{let mut s=core::mem::MaybeUninit::<E<i32>>::uninit();core::ptr::write_bytes(s.as_mut_ptr(),0xa5,1);u64_to_i32(core::ptr::addr_of_mut!((*s.as_mut_ptr()).value),a as u64);dump("u64_to_i32",s.as_ptr());}
{let mut s=core::mem::MaybeUninit::<E<u32>>::uninit();core::ptr::write_bytes(s.as_mut_ptr(),0xa5,1);u64_to_u32(core::ptr::addr_of_mut!((*s.as_mut_ptr()).value),a as u64);dump("u64_to_u32",s.as_ptr());}
}}}
