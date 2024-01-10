# VFS API

The VFS API tries to map over the [std::fs](https://doc.rust-lang.org/std/fs/index.html) calls as clearly as possible.

Every request takes a path and a corresponding action.
The paths look like normal relative paths within the folder `your_node_home/vfs`, but they include 2 parts at the start, a `package_id` and a `drive`.

Example path: `/your_package:publisher.nec/pkg/`. This folder is usually filled with files put into the /pkg folder when installing with app_store.

Capabilities are checked on the package_id/drive part of the path, when calling CreateDrive you'll be given "Read" and "Write" caps that you can share with other processes.

Other processes within your package will have access by default.
They can call actions within existing drives, like VfsAction::Read with path "/your_package:publisher.nec/assets/cat.jpeg".

```rust
pub struct VfsRequest {
    /// path is always prepended by package_id, the capabilities of the topmost folder are checked
    /// "/your_package:publisher.nec/drive_folder/another_folder_or_file"
    pub path: String,
    pub action: VfsAction,
}

pub enum VfsAction {
    CreateDrive,
    CreateDir,
    CreateDirAll,
    CreateFile,
    OpenFile { create: bool },
    CloseFile,
    Write,
    WriteAt,
    Append,
    SyncAll,
    Read,
    ReadDir,
    ReadToEnd,
    ReadExact(u64),
    ReadToString,
    Seek { seek_from: SeekFrom },
    RemoveFile,
    RemoveDir,
    RemoveDirAll,
    Rename { new_path: String },
    Metadata,
    AddZip,
    CopyFile { new_path: String },
    Len,
    SetLen(u64),
    Hash,
}

pub enum SeekFrom {
    Start(u64),
    End(i64),
    Current(i64),
}

pub enum FileType {
    File,
    Directory,
    Symlink,
    Other,
}

pub struct FileMetadata {
    pub file_type: FileType,
    pub len: u64,
}

pub struct DirEntry {
    pub path: String,
    pub file_type: FileType,
}

pub enum VfsResponse {
    Ok,
    Err(VfsError),
    Read,
    SeekFrom(u64),
    ReadDir(Vec<DirEntry>),
    ReadToString(String),
    Metadata(FileMetadata),
    Len(u64),
    Hash([u8; 32]),
}

pub enum VfsError {
    NoCap { action: String, path: String },
    BadBytes { action: String, path: String },
    BadRequest { error: String },
    ParseError { error: String, path: String },
    IOError { error: String, path: String },
    CapChannelFail { error: String },
    BadJson { error: String },
    NotFound { path: String },
    CreateDirError { path: String, error: String },
}
```
