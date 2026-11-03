#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn array_decay(mut p: *mut [i32; 9]) -> *mut i32
{
  return *p;
}

fn main(){unsafe{let mut a=[0i32;9];let p=&raw mut a;println!("{}",(array_decay(p)as usize).wrapping_sub(p as usize));}}
