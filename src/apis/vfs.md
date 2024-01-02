# VFS API

The VFS API tries to map over the [std::fs](https://doc.rust-lang.org/std/fs/index.html) calls as clearly as possible. 

Every request takes a path and a corresponding action. 
The paths look like normal relative paths within the folder `your_node_home/vfs`, but they include 2 parts at the start, a `package_id` and a `drive`.

Example path: `/your_package:publisher.uq/pkg/`. This folder is usually filled with files put into the /pkg folder when installing with app_store.

Capabilities are checked on the package_id/drive part of the path, when calling CreateDrive you'll be given "Read" and "Write" caps that you can share with other processes. 

Other processes within your package will have access by default. 
They can call actions within existing drives, like VfsAction::Read with path "/your_package:publisher.uq/assets/cat.jpeg". 

```rust
pub struct VfsRequest {
    pub path: String,
    pub action: VfsAction,
}

/// VfsActions mostly mirror the behaviour of std::fs
pub enum VfsAction {
    /// creates a drive ["your_package:publisher.uq/your_drive/other_path"] and attaches capabilities
    /// to the process calling it
    CreateDrive,
    CreateDir,
    CreateDirAll,
    CreateFile,
    OpenFile,
    CloseFile,
    WriteAll,
    Write,
    ReWrite,
    WriteAt(u64),
    Append,
    SyncAll,
    Read,
    ReadToEnd,
    ReadDir,
    ReadExact(u64),
    ReadToString,
    Seek(SeekFrom),
    RemoveFile,
    RemoveDir,
    RemoveDirAll,
    Rename(String),
    Metadata,
    AddZip,
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