#![allow(dead_code,unused_variables,unused_parens,unused_mut,non_snake_case)]
#[repr(C)]struct AddressFields{before:u64,i8:[i8;9],u8:[u8;9],i16:[i16;9],u16:[u16;9],i32:[i32;9],u32:[u32;9],i64:[i64;9],u64:[u64;9],float32:[f32;9],float64:[f64;9],pointers:[*mut u8;9],flags:[bool;9],after:u64}
static mut TRACE:u64=0;
unsafe fn counted_index(i:i64)->i64{TRACE=TRACE*10+1;i}
unsafe fn counted_base(p:*mut AddressFields)->*mut AddressFields{TRACE=TRACE*10+2;p}
extern "C"{fn mmap(p:*mut std::ffi::c_void,n:usize,prot:i32,flags:i32,fd:i32,off:isize)->*mut std::ffi::c_void;fn mprotect(p:*mut std::ffi::c_void,n:usize,prot:i32)->i32;fn munmap(p:*mut std::ffi::c_void,n:usize)->i32;}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn address_i8_s(mut p: *mut AddressFields, mut i: i64) -> *mut i8
{
  return (&raw mut (*p).i8).wrapping_byte_offset((i as isize).wrapping_mul(1)) as *mut i8;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn address_i8_u(mut p: *mut AddressFields, mut i: u64) -> *mut i8
{
  return (&raw mut (*p).i8).wrapping_byte_offset((i as isize).wrapping_mul(1)) as *mut i8;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn address_u8_s(mut p: *mut AddressFields, mut i: i64) -> *mut u8
{
  return (&raw mut (*p).u8).wrapping_byte_offset((i as isize).wrapping_mul(1)) as *mut u8;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn address_u8_u(mut p: *mut AddressFields, mut i: u64) -> *mut u8
{
  return (&raw mut (*p).u8).wrapping_byte_offset((i as isize).wrapping_mul(1)) as *mut u8;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn address_i16_s(mut p: *mut AddressFields, mut i: i64) -> *mut i16
{
  return (&raw mut (*p).i16).wrapping_byte_offset((i as isize).wrapping_mul(2)) as *mut i16;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn address_i16_u(mut p: *mut AddressFields, mut i: u64) -> *mut i16
{
  return (&raw mut (*p).i16).wrapping_byte_offset((i as isize).wrapping_mul(2)) as *mut i16;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn address_u16_s(mut p: *mut AddressFields, mut i: i64) -> *mut u16
{
  return (&raw mut (*p).u16).wrapping_byte_offset((i as isize).wrapping_mul(2)) as *mut u16;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn address_u16_u(mut p: *mut AddressFields, mut i: u64) -> *mut u16
{
  return (&raw mut (*p).u16).wrapping_byte_offset((i as isize).wrapping_mul(2)) as *mut u16;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn address_i32_s(mut p: *mut AddressFields, mut i: i64) -> *mut i32
{
  return (&raw mut (*p).i32).wrapping_byte_offset((i as isize).wrapping_mul(4)) as *mut i32;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn address_i32_u(mut p: *mut AddressFields, mut i: u64) -> *mut i32
{
  return (&raw mut (*p).i32).wrapping_byte_offset((i as isize).wrapping_mul(4)) as *mut i32;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn address_u32_s(mut p: *mut AddressFields, mut i: i64) -> *mut u32
{
  return (&raw mut (*p).u32).wrapping_byte_offset((i as isize).wrapping_mul(4)) as *mut u32;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn address_u32_u(mut p: *mut AddressFields, mut i: u64) -> *mut u32
{
  return (&raw mut (*p).u32).wrapping_byte_offset((i as isize).wrapping_mul(4)) as *mut u32;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn address_i64_s(mut p: *mut AddressFields, mut i: i64) -> *mut i64
{
  return (&raw mut (*p).i64).wrapping_byte_offset((i as isize).wrapping_mul(8)) as *mut i64;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn address_i64_u(mut p: *mut AddressFields, mut i: u64) -> *mut i64
{
  return (&raw mut (*p).i64).wrapping_byte_offset((i as isize).wrapping_mul(8)) as *mut i64;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn address_u64_s(mut p: *mut AddressFields, mut i: i64) -> *mut u64
{
  return (&raw mut (*p).u64).wrapping_byte_offset((i as isize).wrapping_mul(8)) as *mut u64;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn address_u64_u(mut p: *mut AddressFields, mut i: u64) -> *mut u64
{
  return (&raw mut (*p).u64).wrapping_byte_offset((i as isize).wrapping_mul(8)) as *mut u64;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn address_float32_s(mut p: *mut AddressFields, mut i: i64) -> *mut f32
{
  return (&raw mut (*p).float32).wrapping_byte_offset((i as isize).wrapping_mul(4)) as *mut f32;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn address_float32_u(mut p: *mut AddressFields, mut i: u64) -> *mut f32
{
  return (&raw mut (*p).float32).wrapping_byte_offset((i as isize).wrapping_mul(4)) as *mut f32;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn address_float64_s(mut p: *mut AddressFields, mut i: i64) -> *mut f64
{
  return (&raw mut (*p).float64).wrapping_byte_offset((i as isize).wrapping_mul(8)) as *mut f64;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn address_float64_u(mut p: *mut AddressFields, mut i: u64) -> *mut f64
{
  return (&raw mut (*p).float64).wrapping_byte_offset((i as isize).wrapping_mul(8)) as *mut f64;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn address_pointers_s(mut p: *mut AddressFields, mut i: i64) -> *mut *mut u8
{
  return (&raw mut (*p).pointers).wrapping_byte_offset((i as isize).wrapping_mul(8)) as *mut *mut u8;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn address_pointers_u(mut p: *mut AddressFields, mut i: u64) -> *mut *mut u8
{
  return (&raw mut (*p).pointers).wrapping_byte_offset((i as isize).wrapping_mul(8)) as *mut *mut u8;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn address_flags_s(mut p: *mut AddressFields, mut i: i64) -> *mut bool
{
  return (&raw mut (*p).flags).wrapping_byte_offset((i as isize).wrapping_mul(1)) as *mut bool;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn address_flags_u(mut p: *mut AddressFields, mut i: u64) -> *mut bool
{
  return (&raw mut (*p).flags).wrapping_byte_offset((i as isize).wrapping_mul(1)) as *mut bool;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn nested_address_s(mut p: *mut [i32; 9], mut i: i64, mut j: i64) -> *mut i32
{
  return (&raw mut *p.wrapping_byte_offset((i as isize).wrapping_mul(36))).wrapping_byte_offset((j as isize).wrapping_mul(4)) as *mut i32;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn nested_address_u(mut p: *mut [i32; 9], mut i: i64, mut j: u64) -> *mut i32
{
  return (&raw mut *p.wrapping_byte_offset((i as isize).wrapping_mul(36))).wrapping_byte_offset((j as isize).wrapping_mul(4)) as *mut i32;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn callback_address(mut p: *mut AddressFields, mut i: i64) -> *mut i32
{
  let mut v1: i64; // rax | rbx
  
  v1 = counted_index(i);
  return (&raw mut (*counted_base(p)).i32).wrapping_byte_offset((v1 as isize).wrapping_mul(4)) as *mut i32;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn wrap_signed(mut p: *mut AddressFields, mut i: i64) -> *mut i32
{
  return (&raw mut (*p).i32).wrapping_byte_offset((i as isize).wrapping_mul(4)) as *mut i32;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn wrap_unsigned(mut p: *mut AddressFields, mut i: u64) -> *mut u32
{
  return ((&raw mut (*p).i32).wrapping_byte_offset((i as isize).wrapping_mul(4)) as *mut i32) as *mut u32;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn wrap_byte(mut p: *mut AddressFields, mut i: u64) -> *mut u8
{
  return ((&raw mut (*p).i32).wrapping_byte_offset((i as isize).wrapping_mul(4)) as *mut i32) as *mut u8;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn logical_address(mut p: *mut AddressFields, mut n: u32) -> *mut i32
{
  return (&raw mut (*p).i32).wrapping_byte_offset(((::core::convert::identity::<u64>(0xffffffffffffffff >> (n as u8 & 0x3f)) as u64) as isize).wrapping_mul(4)) as *mut i32;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn signed_address(mut p: *mut AddressFields, mut n: u32) -> *mut i32
{
  return (&raw mut (*p).i32).wrapping_byte_offset(((::core::convert::identity::<i64>(-0x8000000000000000 >> (n as u8 & 0x3f)) as i64) as isize).wrapping_mul(4)) as *mut i32;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn left_address(mut p: *mut AddressFields, mut n: u32) -> *mut i32
{
  return ((*p).i32 as i64 + (4 << (n as u8 & 0x3f))) as *mut i32;
}
fn main(){unsafe{let values=[0u64,1u64,2u64,7u64,8u64,9u64,0x7fu64,0x80u64,0x7fffffffu64,0x80000000u64,0xffffffffu64,0x7fffffffffffffffu64,0x8000000000000000u64,0xffffffffffffffffu64,0x0123456789abcdefu64,0xfedcba9876543210u64,0xaaaaaaaa55555555u64,0x4000000000000000u64];for mode in 0..2{let memory=mmap(std::ptr::null_mut(),4096,3,0x22,-1,0);assert_ne!(memory as isize,-1);std::ptr::write_bytes(memory as *mut u8,0xa5,4096);let p=memory as *mut AddressFields;let np=(memory.cast::<u8>().add(1024)as *mut[i32;9]).add(2);if mode!=0{assert_eq!(mprotect(memory,4096,0),0);}println!("mode {}",mode);
for i in 0i64..=9{println!("address_i8_s {} {:016x}",i,(address_i8_s(p,i as i64)as usize).wrapping_sub(p as usize)as u64);}
for i in 0i64..=9{println!("address_i8_u {} {:016x}",i,(address_i8_u(p,i as u64)as usize).wrapping_sub(p as usize)as u64);}
for i in 0i64..=9{println!("address_u8_s {} {:016x}",i,(address_u8_s(p,i as i64)as usize).wrapping_sub(p as usize)as u64);}
for i in 0i64..=9{println!("address_u8_u {} {:016x}",i,(address_u8_u(p,i as u64)as usize).wrapping_sub(p as usize)as u64);}
for i in 0i64..=9{println!("address_i16_s {} {:016x}",i,(address_i16_s(p,i as i64)as usize).wrapping_sub(p as usize)as u64);}
for i in 0i64..=9{println!("address_i16_u {} {:016x}",i,(address_i16_u(p,i as u64)as usize).wrapping_sub(p as usize)as u64);}
for i in 0i64..=9{println!("address_u16_s {} {:016x}",i,(address_u16_s(p,i as i64)as usize).wrapping_sub(p as usize)as u64);}
for i in 0i64..=9{println!("address_u16_u {} {:016x}",i,(address_u16_u(p,i as u64)as usize).wrapping_sub(p as usize)as u64);}
for i in 0i64..=9{println!("address_i32_s {} {:016x}",i,(address_i32_s(p,i as i64)as usize).wrapping_sub(p as usize)as u64);}
for i in 0i64..=9{println!("address_i32_u {} {:016x}",i,(address_i32_u(p,i as u64)as usize).wrapping_sub(p as usize)as u64);}
for i in 0i64..=9{println!("address_u32_s {} {:016x}",i,(address_u32_s(p,i as i64)as usize).wrapping_sub(p as usize)as u64);}
for i in 0i64..=9{println!("address_u32_u {} {:016x}",i,(address_u32_u(p,i as u64)as usize).wrapping_sub(p as usize)as u64);}
for i in 0i64..=9{println!("address_i64_s {} {:016x}",i,(address_i64_s(p,i as i64)as usize).wrapping_sub(p as usize)as u64);}
for i in 0i64..=9{println!("address_i64_u {} {:016x}",i,(address_i64_u(p,i as u64)as usize).wrapping_sub(p as usize)as u64);}
for i in 0i64..=9{println!("address_u64_s {} {:016x}",i,(address_u64_s(p,i as i64)as usize).wrapping_sub(p as usize)as u64);}
for i in 0i64..=9{println!("address_u64_u {} {:016x}",i,(address_u64_u(p,i as u64)as usize).wrapping_sub(p as usize)as u64);}
for i in 0i64..=9{println!("address_float32_s {} {:016x}",i,(address_float32_s(p,i as i64)as usize).wrapping_sub(p as usize)as u64);}
for i in 0i64..=9{println!("address_float32_u {} {:016x}",i,(address_float32_u(p,i as u64)as usize).wrapping_sub(p as usize)as u64);}
for i in 0i64..=9{println!("address_float64_s {} {:016x}",i,(address_float64_s(p,i as i64)as usize).wrapping_sub(p as usize)as u64);}
for i in 0i64..=9{println!("address_float64_u {} {:016x}",i,(address_float64_u(p,i as u64)as usize).wrapping_sub(p as usize)as u64);}
for i in 0i64..=9{println!("address_pointers_s {} {:016x}",i,(address_pointers_s(p,i as i64)as usize).wrapping_sub(p as usize)as u64);}
for i in 0i64..=9{println!("address_pointers_u {} {:016x}",i,(address_pointers_u(p,i as u64)as usize).wrapping_sub(p as usize)as u64);}
for i in 0i64..=9{println!("address_flags_s {} {:016x}",i,(address_flags_s(p,i as i64)as usize).wrapping_sub(p as usize)as u64);}
for i in 0i64..=9{println!("address_flags_u {} {:016x}",i,(address_flags_u(p,i as u64)as usize).wrapping_sub(p as usize)as u64);}
for i in -1i64..=1{for j in 0i64..=9{println!("nested_address_s {} {} {:016x}",i,j,(nested_address_s(np,i,j as i64)as usize).wrapping_sub(np as usize)as u64);}}
for i in -1i64..=1{for j in 0i64..=9{println!("nested_address_u {} {} {:016x}",i,j,(nested_address_u(np,i,j as u64)as usize).wrapping_sub(np as usize)as u64);}}
for i in 0i64..=9{TRACE=0;let result=callback_address(p,i)as usize;let trace=TRACE;println!("callback_address {} {:016x} {}",i,result.wrapping_sub(p as usize)as u64,trace);}
for i in values{println!("wrap_signed {:016x} {:016x}",i,(wrap_signed(p,i as i64)as usize).wrapping_sub(p as usize)as u64);}
for i in values{println!("wrap_unsigned {:016x} {:016x}",i,(wrap_unsigned(p,i as u64)as usize).wrapping_sub(p as usize)as u64);}
for i in values{println!("wrap_byte {:016x} {:016x}",i,(wrap_byte(p,i as u64)as usize).wrapping_sub(p as usize)as u64);}
for n in 0u32..64{println!("logical_address {} {:016x}",n,(logical_address(p,n)as usize).wrapping_sub(p as usize)as u64);}
for n in 0u32..64{println!("signed_address {} {:016x}",n,(signed_address(p,n)as usize).wrapping_sub(p as usize)as u64);}
for n in 0u32..64{println!("left_address {} {:016x}",n,(left_address(p,n)as usize).wrapping_sub(p as usize)as u64);}
assert_eq!(munmap(memory,4096),0);}}}
