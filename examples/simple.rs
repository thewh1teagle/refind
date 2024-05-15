fn main() {
    let id = refind::get_id("LICENSE".into()).unwrap();
    let realpath = refind::find_path(&id).unwrap();
    println!("id: {}", id);
    println!("path: {}", realpath.display());
}
