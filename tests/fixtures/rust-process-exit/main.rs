use std::ptr;

#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn exit_choice(present: u32, kind: u32, output: *mut u64) -> u32 {
    if present == 0 {
        return 27;
    }
    if kind == 11 {
        std::process::exit(0);
    }
    ptr::write_volatile(output, 31);
    std::process::exit(1)
}

mod lookalike {
    pub mod process {
        #[inline(never)]
        pub fn exit(value: u32) -> u32 {
            value.wrapping_add(7)
        }
    }
}

#[export_name = "_ZN3std7process11exit_status17h0123456789abcdefE"]
#[inline(never)]
pub extern "C" fn exit_status(value: u32) -> u32 {
    value.wrapping_add(9)
}

#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn returning_names(value: u32, output: *mut u64) -> u32 {
    let first = lookalike::process::exit(value);
    let second = exit_status(first);
    ptr::write_volatile(output, u64::from(second));
    second
}
