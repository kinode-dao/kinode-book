# Saving State

Every Kinode process has access to two system calls that save and load persistent state: `set_state` and `get_state`.
`set_state` takes a byte-vector and saves it in the kernel's disk storage.
`get_state` takes no arguments and returns an optional byte-vector.
If the byte-vector is present, it was previously saved by `set_state`.
If the byte-vector is not present, it was not previously saved.

The byte-vector itself is opaque to the kernel, though not encrypted.
It can be retrieved later only by `get_state`, and only the original process that called `set_state` can retrieve it.

Processes frequently use this feature to maintain key state between restarts, which can happen at any time as a result of crashes, package updates, or node reboots.
It is considered good practice to save state any time the process mutates it.

**Keep in mind that every state set/get incurs an asynchronous disk read/write for the entire state object. If storing large amounts of data, consider using the [`vfs`, `sqlite`, and/or `kv` modules](../system/databases.md)!**

Here's an example of a process that saves and loads state:

```rust
{{#includehidetest ../../code/save-state/save-state/src/lib.rs}}
```

This process has a simple u64 counter that is incremented on each initialization.
It then exits and is restarted (because of its designated behavior in `manifest.json`), and the counter is loaded from state and incremented again.

State serialization and deserialization can be done in a variety of ways, and usually uses `serde::Serialize` and `serde::Deserialize` derived on a particular struct.

When using process state, make sure to handle the case where the state is not present, and if updating an existing process, always handle older state formats if you change the format being stored.
