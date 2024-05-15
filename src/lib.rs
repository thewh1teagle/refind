mod platform;

#[cfg(test)]
mod test;

pub use platform::NormalizePathTrait;

#[cfg(target_os = "linux")]
pub use platform::linux::{find_path, get_id};

#[cfg(target_os = "macos")]
pub use platform::macos::{find_path, get_id};

#[cfg(target_os = "windows")]
pub use platform::windows::{find_path, get_id};
