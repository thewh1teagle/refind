fn main() {
    env_logger::init();
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <id>", args[0]);
        return;
    }
    let id = &args[1];
    let realpath = refind::find_path(id).unwrap();
    println!("id: {}", id);
    println!("path: {}", realpath.display());
}
