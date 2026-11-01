use kuna_console::engine::bootstrap_from_object;
fn main() {
    let args: Vec<_> = std::env::args().collect();
    let mut program = bootstrap_from_object(&args[1], "", &[args[2].clone()]).unwrap();
    program.commit_pending_analysis().unwrap();
    let node = program.arch().types().find_by_name("main.Node").unwrap().unwrap();
    let next = node.get_field(0).unwrap().field_type.get_ptr_to().unwrap();
    println!("Node size: {}; Next pointee size: {}", node.get_size(), next.get_size());
}
