### Databases

The runtime currently comes preloaded with 2 databases, a key value (RocksDB) and sqlite.

These can be created, accessed, and shared amongst processes.
The APIs for doing so you can find here: [KV](./apis/kv.md) and [SQLITE](./apis/sqlite.md).

Similarly to files in the VFS, they are accessed by `package_id` and a `db` name.
Capabilities to read and write can be shared to other processes, and ones in your own package have access by default.

There are useful helper functions in the standard library [Link?] to create/connect, read, write, commit_tx.

### Links

- [KV API](./apis/kv.md)
- [SQLITE API](./apis/sqlite.md)
- [RocksDB](https://github.com/rust-rocksdb/rust-rocksdb)
- [SQLITE](https://www.sqlite.org/docs.html)
