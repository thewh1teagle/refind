use crate::NormalizePathTrait;
use std::path::PathBuf;

#[test]
fn hello() {
    let path = PathBuf::from("Cargo.toml").normalize().unwrap();
    let id = crate::get_id(path.clone()).unwrap();
    let realpath = crate::find_path(&id).unwrap();
    assert_eq!(realpath, path);
}
