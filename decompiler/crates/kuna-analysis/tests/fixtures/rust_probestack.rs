extern "C" {
    fn probe_page(x: u32) -> u32;
    fn probe_tail(x: u32) -> u32;
    fn probe_pages(x: u32) -> u32;
    fn probe_observes_r11(x: u32) -> u32;
    fn probe_zero(x: u32) -> u32;
    fn probe_one(x: u32) -> u32;
    fn probe_page_r11(x: u32) -> u32;
    fn probe_plus_one(x: u32) -> u32;
    fn probe_pages_r11(x: u32) -> u32;
    fn probe_saved_rbp(x: u32) -> u32;
    fn probe_return_slot(x: u32) -> u32;
    fn probe_side_effect(x: u32) -> u32;
    fn probe_observes_flags(x: u32) -> u32;
    static mut probe_counter: u32;
}
fn main() {
    let x = std::env::args().nth(1).unwrap().parse::<u32>().unwrap();
    unsafe {
        let v0 = probe_page(x);
        let v1 = probe_tail(x);
        let v2 = probe_pages(x);
        let v3 = probe_observes_r11(x);
        let v4 = probe_zero(x);
        let v5 = probe_one(x);
        let v6 = probe_page_r11(x);
        let v7 = probe_plus_one(x);
        let v8 = probe_pages_r11(x);
        let v9 = probe_saved_rbp(x);
        let v10 = probe_return_slot(x);
        let v11 = probe_side_effect(x);
        let v12 = probe_observes_flags(x);
        let count = probe_counter;
        println!("{v0} {v1} {v2} {v3} {v4} {v5} {v6} {v7} {v8} {v9} {v10} {v11} {v12} {count}");
    }
}
