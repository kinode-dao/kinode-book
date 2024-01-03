### KV API

```rust
/// Actions are sent to a specific key value database, "db" is the name,
/// "package_id" is the package. Capabilities are checked, you can access another process's
/// database if it has given you the capability.
pub struct KvRequest {
    pub package_id: PackageId,
    pub db: String,
    pub action: KvAction,
}

pub enum KvAction {
    /// New is called to create a new database and be given capabilities,
    /// or to open a connection to an existing one.
    New,
    Set {
        key: Vec<u8>,
        tx_id: Option<u64>,
    },
    Delete {
        key: Vec<u8>,
        tx_id: Option<u64>,
    },
    Get {
        key: Vec<u8>,
    },
    BeginTx,
    Commit {
        tx_id: u64,
    },
    Backup,
}

pub enum KvResponse {
    Ok,
    BeginTx { tx_id: u64 },
    Get { key: Vec<u8> },
    Err { error: KvError },
}

pub enum KvError {
    NoDb,
    DbAlreadyExists,
    KeyNotFound,
    NoTx,
    NoCap { error: String },
    RocksDBError { action: String, error: String },
    InputError { error: String },
    IOError { error: String },
}
```
