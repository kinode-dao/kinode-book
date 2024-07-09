# Databases

Kinode provides key-value databases via [RocksDB](https://rocksdb.org/), and relational databases via [SQLite](https://www.sqlite.org/docs.html).
Processes can create independent databases using wrappers over these libraries, and can read, write, and share these databases with other processes.
The APIs for doing are found here: [KV](./apis/kv.md) and [SQLite](./apis/sqlite.md).

[Similarly to drives in the VFS](./files.md#drives), they are accessed by `package_id` and a `db` name (i.e. [`kv::open()`](https://docs.rs/kinode_process_lib/latest/kinode_process_lib/kv/fn.open.html) and [`sqlite::open()`](https://docs.rs/kinode_process_lib/latest/kinode_process_lib/sqlite/fn.open.html)).
Capabilities to read and write can be shared with other processes.

All examples are using the [`kinode_process_lib`](./process_stdlib/overview.md) functions.

## Usage

For usage examples, see the [key-value API](./apis/kv.md) and the [SQlite API](./apis/sqlite.md).
