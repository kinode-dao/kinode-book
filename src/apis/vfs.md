# VFS API

Draft 28.11.2023

### Overview

The virtual filesystem is a light layer on top of the internal filesystem, and is used to provide a consistent interface for interacting with the filesystem.

Within it you'll find some familiar posix commands, some new. The vfs also handles capabilities, whereas access to the underlying filesystem is essentially root access, mostly reserved for runtime modules. 

The vfs' metadata is stored in a similar way to apps state, by calling save_state.

### API

Requests always have a target drive. Modifying or reading this drive requires a corresponding capability enforced by the VFS. You can pass these capabilities to other apps/nodes who you want to be able to modify/read your files.

[link to general capability guide/examples]

```rust
pub struct VfsRequest {
    pub drive: String,
    pub action: VfsAction,
}
```

[todo implicit full_path behavior describing]

```rust
pub enum VfsAction {
    New,
    Add {
        full_path: String,
        entry_type: AddEntryType,
    },
    Rename {
        full_path: String,
        new_full_path: String,
    },
    Delete(String),
    WriteOffset {
        full_path: String,
        offset: u64,
    },
    Append(String),
    SetSize {
        full_path: String,
        size: u64,
    },
    GetPath(u128),
    GetHash(String),
    GetEntry(String),
    GetFileChunk {
        full_path: String,
        offset: u64,
        length: u64,
    },
    GetEntryLength(String),
}

pub enum AddEntryType {
    Dir,
    NewFile,                     //  add a new file to fs and add name in vfs
    ExistingFile { hash: u128 }, //  link an existing file in fs to a new name in vfs
    ZipArchive,
}
```

```rust
pub enum GetEntryType {
    Dir,
    File,
}
```

A response from the Vfs can be an error, you can branch off of some of the common ones in your app [example]. 

```rust
pub enum VfsResponse {
    Ok,
    Err(VfsError),
    GetPath(Option<String>),
    GetHash(Option<u128>),
    GetEntry {
        // file bytes in payload, if entry was a file
        is_file: bool,
        children: Vec<String>,
    },
    GetFileChunk, // chunk in payload, if file exists
    GetEntryLength(Option<u64>),
}

pub enum VfsError {
    BadJson,
    BadPayload,
    BadDriveName,
    BadDescriptor,
    NoCap,
    EntryNotFound,
    PersistError,
    InternalError,
}
```