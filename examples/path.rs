fn main() {
    // Parse command-line arguments
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <id>", args[0]);
        return;
    }
    let id = &args[1];
    println!("id: {}", id);
    let path = refind::find_path(id).unwrap();

    println!("path: {}", path.display());
}
