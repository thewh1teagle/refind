use eyre::{bail, Result};
use std::{
    fs,
    os::unix::fs::MetadataExt,
    path::{Path, PathBuf},
};

pub fn get_id(path: PathBuf) -> Result<String> {
    let metadata = fs::metadata(path)?;
    let inode = metadata.ino();
    let dev_id = metadata.dev();
    // Combine inode and dev_id with a separator
    let file_id = format!("{}:{}", dev_id, inode);

    Ok(file_id)
}

pub fn find_path(id: &str) -> Result<PathBuf> {
    let parts: Vec<&str> = id.split(':').collect();
    if parts.len() != 2 {
        bail!("Invalid id format. Expected format: 'volume_id:inode'");
    }
    if let Ok(home) = std::env::var("HOME") {
        if let Some(path) = find_path_recursive(id, Path::new(&home))? {
            return Ok(path);
        }
    }

    bail!("not found")
}

fn find_path_recursive(id: &str, dir: &Path) -> Result<Option<PathBuf>> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        let result = fs::metadata(&path);
        if result.is_err() {
            continue;
        }
        let metadata = result.unwrap();
        if metadata.is_dir() && !metadata.is_symlink() {
            if let Some(found_path) = find_path_recursive(id, &path)? {
                return Ok(Some(found_path));
            }
        } else if let Ok(file_id) = get_id(path.clone()) {
            if file_id == id {
                return Ok(Some(path));
            }
        }
    }
    Ok(None)
}
