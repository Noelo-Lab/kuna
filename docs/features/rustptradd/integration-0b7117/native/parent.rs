#![allow(dead_code,unused_mut,unused_variables,unused_parens,non_snake_case)]
#[repr(C)]#[derive(Clone,Copy)]struct Record{before:i64,value:i32,tag:u32}
static mut TRACE:u64=0;
unsafe fn counted_index(i:i64)->i64{TRACE=TRACE*10+1;i}
unsafe fn counted_base(p:*mut i64)->*mut i64{TRACE=TRACE*10+2;p}
unsafe fn counted_value(v:i64)->i64{TRACE=TRACE*10+3;v}
fn bytes<T>(v:&[T]){let b=unsafe{std::slice::from_raw_parts(v.as_ptr() as *const u8,std::mem::size_of_val(v))};for x in b{print!("{:02x}",x);}println!();}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn load_i8_s(mut p: *mut i8, mut i: i64) -> i8
{
  return p[i];
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn store_i8_s(mut p: *mut i8, mut i: i64, mut value: i8)
{
  p[i] = value;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn address_i8_s(mut p: *mut i8, mut i: i64) -> *mut i8
{
  return &mut p[i];
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn load_i8_u(mut p: *mut i8, mut i: u64) -> i8
{
  return p[i];
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn store_i8_u(mut p: *mut i8, mut i: u64, mut value: i8)
{
  p[i] = value;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn address_i8_u(mut p: *mut i8, mut i: u64) -> *mut i8
{
  return &mut p[i];
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn load_u8_s(mut p: *mut u8, mut i: i64) -> u8
{
  return p[i];
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn store_u8_s(mut p: *mut u8, mut i: i64, mut value: u8)
{
  p[i] = value;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn address_u8_s(mut p: *mut u8, mut i: i64) -> *mut u8
{
  return &mut p[i];
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn load_u8_u(mut p: *mut u8, mut i: u64) -> u8
{
  return p[i];
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn store_u8_u(mut p: *mut u8, mut i: u64, mut value: u8)
{
  p[i] = value;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn address_u8_u(mut p: *mut u8, mut i: u64) -> *mut u8
{
  return &mut p[i];
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn load_i16_s(mut p: *mut i16, mut i: i64) -> i16
{
  return p[i];
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn store_i16_s(mut p: *mut i16, mut i: i64, mut value: i16)
{
  p[i] = value;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn address_i16_s(mut p: *mut i16, mut i: i64) -> *mut i16
{
  return &mut p[i];
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn load_i16_u(mut p: *mut i16, mut i: u64) -> i16
{
  return p[i];
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn store_i16_u(mut p: *mut i16, mut i: u64, mut value: i16)
{
  p[i] = value;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn address_i16_u(mut p: *mut i16, mut i: u64) -> *mut i16
{
  return &mut p[i];
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn load_u16_s(mut p: *mut u16, mut i: i64) -> u16
{
  return p[i];
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn store_u16_s(mut p: *mut u16, mut i: i64, mut value: u16)
{
  p[i] = value;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn address_u16_s(mut p: *mut u16, mut i: i64) -> *mut u16
{
  return &mut p[i];
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn load_u16_u(mut p: *mut u16, mut i: u64) -> u16
{
  return p[i];
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn store_u16_u(mut p: *mut u16, mut i: u64, mut value: u16)
{
  p[i] = value;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn address_u16_u(mut p: *mut u16, mut i: u64) -> *mut u16
{
  return &mut p[i];
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn load_i32_s(mut p: *mut i32, mut i: i64) -> i32
{
  return p[i];
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn store_i32_s(mut p: *mut i32, mut i: i64, mut value: i32)
{
  p[i] = value;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn address_i32_s(mut p: *mut i32, mut i: i64) -> *mut i32
{
  return &mut p[i];
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn load_i32_u(mut p: *mut i32, mut i: u64) -> i32
{
  return p[i];
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn store_i32_u(mut p: *mut i32, mut i: u64, mut value: i32)
{
  p[i] = value;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn address_i32_u(mut p: *mut i32, mut i: u64) -> *mut i32
{
  return &mut p[i];
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn load_u32_s(mut p: *mut u32, mut i: i64) -> u32
{
  return p[i];
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn store_u32_s(mut p: *mut u32, mut i: i64, mut value: u32)
{
  p[i] = value;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn address_u32_s(mut p: *mut u32, mut i: i64) -> *mut u32
{
  return &mut p[i];
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn load_u32_u(mut p: *mut u32, mut i: u64) -> u32
{
  return p[i];
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn store_u32_u(mut p: *mut u32, mut i: u64, mut value: u32)
{
  p[i] = value;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn address_u32_u(mut p: *mut u32, mut i: u64) -> *mut u32
{
  return &mut p[i];
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn load_i64_s(mut p: *mut i64, mut i: i64) -> i64
{
  return p[i];
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn store_i64_s(mut p: *mut i64, mut i: i64, mut value: i64)
{
  p[i] = value;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn address_i64_s(mut p: *mut i64, mut i: i64) -> *mut i64
{
  return &mut p[i];
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn load_i64_u(mut p: *mut i64, mut i: u64) -> i64
{
  return p[i];
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn store_i64_u(mut p: *mut i64, mut i: u64, mut value: i64)
{
  p[i] = value;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn address_i64_u(mut p: *mut i64, mut i: u64) -> *mut i64
{
  return &mut p[i];
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn load_u64_s(mut p: *mut u64, mut i: i64) -> u64
{
  return p[i];
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn store_u64_s(mut p: *mut u64, mut i: i64, mut value: u64)
{
  p[i] = value;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn address_u64_s(mut p: *mut u64, mut i: i64) -> *mut u64
{
  return &mut p[i];
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn load_u64_u(mut p: *mut u64, mut i: u64) -> u64
{
  return p[i];
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn store_u64_u(mut p: *mut u64, mut i: u64, mut value: u64)
{
  p[i] = value;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn address_u64_u(mut p: *mut u64, mut i: u64) -> *mut u64
{
  return &mut p[i];
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn field_load(mut p: *mut Record, mut i: i64) -> i32
{
  return p[i].value;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn field_store(mut p: *mut Record, mut i: i64, mut value: i32)
{
  p[i].value = value;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn callback_load(mut p: *mut i64, mut i: i64) -> i64
{
  let mut v1: i64; // rax | rbx
  
  v1 = counted_index(i);
  return counted_base(p)[v1];
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn callback_store(mut p: *mut i64, mut i: i64, mut value: i64)
{
  let mut v1: i64; // rax | rbp
  let mut v2: i64; // rax | rbx
  
  v1 = counted_value(value);
  v2 = counted_index(i);
  counted_base(p)[v2] = v1;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn callback_address(mut p: *mut i64, mut i: i64) -> *mut i64
{
  let mut v1: i64; // rax | rbx
  
  v1 = counted_index(i);
  return &mut counted_base(p)[v1];
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn zero_load(mut p: *mut i32) -> i32
{
  return *p;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn shift_address(mut p: *mut i32, mut n: u32) -> *mut i32
{
  return ((4 << (n as u8 & 0x3f)) + p as i64) as *mut i32;
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn logical_address(mut p: *mut i32, mut n: u32) -> *mut i32
{
  return &mut p[0xffffffffffffffff >> (n as u8 & 0x3f)];
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn signed_address(mut p: *mut i32, mut n: u32) -> *mut i32
{
  return &mut p[-0x8000000000000000 >> (n as u8 & 0x3f)];
}
fn main(){let values:[u64;18]=[0,1,2,0x7f,0x80,0xff,0x7fff,0x8000,0xffff,0x7fffffff,0x80000000,0xffffffff,0x7fffffffffffffff,0x8000000000000000,0xffffffffffffffff,0x0123456789abcdef,0xfedcba9876543210,0xaaaaaaaa55555555];unsafe{for v in values{for i in -7i64..=7{
{let mut a:[i8;17]=[0;17];for k in 0..17{a[k]=(0x9e3779b97f4a7c15u64.wrapping_mul(k as u64).wrapping_add(v)) as i8;}let p=a.as_mut_ptr().add(8);println!("load_i8_s {:016x}",load_i8_s(p,i as i64) as u64);}
{let mut a:[i8;17]=[0;17];for k in 0..17{a[k]=(0x9e3779b97f4a7c15u64.wrapping_mul(k as u64).wrapping_add(v)) as i8;}let p=a.as_mut_ptr().add(8);store_i8_s(p,i as i64,v as i8);print!("store_i8_s ");bytes(&a);}
{let mut a:[i8;17]=[0;17];for k in 0..17{a[k]=(0x9e3779b97f4a7c15u64.wrapping_mul(k as u64).wrapping_add(v)) as i8;}let p=a.as_mut_ptr().add(8);println!("address_i8_s {:016x}",(address_i8_s(p,v as i64) as u64).wrapping_sub(p as u64));}
{let mut a:[i8;17]=[0;17];for k in 0..17{a[k]=(0x9e3779b97f4a7c15u64.wrapping_mul(k as u64).wrapping_add(v)) as i8;}let p=a.as_mut_ptr().add(8);println!("load_i8_u {:016x}",load_i8_u(p,i.unsigned_abs() as u64) as u64);}
{let mut a:[i8;17]=[0;17];for k in 0..17{a[k]=(0x9e3779b97f4a7c15u64.wrapping_mul(k as u64).wrapping_add(v)) as i8;}let p=a.as_mut_ptr().add(8);store_i8_u(p,i.unsigned_abs() as u64,v as i8);print!("store_i8_u ");bytes(&a);}
{let mut a:[i8;17]=[0;17];for k in 0..17{a[k]=(0x9e3779b97f4a7c15u64.wrapping_mul(k as u64).wrapping_add(v)) as i8;}let p=a.as_mut_ptr().add(8);println!("address_i8_u {:016x}",(address_i8_u(p,v as u64) as u64).wrapping_sub(p as u64));}
{let mut a:[u8;17]=[0;17];for k in 0..17{a[k]=(0x9e3779b97f4a7c15u64.wrapping_mul(k as u64).wrapping_add(v)) as u8;}let p=a.as_mut_ptr().add(8);println!("load_u8_s {:016x}",load_u8_s(p,i as i64) as u64);}
{let mut a:[u8;17]=[0;17];for k in 0..17{a[k]=(0x9e3779b97f4a7c15u64.wrapping_mul(k as u64).wrapping_add(v)) as u8;}let p=a.as_mut_ptr().add(8);store_u8_s(p,i as i64,v as u8);print!("store_u8_s ");bytes(&a);}
{let mut a:[u8;17]=[0;17];for k in 0..17{a[k]=(0x9e3779b97f4a7c15u64.wrapping_mul(k as u64).wrapping_add(v)) as u8;}let p=a.as_mut_ptr().add(8);println!("address_u8_s {:016x}",(address_u8_s(p,v as i64) as u64).wrapping_sub(p as u64));}
{let mut a:[u8;17]=[0;17];for k in 0..17{a[k]=(0x9e3779b97f4a7c15u64.wrapping_mul(k as u64).wrapping_add(v)) as u8;}let p=a.as_mut_ptr().add(8);println!("load_u8_u {:016x}",load_u8_u(p,i.unsigned_abs() as u64) as u64);}
{let mut a:[u8;17]=[0;17];for k in 0..17{a[k]=(0x9e3779b97f4a7c15u64.wrapping_mul(k as u64).wrapping_add(v)) as u8;}let p=a.as_mut_ptr().add(8);store_u8_u(p,i.unsigned_abs() as u64,v as u8);print!("store_u8_u ");bytes(&a);}
{let mut a:[u8;17]=[0;17];for k in 0..17{a[k]=(0x9e3779b97f4a7c15u64.wrapping_mul(k as u64).wrapping_add(v)) as u8;}let p=a.as_mut_ptr().add(8);println!("address_u8_u {:016x}",(address_u8_u(p,v as u64) as u64).wrapping_sub(p as u64));}
{let mut a:[i16;17]=[0;17];for k in 0..17{a[k]=(0x9e3779b97f4a7c15u64.wrapping_mul(k as u64).wrapping_add(v)) as i16;}let p=a.as_mut_ptr().add(8);println!("load_i16_s {:016x}",load_i16_s(p,i as i64) as u64);}
{let mut a:[i16;17]=[0;17];for k in 0..17{a[k]=(0x9e3779b97f4a7c15u64.wrapping_mul(k as u64).wrapping_add(v)) as i16;}let p=a.as_mut_ptr().add(8);store_i16_s(p,i as i64,v as i16);print!("store_i16_s ");bytes(&a);}
{let mut a:[i16;17]=[0;17];for k in 0..17{a[k]=(0x9e3779b97f4a7c15u64.wrapping_mul(k as u64).wrapping_add(v)) as i16;}let p=a.as_mut_ptr().add(8);println!("address_i16_s {:016x}",(address_i16_s(p,v as i64) as u64).wrapping_sub(p as u64));}
{let mut a:[i16;17]=[0;17];for k in 0..17{a[k]=(0x9e3779b97f4a7c15u64.wrapping_mul(k as u64).wrapping_add(v)) as i16;}let p=a.as_mut_ptr().add(8);println!("load_i16_u {:016x}",load_i16_u(p,i.unsigned_abs() as u64) as u64);}
{let mut a:[i16;17]=[0;17];for k in 0..17{a[k]=(0x9e3779b97f4a7c15u64.wrapping_mul(k as u64).wrapping_add(v)) as i16;}let p=a.as_mut_ptr().add(8);store_i16_u(p,i.unsigned_abs() as u64,v as i16);print!("store_i16_u ");bytes(&a);}
{let mut a:[i16;17]=[0;17];for k in 0..17{a[k]=(0x9e3779b97f4a7c15u64.wrapping_mul(k as u64).wrapping_add(v)) as i16;}let p=a.as_mut_ptr().add(8);println!("address_i16_u {:016x}",(address_i16_u(p,v as u64) as u64).wrapping_sub(p as u64));}
{let mut a:[u16;17]=[0;17];for k in 0..17{a[k]=(0x9e3779b97f4a7c15u64.wrapping_mul(k as u64).wrapping_add(v)) as u16;}let p=a.as_mut_ptr().add(8);println!("load_u16_s {:016x}",load_u16_s(p,i as i64) as u64);}
{let mut a:[u16;17]=[0;17];for k in 0..17{a[k]=(0x9e3779b97f4a7c15u64.wrapping_mul(k as u64).wrapping_add(v)) as u16;}let p=a.as_mut_ptr().add(8);store_u16_s(p,i as i64,v as u16);print!("store_u16_s ");bytes(&a);}
{let mut a:[u16;17]=[0;17];for k in 0..17{a[k]=(0x9e3779b97f4a7c15u64.wrapping_mul(k as u64).wrapping_add(v)) as u16;}let p=a.as_mut_ptr().add(8);println!("address_u16_s {:016x}",(address_u16_s(p,v as i64) as u64).wrapping_sub(p as u64));}
{let mut a:[u16;17]=[0;17];for k in 0..17{a[k]=(0x9e3779b97f4a7c15u64.wrapping_mul(k as u64).wrapping_add(v)) as u16;}let p=a.as_mut_ptr().add(8);println!("load_u16_u {:016x}",load_u16_u(p,i.unsigned_abs() as u64) as u64);}
{let mut a:[u16;17]=[0;17];for k in 0..17{a[k]=(0x9e3779b97f4a7c15u64.wrapping_mul(k as u64).wrapping_add(v)) as u16;}let p=a.as_mut_ptr().add(8);store_u16_u(p,i.unsigned_abs() as u64,v as u16);print!("store_u16_u ");bytes(&a);}
{let mut a:[u16;17]=[0;17];for k in 0..17{a[k]=(0x9e3779b97f4a7c15u64.wrapping_mul(k as u64).wrapping_add(v)) as u16;}let p=a.as_mut_ptr().add(8);println!("address_u16_u {:016x}",(address_u16_u(p,v as u64) as u64).wrapping_sub(p as u64));}
{let mut a:[i32;17]=[0;17];for k in 0..17{a[k]=(0x9e3779b97f4a7c15u64.wrapping_mul(k as u64).wrapping_add(v)) as i32;}let p=a.as_mut_ptr().add(8);println!("load_i32_s {:016x}",load_i32_s(p,i as i64) as u64);}
{let mut a:[i32;17]=[0;17];for k in 0..17{a[k]=(0x9e3779b97f4a7c15u64.wrapping_mul(k as u64).wrapping_add(v)) as i32;}let p=a.as_mut_ptr().add(8);store_i32_s(p,i as i64,v as i32);print!("store_i32_s ");bytes(&a);}
{let mut a:[i32;17]=[0;17];for k in 0..17{a[k]=(0x9e3779b97f4a7c15u64.wrapping_mul(k as u64).wrapping_add(v)) as i32;}let p=a.as_mut_ptr().add(8);println!("address_i32_s {:016x}",(address_i32_s(p,v as i64) as u64).wrapping_sub(p as u64));}
{let mut a:[i32;17]=[0;17];for k in 0..17{a[k]=(0x9e3779b97f4a7c15u64.wrapping_mul(k as u64).wrapping_add(v)) as i32;}let p=a.as_mut_ptr().add(8);println!("load_i32_u {:016x}",load_i32_u(p,i.unsigned_abs() as u64) as u64);}
{let mut a:[i32;17]=[0;17];for k in 0..17{a[k]=(0x9e3779b97f4a7c15u64.wrapping_mul(k as u64).wrapping_add(v)) as i32;}let p=a.as_mut_ptr().add(8);store_i32_u(p,i.unsigned_abs() as u64,v as i32);print!("store_i32_u ");bytes(&a);}
{let mut a:[i32;17]=[0;17];for k in 0..17{a[k]=(0x9e3779b97f4a7c15u64.wrapping_mul(k as u64).wrapping_add(v)) as i32;}let p=a.as_mut_ptr().add(8);println!("address_i32_u {:016x}",(address_i32_u(p,v as u64) as u64).wrapping_sub(p as u64));}
{let mut a:[u32;17]=[0;17];for k in 0..17{a[k]=(0x9e3779b97f4a7c15u64.wrapping_mul(k as u64).wrapping_add(v)) as u32;}let p=a.as_mut_ptr().add(8);println!("load_u32_s {:016x}",load_u32_s(p,i as i64) as u64);}
{let mut a:[u32;17]=[0;17];for k in 0..17{a[k]=(0x9e3779b97f4a7c15u64.wrapping_mul(k as u64).wrapping_add(v)) as u32;}let p=a.as_mut_ptr().add(8);store_u32_s(p,i as i64,v as u32);print!("store_u32_s ");bytes(&a);}
{let mut a:[u32;17]=[0;17];for k in 0..17{a[k]=(0x9e3779b97f4a7c15u64.wrapping_mul(k as u64).wrapping_add(v)) as u32;}let p=a.as_mut_ptr().add(8);println!("address_u32_s {:016x}",(address_u32_s(p,v as i64) as u64).wrapping_sub(p as u64));}
{let mut a:[u32;17]=[0;17];for k in 0..17{a[k]=(0x9e3779b97f4a7c15u64.wrapping_mul(k as u64).wrapping_add(v)) as u32;}let p=a.as_mut_ptr().add(8);println!("load_u32_u {:016x}",load_u32_u(p,i.unsigned_abs() as u64) as u64);}
{let mut a:[u32;17]=[0;17];for k in 0..17{a[k]=(0x9e3779b97f4a7c15u64.wrapping_mul(k as u64).wrapping_add(v)) as u32;}let p=a.as_mut_ptr().add(8);store_u32_u(p,i.unsigned_abs() as u64,v as u32);print!("store_u32_u ");bytes(&a);}
{let mut a:[u32;17]=[0;17];for k in 0..17{a[k]=(0x9e3779b97f4a7c15u64.wrapping_mul(k as u64).wrapping_add(v)) as u32;}let p=a.as_mut_ptr().add(8);println!("address_u32_u {:016x}",(address_u32_u(p,v as u64) as u64).wrapping_sub(p as u64));}
{let mut a:[i64;17]=[0;17];for k in 0..17{a[k]=(0x9e3779b97f4a7c15u64.wrapping_mul(k as u64).wrapping_add(v)) as i64;}let p=a.as_mut_ptr().add(8);println!("load_i64_s {:016x}",load_i64_s(p,i as i64) as u64);}
{let mut a:[i64;17]=[0;17];for k in 0..17{a[k]=(0x9e3779b97f4a7c15u64.wrapping_mul(k as u64).wrapping_add(v)) as i64;}let p=a.as_mut_ptr().add(8);store_i64_s(p,i as i64,v as i64);print!("store_i64_s ");bytes(&a);}
{let mut a:[i64;17]=[0;17];for k in 0..17{a[k]=(0x9e3779b97f4a7c15u64.wrapping_mul(k as u64).wrapping_add(v)) as i64;}let p=a.as_mut_ptr().add(8);println!("address_i64_s {:016x}",(address_i64_s(p,v as i64) as u64).wrapping_sub(p as u64));}
{let mut a:[i64;17]=[0;17];for k in 0..17{a[k]=(0x9e3779b97f4a7c15u64.wrapping_mul(k as u64).wrapping_add(v)) as i64;}let p=a.as_mut_ptr().add(8);println!("load_i64_u {:016x}",load_i64_u(p,i.unsigned_abs() as u64) as u64);}
{let mut a:[i64;17]=[0;17];for k in 0..17{a[k]=(0x9e3779b97f4a7c15u64.wrapping_mul(k as u64).wrapping_add(v)) as i64;}let p=a.as_mut_ptr().add(8);store_i64_u(p,i.unsigned_abs() as u64,v as i64);print!("store_i64_u ");bytes(&a);}
{let mut a:[i64;17]=[0;17];for k in 0..17{a[k]=(0x9e3779b97f4a7c15u64.wrapping_mul(k as u64).wrapping_add(v)) as i64;}let p=a.as_mut_ptr().add(8);println!("address_i64_u {:016x}",(address_i64_u(p,v as u64) as u64).wrapping_sub(p as u64));}
{let mut a:[u64;17]=[0;17];for k in 0..17{a[k]=(0x9e3779b97f4a7c15u64.wrapping_mul(k as u64).wrapping_add(v)) as u64;}let p=a.as_mut_ptr().add(8);println!("load_u64_s {:016x}",load_u64_s(p,i as i64) as u64);}
{let mut a:[u64;17]=[0;17];for k in 0..17{a[k]=(0x9e3779b97f4a7c15u64.wrapping_mul(k as u64).wrapping_add(v)) as u64;}let p=a.as_mut_ptr().add(8);store_u64_s(p,i as i64,v as u64);print!("store_u64_s ");bytes(&a);}
{let mut a:[u64;17]=[0;17];for k in 0..17{a[k]=(0x9e3779b97f4a7c15u64.wrapping_mul(k as u64).wrapping_add(v)) as u64;}let p=a.as_mut_ptr().add(8);println!("address_u64_s {:016x}",(address_u64_s(p,v as i64) as u64).wrapping_sub(p as u64));}
{let mut a:[u64;17]=[0;17];for k in 0..17{a[k]=(0x9e3779b97f4a7c15u64.wrapping_mul(k as u64).wrapping_add(v)) as u64;}let p=a.as_mut_ptr().add(8);println!("load_u64_u {:016x}",load_u64_u(p,i.unsigned_abs() as u64) as u64);}
{let mut a:[u64;17]=[0;17];for k in 0..17{a[k]=(0x9e3779b97f4a7c15u64.wrapping_mul(k as u64).wrapping_add(v)) as u64;}let p=a.as_mut_ptr().add(8);store_u64_u(p,i.unsigned_abs() as u64,v as u64);print!("store_u64_u ");bytes(&a);}
{let mut a:[u64;17]=[0;17];for k in 0..17{a[k]=(0x9e3779b97f4a7c15u64.wrapping_mul(k as u64).wrapping_add(v)) as u64;}let p=a.as_mut_ptr().add(8);println!("address_u64_u {:016x}",(address_u64_u(p,v as u64) as u64).wrapping_sub(p as u64));}
{let mut a=[Record{before:0,value:0,tag:0};17];for k in 0..17{a[k]=Record{before:v.wrapping_add(k as u64) as i64,value:v.wrapping_add(k as u64) as i32,tag:0xdeadbeef+k as u32};}let p=a.as_mut_ptr().add(8);println!("field_load {:016x}",field_load(p,i) as u64);}
{let mut a=[Record{before:0,value:0,tag:0};17];for k in 0..17{a[k]=Record{before:v.wrapping_add(k as u64) as i64,value:v.wrapping_add(k as u64) as i32,tag:0xdeadbeef+k as u32};}let p=a.as_mut_ptr().add(8);field_store(p,i,v as i32);print!("field_store ");bytes(&a);}
{let mut a=[0i64;17];for k in 0..17{a[k]=v.wrapping_add(k as u64) as i64;}let p=a.as_mut_ptr().add(8);TRACE=0;let result=callback_load(p,i) as u64;let trace=TRACE;println!("callback_load {:016x} {}",result,trace);}
{let mut a=[0i64;17];for k in 0..17{a[k]=v.wrapping_add(k as u64) as i64;}let p=a.as_mut_ptr().add(8);TRACE=0;callback_store(p,i,v as i64);let trace=TRACE;print!("callback_store {} ",trace);bytes(&a);}
{let mut a=[0i64;17];for k in 0..17{a[k]=v.wrapping_add(k as u64) as i64;}let p=a.as_mut_ptr().add(8);TRACE=0;let result=(callback_address(p,v as i64) as u64).wrapping_sub(p as u64);let trace=TRACE;println!("callback_address {:016x} {}",result,trace);}
{let mut x=v as i32;println!("zero_load {:016x}",zero_load(&mut x) as u64);}
}}let mut anchor=0i32;for n in 0..64u32{println!("shift_address {:016x}",(shift_address(&mut anchor,n) as u64).wrapping_sub(&mut anchor as *mut i32 as u64));println!("logical_address {:016x}",(logical_address(&mut anchor,n) as u64).wrapping_sub(&mut anchor as *mut i32 as u64));println!("signed_address {:016x}",(signed_address(&mut anchor,n) as u64).wrapping_sub(&mut anchor as *mut i32 as u64));}}}
