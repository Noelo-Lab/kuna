#![no_std]
#![no_main]

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

fn double(x: u32) -> u32 {
    x.wrapping_mul(2)
}

fn square(x: u32) -> u32 {
    x.wrapping_mul(x)
}

fn negate(x: u32) -> u32 {
    x.wrapping_neg()
}

static OPS: [fn(u32) -> u32; 3] = [double, square, negate];

#[no_mangle]
pub extern "C" fn apply_all(which: usize, n: u32) -> u32 {
    let op = OPS[which % 3];
    let mut acc = 0u32;
    let mut i = 0u32;
    while i < n {
        acc = acc.wrapping_add(op(i));
        i += 1;
    }
    acc
}

#[no_mangle]
pub extern "C" fn collatz(mut n: u32) -> u32 {
    let mut steps = 0;
    while n != 1 {
        n = if n % 2 == 0 { n / 2 } else { n.wrapping_mul(3).wrapping_add(1) };
        steps += 1;
    }
    steps
}
