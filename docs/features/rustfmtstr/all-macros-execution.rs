fn main(){std::panic::set_hook(Box::new(|_|{}));
{let text="é🦀\0{}\n";
let v1=text.as_ptr();
let v3=text.len() as u64;
unsafe{println ! ("|{0:·<8}|{0:🦀^10.3}|{0:>7.3}|" , std :: str :: from_utf8_unchecked (std :: slice :: from_raw_parts (v1 as * const u8 , v3 as usize)));}
}
{let text="é🦀\0{}\n";
let v3_value:u32=u32::MAX;let v3=&v3_value as *const u32;
let v2=text.as_ptr();
let v4=text.len() as u64;
let mut destination=std::mem::MaybeUninit::<String>::uninit();let a0=destination.as_mut_ptr();
unsafe{std :: ptr :: write ((a0 as * mut _) as * mut std :: string :: String , format ! ("{}-{}" ,&* ((v3 as * const _) as * const u32) , std :: str :: from_utf8_unchecked (std :: slice :: from_raw_parts (v2 as * const u8 , v4 as usize))));println!("{:?}",destination.assume_init());}
}
{let text="é🦀\0{}\n";
let v1=text.as_ptr();
let v3=text.len() as u64;
unsafe{println ! ("{{é🙂}} {}\n" , std :: str :: from_utf8_unchecked (std :: slice :: from_raw_parts (v1 as * const u8 , v3 as usize)));}
}
{let text="é🦀\0{}\n";
let v1=text.as_ptr();
let v4=text.len() as u64;
let v2=text.as_ptr();
let v5=text.len() as u64;
unsafe{println ! ("{} / {}" , std :: str :: from_utf8_unchecked (std :: slice :: from_raw_parts (v1 as * const u8 , v4 as usize)) , std :: str :: from_utf8_unchecked (std :: slice :: from_raw_parts (v2 as * const u8 , v5 as usize)));}
}
{let text="é🦀\0{}\n";
let v1=text.as_ptr();
let v3=text.len() as u64;
let error=std::panic::catch_unwind(||unsafe{panic ! ("bad {}" , std :: str :: from_utf8_unchecked (std :: slice :: from_raw_parts (v1 as * const u8 , v3 as usize)))}).unwrap_err();println!("{:?}",error.downcast_ref::<String>().unwrap());
}
{let text="é🦀\0{}\n";
let v1=text.as_ptr();
let v3=text.len() as u64;
unsafe{print ! ("{}" , std :: str :: from_utf8_unchecked (std :: slice :: from_raw_parts (v1 as * const u8 , v3 as usize)));}
}
{let text="é🦀\0{}\n";
let v1=text.as_ptr();
let v3=text.len() as u64;
let mut destination=std::mem::MaybeUninit::<String>::uninit();let a0=destination.as_mut_ptr();
unsafe{std :: ptr :: write ((a0 as * mut _) as * mut std :: string :: String , format ! ("[{}]" , std :: str :: from_utf8_unchecked (std :: slice :: from_raw_parts (v1 as * const u8 , v3 as usize))));println!("{:?}",destination.assume_init());}
}
{let text="é🦀\0{}\n";
let v1=text.as_ptr();
let v3=text.len() as u64;
unsafe{eprintln ! ("stderr {}" , std :: str :: from_utf8_unchecked (std :: slice :: from_raw_parts (v1 as * const u8 , v3 as usize)));}
}
{let text="é🦀\0{}\n";
let v3_value:u32=u32::MAX;let v3=&v3_value as *const u32;
let v2=text.as_ptr();
let v4=text.len() as u64;
unsafe{eprintln ! ("{}: {}" ,&* ((v3 as * const _) as * const u32) , std :: str :: from_utf8_unchecked (std :: slice :: from_raw_parts (v2 as * const u8 , v4 as usize)));}
}
{let text="é🦀\0{}\n";
let v1=text.as_ptr();
let v4=text.len() as u64;
let error=std::panic::catch_unwind(||unsafe{panic ! ("{}" , std :: str :: from_utf8_unchecked (std :: slice :: from_raw_parts (v1 as * const u8 , v4 as usize)))}).unwrap_err();println!("{:?}",error.downcast_ref::<String>().unwrap());
}
{let text="é🦀\0{}\n";
let v3_value:u32=u32::MAX;let v3=&v3_value as *const u32;
let v2=text.as_ptr();
let v4=text.len() as u64;
let mut destination=std::mem::MaybeUninit::<String>::uninit();let a0=destination.as_mut_ptr();
unsafe{std :: ptr :: write ((a0 as * mut _) as * mut std :: string :: String , format ! ("{}-{}" ,&* ((v3 as * const _) as * const u32) , std :: str :: from_utf8_unchecked (std :: slice :: from_raw_parts (v2 as * const u8 , v4 as usize))));println!("{:?}",destination.assume_init());}
}
{let text="é🦀\0{}\n";
let v1=text.as_ptr();
let v4=text.len() as u64;
let error=std::panic::catch_unwind(||unsafe{panic ! ("{}" , std :: str :: from_utf8_unchecked (std :: slice :: from_raw_parts (v1 as * const u8 , v4 as usize)))}).unwrap_err();println!("{:?}",error.downcast_ref::<String>().unwrap());
}
}
