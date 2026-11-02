#[no_mangle]
#[inline(never)]
pub extern "C" fn null_pointer() -> *mut u8 { core::ptr::null_mut() }
#[no_mangle]
#[inline(never)]
pub extern "C" fn maybe_pointer(choice: u64, pointer: *mut u8) -> *mut u8 {
    if choice == 42 { pointer } else { core::ptr::null_mut() }
}
#[no_mangle]
#[inline(never)]
pub extern "C" fn pointer_is_null(pointer: *mut u8) -> i32 { pointer.is_null() as i32 }
#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn clear_pointer(slot: *mut *mut u8) { *slot = core::ptr::null_mut(); }
fn main() {
    let choice: u64 = std::env::args().nth(1).unwrap().parse().unwrap();
    let mut byte = 17u8;
    let pointer = &mut byte as *mut u8;
    let mut slot = pointer;
    let null = std::hint::black_box(null_pointer as extern "C" fn() -> *mut u8)();
    let selected = std::hint::black_box(maybe_pointer as extern "C" fn(u64,*mut u8)->*mut u8)(choice,pointer);
    unsafe { std::hint::black_box(clear_pointer as unsafe extern "C" fn(*mut *mut u8))(&mut slot); }
    println!("{} {} {} {} {}",null.is_null(),selected==pointer,slot.is_null(),std::hint::black_box(pointer_is_null as extern "C" fn(*mut u8)->i32)(null),std::hint::black_box(pointer_is_null as extern "C" fn(*mut u8)->i32)(pointer));
}
