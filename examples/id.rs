fn main() {
    // Parse command-line arguments
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        return;
    }
    let path = &args[1];
    let id = refind::get_id(path.into()).unwrap();

    println!("id: {}", id);
}
