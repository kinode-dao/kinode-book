# Filesystem API

Draft 28.11.2023

### Overview

(TODO: add link to filesystem overview)

Users should be extremely wary of giving any installed package the capability to message the filesystem directly, as this essentially gives the process (TODO: is process the right word here?) root access. Instead, most processes will interact with the filesystem indirectly through using the Get/Set/DeleteState functions, which can be called directly from the standard library [add link].

The u128s used in most commands are file handles, uuids that are unique to each written file.

### API

```rust
pub enum FsAction {
    Write(Option<u128>),
    WriteOffset((u128, u64)),
    Append(Option<u128>),
    Read(u128),
    ReadChunk(ReadChunkRequest),
    Delete(u128),
    Length(u128),
    SetLength((u128, u64)),
    GetState(ProcessId),
    SetState(ProcessId),
    DeleteState(ProcessId),
}

pub struct ReadChunkRequest {
    pub file: u128,
    pub start: u64,
    pub length: u64,
}
```

```rust
pub enum FsResponse {
    Write(u128),
    Read(u128),
    ReadChunk(u128),
    Append(u128),
    Delete(u128),
    Length(u64),
    GetState,
    SetState,
}
```

```rust
pub enum FsError {
    BadBytes { action: String },
    BadJson { json: String, error: String },
    NoJson,
    ReadFailed { file: u128, error: String },
    WriteFailed { file: u128, error: String },
    NotFound { file: u128 },
    S3Error { error: String },
    IOError { error: String },
    EncryptionError { error: String },
    LimitError { error: String },
    MemoryBufferError { error: String },
    LengthError { error: String },
    CreateInitialDirError { path: String, error: String },
}
```