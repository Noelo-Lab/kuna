fn str_plain(text:&str){unsafe{let v1=text.as_ptr() as u64;let v3=text.len() as u64;
  print!("{}",std::str::from_utf8_unchecked(std::slice::from_raw_parts(v1 as *const u8,v3 as usize)));

}}
fn str_line(text:&str){unsafe{let v1=text.as_ptr() as u64;let v3=text.len() as u64;
  println!("{{é🙂}} {}\n",std::str::from_utf8_unchecked(std::slice::from_raw_parts(v1 as *const u8,v3 as usize)));

}}
fn str_stderr(text:&str){unsafe{let v1=text.as_ptr() as u64;let v3=text.len() as u64;
  eprintln!("stderr {}",std::str::from_utf8_unchecked(std::slice::from_raw_parts(v1 as *const u8,v3 as usize)));

}}
fn str_stderr_pair(value:u32,text:&str){unsafe{let v3=&value as *const u32;let v2=text.as_ptr() as u64;let v4=text.len() as u64;
  eprintln!("{}: {}",&*((v3 as *const _) as *const u32),std::str::from_utf8_unchecked(std::slice::from_raw_parts(v2 as *const u8,v4 as usize)));

}}
fn str_flags(text:&str){unsafe{let v1=text.as_ptr() as u64;let v3=text.len() as u64;
  println!("|{0:·<8}|{0:🦀^10.3}|{0:>7.3}|",std::str::from_utf8_unchecked(std::slice::from_raw_parts(v1 as *const u8,v3 as usize)));

}}
fn str_pair(left:&str,right:&str){unsafe{let v1=left.as_ptr() as u64;let v4=left.len() as u64;let v2=right.as_ptr() as u64;let v5=right.len() as u64;
  println!("{} / {}",std::str::from_utf8_unchecked(std::slice::from_raw_parts(v1 as *const u8,v4 as usize)),std::str::from_utf8_unchecked(std::slice::from_raw_parts(v2 as *const u8,v5 as usize)));

}}
fn str_integer(value:u32,text:&str) -> String{unsafe{let v3=&value as *const u32;let v2=text.as_ptr() as u64;let v4=text.len() as u64;let mut destination=std::mem::MaybeUninit::<String>::uninit();let a0=destination.as_mut_ptr() as u64;
  std::ptr::write(a0 as *mut std::string::String,format!("{}-{}",&*((v3 as *const _) as *const u32),std::str::from_utf8_unchecked(std::slice::from_raw_parts(v2 as *const u8,v4 as usize))));
destination.assume_init()
}}
fn str_result(text:&str) -> String{unsafe{let v1=text.as_ptr() as u64;let v3=text.len() as u64;let mut destination=std::mem::MaybeUninit::<String>::uninit();let a0=destination.as_mut_ptr() as u64;
  std::ptr::write(a0 as *mut std::string::String,format!("[{}]",std::str::from_utf8_unchecked(std::slice::from_raw_parts(v1 as *const u8,v3 as usize))));
destination.assume_init()
}}
fn str_panic(text:&str){unsafe{let v1=text.as_ptr() as u64;let v3=text.len() as u64;
  panic!("bad {}",std::str::from_utf8_unchecked(std::slice::from_raw_parts(v1 as *const u8,v3 as usize))); // no-return

}}
fn main() {
    std::panic::set_hook(Box::new(|_|{}));
    let mut strings:Vec<String>=(0u8..=127).map(|c|String::from(c as char)).collect();
    strings.extend(["", "text", "é🙂🦀", "a\0b\n{}\\\"", "\r\t", "e\u{301}", "👩\u{200d}💻", "مرحبا", "日本語"].into_iter().map(String::from));
    for c in [0x80,0x7ff,0x800,0xd7ff,0xe000,0xffff,0x10000,0x10ffff] {strings.push(char::from_u32(c).unwrap().to_string());}
    for size in [2,3,7,8,9,16,31,32,255,256,4096] {strings.push("é{}\0🦀".repeat(size));}
    let values=[0u32,1,7,127,255,256,0x7fffffff,0x80000000,u32::MAX];
    if std::env::args().nth(1).as_deref()==Some("panic") {
        for text in &strings {
            let error=std::panic::catch_unwind(||str_panic(text)).unwrap_err();
            println!("{:?}",error.downcast_ref::<String>().unwrap());
        }
        return;
    }
    for text in &strings {
        str_plain(text);str_line(text);str_stderr(text);str_flags(text);str_pair(text,"right é🦀\0\n");
        println!("{:?}",str_result(text));
        for &value in &values {str_stderr_pair(value,text);println!("{:?}",str_integer(value,text));}
    }
}
