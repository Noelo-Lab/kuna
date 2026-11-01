fn main(){std::panic::set_hook(Box::new(|_|{}));
let payload=std::panic::catch_unwind(|| {
panic!("capacity overflow");
}).unwrap_err();
let message=payload.downcast_ref::<&str>().copied().unwrap_or_else(||payload.downcast_ref::<String>().unwrap().as_str());
assert_eq!(message,"capacity overflow");
println!("0 {message:?}");
let payload=std::panic::catch_unwind(|| {
let value0:u64=u64::MAX;
let value1:u64=u64::MAX;
panic!("copy_from_slice: source slice length ({}) does not match destination slice length ({})",&value0,&value1);
}).unwrap_err();
let message=payload.downcast_ref::<&str>().copied().unwrap_or_else(||payload.downcast_ref::<String>().unwrap().as_str());
println!("1 {message:?}");
let payload=std::panic::catch_unwind(|| {
panic!("user-provided comparison function does not correctly implement a total order");
}).unwrap_err();
let message=payload.downcast_ref::<&str>().copied().unwrap_or_else(||payload.downcast_ref::<String>().unwrap().as_str());
assert_eq!(message,"user-provided comparison function does not correctly implement a total order");
println!("2 {message:?}");
let payload=std::panic::catch_unwind(|| {
panic!("cannot panic during the backtrace function");
}).unwrap_err();
let message=payload.downcast_ref::<&str>().copied().unwrap_or_else(||payload.downcast_ref::<String>().unwrap().as_str());
assert_eq!(message,"cannot panic during the backtrace function");
println!("3 {message:?}");
let payload=std::panic::catch_unwind(|| {
let value0:u64=u64::MAX;
panic!("byte index {} is not an OsStr boundary",&value0);
}).unwrap_err();
let message=payload.downcast_ref::<&str>().copied().unwrap_or_else(||payload.downcast_ref::<String>().unwrap().as_str());
println!("4 {message:?}");
let payload=std::panic::catch_unwind(|| {
panic!("{{x}}\n");
}).unwrap_err();
let message=payload.downcast_ref::<&str>().copied().unwrap_or_else(||payload.downcast_ref::<String>().unwrap().as_str());
assert_eq!(message,"{x}\n");
println!("5 {message:?}");
let payload=std::panic::catch_unwind(|| {
panic!("{{x}}\n");
}).unwrap_err();
let message=payload.downcast_ref::<&str>().copied().unwrap_or_else(||payload.downcast_ref::<String>().unwrap().as_str());
assert_eq!(message,"{x}\n");
println!("6 {message:?}");
let payload=std::panic::catch_unwind(|| {
panic!("{{x}}\n");
}).unwrap_err();
let message=payload.downcast_ref::<&str>().copied().unwrap_or_else(||payload.downcast_ref::<String>().unwrap().as_str());
assert_eq!(message,"{x}\n");
println!("7 {message:?}");
let payload=std::panic::catch_unwind(|| {
let value0:u64=u64::MAX;
panic!("value={}\n",&value0);
}).unwrap_err();
let message=payload.downcast_ref::<&str>().copied().unwrap_or_else(||payload.downcast_ref::<String>().unwrap().as_str());
println!("8 {message:?}");
let payload=std::panic::catch_unwind(|| {
panic!("Formatting argument out of range");
}).unwrap_err();
let message=payload.downcast_ref::<&str>().copied().unwrap_or_else(||payload.downcast_ref::<String>().unwrap().as_str());
assert_eq!(message,"Formatting argument out of range");
println!("9 {message:?}");
let payload=std::panic::catch_unwind(|| {
panic!("");
}).unwrap_err();
let message=payload.downcast_ref::<&str>().copied().unwrap_or_else(||payload.downcast_ref::<String>().unwrap().as_str());
assert_eq!(message,"");
println!("10 {message:?}");
let payload=std::panic::catch_unwind(|| {
let value0:u8=u8::MAX;
panic!("{{|{0:\"<6}|{0:\\>6}|{0:\n^7}|{0:\0<3}|}}\n",&value0);
}).unwrap_err();
let message=payload.downcast_ref::<&str>().copied().unwrap_or_else(||payload.downcast_ref::<String>().unwrap().as_str());
println!("11 {message:?}");
let payload=std::panic::catch_unwind(|| {
let value0:u32=u32::MAX;
panic!("hex={:#010x}",&value0);
}).unwrap_err();
let message=payload.downcast_ref::<&str>().copied().unwrap_or_else(||payload.downcast_ref::<String>().unwrap().as_str());
println!("12 {message:?}");
let payload=std::panic::catch_unwind(|| {
panic!("literal {{é🙂}}\n\n");
}).unwrap_err();
let message=payload.downcast_ref::<&str>().copied().unwrap_or_else(||payload.downcast_ref::<String>().unwrap().as_str());
assert_eq!(message,"literal {é🙂}\n\n");
println!("13 {message:?}");
let payload=std::panic::catch_unwind(|| {
let value0:i8=i8::MIN + 1;
let value1:i8=i8::MIN + 1;
let value2:i8=i8::MIN + 1;
panic!("{:+} {:#x} {:08b}",&value0,&value1,&value2);
}).unwrap_err();
let message=payload.downcast_ref::<&str>().copied().unwrap_or_else(||payload.downcast_ref::<String>().unwrap().as_str());
println!("14 {message:?}");
let payload=std::panic::catch_unwind(|| {
let value0:u16=u16::MAX;
let value1:u32=u32::MAX;
let value2:u16=u16::MAX;
panic!("{0:#08x} {1:+06} {2:>5o} {1:+06}",&value0,&value1,&value2);
}).unwrap_err();
let message=payload.downcast_ref::<&str>().copied().unwrap_or_else(||payload.downcast_ref::<String>().unwrap().as_str());
println!("15 {message:?}");
let payload=std::panic::catch_unwind(|| {
let value0:i64=i64::MIN + 1;
panic!("signed={:+}",&value0);
}).unwrap_err();
let message=payload.downcast_ref::<&str>().copied().unwrap_or_else(||payload.downcast_ref::<String>().unwrap().as_str());
println!("16 {message:?}");
let payload=std::panic::catch_unwind(|| {
let value0:u64=u64::MAX;
panic!("memory allocation of {} bytes failed",&value0);
}).unwrap_err();
let message=payload.downcast_ref::<&str>().copied().unwrap_or_else(||payload.downcast_ref::<String>().unwrap().as_str());
println!("17 {message:?}");
let payload=std::panic::catch_unwind(|| {
panic!("cannot panic during the backtrace function");
}).unwrap_err();
let message=payload.downcast_ref::<&str>().copied().unwrap_or_else(||payload.downcast_ref::<String>().unwrap().as_str());
assert_eq!(message,"cannot panic during the backtrace function");
println!("18 {message:?}");
let payload=std::panic::catch_unwind(|| {
panic!("capacity overflow");
}).unwrap_err();
let message=payload.downcast_ref::<&str>().copied().unwrap_or_else(||payload.downcast_ref::<String>().unwrap().as_str());
assert_eq!(message,"capacity overflow");
println!("19 {message:?}");
let payload=std::panic::catch_unwind(|| {
panic!("user-provided comparison function does not correctly implement a total order");
}).unwrap_err();
let message=payload.downcast_ref::<&str>().copied().unwrap_or_else(||payload.downcast_ref::<String>().unwrap().as_str());
assert_eq!(message,"user-provided comparison function does not correctly implement a total order");
println!("20 {message:?}");
let payload=std::panic::catch_unwind(|| {
panic!("a formatting trait implementation returned an error when the underlying stream did not");
}).unwrap_err();
let message=payload.downcast_ref::<&str>().copied().unwrap_or_else(||payload.downcast_ref::<String>().unwrap().as_str());
assert_eq!(message,"a formatting trait implementation returned an error when the underlying stream did not");
println!("21 {message:?}");
let payload=std::panic::catch_unwind(|| {
panic!("a formatting trait implementation returned an error when the underlying stream did not");
}).unwrap_err();
let message=payload.downcast_ref::<&str>().copied().unwrap_or_else(||payload.downcast_ref::<String>().unwrap().as_str());
assert_eq!(message,"a formatting trait implementation returned an error when the underlying stream did not");
println!("22 {message:?}");
let payload=std::panic::catch_unwind(|| {
panic!("a formatting trait implementation returned an error when the underlying stream did not");
}).unwrap_err();
let message=payload.downcast_ref::<&str>().copied().unwrap_or_else(||payload.downcast_ref::<String>().unwrap().as_str());
assert_eq!(message,"a formatting trait implementation returned an error when the underlying stream did not");
println!("23 {message:?}");
let payload=std::panic::catch_unwind(|| {
let value0:u64=u64::MAX;
panic!("memory allocation of {} bytes failed",&value0);
}).unwrap_err();
let message=payload.downcast_ref::<&str>().copied().unwrap_or_else(||payload.downcast_ref::<String>().unwrap().as_str());
println!("24 {message:?}");
let payload=std::panic::catch_unwind(|| {
panic!("cannot panic during the backtrace function");
}).unwrap_err();
let message=payload.downcast_ref::<&str>().copied().unwrap_or_else(||payload.downcast_ref::<String>().unwrap().as_str());
assert_eq!(message,"cannot panic during the backtrace function");
println!("25 {message:?}");
let payload=std::panic::catch_unwind(|| {
panic!("a formatting trait implementation returned an error when the underlying stream did not");
}).unwrap_err();
let message=payload.downcast_ref::<&str>().copied().unwrap_or_else(||payload.downcast_ref::<String>().unwrap().as_str());
assert_eq!(message,"a formatting trait implementation returned an error when the underlying stream did not");
println!("26 {message:?}");
let payload=std::panic::catch_unwind(|| {
panic!("capacity overflow");
}).unwrap_err();
let message=payload.downcast_ref::<&str>().copied().unwrap_or_else(||payload.downcast_ref::<String>().unwrap().as_str());
assert_eq!(message,"capacity overflow");
println!("27 {message:?}");
let payload=std::panic::catch_unwind(|| {
panic!("user-provided comparison function does not correctly implement a total order");
}).unwrap_err();
let message=payload.downcast_ref::<&str>().copied().unwrap_or_else(||payload.downcast_ref::<String>().unwrap().as_str());
assert_eq!(message,"user-provided comparison function does not correctly implement a total order");
println!("28 {message:?}");
}
