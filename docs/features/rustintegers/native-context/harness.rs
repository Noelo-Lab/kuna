#![allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
static mut value_calls:u32=0;
unsafe fn counted_unsigned(value:u64)->u64 {value_calls+=1;value}
/* EMITTED_BODIES */
fn main(){for a in [0u64,1,0x8000000000000000,0xffffffffffffffff]{for n in 0..64u64{for m in [0u64,1,31,32,63]{unsafe{value_calls=0;let accumulated=accumulate_calls(a as i64,a^m,(n&7)as u32)as u64;let calls=value_calls;println!("{:016x} {:016x} {:016x} {:016x} {:016x} {:016x} {:016x} {:016x} {:016x} {:016x} {:016x} {:016x} {:016x} {:016x} {:016x} {}",insert_bit(a,n),isolated_bit(n),invert_low(n),shift_again(n,m),literal_right(n),signed_right(n)as u64,named_shift(a,n),signed_shift(a as i64,n)as u64,mixed_shift(a,n),accumulate_bits(n),accumulated,mixed_less(a,m as i64,n),mixed_left(a,m as i64,n),mixed_sub_less(a,m as i64,n),mixed_sub_left(a,m as i64,n),calls);}}}}}
