use arrayvec::ArrayVec;

fn main() {
    let mut lens: ArrayVec<u32, 4> = ArrayVec::new();
    for arg in std::env::args() {
        lens.push(arg.len() as u32);
    }
    let first = lens.remove(0);
    println!("{}", first + lens.iter().sum::<u32>());
}
