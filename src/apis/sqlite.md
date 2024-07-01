### SQLite API

Useful helper functions can be found in the [`kinode_process_lib`](../process_stdlib/overview.md).
More discussion of databases in Kinode can be found [here](../databases.md).

#### Creating/Opening a database

```rust
use kinode_process_lib::sqlite;

let db = sqlite::open(our.package_id(), "users", None).unwrap();
// You can now pass this SQLite struct as a reference to other functions
```

#### Write

```rust
let statement = "INSERT INTO users (name) VALUES (?), (?), (?);".to_string();
let params = vec![
serde_json::Value::String("Bob".to_string()),
serde_json::Value::String("Charlie".to_string()),
serde_json::Value::String("Dave".to_string()),
];

db.write(statement, params, None);
```

#### Read

```rust
let query = "SELECT FROM users;".to_string();
let rows = db.read(query, vec![]).unwrap();
// rows: Vec<HashMap<String, serde_json::Value>>
println!("rows: {}", rows.len());
for row in rows {
    println!(row.get("name"));
}
```

#### Transactions

```rust
let tx_id = db.begin_tx();

let statement = "INSERT INTO users (name) VALUES (?);".to_string();
let params = vec![serde_json::Value::String("Eve".to_string())];
let params2 = vec![serde_json::Value::String("Steve".to_string())];

db.write(statement, params, Some(tx_id));
db.write(statement, params2, Some(tx_id));

db.commit_tx(tx_id);
```

### API

```rust
/// Actions are sent to a specific sqlite database, "db" is the name,
/// "package_id" is the package. Capabilities are checked, you can access another process's
/// database if it has given you the capability.
pub struct SqliteRequest {
    pub package_id: PackageId,
    pub db: String,
    pub action: SqliteAction,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SqliteAction {
    Open,
    RemoveDb,
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
