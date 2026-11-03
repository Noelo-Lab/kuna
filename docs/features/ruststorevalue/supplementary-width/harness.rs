#![allow(dead_code,unused_mut,unused_parens,unused_variables)]
#[repr(C)]struct E<T>{before:[u64;2],value:T,after:[u64;2]}
unsafe fn dump<T>(n:&str,p:*const T){print!("{} ",n);for v in core::slice::from_raw_parts(p as*const u8,core::mem::size_of::<T>()){print!("{:02x}",v);}println!();}
/* BODIES */
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
