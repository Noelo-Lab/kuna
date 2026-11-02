use std::fmt;

fn main() {
    assert_eq!(std::mem::size_of::<fmt::Result>(), 1);
    assert_eq!(std::mem::align_of::<fmt::Result>(), 1);
    let ok: u8 = unsafe { std::mem::transmute::<fmt::Result, u8>(Ok(())) };
    let error: u8 = unsafe { std::mem::transmute::<fmt::Result, u8>(Err(fmt::Error)) };
    assert_eq!((ok, error), (0, 1));
    assert_eq!(std::mem::size_of::<String>(), 24);
    assert_eq!(std::mem::align_of::<String>(), 8);
    let mut destination = String::from("prefix:");
    let writer: &mut dyn fmt::Write = &mut destination;
    let pair: [usize; 2] = unsafe { std::mem::transmute(writer) };
    let table = unsafe { std::slice::from_raw_parts(pair[1] as *const usize, 6) };
    assert_eq!((table[1], table[2]), (24, 8));
    assert!([0, 3, 4, 5].iter().all(|&index| table[index] != 0));
    let result = fmt::write(&mut destination, format_args!("{} {}", u32::MAX, "é🙂\0"));
    let result_byte: u8 = unsafe { std::mem::transmute(result) };
    assert_eq!(result_byte, 0);
    assert_eq!(destination, "prefix:4294967295 é🙂\0");
    println!("fmt::Result Ok=0 Err=1; String size=24 align=8; native writer mutated the original buffer");
}
