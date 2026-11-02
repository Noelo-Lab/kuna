mod left {
    #[inline(never)]
    pub fn target(x: u32) -> u32 { x.wrapping_add(10) }
    #[inline(never)]
    pub fn same(x: u32) -> u32 { target(x) }
    pub mod deep {
        #[inline(never)]
        pub fn target(x: u32) -> u32 { x.wrapping_add(20) }
    }
}
mod right {
    #[inline(never)]
    pub fn target(x: u32) -> u32 { x.wrapping_add(30) }
    #[inline(never)]
    pub fn cross(x: u32) -> u32 { super::left::target(x) }
}
#[inline(never)]
fn target(x: u32) -> u32 { x.wrapping_add(40) }
#[no_mangle]
#[inline(never)]
pub extern "C" fn direct_left(x: u32) -> u32 { left::target(x) }
#[no_mangle]
#[inline(never)]
pub extern "C" fn direct_nested(x: u32) -> u32 { left::deep::target(x) }
#[no_mangle]
#[inline(never)]
pub extern "C" fn direct_right(x: u32) -> u32 { right::target(x) }
#[no_mangle]
#[inline(never)]
pub extern "C" fn direct_global(x: u32) -> u32 { target(x) }

fn main() {
    let x = std::env::args().nth(1).map(|s| s.parse::<u32>().unwrap()).unwrap_or(0);
    println!("{} {} {} {} {} {}", direct_left(x), direct_nested(x), direct_right(x),
             direct_global(x), left::same(x), right::cross(x));
}
