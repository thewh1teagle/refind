use eyre::Result;
use std::path::PathBuf;

#[cfg(target_os = "linux")]
pub mod linux;

#[cfg(target_os = "macos")]
pub mod macos;

#[cfg(target_os = "windows")]
pub mod windows;

pub trait NormalizePathTrait {
    fn normalize(&self) -> Result<PathBuf, std::io::Error>;
}

impl NormalizePathTrait for PathBuf {
    fn normalize(&self) -> Result<PathBuf, std::io::Error> {
        let path = dunce::realpath(self)?;
        Ok(path)
    }
}
