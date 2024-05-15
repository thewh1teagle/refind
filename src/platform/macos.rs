use eyre::{bail, Context, Result};
use libc::{fcntl, F_GETPATH, O_RDONLY};
use std::ffi::{CStr, CString};
use std::{fs, os::unix::fs::MetadataExt, path::PathBuf};

pub fn get_id(path: PathBuf) -> Result<String> {
    let metadata = fs::metadata(path)?;
    let inode = metadata.ino();
    let dev_id = metadata.dev();
    // Combine inode and dev_id with a separator
    let file_id = format!("{}:{}", dev_id, inode);

    Ok(file_id)
}
pub fn find_path(id: &str) -> eyre::Result<PathBuf> {
    let parts: Vec<&str> = id.split(':').collect();
    if parts.len() != 2 {
        bail!("Invalid id format. Expected format: 'volume_id:inode'");
    }
    let volume_id = parts[0];
    let inode = parts[1];
    let raw_path = format!("/.vol/{}/{}", volume_id, inode);
    let result = realpath(&raw_path)?;
    Ok(result.into())
}

/// Return real filesystem path to raw volume path which contains volume serial number and file inode
/// Eg. /.vol/{volume_id}/{inode}
fn realpath(vol_path: &str) -> Result<String> {
    // Open the file
    let c_file_path = CString::new(vol_path).context("CString conversion failed")?;
    let fd = unsafe { libc::open(c_file_path.as_ptr(), O_RDONLY) };
    if fd == -1 {
        bail!("Failed to open file");
    }

    // Get the file path using fcntl
    let mut path: [libc::c_char; 4096] = [0; 4096];
    let path_length = unsafe { fcntl(fd, F_GETPATH, path.as_mut_ptr()) };
    if path_length == -1 {
        bail!("Failed to get file path");
    }

    // Convert the C string to Rust string
    let path_string = unsafe {
        let path_cstr = CStr::from_ptr(path.as_ptr());
        path_cstr.to_string_lossy().into_owned()
    };

    // Close the file descriptor
    unsafe { libc::close(fd) };
    Ok(path_string)
}
