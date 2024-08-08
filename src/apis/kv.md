### KV API

Useful helper functions can be found in the [`kinode_process_lib`](../process_stdlib/overview.md).
More discussion of databases in Kinode can be found [here](../system/databases.md).

#### Creating/Opening a database

```rust
use kinode_process_lib::kv;

let kv = kv::open(our.package_id(), "birthdays")?;

// You can now pass this KV struct as a reference to other functions
```

#### Set

```rust
let key = b"hello";
let value= b"world";

let returnvalue = kv.set(&key, &value, None)?;
// The third argument None is for tx_id.
// You can group sets and deletes and commit them later.
```

#### Get

```rust
let key = b"hello";

let returnvalue = kv.get(&key)?;
```

#### Delete

```rust
let key = b"hello";

kv.delete(&key, None)?;
```

#### Transactions

```rust
let tx_id = kv.begin_tx()?;

let key = b"hello";
let key2 = b"deleteme";
let value= b"value";

kv.set(&key, &value, Some(tx_id))?;
kv.delete(&key, Some(tx_id))?;

kv.commit_tx(tx_id)?;
```

### API

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
    Open,
    RemoveDb,
    Set { key: Vec<u8>, tx_id: Option<u64> },
    Delete { key: Vec<u8>, tx_id: Option<u64> },
    Get { key: Vec<u8> },
    BeginTx,
    Commit { tx_id: u64 },
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
    KeyNotFound,
    NoTx,
    NoCap { error: String },
    RocksDBError { action: String, error: String },
    InputError { error: String },
    IOError { error: String },
}
```
