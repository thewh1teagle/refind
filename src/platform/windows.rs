use eyre::{bail, Context, Result};
use std::{
    env::current_dir,
    os::windows::{ffi::OsStrExt, fs::OpenOptionsExt},
    path::{Path, PathBuf},
};
use windows::{
    core::PCWSTR,
    Win32::{
        Foundation::{CloseHandle, GENERIC_READ, MAX_PATH},
        Storage::FileSystem::{
            CreateFileW, FileIdType, GetFileInformationByHandleEx, OpenFileById,
            FILE_FLAG_BACKUP_SEMANTICS, FILE_GENERIC_READ, FILE_ID_DESCRIPTOR,
            FILE_ID_DESCRIPTOR_0, FILE_INFO_BY_HANDLE_CLASS, FILE_SHARE_DELETE, FILE_SHARE_READ,
            FILE_SHARE_WRITE, OPEN_EXISTING,
        },
    },
};

use crate::NormalizePathTrait;

pub fn get_id(path: std::path::PathBuf) -> eyre::Result<String> {
    let id = realid(&path)?.to_string();
    Ok(id)
}

pub fn find_path(id: &str) -> eyre::Result<PathBuf> {
    let id: u64 = id.trim().parse()?;
    // Get realpath from winAPI
    let path = realpath(id)?;
    // Create normalized pathbuf
    // Sometimes windows add irregular prefix
    let path = PathBuf::from(path).normalize()?;
    Ok(path)
}

/// https://github.com/rust-lang/rust/issues/63010
/// Get fileID using GetFileInformationByHandle
/// Similar to the command:
/// fsutil file queryFileID <path>
fn realid(path: &Path) -> Result<u64> {
    use std::os::windows::io::AsRawHandle;

    use windows::Win32::{
        Foundation::HANDLE,
        Storage::FileSystem::{GetFileInformationByHandle, BY_HANDLE_FILE_INFORMATION},
    };

    let file = std::fs::OpenOptions::new()
        .read(true)
        .custom_flags(FILE_FLAG_BACKUP_SEMANTICS.0)
        .open(path)?;

    let mut info: BY_HANDLE_FILE_INFORMATION = unsafe { std::mem::zeroed() };
    // https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-getfileinformationbyhandle
    // This function supports Windows XP+
    let res =
        unsafe { GetFileInformationByHandle(HANDLE(file.as_raw_handle() as isize), &mut info) };
    if res.is_err() {
        let last_error = std::io::Error::last_os_error();
        bail!("path {} {}", path.display(), last_error);
    };

    Ok(((info.nFileIndexHigh as u64) << 32) | (info.nFileIndexLow as u64))
}

/// Get path from file ID using GetFileInformationByID
/// https://github.com/microsoft/Windows-classic-samples/blob/main/Samples/Win7Samples/winbase/io/extendedfileapis/ExtendedFileAPIs.cpp
fn realpath(id: u64) -> Result<String> {
    // Create directory handle for current directory (assume file on the save drive)
    let share_mode = FILE_SHARE_READ | FILE_SHARE_WRITE | FILE_SHARE_DELETE;
    let mut dir_path: Vec<u16> = current_dir()?
        .as_mut_os_str()
        .encode_wide()
        .chain(Some(0)) // Must be null terminated!
        .collect();
    let dir_path = PCWSTR(dir_path.as_mut_ptr());

    // Get current folder handle
    let dir_handle = unsafe {
        CreateFileW(
            dir_path,
            FILE_GENERIC_READ.0,
            share_mode,
            None,
            OPEN_EXISTING,
            FILE_FLAG_BACKUP_SEMANTICS,
            None,
        )
    }?;
    let file_name_info_size = std::mem::size_of::<FILE_ID_DESCRIPTOR>();
    let file_id_descriptor_inner = FILE_ID_DESCRIPTOR_0 { FileId: id as _ };
    let file_id_descriptor = FILE_ID_DESCRIPTOR {
        Type: FileIdType,
        Anonymous: file_id_descriptor_inner,
        dwSize: file_name_info_size as _,
    };
    let file_handle = unsafe {
        OpenFileById(
            dir_handle,
            &file_id_descriptor as _,
            GENERIC_READ.0,
            share_mode,
            None,
            FILE_FLAG_BACKUP_SEMANTICS,
        )
    }?;
    unsafe {
        CloseHandle(dir_handle)?;
    }

    let file_name_info_class = FILE_INFO_BY_HANDLE_CLASS(2 as _); // https://learn.microsoft.com/en-us/windows/win32/api/winbase/ns-winbase-file_name_info
    let name_size = file_name_info_size + (4 * MAX_PATH) as usize; // WCHAR * MAX_PATH
    let mut path_buffer: Vec<u16> = vec![0; name_size];
    let name_ptr = path_buffer.as_mut_ptr();
    unsafe {
        GetFileInformationByHandleEx(
            file_handle,
            file_name_info_class,
            name_ptr as _,
            name_size.try_into().context("try_into")?,
        )
    }?;

    unsafe {
        CloseHandle(file_handle)?;
    }

    // Skip null terminated prefix
    let file_path = decode_path(path_buffer)?;
    Ok(file_path)
}

fn decode_path(buf: Vec<u16>) -> Result<String> {
    let skip = 2;
    let null_pos = buf
        .iter()
        .skip(skip)
        .position(|&c| c == 0) // Find end of text
        .map(|pos| pos + 1)
        .unwrap_or(buf.len());

    // Create a String from the UTF-16 slice up to the first null character
    let string = String::from_utf16(&buf[skip..null_pos + 1])?;
    Ok(string)
}
