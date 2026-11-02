mod r#union {
    #[inline(never)]
    pub fn target(x: u32) -> u32 {
        x.wrapping_add(10)
    }
}

#[no_mangle]
#[inline(never)]
pub extern "C" fn direct_target(x: u32) -> u32 {
    r#union::target(x)
}

fn main() {
    let x = std::env::args().nth(1).unwrap().parse::<u32>().unwrap();
    println!("{}", std::hint::black_box(direct_target)(x));
}
