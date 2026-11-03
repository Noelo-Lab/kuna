#![allow(dead_code,unused_variables,unused_parens,unused_mut,non_snake_case)]
#[repr(C)]struct DecayFields{before:u64,i8:[i8;9],u8:[u8;9],i16:[i16;9],u16:[u16;9],i32:[i32;9],u32:[u32;9],i64:[i64;9],u64:[u64;9],float32:[f32;9],float64:[f64;9],pointers:[*mut u8;9],flags:[bool;9],after:u64}
static mut TRACE:u64=0;
unsafe fn counted_base(p:*mut DecayFields)->*mut DecayFields{TRACE=TRACE*100+1;p}
unsafe fn pass_value_i8(p:*mut i8)->*mut i8{TRACE=TRACE*100+1;p}
unsafe fn pass_value_u8(p:*mut u8)->*mut u8{TRACE=TRACE*100+2;p}
unsafe fn pass_value_i16(p:*mut i16)->*mut i16{TRACE=TRACE*100+3;p}
unsafe fn pass_value_u16(p:*mut u16)->*mut u16{TRACE=TRACE*100+4;p}
unsafe fn pass_value_i32(p:*mut i32)->*mut i32{TRACE=TRACE*100+5;p}
unsafe fn pass_value_u32(p:*mut u32)->*mut u32{TRACE=TRACE*100+6;p}
unsafe fn pass_value_i64(p:*mut i64)->*mut i64{TRACE=TRACE*100+7;p}
unsafe fn pass_value_u64(p:*mut u64)->*mut u64{TRACE=TRACE*100+8;p}
unsafe fn pass_value_float32(p:*mut f32)->*mut f32{TRACE=TRACE*100+9;p}
unsafe fn pass_value_float64(p:*mut f64)->*mut f64{TRACE=TRACE*100+10;p}
unsafe fn pass_value_pointers(p:*mut *mut u8)->*mut *mut u8{TRACE=TRACE*100+11;p}
unsafe fn pass_value_flags(p:*mut bool)->*mut bool{TRACE=TRACE*100+12;p}
#[repr(C)]struct Slot<T>{before:u64,value:*mut T,after:u64}
extern "C"{fn mmap(p:*mut std::ffi::c_void,n:usize,prot:i32,flags:i32,fd:i32,off:isize)->*mut std::ffi::c_void;fn mprotect(p:*mut std::ffi::c_void,n:usize,prot:i32)->i32;fn munmap(p:*mut std::ffi::c_void,n:usize)->i32;}
fn report(name:&str,result:*mut(),base:*mut u8,count:u64,before:u64,after:u64){println!("{} {:016x} {} {:016x} {:016x}",name,(result as usize).wrapping_sub(base as usize)as u64,count,before,after);}
/* EMITTED_BODIES */
fn main(){unsafe{for mode in 0..2{for position in 0..2{let memory=mmap(std::ptr::null_mut(),4096,3,0x22,-1,0);assert_ne!(memory as isize,-1);std::ptr::write_bytes(memory as *mut u8,0xa5,4096);let p=memory.cast::<u8>().add(128*position)as*mut DecayFields;let array=memory.cast::<u8>().add(2048+128*position);if mode!=0{assert_eq!(mprotect(memory,4096,0),0);}println!("mode {} base {}",mode,position);
{TRACE=0;let result=field_i8(p) as *mut();let trace=TRACE;report("field_i8",result,p as *mut u8,trace,0,0);}
{TRACE=0;let result=whole_i8(array as *mut [i8;9]) as *mut();let trace=TRACE;report("whole_i8",result,array,trace,0,0);}
{TRACE=0;let result=end_i8(array as *mut [i8;9]) as *mut();let trace=TRACE;report("end_i8",result,array,trace,0,0);}
{TRACE=0;let result=erased_i8(p) as *mut();let trace=TRACE;report("erased_i8",result,p as *mut u8,trace,0,0);}
{let mut slot=Slot::<i8>{before:0x123456789abcdef0,value:std::ptr::null_mut(),after:0xfedcba9876543210};stored_i8(&raw mut slot.value,p);report("stored_i8",slot.value as *mut(),p as *mut u8,0,slot.before,slot.after);}
{TRACE=0;let result=passed_i8(p) as *mut();let trace=TRACE;report("passed_i8",result,p as *mut u8,trace,0,0);}
{TRACE=0;let result=field_u8(p) as *mut();let trace=TRACE;report("field_u8",result,p as *mut u8,trace,0,0);}
{TRACE=0;let result=whole_u8(array as *mut [u8;9]) as *mut();let trace=TRACE;report("whole_u8",result,array,trace,0,0);}
{TRACE=0;let result=end_u8(array as *mut [u8;9]) as *mut();let trace=TRACE;report("end_u8",result,array,trace,0,0);}
{TRACE=0;let result=erased_u8(p) as *mut();let trace=TRACE;report("erased_u8",result,p as *mut u8,trace,0,0);}
{let mut slot=Slot::<u8>{before:0x123456789abcdef0,value:std::ptr::null_mut(),after:0xfedcba9876543210};stored_u8(&raw mut slot.value,p);report("stored_u8",slot.value as *mut(),p as *mut u8,0,slot.before,slot.after);}
{TRACE=0;let result=passed_u8(p) as *mut();let trace=TRACE;report("passed_u8",result,p as *mut u8,trace,0,0);}
{TRACE=0;let result=field_i16(p) as *mut();let trace=TRACE;report("field_i16",result,p as *mut u8,trace,0,0);}
{TRACE=0;let result=whole_i16(array as *mut [i16;9]) as *mut();let trace=TRACE;report("whole_i16",result,array,trace,0,0);}
{TRACE=0;let result=end_i16(array as *mut [i16;9]) as *mut();let trace=TRACE;report("end_i16",result,array,trace,0,0);}
{TRACE=0;let result=erased_i16(p) as *mut();let trace=TRACE;report("erased_i16",result,p as *mut u8,trace,0,0);}
{let mut slot=Slot::<i16>{before:0x123456789abcdef0,value:std::ptr::null_mut(),after:0xfedcba9876543210};stored_i16(&raw mut slot.value,p);report("stored_i16",slot.value as *mut(),p as *mut u8,0,slot.before,slot.after);}
{TRACE=0;let result=passed_i16(p) as *mut();let trace=TRACE;report("passed_i16",result,p as *mut u8,trace,0,0);}
{TRACE=0;let result=field_u16(p) as *mut();let trace=TRACE;report("field_u16",result,p as *mut u8,trace,0,0);}
{TRACE=0;let result=whole_u16(array as *mut [u16;9]) as *mut();let trace=TRACE;report("whole_u16",result,array,trace,0,0);}
{TRACE=0;let result=end_u16(array as *mut [u16;9]) as *mut();let trace=TRACE;report("end_u16",result,array,trace,0,0);}
{TRACE=0;let result=erased_u16(p) as *mut();let trace=TRACE;report("erased_u16",result,p as *mut u8,trace,0,0);}
{let mut slot=Slot::<u16>{before:0x123456789abcdef0,value:std::ptr::null_mut(),after:0xfedcba9876543210};stored_u16(&raw mut slot.value,p);report("stored_u16",slot.value as *mut(),p as *mut u8,0,slot.before,slot.after);}
{TRACE=0;let result=passed_u16(p) as *mut();let trace=TRACE;report("passed_u16",result,p as *mut u8,trace,0,0);}
{TRACE=0;let result=field_i32(p) as *mut();let trace=TRACE;report("field_i32",result,p as *mut u8,trace,0,0);}
{TRACE=0;let result=whole_i32(array as *mut [i32;9]) as *mut();let trace=TRACE;report("whole_i32",result,array,trace,0,0);}
{TRACE=0;let result=end_i32(array as *mut [i32;9]) as *mut();let trace=TRACE;report("end_i32",result,array,trace,0,0);}
{TRACE=0;let result=erased_i32(p) as *mut();let trace=TRACE;report("erased_i32",result,p as *mut u8,trace,0,0);}
{let mut slot=Slot::<i32>{before:0x123456789abcdef0,value:std::ptr::null_mut(),after:0xfedcba9876543210};stored_i32(&raw mut slot.value,p);report("stored_i32",slot.value as *mut(),p as *mut u8,0,slot.before,slot.after);}
{TRACE=0;let result=passed_i32(p) as *mut();let trace=TRACE;report("passed_i32",result,p as *mut u8,trace,0,0);}
{TRACE=0;let result=field_u32(p) as *mut();let trace=TRACE;report("field_u32",result,p as *mut u8,trace,0,0);}
{TRACE=0;let result=whole_u32(array as *mut [u32;9]) as *mut();let trace=TRACE;report("whole_u32",result,array,trace,0,0);}
{TRACE=0;let result=end_u32(array as *mut [u32;9]) as *mut();let trace=TRACE;report("end_u32",result,array,trace,0,0);}
{TRACE=0;let result=erased_u32(p) as *mut();let trace=TRACE;report("erased_u32",result,p as *mut u8,trace,0,0);}
{let mut slot=Slot::<u32>{before:0x123456789abcdef0,value:std::ptr::null_mut(),after:0xfedcba9876543210};stored_u32(&raw mut slot.value,p);report("stored_u32",slot.value as *mut(),p as *mut u8,0,slot.before,slot.after);}
{TRACE=0;let result=passed_u32(p) as *mut();let trace=TRACE;report("passed_u32",result,p as *mut u8,trace,0,0);}
{TRACE=0;let result=field_i64(p) as *mut();let trace=TRACE;report("field_i64",result,p as *mut u8,trace,0,0);}
{TRACE=0;let result=whole_i64(array as *mut [i64;9]) as *mut();let trace=TRACE;report("whole_i64",result,array,trace,0,0);}
{TRACE=0;let result=end_i64(array as *mut [i64;9]) as *mut();let trace=TRACE;report("end_i64",result,array,trace,0,0);}
{TRACE=0;let result=erased_i64(p) as *mut();let trace=TRACE;report("erased_i64",result,p as *mut u8,trace,0,0);}
{let mut slot=Slot::<i64>{before:0x123456789abcdef0,value:std::ptr::null_mut(),after:0xfedcba9876543210};stored_i64(&raw mut slot.value,p);report("stored_i64",slot.value as *mut(),p as *mut u8,0,slot.before,slot.after);}
{TRACE=0;let result=passed_i64(p) as *mut();let trace=TRACE;report("passed_i64",result,p as *mut u8,trace,0,0);}
{TRACE=0;let result=field_u64(p) as *mut();let trace=TRACE;report("field_u64",result,p as *mut u8,trace,0,0);}
{TRACE=0;let result=whole_u64(array as *mut [u64;9]) as *mut();let trace=TRACE;report("whole_u64",result,array,trace,0,0);}
{TRACE=0;let result=end_u64(array as *mut [u64;9]) as *mut();let trace=TRACE;report("end_u64",result,array,trace,0,0);}
{TRACE=0;let result=erased_u64(p) as *mut();let trace=TRACE;report("erased_u64",result,p as *mut u8,trace,0,0);}
{let mut slot=Slot::<u64>{before:0x123456789abcdef0,value:std::ptr::null_mut(),after:0xfedcba9876543210};stored_u64(&raw mut slot.value,p);report("stored_u64",slot.value as *mut(),p as *mut u8,0,slot.before,slot.after);}
{TRACE=0;let result=passed_u64(p) as *mut();let trace=TRACE;report("passed_u64",result,p as *mut u8,trace,0,0);}
{TRACE=0;let result=field_float32(p) as *mut();let trace=TRACE;report("field_float32",result,p as *mut u8,trace,0,0);}
{TRACE=0;let result=whole_float32(array as *mut [f32;9]) as *mut();let trace=TRACE;report("whole_float32",result,array,trace,0,0);}
{TRACE=0;let result=end_float32(array as *mut [f32;9]) as *mut();let trace=TRACE;report("end_float32",result,array,trace,0,0);}
{TRACE=0;let result=erased_float32(p) as *mut();let trace=TRACE;report("erased_float32",result,p as *mut u8,trace,0,0);}
{let mut slot=Slot::<f32>{before:0x123456789abcdef0,value:std::ptr::null_mut(),after:0xfedcba9876543210};stored_float32(&raw mut slot.value,p);report("stored_float32",slot.value as *mut(),p as *mut u8,0,slot.before,slot.after);}
{TRACE=0;let result=passed_float32(p) as *mut();let trace=TRACE;report("passed_float32",result,p as *mut u8,trace,0,0);}
{TRACE=0;let result=field_float64(p) as *mut();let trace=TRACE;report("field_float64",result,p as *mut u8,trace,0,0);}
{TRACE=0;let result=whole_float64(array as *mut [f64;9]) as *mut();let trace=TRACE;report("whole_float64",result,array,trace,0,0);}
{TRACE=0;let result=end_float64(array as *mut [f64;9]) as *mut();let trace=TRACE;report("end_float64",result,array,trace,0,0);}
{TRACE=0;let result=erased_float64(p) as *mut();let trace=TRACE;report("erased_float64",result,p as *mut u8,trace,0,0);}
{let mut slot=Slot::<f64>{before:0x123456789abcdef0,value:std::ptr::null_mut(),after:0xfedcba9876543210};stored_float64(&raw mut slot.value,p);report("stored_float64",slot.value as *mut(),p as *mut u8,0,slot.before,slot.after);}
{TRACE=0;let result=passed_float64(p) as *mut();let trace=TRACE;report("passed_float64",result,p as *mut u8,trace,0,0);}
{TRACE=0;let result=field_pointers(p) as *mut();let trace=TRACE;report("field_pointers",result,p as *mut u8,trace,0,0);}
{TRACE=0;let result=whole_pointers(array as *mut [*mut u8;9]) as *mut();let trace=TRACE;report("whole_pointers",result,array,trace,0,0);}
{TRACE=0;let result=end_pointers(array as *mut [*mut u8;9]) as *mut();let trace=TRACE;report("end_pointers",result,array,trace,0,0);}
{TRACE=0;let result=erased_pointers(p) as *mut();let trace=TRACE;report("erased_pointers",result,p as *mut u8,trace,0,0);}
{let mut slot=Slot::<*mut u8>{before:0x123456789abcdef0,value:std::ptr::null_mut(),after:0xfedcba9876543210};stored_pointers(&raw mut slot.value,p);report("stored_pointers",slot.value as *mut(),p as *mut u8,0,slot.before,slot.after);}
{TRACE=0;let result=passed_pointers(p) as *mut();let trace=TRACE;report("passed_pointers",result,p as *mut u8,trace,0,0);}
{TRACE=0;let result=field_flags(p) as *mut();let trace=TRACE;report("field_flags",result,p as *mut u8,trace,0,0);}
{TRACE=0;let result=whole_flags(array as *mut [bool;9]) as *mut();let trace=TRACE;report("whole_flags",result,array,trace,0,0);}
{TRACE=0;let result=end_flags(array as *mut [bool;9]) as *mut();let trace=TRACE;report("end_flags",result,array,trace,0,0);}
{TRACE=0;let result=erased_flags(p) as *mut();let trace=TRACE;report("erased_flags",result,p as *mut u8,trace,0,0);}
{let mut slot=Slot::<bool>{before:0x123456789abcdef0,value:std::ptr::null_mut(),after:0xfedcba9876543210};stored_flags(&raw mut slot.value,p);report("stored_flags",slot.value as *mut(),p as *mut u8,0,slot.before,slot.after);}
{TRACE=0;let result=passed_flags(p) as *mut();let trace=TRACE;report("passed_flags",result,p as *mut u8,trace,0,0);}
{TRACE=0;let result=passed_counted(p)as*mut();let trace=TRACE;report("passed_counted",result,p as*mut u8,trace,0,0);}
assert_eq!(munmap(memory,4096),0);}}}}
