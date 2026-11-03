#![allow(dead_code,unused_variables,unused_parens,unused_mut)]
#[repr(C)]struct ArrayFields{before:u64,i8:[i8;9],u8:[u8;9],i16:[i16;9],u16:[u16;9],i32:[i32;9],u32:[u32;9],i64:[i64;9],u64:[u64;9],after:u64}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn array_address_s(mut p: *mut ArrayFields, mut i: i64) -> *mut i32
{
  return &mut (*p).i32[i as usize];
}
fn main(){unsafe{let i=std::env::args().nth(1).unwrap().parse::<i64>().unwrap();let mut value:ArrayFields=std::mem::zeroed();let p=&mut value as *mut ArrayFields;println!("{}",(array_address_s(p,i)as usize).wrapping_sub(p as usize));}}
