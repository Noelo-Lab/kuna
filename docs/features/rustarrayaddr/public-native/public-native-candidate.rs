#[repr(C)]struct Fields{values:[i32;9]}
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn array_address(mut p: *mut Fields, mut i: i64) -> *mut i32
{
  return (&raw mut (*p).values).wrapping_byte_offset((i as isize).wrapping_mul(4)) as *mut i32;
}

fn main(){let mut f=Fields{values:[0;9]};let p=&raw mut f;let result=unsafe{array_address(p,9)};println!("{}",(result as usize).wrapping_sub(p as usize));}
