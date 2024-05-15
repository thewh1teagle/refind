<img width=95 src="https://github.com/thewh1teagle/refind/assets/61390950/bc625185-bb8c-450f-af3a-47780e4bb21f">

# refind

[![Crates](https://img.shields.io/crates/v/patty?logo=rust)](https://crates.io/crates/patty/)
[![License](https://img.shields.io/github/license/thewh1teagle/rookie?color=00aaaa&logo=license)](https://github.com/thewh1teagle/rookie/blob/main/rookie-rs/MIT-LICENSE.txt)

Keep track of files even after they're renamed or moved.

# Introduction

`refind` is cross platform `Rust` crate for locating file by it's `ID`.

Keep track of files even after they're renamed or moved.

# Supported platforms

`Windows` and `macOS`

# Install

```console
cargo add refind
```

# Usage

Create file ID

```rust
fn main() {
    let id = refind::get_id("<path>".into()).unwrap();
    println!("ID: {}", id);
}
```

Find path from ID

```rust
fn main() {
    let realpath = refind::find_path("<id">).unwrap();
    println!("Path: {}", realpath.display());
}
```

# Examples

See [examples](examples)

# ID format

In `Windows`, the ID consists of a string representation of an unsigned 64-bit integer, e.g., `111111111111111`.

In `macOS`, it's the device ID combined with the inode using a `:` separator, e.g., `111111111:2222222222`.

# How it works

### macOS

`macOS` has a special directory, `.vol`, allowing file access via device number and file inode. It also retrieves file paths from descriptors.

`refind` library creates a file ID with device ID and inode from `stat`, facilitating path retrieval via .vol. Finally, it uses `fcntl` with `F_GETPATH` for realpath. see more [Here](https://developer.apple.com/library/archive/qa/qa2001/qa1113.html).

To demonstrate, `stat <path>`, copy device ID and inode, then use

```console
GetFileInfo /.vol/{device id}/{file inode}
```

### Windows

`Windows` defines unique identifier for files and provide a way to get it via `GetFileInformationByHandle`

Later `refind` uses `OpenFileById` to open it by the ID, and uses `GetFileInformationByHandleEx` with `FILE_INFO_BY_HANDLE_CLASS` to get it's path.

To demonstrate, you can get file id with

```console
fsutil file queryFileid <path>
```

And to get it's path from the ID you can use

```console
fsutil file queryFileNamebyid <volume> fileID
```

See [microsoft/extendedfileapis](https://github.com/microsoft/Windows-classic-samples/blob/main/Samples/Win7Samples/winbase/io/extendedfileapis/ExtendedFileAPIs.cpp)

### Linux

Not implemented, see [stackoverflow.com/questions/1406679/](https://stackoverflow.com/questions/1406679/retrieving-the-path-from-a-file-descriptor-or-file)
