### SQLITE API

```rust
/// Actions are sent to a specific sqlite database, "db" is the name,
/// "package_id" is the package. Capabilities are checked, you can access another process's
/// database if it has given you the capability.
pub struct SqliteRequest {
    pub package_id: PackageId,
    pub db: String,
    pub action: SqliteAction,
}

pub enum SqliteAction {
    /// New is called to create a new database and be given capabilities,
    /// or to open a connection to an existing one.
    New,
    Write {
        statement: String,
        tx_id: Option<u64>,
    },
    Read {
        query: String,
    },
    BeginTx,
    Commit {
        tx_id: u64,
    },
    Backup,
}

pub enum SqliteResponse {
    Ok,
    Read,
    BeginTx { tx_id: u64 },
    Err { error: SqliteError },
}

pub enum SqlValue {
    Integer(i64),
    Real(f64),
    Text(String),
    Blob(Vec<u8>),
    Boolean(bool),
    Null,
}

pub enum SqliteError {
    NoDb,
    DbAlreadyExists,
    NoTx,
    NoCap { error: String },
    UnexpectedResponse,
    NotAWriteKeyword,
    NotAReadKeyword,
    InvalidParameters,
    IOError { error: String },
    RusqliteError { error: String },
    InputError { error: String },
}

```
