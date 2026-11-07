#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn zero_array(mut p: *mut [u8; 16])
{
  *p = [0; 16];
}

fn main(){unsafe{let mut value=[0xa5u8;16];zero_array(&raw mut value);assert_eq!(value,[0;16]);for byte in value{print!("{:02x}",byte);}println!();}}
