### Databases

The runtime currently comes preloaded with 2 databases, a key value (RocksDB) and sqlite.

Databases can be created by, accessed by, and shared amongst processes.
The APIs are documented here: [KV](./apis/kv.md) and [SQLITE](./apis/sqlite.md).

Similarly to files in the VFS, they are accessed by `package_id` and a `db` name.
Capabilities to read and write can be shared with other processes; processes within a given package have access by default.

There are useful helper functions in the standard library [Link?] to create/connect, read, write, commit_tx.

### Links

- [KV API](./apis/kv.md)
- [SQLITE API](./apis/sqlite.md)
- [RocksDB](https://github.com/rust-rocksdb/rust-rocksdb)
- [SQLITE](https://www.sqlite.org/docs.html)
