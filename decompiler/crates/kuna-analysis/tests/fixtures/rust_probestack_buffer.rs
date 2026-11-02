extern "C" {
    fn probe_buffer(x: u32) -> u32;
}
fn main() {
    let x = std::env::args().nth(1).unwrap().parse::<u32>().unwrap();
    println!("{}", unsafe { probe_buffer(x) });
}
